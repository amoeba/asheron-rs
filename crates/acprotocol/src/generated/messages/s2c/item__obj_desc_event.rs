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

// Sent whenever a character changes their clothes. It contains the entire description of what their wearing (and possibly their facial features as well). This message is only sent for changes, when the character is first created, the body of this message is included inside the creation message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ObjDescEvent")]
pub struct ItemObjDescEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
    #[serde(rename = "InstanceSequence")]
    pub instance_sequence: u16,
    #[serde(rename = "VisualDescSequence")]
    pub visual_desc_sequence: u16,
}

impl ItemObjDescEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_description = ObjDesc::read(reader)?;
        let instance_sequence = read_u16(reader)?;
        let visual_desc_sequence = read_u16(reader)?;

        Ok(Self {
            object_id,
            object_description,
            instance_sequence,
            visual_desc_sequence,
        })
    }
}

impl crate::readers::ACDataType for ItemObjDescEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemObjDescEvent::read(reader)
    }
}

