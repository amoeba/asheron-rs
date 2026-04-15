use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Quits a chess game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_Quit")]
pub struct GameQuit {}

impl GameQuit {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for GameQuit {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameQuit::read(reader)
    }
}

impl GameQuit {
    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl crate::writers::ACWritable for GameQuit {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        GameQuit::write(self, writer)
    }
}

