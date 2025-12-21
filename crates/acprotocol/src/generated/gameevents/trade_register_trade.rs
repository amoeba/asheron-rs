use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// RegisterTrade: Send to begin a trade and display the trade window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_RegisterTrade")]
pub struct TradeRegisterTrade {
    #[serde(rename = "InitiatorId")]
    pub initiator_id: ObjectId,
    #[serde(rename = "PartnerId")]
    pub partner_id: ObjectId,
    #[serde(rename = "Stamp")]
    pub stamp: i64,
}

impl crate::readers::ACDataType for TradeRegisterTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "TradeRegisterTrade").entered();

        #[cfg(feature = "tracing")]
        let _field_span_initiator_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "InitiatorId", position = pos).entered()
        };
        let initiator_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_initiator_id);
        #[cfg(feature = "tracing")]
        let _field_span_partner_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PartnerId", position = pos).entered()
        };
        let partner_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_partner_id);
        #[cfg(feature = "tracing")]
        let _field_span_stamp = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Stamp", position = pos).entered()
        };
        let stamp = read_i64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_stamp);

        Ok(Self {
            initiator_id,
            partner_id,
            stamp,
        })
    }
}

