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

// Player is commiting suicide
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_Suicide")]
pub struct CharacterSuicide {}

impl CharacterSuicide {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterSuicide {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterSuicide::read(reader)
    }
}

impl CharacterSuicide {
    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl crate::writers::ACWritable for CharacterSuicide {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        CharacterSuicide::write(self, writer)
    }
}

