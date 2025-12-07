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

// Get Inscription Response, doesn't seem to be really used by client
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_GetInscriptionResponse")]
pub struct ItemGetInscriptionResponse {
    #[serde(rename = "Inscription")]
    pub inscription: String,
    #[serde(rename = "ScribeName")]
    pub scribe_name: String,
    #[serde(rename = "ScribeAccount")]
    pub scribe_account: String,
}

impl ItemGetInscriptionResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let inscription = read_string(reader)?;
        let scribe_name = read_string(reader)?;
        let scribe_account = read_string(reader)?;

        Ok(Self {
            inscription,
            scribe_name,
            scribe_account,
        })
    }
}

impl crate::readers::ACDataType for ItemGetInscriptionResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemGetInscriptionResponse::read(reader)
    }
}

