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

// Declines a trade, when cancel is clicked?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_DeclineTrade")]
pub struct TradeDeclineTrade {}

impl TradeDeclineTrade {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for TradeDeclineTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeDeclineTrade::read(reader)
    }
}

