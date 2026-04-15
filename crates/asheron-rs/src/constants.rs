/// Network packet header size (Ethernet + IP + UDP combined)
pub const PACKET_HEADER_SIZE: usize = 42;

/// Byte offset for UDP destination port in raw packet data
pub const UDP_DEST_PORT_OFFSET: usize = 36;
