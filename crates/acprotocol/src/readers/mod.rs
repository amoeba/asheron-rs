use std::io::Read;

pub mod common {
    include!("../generated/readers/common.rs");
}

pub mod c2s {
    include!("../generated/readers/c2s.rs");
}

pub mod s2c {
    include!("../generated/readers/s2c.rs");
}

/// Read a single byte (u8)
pub fn read_u8(reader: &mut impl Read) -> Result<u8, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}

/// Read a signed byte (i8)
pub fn read_i8(reader: &mut impl Read) -> Result<i8, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    Ok(i8::from_le_bytes(buf))
}

/// Read a 16-bit unsigned integer (u16, little-endian)
pub fn read_u16(reader: &mut impl Read) -> Result<u16, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

/// Read a 16-bit signed integer (i16, little-endian)
pub fn read_i16(reader: &mut impl Read) -> Result<i16, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(i16::from_le_bytes(buf))
}

/// Read a 32-bit unsigned integer (u32, little-endian)
pub fn read_u32(reader: &mut impl Read) -> Result<u32, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

/// Read a 32-bit signed integer (i32, little-endian)
pub fn read_i32(reader: &mut impl Read) -> Result<i32, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

/// Read a 64-bit unsigned integer (u64, little-endian)
pub fn read_u64(reader: &mut impl Read) -> Result<u64, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

/// Read a 64-bit signed integer (i64, little-endian)
pub fn read_i64(reader: &mut impl Read) -> Result<i64, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(i64::from_le_bytes(buf))
}

/// Read a 32-bit floating point number (f32, little-endian)
pub fn read_f32(reader: &mut impl Read) -> Result<f32, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

/// Read a 64-bit floating point number (f64, little-endian)
pub fn read_f64(reader: &mut impl Read) -> Result<f64, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(f64::from_le_bytes(buf))
}

/// Read a boolean (represented as u32, 0 = false, non-zero = true)
pub fn read_bool(reader: &mut impl Read) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(read_u32(reader)? != 0)
}

/// Read a string (u16 length-prefixed, aligned to 4-byte boundary)
pub fn read_string(reader: &mut impl Read) -> Result<String, Box<dyn std::error::Error>> {
    let len = read_u16(reader)? as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;

    // Read padding to align to 4 bytes
    let padding = (4 - ((len + 2) % 4)) % 4;
    if padding > 0 {
        let mut pad_buf = vec![0u8; padding];
        reader.read_exact(&mut pad_buf)?;
    }

    Ok(String::from_utf8(buf)?)
}

/// Read an ObjectId (u32)
pub fn read_object_id(reader: &mut impl Read) -> Result<u32, Box<dyn std::error::Error>> {
    read_u32(reader)
}
