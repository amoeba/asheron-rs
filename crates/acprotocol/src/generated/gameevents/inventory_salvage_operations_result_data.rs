use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Salvage operation results
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_SalvageOperationsResultData")]
pub struct InventorySalvageOperationsResultData {
    #[serde(rename = "Result")]
    pub result: SalvageOperationsResultData,
}

impl crate::readers::ACDataType for InventorySalvageOperationsResultData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "InventorySalvageOperationsResultData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_result = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Result", position = pos).entered()
        };
        let result = SalvageOperationsResultData::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_result);

        Ok(Self {
            result,
        })
    }
}

impl crate::writers::ACWritable for InventorySalvageOperationsResultData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "InventorySalvageOperationsResultData").entered();

        self.result.write(writer)?;
        Ok(())
    }
}

