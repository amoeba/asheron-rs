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

// These are animations. Whenever a human, monster or object moves - one of these little messages is sent. Even idle emotes (like head scratching and nodding) are sent in this manner.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_SetObjectMovement")]
pub struct MovementSetObjectMovement {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "MovementData")]
    pub movement_data: MovementData,
}

impl MovementSetObjectMovement {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let movement_data = MovementData::read(reader)?;

        Ok(Self {
            object_id,
            object_instance_sequence,
            movement_data,
        })
    }
}

impl crate::readers::ACDataType for MovementSetObjectMovement {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementSetObjectMovement::read(reader)
    }
}

