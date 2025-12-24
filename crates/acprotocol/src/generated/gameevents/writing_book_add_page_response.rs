use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Response to an attempt to add a page to a book.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookAddPageResponse")]
pub struct WritingBookAddPageResponse {
    #[serde(rename = "BookId")]
    pub book_id: ObjectId,
    #[serde(rename = "PageNumber")]
    pub page_number: u32,
    #[serde(rename = "Success")]
    pub success: bool,
}

impl crate::readers::ACDataType for WritingBookAddPageResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WritingBookAddPageResponse").entered();

        #[cfg(feature = "tracing")]
        let _field_span_book_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BookId", position = pos).entered()
        };
        let book_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_book_id);
        #[cfg(feature = "tracing")]
        let _field_span_page_number = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PageNumber", position = pos).entered()
        };
        let page_number = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_page_number);
        #[cfg(feature = "tracing")]
        let _field_span_success = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Success", position = pos).entered()
        };
        let success = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_success);

        Ok(Self {
            book_id,
            page_number,
            success,
        })
    }
}

impl crate::writers::ACWritable for WritingBookAddPageResponse {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WritingBookAddPageResponse").entered();

        self.book_id.write(writer)?;
        write_u32(writer, self.page_number)?;
        write_bool(writer, self.success)?;
        Ok(())
    }
}

