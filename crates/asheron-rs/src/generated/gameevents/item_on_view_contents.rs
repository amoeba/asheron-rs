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

// Set Pack Contents
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_OnViewContents")]
pub struct ItemOnViewContents {
    #[serde(rename = "ContainerId")]
    pub container_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ContentProfile>,
}

impl crate::readers::ACDataType for ItemOnViewContents {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemOnViewContents").entered();

        #[cfg(feature = "tracing")]
        let _field_span_container_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContainerId", position = pos).entered()
        };
        let container_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_container_id);
        #[cfg(feature = "tracing")]
        let _field_span_items = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Items", position = pos).entered()
        };
        let items = read_packable_list::<ContentProfile>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_items);

        Ok(Self {
            container_id,
            items,
        })
    }
}

impl crate::writers::ACWritable for ItemOnViewContents {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ItemOnViewContents").entered();

        self.container_id.write(writer)?;
        write_packable_list::<ContentProfile>(writer, &self.items)?;
        Ok(())
    }
}

