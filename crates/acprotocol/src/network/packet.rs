use serde::Serialize;
use std::error::Error;

#[cfg(test)]
use std::io::Cursor;

use crate::enums::PacketHeaderFlags;
use crate::readers::{read_u16, read_u32, ACDataType, ACReader};
use crate::writers::{write_u16, write_u32, ACWritable, ACWriter};

#[derive(Debug, Clone, Serialize)]
pub struct PacketHeader {
    pub sequence: u32,
    pub flags: PacketHeaderFlags,
    pub checksum: u32,
    pub id: u16,
    pub time: u16,
    pub size: u16,
    pub iteration: u16,
}

impl PacketHeader {
    pub const BASE_SIZE: usize = 20;

    pub fn with_flags(flags: PacketHeaderFlags) -> Self {
        Self {
            sequence: 0,
            flags,
            checksum: 0,
            id: 0,
            time: 0,
            size: 0,
            iteration: 0,
        }
    }
}

impl ACDataType for PacketHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        let sequence = read_u32(reader)?;
        let flags = PacketHeaderFlags::read(reader)?;
        let checksum = read_u32(reader)?;
        let id = read_u16(reader)?;
        let time = read_u16(reader)?;
        let size = read_u16(reader)?;
        let iteration = read_u16(reader)?;

        Ok(Self {
            sequence,
            flags,
            checksum,
            id,
            time,
            size,
            iteration,
        })
    }
}

impl ACWritable for PacketHeader {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_u32(writer, self.sequence)?;
        self.flags.write(writer)?;
        write_u32(writer, self.checksum)?;
        write_u16(writer, self.id)?;
        write_u16(writer, self.time)?;
        write_u16(writer, self.size)?;
        write_u16(writer, self.iteration)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_header_flags() {
        assert_eq!(PacketHeaderFlags::NONE.bits(), 0x00000000);
        assert_eq!(PacketHeaderFlags::RETRANSMISSION.bits(), 0x00000001);
        assert_eq!(PacketHeaderFlags::BLOB_FRAGMENTS.bits(), 0x00000004);
        assert_eq!(PacketHeaderFlags::ACK_SEQUENCE.bits(), 0x00004000);
        assert_eq!(PacketHeaderFlags::TIME_SYNC.bits(), 0x01000000);

        // Test flag combinations
        let combined = PacketHeaderFlags::RETRANSMISSION | PacketHeaderFlags::ACK_SEQUENCE;
        assert!(combined.contains(PacketHeaderFlags::RETRANSMISSION));
        assert!(combined.contains(PacketHeaderFlags::ACK_SEQUENCE));
        assert!(!combined.contains(PacketHeaderFlags::TIME_SYNC));
    }

    #[test]
    fn test_packet_header_parse() {
        // Construct a minimal packet header (20 bytes)
        let data = vec![
            0x01, 0x00, 0x00, 0x00, // sequence: 1
            0x04, 0x00, 0x00, 0x00, // flags: BLOB_FRAGMENTS (0x04)
            0xAA, 0xBB, 0xCC, 0xDD, // checksum: 0xDDCCBBAA
            0x34, 0x12, // id: 0x1234
            0x78, 0x56, // time: 0x5678
            0x00, 0x02, // size: 512
            0x01, 0x00, // iteration: 1
        ];

        let mut cursor = Cursor::new(&data[..]);
        let header = PacketHeader::read(&mut cursor).unwrap();

        assert_eq!(header.sequence, 1);
        assert_eq!(header.flags, PacketHeaderFlags::BLOB_FRAGMENTS);
        assert_eq!(header.checksum, 0xDDCCBBAA);
        assert_eq!(header.id, 0x1234);
        assert_eq!(header.time, 0x5678);
        assert_eq!(header.size, 512);
        assert_eq!(header.iteration, 1);
    }

    #[test]
    fn test_packet_header_parse_with_multiple_flags() {
        // Test with multiple flags set
        let data = vec![
            0x00, 0x00, 0x00, 0x00, // sequence: 0
            0x05, 0x40, 0x00, 0x00, // flags: BLOB_FRAGMENTS | ACK_SEQUENCE (0x4005)
            0x00, 0x00, 0x00, 0x00, // checksum: 0
            0x00, 0x00, // id: 0
            0x00, 0x00, // time: 0
            0x64, 0x00, // size: 100
            0x00, 0x00, // iteration: 0
        ];

        let mut cursor = Cursor::new(&data[..]);
        let header = PacketHeader::read(&mut cursor).unwrap();

        assert!(header.flags.contains(PacketHeaderFlags::BLOB_FRAGMENTS));
        assert!(header.flags.contains(PacketHeaderFlags::ACK_SEQUENCE));
        assert_eq!(header.size, 100);
    }

    #[test]
    fn test_packet_header_parse_insufficient_data() {
        // Only 10 bytes when 20 are needed
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];

        let mut cursor = Cursor::new(&data[..]);
        let result = PacketHeader::read(&mut cursor);

        assert!(result.is_err());
    }

    #[test]
    fn test_packet_header_write_read_roundtrip() {
        use crate::writers::ACWritable;

        // Create a test header
        let original = PacketHeader {
            sequence: 42,
            flags: PacketHeaderFlags::BLOB_FRAGMENTS | PacketHeaderFlags::ACK_SEQUENCE,
            checksum: 0xDEADBEEF,
            id: 0x1234,
            time: 0x5678,
            size: 1024,
            iteration: 7,
        };

        // Write to buffer
        let mut buffer = Cursor::new(Vec::new());
        original.write(&mut buffer).unwrap();

        // Read back
        buffer.set_position(0);
        let read_back = PacketHeader::read(&mut buffer).unwrap();

        // Verify all fields match
        assert_eq!(read_back.sequence, original.sequence);
        assert_eq!(read_back.flags, original.flags);
        assert_eq!(read_back.checksum, original.checksum);
        assert_eq!(read_back.id, original.id);
        assert_eq!(read_back.time, original.time);
        assert_eq!(read_back.size, original.size);
        assert_eq!(read_back.iteration, original.iteration);
    }

    #[test]
    fn test_packet_header_with_flags() {
        let flags = PacketHeaderFlags::BLOB_FRAGMENTS | PacketHeaderFlags::RETRANSMISSION;
        let header = PacketHeader::with_flags(flags);

        assert_eq!(header.flags, flags);
        assert_eq!(header.sequence, 0);
        assert_eq!(header.checksum, 0);
        assert_eq!(header.id, 0);
        assert_eq!(header.time, 0);
        assert_eq!(header.size, 0);
        assert_eq!(header.iteration, 0);
    }
}
