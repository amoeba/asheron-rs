use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Fellowship Change openness
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_ChangeFellowOpeness")]
pub struct FellowshipChangeFellowOpeness {
    #[serde(rename = "Open")]
    pub open: bool,
}

impl FellowshipChangeFellowOpeness {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let open = read_bool(reader)?;

        Ok(Self {
            open,
        })
    }
}

impl crate::readers::ACDataType for FellowshipChangeFellowOpeness {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipChangeFellowOpeness::read(reader)
    }
}

