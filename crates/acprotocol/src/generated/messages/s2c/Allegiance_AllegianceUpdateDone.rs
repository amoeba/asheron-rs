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

// Allegiance update finished
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdateDone")]
pub struct AllegianceAllegianceUpdateDone {
    #[serde(rename = "FailureType")]
    pub failure_type: WeenieError,
}

impl AllegianceAllegianceUpdateDone {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let failure_type = WeenieError::try_from(read_u32(reader)?)?;

        Ok(Self {
            failure_type,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceUpdateDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceUpdateDone::read(reader)
    }
}

