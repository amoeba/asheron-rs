use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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
        let object_id = ObjectId::read(reader)?;
        let height = AttackHeight::try_from(read_u32(reader)?)?;
        let power = read_f32(reader)?;

        Ok(Self {
            object_id,
            height,
            power,
        })
    }
}

