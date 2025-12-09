use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;

// Client to Server AC packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct C2SPacket {
    #[serde(rename = "Sequence")]
    pub sequence: u32,
    #[serde(rename = "Flags")]
    pub flags: PacketHeaderFlags,
    #[serde(rename = "Checksum")]
    pub checksum: u32,
    #[serde(rename = "RecipientId")]
    pub recipient_id: u16,
    #[serde(rename = "TimeSinceLastPacket")]
    pub time_since_last_packet: u16,
    #[serde(rename = "Size")]
    pub size: u16,
    #[serde(rename = "Iteration")]
    pub iteration: u16,
    #[serde(rename = "ServerSwitch")]
    pub server_switch: Option<ServerSwitchHeader>,
    #[serde(rename = "RetransmitSequences")]
    pub retransmit_sequences: Option<PackableList<uint>>,
    #[serde(rename = "RejectSequences")]
    pub reject_sequences: Option<PackableList<uint>>,
    #[serde(rename = "AckSequence")]
    pub ack_sequence: Option<u32>,
    #[serde(rename = "LoginRequest")]
    pub login_request: Option<LoginRequestHeader>,
    #[serde(rename = "WorldLoginRequest")]
    pub world_login_request: Option<u64>,
    #[serde(rename = "ConnectResponse")]
    pub connect_response: Option<u64>,
    #[serde(rename = "CICMDCommand")]
    pub cicmd_command: Option<CICMDCommandHeader>,
    #[serde(rename = "Time")]
    pub time: Option<u64>,
    #[serde(rename = "EchoTime")]
    pub echo_time: Option<f32>,
    #[serde(rename = "Flow")]
    pub flow: Option<FlowHeader>,
    #[serde(rename = "Fragments")]
    pub fragments: Option<BlobFragments>,
}

impl C2SPacket {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u32(reader)?;
        let flags = Ok::<_, Box<dyn std::error::Error>>(PacketHeaderFlags::from_bits_retain(read_u32(reader)?))?;
        let checksum = read_u32(reader)?;
        let recipient_id = read_u16(reader)?;
        let time_since_last_packet = read_u16(reader)?;
        let size = read_u16(reader)?;
        let iteration = read_u16(reader)?;
        let mut server_switch = None;
        if (flags.bits() & PacketHeaderFlags::SERVER_SWITCH.bits()) != 0 {
            server_switch = Some(ServerSwitchHeader::read(reader)?);
        }
        let mut retransmit_sequences = None;
        if (flags.bits() & PacketHeaderFlags::REQUEST_RETRANSMIT.bits()) != 0 {
            retransmit_sequences = Some(read_packable_list::<u32>(reader)?);
        }
        let mut reject_sequences = None;
        if (flags.bits() & PacketHeaderFlags::REJECT_RETRANSMIT.bits()) != 0 {
            reject_sequences = Some(read_packable_list::<u32>(reader)?);
        }
        let mut ack_sequence = None;
        if (flags.bits() & PacketHeaderFlags::ACK_SEQUENCE.bits()) != 0 {
            ack_sequence = Some(read_u32(reader)?);
        }
        let mut login_request = None;
        if (flags.bits() & PacketHeaderFlags::LOGIN_REQUEST.bits()) != 0 {
            login_request = Some(LoginRequestHeader::read(reader)?);
        }
        let mut world_login_request = None;
        if (flags.bits() & PacketHeaderFlags::WORLD_LOGIN_REQUEST.bits()) != 0 {
            world_login_request = Some(read_u64(reader)?);
        }
        let mut connect_response = None;
        if (flags.bits() & PacketHeaderFlags::CONNECT_RESPONSE.bits()) != 0 {
            connect_response = Some(read_u64(reader)?);
        }
        let mut cicmd_command = None;
        if (flags.bits() & PacketHeaderFlags::CICMDCOMMAND.bits()) != 0 {
            cicmd_command = Some(CICMDCommandHeader::read(reader)?);
        }
        let mut time = None;
        if (flags.bits() & PacketHeaderFlags::TIME_SYNC.bits()) != 0 {
            time = Some(read_u64(reader)?);
        }
        let mut echo_time = None;
        if (flags.bits() & PacketHeaderFlags::ECHO_REQUEST.bits()) != 0 {
            echo_time = Some(read_f32(reader)?);
        }
        let mut flow = None;
        if (flags.bits() & PacketHeaderFlags::FLOW.bits()) != 0 {
            flow = Some(FlowHeader::read(reader)?);
        }
        let mut fragments = None;
        if (flags.bits() & PacketHeaderFlags::BLOB_FRAGMENTS.bits()) != 0 {
            fragments = Some(BlobFragments::read(reader)?);
        }

        Ok(Self {
            sequence,
            flags,
            checksum,
            recipient_id,
            time_since_last_packet,
            size,
            iteration,
            server_switch,
            retransmit_sequences,
            reject_sequences,
            ack_sequence,
            login_request,
            world_login_request,
            connect_response,
            cicmd_command,
            time,
            echo_time,
            flow,
            fragments,
        })
    }
}

impl crate::readers::ACDataType for C2SPacket {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        C2SPacket::read(reader)
    }
}

