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

// Fellowship Assign a new leader
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_AssignNewLeader")]
pub struct FellowshipAssignNewLeader {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

impl FellowshipAssignNewLeader {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for FellowshipAssignNewLeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipAssignNewLeader::read(reader)
    }
}

