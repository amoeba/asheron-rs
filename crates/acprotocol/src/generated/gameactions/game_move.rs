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

// Makes a chess move
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_Move")]
pub struct GameMove {
    #[serde(rename = "XFrom")]
    pub x_from: i32,
    #[serde(rename = "YFrom")]
    pub y_from: i32,
    #[serde(rename = "XTo")]
    pub x_to: i32,
    #[serde(rename = "YTo")]
    pub y_to: i32,
}

impl crate::readers::ACDataType for GameMove {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GameMove").entered();

        #[cfg(feature = "tracing")]
        let _field_span_x_from = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "XFrom", position = pos).entered()
        };
        let x_from = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_x_from);
        #[cfg(feature = "tracing")]
        let _field_span_y_from = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "YFrom", position = pos).entered()
        };
        let y_from = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_y_from);
        #[cfg(feature = "tracing")]
        let _field_span_x_to = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "XTo", position = pos).entered()
        };
        let x_to = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_x_to);
        #[cfg(feature = "tracing")]
        let _field_span_y_to = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "YTo", position = pos).entered()
        };
        let y_to = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_y_to);

        Ok(Self {
            x_from,
            y_from,
            x_to,
            y_to,
        })
    }
}

