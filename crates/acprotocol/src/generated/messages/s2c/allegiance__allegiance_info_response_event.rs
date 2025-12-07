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

// Returns data for a player's allegiance information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceInfoResponseEvent")]
pub struct AllegianceAllegianceInfoResponseEvent {
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "Profile")]
    pub profile: AllegianceProfile,
}

impl AllegianceAllegianceInfoResponseEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_id = ObjectId::read(reader)?;
        let profile = AllegianceProfile::read(reader)?;

        Ok(Self {
            target_id,
            profile,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceInfoResponseEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceInfoResponseEvent::read(reader)
    }
}

