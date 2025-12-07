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

// Remove a spell from a spell bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RemoveSpellFavorite")]
pub struct CharacterRemoveSpellFavorite {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
    #[serde(rename = "SpellBar")]
    pub spell_bar: u32,
}

impl CharacterRemoveSpellFavorite {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;
        let spell_bar = read_u32(reader)?;

        Ok(Self {
            spell_id,
            spell_bar,
        })
    }
}

impl crate::readers::ACDataType for CharacterRemoveSpellFavorite {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterRemoveSpellFavorite::read(reader)
    }
}

