use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Applies an effect with visual and sound by providing the type to be looked up in the Physics Script Table
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayScriptType")]
pub struct EffectsPlayScriptType {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ScriptType")]
    pub script_type: i32,
    #[serde(rename = "Speed")]
    pub speed: f32,
}

impl EffectsPlayScriptType {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let script_type = read_i32(reader)?;
        let speed = read_f32(reader)?;

        Ok(Self {
            object_id,
            script_type,
            speed,
        })
    }
}

impl crate::readers::ACDataType for EffectsPlayScriptType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EffectsPlayScriptType::read(reader)
    }
}

