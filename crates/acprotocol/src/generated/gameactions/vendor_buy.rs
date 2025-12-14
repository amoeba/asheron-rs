use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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
        let object_id = ObjectId::read(reader)?;
        let items = read_packable_list::<ItemProfile>(reader)?;
        let alternate_currency_id = read_u32(reader)?;

        Ok(Self {
            object_id,
            items,
            alternate_currency_id,
        })
    }
}

