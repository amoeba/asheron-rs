use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// A portal storm is brewing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Misc_PortalStormBrewing")]
pub struct MiscPortalStormBrewing {
    #[serde(rename = "Extent")]
    pub extent: f32,
}

impl crate::readers::ACDataType for MiscPortalStormBrewing {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let extent = read_f32(reader)?;

        Ok(Self {
            extent,
        })
    }
}

