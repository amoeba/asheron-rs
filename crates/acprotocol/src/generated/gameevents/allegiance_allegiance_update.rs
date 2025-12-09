use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Returns info related to your monarch, patron and vassals.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdate")]
pub struct AllegianceAllegianceUpdate {
    #[serde(rename = "Rank")]
    pub rank: u32,
    #[serde(rename = "Profile")]
    pub profile: AllegianceProfile,
}

impl AllegianceAllegianceUpdate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let rank = read_u32(reader)?;
        let profile = AllegianceProfile::read(reader)?;

        Ok(Self {
            rank,
            profile,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceUpdate::read(reader)
    }
}

