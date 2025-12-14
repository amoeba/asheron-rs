use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Move to state data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_MoveToState")]
pub struct MovementMoveToState {
    #[serde(rename = "MoveToState")]
    pub move_to_state: MoveToStatePack,
}

impl crate::readers::ACDataType for MovementMoveToState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let move_to_state = MoveToStatePack::read(reader)?;

        Ok(Self {
            move_to_state,
        })
    }
}

