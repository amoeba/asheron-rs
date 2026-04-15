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

// Login of player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_CreatePlayer")]
pub struct LoginCreatePlayer {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
}

impl crate::readers::ACDataType for LoginCreatePlayer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "LoginCreatePlayer").entered();

        #[cfg(feature = "tracing")]
        let _field_span_character_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CharacterId", position = pos).entered()
        };
        let character_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_character_id);

        Ok(Self {
            character_id,
        })
    }
}

impl crate::writers::ACWritable for LoginCreatePlayer {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "LoginCreatePlayer").entered();

        self.character_id.write(writer)?;
        Ok(())
    }
}

