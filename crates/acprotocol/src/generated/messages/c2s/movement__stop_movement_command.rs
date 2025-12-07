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

// Stops a movement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_StopMovementCommand")]
pub struct MovementStopMovementCommand {
    #[serde(rename = "Motion")]
    pub motion: u32,
    #[serde(rename = "HoldKey")]
    pub hold_key: HoldKey,
}

impl MovementStopMovementCommand {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let motion = read_u32(reader)?;
        let hold_key = HoldKey::try_from(read_u32(reader)?)?;

        Ok(Self {
            motion,
            hold_key,
        })
    }
}

impl crate::readers::ACDataType for MovementStopMovementCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementStopMovementCommand::read(reader)
    }
}

