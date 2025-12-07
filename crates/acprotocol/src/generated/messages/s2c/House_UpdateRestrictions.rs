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

// Update Restrictions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRestrictions")]
pub struct HouseUpdateRestrictions {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "Restrictions")]
    pub restrictions: RestrictionDB,
}

impl HouseUpdateRestrictions {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let sender_id = ObjectId::read(reader)?;
        let restrictions = RestrictionDB::read(reader)?;

        Ok(Self {
            sequence,
            sender_id,
            restrictions,
        })
    }
}

impl crate::readers::ACDataType for HouseUpdateRestrictions {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseUpdateRestrictions::read(reader)
    }
}

