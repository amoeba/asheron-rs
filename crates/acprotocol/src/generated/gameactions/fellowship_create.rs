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

// Create a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Create")]
pub struct FellowshipCreate {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ShareXP")]
    pub share_xp: bool,
}

impl FellowshipCreate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let name = read_string(reader)?;
        let share_xp = read_bool(reader)?;

        Ok(Self {
            name,
            share_xp,
        })
    }
}

impl crate::readers::ACDataType for FellowshipCreate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipCreate::read(reader)
    }
}

