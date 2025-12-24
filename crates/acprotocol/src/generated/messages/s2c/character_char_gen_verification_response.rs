use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

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
    Type1(CharacterCharGenVerificationResponseType1),
}

impl CharacterCharGenVerificationResponseType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterCharGenVerificationResponseType1").entered();

        let character_id = ObjectId::read(reader)?;
        let name = read_string(reader)?;
        let seconds_until_deletion = read_u32(reader)?;
        align_dword(reader)?;

        Ok(Self {
            character_id,
            name,
            seconds_until_deletion,
        })
    }
}

impl CharacterCharGenVerificationResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterCharGenVerificationResponse").entered();

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

impl CharacterCharGenVerificationResponseType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterCharGenVerificationResponseType1").entered();

        self.character_id.write(writer)?;
        write_string(writer, &self.name)?;
        write_u32(writer, self.seconds_until_deletion)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for CharacterCharGenVerificationResponseType1 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        CharacterCharGenVerificationResponseType1::write(self, writer)
    }
}

impl CharacterCharGenVerificationResponse {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterCharGenVerificationResponse").entered();


        match self {
            Self::Type1(variant_struct) => {
                CharacterCharGenVerificationResponseType1::write(variant_struct, writer)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for CharacterCharGenVerificationResponse {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        CharacterCharGenVerificationResponse::write(self, writer)
    }
}

