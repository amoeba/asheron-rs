use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Sets the allegiance message of the day, /allegiance motd set
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetMotd")]
pub struct AllegianceSetMotd {
    #[serde(rename = "Message")]
    pub message: String,
}

impl crate::readers::ACDataType for AllegianceSetMotd {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

