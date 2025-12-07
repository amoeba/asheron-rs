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

// Removes a player ban from the allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_RemoveAllegianceBan")]
pub struct AllegianceRemoveAllegianceBan {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

impl AllegianceRemoveAllegianceBan {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;

        Ok(Self {
            character_name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceRemoveAllegianceBan {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceRemoveAllegianceBan::read(reader)
    }
}

