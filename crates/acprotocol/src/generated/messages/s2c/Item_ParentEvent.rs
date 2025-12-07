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

// Sets the parent for an object, eg. equipting an object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ParentEvent")]
pub struct ItemParentEvent {
    #[serde(rename = "ParentId")]
    pub parent_id: ObjectId,
    #[serde(rename = "ChildId")]
    pub child_id: ObjectId,
    #[serde(rename = "Location")]
    pub location: ParentLocation,
    #[serde(rename = "Placement")]
    pub placement: Placement,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ChildPositionSequence")]
    pub child_position_sequence: u16,
}

impl ItemParentEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let parent_id = ObjectId::read(reader)?;
        let child_id = ObjectId::read(reader)?;
        let location = ParentLocation::try_from(read_u32(reader)?)?;
        let placement = Placement::try_from(read_u32(reader)?)?;
        let object_instance_sequence = read_u16(reader)?;
        let child_position_sequence = read_u16(reader)?;

        Ok(Self {
            parent_id,
            child_id,
            location,
            placement,
            object_instance_sequence,
            child_position_sequence,
        })
    }
}

impl crate::readers::ACDataType for ItemParentEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemParentEvent::read(reader)
    }
}

