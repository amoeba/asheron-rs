use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Close Assess Panel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_AppraiseDone")]
pub struct ItemAppraiseDone {
    #[serde(rename = "Unknown")]
    pub unknown: u32,
}

impl crate::readers::ACDataType for ItemAppraiseDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown = read_u32(reader)?;

        Ok(Self {
            unknown,
        })
    }
}

