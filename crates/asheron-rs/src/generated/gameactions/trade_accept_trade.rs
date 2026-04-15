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

// Accepts a trade.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_AcceptTrade")]
pub struct TradeAcceptTrade {
    #[serde(rename = "Contents")]
    pub contents: Trade,
}

impl crate::readers::ACDataType for TradeAcceptTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "TradeAcceptTrade").entered();

        #[cfg(feature = "tracing")]
        let _field_span_contents = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Contents", position = pos).entered()
        };
        let contents = Trade::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_contents);

        Ok(Self {
            contents,
        })
    }
}

impl crate::writers::ACWritable for TradeAcceptTrade {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "TradeAcceptTrade").entered();

        self.contents.write(writer)?;
        Ok(())
    }
}

