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

// Sent when you first open a book, contains the entire table of contents.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookOpen")]
pub struct WritingBookOpen {
    #[serde(rename = "BookId")]
    pub book_id: ObjectId,
    #[serde(rename = "MaxNumPages")]
    pub max_num_pages: u32,
    #[serde(rename = "PageData")]
    pub page_data: PageDataList,
    #[serde(rename = "Inscription")]
    pub inscription: String,
    #[serde(rename = "ScribeId")]
    pub scribe_id: ObjectId,
    #[serde(rename = "ScribeName")]
    pub scribe_name: String,
}

impl crate::readers::ACDataType for WritingBookOpen {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WritingBookOpen").entered();

        #[cfg(feature = "tracing")]
        let _field_span_book_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BookId", position = pos).entered()
        };
        let book_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_book_id);
        #[cfg(feature = "tracing")]
        let _field_span_max_num_pages = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxNumPages", position = pos).entered()
        };
        let max_num_pages = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_num_pages);
        #[cfg(feature = "tracing")]
        let _field_span_page_data = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PageData", position = pos).entered()
        };
        let page_data = PageDataList::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_page_data);
        #[cfg(feature = "tracing")]
        let _field_span_inscription = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Inscription", position = pos).entered()
        };
        let inscription = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_inscription);
        #[cfg(feature = "tracing")]
        let _field_span_scribe_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ScribeId", position = pos).entered()
        };
        let scribe_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_scribe_id);
        #[cfg(feature = "tracing")]
        let _field_span_scribe_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ScribeName", position = pos).entered()
        };
        let scribe_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_scribe_name);

        Ok(Self {
            book_id,
            max_num_pages,
            page_data,
            inscription,
            scribe_id,
            scribe_name,
        })
    }
}

impl crate::writers::ACWritable for WritingBookOpen {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WritingBookOpen").entered();

        self.book_id.write(writer)?;
        write_u32(writer, self.max_num_pages)?;
        self.page_data.write(writer)?;
        write_string(writer, &self.inscription)?;
        self.scribe_id.write(writer)?;
        write_string(writer, &self.scribe_name)?;
        Ok(())
    }
}

