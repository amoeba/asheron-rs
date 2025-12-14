use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// House Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseStatus")]
pub struct HouseHouseStatus {
    #[serde(rename = "NoticeType")]
    pub notice_type: u32,
}

impl crate::readers::ACDataType for HouseHouseStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let notice_type = read_u32(reader)?;

        Ok(Self {
            notice_type,
        })
    }
}

