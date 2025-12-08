use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::common::*;
#[allow(unused_imports)]
use crate::enums::*;

// Instructs the client to show the portal graphic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayerTeleport")]
pub struct EffectsPlayerTeleport {
    #[serde(rename = "ObjectTeleportSequence")]
    pub object_teleport_sequence: u16,
}

impl EffectsPlayerTeleport {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_teleport_sequence = read_u16(reader)?;
        align_dword(reader)?;

        Ok(Self {
            object_teleport_sequence,
        })
    }
}

impl crate::readers::ACDataType for EffectsPlayerTeleport {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EffectsPlayerTeleport::read(reader)
    }
}

