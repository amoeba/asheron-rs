use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;

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

impl CharacterStartBarber {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let base_palette = DataId::read(reader)?;
        let head_object = DataId::read(reader)?;
        let head_texture = DataId::read(reader)?;
        let default_head_texture = DataId::read(reader)?;
        let eyes_texture = DataId::read(reader)?;
        let default_eyes_texture = DataId::read(reader)?;
        let nose_texture = DataId::read(reader)?;
        let default_nose_texture = DataId::read(reader)?;
        let mouth_texture = DataId::read(reader)?;
        let default_mouth_texture = DataId::read(reader)?;
        let skin_palette = DataId::read(reader)?;
        let hair_palette = DataId::read(reader)?;
        let eyes_palette = DataId::read(reader)?;
        let setup_id = DataId::read(reader)?;
        let option1 = read_i32(reader)?;
        let option2 = read_i32(reader)?;

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

impl crate::readers::ACDataType for CharacterStartBarber {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterStartBarber::read(reader)
    }
}

