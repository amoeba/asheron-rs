use serde::{Serialize, Deserialize};
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

impl crate::readers::ACDataType for CommunicationSetSquelchDB {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let squelch_db = SquelchDB::read(reader)?;

        Ok(Self {
            squelch_db,
        })
    }
}

