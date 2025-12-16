use std::io::{Cursor, Read};

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::BufReader;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

/// Represents a single packet from a pcap file
#[derive(Debug, Clone)]
pub struct Packet {
    /// Timestamp (seconds)
    pub ts_sec: u32,
    /// Timestamp (microseconds)
    pub ts_usec: u32,
    /// Captured packet data
    pub data: Vec<u8>,
}

/// Iterator over packets in a pcap file
pub struct PcapIterator<R: Read> {
    reader: R,
    is_big_endian: bool,
}

impl<R: Read> PcapIterator<R> {
    /// Create a new pcap iterator from a reader
    pub fn new(mut reader: R) -> std::io::Result<Self> {
        // Read and parse pcap file header (24 bytes)
        let mut header = [0u8; 24];
        reader.read_exact(&mut header)?;

        // Check magic number to determine endianness
        // The magic bytes are always stored in the file's native endianness
        // 0xa1b2c3d4 = little-endian PCAP file
        // 0xd4c3b2a1 = big-endian PCAP file
        let magic = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        let is_big_endian = if magic == 0xa1b2c3d4 {
            false // little-endian PCAP file - the bytes read as LE give this magic
        } else if magic == 0xd4c3b2a1 {
            true // big-endian PCAP file - the bytes read as LE give this magic
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid pcap magic number",
            ));
        };

        Ok(PcapIterator {
            reader,
            is_big_endian,
        })
    }

    /// Create a new pcap iterator from a byte slice (WASM-compatible)
    pub fn from_bytes(bytes: &[u8]) -> std::io::Result<PcapIterator<Cursor<&[u8]>>> {
        let cursor = Cursor::new(bytes);
        PcapIterator::new(cursor)
    }

    fn read_u32(&self, bytes: &[u8]) -> u32 {
        let val = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if self.is_big_endian {
            val.swap_bytes()
        } else {
            val
        }
    }
}

impl<R: Read> Iterator for PcapIterator<R> {
    type Item = std::io::Result<Packet>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut packet_header = [0u8; 16];
        match self.reader.read_exact(&mut packet_header) {
            Ok(()) => {
                let ts_sec = self.read_u32(&packet_header[0..4]);
                let ts_usec = self.read_u32(&packet_header[4..8]);
                let incl_len = self.read_u32(&packet_header[8..12]);

                // Read packet data
                let mut data = vec![0u8; incl_len as usize];
                match self.reader.read_exact(&mut data) {
                    Ok(()) => Some(Ok(Packet {
                        ts_sec,
                        ts_usec,
                        data,
                    })),
                    Err(e) => Some(Err(e)),
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}

/// Open a pcap file and return an iterator over its packets
///
/// This function is only available on non-WASM targets.
/// For WASM compatibility, use `PcapIterator::from_bytes()` instead.
#[cfg(not(target_arch = "wasm32"))]
pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<PcapIterator<BufReader<File>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    PcapIterator::new(reader)
}
