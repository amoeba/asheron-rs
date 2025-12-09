use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// House Profile
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseTransaction")]
pub struct HouseHouseTransaction {
    #[serde(rename = "NoticeType")]
    pub notice_type: u32,
}

impl HouseHouseTransaction {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let notice_type = read_u32(reader)?;

        Ok(Self {
            notice_type,
        })
    }
}

impl crate::readers::ACDataType for HouseHouseTransaction {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseHouseTransaction::read(reader)
    }
}

