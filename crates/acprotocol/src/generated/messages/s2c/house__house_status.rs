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

// House Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseStatus")]
pub struct HouseHouseStatus {
    #[serde(rename = "NoticeType")]
    pub notice_type: u32,
}

impl HouseHouseStatus {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let notice_type = read_u32(reader)?;

        Ok(Self {
            notice_type,
        })
    }
}

impl crate::readers::ACDataType for HouseHouseStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseHouseStatus::read(reader)
    }
}

