use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// RemoveFromTrade: Item was removed from trade window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_AddToTrade")]
pub struct TradeAddToTrade {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Side")]
    pub side: TradeSide,
}

impl crate::readers::ACDataType for TradeAddToTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let side = TradeSide::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            side,
        })
    }
}

