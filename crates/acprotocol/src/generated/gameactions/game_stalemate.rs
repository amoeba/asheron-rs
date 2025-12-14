use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Offer or confirm stalemate
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_Stalemate")]
pub struct GameStalemate {
    #[serde(rename = "On")]
    pub on: bool,
}

impl crate::readers::ACDataType for GameStalemate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let on = read_bool(reader)?;

        Ok(Self {
            on,
        })
    }
}

