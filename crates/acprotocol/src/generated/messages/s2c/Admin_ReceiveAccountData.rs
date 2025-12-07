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

// Admin Receive Account Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_ReceiveAccountData")]
pub struct AdminReceiveAccountData {
    #[serde(rename = "Unknown")]
    pub unknown: u32,
    #[serde(rename = "AdminAccountData")]
    pub admin_account_data: PackableList<AdminAccountData>,
}

impl AdminReceiveAccountData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown = read_u32(reader)?;
        let admin_account_data = read_packable_list::<AdminAccountData>(reader)?;

        Ok(Self {
            unknown,
            admin_account_data,
        })
    }
}

impl crate::readers::ACDataType for AdminReceiveAccountData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminReceiveAccountData::read(reader)
    }
}

