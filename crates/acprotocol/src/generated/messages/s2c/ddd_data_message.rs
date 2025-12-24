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

// Add or update a dat file Resource.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DDDDataMessageType0 {
    #[serde(rename = "DatFile")]
    pub dat_file: DatFileType,
    #[serde(rename = "ResourceType")]
    pub resource_type: u32,
    #[serde(rename = "ResourceId")]
    pub resource_id: DataId,
    #[serde(rename = "Iteration")]
    pub iteration: u32,
    #[serde(rename = "Version")]
    pub version: u32,
    #[serde(rename = "DataSize")]
    pub data_size: u32,
    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DDDDataMessageType1 {
    #[serde(rename = "DatFile")]
    pub dat_file: DatFileType,
    #[serde(rename = "ResourceType")]
    pub resource_type: u32,
    #[serde(rename = "ResourceId")]
    pub resource_id: DataId,
    #[serde(rename = "Iteration")]
    pub iteration: u32,
    #[serde(rename = "Version")]
    pub version: u32,
    #[serde(rename = "DataSize")]
    pub data_size: u32,
    #[serde(rename = "FileSize")]
    pub file_size: u32,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_DataMessage")]
#[serde(tag = "Compression")]
pub enum DDDDataMessage {
    Type0(DDDDataMessageType0),
    Type1(DDDDataMessageType1),
}

impl DDDDataMessageType0 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, dat_file: DatFileType, resource_type: uint, resource_id: DataId, iteration: uint, version: uint, data_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "DDDDataMessageType0").entered();

        let data = read_vec::<u8>(reader, data_size as usize)?;

        Ok(Self {
            dat_file,
            resource_type,
            resource_id,
            iteration,
            version,
            data_size,
            data,
        })
    }
}

impl DDDDataMessageType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, dat_file: DatFileType, resource_type: uint, resource_id: DataId, iteration: uint, version: uint, data_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "DDDDataMessageType1").entered();

        let file_size = read_u32(reader)?;
        let data = read_vec::<u8>(reader, data_size as usize)?;

        Ok(Self {
            dat_file,
            resource_type,
            resource_id,
            iteration,
            version,
            data_size,
            file_size,
            data,
        })
    }
}

impl DDDDataMessage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "DDDDataMessage").entered();

        let dat_file = DatFileType::try_from(read_i64(reader)?)?;
        let resource_type = read_u32(reader)?;
        let resource_id = DataId::read(reader)?;
        let iteration = read_u32(reader)?;
        let compression = CompressionType::try_from(read_u8(reader)?)?;
        let version = read_u32(reader)?;
        let data_size = read_u32(reader)?;

        match compression {
            CompressionType::None => {
                let variant_struct = DDDDataMessageType0::read(reader, dat_file, resource_type, resource_id, iteration, version, data_size)?;
                Ok(Self::Type0(variant_struct))
            },
            CompressionType::ZLib => {
                let variant_struct = DDDDataMessageType1::read(reader, dat_file, resource_type, resource_id, iteration, version, data_size)?;
                Ok(Self::Type1(variant_struct))
            },
        }
    }
}

impl crate::readers::ACDataType for DDDDataMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDDataMessage::read(reader)
    }
}

impl DDDDataMessageType0 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "DDDDataMessageType0").entered();

        write_vec::<u8>(writer, &self.data)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for DDDDataMessageType0 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        DDDDataMessageType0::write(self, writer)
    }
}

impl DDDDataMessageType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "DDDDataMessageType1").entered();

        write_u32(writer, self.file_size)?;
        write_vec::<u8>(writer, &self.data)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for DDDDataMessageType1 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        DDDDataMessageType1::write(self, writer)
    }
}

impl DDDDataMessage {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "DDDDataMessage").entered();


        match self {
            Self::Type0(variant_struct) => {
                DDDDataMessageType0::write(variant_struct, writer)?;
            },
            Self::Type1(variant_struct) => {
                DDDDataMessageType1::write(variant_struct, writer)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for DDDDataMessage {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        DDDDataMessage::write(self, writer)
    }
}

