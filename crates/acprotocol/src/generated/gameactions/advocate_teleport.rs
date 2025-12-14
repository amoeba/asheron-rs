use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Advocate Teleport
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Advocate_Teleport")]
pub struct AdvocateTeleport {
    #[serde(rename = "ObjectId")]
    pub object_id: String,
    #[serde(rename = "Destination")]
    pub destination: Position,
}

impl crate::readers::ACDataType for AdvocateTeleport {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = read_string(reader)?;
        let destination = Position::read(reader)?;

        Ok(Self {
            object_id,
            destination,
        })
    }
}

