use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AdminQueryPluginListResponse").entered();

        #[cfg(feature = "tracing")]
        let _field_span_context = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Context", position = pos).entered()
        };
        let context = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_context);
        #[cfg(feature = "tracing")]
        let _field_span_plugin_list = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PluginList", position = pos).entered()
        };
        let plugin_list = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_plugin_list);

        Ok(Self {
            context,
            plugin_list,
        })
    }
}

impl crate::writers::ACWritable for AdminQueryPluginListResponse {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AdminQueryPluginListResponse").entered();

        write_u32(writer, self.context)?;
        write_string(writer, &self.plugin_list)?;
        Ok(())
    }
}

