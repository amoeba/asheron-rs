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

// CloseTrade: End trading
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_CloseTrade")]
pub struct TradeCloseTrade {
    #[serde(rename = "Reason")]
    pub reason: EndTradeReason,
}

impl crate::readers::ACDataType for TradeCloseTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "TradeCloseTrade").entered();

        #[cfg(feature = "tracing")]
        let _field_span_reason = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Reason", position = pos).entered()
        };
        let reason = EndTradeReason::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_reason);

        Ok(Self {
            reason,
        })
    }
}

impl crate::writers::ACWritable for TradeCloseTrade {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "TradeCloseTrade").entered();

        write_u32(writer, self.reason.clone() as u32)?;
        Ok(())
    }
}

