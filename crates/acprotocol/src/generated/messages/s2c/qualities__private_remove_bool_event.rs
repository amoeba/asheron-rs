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

// Remove a bool property from the charactert
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveBoolEvent")]
pub struct QualitiesPrivateRemoveBoolEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Type")]
    pub type_: PropertyBool,
}

impl QualitiesPrivateRemoveBoolEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let type_ = PropertyBool::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateRemoveBoolEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateRemoveBoolEvent::read(reader)
    }
}

