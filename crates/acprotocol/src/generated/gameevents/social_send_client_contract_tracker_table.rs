use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Sends all contract data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendClientContractTrackerTable")]
pub struct SocialSendClientContractTrackerTable {
    #[serde(rename = "ContractTracker")]
    pub contract_tracker: ContractTrackerTable,
}

impl SocialSendClientContractTrackerTable {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contract_tracker = ContractTrackerTable::read(reader)?;

        Ok(Self {
            contract_tracker,
        })
    }
}

impl crate::readers::ACDataType for SocialSendClientContractTrackerTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialSendClientContractTrackerTable::read(reader)
    }
}

