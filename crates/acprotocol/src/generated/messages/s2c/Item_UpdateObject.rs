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

// Update an existing object's data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UpdateObject")]
pub struct ItemUpdateObject {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDesc")]
    pub object_desc: ObjDesc,
    #[serde(rename = "PhysicsDesc")]
    pub physics_desc: PhysicsDesc,
    #[serde(rename = "WeenieDesc")]
    pub weenie_desc: PublicWeenieDesc,
}

impl ItemUpdateObject {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_desc = ObjDesc::read(reader)?;
        let physics_desc = PhysicsDesc::read(reader)?;
        let weenie_desc = PublicWeenieDesc::read(reader)?;

        Ok(Self {
            object_id,
            object_desc,
            physics_desc,
            weenie_desc,
        })
    }
}

impl crate::readers::ACDataType for ItemUpdateObject {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemUpdateObject::read(reader)
    }
}

