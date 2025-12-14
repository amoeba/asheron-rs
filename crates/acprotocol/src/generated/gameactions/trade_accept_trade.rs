use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Accepts a trade.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_AcceptTrade")]
pub struct TradeAcceptTrade {
    #[serde(rename = "Contents")]
    pub contents: Trade,
}

impl crate::readers::ACDataType for TradeAcceptTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = Trade::read(reader)?;

        Ok(Self {
            contents,
        })
    }
}

