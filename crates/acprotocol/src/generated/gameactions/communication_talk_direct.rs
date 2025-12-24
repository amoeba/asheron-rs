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

// Direct message by Id
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TalkDirect")]
pub struct CommunicationTalkDirect {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
}

impl crate::readers::ACDataType for CommunicationTalkDirect {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationTalkDirect").entered();

        #[cfg(feature = "tracing")]
        let _field_span_message = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Message", position = pos).entered()
        };
        let message = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_message);
        #[cfg(feature = "tracing")]
        let _field_span_target_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TargetId", position = pos).entered()
        };
        let target_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_target_id);

        Ok(Self {
            message,
            target_id,
        })
    }
}

impl crate::writers::ACWritable for CommunicationTalkDirect {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CommunicationTalkDirect").entered();

        write_string(writer, &self.message)?;
        self.target_id.write(writer)?;
        Ok(())
    }
}

