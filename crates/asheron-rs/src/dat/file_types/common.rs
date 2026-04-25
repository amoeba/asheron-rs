use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Result, Seek, SeekFrom};

/// C# `ReadCompressedUInt32`: 1, 2, or 4 bytes depending on high bits.
pub(crate) fn read_compressed_u32<R: Read>(r: &mut R) -> Result<u32> {
    let b0 = r.read_u8()?;
    if (b0 & 0x80) == 0 {
        return Ok(b0 as u32);
    }
    let b1 = r.read_u8()?;
    if (b0 & 0x40) == 0 {
        return Ok((((b0 & 0x7F) as u32) << 8) | b1 as u32);
    }
    let hi = r.read_u16::<LittleEndian>()? as u32;
    Ok(((((b0 & 0x3F) as u32) << 8 | b1 as u32) << 16) | hi)
}

/// C# `BinaryReader.ReadString()`: 7-bit encoded length then UTF-8 bytes.
pub(crate) fn read_ac_string<R: Read>(r: &mut R) -> Result<String> {
    let mut len: u32 = 0;
    let mut shift = 0u32;
    loop {
        let b = r.read_u8()?;
        len |= ((b & 0x7F) as u32) << shift;
        if (b & 0x80) == 0 {
            break;
        }
        shift += 7;
    }
    let mut buf = vec![0u8; len as usize];
    r.read_exact(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

/// C# `ReadAsDataIDOfKnownType`: 2-byte or 4-byte partial ID added to base.
pub(crate) fn read_data_id_of_known_type<R: Read>(r: &mut R, known_type: u32) -> Result<u32> {
    let value = r.read_u16::<LittleEndian>()? as u32;
    if (value & 0x8000) != 0 {
        let lower = r.read_u16::<LittleEndian>()? as u32;
        let higher = (value & 0x3FFF) << 16;
        return Ok(known_type + (higher | lower));
    }
    Ok(known_type + value)
}

/// Align the stream to the next 4-byte boundary (C# `AlignBoundary`).
pub(crate) fn align_boundary<R: Read + Seek>(r: &mut R) -> Result<()> {
    let pos = r.stream_position()?;
    let delta = pos % 4;
    if delta != 0 {
        r.seek(SeekFrom::Current((4 - delta) as i64))?;
    }
    Ok(())
}

pub(crate) fn read_smart_array_u32<R: Read>(r: &mut R) -> Result<Vec<u32>> {
    let count = read_compressed_u32(r)? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(r.read_u32::<LittleEndian>()?);
    }
    Ok(out)
}

pub(crate) fn read_smart_array_i32<R: Read>(r: &mut R) -> Result<Vec<i32>> {
    let count = read_compressed_u32(r)? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(r.read_i32::<LittleEndian>()?);
    }
    Ok(out)
}
