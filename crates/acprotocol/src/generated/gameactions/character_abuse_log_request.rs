use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Sends an abuse report.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AbuseLogRequest")]
pub struct CharacterAbuseLogRequest {
    #[serde(rename = "Character")]
    pub character: String,
    #[serde(rename = "Status")]
    pub status: u32,
    #[serde(rename = "Complaint")]
    pub complaint: String,
}

impl CharacterAbuseLogRequest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character = read_string(reader)?;
        let status = read_u32(reader)?;
        let complaint = read_string(reader)?;

        Ok(Self {
            character,
            status,
            complaint,
        })
    }
}

impl crate::readers::ACDataType for CharacterAbuseLogRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterAbuseLogRequest::read(reader)
    }
}

