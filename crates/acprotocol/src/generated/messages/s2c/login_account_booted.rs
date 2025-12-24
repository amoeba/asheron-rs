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

// Account has been booted
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AccountBooted")]
pub struct LoginAccountBooted {
    #[serde(rename = "AdditionalReasonText")]
    pub additional_reason_text: String,
    #[serde(rename = "ReasonText")]
    pub reason_text: String,
}

impl crate::readers::ACDataType for LoginAccountBooted {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "LoginAccountBooted").entered();

        #[cfg(feature = "tracing")]
        let _field_span_additional_reason_text = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AdditionalReasonText", position = pos).entered()
        };
        let additional_reason_text = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_additional_reason_text);
        #[cfg(feature = "tracing")]
        let _field_span_reason_text = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ReasonText", position = pos).entered()
        };
        let reason_text = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_reason_text);

        Ok(Self {
            additional_reason_text,
            reason_text,
        })
    }
}

impl crate::writers::ACWritable for LoginAccountBooted {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "LoginAccountBooted").entered();

        write_string(writer, &self.additional_reason_text)?;
        write_string(writer, &self.reason_text)?;
        Ok(())
    }
}

