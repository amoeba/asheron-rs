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

// Request update to book data (seems to be sent after failed add page)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookAddPage")]
pub struct WritingBookAddPage {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

impl WritingBookAddPage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for WritingBookAddPage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookAddPage::read(reader)
    }
}

