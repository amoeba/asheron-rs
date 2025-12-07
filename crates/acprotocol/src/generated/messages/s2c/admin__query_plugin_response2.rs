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

// TODO: QueryPluginResponse
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_QueryPluginResponse2")]
pub struct AdminQueryPluginResponse2 {}

impl AdminQueryPluginResponse2 {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AdminQueryPluginResponse2 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminQueryPluginResponse2::read(reader)
    }
}

