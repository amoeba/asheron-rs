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

// Move response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_MoveResponse")]
pub struct GameMoveResponse {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "MoveResult")]
    pub move_result: ChessMoveResult,
}

impl crate::readers::ACDataType for GameMoveResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GameMoveResponse").entered();

        #[cfg(feature = "tracing")]
        let _field_span_game_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "GameId", position = pos).entered()
        };
        let game_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_game_id);
        #[cfg(feature = "tracing")]
        let _field_span_move_result = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MoveResult", position = pos).entered()
        };
        let move_result = ChessMoveResult::try_from(read_i32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_move_result);

        Ok(Self {
            game_id,
            move_result,
        })
    }
}

