use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Sets the player visual desc, TODO confirm this
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SetPlayerVisualDesc")]
pub struct CharacterSetPlayerVisualDesc {
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
}

impl crate::readers::ACDataType for CharacterSetPlayerVisualDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_description = ObjDesc::read(reader)?;

        Ok(Self {
            object_description,
        })
    }
}

