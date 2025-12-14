use serde::{Serialize, Deserialize};
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

impl crate::readers::ACDataType for FellowshipChangeFellowOpeness {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let open = read_bool(reader)?;

        Ok(Self {
            open,
        })
    }
}

