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

// Create or join a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_FullUpdate")]
pub struct FellowshipFullUpdate {
    #[serde(rename = "Fellowship")]
    pub fellowship: Fellowship,
}

impl FellowshipFullUpdate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let fellowship = Fellowship::read(reader)?;

        Ok(Self {
            fellowship,
        })
    }
}

impl crate::readers::ACDataType for FellowshipFullUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipFullUpdate::read(reader)
    }
}

