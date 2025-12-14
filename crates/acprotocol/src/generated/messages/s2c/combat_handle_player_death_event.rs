use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// A Player Kill occurred nearby (also sent for suicides).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandlePlayerDeathEvent")]
pub struct CombatHandlePlayerDeathEvent {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "KilledId")]
    pub killed_id: ObjectId,
    #[serde(rename = "KillerId")]
    pub killer_id: ObjectId,
}

impl crate::readers::ACDataType for CombatHandlePlayerDeathEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let killed_id = ObjectId::read(reader)?;
        let killer_id = ObjectId::read(reader)?;

        Ok(Self {
            message,
            killed_id,
            killer_id,
        })
    }
}

