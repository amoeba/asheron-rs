use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Quit the fellowship
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

