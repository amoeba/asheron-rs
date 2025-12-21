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

// Changes a specific players storage permission, /house storage add/remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ChangeStoragePermission")]
pub struct HouseChangeStoragePermission {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
    #[serde(rename = "HasPermission")]
    pub has_permission: bool,
}

impl crate::readers::ACDataType for HouseChangeStoragePermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseChangeStoragePermission").entered();

        #[cfg(feature = "tracing")]
        let _field_span_guest_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "GuestName", position = pos).entered()
        };
        let guest_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_guest_name);
        #[cfg(feature = "tracing")]
        let _field_span_has_permission = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HasPermission", position = pos).entered()
        };
        let has_permission = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_has_permission);

        Ok(Self {
            guest_name,
            has_permission,
        })
    }
}

