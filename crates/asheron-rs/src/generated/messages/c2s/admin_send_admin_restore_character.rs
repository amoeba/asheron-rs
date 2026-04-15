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

// Admin command to restore a character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_SendAdminRestoreCharacter")]
pub struct AdminSendAdminRestoreCharacter {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "RestoredCharName")]
    pub restored_char_name: String,
    #[serde(rename = "AccountToRestoreTo")]
    pub account_to_restore_to: String,
}

impl crate::readers::ACDataType for AdminSendAdminRestoreCharacter {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AdminSendAdminRestoreCharacter").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_restored_char_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "RestoredCharName", position = pos).entered()
        };
        let restored_char_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_restored_char_name);
        #[cfg(feature = "tracing")]
        let _field_span_account_to_restore_to = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AccountToRestoreTo", position = pos).entered()
        };
        let account_to_restore_to = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_account_to_restore_to);

        Ok(Self {
            object_id,
            restored_char_name,
            account_to_restore_to,
        })
    }
}

impl crate::writers::ACWritable for AdminSendAdminRestoreCharacter {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AdminSendAdminRestoreCharacter").entered();

        self.object_id.write(writer)?;
        write_string(writer, &self.restored_char_name)?;
        write_string(writer, &self.account_to_restore_to)?;
        Ok(())
    }
}

