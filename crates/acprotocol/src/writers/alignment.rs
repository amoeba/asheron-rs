use super::ACWriter;
use std::error::Error;

/// Align to a boundary specified by a const generic parameter
/// This function writes zero-padding bytes to reach the next alignment boundary
/// This approach ensures the output is complete and valid, and works with non-seekable writers
///
/// # Example
/// ```ignore
/// # use acprotocol::writers::align_write;
/// # use std::io::Cursor;
/// # let mut buffer = Vec::new();
/// # let mut cursor = Cursor::new(&mut buffer);
/// align_write::<4>(&mut cursor)?;  // Align to 4-byte boundary
/// align_write::<8>(&mut cursor)?;  // Align to 8-byte boundary
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[inline]
pub fn align_write<const N: usize>(writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
    let pos = writer.stream_position()? as usize;
    let padding = (N - (pos % N)) % N;
    if padding > 0 {
        writer.write_all(&vec![0u8; padding])?;
    }
    Ok(())
}

/// Align to a DWORD (4-byte) boundary
#[inline]
pub fn align_dword_write(writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
    align_write::<4>(writer)
}

/// Align to a WORD (2-byte) boundary
#[inline]
pub fn align_word_write(writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
    align_write::<2>(writer)
}

/// Align to a QWORD (8-byte) boundary
#[inline]
pub fn align_qword_write(writer: &mut dyn ACWriter) -> Result<(), Box<dyn Error>> {
    align_write::<8>(writer)
}
