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

// Start game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_StartGame")]
pub struct GameStartGame {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "Team")]
    pub team: i32,
}

impl crate::readers::ACDataType for GameStartGame {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GameStartGame").entered();

        #[cfg(feature = "tracing")]
        let _field_span_game_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "GameId", position = pos).entered()
        };
        let game_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_game_id);
        #[cfg(feature = "tracing")]
        let _field_span_team = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Team", position = pos).entered()
        };
        let team = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_team);

        Ok(Self {
            game_id,
            team,
        })
    }
}

impl crate::writers::ACWritable for GameStartGame {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GameStartGame").entered();

        write_u32(writer, self.game_id)?;
        write_i32(writer, self.team)?;
        Ok(())
    }
}

