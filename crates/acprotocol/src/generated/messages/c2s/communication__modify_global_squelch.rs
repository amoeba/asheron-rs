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

// Changes the global filters, /filter -type as well as /chat and /notell
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ModifyGlobalSquelch")]
pub struct CommunicationModifyGlobalSquelch {
    #[serde(rename = "Add")]
    pub add: bool,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

impl CommunicationModifyGlobalSquelch {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;

        Ok(Self {
            add,
            type_,
        })
    }
}

impl crate::readers::ACDataType for CommunicationModifyGlobalSquelch {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationModifyGlobalSquelch::read(reader)
    }
}

