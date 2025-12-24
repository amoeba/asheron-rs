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

// Sets the player visual desc, TODO confirm this
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SetPlayerVisualDesc")]
pub struct CharacterSetPlayerVisualDesc {
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
}

impl crate::readers::ACDataType for CharacterSetPlayerVisualDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterSetPlayerVisualDesc").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_description = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectDescription", position = pos).entered()
        };
        let object_description = ObjDesc::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_description);

        Ok(Self {
            object_description,
        })
    }
}

impl crate::writers::ACWritable for CharacterSetPlayerVisualDesc {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterSetPlayerVisualDesc").entered();

        self.object_description.write(writer)?;
        Ok(())
    }
}

