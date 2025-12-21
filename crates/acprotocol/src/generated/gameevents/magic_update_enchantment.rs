use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Apply an enchantment to your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateEnchantment")]
pub struct MagicUpdateEnchantment {
    #[serde(rename = "Enchantment")]
    pub enchantment: Enchantment,
}

impl crate::readers::ACDataType for MagicUpdateEnchantment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MagicUpdateEnchantment").entered();

        #[cfg(feature = "tracing")]
        let _field_span_enchantment = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Enchantment", position = pos).entered()
        };
        let enchantment = Enchantment::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_enchantment);

        Ok(Self {
            enchantment,
        })
    }
}

