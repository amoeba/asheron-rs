use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Perform the allegiance lock action
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_DoAllegianceLockAction")]
pub struct AllegianceDoAllegianceLockAction {
    #[serde(rename = "Action")]
    pub action: AllegianceLockAction,
}

impl crate::readers::ACDataType for AllegianceDoAllegianceLockAction {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let action = AllegianceLockAction::try_from(read_u32(reader)?)?;

        Ok(Self {
            action,
        })
    }
}

