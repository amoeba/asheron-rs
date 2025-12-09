use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Request allegiance info for a player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceInfoRequest")]
pub struct AllegianceAllegianceInfoRequest {
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

impl AllegianceAllegianceInfoRequest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_name = read_string(reader)?;

        Ok(Self {
            target_name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceInfoRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceInfoRequest::read(reader)
    }
}

