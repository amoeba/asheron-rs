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

// Changes the global filters, /filter -type as well as /chat and /notell
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ModifyGlobalSquelch")]
pub struct CommunicationModifyGlobalSquelch {
    #[serde(rename = "Add")]
    pub add: bool,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

impl crate::readers::ACDataType for CommunicationModifyGlobalSquelch {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationModifyGlobalSquelch").entered();

        #[cfg(feature = "tracing")]
        let _field_span_add = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Add", position = pos).entered()
        };
        let add = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_add);
        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);

        Ok(Self {
            add,
            type_,
        })
    }
}

