use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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
        let base_qualities = ACBaseQualities::read(reader)?;
        let qualities = ACQualities::read(reader)?;
        let player_module = PlayerModule::read(reader)?;
        let content_profile = read_packable_list::<ContentProfile>(reader)?;
        let inventory_placement = read_packable_list::<InventoryPlacement>(reader)?;

        Ok(Self {
            base_qualities,
            qualities,
            player_module,
            content_profile,
            inventory_placement,
        })
    }
}

