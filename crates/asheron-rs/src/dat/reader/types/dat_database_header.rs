use std::{
    error::Error,
    io::{Read, Seek, SeekFrom},
};

use byteorder::{LittleEndian, ReadBytesExt};

#[cfg(feature = "dat-tokio")]
use crate::dat::reader::range_reader::RangeReader;

pub const DAT_HEADER_OFFSET: u64 = 0x140;

#[derive(Debug)]
pub struct DatDatabaseHeader {
    pub file_type: u32,
    pub block_size: u32,
    pub file_size: u32,
    pub data_set: u32,
    pub data_subset: u32,
    pub free_head: u32,
    pub free_tail: u32,
    pub free_count: u32,
    pub btree: u32,
    pub new_lru: u32,
    pub old_lru: u32,
    pub use_lru: bool,
    pub master_map_id: u32,
    pub engine_pack_version: u32,
    pub game_pack_version: u32,
    pub version_major: Vec<u8>,
    pub version_minor: u32,
}

impl DatDatabaseHeader {
    /// Common function to parse header data from a buffer
    fn from_buffer<R: Read>(reader: &mut R) -> Result<DatDatabaseHeader, Box<dyn Error>> {
        let file_type = reader.read_u32::<LittleEndian>()?;
        let block_size = reader.read_u32::<LittleEndian>()?;
        let file_size = reader.read_u32::<LittleEndian>()?;
        let data_set = reader.read_u32::<LittleEndian>()?;
        let data_subset = reader.read_u32::<LittleEndian>()?;
        let free_head = reader.read_u32::<LittleEndian>()?;
        let free_tail = reader.read_u32::<LittleEndian>()?;
        let free_count = reader.read_u32::<LittleEndian>()?;
        let btree = reader.read_u32::<LittleEndian>()?;
        let new_lru = reader.read_u32::<LittleEndian>()?;
        let old_lru = reader.read_u32::<LittleEndian>()?;
        let use_lru = reader.read_u32::<LittleEndian>()? == 1;
        let master_map_id = reader.read_u32::<LittleEndian>()?;
        let engine_pack_version = reader.read_u32::<LittleEndian>()?;
        let game_pack_version = reader.read_u32::<LittleEndian>()?;
        let mut version_major = vec![0; 16];
        reader.read_exact(&mut version_major)?;
        let version_minor = reader.read_u32::<LittleEndian>()?;

        Ok(DatDatabaseHeader {
            file_type,
            block_size,
            file_size,
            data_set,
            data_subset,
            free_head,
            free_tail,
            free_count,
            btree,
            new_lru,
            old_lru,
            use_lru,
            master_map_id,
            engine_pack_version,
            game_pack_version,
            version_major,
            version_minor,
        })
    }

    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<DatDatabaseHeader, Box<dyn Error>> {
        reader.seek(SeekFrom::Start(DAT_HEADER_OFFSET))?;
        Self::from_buffer(reader)
    }

    #[cfg(feature = "dat-tokio")]
    pub async fn read_async<R: RangeReader>(
        reader: &mut R,
    ) -> Result<DatDatabaseHeader, Box<dyn Error>> {
        // Calculate the total size of the header data: 16 u32s + 16 bytes for version_major
        const HEADER_SIZE: usize = 16 * 4 + 16;

        // Read the header data in one range read operation
        let data = reader
            .read_range(DAT_HEADER_OFFSET as u32, HEADER_SIZE)
            .await?;
        let mut cursor = std::io::Cursor::new(data);

        Self::from_buffer(&mut cursor)
    }
}
