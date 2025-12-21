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

// Add or update a dat file Resource.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_InterrogationMessage")]
pub struct DDDInterrogationMessage {
    #[serde(rename = "ServersRegion")]
    pub servers_region: u32,
    #[serde(rename = "NameRuleLanguage")]
    pub name_rule_language: u32,
    #[serde(rename = "ProductId")]
    pub product_id: u32,
    #[serde(rename = "SupportedLanguages")]
    pub supported_languages: PackableList<u32>,
}

impl crate::readers::ACDataType for DDDInterrogationMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "DDDInterrogationMessage").entered();

        #[cfg(feature = "tracing")]
        let _field_span_servers_region = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ServersRegion", position = pos).entered()
        };
        let servers_region = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_servers_region);
        #[cfg(feature = "tracing")]
        let _field_span_name_rule_language = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NameRuleLanguage", position = pos).entered()
        };
        let name_rule_language = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name_rule_language);
        #[cfg(feature = "tracing")]
        let _field_span_product_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ProductId", position = pos).entered()
        };
        let product_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_product_id);
        #[cfg(feature = "tracing")]
        let _field_span_supported_languages = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SupportedLanguages", position = pos).entered()
        };
        let supported_languages = read_packable_list::<u32>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_supported_languages);

        Ok(Self {
            servers_region,
            name_rule_language,
            product_id,
            supported_languages,
        })
    }
}

