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

// Starts a missle attack against a target
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_TargetedMissileAttack")]
pub struct CombatTargetedMissileAttack {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Height")]
    pub height: AttackHeight,
    #[serde(rename = "Accuracy")]
    pub accuracy: f32,
}

impl crate::readers::ACDataType for CombatTargetedMissileAttack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CombatTargetedMissileAttack").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_height = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Height", position = pos).entered()
        };
        let height = AttackHeight::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_height);
        #[cfg(feature = "tracing")]
        let _field_span_accuracy = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Accuracy", position = pos).entered()
        };
        let accuracy = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_accuracy);

        Ok(Self {
            object_id,
            height,
            accuracy,
        })
    }
}

