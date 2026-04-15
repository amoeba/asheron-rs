#[derive(Debug)]
pub struct DatBlock {
    /// Pointer to the next block (8 bytes)
    pub next_block_offset: u32,
    /// The actual data content of this block
    pub data: Vec<u8>,
}
