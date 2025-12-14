use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Add/Update a member to your fellowship.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_UpdateFellow")]
pub struct FellowshipUpdateFellow {
    #[serde(rename = "Fellow")]
    pub fellow: Fellow,
    #[serde(rename = "UpdateType")]
    pub update_type: FellowUpdateType,
}

impl crate::readers::ACDataType for FellowshipUpdateFellow {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let fellow = Fellow::read(reader)?;
        let update_type = FellowUpdateType::try_from(read_u32(reader)?)?;

        Ok(Self {
            fellow,
            update_type,
        })
    }
}

