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

