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

// Account has been banned
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AccountBanned")]
pub struct LoginAccountBanned {
    #[serde(rename = "BannedUntil")]
    pub banned_until: u32,
    #[serde(rename = "Text")]
    pub text: String,
}

impl crate::readers::ACDataType for LoginAccountBanned {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "LoginAccountBanned").entered();

        #[cfg(feature = "tracing")]
        let _field_span_banned_until = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BannedUntil", position = pos).entered()
        };
        let banned_until = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_banned_until);
        #[cfg(feature = "tracing")]
        let _field_span_text = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Text", position = pos).entered()
        };
        let text = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_text);

        Ok(Self {
            banned_until,
            text,
        })
    }
}

impl crate::writers::ACWritable for LoginAccountBanned {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "LoginAccountBanned").entered();

        write_u32(writer, self.banned_until)?;
        write_string(writer, &self.text)?;
        Ok(())
    }
}

