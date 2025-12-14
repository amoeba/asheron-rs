use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// You just died.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleVictimNotificationEventSelf")]
pub struct CombatHandleVictimNotificationEventSelf {
    #[serde(rename = "Message")]
    pub message: String,
}

impl crate::readers::ACDataType for CombatHandleVictimNotificationEventSelf {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

