use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Cast a spell with no target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_CastUntargetedSpell")]
pub struct MagicCastUntargetedSpell {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

impl MagicCastUntargetedSpell {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for MagicCastUntargetedSpell {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicCastUntargetedSpell::read(reader)
    }
}

