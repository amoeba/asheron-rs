use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Create an object somewhere in the world
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_CreateObject")]
pub struct ItemCreateObject {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
    #[serde(rename = "PhysicsDescription")]
    pub physics_description: PhysicsDesc,
    #[serde(rename = "WeenieDescription")]
    pub weenie_description: PublicWeenieDesc,
}

impl crate::readers::ACDataType for ItemCreateObject {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_description = ObjDesc::read(reader)?;
        let physics_description = PhysicsDesc::read(reader)?;
        let weenie_description = PublicWeenieDesc::read(reader)?;

        Ok(Self {
            object_id,
            object_description,
            physics_description,
            weenie_description,
        })
    }
}

