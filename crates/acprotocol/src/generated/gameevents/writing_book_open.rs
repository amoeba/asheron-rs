use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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

impl WritingBookOpen {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let book_id = ObjectId::read(reader)?;
        let max_num_pages = read_u32(reader)?;
        let page_data = PageDataList::read(reader)?;
        let inscription = read_string(reader)?;
        let scribe_id = ObjectId::read(reader)?;
        let scribe_name = read_string(reader)?;

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

impl crate::readers::ACDataType for WritingBookOpen {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookOpen::read(reader)
    }
}

