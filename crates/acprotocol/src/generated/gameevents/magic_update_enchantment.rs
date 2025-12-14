use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Apply an enchantment to your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateEnchantment")]
pub struct MagicUpdateEnchantment {
    #[serde(rename = "Enchantment")]
    pub enchantment: Enchantment,
}

impl crate::readers::ACDataType for MagicUpdateEnchantment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let enchantment = Enchantment::read(reader)?;

        Ok(Self {
            enchantment,
        })
    }
}

