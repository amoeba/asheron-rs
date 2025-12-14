use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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
        let object_id = ObjectId::read(reader)?;
        let height = AttackHeight::try_from(read_u32(reader)?)?;
        let accuracy = read_f32(reader)?;

        Ok(Self {
            object_id,
            height,
            accuracy,
        })
    }
}

