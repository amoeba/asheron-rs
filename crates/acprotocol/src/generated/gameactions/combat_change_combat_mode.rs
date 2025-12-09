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

// Changes the combat mode
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_ChangeCombatMode")]
pub struct CombatChangeCombatMode {
    #[serde(rename = "Mode")]
    pub mode: CombatMode,
}

impl CombatChangeCombatMode {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let mode = Ok::<_, Box<dyn std::error::Error>>(CombatMode::from_bits_retain(read_u32(reader)?))?;

        Ok(Self {
            mode,
        })
    }
}

impl crate::readers::ACDataType for CombatChangeCombatMode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatChangeCombatMode::read(reader)
    }
}

