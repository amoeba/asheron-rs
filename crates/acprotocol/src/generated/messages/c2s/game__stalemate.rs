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

// Offer or confirm stalemate
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_Stalemate")]
pub struct GameStalemate {
    #[serde(rename = "On")]
    pub on: bool,
}

impl GameStalemate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let on = read_bool(reader)?;

        Ok(Self {
            on,
        })
    }
}

impl crate::readers::ACDataType for GameStalemate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameStalemate::read(reader)
    }
}

