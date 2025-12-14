use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Salvage operation results
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_SalvageOperationsResultData")]
pub struct InventorySalvageOperationsResultData {
    #[serde(rename = "Result")]
    pub result: SalvageOperationsResultData,
}

impl crate::readers::ACDataType for InventorySalvageOperationsResultData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let result = SalvageOperationsResultData::read(reader)?;

        Ok(Self {
            result,
        })
    }
}

