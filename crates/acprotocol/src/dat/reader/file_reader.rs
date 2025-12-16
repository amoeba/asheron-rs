#[cfg(feature = "dat-tokio")]
use futures::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, SeekFrom};

#[cfg(feature = "dat-tokio")]
use crate::dat::reader::range_reader::RangeReader;

/// File-based implementation of RangeReader using seek
#[cfg(feature = "dat-tokio")]
pub struct FileRangeReader<R> {
    reader: R,
}

#[cfg(feature = "dat-tokio")]
impl<R> FileRangeReader<R>
where
    R: AsyncRead + AsyncSeek + Unpin + Send,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

#[cfg(feature = "dat-tokio")]
impl<R> RangeReader for FileRangeReader<R>
where
    R: AsyncRead + AsyncSeek + Unpin + Send,
{
    fn read_range(
        &mut self,
        offset: u32,
        length: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Box<dyn std::error::Error>>> {
        async move {
            // Seek to the position
            self.reader.seek(SeekFrom::Start(offset.into())).await?;

            // Read exactly the requested bytes
            let mut buffer = vec![0u8; length];
            self.reader.read_exact(&mut buffer).await?;

            Ok(buffer)
        }
    }
}
