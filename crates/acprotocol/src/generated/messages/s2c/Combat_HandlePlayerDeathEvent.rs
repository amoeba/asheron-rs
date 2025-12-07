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

impl CombatHandlePlayerDeathEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
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

impl crate::readers::ACDataType for CombatHandlePlayerDeathEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandlePlayerDeathEvent::read(reader)
    }
}

