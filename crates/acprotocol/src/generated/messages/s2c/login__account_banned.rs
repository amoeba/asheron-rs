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

