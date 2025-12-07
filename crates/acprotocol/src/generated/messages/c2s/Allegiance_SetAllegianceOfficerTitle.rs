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

// Sets an allegiance officer title for a given level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceOfficerTitle")]
pub struct AllegianceSetAllegianceOfficerTitle {
    #[serde(rename = "Level")]
    pub level: AllegianceOfficerLevel,
    #[serde(rename = "Title")]
    pub title: String,
}

impl AllegianceSetAllegianceOfficerTitle {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let level = AllegianceOfficerLevel::try_from(read_u32(reader)?)?;
        let title = read_string(reader)?;

        Ok(Self {
            level,
            title,
        })
    }
}

impl crate::readers::ACDataType for AllegianceSetAllegianceOfficerTitle {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceSetAllegianceOfficerTitle::read(reader)
    }
}

