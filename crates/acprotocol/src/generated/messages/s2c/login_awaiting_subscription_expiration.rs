use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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

