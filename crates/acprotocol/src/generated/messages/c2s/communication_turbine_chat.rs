use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Send or receive a message using Turbine Chat.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationTurbineChatType1 {
    #[serde(rename = "MmessageSize")]
    pub mmessage_size: u32,
    #[serde(rename = "TargetType")]
    pub target_type: i32,
    #[serde(rename = "TargetId")]
    pub target_id: i32,
    #[serde(rename = "TransportType")]
    pub transport_type: i32,
    #[serde(rename = "TransportId")]
    pub transport_id: i32,
    #[serde(rename = "Cookie")]
    pub cookie: i32,
    #[serde(rename = "PayloadSize")]
    pub payload_size: u32,
    pub blob_dispatch_type: CommunicationTurbineChatType1BlobDispatchTypeVariant,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationTurbineChatType1BlobDispatchTypeVariantType1 {
    #[serde(rename = "RoomId")]
    pub room_id: u32,
    #[serde(rename = "DisplayName")]
    pub display_name: WString,
    #[serde(rename = "Text")]
    pub text: WString,
    #[serde(rename = "ExtraDataSize")]
    pub extra_data_size: u32,
    #[serde(rename = "SpeakerId")]
    pub speaker_id: ObjectId,
    #[serde(rename = "HResult")]
    pub h_result: i32,
    #[serde(rename = "ChatType")]
    pub chat_type: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "BlobDispatchType")]
pub enum CommunicationTurbineChatType1BlobDispatchTypeVariant {
    #[serde(rename = "0x01")]
    Type1(CommunicationTurbineChatType1BlobDispatchTypeVariantType1),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationTurbineChatType3 {
    #[serde(rename = "MmessageSize")]
    pub mmessage_size: u32,
    #[serde(rename = "TargetType")]
    pub target_type: i32,
    #[serde(rename = "TargetId")]
    pub target_id: i32,
    #[serde(rename = "TransportType")]
    pub transport_type: i32,
    #[serde(rename = "TransportId")]
    pub transport_id: i32,
    #[serde(rename = "Cookie")]
    pub cookie: i32,
    #[serde(rename = "PayloadSize")]
    pub payload_size: u32,
    pub blob_dispatch_type: CommunicationTurbineChatType3BlobDispatchTypeVariant,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationTurbineChatType3BlobDispatchTypeVariantType2 {
    #[serde(rename = "ContextId")]
    pub context_id: u32,
    #[serde(rename = "ResponseId")]
    pub response_id: u32,
    #[serde(rename = "MethodId")]
    pub method_id: u32,
    #[serde(rename = "RoomId")]
    pub room_id: u32,
    #[serde(rename = "Text")]
    pub text: WString,
    #[serde(rename = "ExtraDataSize")]
    pub extra_data_size: u32,
    #[serde(rename = "SpeakerId")]
    pub speaker_id: ObjectId,
    #[serde(rename = "HResult")]
    pub h_result: i32,
    #[serde(rename = "ChatType")]
    pub chat_type: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "BlobDispatchType")]
pub enum CommunicationTurbineChatType3BlobDispatchTypeVariant {
    #[serde(rename = "0x02")]
    Type2(CommunicationTurbineChatType3BlobDispatchTypeVariantType2),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationTurbineChatType5 {
    #[serde(rename = "MmessageSize")]
    pub mmessage_size: u32,
    #[serde(rename = "TargetType")]
    pub target_type: i32,
    #[serde(rename = "TargetId")]
    pub target_id: i32,
    #[serde(rename = "TransportType")]
    pub transport_type: i32,
    #[serde(rename = "TransportId")]
    pub transport_id: i32,
    #[serde(rename = "Cookie")]
    pub cookie: i32,
    #[serde(rename = "PayloadSize")]
    pub payload_size: u32,
    pub blob_dispatch_type: CommunicationTurbineChatType5BlobDispatchTypeVariant,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationTurbineChatType5BlobDispatchTypeVariantType1 {
    #[serde(rename = "ContextId")]
    pub context_id: u32,
    #[serde(rename = "ResponseId")]
    pub response_id: u32,
    #[serde(rename = "MethodId")]
    pub method_id: u32,
    #[serde(rename = "HResult")]
    pub h_result: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "BlobDispatchType")]
pub enum CommunicationTurbineChatType5BlobDispatchTypeVariant {
    #[serde(rename = "0x01")]
    #[serde(alias = "0x02")]
    Type1(CommunicationTurbineChatType5BlobDispatchTypeVariantType1),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TurbineChat")]
#[serde(tag = "Type")]
pub enum CommunicationTurbineChat {
    Type1(CommunicationTurbineChatType1),
    Type3(CommunicationTurbineChatType3),
    Type5(CommunicationTurbineChatType5),
}

impl CommunicationTurbineChatType1 {
    pub fn read(reader: &mut dyn ACReader, mmessage_size: uint, target_type: int, target_id: int, transport_type: int, transport_id: int, cookie: int, payload_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = CommunicationTurbineChatType1BlobDispatchTypeVariant::read(reader)?;

        Ok(Self {
            mmessage_size,
            target_type,
            target_id,
            transport_type,
            transport_id,
            cookie,
            payload_size,
            blob_dispatch_type,
        })
    }
}

impl CommunicationTurbineChatType1BlobDispatchTypeVariant {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = read_u8(reader)?;

        match blob_dispatch_type {
            0x01 => {
                let room_id = read_u32(reader)?;
                let display_name = read_wstring(reader).map(WString)?;
                let text = read_wstring(reader).map(WString)?;
                let extra_data_size = read_u32(reader)?;
                let speaker_id = ObjectId::read(reader)?;
                let h_result = read_i32(reader)?;
                let chat_type = read_u32(reader)?;
                Ok(Self::Type1(CommunicationTurbineChatType1BlobDispatchTypeVariantType1 {
                    room_id,
                    display_name,
                    text,
                    extra_data_size,
                    speaker_id,
                    h_result,
                    chat_type,
                }))
            },
            _ => Err("Unknown nested switch value".into()),
        }
    }
}


impl CommunicationTurbineChatType3 {
    pub fn read(reader: &mut dyn ACReader, mmessage_size: uint, target_type: int, target_id: int, transport_type: int, transport_id: int, cookie: int, payload_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = CommunicationTurbineChatType3BlobDispatchTypeVariant::read(reader)?;

        Ok(Self {
            mmessage_size,
            target_type,
            target_id,
            transport_type,
            transport_id,
            cookie,
            payload_size,
            blob_dispatch_type,
        })
    }
}

impl CommunicationTurbineChatType3BlobDispatchTypeVariant {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = read_u8(reader)?;

        match blob_dispatch_type {
            0x02 => {
                let context_id = read_u32(reader)?;
                let response_id = read_u32(reader)?;
                let method_id = read_u32(reader)?;
                let room_id = read_u32(reader)?;
                let text = read_wstring(reader).map(WString)?;
                let extra_data_size = read_u32(reader)?;
                let speaker_id = ObjectId::read(reader)?;
                let h_result = read_i32(reader)?;
                let chat_type = read_u32(reader)?;
                Ok(Self::Type2(CommunicationTurbineChatType3BlobDispatchTypeVariantType2 {
                    context_id,
                    response_id,
                    method_id,
                    room_id,
                    text,
                    extra_data_size,
                    speaker_id,
                    h_result,
                    chat_type,
                }))
            },
            _ => Err("Unknown nested switch value".into()),
        }
    }
}


impl CommunicationTurbineChatType5 {
    pub fn read(reader: &mut dyn ACReader, mmessage_size: uint, target_type: int, target_id: int, transport_type: int, transport_id: int, cookie: int, payload_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = CommunicationTurbineChatType5BlobDispatchTypeVariant::read(reader)?;

        Ok(Self {
            mmessage_size,
            target_type,
            target_id,
            transport_type,
            transport_id,
            cookie,
            payload_size,
            blob_dispatch_type,
        })
    }
}

impl CommunicationTurbineChatType5BlobDispatchTypeVariant {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = read_u8(reader)?;

        match blob_dispatch_type {
            0x01 | 0x02 => {
                let context_id = read_u32(reader)?;
                let response_id = read_u32(reader)?;
                let method_id = read_u32(reader)?;
                let h_result = read_i32(reader)?;
                Ok(Self::Type1(CommunicationTurbineChatType5BlobDispatchTypeVariantType1 {
                    context_id,
                    response_id,
                    method_id,
                    h_result,
                }))
            },
            _ => Err("Unknown nested switch value".into()),
        }
    }
}


impl CommunicationTurbineChat {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let mmessage_size = read_u32(reader)?;
        let type_ = TurbineChatType::try_from(read_u32(reader)?)?;
        #[allow(unused_variables)]
        let blob_dispatch_type = read_u32(reader)?;
        let target_type = read_i32(reader)?;
        let target_id = read_i32(reader)?;
        let transport_type = read_i32(reader)?;
        let transport_id = read_i32(reader)?;
        let cookie = read_i32(reader)?;
        let payload_size = read_u32(reader)?;

        match type_ {
            TurbineChatType::ServerToClientMessage => {
                let variant_struct = CommunicationTurbineChatType1::read(reader, mmessage_size, target_type, target_id, transport_type, transport_id, cookie, payload_size)?;
                Ok(Self::Type1(variant_struct))
            },
            TurbineChatType::ClientToServerMessage => {
                let variant_struct = CommunicationTurbineChatType3::read(reader, mmessage_size, target_type, target_id, transport_type, transport_id, cookie, payload_size)?;
                Ok(Self::Type3(variant_struct))
            },
            TurbineChatType::AckClientToServerMessage => {
                let variant_struct = CommunicationTurbineChatType5::read(reader, mmessage_size, target_type, target_id, transport_type, transport_id, cookie, payload_size)?;
                Ok(Self::Type5(variant_struct))
            },
        }
    }
}

impl crate::readers::ACDataType for CommunicationTurbineChat {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationTurbineChat::read(reader)
    }
}

