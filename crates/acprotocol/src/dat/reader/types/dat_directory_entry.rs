use std::{
    error::Error,
    io::{Read, Seek},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::dat::{DatFileSubtype, DatFileType};

#[derive(Debug, Clone, Copy)]
pub struct DatDirectoryEntry {
    pub bit_flags: u32,
    pub object_id: u32,
    pub file_offset: u32,
    pub file_size: u32,
    pub date: u32,
    pub iteration: u32,
}

impl DatDirectoryEntry {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<DatDirectoryEntry, Box<dyn Error>> {
        Ok(DatDirectoryEntry {
            bit_flags: reader.read_u32::<LittleEndian>()?,
            object_id: reader.read_u32::<LittleEndian>()?,
            file_offset: reader.read_u32::<LittleEndian>()?,
            file_size: reader.read_u32::<LittleEndian>()?,
            date: reader.read_u32::<LittleEndian>()?,
            iteration: reader.read_u32::<LittleEndian>()?,
        })
    }

    pub fn file_type(&self) -> DatFileType {
        match self.object_id {
            0x06000000..=0x07FFFFFF => DatFileType::Texture,
            _ => DatFileType::Unknown,
        }
    }

    // WIP: Use this to let datfiles be specialized things like icons
    pub fn file_subtype(&self) -> DatFileSubtype {
        // TODO
        DatFileSubtype::None
    }
}
