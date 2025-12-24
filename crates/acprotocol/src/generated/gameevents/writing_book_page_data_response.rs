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

// Contains the text of a single page of a book, parchment or sign.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookPageDataResponse")]
pub struct WritingBookPageDataResponse {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Page")]
    pub page: u32,
    #[serde(rename = "PageData")]
    pub page_data: PageData,
}

impl crate::readers::ACDataType for WritingBookPageDataResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WritingBookPageDataResponse").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_page = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Page", position = pos).entered()
        };
        let page = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_page);
        #[cfg(feature = "tracing")]
        let _field_span_page_data = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PageData", position = pos).entered()
        };
        let page_data = PageData::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_page_data);

        Ok(Self {
            object_id,
            page,
            page_data,
        })
    }
}

impl crate::writers::ACWritable for WritingBookPageDataResponse {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WritingBookPageDataResponse").entered();

        self.object_id.write(writer)?;
        write_u32(writer, self.page)?;
        self.page_data.write(writer)?;
        Ok(())
    }
}

