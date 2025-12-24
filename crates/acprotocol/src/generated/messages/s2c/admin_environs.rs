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

// This appears to be an admin command to change the environment (light, fog, sounds, colors)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_Environs")]
pub struct AdminEnvirons {
    #[serde(rename = "EnvrionOption")]
    pub envrion_option: EnvrionChangeType,
}

impl crate::readers::ACDataType for AdminEnvirons {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AdminEnvirons").entered();

        #[cfg(feature = "tracing")]
        let _field_span_envrion_option = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "EnvrionOption", position = pos).entered()
        };
        let envrion_option = EnvrionChangeType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_envrion_option);

        Ok(Self {
            envrion_option,
        })
    }
}

impl crate::writers::ACWritable for AdminEnvirons {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AdminEnvirons").entered();

        write_u32(writer, self.envrion_option.clone() as u32)?;
        Ok(())
    }
}

