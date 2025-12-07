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

// Sets the allegiance name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceName")]
pub struct AllegianceSetAllegianceName {
    #[serde(rename = "Name")]
    pub name: String,
}

impl AllegianceSetAllegianceName {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let name = read_string(reader)?;

        Ok(Self {
            name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceSetAllegianceName {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceSetAllegianceName::read(reader)
    }
}

