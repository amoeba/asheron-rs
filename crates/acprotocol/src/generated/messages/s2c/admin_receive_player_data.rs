use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Admin Receive Player Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_ReceivePlayerData")]
pub struct AdminReceivePlayerData {
    #[serde(rename = "Unknown")]
    pub unknown: i32,
    #[serde(rename = "AdminPlayerData")]
    pub admin_player_data: PackableList<AdminPlayerData>,
}

impl crate::readers::ACDataType for AdminReceivePlayerData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown = read_i32(reader)?;
        let admin_player_data = read_packable_list::<AdminPlayerData>(reader)?;

        Ok(Self {
            unknown,
            admin_player_data,
        })
    }
}

