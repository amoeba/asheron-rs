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

// Admin Returns a plugin list response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_QueryPluginListResponse")]
pub struct AdminQueryPluginListResponse {
    #[serde(rename = "Context")]
    pub context: u32,
    #[serde(rename = "PluginList")]
    pub plugin_list: String,
}

impl AdminQueryPluginListResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let context = read_u32(reader)?;
        let plugin_list = read_string(reader)?;

        Ok(Self {
            context,
            plugin_list,
        })
    }
}

impl crate::readers::ACDataType for AdminQueryPluginListResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminQueryPluginListResponse::read(reader)
    }
}

