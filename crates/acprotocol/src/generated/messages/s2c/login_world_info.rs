use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "LoginWorldInfo").entered();

        #[cfg(feature = "tracing")]
        let _field_span_connections = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Connections", position = pos).entered()
        };
        let connections = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_connections);
        #[cfg(feature = "tracing")]
        let _field_span_max_connections = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxConnections", position = pos).entered()
        };
        let max_connections = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_connections);
        #[cfg(feature = "tracing")]
        let _field_span_world_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WorldName", position = pos).entered()
        };
        let world_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_world_name);

        Ok(Self {
            connections,
            max_connections,
            world_name,
        })
    }
}

