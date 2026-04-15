use std::{
    error::Error,
    io::{Seek, Write},
};

/// Trait for writers that support both Write and Seek operations
pub trait ACWriter: Write + Seek {}
impl<T: Write + Seek> ACWriter for T {}

/// Trait for types that can be written to a binary stream
pub trait ACWritable {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>>;
}
