use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Friends list update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_FriendsUpdate")]
pub struct SocialFriendsUpdate {
    #[serde(rename = "Friends")]
    pub friends: PackableList<FriendData>,
    #[serde(rename = "Type")]
    pub type_: FriendsUpdateType,
}

impl crate::readers::ACDataType for SocialFriendsUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let friends = read_packable_list::<FriendData>(reader)?;
        let type_ = Ok::<_, Box<dyn std::error::Error>>(FriendsUpdateType::from_bits_retain(read_u32(reader)?))?;

        Ok(Self {
            friends,
            type_,
        })
    }
}

