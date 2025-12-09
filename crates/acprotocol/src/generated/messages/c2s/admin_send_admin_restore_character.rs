use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Admin command to restore a character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_SendAdminRestoreCharacter")]
pub struct AdminSendAdminRestoreCharacter {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "RestoredCharName")]
    pub restored_char_name: String,
    #[serde(rename = "AccountToRestoreTo")]
    pub account_to_restore_to: String,
}

impl AdminSendAdminRestoreCharacter {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let restored_char_name = read_string(reader)?;
        let account_to_restore_to = read_string(reader)?;

        Ok(Self {
            object_id,
            restored_char_name,
            account_to_restore_to,
        })
    }
}

impl crate::readers::ACDataType for AdminSendAdminRestoreCharacter {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminSendAdminRestoreCharacter::read(reader)
    }
}

