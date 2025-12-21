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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AdminReceivePlayerData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_unknown = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Unknown", position = pos).entered()
        };
        let unknown = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_unknown);
        #[cfg(feature = "tracing")]
        let _field_span_admin_player_data = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AdminPlayerData", position = pos).entered()
        };
        let admin_player_data = read_packable_list::<AdminPlayerData>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_admin_player_data);

        Ok(Self {
            unknown,
            admin_player_data,
        })
    }
}

