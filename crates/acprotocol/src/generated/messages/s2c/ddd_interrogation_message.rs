use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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

impl DDDInterrogationMessage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let servers_region = read_u32(reader)?;
        let name_rule_language = read_u32(reader)?;
        let product_id = read_u32(reader)?;
        let supported_languages = read_packable_list::<u32>(reader)?;

        Ok(Self {
            servers_region,
            name_rule_language,
            product_id,
            supported_languages,
        })
    }
}

impl crate::readers::ACDataType for DDDInterrogationMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDInterrogationMessage::read(reader)
    }
}

