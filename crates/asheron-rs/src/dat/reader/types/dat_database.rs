use std::{
    error::Error,
    io::{Read, Seek},
};

use super::{
    dat_database_header::DatDatabaseHeader, dat_directory::DatDirectory,
    dat_directory_entry::DatDirectoryEntry,
};

#[cfg(feature = "dat-tokio")]
use crate::dat::reader::range_reader::RangeReader;

#[derive(Debug)]
pub struct DatDatabase {
    pub header: DatDatabaseHeader,
    pub root_dir: DatDirectory,
}

impl DatDatabase {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<DatDatabase, Box<dyn Error>> {
        let header: DatDatabaseHeader = DatDatabaseHeader::read(reader)?;
        let root_dir = DatDirectory::read(reader, header.btree, header.block_size)?;

        Ok(DatDatabase { header, root_dir })
    }

    #[cfg(feature = "dat-tokio")]
    pub async fn read_async<R: RangeReader>(reader: &mut R) -> Result<DatDatabase, Box<dyn Error>> {
        let header: DatDatabaseHeader = DatDatabaseHeader::read_async(reader).await?;
        let root_dir = DatDirectory::read_async(reader, header.btree, header.block_size).await?;

        Ok(DatDatabase { header, root_dir })
    }

    pub fn list_files(&self, recursive: bool) -> Result<Vec<DatDirectoryEntry>, Box<dyn Error>> {
        let mut files_list: Vec<DatDirectoryEntry> = Vec::new();
        self.root_dir.list_files(&mut files_list, recursive)?;

        Ok(files_list)
    }
}
