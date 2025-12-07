use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;

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

