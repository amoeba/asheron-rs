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

// Removes an allegiance officer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_RemoveAllegianceOfficer")]
pub struct AllegianceRemoveAllegianceOfficer {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

impl AllegianceRemoveAllegianceOfficer {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;

        Ok(Self {
            character_name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceRemoveAllegianceOfficer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceRemoveAllegianceOfficer::read(reader)
    }
}

