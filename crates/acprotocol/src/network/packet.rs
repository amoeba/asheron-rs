use serde::Serialize;
use std::io::{self, Read, Seek};

#[cfg(test)]
use std::io::Cursor;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
    pub struct PacketHeaderFlags: u32 {
        const NONE = 0x00000000;
        const RETRANSMISSION = 0x00000001;
        const ENCRYPTED_CHECKSUM = 0x00000002;
        const BLOB_FRAGMENTS = 0x00000004;
        const SERVER_SWITCH = 0x00000100;
        const LOGON_SERVER_ADDR = 0x00000200;
        const EMPTY_HEADER1 = 0x00000400;
        const REFERRAL = 0x00000800;
        const REQUEST_RETRANSMIT = 0x00001000;
        const REJECT_RETRANSMIT = 0x00002000;
        const ACK_SEQUENCE = 0x00004000;
        const DISCONNECT = 0x00008000;
        const LOGIN_REQUEST = 0x00010000;
        const WORLD_LOGIN_REQUEST = 0x00020000;
        const CONNECT_REQUEST = 0x00040000;
        const CONNECT_RESPONSE = 0x00080000;
        const NET_ERROR = 0x00100000;
        const NET_ERROR_DISCONNECT = 0x00200000;
        const CICMD_COMMAND = 0x00400000;
        const TIME_SYNC = 0x01000000;
        const ECHO_REQUEST = 0x02000000;
        const ECHO_RESPONSE = 0x04000000;
        const FLOW = 0x08000000;
    }
}

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

    pub fn parse(reader: &mut (impl Read + Seek)) -> io::Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let sequence = u32::from_le_bytes(buf);

        reader.read_exact(&mut buf)?;
        let flags_raw = u32::from_le_bytes(buf);
        let flags = PacketHeaderFlags::from_bits_truncate(flags_raw);

        reader.read_exact(&mut buf)?;
        let checksum = u32::from_le_bytes(buf);

        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        let id = u16::from_le_bytes(buf);

        reader.read_exact(&mut buf)?;
        let time = u16::from_le_bytes(buf);

        reader.read_exact(&mut buf)?;
        let size = u16::from_le_bytes(buf);

        reader.read_exact(&mut buf)?;
        let iteration = u16::from_le_bytes(buf);

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
        let header = PacketHeader::parse(&mut cursor).unwrap();

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
        let header = PacketHeader::parse(&mut cursor).unwrap();

        assert!(header.flags.contains(PacketHeaderFlags::BLOB_FRAGMENTS));
        assert!(header.flags.contains(PacketHeaderFlags::ACK_SEQUENCE));
        assert_eq!(header.size, 100);
    }

    #[test]
    fn test_packet_header_parse_insufficient_data() {
        // Only 10 bytes when 20 are needed
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];

        let mut cursor = Cursor::new(&data[..]);
        let result = PacketHeader::parse(&mut cursor);

        assert!(result.is_err());
    }

    #[test]
    fn test_packet_header_base_size() {
        assert_eq!(PacketHeader::BASE_SIZE, 20);
    }
}
