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

// A portal storm is imminent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Misc_PortalStormImminent")]
pub struct MiscPortalStormImminent {
    #[serde(rename = "Extent")]
    pub extent: f32,
}

impl MiscPortalStormImminent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let extent = read_f32(reader)?;

        Ok(Self {
            extent,
        })
    }
}

impl crate::readers::ACDataType for MiscPortalStormImminent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MiscPortalStormImminent::read(reader)
    }
}

