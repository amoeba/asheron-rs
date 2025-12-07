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

// Start game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_StartGame")]
pub struct GameStartGame {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "Team")]
    pub team: i32,
}

impl GameStartGame {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let team = read_i32(reader)?;

        Ok(Self {
            game_id,
            team,
        })
    }
}

impl crate::readers::ACDataType for GameStartGame {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameStartGame::read(reader)
    }
}

