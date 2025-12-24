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

// HandleAttackerNotificationEvent: You have hit your target with a melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleAttackerNotificationEvent")]
pub struct CombatHandleAttackerNotificationEvent {
    #[serde(rename = "DefenderName")]
    pub defender_name: String,
    #[serde(rename = "Type")]
    pub type_: DamageType,
    #[serde(rename = "DamagePercent")]
    pub damage_percent: f32,
    #[serde(rename = "Damage")]
    pub damage: u32,
    #[serde(rename = "Critical")]
    pub critical: bool,
    #[serde(rename = "AttackConditions")]
    pub attack_conditions: AttackConditionsMask,
}

impl crate::readers::ACDataType for CombatHandleAttackerNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CombatHandleAttackerNotificationEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_defender_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DefenderName", position = pos).entered()
        };
        let defender_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_defender_name);
        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = Ok::<_, Box<dyn std::error::Error>>(DamageType::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);
        #[cfg(feature = "tracing")]
        let _field_span_damage_percent = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DamagePercent", position = pos).entered()
        };
        let damage_percent = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_damage_percent);
        #[cfg(feature = "tracing")]
        let _field_span_damage = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Damage", position = pos).entered()
        };
        let damage = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_damage);
        #[cfg(feature = "tracing")]
        let _field_span_critical = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Critical", position = pos).entered()
        };
        let critical = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_critical);
        #[cfg(feature = "tracing")]
        let _field_span_attack_conditions = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AttackConditions", position = pos).entered()
        };
        let attack_conditions = Ok::<_, Box<dyn std::error::Error>>(AttackConditionsMask::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_attack_conditions);

        Ok(Self {
            defender_name,
            type_,
            damage_percent,
            damage,
            critical,
            attack_conditions,
        })
    }
}

impl crate::writers::ACWritable for CombatHandleAttackerNotificationEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CombatHandleAttackerNotificationEvent").entered();

        write_string(writer, &self.defender_name)?;
        write_u32(writer, self.type_.bits())?;
        write_f32(writer, self.damage_percent)?;
        write_u32(writer, self.damage)?;
        write_bool(writer, self.critical)?;
        write_u32(writer, self.attack_conditions.bits())?;
        Ok(())
    }
}

