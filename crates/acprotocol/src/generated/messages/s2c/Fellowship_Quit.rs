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

// Member left fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Quit")]
pub struct FellowshipQuit {
    #[serde(rename = "Disband")]
    pub disband: bool,
}

impl FellowshipQuit {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let disband = read_bool(reader)?;

        Ok(Self {
            disband,
        })
    }
}

impl crate::readers::ACDataType for FellowshipQuit {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipQuit::read(reader)
    }
}

