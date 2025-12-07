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

// Add a spell to your spellbook.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateSpell")]
pub struct MagicUpdateSpell {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

impl MagicUpdateSpell {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for MagicUpdateSpell {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicUpdateSpell::read(reader)
    }
}

