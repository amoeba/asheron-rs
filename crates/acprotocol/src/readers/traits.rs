use std::{
    error::Error,
    io::{Read, Seek},
};

/// Trait for readers that support both Read and Seek operations
pub trait ACReader: Read + Seek {}
impl<T: Read + Seek> ACReader for T {}

/// Trait for types that can be read from a binary stream
pub trait ACDataType: Sized {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>>;
}
