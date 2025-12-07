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

// Break allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_BreakAllegiance")]
pub struct AllegianceBreakAllegiance {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

impl AllegianceBreakAllegiance {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for AllegianceBreakAllegiance {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceBreakAllegiance::read(reader)
    }
}

