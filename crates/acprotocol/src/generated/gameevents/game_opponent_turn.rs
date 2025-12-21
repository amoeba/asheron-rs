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

// Opponent Turn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_OpponentTurn")]
pub struct GameOpponentTurn {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "Team")]
    pub team: i32,
    #[serde(rename = "GameMove")]
    pub game_move: GameMoveData,
}

impl crate::readers::ACDataType for GameOpponentTurn {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GameOpponentTurn").entered();

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
        #[cfg(feature = "tracing")]
        let _field_span_game_move = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "GameMove", position = pos).entered()
        };
        let game_move = GameMoveData::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_game_move);

        Ok(Self {
            game_id,
            team,
            game_move,
        })
    }
}

