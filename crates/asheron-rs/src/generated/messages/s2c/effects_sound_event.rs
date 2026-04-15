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

// Applies a sound effect.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_SoundEvent")]
pub struct EffectsSoundEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "SoundType")]
    pub sound_type: Sound,
    #[serde(rename = "Volume")]
    pub volume: f32,
}

impl crate::readers::ACDataType for EffectsSoundEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EffectsSoundEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_sound_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SoundType", position = pos).entered()
        };
        let sound_type = Sound::try_from(read_i32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sound_type);
        #[cfg(feature = "tracing")]
        let _field_span_volume = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Volume", position = pos).entered()
        };
        let volume = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_volume);

        Ok(Self {
            object_id,
            sound_type,
            volume,
        })
    }
}

impl crate::writers::ACWritable for EffectsSoundEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EffectsSoundEvent").entered();

        self.object_id.write(writer)?;
        write_i32(writer, self.sound_type.clone() as i32)?;
        write_f32(writer, self.volume)?;
        Ok(())
    }
}

