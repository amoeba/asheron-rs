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

// Boots a player from the allegiance, optionally all characters on their account
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_BreakAllegianceBoot")]
pub struct AllegianceBreakAllegianceBoot {
    #[serde(rename = "BooteeName")]
    pub bootee_name: String,
    #[serde(rename = "AccountBoot")]
    pub account_boot: bool,
}

impl AllegianceBreakAllegianceBoot {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let bootee_name = read_string(reader)?;
        let account_boot = read_bool(reader)?;

        Ok(Self {
            bootee_name,
            account_boot,
        })
    }
}

impl crate::readers::ACDataType for AllegianceBreakAllegianceBoot {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceBreakAllegianceBoot::read(reader)
    }
}

