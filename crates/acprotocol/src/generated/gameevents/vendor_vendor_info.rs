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

// Open the buy/sell panel for a merchant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Vendor_VendorInfo")]
pub struct VendorVendorInfo {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Profile")]
    pub profile: VendorProfile,
    #[serde(rename = "Items")]
    pub items: PackableList<ItemProfile>,
}

impl crate::readers::ACDataType for VendorVendorInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "VendorVendorInfo").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_profile = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Profile", position = pos).entered()
        };
        let profile = VendorProfile::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_profile);
        #[cfg(feature = "tracing")]
        let _field_span_items = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Items", position = pos).entered()
        };
        let items = read_packable_list::<ItemProfile>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_items);

        Ok(Self {
            object_id,
            profile,
            items,
        })
    }
}

