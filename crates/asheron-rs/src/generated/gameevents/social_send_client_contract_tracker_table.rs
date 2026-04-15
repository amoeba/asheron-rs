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

// Sends all contract data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendClientContractTrackerTable")]
pub struct SocialSendClientContractTrackerTable {
    #[serde(rename = "ContractTracker")]
    pub contract_tracker: ContractTrackerTable,
}

impl crate::readers::ACDataType for SocialSendClientContractTrackerTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SocialSendClientContractTrackerTable").entered();

        #[cfg(feature = "tracing")]
        let _field_span_contract_tracker = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContractTracker", position = pos).entered()
        };
        let contract_tracker = ContractTrackerTable::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_contract_tracker);

        Ok(Self {
            contract_tracker,
        })
    }
}

impl crate::writers::ACWritable for SocialSendClientContractTrackerTable {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "SocialSendClientContractTrackerTable").entered();

        self.contract_tracker.write(writer)?;
        Ok(())
    }
}

