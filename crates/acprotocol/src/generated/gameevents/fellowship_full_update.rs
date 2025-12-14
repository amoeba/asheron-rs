use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Create or join a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_FullUpdate")]
pub struct FellowshipFullUpdate {
    #[serde(rename = "Fellowship")]
    pub fellowship: Fellowship,
}

impl crate::readers::ACDataType for FellowshipFullUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let fellowship = Fellowship::read(reader)?;

        Ok(Self {
            fellowship,
        })
    }
}

