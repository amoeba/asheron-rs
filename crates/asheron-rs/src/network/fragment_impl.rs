/// Maximum size of a fragment chunk (in bytes)
pub const FRAGMENT_CHUNK_SIZE: usize = 448;

use crate::generated::network::{Fragment, FragmentHeader};

/// Extended fragment information that's not in the generated struct
#[derive(Debug, Clone)]
pub struct FragmentMetadata {
    pub size: u16,
    pub group: u16,
    pub received_chunks: usize,
    pub total_length: usize, // Track actual data length like C# implementation
    pub chunks: Vec<bool>,
}

impl FragmentMetadata {
    fn new(count: u16) -> Self {
        let count_usize = count as usize;
        Self {
            size: 0,
            group: 0,
            received_chunks: 0,
            total_length: 0,
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
    /// chunk_size is the size of this specific fragment (for tracking total length)
    pub fn add_chunk(&mut self, data: &[u8], index: usize, chunk_size: usize) {
        let start = index * FRAGMENT_CHUNK_SIZE;
        let end = start + data.len();
        if end <= self.data.len() {
            self.data[start..end].copy_from_slice(data);

            // Track received chunks and update total length to the maximum written position
            let mut store = get_metadata_store().lock().unwrap();
            if let Some(metadata) = store.get_mut(&self.header.sequence) {
                if !metadata.chunks[index] {
                    metadata.chunks[index] = true;
                    metadata.received_chunks += 1;
                }
                // Track the maximum end position (start of this chunk + its actual size)
                let chunk_end = start + chunk_size;
                if chunk_end > metadata.total_length {
                    metadata.total_length = chunk_end;
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
    /// Like C#, return the full buffer - message parsers are self-delimiting
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fragment_chunk_size() {
        assert_eq!(FRAGMENT_CHUNK_SIZE, 448);
    }

    #[test]
    fn test_fragment_new() {
        let fragment = Fragment::new(12345, 3);

        assert_eq!(fragment.header.sequence, 12345);
        assert_eq!(fragment.header.count, 3);
        assert_eq!(fragment.data.len(), 3 * FRAGMENT_CHUNK_SIZE);
        assert!(!fragment.is_complete());

        // Cleanup
        fragment.cleanup();
    }

    #[test]
    fn test_fragment_add_chunk() {
        let mut fragment = Fragment::new(100, 2);

        // Add first chunk
        let chunk1 = vec![0xAA; 100];
        fragment.add_chunk(&chunk1, 0, 100);

        // Not complete yet
        assert!(!fragment.is_complete());

        // Add second chunk
        let chunk2 = vec![0xBB; 200];
        fragment.add_chunk(&chunk2, 1, 200);

        // Now it should be complete
        assert!(fragment.is_complete());

        // Verify data - returns full buffer like C# implementation
        let data = fragment.get_data();
        assert_eq!(data.len(), 2 * FRAGMENT_CHUNK_SIZE); // Full allocated buffer
        assert_eq!(data[0], 0xAA); // First chunk starts here
        assert_eq!(data[99], 0xAA); // First chunk ends at 99
        assert_eq!(data[FRAGMENT_CHUNK_SIZE], 0xBB); // Second chunk starts at 448
        assert_eq!(data[FRAGMENT_CHUNK_SIZE + 199], 0xBB); // Second chunk ends at 647

        // Cleanup
        fragment.cleanup();
    }

    #[test]
    fn test_fragment_is_complete() {
        let mut fragment = Fragment::new(200, 3);

        assert!(!fragment.is_complete());

        fragment.add_chunk(&[1; 10], 0, 10);
        assert!(!fragment.is_complete());

        fragment.add_chunk(&[2; 10], 1, 10);
        assert!(!fragment.is_complete());

        fragment.add_chunk(&[3; 10], 2, 10);
        assert!(fragment.is_complete());

        // Cleanup
        fragment.cleanup();
    }

    #[test]
    fn test_fragment_duplicate_chunk() {
        let mut fragment = Fragment::new(300, 2);

        // Add same chunk twice
        fragment.add_chunk(&[0xFF; 50], 0, 50);
        assert!(!fragment.is_complete());

        fragment.add_chunk(&[0xEE; 50], 0, 50); // Duplicate index 0
        assert!(!fragment.is_complete()); // Still not complete (missing index 1)

        fragment.add_chunk(&[0xDD; 50], 1, 50);
        assert!(fragment.is_complete());

        // Cleanup
        fragment.cleanup();
    }

    #[test]
    fn test_fragment_set_and_get_info() {
        let mut fragment = Fragment::new(400, 1);

        fragment.set_fragment_info(1024, 5);

        assert_eq!(fragment.get_size(), 1024);
        assert_eq!(fragment.get_group(), 5);

        // Cleanup
        fragment.cleanup();
    }

    #[test]
    fn test_fragment_get_data() {
        let mut fragment = Fragment::new(500, 1);

        let test_data = vec![0x12, 0x34, 0x56, 0x78];
        fragment.add_chunk(&test_data, 0, test_data.len());

        let data = fragment.get_data();
        assert_eq!(data.len(), FRAGMENT_CHUNK_SIZE); // Returns full buffer like C#
        assert_eq!(data[0], 0x12);
        assert_eq!(data[1], 0x34);
        assert_eq!(data[2], 0x56);
        assert_eq!(data[3], 0x78);

        // Cleanup
        fragment.cleanup();
    }

    #[test]
    fn test_fragment_cleanup() {
        let fragment = Fragment::new(600, 1);

        // Verify metadata exists
        {
            let store = get_metadata_store().lock().unwrap();
            assert!(store.contains_key(&600));
        }

        // Cleanup
        fragment.cleanup();

        // Verify metadata removed
        {
            let store = get_metadata_store().lock().unwrap();
            assert!(!store.contains_key(&600));
        }
    }

    #[test]
    fn test_fragment_metadata_new() {
        let metadata = FragmentMetadata::new(5);

        assert_eq!(metadata.size, 0);
        assert_eq!(metadata.group, 0);
        assert_eq!(metadata.received_chunks, 0);
        assert_eq!(metadata.chunks.len(), 5);
        assert!(metadata.chunks.iter().all(|&c| !c));
    }
}
