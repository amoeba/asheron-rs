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

// Updates a contract data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendClientContractTracker")]
pub struct SocialSendClientContractTracker {
    #[serde(rename = "ContractTracker")]
    pub contract_tracker: ContractTracker,
    #[serde(rename = "DeleteContract")]
    pub delete_contract: bool,
    #[serde(rename = "SetAsDisplayContract")]
    pub set_as_display_contract: bool,
}

impl SocialSendClientContractTracker {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contract_tracker = ContractTracker::read(reader)?;
        let delete_contract = read_bool(reader)?;
        let set_as_display_contract = read_bool(reader)?;

        Ok(Self {
            contract_tracker,
            delete_contract,
            set_as_display_contract,
        })
    }
}

impl crate::readers::ACDataType for SocialSendClientContractTracker {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialSendClientContractTracker::read(reader)
    }
}

