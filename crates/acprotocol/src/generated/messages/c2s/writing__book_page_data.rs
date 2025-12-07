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

// Requests data for a page in a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookPageData")]
pub struct WritingBookPageData {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "PageNum")]
    pub page_num: i32,
}

impl WritingBookPageData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let page_num = read_i32(reader)?;

        Ok(Self {
            object_id,
            page_num,
        })
    }
}

impl crate::readers::ACDataType for WritingBookPageData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookPageData::read(reader)
    }
}

