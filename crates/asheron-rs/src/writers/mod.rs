use crate::types::{PHashTable, PackableHashTable, PackableList};
use encoding_rs::WINDOWS_1252;
use std::cmp::Eq;
use std::error::Error;
use std::hash::Hash;

pub mod alignment;
pub mod traits;
pub use alignment::{align_dword_write, align_qword_write, align_word_write, align_write};
pub use traits::{ACWritable, ACWriter};

/// Write an item of type T to the writer
pub fn write_item<T: ACWritable>(
    writer: &mut dyn ACWriter,
    value: &T,
) -> Result<(), Box<dyn Error>> {
    value.write(writer)
}

// Implement ACWritable for primitive types
impl ACWritable for u8 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_u8(writer, *self)
    }
}

impl ACWritable for i8 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_i8(writer, *self)
    }
}

impl ACWritable for u16 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_u16(writer, *self)
    }
}

impl ACWritable for i16 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_i16(writer, *self)
    }
}

impl ACWritable for u32 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_u32(writer, *self)
    }
}

impl ACWritable for i32 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_i32(writer, *self)
    }
}

impl ACWritable for u64 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_u64(writer, *self)
    }
}

impl ACWritable for i64 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_i64(writer, *self)
    }
}

impl ACWritable for f32 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_f32(writer, *self)
    }
}

impl ACWritable for f64 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_f64(writer, *self)
    }
}

impl ACWritable for bool {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_bool(writer, *self)
    }
}

impl ACWritable for String {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
        write_string(writer, self)
    }
}

/// Write a single byte (u8)
pub fn write_u8(writer: &mut dyn ACWriter, value: u8) -> Result<(), Box<dyn Error>> {
    writer.write_all(&[value])?;
    Ok(())
}

/// Write a signed byte (i8)
pub fn write_i8(writer: &mut dyn ACWriter, value: i8) -> Result<(), Box<dyn Error>> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

/// Write a 16-bit unsigned integer (u16, little-endian)
pub fn write_u16(writer: &mut dyn ACWriter, value: u16) -> Result<(), Box<dyn Error>> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

/// Write a 16-bit signed integer (i16, little-endian)
pub fn write_i16(writer: &mut dyn ACWriter, value: i16) -> Result<(), Box<dyn Error>> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

/// Write a 32-bit unsigned integer (u32, little-endian)
pub fn write_u32(writer: &mut dyn ACWriter, value: u32) -> Result<(), Box<dyn Error>> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

/// Write a 32-bit signed integer (i32, little-endian)
pub fn write_i32(writer: &mut dyn ACWriter, value: i32) -> Result<(), Box<dyn Error>> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

/// Write a 64-bit unsigned integer (u64, little-endian)
pub fn write_u64(writer: &mut dyn ACWriter, value: u64) -> Result<(), Box<dyn Error>> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

/// Write a 64-bit signed integer (i64, little-endian)
pub fn write_i64(writer: &mut dyn ACWriter, value: i64) -> Result<(), Box<dyn Error>> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

/// Write a 32-bit floating point number (f32, little-endian)
pub fn write_f32(writer: &mut dyn ACWriter, value: f32) -> Result<(), Box<dyn Error>> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

/// Write a 64-bit floating point number (f64, little-endian)
pub fn write_f64(writer: &mut dyn ACWriter, value: f64) -> Result<(), Box<dyn Error>> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

/// Write a boolean (represented as u32, 0 = false, 1 = true)
pub fn write_bool(writer: &mut dyn ACWriter, value: bool) -> Result<(), Box<dyn Error>> {
    write_u32(writer, if value { 1 } else { 0 })
}

/// Write a string (16-bit length-prefixed, Windows-1252 encoded, aligned to 4-byte boundary)
/// Length is encoded as a fixed 2-byte signed integer (Int16)
/// If length >= 32767, writes -1 as Int16 followed by 32-bit length
pub fn write_string(writer: &mut dyn ACWriter, value: &str) -> Result<(), Box<dyn Error>> {
    // Encode to Windows-1252
    let (encoded, _encoding_used, _had_errors) = WINDOWS_1252.encode(value);
    let len = encoded.len();

    let bytes_written = if len >= 32767 {
        // Write -1 as i16, then length as i32
        write_i16(writer, -1)?;
        write_i32(writer, len as i32)?;
        2 + 4 // bytes for length prefix
    } else {
        // Write length as i16
        write_i16(writer, len as i16)?;
        2 // bytes for length prefix
    };

    // Write string bytes
    writer.write_all(&encoded)?;
    let bytes_written = bytes_written + len;

    // Write padding to align to 4 bytes
    let padding = (4 - (bytes_written % 4)) % 4;
    if padding > 0 {
        writer.write_all(&vec![0u8; padding])?;
    }

    Ok(())
}

/// Write an ObjectId (u32)
pub fn write_object_id(writer: &mut dyn ACWriter, value: u32) -> Result<(), Box<dyn Error>> {
    write_u32(writer, value)
}

/// Write a WString (variable-length Unicode string)
/// Length encoding: 1 byte if < 128, or 2 bytes if high bit set in first byte
/// String encoding: UTF-16LE
pub fn write_wstring(writer: &mut dyn ACWriter, value: &str) -> Result<(), Box<dyn Error>> {
    let utf16: Vec<u16> = value.encode_utf16().collect();
    let length = utf16.len();

    // Write variable-length encoding
    if length < 128 {
        write_u8(writer, length as u8)?;
    } else {
        // High bit set: 2-byte length
        let first_byte = ((length >> 8) as u8) | 0x80;
        let second_byte = (length & 0xFF) as u8;
        write_u8(writer, first_byte)?;
        write_u8(writer, second_byte)?;
    }

    // Write UTF-16LE bytes
    for code_unit in utf16 {
        writer.write_all(&code_unit.to_le_bytes())?;
    }

    Ok(())
}

/// Write a String32L (complex packed string format used in Login header packets)
/// Format: u32 data length (including packed word prefix), followed by packed word string length, then string data
pub fn write_string32l(
    writer: &mut dyn ACWriter,
    value: &str,
    pad: bool,
) -> Result<(), Box<dyn Error>> {
    let mut bytes_written = 0usize;

    // Encode to Windows-1252
    let (encoded, _encoding_used, _had_errors) = WINDOWS_1252.encode(value);
    let string_len = encoded.len();

    if string_len == 0 {
        write_u32(writer, 0)?;
        return Ok(());
    }

    // Calculate total data length including packed word prefix
    let packed_word_size = if string_len > 255 { 2 } else { 1 };
    let total_data_len = packed_word_size + string_len;

    // Write u32 data length
    write_u32(writer, total_data_len as u32)?;
    bytes_written += 4;

    // Write packed word for string length
    if string_len > 255 {
        let first_byte = ((string_len >> 8) as u8) | 0x80;
        let second_byte = (string_len & 0xFF) as u8;
        write_u8(writer, first_byte)?;
        write_u8(writer, second_byte)?;
        bytes_written += 2;
    } else {
        write_u8(writer, string_len as u8)?;
        bytes_written += 1;
    }

    // Write the actual string
    if string_len > 0 {
        writer.write_all(&encoded)?;
        bytes_written += string_len;
    }

    // Apply padding if requested
    if pad {
        let align = bytes_written % 4;
        if align > 0 {
            writer.write_all(&vec![0u8; 4 - align])?;
        }
    }

    Ok(())
}

/// Write a PackedWORD (variable-length i16)
/// Format: 1 byte if value < 128 and value >= 0, otherwise 2 bytes with high bit flag
pub fn write_packed_word(writer: &mut dyn ACWriter, value: i16) -> Result<(), Box<dyn Error>> {
    if (0..128).contains(&value) {
        write_u8(writer, value as u8)?;
    } else {
        let high_byte = ((value >> 8) as u8) | 0x80;
        let low_byte = (value & 0xFF) as u8;
        write_u8(writer, high_byte)?;
        write_u8(writer, low_byte)?;
    }
    Ok(())
}

/// Write a PackedDWORD (variable-length u32)
/// Format: 2 bytes if value < 0x8000, otherwise 4 bytes with high bit flag
pub fn write_packed_dword(writer: &mut dyn ACWriter, value: u32) -> Result<(), Box<dyn Error>> {
    if value < 0x8000 {
        write_i16(writer, value as i16)?;
    } else {
        // Set high bit and write 4 bytes
        let high_word = ((value >> 16) as u16) | 0x8000;
        let low_word = (value & 0xFFFF) as u16;
        write_u16(writer, high_word)?;
        write_u16(writer, low_word)?;
    }
    Ok(())
}

/// Write a 16-bit unsigned integer (big-endian)
pub fn write_u16_be(writer: &mut dyn ACWriter, value: u16) -> Result<(), Box<dyn Error>> {
    writer.write_all(&value.to_be_bytes())?;
    Ok(())
}

/// Write a `List<T>` with a custom element writer function
/// Format: u32 count followed by count items written by the provided function
pub fn write_list_with<T>(
    writer: &mut dyn ACWriter,
    items: &[T],
    mut write_element: impl FnMut(&mut dyn ACWriter, &T) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    write_u32(writer, items.len() as u32)?;
    for item in items {
        write_element(writer, item)?;
    }
    Ok(())
}

/// Write a `List<T>` where T implements ACWritable
/// Format: u32 count followed by count items
pub fn write_list<T: ACWritable>(
    writer: &mut dyn ACWriter,
    items: &[T],
) -> Result<(), Box<dyn Error>> {
    write_u32(writer, items.len() as u32)?;
    for item in items {
        write_item(writer, item)?;
    }
    Ok(())
}

/// Write a vector of T with a custom element writer function
/// Format: count items written by the provided function (no count prefix)
pub fn write_vec_with<T>(
    writer: &mut dyn ACWriter,
    items: &[T],
    mut write_element: impl FnMut(&mut dyn ACWriter, &T) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    for item in items {
        write_element(writer, item)?;
    }
    Ok(())
}

/// Write a vector of T where T implements ACWritable
/// Format: count items (no count prefix)
pub fn write_vec<T: ACWritable>(
    writer: &mut dyn ACWriter,
    items: &[T],
) -> Result<(), Box<dyn Error>> {
    for item in items {
        write_item(writer, item)?;
    }
    Ok(())
}

/// Write a `PackableList<T>` with a custom element writer function
/// Format: u32 count followed by count items written by the provided function
pub fn write_packable_list_with<T>(
    writer: &mut dyn ACWriter,
    list: &PackableList<T>,
    mut write_element: impl FnMut(&mut dyn ACWriter, &T) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    writer.write_all(&list.count.to_le_bytes())?;
    for item in &list.list {
        write_element(writer, item)?;
    }
    Ok(())
}

/// Write a `PackableList<T>` where T implements ACWritable
/// Format: u32 count followed by count items
pub fn write_packable_list<T: ACWritable>(
    writer: &mut dyn ACWriter,
    list: &PackableList<T>,
) -> Result<(), Box<dyn Error>> {
    writer.write_all(&list.count.to_le_bytes())?;
    for item in &list.list {
        write_item(writer, item)?;
    }
    Ok(())
}

/// Write a PackableHashTable<K, V> with custom key and value writer functions
/// Format: i16 count, i16 max_size, followed by count key-value pairs
pub fn write_packable_hash_table_with<K, V>(
    writer: &mut dyn ACWriter,
    table: &PackableHashTable<K, V>,
    mut write_key: impl FnMut(&mut dyn ACWriter, &K) -> Result<(), Box<dyn Error>>,
    mut write_value: impl FnMut(&mut dyn ACWriter, &V) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>>
where
    K: Eq + Hash,
{
    writer.write_all(&(table.count as i16).to_le_bytes())?;
    writer.write_all(&(table.max_size as i16).to_le_bytes())?;

    for (key, value) in &table.table {
        write_key(writer, key)?;
        write_value(writer, value)?;
    }
    Ok(())
}

/// Write a PackableHashTable<K, V> where K and V implement ACWritable
/// Format: i16 count, i16 max_size, followed by count key-value pairs
pub fn write_packable_hash_table<K: ACWritable + Eq + Hash, V: ACWritable>(
    writer: &mut dyn ACWriter,
    table: &PackableHashTable<K, V>,
) -> Result<(), Box<dyn Error>> {
    writer.write_all(&(table.count as i16).to_le_bytes())?;
    writer.write_all(&(table.max_size as i16).to_le_bytes())?;

    for (key, value) in &table.table {
        write_item(writer, key)?;
        write_item(writer, value)?;
    }
    Ok(())
}

/// Write a PHashTable<K, V> with custom key and value writer functions
/// Format: u32 packed_size (with count in lower 24 bits), followed by count key-value pairs
pub fn write_phash_table_with<K, V>(
    writer: &mut dyn ACWriter,
    table: &PHashTable<K, V>,
    mut write_key: impl FnMut(&mut dyn ACWriter, &K) -> Result<(), Box<dyn Error>>,
    mut write_value: impl FnMut(&mut dyn ACWriter, &V) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>>
where
    K: Eq + Hash,
{
    writer.write_all(&table.packed_size.to_le_bytes())?;

    let count = (table.packed_size & 0xFFFFFF) as usize;
    let mut written = 0;
    for (key, value) in &table.table {
        write_key(writer, key)?;
        write_value(writer, value)?;
        written += 1;
        if written >= count {
            break;
        }
    }
    Ok(())
}

/// Write a PHashTable<K, V> where K and V implement ACWritable
/// Format: u32 packed_size (with count in lower 24 bits), followed by count key-value pairs
pub fn write_phash_table<K: ACWritable + Eq + Hash, V: ACWritable>(
    writer: &mut dyn ACWriter,
    table: &PHashTable<K, V>,
) -> Result<(), Box<dyn Error>> {
    writer.write_all(&table.packed_size.to_le_bytes())?;

    let count = (table.packed_size & 0xFFFFFF) as usize;
    let mut written = 0;
    for (key, value) in &table.table {
        write_item(writer, key)?;
        write_item(writer, value)?;
        written += 1;
        if written >= count {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readers::*;
    use std::io::Cursor;

    #[test]
    fn test_u8_roundtrip() {
        let original = 0x42u8;
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_u8(&mut cursor, original).unwrap();

        cursor.set_position(0);
        let read_back = read_u8(&mut cursor).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_i8_roundtrip() {
        let original = -42i8;
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_i8(&mut cursor, original).unwrap();

        cursor.set_position(0);
        let read_back = read_i8(&mut cursor).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_u16_roundtrip() {
        let original = 0x1234u16;
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_u16(&mut cursor, original).unwrap();

        cursor.set_position(0);
        let read_back = read_u16(&mut cursor).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_i16_roundtrip() {
        let original = -1234i16;
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_i16(&mut cursor, original).unwrap();

        cursor.set_position(0);
        let read_back = read_i16(&mut cursor).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_u32_roundtrip() {
        let original = 0x12345678u32;
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_u32(&mut cursor, original).unwrap();

        cursor.set_position(0);
        let read_back = read_u32(&mut cursor).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_i32_roundtrip() {
        let original = -123456i32;
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_i32(&mut cursor, original).unwrap();

        cursor.set_position(0);
        let read_back = read_i32(&mut cursor).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_u64_roundtrip() {
        let original = 0x123456789ABCDEFu64;
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_u64(&mut cursor, original).unwrap();

        cursor.set_position(0);
        let read_back = read_u64(&mut cursor).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_i64_roundtrip() {
        let original = -123456789i64;
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_i64(&mut cursor, original).unwrap();

        cursor.set_position(0);
        let read_back = read_i64(&mut cursor).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_f32_roundtrip() {
        let original = std::f32::consts::PI;
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_f32(&mut cursor, original).unwrap();

        cursor.set_position(0);
        let read_back = read_f32(&mut cursor).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_f64_roundtrip() {
        let original = std::f64::consts::PI;
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_f64(&mut cursor, original).unwrap();

        cursor.set_position(0);
        let read_back = read_f64(&mut cursor).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_bool_roundtrip() {
        for original in [true, false] {
            let mut buffer = Vec::new();
            let mut cursor = Cursor::new(&mut buffer);

            write_bool(&mut cursor, original).unwrap();

            cursor.set_position(0);
            let read_back = read_bool(&mut cursor).unwrap();

            assert_eq!(original, read_back);
        }
    }

    #[test]
    fn test_string_roundtrip() {
        let test_cases = vec![
            "",
            "Hello, World!",
            "A",
            "This is a longer string to test the string encoding",
        ];

        for original in test_cases {
            let mut buffer = Vec::new();
            let mut cursor = Cursor::new(&mut buffer);

            write_string(&mut cursor, original).unwrap();

            cursor.set_position(0);
            let read_back = read_string(&mut cursor).unwrap();

            assert_eq!(original, read_back, "Failed for string: '{}'", original);
        }
    }

    #[test]
    fn test_wstring_roundtrip() {
        let test_cases = vec!["", "Hello", "Unicode: こんにちは", "Emoji: 😀"];

        for original in test_cases {
            let mut buffer = Vec::new();
            let mut cursor = Cursor::new(&mut buffer);

            write_wstring(&mut cursor, original).unwrap();

            cursor.set_position(0);
            let read_back = read_wstring(&mut cursor).unwrap();

            assert_eq!(original, read_back, "Failed for wstring: '{}'", original);
        }
    }

    #[test]
    fn test_string32l_roundtrip() {
        let test_cases = vec![
            ("", false),
            ("Test", false),
            ("This is a test string", false),
            ("Padded string", true),
        ];

        for (original, pad) in test_cases {
            let mut buffer = Vec::new();
            let mut cursor = Cursor::new(&mut buffer);

            write_string32l(&mut cursor, original, pad).unwrap();

            cursor.set_position(0);
            let read_back = read_string32l(&mut cursor, pad).unwrap();

            assert_eq!(
                original, read_back,
                "Failed for string32l: '{}' (pad={})",
                original, pad
            );
        }
    }

    #[test]
    fn test_packed_word_roundtrip() {
        // PackedWORD can only encode values in range [0, 32767] (15 bits with sign)
        // due to the high bit being used as a length flag
        let test_cases = vec![0i16, 1, 127, 128, 255, 256, 1000, 32767];

        for original in test_cases {
            let mut buffer = Vec::new();
            let mut cursor = Cursor::new(&mut buffer);

            write_packed_word(&mut cursor, original).unwrap();

            cursor.set_position(0);
            let read_back = read_packed_word(&mut cursor).unwrap();

            assert_eq!(original, read_back, "Failed for packed_word: {}", original);
        }
    }

    #[test]
    fn test_packed_dword_roundtrip() {
        // PackedDWORD can encode:
        // - Values < 0x8000 in 2 bytes
        // - Values >= 0x8000 and < 0x80000000 in 4 bytes (31 bits)
        // The high bit of the first word is used as a length flag
        let test_cases = vec![
            0u32, 1, 127, 128, 255, 256, 0x7FFF, 0x8000, 0xFFFF, 0x10000,
            0x7FFFFFFF, // Max value that can be encoded
        ];

        for original in test_cases {
            let mut buffer = Vec::new();
            let mut cursor = Cursor::new(&mut buffer);

            write_packed_dword(&mut cursor, original).unwrap();

            cursor.set_position(0);
            let read_back = read_packed_dword(&mut cursor).unwrap();

            assert_eq!(original, read_back, "Failed for packed_dword: {}", original);
        }
    }

    #[test]
    fn test_list_roundtrip() {
        let original = vec![1u32, 2, 3, 4, 5];
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_list(&mut cursor, &original).unwrap();

        cursor.set_position(0);
        let read_back = read_list::<u32>(&mut cursor).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_vec_roundtrip() {
        let original = vec![10i16, 20, 30];
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_vec(&mut cursor, &original).unwrap();

        cursor.set_position(0);
        let read_back = read_vec::<i16>(&mut cursor, original.len()).unwrap();

        assert_eq!(original, read_back);
    }

    #[test]
    fn test_packable_list_roundtrip() {
        let original = PackableList {
            count: 3,
            list: vec![100u32, 200, 300],
        };
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_packable_list(&mut cursor, &original).unwrap();

        cursor.set_position(0);
        let read_back = read_packable_list::<u32>(&mut cursor).unwrap();

        assert_eq!(original.count, read_back.count);
        assert_eq!(original.list, read_back.list);
    }

    #[test]
    fn test_packable_hash_table_roundtrip() {
        let mut table = std::collections::HashMap::new();
        table.insert(1u32, 100u32);
        table.insert(2, 200);
        table.insert(3, 300);

        let original = PackableHashTable {
            count: 3,
            max_size: 10,
            table,
        };

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_packable_hash_table(&mut cursor, &original).unwrap();

        cursor.set_position(0);
        let read_back = read_packable_hash_table::<u32, u32>(&mut cursor).unwrap();

        assert_eq!(original.count, read_back.count);
        assert_eq!(original.max_size, read_back.max_size);
        assert_eq!(original.table, read_back.table);
    }

    #[test]
    fn test_phash_table_roundtrip() {
        let mut table = std::collections::HashMap::new();
        table.insert(10u32, 1000u32);
        table.insert(20, 2000);

        let original = PHashTable {
            packed_size: 2, // count in lower 24 bits
            table,
        };

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        write_phash_table(&mut cursor, &original).unwrap();

        cursor.set_position(0);
        let read_back = read_phash_table::<u32, u32>(&mut cursor).unwrap();

        assert_eq!(original.packed_size, read_back.packed_size);
        assert_eq!(original.table, read_back.table);
    }

    #[test]
    fn test_alignment_write() {
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        // Write 1 byte
        write_u8(&mut cursor, 0xFF).unwrap();
        assert_eq!(cursor.position(), 1);

        // Align to 4-byte boundary (should write 3 padding bytes)
        align_dword_write(&mut cursor).unwrap();
        assert_eq!(cursor.position(), 4);

        // Write another byte
        write_u8(&mut cursor, 0xAA).unwrap();
        assert_eq!(cursor.position(), 5);

        // Align to 4-byte boundary (should write 3 padding bytes)
        align_dword_write(&mut cursor).unwrap();
        assert_eq!(cursor.position(), 8);

        // Verify buffer contents
        assert_eq!(buffer, vec![0xFF, 0, 0, 0, 0xAA, 0, 0, 0]);
    }

    #[test]
    fn test_acwritable_trait() {
        // Test that primitive types implement ACWritable
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        let value: u32 = 42;
        value.write(&mut cursor).unwrap();

        cursor.set_position(0);
        let read_back = read_u32(&mut cursor).unwrap();
        assert_eq!(value, read_back);
    }
}
