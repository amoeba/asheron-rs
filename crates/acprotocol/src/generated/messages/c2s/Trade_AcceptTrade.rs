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

// Accepts a trade.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_AcceptTrade")]
pub struct TradeAcceptTrade {
    #[serde(rename = "Contents")]
    pub contents: Trade,
}

impl TradeAcceptTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = Trade::read(reader)?;

        Ok(Self {
            contents,
        })
    }
}

impl crate::readers::ACDataType for TradeAcceptTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeAcceptTrade::read(reader)
    }
}

