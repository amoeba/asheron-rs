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

impl crate::readers::ACDataType for AdminQueryPluginResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AdminQueryPluginResponse").entered();

        #[cfg(feature = "tracing")]
        let _field_span_context = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Context", position = pos).entered()
        };
        let context = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_context);
        #[cfg(feature = "tracing")]
        let _field_span_success = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Success", position = pos).entered()
        };
        let success = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_success);
        #[cfg(feature = "tracing")]
        let _field_span_plugin_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PluginName", position = pos).entered()
        };
        let plugin_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_plugin_name);
        #[cfg(feature = "tracing")]
        let _field_span_plugin_author = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PluginAuthor", position = pos).entered()
        };
        let plugin_author = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_plugin_author);
        #[cfg(feature = "tracing")]
        let _field_span_plugin_email = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PluginEmail", position = pos).entered()
        };
        let plugin_email = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_plugin_email);
        #[cfg(feature = "tracing")]
        let _field_span_plugin_webpage = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PluginWebpage", position = pos).entered()
        };
        let plugin_webpage = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_plugin_webpage);

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

