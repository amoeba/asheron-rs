use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    error::Error,
    io::{Cursor, Read, Seek, SeekFrom, Write},
};

#[cfg(feature = "dat-tokio")]
use crate::dat::reader::range_reader::RangeReader;

#[derive(Debug)]
pub struct DatBlockReader {}

impl DatBlockReader {
    pub fn read<R: Read + Seek>(
        reader: &mut R,
        offset: u32,
        size: u32,
        block_size: u32,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        reader.seek(SeekFrom::Start(offset as u64))?;

        let mut buffer = vec![0; size as usize];
        let mut writer = Cursor::new(&mut buffer);
        let mut left_to_read = size;
        let mut next_address = reader.read_u32::<LittleEndian>()?;

        while left_to_read > 0 {
            if left_to_read < block_size {
                let mut data: Vec<u8> = vec![0; left_to_read as usize];
                reader.read_exact(&mut data)?;
                writer.write_all(&data)?;
                break;
            } else {
                let mut data: Vec<u8> = vec![0; (block_size as usize) - 4];
                reader.read_exact(&mut data)?;
                writer.write_all(&data)?;
                reader.seek(SeekFrom::Start(next_address as u64))?;
                next_address = reader.read_u32::<LittleEndian>()?;
                left_to_read -= block_size - 4;
            }
        }
        Ok(buffer)
    }

    #[cfg(feature = "dat-tokio")]
    pub async fn read_async<R: RangeReader>(
        reader: &mut R,
        offset: u32,
        size: u32,
        block_size: u32,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = vec![0; size as usize];
        let mut writer = Cursor::new(&mut buffer);
        let mut left_to_read = size;

        // Read the first 4 bytes to get the next address
        let first_chunk = reader.read_range(offset, 4).await?;
        let mut cursor = Cursor::new(first_chunk);
        let mut next_address = cursor.read_u32::<LittleEndian>()?;
        let mut current_offset = offset;

        while left_to_read > 0 {
            if left_to_read < block_size {
                // Read the remaining data
                let data = reader
                    .read_range(current_offset + 4, left_to_read as usize)
                    .await?;
                writer.write_all(&data)?;
                break;
            } else {
                // Read the data part of this block (excluding the 4-byte pointer)
                let data_size = (block_size as usize) - 4;
                let data = reader.read_range(current_offset + 4, data_size).await?;
                writer.write_all(&data)?;

                // Move to the next block and read its pointer
                current_offset = next_address;
                let next_chunk = reader.read_range(current_offset, 4).await?;
                let mut cursor = Cursor::new(next_chunk);
                next_address = cursor.read_u32::<LittleEndian>()?;

                left_to_read -= block_size - 4;
            }
        }
        Ok(buffer)
    }
}
