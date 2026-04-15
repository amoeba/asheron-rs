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

// Client to Server AC packet.
#[allow(dead_code)]
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
    #[serde(rename = "ServerSwitch", skip_serializing_if = "Option::is_none")]
    pub server_switch: Option<ServerSwitchHeader>,
    #[serde(rename = "RetransmitSequences", skip_serializing_if = "Option::is_none")]
    pub retransmit_sequences: Option<PackableList<u32>>,
    #[serde(rename = "RejectSequences", skip_serializing_if = "Option::is_none")]
    pub reject_sequences: Option<PackableList<u32>>,
    #[serde(rename = "AckSequence", skip_serializing_if = "Option::is_none")]
    pub ack_sequence: Option<u32>,
    #[serde(rename = "LoginRequest", skip_serializing_if = "Option::is_none")]
    pub login_request: Option<LoginRequestHeader>,
    #[serde(rename = "WorldLoginRequest", skip_serializing_if = "Option::is_none")]
    pub world_login_request: Option<u64>,
    #[serde(rename = "ConnectResponse", skip_serializing_if = "Option::is_none")]
    pub connect_response: Option<u64>,
    #[serde(rename = "CICMDCommand", skip_serializing_if = "Option::is_none")]
    pub cicmd_command: Option<CICMDCommandHeader>,
    #[serde(rename = "Time", skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(rename = "EchoTime", skip_serializing_if = "Option::is_none")]
    pub echo_time: Option<f32>,
    #[serde(rename = "Flow", skip_serializing_if = "Option::is_none")]
    pub flow: Option<FlowHeader>,
    #[serde(rename = "Fragments", skip_serializing_if = "Option::is_none")]
    pub fragments: Option<BlobFragments>,
}

impl crate::readers::ACDataType for C2SPacket {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "C2SPacket").entered();

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
        let mut server_switch = None;
        if (flags.bits() & PacketHeaderFlags::SERVER_SWITCH.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_server_switch = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ServerSwitch", position = pos).entered()
            };
            server_switch = Some(ServerSwitchHeader::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_server_switch);
        }
        let mut retransmit_sequences = None;
        if (flags.bits() & PacketHeaderFlags::REQUEST_RETRANSMIT.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_retransmit_sequences = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "RetransmitSequences", position = pos).entered()
            };
            retransmit_sequences = Some(read_packable_list::<u32>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_retransmit_sequences);
        }
        let mut reject_sequences = None;
        if (flags.bits() & PacketHeaderFlags::REJECT_RETRANSMIT.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_reject_sequences = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "RejectSequences", position = pos).entered()
            };
            reject_sequences = Some(read_packable_list::<u32>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_reject_sequences);
        }
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
        let mut login_request = None;
        if (flags.bits() & PacketHeaderFlags::LOGIN_REQUEST.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_login_request = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "LoginRequest", position = pos).entered()
            };
            login_request = Some(LoginRequestHeader::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_login_request);
        }
        let mut world_login_request = None;
        if (flags.bits() & PacketHeaderFlags::WORLD_LOGIN_REQUEST.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_world_login_request = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "WorldLoginRequest", position = pos).entered()
            };
            world_login_request = Some(read_u64(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_world_login_request);
        }
        let mut connect_response = None;
        if (flags.bits() & PacketHeaderFlags::CONNECT_RESPONSE.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_connect_response = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ConnectResponse", position = pos).entered()
            };
            connect_response = Some(read_u64(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_connect_response);
        }
        let mut cicmd_command = None;
        if (flags.bits() & PacketHeaderFlags::CICMDCOMMAND.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_cicmd_command = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CICMDCommand", position = pos).entered()
            };
            cicmd_command = Some(CICMDCommandHeader::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_cicmd_command);
        }
        let mut time = None;
        if (flags.bits() & PacketHeaderFlags::TIME_SYNC.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_time = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Time", position = pos).entered()
            };
            time = Some(read_u64(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_time);
        }
        let mut echo_time = None;
        if (flags.bits() & PacketHeaderFlags::ECHO_REQUEST.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_echo_time = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "EchoTime", position = pos).entered()
            };
            echo_time = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_echo_time);
        }
        let mut flow = None;
        if (flags.bits() & PacketHeaderFlags::FLOW.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_flow = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Flow", position = pos).entered()
            };
            flow = Some(FlowHeader::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_flow);
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

impl crate::writers::ACWritable for C2SPacket {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "C2SPacket").entered();

        write_u32(writer, self.sequence)?;
        write_u32(writer, self.flags.bits())?;
        write_u32(writer, self.checksum)?;
        write_u16(writer, self.recipient_id)?;
        write_u16(writer, self.time_since_last_packet)?;
        write_u16(writer, self.size)?;
        write_u16(writer, self.iteration)?;
        if (self.flags.bits() & PacketHeaderFlags::SERVER_SWITCH.bits()) != 0
            && let Some(ref value) = self.server_switch {
                value.write(writer)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::REQUEST_RETRANSMIT.bits()) != 0
            && let Some(ref value) = self.retransmit_sequences {
                write_packable_list::<u32>(writer, value)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::REJECT_RETRANSMIT.bits()) != 0
            && let Some(ref value) = self.reject_sequences {
                write_packable_list::<u32>(writer, value)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::ACK_SEQUENCE.bits()) != 0
            && let Some(ref value) = self.ack_sequence {
                write_u32(writer, *value)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::LOGIN_REQUEST.bits()) != 0
            && let Some(ref value) = self.login_request {
                value.write(writer)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::WORLD_LOGIN_REQUEST.bits()) != 0
            && let Some(ref value) = self.world_login_request {
                write_u64(writer, *value)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::CONNECT_RESPONSE.bits()) != 0
            && let Some(ref value) = self.connect_response {
                write_u64(writer, *value)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::CICMDCOMMAND.bits()) != 0
            && let Some(ref value) = self.cicmd_command {
                value.write(writer)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::TIME_SYNC.bits()) != 0
            && let Some(ref value) = self.time {
                write_u64(writer, *value)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::ECHO_REQUEST.bits()) != 0
            && let Some(ref value) = self.echo_time {
                write_f32(writer, *value)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::FLOW.bits()) != 0
            && let Some(ref value) = self.flow {
                value.write(writer)?;
            }
        if (self.flags.bits() & PacketHeaderFlags::BLOB_FRAGMENTS.bits()) != 0
            && let Some(ref value) = self.fragments {
                value.write(writer)?;
            }
        Ok(())
    }
}

