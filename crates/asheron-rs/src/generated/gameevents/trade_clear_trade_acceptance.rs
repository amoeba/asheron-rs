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

// ClearTradeAcceptance: Failure to complete a trade
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_ClearTradeAcceptance")]
pub struct TradeClearTradeAcceptance {}

impl TradeClearTradeAcceptance {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for TradeClearTradeAcceptance {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeClearTradeAcceptance::read(reader)
    }
}

impl TradeClearTradeAcceptance {
    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl crate::writers::ACWritable for TradeClearTradeAcceptance {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        TradeClearTradeAcceptance::write(self, writer)
    }
}

