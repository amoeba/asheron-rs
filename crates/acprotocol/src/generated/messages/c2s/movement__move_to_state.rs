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

// Move to state data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_MoveToState")]
pub struct MovementMoveToState {
    #[serde(rename = "MoveToState")]
    pub move_to_state: MoveToStatePack,
}

impl MovementMoveToState {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let move_to_state = MoveToStatePack::read(reader)?;

        Ok(Self {
            move_to_state,
        })
    }
}

impl crate::readers::ACDataType for MovementMoveToState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementMoveToState::read(reader)
    }
}

