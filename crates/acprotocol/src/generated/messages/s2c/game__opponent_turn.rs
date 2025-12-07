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

impl GameOpponentTurn {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let team = read_i32(reader)?;
        let game_move = GameMoveData::read(reader)?;

        Ok(Self {
            game_id,
            team,
            game_move,
        })
    }
}

impl crate::readers::ACDataType for GameOpponentTurn {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameOpponentTurn::read(reader)
    }
}

