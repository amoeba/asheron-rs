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

// Add/Update a member to your fellowship.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_UpdateFellow")]
pub struct FellowshipUpdateFellow {
    #[serde(rename = "Fellow")]
    pub fellow: Fellow,
    #[serde(rename = "UpdateType")]
    pub update_type: FellowUpdateType,
}

impl FellowshipUpdateFellow {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let fellow = Fellow::read(reader)?;
        let update_type = FellowUpdateType::try_from(read_u32(reader)?)?;

        Ok(Self {
            fellow,
            update_type,
        })
    }
}

impl crate::readers::ACDataType for FellowshipUpdateFellow {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipUpdateFellow::read(reader)
    }
}

