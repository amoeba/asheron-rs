use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Asks server for a new object description
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Object_SendForceObjdesc")]
pub struct ObjectSendForceObjdesc {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

impl ObjectSendForceObjdesc {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for ObjectSendForceObjdesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ObjectSendForceObjdesc::read(reader)
    }
}

