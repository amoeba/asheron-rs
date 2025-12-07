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

