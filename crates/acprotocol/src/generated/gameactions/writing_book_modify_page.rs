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

// Updates a page in a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookModifyPage")]
pub struct WritingBookModifyPage {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "PageNum")]
    pub page_num: i32,
    #[serde(rename = "PageText")]
    pub page_text: String,
}

impl crate::readers::ACDataType for WritingBookModifyPage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WritingBookModifyPage").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_page_num = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PageNum", position = pos).entered()
        };
        let page_num = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_page_num);
        #[cfg(feature = "tracing")]
        let _field_span_page_text = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PageText", position = pos).entered()
        };
        let page_text = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_page_text);

        Ok(Self {
            object_id,
            page_num,
            page_text,
        })
    }
}

impl crate::writers::ACWritable for WritingBookModifyPage {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WritingBookModifyPage").entered();

        self.object_id.write(writer)?;
        write_i32(writer, self.page_num)?;
        write_string(writer, &self.page_text)?;
        Ok(())
    }
}

