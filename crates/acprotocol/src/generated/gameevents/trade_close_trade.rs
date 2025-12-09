use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// CloseTrade: End trading
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_CloseTrade")]
pub struct TradeCloseTrade {
    #[serde(rename = "Reason")]
    pub reason: EndTradeReason,
}

impl TradeCloseTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let reason = EndTradeReason::try_from(read_u32(reader)?)?;

        Ok(Self {
            reason,
        })
    }
}

impl crate::readers::ACDataType for TradeCloseTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeCloseTrade::read(reader)
    }
}

