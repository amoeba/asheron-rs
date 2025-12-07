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

impl CombatHandleAttackerNotificationEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let defender_name = read_string(reader)?;
        let type_ = DamageType::try_from(read_u32(reader)?)?;
        let damage_percent = read_f32(reader)?;
        let damage = read_u32(reader)?;
        let critical = read_bool(reader)?;
        let attack_conditions = AttackConditionsMask::try_from(read_u32(reader)?)?;

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

impl crate::readers::ACDataType for CombatHandleAttackerNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleAttackerNotificationEvent::read(reader)
    }
}

