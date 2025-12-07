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

// Removes a friend
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_RemoveFriend")]
pub struct SocialRemoveFriend {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

impl SocialRemoveFriend {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for SocialRemoveFriend {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialRemoveFriend::read(reader)
    }
}

