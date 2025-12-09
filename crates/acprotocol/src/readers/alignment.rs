use super::ACReader;
use std::error::Error;
use std::io::SeekFrom;

/// Align to a boundary specified by a const generic parameter
/// This function is fully inlined and the const parameter allows
/// compile-time optimization of the alignment calculation
///
/// # Example
/// ```ignore
/// # use acprotocol::readers::align;
/// # use std::io::Cursor;
/// # let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
/// # let mut cursor = Cursor::new(data);
/// align::<4>(&mut cursor)?;  // Align to 4-byte boundary
/// align::<8>(&mut cursor)?;  // Align to 8-byte boundary
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[inline]
pub fn align<const N: usize>(reader: &mut dyn ACReader) -> Result<(), Box<dyn Error>> {
    let pos = reader.stream_position()? as usize;
    let padding = (N - (pos % N)) % N;
    if padding > 0 {
        reader.seek(SeekFrom::Current(padding as i64))?;
    }
    Ok(())
}

/// Align to a DWORD (4-byte) boundary
#[inline]
pub fn align_dword(reader: &mut dyn ACReader) -> Result<(), Box<dyn Error>> {
    align::<4>(reader)
}

/// Align to a WORD (2-byte) boundary
#[inline]
pub fn align_word(reader: &mut dyn ACReader) -> Result<(), Box<dyn Error>> {
    align::<2>(reader)
}

/// Align to a QWORD (8-byte) boundary
#[inline]
pub fn align_qword(reader: &mut dyn ACReader) -> Result<(), Box<dyn Error>> {
    align::<8>(reader)
}
