use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Update multiple enchantments from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateMultipleEnchantments")]
pub struct MagicUpdateMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    pub enchantments: PackableList<Enchantment>,
}

impl MagicUpdateMultipleEnchantments {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let enchantments = read_packable_list::<Enchantment>(reader)?;

        Ok(Self {
            enchantments,
        })
    }
}

impl crate::readers::ACDataType for MagicUpdateMultipleEnchantments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicUpdateMultipleEnchantments::read(reader)
    }
}

