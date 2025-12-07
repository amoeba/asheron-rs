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

