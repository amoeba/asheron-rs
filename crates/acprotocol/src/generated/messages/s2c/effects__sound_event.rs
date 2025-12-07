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

impl EffectsSoundEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let sound_type = Sound::try_from(read_i32(reader)?)?;
        let volume = read_f32(reader)?;

        Ok(Self {
            object_id,
            sound_type,
            volume,
        })
    }
}

impl crate::readers::ACDataType for EffectsSoundEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EffectsSoundEvent::read(reader)
    }
}

