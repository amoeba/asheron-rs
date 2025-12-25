use std::collections::HashMap;
use std::io;

use crate::generated::network::Fragment;
use crate::readers::ACDataType;

use super::packet::PacketHeader;
use super::packet_reader::PacketReader;
use super::raw_message::RawMessage;
use crate::enums::PacketHeaderFlags;

/// Information about a fragment extracted from a packet
#[derive(Debug, Clone)]
pub struct ExtractedFragment {
    pub sequence: u32,
    pub id: u32,
    pub index: u16,
    pub count: u16,
    pub data: Vec<u8>,
    pub is_complete: bool,
}

/// Parses packets and assembles fragments into complete messages
pub struct FragmentAssembler {
    pending_fragments: HashMap<u32, Fragment>,
    next_message_id: u32,
}

impl FragmentAssembler {
    pub fn new() -> Self {
        Self {
            pending_fragments: HashMap::new(),
            next_message_id: 0,
        }
    }

    /// Parse a network packet's payload and extract fragments, returning any completed messages
    pub fn parse_packet_payload(&mut self, payload: &[u8]) -> io::Result<Vec<RawMessage>> {
        // PCAP packets include the full network stack: Ethernet (14) + IP (20) + UDP (8) = 42 bytes
        // We need to skip these headers to get to the AC protocol payload
        let ac_payload = if payload.len() > 42 {
            &payload[42..]
        } else {
            payload
        };

        let mut completed_messages = Vec::new();
        let mut reader = PacketReader::new(ac_payload);

        while reader.remaining() > 0 {
            let start_pos = reader.position();

            // Parse packet header
            let header = PacketHeader::read(&mut reader)
                .map_err(|e| io::Error::other(e.to_string()))?;

            // Calculate packet boundaries (header is always 20 bytes + variable size payload)
            let packet_end = start_pos + PacketHeader::BASE_SIZE + header.size as usize;

            // Parse optional headers based on flags
            // NOTE: We must parse ALL optional headers to advance reader correctly!
            if header.flags.contains(PacketHeaderFlags::SERVER_SWITCH) {
                reader.read_u32()?; // SeqNo
                reader.read_u32()?; // Type
            }
            if header.flags.contains(PacketHeaderFlags::LOGON_SERVER_ADDR) {
                reader.read_u16()?; // Family
                reader.read_u16()?; // Port
                reader.read_bytes(4)?; // Address (IPv4)
                reader.read_bytes(8)?; // Zero padding
            }
            if header.flags.contains(PacketHeaderFlags::REQUEST_RETRANSMIT) {
                let num = reader.read_u32()?;
                for _ in 0..num {
                    reader.read_u32()?; // sequence id
                }
            }
            if header.flags.contains(PacketHeaderFlags::REFERRAL) {
                reader.read_bytes(8)?; // Cookie (u64)
                // SocketAddr: Family (i16), Port (u16), Address (4 bytes), Zero (8 bytes)
                reader.read_u16()?; // Family
                reader.read_u16()?; // Port
                reader.read_bytes(4)?; // Address
                reader.read_bytes(8)?; // Zero padding
                reader.read_u16()?; // IdServer
                reader.read_u16()?; // Padding
                reader.read_u32()?; // Unknown
            }
            if header.flags.contains(PacketHeaderFlags::ACK_SEQUENCE) {
                reader.read_u32()?; // sequence
            }
            if header.flags.contains(PacketHeaderFlags::LOGIN_REQUEST) {
                // LoginRequest has variable-length strings - skip the entire payload
                // as we can't reliably parse it without proper error recovery.
                // C# implementation parses these fields but we don't need them for fragment assembly.
                // Jump to the end of this packet's data.
                reader.set_position(packet_end);
                // No fragments can follow LOGIN_REQUEST in the same packet
                continue;
            }
            if header
                .flags
                .contains(PacketHeaderFlags::WORLD_LOGIN_REQUEST)
            {
                reader.read_bytes(8)?; // Prim (u64)
            }
            if header.flags.contains(PacketHeaderFlags::CONNECT_REQUEST) {
                reader.read_bytes(8)?; // ServerTime (u64)
                reader.read_bytes(8)?; // Cookie (u64)
                reader.read_u32()?; // NetID
                reader.read_u32()?; // OutgoingSeed
                reader.read_u32()?; // IncomingSeed
                reader.read_u32()?; // Unknown
            }
            if header.flags.contains(PacketHeaderFlags::CONNECT_RESPONSE) {
                reader.read_bytes(8)?; // Prim (u64)
            }
            if header.flags.contains(PacketHeaderFlags::NET_ERROR) {
                reader.read_u32()?; // StringId
                reader.read_u32()?; // TableId
            }
            if header
                .flags
                .contains(PacketHeaderFlags::NET_ERROR_DISCONNECT)
            {
                reader.read_u32()?; // StringId
                reader.read_u32()?; // TableId
            }
            if header.flags.contains(PacketHeaderFlags::CICMDCOMMAND) {
                // CICMDCommand: Command (u32) + Param (u32)
                reader.read_u32()?; // Command
                reader.read_u32()?; // Param
            }
            if header.flags.contains(PacketHeaderFlags::TIME_SYNC) {
                reader.read_bytes(8)?; // Time (f64)
            }
            if header.flags.contains(PacketHeaderFlags::ECHO_REQUEST) {
                reader.read_bytes(4)?; // LocalTime (f32)
            }
            if header.flags.contains(PacketHeaderFlags::ECHO_RESPONSE) {
                reader.read_bytes(4)?; // LocalTime (f32)
                reader.read_bytes(4)?; // HoldingTime (f32)
            }
            if header.flags.contains(PacketHeaderFlags::FLOW) {
                reader.read_u32()?; // DataReceived
                reader.read_u16()?; // Interval
            }

            // If this packet has fragments, parse them
            if header.flags.contains(PacketHeaderFlags::BLOB_FRAGMENTS) {
                let header_flags = header.flags.bits();
                while reader.position() < packet_end && reader.remaining() > 0 {
                    match self.parse_fragment_internal(
                        &mut reader,
                        Some(header.iteration),
                        Some(header_flags),
                    ) {
                        Ok(Some(msg)) => {
                            completed_messages.push(msg);
                        }
                        Ok(None) => {
                            // Fragment received but not complete yet
                        }
                        Err(_e) => {
                            // Fragment parsing failed - skip to end of packet like C# does
                            // C# catches exceptions in ParseFragment and returns null,
                            // then the outer loop advances to the next packet
                            break; // Break to next packet
                        }
                    }
                }
            }

            // Move to next packet
            if reader.position() < packet_end {
                reader.set_position(packet_end);
            }
        }

        Ok(completed_messages)
    }

    /// Parse a single fragment from the reader
    /// Returns Some(RawMessage) if the fragment completes a message, None otherwise
    fn parse_fragment_internal(
        &mut self,
        reader: &mut PacketReader,
        packet_iteration: Option<u16>,
        header_flags: Option<u32>,
    ) -> io::Result<Option<RawMessage>> {
        let sequence = reader.read_u32()?;
        let id = reader.read_u32()?;
        let count = reader.read_u16()?;
        let size = reader.read_u16()?;
        let index = reader.read_u16()?;
        let group = reader.read_u16()?;

        // Calculate fragment data length (size includes 16-byte header)
        let frag_length = size.saturating_sub(16) as usize;

        if reader.remaining() < frag_length {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Fragment data too short",
            ));
        }

        let data = reader.read_bytes(frag_length)?;

        // Update or create fragment entry
        let fragment = self
            .pending_fragments
            .entry(sequence)
            .or_insert_with(|| Fragment::new(sequence, count));

        fragment.add_chunk(&data, index as usize, frag_length); // Pass chunk size
        fragment.header.id = id;
        fragment.header.index = index;
        fragment.set_fragment_info(size, group);

        // Check if this completes the fragment assembly
        if fragment.is_complete() {
            let assembled_data = fragment.get_data().to_vec();
            fragment.cleanup();
            self.pending_fragments.remove(&sequence);

            // Try to parse as a message
            let msg_id = self.next_message_id;
            self.next_message_id += 1;

            let parsed_msg = RawMessage::from_fragment_with_iteration(
                assembled_data,
                sequence,
                msg_id,
                packet_iteration,
                header_flags,
            )?;
            Ok(Some(parsed_msg))
        } else {
            Ok(None)
        }
    }

    /// Parse a single fragment from the reader
    /// Returns Some(RawMessage) if the fragment completes a message, None otherwise
    #[allow(dead_code)]
    fn parse_fragment(&mut self, reader: &mut PacketReader) -> io::Result<Option<RawMessage>> {
        self.parse_fragment_internal(reader, None, None)
    }
}

impl Default for FragmentAssembler {
    fn default() -> Self {
        Self::new()
    }
}
