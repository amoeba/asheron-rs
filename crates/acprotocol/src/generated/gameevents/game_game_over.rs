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

// End of Chess game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_GameOver")]
pub struct GameGameOver {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "TeamWinner")]
    pub team_winner: i32,
}

impl crate::readers::ACDataType for GameGameOver {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GameGameOver").entered();

        #[cfg(feature = "tracing")]
        let _field_span_game_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "GameId", position = pos).entered()
        };
        let game_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_game_id);
        #[cfg(feature = "tracing")]
        let _field_span_team_winner = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TeamWinner", position = pos).entered()
        };
        let team_winner = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_team_winner);

        Ok(Self {
            game_id,
            team_winner,
        })
    }
}

