use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Update multiple enchantments from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateMultipleEnchantments")]
pub struct MagicUpdateMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    pub enchantments: PackableList<Enchantment>,
}

impl crate::readers::ACDataType for MagicUpdateMultipleEnchantments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MagicUpdateMultipleEnchantments").entered();

        #[cfg(feature = "tracing")]
        let _field_span_enchantments = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Enchantments", position = pos).entered()
        };
        let enchantments = read_packable_list::<Enchantment>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_enchantments);

        Ok(Self {
            enchantments,
        })
    }
}

impl crate::writers::ACWritable for MagicUpdateMultipleEnchantments {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MagicUpdateMultipleEnchantments").entered();

        write_packable_list::<Enchantment>(writer, &self.enchantments)?;
        Ok(())
    }
}

