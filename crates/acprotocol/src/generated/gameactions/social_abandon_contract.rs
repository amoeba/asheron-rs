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

// Abandons a contract
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_AbandonContract")]
pub struct SocialAbandonContract {
    #[serde(rename = "ContractId")]
    pub contract_id: ContractId,
}

impl crate::readers::ACDataType for SocialAbandonContract {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SocialAbandonContract").entered();

        #[cfg(feature = "tracing")]
        let _field_span_contract_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContractId", position = pos).entered()
        };
        let contract_id = ContractId::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_contract_id);

        Ok(Self {
            contract_id,
        })
    }
}

