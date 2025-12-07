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

// Makes a chess move
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_Move")]
pub struct GameMove {
    #[serde(rename = "XFrom")]
    pub x_from: i32,
    #[serde(rename = "YFrom")]
    pub y_from: i32,
    #[serde(rename = "XTo")]
    pub x_to: i32,
    #[serde(rename = "YTo")]
    pub y_to: i32,
}

impl GameMove {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let x_from = read_i32(reader)?;
        let y_from = read_i32(reader)?;
        let x_to = read_i32(reader)?;
        let y_to = read_i32(reader)?;

        Ok(Self {
            x_from,
            y_from,
            x_to,
            y_to,
        })
    }
}

impl crate::readers::ACDataType for GameMove {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameMove::read(reader)
    }
}

