use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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
        let book_id = ObjectId::read(reader)?;
        let page_number = read_u32(reader)?;
        let success = read_bool(reader)?;

        Ok(Self {
            book_id,
            page_number,
            success,
        })
    }
}

