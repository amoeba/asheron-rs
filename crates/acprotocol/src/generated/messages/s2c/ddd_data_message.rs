use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::common::*;
#[allow(unused_imports)]
use crate::enums::*;

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
    pub data: Vec<byte>,
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
    pub data: Vec<byte>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_DataMessage")]
#[serde(tag = "Compression")]
pub enum DDDDataMessage {
    #[serde(rename = "0x00")]
    Type0(DDDDataMessageType0),
    #[serde(rename = "0x01")]
    Type1(DDDDataMessageType1),
}

impl DDDDataMessageType0 {
    pub fn read(reader: &mut dyn ACReader, dat_file: DatFileType, resource_type: uint, resource_id: DataId, iteration: uint, version: uint, data_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
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
    pub fn read(reader: &mut dyn ACReader, dat_file: DatFileType, resource_type: uint, resource_id: DataId, iteration: uint, version: uint, data_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
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

