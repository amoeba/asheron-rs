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

// Starts a melee attack against a target
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_TargetedMeleeAttack")]
pub struct CombatTargetedMeleeAttack {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Height")]
    pub height: AttackHeight,
    #[serde(rename = "Power")]
    pub power: f32,
}

impl crate::readers::ACDataType for CombatTargetedMeleeAttack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CombatTargetedMeleeAttack").entered();

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
        let _field_span_power = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Power", position = pos).entered()
        };
        let power = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_power);

        Ok(Self {
            object_id,
            height,
            power,
        })
    }
}

impl crate::writers::ACWritable for CombatTargetedMeleeAttack {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CombatTargetedMeleeAttack").entered();

        self.object_id.write(writer)?;
        write_u32(writer, self.height.clone() as u32)?;
        write_f32(writer, self.power)?;
        Ok(())
    }
}

