use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "S2CPacket").entered();

        #[cfg(feature = "tracing")]
        let _field_span_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Sequence", position = pos).entered()
        };
        let sequence = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = Ok::<_, Box<dyn std::error::Error>>(PacketHeaderFlags::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_checksum = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Checksum", position = pos).entered()
        };
        let checksum = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_checksum);
        #[cfg(feature = "tracing")]
        let _field_span_recipient_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "RecipientId", position = pos).entered()
        };
        let recipient_id = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_recipient_id);
        #[cfg(feature = "tracing")]
        let _field_span_time_since_last_packet = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TimeSinceLastPacket", position = pos).entered()
        };
        let time_since_last_packet = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_time_since_last_packet);
        #[cfg(feature = "tracing")]
        let _field_span_size = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Size", position = pos).entered()
        };
        let size = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_size);
        #[cfg(feature = "tracing")]
        let _field_span_iteration = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Iteration", position = pos).entered()
        };
        let iteration = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_iteration);
        let mut ack_sequence = None;
        if (flags.bits() & PacketHeaderFlags::ACK_SEQUENCE.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_ack_sequence = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "AckSequence", position = pos).entered()
            };
            ack_sequence = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_ack_sequence);
        }
        let mut logon_server_addr = None;
        if (flags.bits() & PacketHeaderFlags::LOGON_SERVER_ADDR.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_logon_server_addr = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "LogonServerAddr", position = pos).entered()
            };
            logon_server_addr = Some(SocketAddress::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_logon_server_addr);
        }
        let mut referral = None;
        if (flags.bits() & PacketHeaderFlags::REFERRAL.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_referral = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Referral", position = pos).entered()
            };
            referral = Some(ReferralHeader::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_referral);
        }
        let mut connect_request = None;
        if (flags.bits() & PacketHeaderFlags::CONNECT_REQUEST.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_connect_request = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ConnectRequest", position = pos).entered()
            };
            connect_request = Some(ConnectRequestHeader::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_connect_request);
        }
        let mut net_error = None;
        if (flags.bits() & PacketHeaderFlags::NET_ERROR.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_net_error = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "NetError", position = pos).entered()
            };
            net_error = Some(NetError::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_net_error);
        }
        let mut net_error_disconnect = None;
        if (flags.bits() & PacketHeaderFlags::NET_ERROR_DISCONNECT.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_net_error_disconnect = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "NetErrorDisconnect", position = pos).entered()
            };
            net_error_disconnect = Some(NetError::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_net_error_disconnect);
        }
        let mut echo_response = None;
        if (flags.bits() & PacketHeaderFlags::ECHO_RESPONSE.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_echo_response = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "EchoResponse", position = pos).entered()
            };
            echo_response = Some(EchoResponseHeader::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_echo_response);
        }
        let mut fragments = None;
        if (flags.bits() & PacketHeaderFlags::BLOB_FRAGMENTS.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_fragments = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Fragments", position = pos).entered()
            };
            fragments = Some(BlobFragments::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_fragments);
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

impl crate::writers::ACWritable for S2CPacket {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "S2CPacket").entered();

        write_u32(writer, self.sequence)?;
        write_u32(writer, self.flags.bits())?;
        write_u32(writer, self.checksum)?;
        write_u16(writer, self.recipient_id)?;
        write_u16(writer, self.time_since_last_packet)?;
        write_u16(writer, self.size)?;
        write_u16(writer, self.iteration)?;
        if (self.flags.bits() & PacketHeaderFlags::ACK_SEQUENCE.bits()) != 0
            && let Some(ref value) = self.ack_sequence {
                write_u32(writer, *value)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::LOGON_SERVER_ADDR.bits()) != 0
            && let Some(ref value) = self.logon_server_addr {
                value.write(writer)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::REFERRAL.bits()) != 0
            && let Some(ref value) = self.referral {
                value.write(writer)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::CONNECT_REQUEST.bits()) != 0
            && let Some(ref value) = self.connect_request {
                value.write(writer)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::NET_ERROR.bits()) != 0
            && let Some(ref value) = self.net_error {
                value.write(writer)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::NET_ERROR_DISCONNECT.bits()) != 0
            && let Some(ref value) = self.net_error_disconnect {
                value.write(writer)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::ECHO_RESPONSE.bits()) != 0
            && let Some(ref value) = self.echo_response {
                value.write(writer)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::BLOB_FRAGMENTS.bits()) != 0
            && let Some(ref value) = self.fragments {
                value.write(writer)?;
            }
        Ok(())
    }
}

