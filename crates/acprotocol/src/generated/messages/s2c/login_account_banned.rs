use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Account has been banned
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AccountBanned")]
pub struct LoginAccountBanned {
    #[serde(rename = "BannedUntil")]
    pub banned_until: u32,
    #[serde(rename = "Text")]
    pub text: String,
}

impl LoginAccountBanned {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let banned_until = read_u32(reader)?;
        let text = read_string(reader)?;

        Ok(Self {
            banned_until,
            text,
        })
    }
}

impl crate::readers::ACDataType for LoginAccountBanned {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginAccountBanned::read(reader)
    }
}

