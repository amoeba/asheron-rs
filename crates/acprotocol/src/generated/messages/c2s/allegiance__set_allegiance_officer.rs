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

// Sets an allegiance officer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceOfficer")]
pub struct AllegianceSetAllegianceOfficer {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "Level")]
    pub level: AllegianceOfficerLevel,
}

impl AllegianceSetAllegianceOfficer {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;
        let level = AllegianceOfficerLevel::try_from(read_u32(reader)?)?;

        Ok(Self {
            character_name,
            level,
        })
    }
}

impl crate::readers::ACDataType for AllegianceSetAllegianceOfficer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceSetAllegianceOfficer::read(reader)
    }
}

