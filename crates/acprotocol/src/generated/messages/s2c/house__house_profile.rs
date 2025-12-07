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

// Buy a dwelling or pay maintenance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseProfile")]
pub struct HouseHouseProfile {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Profile")]
    pub profile: HouseProfile,
}

impl HouseHouseProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let profile = HouseProfile::read(reader)?;

        Ok(Self {
            object_id,
            profile,
        })
    }
}

impl crate::readers::ACDataType for HouseHouseProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseHouseProfile::read(reader)
    }
}

