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

// Buy from a vendor
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Vendor_Buy")]
pub struct VendorBuy {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ItemProfile>,
    #[serde(rename = "AlternateCurrencyId")]
    pub alternate_currency_id: u32,
}

impl crate::readers::ACDataType for VendorBuy {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "VendorBuy").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_items = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Items", position = pos).entered()
        };
        let items = read_packable_list::<ItemProfile>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_items);
        #[cfg(feature = "tracing")]
        let _field_span_alternate_currency_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AlternateCurrencyId", position = pos).entered()
        };
        let alternate_currency_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alternate_currency_id);

        Ok(Self {
            object_id,
            items,
            alternate_currency_id,
        })
    }
}

impl crate::writers::ACWritable for VendorBuy {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "VendorBuy").entered();

        self.object_id.write(writer)?;
        write_packable_list::<ItemProfile>(writer, &self.items)?;
        write_u32(writer, self.alternate_currency_id)?;
        Ok(())
    }
}

