use super::reader::BinaryReader;
use std::io;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
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

    pub fn parse(reader: &mut BinaryReader) -> io::Result<Self> {
        let sequence = reader.read_u32()?;
        let flags_raw = reader.read_u32()?;
        let flags = PacketHeaderFlags::from_bits_truncate(flags_raw);
        let checksum = reader.read_u32()?;
        let id = reader.read_u16()?;
        let time = reader.read_u16()?;
        let size = reader.read_u16()?;
        let iteration = reader.read_u16()?;

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
