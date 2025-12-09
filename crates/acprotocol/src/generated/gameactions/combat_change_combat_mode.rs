use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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

