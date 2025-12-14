use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// TradeFailure: Failure to add a trade item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_TradeFailure")]
pub struct TradeTradeFailure {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Reason")]
    pub reason: u32,
}

impl crate::readers::ACDataType for TradeTradeFailure {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let reason = read_u32(reader)?;

        Ok(Self {
            object_id,
            reason,
        })
    }
}

