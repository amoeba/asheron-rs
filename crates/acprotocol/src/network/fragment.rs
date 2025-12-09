/// Maximum size of a fragment chunk (in bytes)
pub const FRAGMENT_CHUNK_SIZE: usize = 448;

#[derive(Debug, Clone)]
pub struct FragmentHeader {
    pub sequence: u32,
    pub id: u32,
    pub count: u16,
    pub size: u16,
    pub index: u16,
    pub group: u16,
}

impl FragmentHeader {
    pub const SIZE: usize = 16;

    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            sequence: u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
            id: u32::from_le_bytes([data[4], data[5], data[6], data[7]]),
            count: u16::from_le_bytes([data[8], data[9]]),
            size: u16::from_le_bytes([data[10], data[11]]),
            index: u16::from_le_bytes([data[12], data[13]]),
            group: u16::from_le_bytes([data[14], data[15]]),
        })
    }
}

/// Represents a single fragment being assembled
#[derive(Debug, Clone)]
pub struct Fragment {
    pub header: FragmentHeader,
    pub sequence: u32,
    pub data: Vec<u8>,
    pub count: usize,
    pub received: usize,
    pub length: usize,
    chunks: Vec<bool>,
}

impl Fragment {
    /// Create a new fragment with the given sequence and chunk count
    pub fn new(sequence: u32, count: u16) -> Self {
        let count_usize = count as usize;
        Self {
            header: FragmentHeader {
                sequence,
                id: 0,
                count,
                size: 0,
                index: 0,
                group: 0,
            },
            sequence,
            data: vec![0; count_usize * FRAGMENT_CHUNK_SIZE],
            count: count_usize,
            received: 0,
            length: 0,
            chunks: vec![false; count_usize],
        }
    }

    /// Add a chunk of data at the specified index
    pub fn add_chunk(&mut self, data: &[u8], index: usize) {
        if index < self.chunks.len() && !self.chunks[index] {
            let start = index * FRAGMENT_CHUNK_SIZE;
            let end = start + data.len();
            if end <= self.data.len() {
                self.data[start..end].copy_from_slice(data);
                self.received += 1;
                self.length += data.len();
                self.chunks[index] = true;
            }
        }
    }

    /// Check if all fragments have been received
    pub fn is_complete(&self) -> bool {
        self.count == self.received
    }

    /// Get the assembled data
    pub fn get_data(&self) -> &[u8] {
        &self.data[..self.length]
    }
}
