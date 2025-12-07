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

// Remove a string property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveStringEvent")]
pub struct QualitiesPrivateRemoveStringEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Type")]
    pub type_: PropertyString,
}

impl QualitiesPrivateRemoveStringEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let type_ = PropertyString::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateRemoveStringEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateRemoveStringEvent::read(reader)
    }
}

