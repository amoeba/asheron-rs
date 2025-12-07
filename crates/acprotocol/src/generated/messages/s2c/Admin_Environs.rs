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

// This appears to be an admin command to change the environment (light, fog, sounds, colors)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_Environs")]
pub struct AdminEnvirons {
    #[serde(rename = "EnvrionOption")]
    pub envrion_option: EnvrionChangeType,
}

impl AdminEnvirons {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let envrion_option = EnvrionChangeType::try_from(read_u32(reader)?)?;

        Ok(Self {
            envrion_option,
        })
    }
}

impl crate::readers::ACDataType for AdminEnvirons {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminEnvirons::read(reader)
    }
}

