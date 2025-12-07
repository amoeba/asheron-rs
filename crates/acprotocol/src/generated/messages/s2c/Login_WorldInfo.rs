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

// The name of the current world.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_WorldInfo")]
pub struct LoginWorldInfo {
    #[serde(rename = "Connections")]
    pub connections: u32,
    #[serde(rename = "MaxConnections")]
    pub max_connections: u32,
    #[serde(rename = "WorldName")]
    pub world_name: String,
}

impl LoginWorldInfo {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let connections = read_u32(reader)?;
        let max_connections = read_u32(reader)?;
        let world_name = read_string(reader)?;

        Ok(Self {
            connections,
            max_connections,
            world_name,
        })
    }
}

impl crate::readers::ACDataType for LoginWorldInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginWorldInfo::read(reader)
    }
}

