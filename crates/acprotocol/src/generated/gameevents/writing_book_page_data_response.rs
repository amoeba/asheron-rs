use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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
        let object_id = ObjectId::read(reader)?;
        let page = read_u32(reader)?;
        let page_data = PageData::read(reader)?;

        Ok(Self {
            object_id,
            page,
            page_data,
        })
    }
}

