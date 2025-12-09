use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Sets both the position and movement, such as when materializing at a lifestone
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_PositionAndMovementEvent")]
pub struct MovementPositionAndMovementEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Position")]
    pub position: PositionPack,
    #[serde(rename = "MovementData")]
    pub movement_data: MovementData,
}

impl MovementPositionAndMovementEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let position = PositionPack::read(reader)?;
        let movement_data = MovementData::read(reader)?;

        Ok(Self {
            object_id,
            position,
            movement_data,
        })
    }
}

impl crate::readers::ACDataType for MovementPositionAndMovementEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementPositionAndMovementEvent::read(reader)
    }
}

