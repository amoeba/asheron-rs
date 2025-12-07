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

// Remove an enchantment from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_RemoveEnchantment")]
pub struct MagicRemoveEnchantment {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

impl MagicRemoveEnchantment {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for MagicRemoveEnchantment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicRemoveEnchantment::read(reader)
    }
}

