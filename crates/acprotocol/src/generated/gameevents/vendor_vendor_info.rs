use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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
        let object_id = ObjectId::read(reader)?;
        let profile = VendorProfile::read(reader)?;
        let items = read_packable_list::<ItemProfile>(reader)?;

        Ok(Self {
            object_id,
            profile,
            items,
        })
    }
}

