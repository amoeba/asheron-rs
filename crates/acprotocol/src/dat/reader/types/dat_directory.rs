use std::{
    error::Error,
    io::{Cursor, Read, Seek},
};

use crate::dat::reader::{
    dat_block_reader::DatBlockReader,
    types::{dat_directory_entry::DatDirectoryEntry, dat_directory_header::DatDirectoryHeader},
};

#[cfg(feature = "dat-tokio")]
use crate::dat::reader::range_reader::RangeReader;

pub const DAT_DIRECTORY_HEADER_OBJECT_SIZE: u32 = 0x6B4;

#[derive(Debug)]
pub struct DatDirectory {
    header: DatDirectoryHeader,
    directories: Vec<DatDirectory>,
}

impl DatDirectory {
    pub fn read<R: Read + Seek>(
        reader: &mut R,
        offset: u32,
        block_size: u32,
    ) -> Result<DatDirectory, Box<dyn Error>> {
        // Read DatDirectoryHeader
        let header_buf =
            DatBlockReader::read(reader, offset, DAT_DIRECTORY_HEADER_OBJECT_SIZE, block_size)?;
        let mut header_reader = Cursor::new(header_buf);
        let header = DatDirectoryHeader::read(&mut header_reader)?;

        let mut directories: Vec<DatDirectory> = Vec::new();

        // Recurse only if we're not a leaf
        if header.branches[0] != 0 {
            for i in 0..header.entry_count + 1 {
                let dir = DatDirectory::read(reader, header.branches[i as usize], block_size)?;
                directories.push(dir);
            }
        }

        Ok(DatDirectory {
            header,
            directories,
        })
    }

    #[cfg(feature = "dat-tokio")]
    pub fn read_async<R: RangeReader>(
        reader: &mut R,
        offset: u32,
        block_size: u32,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<DatDirectory, Box<dyn Error>>> + '_>,
    > {
        Box::pin(async move {
            // Read DatDirectoryHeader using async block reader
            let header_buf = DatBlockReader::read_async(
                reader,
                offset,
                DAT_DIRECTORY_HEADER_OBJECT_SIZE,
                block_size,
            )
            .await?;
            let mut header_reader = Cursor::new(header_buf);
            let header = DatDirectoryHeader::read(&mut header_reader)?;

            let mut directories: Vec<DatDirectory> = Vec::new();

            // Recurse only if we're not a leaf
            if header.branches[0] != 0 {
                for i in 0..header.entry_count + 1 {
                    let dir =
                        DatDirectory::read_async(reader, header.branches[i as usize], block_size)
                            .await?;
                    directories.push(dir);
                }
            }

            Ok(DatDirectory {
                header,
                directories,
            })
        })
    }

    pub fn list_files(
        &self,
        files_list: &mut Vec<DatDirectoryEntry>,
        recursive: bool,
    ) -> Result<(), Box<dyn Error>> {
        if recursive {
            for i in 0..self.directories.len() {
                self.directories[i].list_files(files_list, recursive)?;
            }
        }

        for i in 0..self.header.entries.len() {
            files_list.push(self.header.entries[i as usize]);
        }

        Ok(())
    }
}
