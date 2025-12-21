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

// Opens barber UI
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_StartBarber")]
pub struct CharacterStartBarber {
    #[serde(rename = "BasePalette")]
    pub base_palette: DataId,
    #[serde(rename = "HeadObject")]
    pub head_object: DataId,
    #[serde(rename = "HeadTexture")]
    pub head_texture: DataId,
    #[serde(rename = "DefaultHeadTexture")]
    pub default_head_texture: DataId,
    #[serde(rename = "EyesTexture")]
    pub eyes_texture: DataId,
    #[serde(rename = "DefaultEyesTexture")]
    pub default_eyes_texture: DataId,
    #[serde(rename = "NoseTexture")]
    pub nose_texture: DataId,
    #[serde(rename = "DefaultNoseTexture")]
    pub default_nose_texture: DataId,
    #[serde(rename = "MouthTexture")]
    pub mouth_texture: DataId,
    #[serde(rename = "DefaultMouthTexture")]
    pub default_mouth_texture: DataId,
    #[serde(rename = "SkinPalette")]
    pub skin_palette: DataId,
    #[serde(rename = "HairPalette")]
    pub hair_palette: DataId,
    #[serde(rename = "EyesPalette")]
    pub eyes_palette: DataId,
    #[serde(rename = "SetupId")]
    pub setup_id: DataId,
    #[serde(rename = "Option1")]
    pub option1: i32,
    #[serde(rename = "Option2")]
    pub option2: i32,
}

impl crate::readers::ACDataType for CharacterStartBarber {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterStartBarber").entered();

        #[cfg(feature = "tracing")]
        let _field_span_base_palette = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BasePalette", position = pos).entered()
        };
        let base_palette = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_base_palette);
        #[cfg(feature = "tracing")]
        let _field_span_head_object = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HeadObject", position = pos).entered()
        };
        let head_object = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_head_object);
        #[cfg(feature = "tracing")]
        let _field_span_head_texture = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HeadTexture", position = pos).entered()
        };
        let head_texture = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_head_texture);
        #[cfg(feature = "tracing")]
        let _field_span_default_head_texture = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DefaultHeadTexture", position = pos).entered()
        };
        let default_head_texture = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_default_head_texture);
        #[cfg(feature = "tracing")]
        let _field_span_eyes_texture = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "EyesTexture", position = pos).entered()
        };
        let eyes_texture = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_eyes_texture);
        #[cfg(feature = "tracing")]
        let _field_span_default_eyes_texture = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DefaultEyesTexture", position = pos).entered()
        };
        let default_eyes_texture = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_default_eyes_texture);
        #[cfg(feature = "tracing")]
        let _field_span_nose_texture = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NoseTexture", position = pos).entered()
        };
        let nose_texture = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_nose_texture);
        #[cfg(feature = "tracing")]
        let _field_span_default_nose_texture = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DefaultNoseTexture", position = pos).entered()
        };
        let default_nose_texture = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_default_nose_texture);
        #[cfg(feature = "tracing")]
        let _field_span_mouth_texture = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MouthTexture", position = pos).entered()
        };
        let mouth_texture = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_mouth_texture);
        #[cfg(feature = "tracing")]
        let _field_span_default_mouth_texture = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DefaultMouthTexture", position = pos).entered()
        };
        let default_mouth_texture = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_default_mouth_texture);
        #[cfg(feature = "tracing")]
        let _field_span_skin_palette = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SkinPalette", position = pos).entered()
        };
        let skin_palette = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_skin_palette);
        #[cfg(feature = "tracing")]
        let _field_span_hair_palette = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HairPalette", position = pos).entered()
        };
        let hair_palette = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_hair_palette);
        #[cfg(feature = "tracing")]
        let _field_span_eyes_palette = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "EyesPalette", position = pos).entered()
        };
        let eyes_palette = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_eyes_palette);
        #[cfg(feature = "tracing")]
        let _field_span_setup_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SetupId", position = pos).entered()
        };
        let setup_id = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_setup_id);
        #[cfg(feature = "tracing")]
        let _field_span_option1 = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Option1", position = pos).entered()
        };
        let option1 = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_option1);
        #[cfg(feature = "tracing")]
        let _field_span_option2 = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Option2", position = pos).entered()
        };
        let option2 = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_option2);

        Ok(Self {
            base_palette,
            head_object,
            head_texture,
            default_head_texture,
            eyes_texture,
            default_eyes_texture,
            nose_texture,
            default_nose_texture,
            mouth_texture,
            default_mouth_texture,
            skin_palette,
            hair_palette,
            eyes_palette,
            setup_id,
            option1,
            option2,
        })
    }
}

