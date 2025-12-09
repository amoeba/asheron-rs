use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Changes an objects vector, for things like jumping
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_VectorUpdate")]
pub struct MovementVectorUpdate {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Velocity")]
    pub velocity: Vector3,
    #[serde(rename = "Omega")]
    pub omega: Vector3,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectVectorSequence")]
    pub object_vector_sequence: u16,
}

impl MovementVectorUpdate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let velocity = Vector3::read(reader)?;
        let omega = Vector3::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let object_vector_sequence = read_u16(reader)?;

        Ok(Self {
            object_id,
            velocity,
            omega,
            object_instance_sequence,
            object_vector_sequence,
        })
    }
}

impl crate::readers::ACDataType for MovementVectorUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementVectorUpdate::read(reader)
    }
}

