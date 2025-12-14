use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// HandleEvasionDefenderNotificationEvent: You have evaded a creature's melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleEvasionDefenderNotificationEvent")]
pub struct CombatHandleEvasionDefenderNotificationEvent {
    #[serde(rename = "AttackerName")]
    pub attacker_name: String,
}

impl crate::readers::ACDataType for CombatHandleEvasionDefenderNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let attacker_name = read_string(reader)?;

        Ok(Self {
            attacker_name,
        })
    }
}

