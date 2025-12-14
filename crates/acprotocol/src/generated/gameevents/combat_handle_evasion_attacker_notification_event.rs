use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// HandleEvasionAttackerNotificationEvent: Your target has evaded your melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleEvasionAttackerNotificationEvent")]
pub struct CombatHandleEvasionAttackerNotificationEvent {
    #[serde(rename = "DefenderName")]
    pub defender_name: String,
}

impl crate::readers::ACDataType for CombatHandleEvasionAttackerNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let defender_name = read_string(reader)?;

        Ok(Self {
            defender_name,
        })
    }
}

