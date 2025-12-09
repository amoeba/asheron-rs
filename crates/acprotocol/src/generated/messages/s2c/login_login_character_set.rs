use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// The list of characters on the current account.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_LoginCharacterSet")]
pub struct LoginLoginCharacterSet {
    #[serde(rename = "Status")]
    pub status: u32,
    #[serde(rename = "Characters")]
    pub characters: PackableList<CharacterIdentity>,
    #[serde(rename = "DeletedCharacters")]
    pub deleted_characters: PackableList<CharacterIdentity>,
    #[serde(rename = "NumAllowedCharacters")]
    pub num_allowed_characters: u32,
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "UseTurbineChat")]
    pub use_turbine_chat: bool,
    #[serde(rename = "HasThroneofDestiny")]
    pub has_throneof_destiny: bool,
}

impl LoginLoginCharacterSet {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let status = read_u32(reader)?;
        let characters = read_packable_list::<CharacterIdentity>(reader)?;
        let deleted_characters = read_packable_list::<CharacterIdentity>(reader)?;
        let num_allowed_characters = read_u32(reader)?;
        let account = read_string(reader)?;
        let use_turbine_chat = read_bool(reader)?;
        let has_throneof_destiny = read_bool(reader)?;

        Ok(Self {
            status,
            characters,
            deleted_characters,
            num_allowed_characters,
            account,
            use_turbine_chat,
            has_throneof_destiny,
        })
    }
}

impl crate::readers::ACDataType for LoginLoginCharacterSet {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginLoginCharacterSet::read(reader)
    }
}

