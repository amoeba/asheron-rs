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

// Asks server for a new object description
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Object_SendForceObjdesc")]
pub struct ObjectSendForceObjdesc {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

impl crate::readers::ACDataType for ObjectSendForceObjdesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ObjectSendForceObjdesc").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);

        Ok(Self {
            object_id,
        })
    }
}

impl crate::writers::ACWritable for ObjectSendForceObjdesc {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ObjectSendForceObjdesc").entered();

        self.object_id.write(writer)?;
        Ok(())
    }
}

