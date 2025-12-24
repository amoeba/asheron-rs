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

// Sets a new fill complevel for a component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SetDesiredComponentLevel")]
pub struct CharacterSetDesiredComponentLevel {
    #[serde(rename = "Wcid")]
    pub wcid: u32,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

impl crate::readers::ACDataType for CharacterSetDesiredComponentLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterSetDesiredComponentLevel").entered();

        #[cfg(feature = "tracing")]
        let _field_span_wcid = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Wcid", position = pos).entered()
        };
        let wcid = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_wcid);
        #[cfg(feature = "tracing")]
        let _field_span_amount = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Amount", position = pos).entered()
        };
        let amount = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_amount);

        Ok(Self {
            wcid,
            amount,
        })
    }
}

impl crate::writers::ACWritable for CharacterSetDesiredComponentLevel {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterSetDesiredComponentLevel").entered();

        write_u32(writer, self.wcid)?;
        write_u32(writer, self.amount)?;
        Ok(())
    }
}

