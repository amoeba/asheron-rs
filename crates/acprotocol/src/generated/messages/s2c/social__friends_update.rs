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

// Friends list update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_FriendsUpdate")]
pub struct SocialFriendsUpdate {
    #[serde(rename = "Friends")]
    pub friends: PackableList<FriendData>,
    #[serde(rename = "Type")]
    pub type_: FriendsUpdateType,
}

impl SocialFriendsUpdate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let friends = read_packable_list::<FriendData>(reader)?;
        let type_ = FriendsUpdateType::try_from(read_u32(reader)?)?;

        Ok(Self {
            friends,
            type_,
        })
    }
}

impl crate::readers::ACDataType for SocialFriendsUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialFriendsUpdate::read(reader)
    }
}

