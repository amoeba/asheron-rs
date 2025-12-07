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

// Sets a person as an approved vassal
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceApprovedVassal")]
pub struct AllegianceSetAllegianceApprovedVassal {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

impl AllegianceSetAllegianceApprovedVassal {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;

        Ok(Self {
            character_name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceSetAllegianceApprovedVassal {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceSetAllegianceApprovedVassal::read(reader)
    }
}

