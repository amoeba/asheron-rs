use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Removes an item from the trade window, not sure if this is used still?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_RemoveFromTrade")]
pub struct TradeRemoveFromTrade {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Side")]
    pub side: TradeSide,
}

impl TradeRemoveFromTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let side = TradeSide::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            side,
        })
    }
}

impl crate::readers::ACDataType for TradeRemoveFromTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeRemoveFromTrade::read(reader)
    }
}

