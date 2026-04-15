use std::error::Error;

#[cfg(feature = "dat-cloudflare")]
use worker::{Bucket, Range};

use crate::dat::reader::range_reader::RangeReader;

#[cfg(feature = "dat-cloudflare")]
pub struct WorkerR2RangeReader {
    bucket: Bucket,
    key: String,
}

#[cfg(feature = "dat-cloudflare")]
impl WorkerR2RangeReader {
    pub fn new(bucket: Bucket, key: String) -> Self {
        Self { bucket, key }
    }
}

#[cfg(feature = "dat-cloudflare")]
impl RangeReader for WorkerR2RangeReader {
    fn read_range(
        &mut self,
        offset: u32,
        length: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Box<dyn Error>>> {
        let bucket = self.bucket.clone();
        let key = self.key.clone();

        async move {
            let range = Range::OffsetWithLength {
                offset: offset as u64,
                length: length as u64,
            };

            let object = bucket
                .get(&key)
                .range(range)
                .execute()
                .await
                .map_err(|e| -> Box<dyn Error> { format!("R2 get failed: {:?}", e).into() })?;

            match object {
                Some(obj) => {
                    let stream = obj
                        .body()
                        .ok_or_else(|| -> Box<dyn Error> { "No body in R2 object".into() })?;

                    let bytes = stream.bytes().await.map_err(|e| -> Box<dyn Error> {
                        format!("Failed to read bytes: {:?}", e).into()
                    })?;
                    Ok(bytes.to_vec())
                }
                None => Err("Object not found in R2 bucket".into()),
            }
        }
    }
}
