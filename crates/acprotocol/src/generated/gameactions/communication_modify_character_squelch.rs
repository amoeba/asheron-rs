use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Changes an account squelch
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ModifyCharacterSquelch")]
pub struct CommunicationModifyCharacterSquelch {
    #[serde(rename = "Add")]
    pub add: bool,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

impl crate::readers::ACDataType for CommunicationModifyCharacterSquelch {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;
        let object_id = ObjectId::read(reader)?;
        let character_name = read_string(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;

        Ok(Self {
            add,
            object_id,
            character_name,
            type_,
        })
    }
}

