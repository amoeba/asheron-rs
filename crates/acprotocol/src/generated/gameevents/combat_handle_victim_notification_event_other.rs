use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Message for a death, something you killed or your own death message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleVictimNotificationEventOther")]
pub struct CombatHandleVictimNotificationEventOther {
    #[serde(rename = "Message")]
    pub message: String,
}

impl CombatHandleVictimNotificationEventOther {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

impl crate::readers::ACDataType for CombatHandleVictimNotificationEventOther {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleVictimNotificationEventOther::read(reader)
    }
}

