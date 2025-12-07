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

// Sent when your subsciption is about to expire
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AwaitingSubscriptionExpiration")]
pub struct LoginAwaitingSubscriptionExpiration {
    #[serde(rename = "SecondsRemaining")]
    pub seconds_remaining: u32,
}

impl LoginAwaitingSubscriptionExpiration {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let seconds_remaining = read_u32(reader)?;

        Ok(Self {
            seconds_remaining,
        })
    }
}

impl crate::readers::ACDataType for LoginAwaitingSubscriptionExpiration {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginAwaitingSubscriptionExpiration::read(reader)
    }
}

