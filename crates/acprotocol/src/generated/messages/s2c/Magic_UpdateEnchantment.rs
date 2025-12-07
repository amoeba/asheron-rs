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

// Apply an enchantment to your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateEnchantment")]
pub struct MagicUpdateEnchantment {
    #[serde(rename = "Enchantment")]
    pub enchantment: Enchantment,
}

impl MagicUpdateEnchantment {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let enchantment = Enchantment::read(reader)?;

        Ok(Self {
            enchantment,
        })
    }
}

impl crate::readers::ACDataType for MagicUpdateEnchantment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicUpdateEnchantment::read(reader)
    }
}

