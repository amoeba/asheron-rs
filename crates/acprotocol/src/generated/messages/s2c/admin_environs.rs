use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// This appears to be an admin command to change the environment (light, fog, sounds, colors)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_Environs")]
pub struct AdminEnvirons {
    #[serde(rename = "EnvrionOption")]
    pub envrion_option: EnvrionChangeType,
}

impl crate::readers::ACDataType for AdminEnvirons {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let envrion_option = EnvrionChangeType::try_from(read_u32(reader)?)?;

        Ok(Self {
            envrion_option,
        })
    }
}

