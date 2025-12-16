use std::{
    error::Error,
    io::{Read, Seek},
};

use byteorder::{LittleEndian, ReadBytesExt};

use super::dat_directory_entry::DatDirectoryEntry;

#[derive(Debug)]
pub struct DatDirectoryHeader {
    pub branches: Vec<u32>,
    pub entry_count: u32,
    pub entries: Vec<DatDirectoryEntry>,
}

impl DatDirectoryHeader {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<DatDirectoryHeader, Box<dyn Error>> {
        let mut branches = vec![0; 62];

        for branch in &mut branches {
            *branch = reader.read_u32::<LittleEndian>()?;
        }

        let entry_count = reader.read_u32::<LittleEndian>()?;

        let mut entries = vec![];

        for _ in 0..entry_count {
            entries.push(DatDirectoryEntry::read(reader)?);
        }

        Ok(DatDirectoryHeader {
            branches,
            entry_count,
            entries,
        })
    }
}
