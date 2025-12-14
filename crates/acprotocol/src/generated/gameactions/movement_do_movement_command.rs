use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Performs a movement based on input
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_DoMovementCommand")]
pub struct MovementDoMovementCommand {
    #[serde(rename = "Motion")]
    pub motion: u32,
    #[serde(rename = "Speed")]
    pub speed: f32,
    #[serde(rename = "HoldKey")]
    pub hold_key: HoldKey,
}

impl crate::readers::ACDataType for MovementDoMovementCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let motion = read_u32(reader)?;
        let speed = read_f32(reader)?;
        let hold_key = HoldKey::try_from(read_u32(reader)?)?;

        Ok(Self {
            motion,
            speed,
            hold_key,
        })
    }
}

