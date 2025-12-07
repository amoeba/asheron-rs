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

// Character creation screen initilised.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterCharGenVerificationResponseType1 {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "SecondsUntilDeletion")]
    pub seconds_until_deletion: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharGenVerificationResponse")]
#[serde(tag = "ResponseType")]
pub enum CharacterCharGenVerificationResponse {
    #[serde(rename = "0x01")]
    Type1(CharacterCharGenVerificationResponseType1),
}

impl CharacterCharGenVerificationResponseType1 {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;
        let name = read_string(reader)?;
        let seconds_until_deletion = read_u32(reader)?;
        let __alignment_marker_align_dword = align_dword(reader)?;

        Ok(Self {
            character_id,
            name,
            seconds_until_deletion,
        })
    }
}

impl CharacterCharGenVerificationResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let response_type = CharGenResponseType::try_from(read_u32(reader)?)?;

        match response_type {
            CharGenResponseType::OK => {
                let variant_struct = CharacterCharGenVerificationResponseType1::read(reader)?;
                Ok(Self::Type1(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "response_type", response_type).into()),
        }
    }
}

impl crate::readers::ACDataType for CharacterCharGenVerificationResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterCharGenVerificationResponse::read(reader)
    }
}

