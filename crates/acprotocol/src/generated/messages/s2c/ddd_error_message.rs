use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// DDD error
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_ErrorMessage")]
pub struct DDDErrorMessage {
    #[serde(rename = "ResourceType")]
    pub resource_type: u32,
    #[serde(rename = "ResourceId")]
    pub resource_id: DataId,
    #[serde(rename = "RError")]
    pub r_error: u32,
}

impl crate::readers::ACDataType for DDDErrorMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let resource_type = read_u32(reader)?;
        let resource_id = DataId::read(reader)?;
        let r_error = read_u32(reader)?;

        Ok(Self {
            resource_type,
            resource_id,
            r_error,
        })
    }
}

