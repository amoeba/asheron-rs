use std::io::{Read, Seek, SeekFrom};

/// Binary data reader for parsing network packets
pub struct BinaryReader<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> BinaryReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, pos: usize) {
        self.position = pos;
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.position)
    }

    pub fn read_u8(&mut self) -> std::io::Result<u8> {
        if self.position >= self.data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Not enough data",
            ));
        }
        let val = self.data[self.position];
        self.position += 1;
        Ok(val)
    }

    pub fn read_u16(&mut self) -> std::io::Result<u16> {
        if self.position + 2 > self.data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Not enough data",
            ));
        }
        let val = u16::from_le_bytes([self.data[self.position], self.data[self.position + 1]]);
        self.position += 2;
        Ok(val)
    }

    pub fn read_u32(&mut self) -> std::io::Result<u32> {
        if self.position + 4 > self.data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Not enough data",
            ));
        }
        let val = u32::from_le_bytes([
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
        ]);
        self.position += 4;
        Ok(val)
    }

    pub fn read_bytes(&mut self, len: usize) -> std::io::Result<Vec<u8>> {
        if self.position + len > self.data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Not enough data",
            ));
        }
        let result = self.data[self.position..self.position + len].to_vec();
        self.position += len;
        Ok(result)
    }
}
