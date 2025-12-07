use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;

// Move response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_MoveResponse")]
pub struct GameMoveResponse {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "MoveResult")]
    pub move_result: ChessMoveResult,
}

impl GameMoveResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let move_result = ChessMoveResult::try_from(read_i32(reader)?)?;

        Ok(Self {
            game_id,
            move_result,
        })
    }
}

impl crate::readers::ACDataType for GameMoveResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameMoveResponse::read(reader)
    }
}

