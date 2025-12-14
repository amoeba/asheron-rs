use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Set's the current state of the object. Client appears to only process the following state changes post creation: NoDraw, LightingOn, Hidden
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_SetState")]
pub struct ItemSetState {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "NewState")]
    pub new_state: PhysicsState,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectStateSequence")]
    pub object_state_sequence: u16,
}

impl crate::readers::ACDataType for ItemSetState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let new_state = Ok::<_, Box<dyn std::error::Error>>(PhysicsState::from_bits_retain(read_u32(reader)?))?;
        let object_instance_sequence = read_u16(reader)?;
        let object_state_sequence = read_u16(reader)?;

        Ok(Self {
            object_id,
            new_state,
            object_instance_sequence,
            object_state_sequence,
        })
    }
}

