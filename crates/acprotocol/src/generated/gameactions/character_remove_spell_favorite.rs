use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Remove a spell from a spell bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RemoveSpellFavorite")]
pub struct CharacterRemoveSpellFavorite {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
    #[serde(rename = "SpellBar")]
    pub spell_bar: u32,
}

impl crate::readers::ACDataType for CharacterRemoveSpellFavorite {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;
        let spell_bar = read_u32(reader)?;

        Ok(Self {
            spell_id,
            spell_bar,
        })
    }
}

