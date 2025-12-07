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

// Modify whether house hooks are visibile or not, /house hooks on/off
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_SetHooksVisibility")]
pub struct HouseSetHooksVisibility {
    #[serde(rename = "Visible")]
    pub visible: bool,
}

impl HouseSetHooksVisibility {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let visible = read_bool(reader)?;

        Ok(Self {
            visible,
        })
    }
}

impl crate::readers::ACDataType for HouseSetHooksVisibility {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseSetHooksVisibility::read(reader)
    }
}

