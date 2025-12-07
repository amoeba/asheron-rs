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

// QueryHealthResponse: Update a creature's health bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_QueryHealthResponse")]
pub struct CombatQueryHealthResponse {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Health")]
    pub health: f32,
}

impl CombatQueryHealthResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let health = read_f32(reader)?;

        Ok(Self {
            object_id,
            health,
        })
    }
}

impl crate::readers::ACDataType for CombatQueryHealthResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatQueryHealthResponse::read(reader)
    }
}

