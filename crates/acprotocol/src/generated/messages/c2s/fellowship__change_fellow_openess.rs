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

