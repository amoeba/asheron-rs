use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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
        let game_id = read_u32(reader)?;
        let move_result = ChessMoveResult::try_from(read_i32(reader)?)?;

        Ok(Self {
            game_id,
            move_result,
        })
    }
}

