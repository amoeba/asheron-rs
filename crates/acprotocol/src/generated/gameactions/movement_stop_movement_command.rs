use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Stops a movement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_StopMovementCommand")]
pub struct MovementStopMovementCommand {
    #[serde(rename = "Motion")]
    pub motion: u32,
    #[serde(rename = "HoldKey")]
    pub hold_key: HoldKey,
}

impl crate::readers::ACDataType for MovementStopMovementCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let motion = read_u32(reader)?;
        let hold_key = HoldKey::try_from(read_u32(reader)?)?;

        Ok(Self {
            motion,
            hold_key,
        })
    }
}

