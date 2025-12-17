use std::io::{self, Cursor, Read, Seek, SeekFrom};

/// Helper wrapper around Cursor that implements ACReader
/// Provides clean method names while reading directly without indirection
pub(crate) struct PacketReader<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> PacketReader<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    pub(crate) fn position(&self) -> usize {
        self.cursor.position() as usize
    }

    pub(crate) fn set_position(&mut self, pos: usize) {
        self.cursor.set_position(pos as u64);
    }

    pub(crate) fn remaining(&self) -> usize {
        let pos = self.cursor.position();
        let len = self.cursor.get_ref().len() as u64;
        len.saturating_sub(pos) as usize
    }

    // Read directly without indirection through crate::readers
    pub(crate) fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.cursor.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    pub(crate) fn read_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0u8; 2];
        self.cursor.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    pub(crate) fn read_bytes(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.cursor.read_exact(&mut buf)?;
        Ok(buf)
    }
}

// Implement Read and Seek to make PacketReader implement ACReader
impl<'a> Read for PacketReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.cursor.read(buf)
    }
}

impl<'a> Seek for PacketReader<'a> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.cursor.seek(pos)
    }
}

// PacketReader now automatically implements ACReader (Read + Seek)!
// Methods read directly, avoiding indirection through crate::readers
