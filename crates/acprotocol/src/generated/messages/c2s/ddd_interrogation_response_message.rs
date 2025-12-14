use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// TODO
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_InterrogationResponseMessage")]
pub struct DDDInterrogationResponseMessage {
    #[serde(rename = "Language")]
    pub language: u32,
    #[serde(rename = "Files")]
    pub files: PackableList<i64>,
}

impl crate::readers::ACDataType for DDDInterrogationResponseMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let language = read_u32(reader)?;
        let files = read_packable_list::<i64>(reader)?;

        Ok(Self {
            language,
            files,
        })
    }
}

