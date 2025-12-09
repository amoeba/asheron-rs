use crate::types::{PHashTable, PackableHashTable, PackableList};
use std::cmp::Eq;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

pub mod alignment;
pub mod traits;
pub use alignment::{align, align_dword, align_qword, align_word};
pub use traits::{ACDataType, ACReader};

/// Read an item of type T from the reader
pub fn read_item<T: ACDataType>(reader: &mut dyn ACReader) -> Result<T, Box<dyn Error>> {
    T::read(reader)
}

// Implement ACDataType for primitive types
impl ACDataType for u8 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_u8(reader)
    }
}

impl ACDataType for i8 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_i8(reader)
    }
}

impl ACDataType for u16 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_u16(reader)
    }
}

impl ACDataType for i16 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_i16(reader)
    }
}

impl ACDataType for u32 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_u32(reader)
    }
}

impl ACDataType for i32 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_i32(reader)
    }
}

impl ACDataType for u64 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_u64(reader)
    }
}

impl ACDataType for i64 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_i64(reader)
    }
}

impl ACDataType for f32 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_f32(reader)
    }
}

impl ACDataType for f64 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_f64(reader)
    }
}

impl ACDataType for bool {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_bool(reader)
    }
}

impl ACDataType for String {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn Error>> {
        read_string(reader)
    }
}

/// Read a single byte (u8)
pub fn read_u8(reader: &mut dyn ACReader) -> Result<u8, Box<dyn Error>> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}

/// Read a signed byte (i8)
pub fn read_i8(reader: &mut dyn ACReader) -> Result<i8, Box<dyn Error>> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    Ok(i8::from_le_bytes(buf))
}

/// Read a 16-bit unsigned integer (u16, little-endian)
pub fn read_u16(reader: &mut dyn ACReader) -> Result<u16, Box<dyn Error>> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

/// Read a 16-bit signed integer (i16, little-endian)
pub fn read_i16(reader: &mut dyn ACReader) -> Result<i16, Box<dyn Error>> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(i16::from_le_bytes(buf))
}

/// Read a 32-bit unsigned integer (u32, little-endian)
pub fn read_u32(reader: &mut dyn ACReader) -> Result<u32, Box<dyn Error>> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

/// Read a 32-bit signed integer (i32, little-endian)
pub fn read_i32(reader: &mut dyn ACReader) -> Result<i32, Box<dyn Error>> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

/// Read a 64-bit unsigned integer (u64, little-endian)
pub fn read_u64(reader: &mut dyn ACReader) -> Result<u64, Box<dyn Error>> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

/// Read a 64-bit signed integer (i64, little-endian)
pub fn read_i64(reader: &mut dyn ACReader) -> Result<i64, Box<dyn Error>> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(i64::from_le_bytes(buf))
}

/// Read a 32-bit floating point number (f32, little-endian)
pub fn read_f32(reader: &mut dyn ACReader) -> Result<f32, Box<dyn Error>> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

/// Read a 64-bit floating point number (f64, little-endian)
pub fn read_f64(reader: &mut dyn ACReader) -> Result<f64, Box<dyn Error>> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(f64::from_le_bytes(buf))
}

/// Read a boolean (represented as u32, 0 = false, 1 = true)
pub fn read_bool(reader: &mut dyn ACReader) -> Result<bool, Box<dyn Error>> {
    Ok(read_u32(reader)? == 1)
}

/// Read a string (i16 length-prefixed, aligned to 4-byte boundary)
/// If length is -1, reads a 32-bit length instead (special case for long strings)
pub fn read_string(reader: &mut dyn ACReader) -> Result<String, Box<dyn Error>> {
    let len_i16 = read_i16(reader)?;
    let len = if len_i16 == -1 {
        // Special case: -1 means read a 32-bit length
        read_i32(reader)? as usize
    } else {
        len_i16 as usize
    };

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;

    // Calculate bytes read (including length prefix)
    let bytes_read = if len_i16 == -1 {
        2 + 4 + len // i16 + i32 + string bytes
    } else {
        2 + len // i16 + string bytes
    };

    // Read padding to align to 4 bytes
    let padding = (4 - (bytes_read % 4)) % 4;
    if padding > 0 {
        let mut pad_buf = vec![0u8; padding];
        reader.read_exact(&mut pad_buf)?;
    }

    Ok(String::from_utf8(buf)?)
}

/// Read an ObjectId (u32)
pub fn read_object_id(reader: &mut dyn ACReader) -> Result<u32, Box<dyn Error>> {
    read_u32(reader)
}

/// Read a WString (variable-length Unicode string)
/// Length encoding: 1 byte if < 128, or 2 bytes if high bit set in first byte
/// String encoding: UTF-16LE
pub fn read_wstring(reader: &mut dyn ACReader) -> Result<String, Box<dyn Error>> {
    // Read length with variable encoding
    let first_byte = read_u8(reader)?;
    let length = if (first_byte & 0x80) != 0 {
        // High bit set: 2-byte length
        let second_byte = read_u8(reader)?;
        (((first_byte & 0x7f) as usize) << 8) | (second_byte as usize)
    } else {
        // Single byte length
        first_byte as usize
    };

    // Read UTF-16 bytes (2 bytes per character)
    let byte_count = length * 2;
    let mut buf = vec![0u8; byte_count];
    reader.read_exact(&mut buf)?;

    // Decode as UTF-16LE
    let u16_vec: Vec<u16> = buf
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    Ok(String::from_utf16(&u16_vec)?)
}

/// Read a String32L (complex packed string format used in Login header packets)
/// Format: u32 data length, followed by packed word string length, then string data
/// The packed word adds 1 or 2 byte prefix that needs to be discarded
pub fn read_string32l(reader: &mut dyn ACReader, pad: bool) -> Result<String, Box<dyn Error>> {
    let mut bytes_read = 0usize;
    let mut length = read_u32(reader)?;
    bytes_read += 4;

    if length == 0 {
        return Ok(String::new());
    }

    // Read and discard the first byte of the packed word
    read_u8(reader)?;
    bytes_read += 1;
    length -= 1;

    // If length > 255, read and discard the second byte
    if length > 255 {
        read_u8(reader)?;
        bytes_read += 1;
        length -= 1;
    }

    // Read the actual string
    let string = if length > 0 {
        let mut buf = vec![0u8; length as usize];
        reader.read_exact(&mut buf)?;
        bytes_read += length as usize;
        String::from_utf8(buf)?
    } else {
        String::new()
    };

    // Apply padding if requested
    if pad {
        let align = bytes_read % 4;
        if align > 0 {
            let mut pad_buf = vec![0u8; 4 - align];
            reader.read_exact(&mut pad_buf)?;
        }
    }

    Ok(string)
}

/// Read a PackedWORD (variable-length i16)
/// Format: 1 byte if value < 128, otherwise 2 bytes with high bit flag
pub fn read_packed_word(reader: &mut dyn ACReader) -> Result<i16, Box<dyn Error>> {
    let mut tmp = read_u8(reader)? as i16;
    if (tmp & 0x80) != 0 {
        tmp = ((tmp & 0x7f) << 8) | (read_u8(reader)? as i16);
    }
    Ok(tmp)
}

/// Read a PackedDWORD (variable-length u32)
/// Format: 2 bytes if high bit not set, otherwise 4 bytes
pub fn read_packed_dword(reader: &mut dyn ACReader) -> Result<u32, Box<dyn Error>> {
    let tmp = read_i16(reader)?;
    if (tmp as i16) < 0 {
        // Check if high bit is set (negative when interpreted as i16)
        // High bit set: read another 2 bytes
        let high_part = ((tmp as u32) << 16) & 0x7FFF_FFFF;
        let low_part = read_u16(reader)? as u32;
        Ok(high_part | low_part)
    } else {
        Ok(tmp as u32)
    }
}

/// Read a 16-bit unsigned integer (big-endian)
pub fn read_u16_be(reader: &mut dyn ACReader) -> Result<u16, Box<dyn Error>> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

/// Read a List<T> with a custom element reader function
/// Format: u32 count followed by count items read by the provided function
pub fn read_list_with<T>(
    reader: &mut dyn ACReader,
    mut read_element: impl FnMut(&mut dyn ACReader) -> Result<T, Box<dyn Error>>,
) -> Result<Vec<T>, Box<dyn Error>> {
    let count = read_u32(reader)? as usize;
    let mut list = Vec::with_capacity(count);
    for _ in 0..count {
        list.push(read_element(reader)?);
    }
    Ok(list)
}

/// Read a List<T> where T implements ACDataType
/// Format: u32 count followed by count items
pub fn read_list<T: ACDataType>(reader: &mut dyn ACReader) -> Result<Vec<T>, Box<dyn Error>> {
    let count = read_u32(reader)? as usize;
    let mut list = Vec::with_capacity(count);
    for _ in 0..count {
        list.push(read_item::<T>(reader)?);
    }
    Ok(list)
}

/// Read a vector of T with a custom element reader function and specified count
/// Format: count items read by the provided function
pub fn read_vec_with<T>(
    reader: &mut dyn ACReader,
    count: usize,
    mut read_element: impl FnMut(&mut dyn ACReader) -> Result<T, Box<dyn Error>>,
) -> Result<Vec<T>, Box<dyn Error>> {
    let mut vec = Vec::with_capacity(count);
    for _ in 0..count {
        vec.push(read_element(reader)?);
    }
    Ok(vec)
}

/// Read a vector of T where T implements ACDataType and specified count
pub fn read_vec<T: ACDataType>(
    reader: &mut dyn ACReader,
    count: usize,
) -> Result<Vec<T>, Box<dyn Error>> {
    let mut vec = Vec::with_capacity(count);
    for _ in 0..count {
        vec.push(read_item::<T>(reader)?);
    }
    Ok(vec)
}

/// Read a PackableList<T> with a custom element reader function
/// Format: u32 count followed by count items read by the provided function
pub fn read_packable_list_with<T>(
    reader: &mut dyn ACReader,
    mut read_element: impl FnMut(&mut dyn ACReader) -> Result<T, Box<dyn Error>>,
) -> Result<PackableList<T>, Box<dyn Error>> {
    let mut count_buf = [0u8; 4];
    reader.read_exact(&mut count_buf)?;
    let count = u32::from_le_bytes(count_buf) as usize;
    let mut list = Vec::with_capacity(count);
    for _ in 0..count {
        list.push(read_element(reader)?);
    }
    Ok(PackableList {
        count: count as u32,
        list,
    })
}

/// Read a PackableList<T> where T implements ACDataType
/// Format: u32 count followed by count items
pub fn read_packable_list<T: ACDataType>(
    reader: &mut dyn ACReader,
) -> Result<PackableList<T>, Box<dyn Error>> {
    let count = read_u32(reader)? as usize;
    let mut list = Vec::with_capacity(count);
    for _ in 0..count {
        list.push(read_item::<T>(reader)?);
    }
    Ok(PackableList {
        count: count as u32,
        list,
    })
}

/// Read a PackableHashTable<K, V> with custom key and value reader functions
/// Format: u16 count, u16 max_size, followed by count key-value pairs
pub fn read_packable_hash_table_with<K, V>(
    reader: &mut dyn ACReader,
    mut read_key: impl FnMut(&mut dyn ACReader) -> Result<K, Box<dyn Error>>,
    mut read_value: impl FnMut(&mut dyn ACReader) -> Result<V, Box<dyn Error>>,
) -> Result<PackableHashTable<K, V>, Box<dyn Error>>
where
    K: Eq + Hash,
{
    let mut count_buf = [0u8; 2];
    reader.read_exact(&mut count_buf)?;
    let count = u16::from_le_bytes(count_buf) as usize;

    let mut max_size_buf = [0u8; 2];
    reader.read_exact(&mut max_size_buf)?;
    let max_size = u16::from_le_bytes(max_size_buf);

    let mut table = HashMap::with_capacity(count);
    for _ in 0..count {
        let key = read_key(reader)?;
        let value = read_value(reader)?;
        table.insert(key, value);
    }
    Ok(PackableHashTable {
        count: count as u16,
        max_size,
        table,
    })
}

/// Read a PHashTable<K, V> with custom key and value reader functions
/// Format: u32 packed_size (with count in lower 24 bits), followed by count key-value pairs
pub fn read_phash_table_with<K, V>(
    reader: &mut dyn ACReader,
    mut read_key: impl FnMut(&mut dyn ACReader) -> Result<K, Box<dyn Error>>,
    mut read_value: impl FnMut(&mut dyn ACReader) -> Result<V, Box<dyn Error>>,
) -> Result<PHashTable<K, V>, Box<dyn Error>>
where
    K: Eq + Hash,
{
    let mut packed_size_buf = [0u8; 4];
    reader.read_exact(&mut packed_size_buf)?;
    let packed_size = u32::from_le_bytes(packed_size_buf);
    let count = (packed_size & 0xFFFFFF) as usize;

    let mut table = HashMap::with_capacity(count);
    for _ in 0..count {
        let key = read_key(reader)?;
        let value = read_value(reader)?;
        table.insert(key, value);
    }
    Ok(PHashTable { packed_size, table })
}

/// Read a PHashTable<K, V> where K and V implement ACDataType
/// Format: u32 packed_size (with count in lower 24 bits), followed by count key-value pairs
pub fn read_phash_table<K: ACDataType + Eq + Hash, V: ACDataType>(
    reader: &mut dyn ACReader,
) -> Result<PHashTable<K, V>, Box<dyn Error>> {
    let packed_size = read_u32(reader)?;
    let count = (packed_size & 0xFFFFFF) as usize;

    let mut table = HashMap::with_capacity(count);
    for _ in 0..count {
        let key = read_item::<K>(reader)?;
        let value = read_item::<V>(reader)?;
        table.insert(key, value);
    }
    Ok(PHashTable { packed_size, table })
}
