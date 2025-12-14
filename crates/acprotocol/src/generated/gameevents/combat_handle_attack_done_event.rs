use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// HandleAttackDoneEvent: Melee attack completed
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleAttackDoneEvent")]
pub struct CombatHandleAttackDoneEvent {
    #[serde(rename = "Number")]
    pub number: u32,
}

impl crate::readers::ACDataType for CombatHandleAttackDoneEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let number = read_u32(reader)?;

        Ok(Self {
            number,
        })
    }
}

