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

// Update Restrictions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRestrictions")]
pub struct HouseUpdateRestrictions {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "Restrictions")]
    pub restrictions: RestrictionDB,
}

impl crate::readers::ACDataType for HouseUpdateRestrictions {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseUpdateRestrictions").entered();

        #[cfg(feature = "tracing")]
        let _field_span_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Sequence", position = pos).entered()
        };
        let sequence = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_sender_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SenderId", position = pos).entered()
        };
        let sender_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sender_id);
        #[cfg(feature = "tracing")]
        let _field_span_restrictions = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Restrictions", position = pos).entered()
        };
        let restrictions = RestrictionDB::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_restrictions);

        Ok(Self {
            sequence,
            sender_id,
            restrictions,
        })
    }
}

impl crate::writers::ACWritable for HouseUpdateRestrictions {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HouseUpdateRestrictions").entered();

        write_u8(writer, self.sequence)?;
        self.sender_id.write(writer)?;
        self.restrictions.write(writer)?;
        Ok(())
    }
}

