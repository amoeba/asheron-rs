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

// Display a message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TextboxString")]
pub struct CommunicationTextboxString {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

impl crate::readers::ACDataType for CommunicationTextboxString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let text = read_string(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;

        Ok(Self {
            text,
            type_,
        })
    }
}

