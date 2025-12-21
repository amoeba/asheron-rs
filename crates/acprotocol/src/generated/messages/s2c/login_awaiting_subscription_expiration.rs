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

// Sent when your subsciption is about to expire
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AwaitingSubscriptionExpiration")]
pub struct LoginAwaitingSubscriptionExpiration {
    #[serde(rename = "SecondsRemaining")]
    pub seconds_remaining: u32,
}

impl crate::readers::ACDataType for LoginAwaitingSubscriptionExpiration {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "LoginAwaitingSubscriptionExpiration").entered();

        #[cfg(feature = "tracing")]
        let _field_span_seconds_remaining = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SecondsRemaining", position = pos).entered()
        };
        let seconds_remaining = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_seconds_remaining);

        Ok(Self {
            seconds_remaining,
        })
    }
}

