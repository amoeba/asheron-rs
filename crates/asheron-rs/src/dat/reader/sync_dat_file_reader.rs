use crate::dat::reader::{range_reader::RangeReaderSync, types::dat_block::DatBlock};

/// A synchronous entry point for reading block-based files
#[derive(Debug)]
pub struct SyncDatFileReader {
    pub size: usize,
    pub block_size: usize,
    pub left_to_read: usize,
}

impl SyncDatFileReader {
    pub fn new(size: usize, block_size: usize) -> Result<Self, Box<dyn std::error::Error>> {
        if block_size < 4 {
            return Err("block_size must be at least 4 bytes (for the pointer)".into());
        }
        Ok(Self {
            size,
            block_size,
            left_to_read: size,
        })
    }

    /// Read a file starting at the given offset for the specified total size
    pub fn read_file<R>(
        &mut self,
        reader: &mut R,
        start_offset: u32,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    where
        R: RangeReaderSync,
    {
        let mut buffer = Vec::with_capacity(self.size);
        let mut next_address = start_offset;

        while self.left_to_read > 0 {
            let block = self.read_block(reader, next_address)?;
            buffer.extend_from_slice(&block.data);

            if self.left_to_read > 0 {
                next_address = block.next_block_offset
            }
        }

        Ok(buffer)
    }

    /// Read a single block from the given offset
    ///
    /// A block is [ next_offset, data ]. For the last block, the next_offset
    /// is 0
    fn read_block<R>(
        &mut self,
        reader: &mut R,
        offset: u32,
    ) -> Result<DatBlock, Box<dyn std::error::Error>>
    where
        R: RangeReaderSync,
    {
        // Determine the size of the next read. This is either an entire block
        // when we have more than a block worth of data to read or whatever is
        // left to read (+ 4 bytes for the pointer).
        let next_read_size = std::cmp::min(self.block_size, self.left_to_read + 4);
        self.left_to_read -= next_read_size - 4;

        // Dispatch the read to the underlying read implementation
        let block_data = reader.read_range(offset, next_read_size)?;

        if block_data.len() < 4 {
            return Err("Block too small to contain pointer".into());
        }

        // Create and return the DatBlock
        let next_offset_bytes: [u8; 4] = block_data[0..4].try_into()?;
        let next_block_offset = u32::from_le_bytes(next_offset_bytes);
        let data: Vec<u8> = block_data[4..].to_vec();

        Ok(DatBlock {
            next_block_offset,
            data,
        })
    }
}
