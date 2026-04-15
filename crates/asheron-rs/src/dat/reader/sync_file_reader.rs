use std::io::{Read, Seek, SeekFrom};

use crate::dat::reader::range_reader::RangeReaderSync;

pub struct SyncFileRangeReader<R> {
    reader: R,
}

impl<R> SyncFileRangeReader<R>
where
    R: Read + Seek,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> RangeReaderSync for SyncFileRangeReader<R>
where
    R: Read + Seek,
{
    fn read_range(
        &mut self,
        offset: u32,
        length: usize,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Seek to the position
        self.reader.seek(SeekFrom::Start(offset.into()))?;

        // Read exactly the requested bytes
        let mut buffer = vec![0u8; length];
        self.reader.read_exact(&mut buffer)?;

        Ok(buffer)
    }
}
