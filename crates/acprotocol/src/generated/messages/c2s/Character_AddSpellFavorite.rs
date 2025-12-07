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

impl CharacterAddSpellFavorite {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
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

impl crate::readers::ACDataType for CharacterAddSpellFavorite {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterAddSpellFavorite::read(reader)
    }
}

