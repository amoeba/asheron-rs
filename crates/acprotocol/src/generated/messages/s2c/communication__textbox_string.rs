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

// Display a message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TextboxString")]
pub struct CommunicationTextboxString {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

impl CommunicationTextboxString {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let text = read_string(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;

        Ok(Self {
            text,
            type_,
        })
    }
}

impl crate::readers::ACDataType for CommunicationTextboxString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationTextboxString::read(reader)
    }
}

