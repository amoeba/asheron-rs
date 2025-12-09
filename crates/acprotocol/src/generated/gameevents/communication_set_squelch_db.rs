use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Squelch and Filter List
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_SetSquelchDB")]
pub struct CommunicationSetSquelchDB {
    #[serde(rename = "SquelchDB")]
    pub squelch_db: SquelchDB,
}

impl CommunicationSetSquelchDB {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let squelch_db = SquelchDB::read(reader)?;

        Ok(Self {
            squelch_db,
        })
    }
}

impl crate::readers::ACDataType for CommunicationSetSquelchDB {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationSetSquelchDB::read(reader)
    }
}

