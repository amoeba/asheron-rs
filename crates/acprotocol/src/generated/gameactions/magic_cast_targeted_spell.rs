use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Cast a spell on a target
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_CastTargetedSpell")]
pub struct MagicCastTargetedSpell {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

impl MagicCastTargetedSpell {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            object_id,
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for MagicCastTargetedSpell {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicCastTargetedSpell::read(reader)
    }
}

