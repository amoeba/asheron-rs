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

// Salvage operation results
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_SalvageOperationsResultData")]
pub struct InventorySalvageOperationsResultData {
    #[serde(rename = "Result")]
    pub result: SalvageOperationsResultData,
}

impl InventorySalvageOperationsResultData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let result = SalvageOperationsResultData::read(reader)?;

        Ok(Self {
            result,
        })
    }
}

impl crate::readers::ACDataType for InventorySalvageOperationsResultData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventorySalvageOperationsResultData::read(reader)
    }
}

