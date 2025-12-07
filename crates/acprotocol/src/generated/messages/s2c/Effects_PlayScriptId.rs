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

// Instructs the client to play a script. (Not seen so far, maybe prefered PlayScriptType)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayScriptId")]
pub struct EffectsPlayScriptId {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ScriptId")]
    pub script_id: DataId,
}

impl EffectsPlayScriptId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let script_id = DataId::read(reader)?;

        Ok(Self {
            object_id,
            script_id,
        })
    }
}

impl crate::readers::ACDataType for EffectsPlayScriptId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EffectsPlayScriptId::read(reader)
    }
}

