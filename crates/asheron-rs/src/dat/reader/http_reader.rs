#[cfg(feature = "dat-http")]
use crate::dat::reader::range_reader::RangeReader;

#[cfg(feature = "dat-http")]
#[derive(Default)]
pub struct HttpRangeReaderOptions {
    allow_fallback_full_read: bool,
}

#[cfg(feature = "dat-http")]
pub struct HttpRangeReader {
    url: String,
    client: reqwest::Client,
    options: HttpRangeReaderOptions,
}

#[cfg(feature = "dat-http")]
impl HttpRangeReader {
    pub fn new(client: reqwest::Client, url: String, options: HttpRangeReaderOptions) -> Self {
        Self {
            url,
            client,
            options,
        }
    }

    /// Convenience constructor that creates a default client
    pub fn with_default_client(url: String) -> Self {
        Self::new(
            reqwest::Client::new(),
            url,
            HttpRangeReaderOptions::default(),
        )
    }
}

#[cfg(feature = "dat-http")]
impl RangeReader for HttpRangeReader {
    fn read_range(
        &mut self,
        offset: u32,
        length: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Box<dyn std::error::Error>>> {
        let end_byte = offset + length as u32 - 1;
        let range_header = format!("bytes={}-{}", offset, end_byte);
        let url = self.url.clone();
        let client = self.client.clone();

        async move {
            let response = client
                .get(&url)
                .header("Range", range_header)
                .send()
                .await?;

            // Check if the server supports range requests
            if response.status() == reqwest::StatusCode::PARTIAL_CONTENT {
                let bytes = response.bytes().await?;
                Ok(bytes.to_vec())
            } else if response.status().is_success() {
                if !self.options.allow_fallback_full_read {
                    return Err(
                        "HTTP server doesn't support range requests and falling back to reading the entire resource (allow_fallback_full_read) was disabled using options.".into(),
                    );
                }

                // Server doesn't support ranges, but returned full content
                // We'll take just the part we need
                let bytes = response.bytes().await?;
                let start = offset as usize;
                let end = std::cmp::min(start + length, bytes.len());

                if start >= bytes.len() {
                    return Err("Offset beyond file size".into());
                }

                Ok(bytes[start..end].to_vec())
            } else {
                Err(format!("HTTP request failed with status: {}", response.status()).into())
            }
        }
    }
}
