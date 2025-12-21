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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SocialFriendsUpdate").entered();

        #[cfg(feature = "tracing")]
        let _field_span_friends = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Friends", position = pos).entered()
        };
        let friends = read_packable_list::<FriendData>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_friends);
        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = Ok::<_, Box<dyn std::error::Error>>(FriendsUpdateType::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);

        Ok(Self {
            friends,
            type_,
        })
    }
}

