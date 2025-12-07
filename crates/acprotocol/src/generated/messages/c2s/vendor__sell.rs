use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;

// Sell to a vendor
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Vendor_Sell")]
pub struct VendorSell {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ItemProfile>,
}

impl VendorSell {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let items = read_packable_list::<ItemProfile>(reader)?;

        Ok(Self {
            object_id,
            items,
        })
    }
}

impl crate::readers::ACDataType for VendorSell {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        VendorSell::read(reader)
    }
}

