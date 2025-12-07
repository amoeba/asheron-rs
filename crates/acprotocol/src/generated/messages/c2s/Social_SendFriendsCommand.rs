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

// Seems to be a legacy friends command, /friends old, for when Jan 2006 event changed the friends list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendFriendsCommand")]
pub struct SocialSendFriendsCommand {
    #[serde(rename = "Command")]
    pub command: u32,
    #[serde(rename = "Player")]
    pub player: String,
}

impl SocialSendFriendsCommand {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let command = read_u32(reader)?;
        let player = read_string(reader)?;

        Ok(Self {
            command,
            player,
        })
    }
}

impl crate::readers::ACDataType for SocialSendFriendsCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialSendFriendsCommand::read(reader)
    }
}

