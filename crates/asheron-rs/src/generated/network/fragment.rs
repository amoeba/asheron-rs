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

// A single fragment containing header and payload data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fragment {
    #[serde(rename = "Header")]
    pub header: FragmentHeader,
    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

impl crate::readers::ACDataType for Fragment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Fragment").entered();

        #[cfg(feature = "tracing")]
        let _field_span_header = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Header", position = pos).entered()
        };
        let header = FragmentHeader::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_header);
        #[cfg(feature = "tracing")]
        let _field_span_data = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Data", position = pos).entered()
        };
        let data = {
                let mut data = Vec::new();
                let _ = reader.read_to_end(&mut data);
                Ok::<_, Box<dyn std::error::Error>>(data)
            }?;
        #[cfg(feature = "tracing")]
        drop(_field_span_data);

        Ok(Self {
            header,
            data,
        })
    }
}

impl crate::writers::ACWritable for Fragment {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Fragment").entered();

        self.header.write(writer)?;
        write_vec::<u8>(writer, &self.data)?;
        Ok(())
    }
}

