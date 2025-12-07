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

// Joining game response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_JoinGameResponse")]
pub struct GameJoinGameResponse {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "Team")]
    pub team: i32,
}

impl GameJoinGameResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let team = read_i32(reader)?;

        Ok(Self {
            game_id,
            team,
        })
    }
}

impl crate::readers::ACDataType for GameJoinGameResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameJoinGameResponse::read(reader)
    }
}

