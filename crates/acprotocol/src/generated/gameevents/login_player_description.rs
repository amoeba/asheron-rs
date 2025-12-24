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

// Information describing your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_PlayerDescription")]
pub struct LoginPlayerDescription {
    #[serde(rename = "BaseQualities")]
    pub base_qualities: ACBaseQualities,
    #[serde(rename = "Qualities")]
    pub qualities: ACQualities,
    #[serde(rename = "PlayerModule")]
    pub player_module: PlayerModule,
    #[serde(rename = "ContentProfile")]
    pub content_profile: PackableList<ContentProfile>,
    #[serde(rename = "InventoryPlacement")]
    pub inventory_placement: PackableList<InventoryPlacement>,
}

impl crate::readers::ACDataType for LoginPlayerDescription {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "LoginPlayerDescription").entered();

        #[cfg(feature = "tracing")]
        let _field_span_base_qualities = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BaseQualities", position = pos).entered()
        };
        let base_qualities = ACBaseQualities::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_base_qualities);
        #[cfg(feature = "tracing")]
        let _field_span_qualities = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Qualities", position = pos).entered()
        };
        let qualities = ACQualities::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_qualities);
        #[cfg(feature = "tracing")]
        let _field_span_player_module = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PlayerModule", position = pos).entered()
        };
        let player_module = PlayerModule::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_player_module);
        #[cfg(feature = "tracing")]
        let _field_span_content_profile = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContentProfile", position = pos).entered()
        };
        let content_profile = read_packable_list::<ContentProfile>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_content_profile);
        #[cfg(feature = "tracing")]
        let _field_span_inventory_placement = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "InventoryPlacement", position = pos).entered()
        };
        let inventory_placement = read_packable_list::<InventoryPlacement>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_inventory_placement);

        Ok(Self {
            base_qualities,
            qualities,
            player_module,
            content_profile,
            inventory_placement,
        })
    }
}

impl crate::writers::ACWritable for LoginPlayerDescription {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "LoginPlayerDescription").entered();

        self.base_qualities.write(writer)?;
        self.qualities.write(writer)?;
        self.player_module.write(writer)?;
        write_packable_list::<ContentProfile>(writer, &self.content_profile)?;
        write_packable_list::<InventoryPlacement>(writer, &self.inventory_placement)?;
        Ok(())
    }
}

