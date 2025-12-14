use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// DDD request for data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_RequestDataMessage")]
pub struct DDDRequestDataMessage {
    #[serde(rename = "ResourceType")]
    pub resource_type: u32,
    #[serde(rename = "ResourceId")]
    pub resource_id: DataId,
}

impl crate::readers::ACDataType for DDDRequestDataMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let resource_type = read_u32(reader)?;
        let resource_id = DataId::read(reader)?;

        Ok(Self {
            resource_type,
            resource_id,
        })
    }
}

