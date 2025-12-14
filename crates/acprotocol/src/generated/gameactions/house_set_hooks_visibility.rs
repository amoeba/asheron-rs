use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Modify whether house hooks are visibile or not, /house hooks on/off
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_SetHooksVisibility")]
pub struct HouseSetHooksVisibility {
    #[serde(rename = "Visible")]
    pub visible: bool,
}

impl crate::readers::ACDataType for HouseSetHooksVisibility {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let visible = read_bool(reader)?;

        Ok(Self {
            visible,
        })
    }
}

