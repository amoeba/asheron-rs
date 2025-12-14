use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

// Server to Client AC packet.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct S2CPacket {
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
    #[serde(rename = "AckSequence", skip_serializing_if = "Option::is_none")]
    pub ack_sequence: Option<u32>,
    #[serde(rename = "LogonServerAddr", skip_serializing_if = "Option::is_none")]
    pub logon_server_addr: Option<SocketAddress>,
    #[serde(rename = "Referral", skip_serializing_if = "Option::is_none")]
    pub referral: Option<ReferralHeader>,
    #[serde(rename = "ConnectRequest", skip_serializing_if = "Option::is_none")]
    pub connect_request: Option<ConnectRequestHeader>,
    #[serde(rename = "NetError", skip_serializing_if = "Option::is_none")]
    pub net_error: Option<NetError>,
    #[serde(rename = "NetErrorDisconnect", skip_serializing_if = "Option::is_none")]
    pub net_error_disconnect: Option<NetError>,
    #[serde(rename = "EchoResponse", skip_serializing_if = "Option::is_none")]
    pub echo_response: Option<EchoResponseHeader>,
    #[serde(rename = "Fragments", skip_serializing_if = "Option::is_none")]
    pub fragments: Option<BlobFragments>,
}

impl crate::readers::ACDataType for S2CPacket {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u32(reader)?;
        let flags = Ok::<_, Box<dyn std::error::Error>>(PacketHeaderFlags::from_bits_retain(read_u32(reader)?))?;
        let checksum = read_u32(reader)?;
        let recipient_id = read_u16(reader)?;
        let time_since_last_packet = read_u16(reader)?;
        let size = read_u16(reader)?;
        let iteration = read_u16(reader)?;
        let mut ack_sequence = None;
        if (flags.bits() & PacketHeaderFlags::ACK_SEQUENCE.bits()) != 0 {
            ack_sequence = Some(read_u32(reader)?);
        }
        let mut logon_server_addr = None;
        if (flags.bits() & PacketHeaderFlags::LOGON_SERVER_ADDR.bits()) != 0 {
            logon_server_addr = Some(SocketAddress::read(reader)?);
        }
        let mut referral = None;
        if (flags.bits() & PacketHeaderFlags::REFERRAL.bits()) != 0 {
            referral = Some(ReferralHeader::read(reader)?);
        }
        let mut connect_request = None;
        if (flags.bits() & PacketHeaderFlags::CONNECT_REQUEST.bits()) != 0 {
            connect_request = Some(ConnectRequestHeader::read(reader)?);
        }
        let mut net_error = None;
        if (flags.bits() & PacketHeaderFlags::NET_ERROR.bits()) != 0 {
            net_error = Some(NetError::read(reader)?);
        }
        let mut net_error_disconnect = None;
        if (flags.bits() & PacketHeaderFlags::NET_ERROR_DISCONNECT.bits()) != 0 {
            net_error_disconnect = Some(NetError::read(reader)?);
        }
        let mut echo_response = None;
        if (flags.bits() & PacketHeaderFlags::ECHO_RESPONSE.bits()) != 0 {
            echo_response = Some(EchoResponseHeader::read(reader)?);
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
            ack_sequence,
            logon_server_addr,
            referral,
            connect_request,
            net_error,
            net_error_disconnect,
            echo_response,
            fragments,
        })
    }
}

