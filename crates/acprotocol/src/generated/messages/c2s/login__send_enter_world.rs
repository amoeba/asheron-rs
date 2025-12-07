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

// The character to log in.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_SendEnterWorld")]
pub struct LoginSendEnterWorld {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
    #[serde(rename = "Account")]
    pub account: String,
}

impl LoginSendEnterWorld {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;
        let account = read_string(reader)?;

        Ok(Self {
            character_id,
            account,
        })
    }
}

impl crate::readers::ACDataType for LoginSendEnterWorld {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginSendEnterWorld::read(reader)
    }
}

