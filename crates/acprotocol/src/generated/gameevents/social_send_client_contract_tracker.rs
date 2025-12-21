use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

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

impl crate::readers::ACDataType for SocialSendClientContractTracker {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SocialSendClientContractTracker").entered();

        #[cfg(feature = "tracing")]
        let _field_span_contract_tracker = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContractTracker", position = pos).entered()
        };
        let contract_tracker = ContractTracker::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_contract_tracker);
        #[cfg(feature = "tracing")]
        let _field_span_delete_contract = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DeleteContract", position = pos).entered()
        };
        let delete_contract = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_delete_contract);
        #[cfg(feature = "tracing")]
        let _field_span_set_as_display_contract = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SetAsDisplayContract", position = pos).entered()
        };
        let set_as_display_contract = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_set_as_display_contract);

        Ok(Self {
            contract_tracker,
            delete_contract,
            set_as_display_contract,
        })
    }
}

