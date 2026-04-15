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

// Display an allegiance login/logout message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceLoginNotificationEvent")]
pub struct AllegianceAllegianceLoginNotificationEvent {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
    #[serde(rename = "IsLoggedIn")]
    pub is_logged_in: bool,
}

impl crate::readers::ACDataType for AllegianceAllegianceLoginNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceAllegianceLoginNotificationEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_character_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CharacterId", position = pos).entered()
        };
        let character_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_character_id);
        #[cfg(feature = "tracing")]
        let _field_span_is_logged_in = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "IsLoggedIn", position = pos).entered()
        };
        let is_logged_in = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_is_logged_in);

        Ok(Self {
            character_id,
            is_logged_in,
        })
    }
}

impl crate::writers::ACWritable for AllegianceAllegianceLoginNotificationEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AllegianceAllegianceLoginNotificationEvent").entered();

        self.character_id.write(writer)?;
        write_bool(writer, self.is_logged_in)?;
        Ok(())
    }
}

