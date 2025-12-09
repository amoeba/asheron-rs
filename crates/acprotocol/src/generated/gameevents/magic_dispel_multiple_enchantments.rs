use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Silently remove multiple enchantments from your character (no message in the chat window).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_DispelMultipleEnchantments")]
pub struct MagicDispelMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    pub enchantments: PackableList<LayeredSpellId>,
}

impl MagicDispelMultipleEnchantments {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let enchantments = read_packable_list::<LayeredSpellId>(reader)?;

        Ok(Self {
            enchantments,
        })
    }
}

impl crate::readers::ACDataType for MagicDispelMultipleEnchantments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicDispelMultipleEnchantments::read(reader)
    }
}

