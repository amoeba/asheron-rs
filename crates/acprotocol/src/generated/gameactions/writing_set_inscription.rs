use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Sets the inscription on an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_SetInscription")]
pub struct WritingSetInscription {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Inscription")]
    pub inscription: String,
}

impl WritingSetInscription {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let inscription = read_string(reader)?;

        Ok(Self {
            object_id,
            inscription,
        })
    }
}

impl crate::readers::ACDataType for WritingSetInscription {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingSetInscription::read(reader)
    }
}

