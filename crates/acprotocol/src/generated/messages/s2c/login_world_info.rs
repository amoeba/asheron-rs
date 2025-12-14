use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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

impl crate::readers::ACDataType for LoginWorldInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
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

