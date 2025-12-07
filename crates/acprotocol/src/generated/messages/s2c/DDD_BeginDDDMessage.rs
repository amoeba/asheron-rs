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

// A list of dat files that need to be patched
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_BeginDDDMessage")]
pub struct DDDBeginDDDMessage {
    #[serde(rename = "DataExpected")]
    pub data_expected: u32,
    #[serde(rename = "Revisions")]
    pub revisions: PackableList<DDDRevision>,
}

impl DDDBeginDDDMessage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let data_expected = read_u32(reader)?;
        let revisions = read_packable_list::<DDDRevision>(reader)?;

        Ok(Self {
            data_expected,
            revisions,
        })
    }
}

impl crate::readers::ACDataType for DDDBeginDDDMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDBeginDDDMessage::read(reader)
    }
}

