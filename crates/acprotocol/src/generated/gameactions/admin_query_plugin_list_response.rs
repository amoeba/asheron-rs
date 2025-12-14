use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Admin Returns a plugin list response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_QueryPluginListResponse")]
pub struct AdminQueryPluginListResponse {
    #[serde(rename = "Context")]
    pub context: u32,
    #[serde(rename = "PluginList")]
    pub plugin_list: String,
}

impl crate::readers::ACDataType for AdminQueryPluginListResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let context = read_u32(reader)?;
        let plugin_list = read_string(reader)?;

        Ok(Self {
            context,
            plugin_list,
        })
    }
}

