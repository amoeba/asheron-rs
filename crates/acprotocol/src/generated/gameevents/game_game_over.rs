use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// End of Chess game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_GameOver")]
pub struct GameGameOver {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "TeamWinner")]
    pub team_winner: i32,
}

impl GameGameOver {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let team_winner = read_i32(reader)?;

        Ok(Self {
            game_id,
            team_winner,
        })
    }
}

impl crate::readers::ACDataType for GameGameOver {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameGameOver::read(reader)
    }
}

