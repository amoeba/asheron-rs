use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Abandons a contract
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_AbandonContract")]
pub struct SocialAbandonContract {
    #[serde(rename = "ContractId")]
    pub contract_id: ContractId,
}

impl SocialAbandonContract {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contract_id = ContractId::try_from(read_u32(reader)?)?;

        Ok(Self {
            contract_id,
        })
    }
}

impl crate::readers::ACDataType for SocialAbandonContract {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialAbandonContract::read(reader)
    }
}

