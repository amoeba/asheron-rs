use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Add a spell to a spell bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AddSpellFavorite")]
pub struct CharacterAddSpellFavorite {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
    #[serde(rename = "Index")]
    pub index: u32,
    #[serde(rename = "SpellBar")]
    pub spell_bar: u32,
}

impl crate::readers::ACDataType for CharacterAddSpellFavorite {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;
        let index = read_u32(reader)?;
        let spell_bar = read_u32(reader)?;

        Ok(Self {
            spell_id,
            index,
            spell_bar,
        })
    }
}

