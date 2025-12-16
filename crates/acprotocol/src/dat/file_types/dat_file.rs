use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::io::{Read, Result};

pub trait DatFileRead: Sized {
    fn read<R: Read>(reader: &mut R) -> Result<Self>;
}

#[derive(Debug)]
pub struct DatFile<T> {
    pub id: i32,
    pub inner: T,
}

impl<T: DatFileRead> DatFile<T> {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let id = reader.read_i32::<LittleEndian>()?;
        let inner = T::read(reader)?;

        Ok(Self { id, inner })
    }
}
