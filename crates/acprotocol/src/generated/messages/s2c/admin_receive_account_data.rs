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

// Admin Receive Account Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_ReceiveAccountData")]
pub struct AdminReceiveAccountData {
    #[serde(rename = "Unknown")]
    pub unknown: u32,
    #[serde(rename = "AdminAccountData")]
    pub admin_account_data: PackableList<AdminAccountData>,
}

impl crate::readers::ACDataType for AdminReceiveAccountData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AdminReceiveAccountData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_unknown = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Unknown", position = pos).entered()
        };
        let unknown = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_unknown);
        #[cfg(feature = "tracing")]
        let _field_span_admin_account_data = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AdminAccountData", position = pos).entered()
        };
        let admin_account_data = read_packable_list::<AdminAccountData>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_admin_account_data);

        Ok(Self {
            unknown,
            admin_account_data,
        })
    }
}

impl crate::writers::ACWritable for AdminReceiveAccountData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AdminReceiveAccountData").entered();

        write_u32(writer, self.unknown)?;
        write_packable_list::<AdminAccountData>(writer, &self.admin_account_data)?;
        Ok(())
    }
}

