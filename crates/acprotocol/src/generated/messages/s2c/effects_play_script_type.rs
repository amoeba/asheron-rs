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

// Applies an effect with visual and sound by providing the type to be looked up in the Physics Script Table
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayScriptType")]
pub struct EffectsPlayScriptType {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ScriptType")]
    pub script_type: i32,
    #[serde(rename = "Speed")]
    pub speed: f32,
}

impl crate::readers::ACDataType for EffectsPlayScriptType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EffectsPlayScriptType").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_script_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ScriptType", position = pos).entered()
        };
        let script_type = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_script_type);
        #[cfg(feature = "tracing")]
        let _field_span_speed = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Speed", position = pos).entered()
        };
        let speed = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_speed);

        Ok(Self {
            object_id,
            script_type,
            speed,
        })
    }
}

