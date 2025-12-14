use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

// Sent whenever an object is being deleted from the scene.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_DeleteObject")]
pub struct ItemDeleteObject {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
}

impl crate::readers::ACDataType for ItemDeleteObject {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let _ = align_dword(reader)?;

        Ok(Self {
            object_id,
            object_instance_sequence,
        })
    }
}

