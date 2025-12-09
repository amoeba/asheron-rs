use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Display an allegiance login/logout message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceLoginNotificationEvent")]
pub struct AllegianceAllegianceLoginNotificationEvent {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
    #[serde(rename = "IsLoggedIn")]
    pub is_logged_in: bool,
}

impl AllegianceAllegianceLoginNotificationEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;
        let is_logged_in = read_bool(reader)?;

        Ok(Self {
            character_id,
            is_logged_in,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceLoginNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceLoginNotificationEvent::read(reader)
    }
}

