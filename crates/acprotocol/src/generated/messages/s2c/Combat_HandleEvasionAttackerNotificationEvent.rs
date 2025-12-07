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

// HandleEvasionAttackerNotificationEvent: Your target has evaded your melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleEvasionAttackerNotificationEvent")]
pub struct CombatHandleEvasionAttackerNotificationEvent {
    #[serde(rename = "DefenderName")]
    pub defender_name: String,
}

impl CombatHandleEvasionAttackerNotificationEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let defender_name = read_string(reader)?;

        Ok(Self {
            defender_name,
        })
    }
}

impl crate::readers::ACDataType for CombatHandleEvasionAttackerNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleEvasionAttackerNotificationEvent::read(reader)
    }
}

