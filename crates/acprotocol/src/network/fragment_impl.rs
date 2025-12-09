/// Maximum size of a fragment chunk (in bytes)
pub const FRAGMENT_CHUNK_SIZE: usize = 448;

use crate::generated::network::{Fragment, FragmentHeader};

/// Extended fragment information that's not in the generated struct
#[derive(Debug, Clone)]
pub struct FragmentMetadata {
    pub size: u16,
    pub group: u16,
    pub received_chunks: usize,
    pub chunks: Vec<bool>,
}

impl FragmentMetadata {
    fn new(count: u16) -> Self {
        let count_usize = count as usize;
        Self {
            size: 0,
            group: 0,
            received_chunks: 0,
            chunks: vec![false; count_usize],
        }
    }
}

// We use a thread-local or module-level storage approach since we can't add fields to generated struct
// This maps sequence IDs to metadata
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

fn get_metadata_store() -> &'static Mutex<HashMap<u32, FragmentMetadata>> {
    static STORE: OnceLock<Mutex<HashMap<u32, FragmentMetadata>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

impl Fragment {
    /// Create a new fragment with the given sequence and chunk count
    pub fn new(sequence: u32, count: u16) -> Self {
        let mut store = get_metadata_store().lock().unwrap();
        store.insert(sequence, FragmentMetadata::new(count));

        Self {
            header: FragmentHeader {
                sequence,
                id: 0,
                count,
                index: 0,
            },
            data: vec![0; count as usize * FRAGMENT_CHUNK_SIZE],
        }
    }

    /// Add a chunk of data at the specified index
    pub fn add_chunk(&mut self, data: &[u8], index: usize) {
        let start = index * FRAGMENT_CHUNK_SIZE;
        let end = start + data.len();
        if end <= self.data.len() {
            self.data[start..end].copy_from_slice(data);

            // Track received chunks
            let mut store = get_metadata_store().lock().unwrap();
            if let Some(metadata) = store.get_mut(&self.header.sequence) {
                if !metadata.chunks[index] {
                    metadata.chunks[index] = true;
                    metadata.received_chunks += 1;
                }
            }
        }
    }

    /// Check if all fragments have been received
    pub fn is_complete(&self) -> bool {
        let store = get_metadata_store().lock().unwrap();
        if let Some(metadata) = store.get(&self.header.sequence) {
            metadata.received_chunks == self.header.count as usize
        } else {
            false
        }
    }

    /// Get the assembled data
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// Set size and group on the fragment
    pub fn set_fragment_info(&mut self, size: u16, group: u16) {
        let mut store = get_metadata_store().lock().unwrap();
        if let Some(metadata) = store.get_mut(&self.header.sequence) {
            metadata.size = size;
            metadata.group = group;
        }
    }

    /// Get size for this fragment
    #[allow(dead_code)]
    pub fn get_size(&self) -> u16 {
        let store = get_metadata_store().lock().unwrap();
        store
            .get(&self.header.sequence)
            .map(|m| m.size)
            .unwrap_or(0)
    }

    /// Get group for this fragment
    #[allow(dead_code)]
    pub fn get_group(&self) -> u16 {
        let store = get_metadata_store().lock().unwrap();
        store
            .get(&self.header.sequence)
            .map(|m| m.group)
            .unwrap_or(0)
    }

    /// Clean up metadata when fragment is done
    pub fn cleanup(&self) {
        let mut store = get_metadata_store().lock().unwrap();
        store.remove(&self.header.sequence);
    }
}
