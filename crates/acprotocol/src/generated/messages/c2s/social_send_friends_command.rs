use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Seems to be a legacy friends command, /friends old, for when Jan 2006 event changed the friends list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendFriendsCommand")]
pub struct SocialSendFriendsCommand {
    #[serde(rename = "Command")]
    pub command: u32,
    #[serde(rename = "Player")]
    pub player: String,
}

impl crate::readers::ACDataType for SocialSendFriendsCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let command = read_u32(reader)?;
        let player = read_string(reader)?;

        Ok(Self {
            command,
            player,
        })
    }
}

