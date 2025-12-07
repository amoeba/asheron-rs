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

