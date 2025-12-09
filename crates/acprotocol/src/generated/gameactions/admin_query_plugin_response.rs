use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Admin Returns plugin info
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_QueryPluginResponse")]
pub struct AdminQueryPluginResponse {
    #[serde(rename = "Context")]
    pub context: u32,
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "PluginName")]
    pub plugin_name: String,
    #[serde(rename = "PluginAuthor")]
    pub plugin_author: String,
    #[serde(rename = "PluginEmail")]
    pub plugin_email: String,
    #[serde(rename = "PluginWebpage")]
    pub plugin_webpage: String,
}

impl AdminQueryPluginResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let context = read_u32(reader)?;
        let success = read_bool(reader)?;
        let plugin_name = read_string(reader)?;
        let plugin_author = read_string(reader)?;
        let plugin_email = read_string(reader)?;
        let plugin_webpage = read_string(reader)?;

        Ok(Self {
            context,
            success,
            plugin_name,
            plugin_author,
            plugin_email,
            plugin_webpage,
        })
    }
}

impl crate::readers::ACDataType for AdminQueryPluginResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminQueryPluginResponse::read(reader)
    }
}

