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

// Close Assess Panel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_AppraiseDone")]
pub struct ItemAppraiseDone {
    #[serde(rename = "Unknown")]
    pub unknown: u32,
}

impl ItemAppraiseDone {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown = read_u32(reader)?;

        Ok(Self {
            unknown,
        })
    }
}

impl crate::readers::ACDataType for ItemAppraiseDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemAppraiseDone::read(reader)
    }
}

