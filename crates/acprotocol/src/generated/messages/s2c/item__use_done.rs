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

// UseDone: Ready. Previous action complete
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UseDone")]
pub struct ItemUseDone {
    #[serde(rename = "FailureType")]
    pub failure_type: WeenieError,
}

impl ItemUseDone {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let failure_type = WeenieError::try_from(read_u32(reader)?)?;

        Ok(Self {
            failure_type,
        })
    }
}

impl crate::readers::ACDataType for ItemUseDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemUseDone::read(reader)
    }
}

