use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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
        let object_id = ObjectId::read(reader)?;
        let page_num = read_i32(reader)?;
        let page_text = read_string(reader)?;

        Ok(Self {
            object_id,
            page_num,
            page_text,
        })
    }
}

