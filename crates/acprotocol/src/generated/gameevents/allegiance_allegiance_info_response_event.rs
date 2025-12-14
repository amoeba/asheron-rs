use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Returns data for a player's allegiance information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceInfoResponseEvent")]
pub struct AllegianceAllegianceInfoResponseEvent {
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "Profile")]
    pub profile: AllegianceProfile,
}

impl crate::readers::ACDataType for AllegianceAllegianceInfoResponseEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_id = ObjectId::read(reader)?;
        let profile = AllegianceProfile::read(reader)?;

        Ok(Self {
            target_id,
            profile,
        })
    }
}

