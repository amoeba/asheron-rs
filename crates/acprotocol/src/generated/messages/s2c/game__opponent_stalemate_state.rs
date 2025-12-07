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

// Opponent Stalemate State
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_OpponentStalemateState")]
pub struct GameOpponentStalemateState {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "Team")]
    pub team: i32,
    #[serde(rename = "On")]
    pub on: bool,
}

impl GameOpponentStalemateState {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let team = read_i32(reader)?;
        let on = read_bool(reader)?;

        Ok(Self {
            game_id,
            team,
            on,
        })
    }
}

impl crate::readers::ACDataType for GameOpponentStalemateState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameOpponentStalemateState::read(reader)
    }
}

