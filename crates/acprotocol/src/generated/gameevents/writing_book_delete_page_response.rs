use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Response to an attempt to delete a page from a book.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookDeletePageResponse")]
pub struct WritingBookDeletePageResponse {
    #[serde(rename = "BookId")]
    pub book_id: ObjectId,
    #[serde(rename = "PageNumber")]
    pub page_number: u32,
    #[serde(rename = "Success")]
    pub success: bool,
}

impl WritingBookDeletePageResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
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

impl crate::readers::ACDataType for WritingBookDeletePageResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookDeletePageResponse::read(reader)
    }
}

