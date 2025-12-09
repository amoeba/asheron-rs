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

// HandleDefenderNotificationEvent: You have been hit by a creature's melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleDefenderNotificationEvent")]
pub struct CombatHandleDefenderNotificationEvent {
    #[serde(rename = "AttackerName")]
    pub attacker_name: String,
    #[serde(rename = "Type")]
    pub type_: DamageType,
    #[serde(rename = "DamagePercent")]
    pub damage_percent: f32,
    #[serde(rename = "Damage")]
    pub damage: u32,
    #[serde(rename = "Location")]
    pub location: DamageLocation,
    #[serde(rename = "Critical")]
    pub critical: bool,
    #[serde(rename = "AttackConditions")]
    pub attack_conditions: AttackConditionsMask,
}

impl CombatHandleDefenderNotificationEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let attacker_name = read_string(reader)?;
        let type_ = Ok::<_, Box<dyn std::error::Error>>(DamageType::from_bits_retain(read_u32(reader)?))?;
        let damage_percent = read_f32(reader)?;
        let damage = read_u32(reader)?;
        let location = DamageLocation::try_from(read_u32(reader)?)?;
        let critical = read_bool(reader)?;
        let attack_conditions = Ok::<_, Box<dyn std::error::Error>>(AttackConditionsMask::from_bits_retain(read_u32(reader)?))?;

        Ok(Self {
            attacker_name,
            type_,
            damage_percent,
            damage,
            location,
            critical,
            attack_conditions,
        })
    }
}

impl crate::readers::ACDataType for CombatHandleDefenderNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleDefenderNotificationEvent::read(reader)
    }
}

