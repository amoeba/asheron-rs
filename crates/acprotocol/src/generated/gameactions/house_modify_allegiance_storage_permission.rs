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

// Modify whether allegiance members can access storage, /house storage add_allegiance/remove_allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ModifyAllegianceStoragePermission")]
pub struct HouseModifyAllegianceStoragePermission {
    #[serde(rename = "Add")]
    pub add: bool,
}

impl crate::readers::ACDataType for HouseModifyAllegianceStoragePermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseModifyAllegianceStoragePermission").entered();

        #[cfg(feature = "tracing")]
        let _field_span_add = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Add", position = pos).entered()
        };
        let add = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_add);

        Ok(Self {
            add,
        })
    }
}

impl crate::writers::ACWritable for HouseModifyAllegianceStoragePermission {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HouseModifyAllegianceStoragePermission").entered();

        write_bool(writer, self.add)?;
        Ok(())
    }
}

