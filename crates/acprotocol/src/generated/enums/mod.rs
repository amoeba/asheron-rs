use serde::{Serialize, Deserialize};
use num_enum::TryFromPrimitive;
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PacketHeaderFlags: u32 {
        const NONE = 0x0;
        const RETRANSMISSION = 0x1;
        const ENCRYPTED_CHECKSUM = 0x2;
        const BLOB_FRAGMENTS = 0x4;
        const SERVER_SWITCH = 0x100;
        const LOGON_SERVER_ADDR = 0x200;
        const EMPTY_HEADER1 = 0x400;
        const REFERRAL = 0x800;
        const REQUEST_RETRANSMIT = 0x1000;
        const REJECT_RETRANSMIT = 0x2000;
        const ACK_SEQUENCE = 0x4000;
        const DISCONNECT = 0x8000;
        const LOGIN_REQUEST = 0x10000;
        const WORLD_LOGIN_REQUEST = 0x20000;
        const CONNECT_REQUEST = 0x40000;
        const CONNECT_RESPONSE = 0x80000;
        const NET_ERROR = 0x100000;
        const NET_ERROR_DISCONNECT = 0x200000;
        const CICMDCOMMAND = 0x400000;
        const TIME_SYNC = 0x1000000;
        const ECHO_REQUEST = 0x2000000;
        const ECHO_RESPONSE = 0x4000000;
        const FLOW = 0x8000000;
    }
}

impl crate::readers::ACDataType for PacketHeaderFlags {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PacketHeaderFlags::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for PacketHeaderFlags {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum FragmentGroup {
    Event = 0x5,
    Private = 0x9,
    Object = 0xA,
}

impl crate::readers::ACDataType for FragmentGroup {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(FragmentGroup::try_from(value)?)
    }
}

impl crate::writers::ACWritable for FragmentGroup {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.clone() as u16)?;
        Ok(())
    }
}

impl std::fmt::Display for FragmentGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FragmentGroup::Event => "Event",
            FragmentGroup::Private => "Private",
            FragmentGroup::Object => "Object",
        };
        write!(f, "{}", s)
    }
}

/// The type of server to switch
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ServerSwitchType {
    World = 0x0,
    Logon = 0x1,
}

impl crate::readers::ACDataType for ServerSwitchType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ServerSwitchType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ServerSwitchType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ServerSwitchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ServerSwitchType::World => "World",
            ServerSwitchType::Logon => "Logon",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AuthFlags {
    None = 0x0,
    EnableCrypto = 0x1,
    AdminAccountOverride = 0x2,
    LastDefault = 0x4,
}

impl crate::readers::ACDataType for AuthFlags {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AuthFlags::try_from(value)?)
    }
}

impl crate::writers::ACWritable for AuthFlags {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for AuthFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AuthFlags::None => "None",
            AuthFlags::EnableCrypto => "EnableCrypto",
            AuthFlags::AdminAccountOverride => "AdminAccountOverride",
            AuthFlags::LastDefault => "LastDefault",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum NetAuthType {
    Undef = 0x0,
    Account = 0x1,
    AccountPassword = 0x2,
    GlsTicket = 0x40000002,
}

impl crate::readers::ACDataType for NetAuthType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(NetAuthType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for NetAuthType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for NetAuthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            NetAuthType::Undef => "Undef",
            NetAuthType::Account => "Account",
            NetAuthType::AccountPassword => "AccountPassword",
            NetAuthType::GlsTicket => "GlsTicket",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum GameMessageGroup {
    Event = 0x1,
    Control = 0x2,
    Weenie = 0x3,
    Logon = 0x4,
    Database = 0x5,
    SecureControl = 0x6,
    SecureWeenie = 0x7,
    SecureLogin = 0x8,
    UIQueue = 0x9,
    SmartBox = 0xA,
}

impl crate::readers::ACDataType for GameMessageGroup {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(GameMessageGroup::try_from(value)?)
    }
}

impl crate::writers::ACWritable for GameMessageGroup {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for GameMessageGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GameMessageGroup::Event => "Event",
            GameMessageGroup::Control => "Control",
            GameMessageGroup::Weenie => "Weenie",
            GameMessageGroup::Logon => "Logon",
            GameMessageGroup::Database => "Database",
            GameMessageGroup::SecureControl => "SecureControl",
            GameMessageGroup::SecureWeenie => "SecureWeenie",
            GameMessageGroup::SecureLogin => "SecureLogin",
            GameMessageGroup::UIQueue => "UIQueue",
            GameMessageGroup::SmartBox => "SmartBox",
        };
        write!(f, "{}", s)
    }
}

/// Client to Server message opcodes
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum C2SMessage {
    #[serde(rename = "Login_LogOffCharacter")]
    LoginLogOffCharacter = 0xF653,
    #[serde(rename = "Character_CharacterDelete")]
    CharacterCharacterDelete = 0xF655,
    #[serde(rename = "Character_SendCharGenResult")]
    CharacterSendCharGenResult = 0xF656,
    #[serde(rename = "Login_SendEnterWorld")]
    LoginSendEnterWorld = 0xF657,
    #[serde(rename = "Object_SendForceObjdesc")]
    ObjectSendForceObjdesc = 0xF6EA,
    #[serde(rename = "Login_SendEnterWorldRequest")]
    LoginSendEnterWorldRequest = 0xF7C8,
    #[serde(rename = "Admin_SendAdminGetServerVersion")]
    AdminSendAdminGetServerVersion = 0xF7CC,
    #[serde(rename = "Social_SendFriendsCommand")]
    SocialSendFriendsCommand = 0xF7CD,
    #[serde(rename = "Admin_SendAdminRestoreCharacter")]
    AdminSendAdminRestoreCharacter = 0xF7D9,
    #[serde(rename = "Communication_TurbineChat")]
    CommunicationTurbineChat = 0xF7DE,
    #[serde(rename = "DDD_RequestDataMessage")]
    DDDRequestDataMessage = 0xF7E3,
    #[serde(rename = "DDD_InterrogationResponseMessage")]
    DDDInterrogationResponseMessage = 0xF7E6,
    #[serde(rename = "DDD_OnEndDDD")]
    DDDOnEndDDD = 0xF7EA,
    #[serde(rename = "DDD_EndDDDMessage")]
    DDDEndDDDMessage = 0xF7EB,
    #[serde(rename = "Ordered_GameAction")]
    OrderedGameAction = 0xF7B1,
}

impl crate::readers::ACDataType for C2SMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(C2SMessage::try_from(value)?)
    }
}

impl crate::writers::ACWritable for C2SMessage {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for C2SMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            C2SMessage::LoginLogOffCharacter => "Login_LogOffCharacter",
            C2SMessage::CharacterCharacterDelete => "Character_CharacterDelete",
            C2SMessage::CharacterSendCharGenResult => "Character_SendCharGenResult",
            C2SMessage::LoginSendEnterWorld => "Login_SendEnterWorld",
            C2SMessage::ObjectSendForceObjdesc => "Object_SendForceObjdesc",
            C2SMessage::LoginSendEnterWorldRequest => "Login_SendEnterWorldRequest",
            C2SMessage::AdminSendAdminGetServerVersion => "Admin_SendAdminGetServerVersion",
            C2SMessage::SocialSendFriendsCommand => "Social_SendFriendsCommand",
            C2SMessage::AdminSendAdminRestoreCharacter => "Admin_SendAdminRestoreCharacter",
            C2SMessage::CommunicationTurbineChat => "Communication_TurbineChat",
            C2SMessage::DDDRequestDataMessage => "DDD_RequestDataMessage",
            C2SMessage::DDDInterrogationResponseMessage => "DDD_InterrogationResponseMessage",
            C2SMessage::DDDOnEndDDD => "DDD_OnEndDDD",
            C2SMessage::DDDEndDDDMessage => "DDD_EndDDDMessage",
            C2SMessage::OrderedGameAction => "Ordered_GameAction",
        };
        write!(f, "{}", s)
    }
}

/// Server to Client message opcodes
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum S2CMessage {
    #[serde(rename = "Item_ServerSaysRemove")]
    ItemServerSaysRemove = 0x24,
    #[serde(rename = "Character_ServerSaysAttemptFailed")]
    CharacterServerSaysAttemptFailed = 0xA0,
    #[serde(rename = "Item_UpdateStackSize")]
    ItemUpdateStackSize = 0x197,
    #[serde(rename = "Combat_HandlePlayerDeathEvent")]
    CombatHandlePlayerDeathEvent = 0x19E,
    #[serde(rename = "Qualities_PrivateRemoveIntEvent")]
    QualitiesPrivateRemoveIntEvent = 0x1D1,
    #[serde(rename = "Qualities_RemoveIntEvent")]
    QualitiesRemoveIntEvent = 0x1D2,
    #[serde(rename = "Qualities_PrivateRemoveBoolEvent")]
    QualitiesPrivateRemoveBoolEvent = 0x1D3,
    #[serde(rename = "Qualities_RemoveBoolEvent")]
    QualitiesRemoveBoolEvent = 0x1D4,
    #[serde(rename = "Qualities_PrivateRemoveFloatEvent")]
    QualitiesPrivateRemoveFloatEvent = 0x1D5,
    #[serde(rename = "Qualities_RemoveFloatEvent")]
    QualitiesRemoveFloatEvent = 0x1D6,
    #[serde(rename = "Qualities_PrivateRemoveStringEvent")]
    QualitiesPrivateRemoveStringEvent = 0x1D7,
    #[serde(rename = "Qualities_RemoveStringEvent")]
    QualitiesRemoveStringEvent = 0x1D8,
    #[serde(rename = "Qualities_PrivateRemoveDataIdEvent")]
    QualitiesPrivateRemoveDataIdEvent = 0x1D9,
    #[serde(rename = "Qualities_RemoveDataIdEvent")]
    QualitiesRemoveDataIdEvent = 0x1DA,
    #[serde(rename = "Qualities_PrivateRemoveInstanceIdEvent")]
    QualitiesPrivateRemoveInstanceIdEvent = 0x1DB,
    #[serde(rename = "Qualities_RemoveInstanceIdEvent")]
    QualitiesRemoveInstanceIdEvent = 0x1DC,
    #[serde(rename = "Qualities_PrivateRemovePositionEvent")]
    QualitiesPrivateRemovePositionEvent = 0x1DD,
    #[serde(rename = "Qualities_RemovePositionEvent")]
    QualitiesRemovePositionEvent = 0x1DE,
    #[serde(rename = "Communication_HearEmote")]
    CommunicationHearEmote = 0x1E0,
    #[serde(rename = "Communication_HearSoulEmote")]
    CommunicationHearSoulEmote = 0x1E2,
    #[serde(rename = "Qualities_PrivateRemoveInt64Event")]
    QualitiesPrivateRemoveInt64Event = 0x2B8,
    #[serde(rename = "Qualities_RemoveInt64Event")]
    QualitiesRemoveInt64Event = 0x2B9,
    #[serde(rename = "Communication_HearSpeech")]
    CommunicationHearSpeech = 0x2BB,
    #[serde(rename = "Communication_HearRangedSpeech")]
    CommunicationHearRangedSpeech = 0x2BC,
    #[serde(rename = "Qualities_PrivateUpdateInt")]
    QualitiesPrivateUpdateInt = 0x2CD,
    #[serde(rename = "Qualities_UpdateInt")]
    QualitiesUpdateInt = 0x2CE,
    #[serde(rename = "Qualities_PrivateUpdateInt64")]
    QualitiesPrivateUpdateInt64 = 0x2CF,
    #[serde(rename = "Qualities_UpdateInt64")]
    QualitiesUpdateInt64 = 0x2D0,
    #[serde(rename = "Qualities_PrivateUpdateBool")]
    QualitiesPrivateUpdateBool = 0x2D1,
    #[serde(rename = "Qualities_UpdateBool")]
    QualitiesUpdateBool = 0x2D2,
    #[serde(rename = "Qualities_PrivateUpdateFloat")]
    QualitiesPrivateUpdateFloat = 0x2D3,
    #[serde(rename = "Qualities_UpdateFloat")]
    QualitiesUpdateFloat = 0x2D4,
    #[serde(rename = "Qualities_PrivateUpdateString")]
    QualitiesPrivateUpdateString = 0x2D5,
    #[serde(rename = "Qualities_UpdateString")]
    QualitiesUpdateString = 0x2D6,
    #[serde(rename = "Qualities_PrivateUpdateDataId")]
    QualitiesPrivateUpdateDataId = 0x2D7,
    #[serde(rename = "Qualities_UpdateDataId")]
    QualitiesUpdateDataId = 0x2D8,
    #[serde(rename = "Qualities_PrivateUpdateInstanceId")]
    QualitiesPrivateUpdateInstanceId = 0x2D9,
    #[serde(rename = "Qualities_UpdateInstanceId")]
    QualitiesUpdateInstanceId = 0x2DA,
    #[serde(rename = "Qualities_PrivateUpdatePosition")]
    QualitiesPrivateUpdatePosition = 0x2DB,
    #[serde(rename = "Qualities_UpdatePosition")]
    QualitiesUpdatePosition = 0x2DC,
    #[serde(rename = "Qualities_PrivateUpdateSkill")]
    QualitiesPrivateUpdateSkill = 0x2DD,
    #[serde(rename = "Qualities_UpdateSkill")]
    QualitiesUpdateSkill = 0x2DE,
    #[serde(rename = "Qualities_PrivateUpdateSkillLevel")]
    QualitiesPrivateUpdateSkillLevel = 0x2DF,
    #[serde(rename = "Qualities_UpdateSkillLevel")]
    QualitiesUpdateSkillLevel = 0x2E0,
    #[serde(rename = "Qualities_PrivateUpdateSkillAC")]
    QualitiesPrivateUpdateSkillAC = 0x2E1,
    #[serde(rename = "Qualities_UpdateSkillAC")]
    QualitiesUpdateSkillAC = 0x2E2,
    #[serde(rename = "Qualities_PrivateUpdateAttribute")]
    QualitiesPrivateUpdateAttribute = 0x2E3,
    #[serde(rename = "Qualities_UpdateAttribute")]
    QualitiesUpdateAttribute = 0x2E4,
    #[serde(rename = "Qualities_PrivateUpdateAttributeLevel")]
    QualitiesPrivateUpdateAttributeLevel = 0x2E5,
    #[serde(rename = "Qualities_UpdateAttributeLevel")]
    QualitiesUpdateAttributeLevel = 0x2E6,
    #[serde(rename = "Qualities_PrivateUpdateAttribute2nd")]
    QualitiesPrivateUpdateAttribute2nd = 0x2E7,
    #[serde(rename = "Qualities_UpdateAttribute2nd")]
    QualitiesUpdateAttribute2nd = 0x2E8,
    #[serde(rename = "Qualities_PrivateUpdateAttribute2ndLevel")]
    QualitiesPrivateUpdateAttribute2ndLevel = 0x2E9,
    #[serde(rename = "Qualities_UpdateAttribute2ndLevel")]
    QualitiesUpdateAttribute2ndLevel = 0x2EA,
    #[serde(rename = "Admin_Environs")]
    AdminEnvirons = 0xEA60,
    #[serde(rename = "Movement_PositionAndMovementEvent")]
    MovementPositionAndMovementEvent = 0xF619,
    #[serde(rename = "Item_ObjDescEvent")]
    ItemObjDescEvent = 0xF625,
    #[serde(rename = "Character_SetPlayerVisualDesc")]
    CharacterSetPlayerVisualDesc = 0xF630,
    #[serde(rename = "Character_CharGenVerificationResponse")]
    CharacterCharGenVerificationResponse = 0xF643,
    #[serde(rename = "Login_AwaitingSubscriptionExpiration")]
    LoginAwaitingSubscriptionExpiration = 0xF651,
    #[serde(rename = "Login_LogOffCharacter")]
    LoginLogOffCharacter = 0xF653,
    #[serde(rename = "Character_CharacterDelete")]
    CharacterCharacterDelete = 0xF655,
    #[serde(rename = "Login_LoginCharacterSet")]
    LoginLoginCharacterSet = 0xF658,
    #[serde(rename = "Character_CharacterError")]
    CharacterCharacterError = 0xF659,
    #[serde(rename = "Item_CreateObject")]
    ItemCreateObject = 0xF745,
    #[serde(rename = "Login_CreatePlayer")]
    LoginCreatePlayer = 0xF746,
    #[serde(rename = "Item_DeleteObject")]
    ItemDeleteObject = 0xF747,
    #[serde(rename = "Movement_PositionEvent")]
    MovementPositionEvent = 0xF748,
    #[serde(rename = "Item_ParentEvent")]
    ItemParentEvent = 0xF749,
    #[serde(rename = "Inventory_PickupEvent")]
    InventoryPickupEvent = 0xF74A,
    #[serde(rename = "Item_SetState")]
    ItemSetState = 0xF74B,
    #[serde(rename = "Movement_SetObjectMovement")]
    MovementSetObjectMovement = 0xF74C,
    #[serde(rename = "Movement_VectorUpdate")]
    MovementVectorUpdate = 0xF74E,
    #[serde(rename = "Effects_SoundEvent")]
    EffectsSoundEvent = 0xF750,
    #[serde(rename = "Effects_PlayerTeleport")]
    EffectsPlayerTeleport = 0xF751,
    #[serde(rename = "Effects_PlayScriptId")]
    EffectsPlayScriptId = 0xF754,
    #[serde(rename = "Effects_PlayScriptType")]
    EffectsPlayScriptType = 0xF755,
    #[serde(rename = "Login_AccountBanned")]
    LoginAccountBanned = 0xF7C1,
    #[serde(rename = "Admin_ReceiveAccountData")]
    AdminReceiveAccountData = 0xF7CA,
    #[serde(rename = "Admin_ReceivePlayerData")]
    AdminReceivePlayerData = 0xF7CB,
    #[serde(rename = "Item_UpdateObject")]
    ItemUpdateObject = 0xF7DB,
    #[serde(rename = "Login_AccountBooted")]
    LoginAccountBooted = 0xF7DC,
    #[serde(rename = "Communication_TurbineChat")]
    CommunicationTurbineChat = 0xF7DE,
    #[serde(rename = "Login_EnterGame_ServerReady")]
    LoginEnterGameServerReady = 0xF7DF,
    #[serde(rename = "Communication_TextboxString")]
    CommunicationTextboxString = 0xF7E0,
    #[serde(rename = "Login_WorldInfo")]
    LoginWorldInfo = 0xF7E1,
    #[serde(rename = "DDD_DataMessage")]
    DDDDataMessage = 0xF7E2,
    #[serde(rename = "DDD_ErrorMessage")]
    DDDErrorMessage = 0xF7E4,
    #[serde(rename = "DDD_InterrogationMessage")]
    DDDInterrogationMessage = 0xF7E5,
    #[serde(rename = "DDD_BeginDDDMessage")]
    DDDBeginDDDMessage = 0xF7E7,
    #[serde(rename = "DDD_OnEndDDD")]
    DDDOnEndDDD = 0xF7EA,
    #[serde(rename = "Ordered_GameEvent")]
    OrderedGameEvent = 0xF7B0,
}

impl crate::readers::ACDataType for S2CMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(S2CMessage::try_from(value)?)
    }
}

impl crate::writers::ACWritable for S2CMessage {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for S2CMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            S2CMessage::ItemServerSaysRemove => "Item_ServerSaysRemove",
            S2CMessage::CharacterServerSaysAttemptFailed => "Character_ServerSaysAttemptFailed",
            S2CMessage::ItemUpdateStackSize => "Item_UpdateStackSize",
            S2CMessage::CombatHandlePlayerDeathEvent => "Combat_HandlePlayerDeathEvent",
            S2CMessage::QualitiesPrivateRemoveIntEvent => "Qualities_PrivateRemoveIntEvent",
            S2CMessage::QualitiesRemoveIntEvent => "Qualities_RemoveIntEvent",
            S2CMessage::QualitiesPrivateRemoveBoolEvent => "Qualities_PrivateRemoveBoolEvent",
            S2CMessage::QualitiesRemoveBoolEvent => "Qualities_RemoveBoolEvent",
            S2CMessage::QualitiesPrivateRemoveFloatEvent => "Qualities_PrivateRemoveFloatEvent",
            S2CMessage::QualitiesRemoveFloatEvent => "Qualities_RemoveFloatEvent",
            S2CMessage::QualitiesPrivateRemoveStringEvent => "Qualities_PrivateRemoveStringEvent",
            S2CMessage::QualitiesRemoveStringEvent => "Qualities_RemoveStringEvent",
            S2CMessage::QualitiesPrivateRemoveDataIdEvent => "Qualities_PrivateRemoveDataIdEvent",
            S2CMessage::QualitiesRemoveDataIdEvent => "Qualities_RemoveDataIdEvent",
            S2CMessage::QualitiesPrivateRemoveInstanceIdEvent => "Qualities_PrivateRemoveInstanceIdEvent",
            S2CMessage::QualitiesRemoveInstanceIdEvent => "Qualities_RemoveInstanceIdEvent",
            S2CMessage::QualitiesPrivateRemovePositionEvent => "Qualities_PrivateRemovePositionEvent",
            S2CMessage::QualitiesRemovePositionEvent => "Qualities_RemovePositionEvent",
            S2CMessage::CommunicationHearEmote => "Communication_HearEmote",
            S2CMessage::CommunicationHearSoulEmote => "Communication_HearSoulEmote",
            S2CMessage::QualitiesPrivateRemoveInt64Event => "Qualities_PrivateRemoveInt64Event",
            S2CMessage::QualitiesRemoveInt64Event => "Qualities_RemoveInt64Event",
            S2CMessage::CommunicationHearSpeech => "Communication_HearSpeech",
            S2CMessage::CommunicationHearRangedSpeech => "Communication_HearRangedSpeech",
            S2CMessage::QualitiesPrivateUpdateInt => "Qualities_PrivateUpdateInt",
            S2CMessage::QualitiesUpdateInt => "Qualities_UpdateInt",
            S2CMessage::QualitiesPrivateUpdateInt64 => "Qualities_PrivateUpdateInt64",
            S2CMessage::QualitiesUpdateInt64 => "Qualities_UpdateInt64",
            S2CMessage::QualitiesPrivateUpdateBool => "Qualities_PrivateUpdateBool",
            S2CMessage::QualitiesUpdateBool => "Qualities_UpdateBool",
            S2CMessage::QualitiesPrivateUpdateFloat => "Qualities_PrivateUpdateFloat",
            S2CMessage::QualitiesUpdateFloat => "Qualities_UpdateFloat",
            S2CMessage::QualitiesPrivateUpdateString => "Qualities_PrivateUpdateString",
            S2CMessage::QualitiesUpdateString => "Qualities_UpdateString",
            S2CMessage::QualitiesPrivateUpdateDataId => "Qualities_PrivateUpdateDataId",
            S2CMessage::QualitiesUpdateDataId => "Qualities_UpdateDataId",
            S2CMessage::QualitiesPrivateUpdateInstanceId => "Qualities_PrivateUpdateInstanceId",
            S2CMessage::QualitiesUpdateInstanceId => "Qualities_UpdateInstanceId",
            S2CMessage::QualitiesPrivateUpdatePosition => "Qualities_PrivateUpdatePosition",
            S2CMessage::QualitiesUpdatePosition => "Qualities_UpdatePosition",
            S2CMessage::QualitiesPrivateUpdateSkill => "Qualities_PrivateUpdateSkill",
            S2CMessage::QualitiesUpdateSkill => "Qualities_UpdateSkill",
            S2CMessage::QualitiesPrivateUpdateSkillLevel => "Qualities_PrivateUpdateSkillLevel",
            S2CMessage::QualitiesUpdateSkillLevel => "Qualities_UpdateSkillLevel",
            S2CMessage::QualitiesPrivateUpdateSkillAC => "Qualities_PrivateUpdateSkillAC",
            S2CMessage::QualitiesUpdateSkillAC => "Qualities_UpdateSkillAC",
            S2CMessage::QualitiesPrivateUpdateAttribute => "Qualities_PrivateUpdateAttribute",
            S2CMessage::QualitiesUpdateAttribute => "Qualities_UpdateAttribute",
            S2CMessage::QualitiesPrivateUpdateAttributeLevel => "Qualities_PrivateUpdateAttributeLevel",
            S2CMessage::QualitiesUpdateAttributeLevel => "Qualities_UpdateAttributeLevel",
            S2CMessage::QualitiesPrivateUpdateAttribute2nd => "Qualities_PrivateUpdateAttribute2nd",
            S2CMessage::QualitiesUpdateAttribute2nd => "Qualities_UpdateAttribute2nd",
            S2CMessage::QualitiesPrivateUpdateAttribute2ndLevel => "Qualities_PrivateUpdateAttribute2ndLevel",
            S2CMessage::QualitiesUpdateAttribute2ndLevel => "Qualities_UpdateAttribute2ndLevel",
            S2CMessage::AdminEnvirons => "Admin_Environs",
            S2CMessage::MovementPositionAndMovementEvent => "Movement_PositionAndMovementEvent",
            S2CMessage::ItemObjDescEvent => "Item_ObjDescEvent",
            S2CMessage::CharacterSetPlayerVisualDesc => "Character_SetPlayerVisualDesc",
            S2CMessage::CharacterCharGenVerificationResponse => "Character_CharGenVerificationResponse",
            S2CMessage::LoginAwaitingSubscriptionExpiration => "Login_AwaitingSubscriptionExpiration",
            S2CMessage::LoginLogOffCharacter => "Login_LogOffCharacter",
            S2CMessage::CharacterCharacterDelete => "Character_CharacterDelete",
            S2CMessage::LoginLoginCharacterSet => "Login_LoginCharacterSet",
            S2CMessage::CharacterCharacterError => "Character_CharacterError",
            S2CMessage::ItemCreateObject => "Item_CreateObject",
            S2CMessage::LoginCreatePlayer => "Login_CreatePlayer",
            S2CMessage::ItemDeleteObject => "Item_DeleteObject",
            S2CMessage::MovementPositionEvent => "Movement_PositionEvent",
            S2CMessage::ItemParentEvent => "Item_ParentEvent",
            S2CMessage::InventoryPickupEvent => "Inventory_PickupEvent",
            S2CMessage::ItemSetState => "Item_SetState",
            S2CMessage::MovementSetObjectMovement => "Movement_SetObjectMovement",
            S2CMessage::MovementVectorUpdate => "Movement_VectorUpdate",
            S2CMessage::EffectsSoundEvent => "Effects_SoundEvent",
            S2CMessage::EffectsPlayerTeleport => "Effects_PlayerTeleport",
            S2CMessage::EffectsPlayScriptId => "Effects_PlayScriptId",
            S2CMessage::EffectsPlayScriptType => "Effects_PlayScriptType",
            S2CMessage::LoginAccountBanned => "Login_AccountBanned",
            S2CMessage::AdminReceiveAccountData => "Admin_ReceiveAccountData",
            S2CMessage::AdminReceivePlayerData => "Admin_ReceivePlayerData",
            S2CMessage::ItemUpdateObject => "Item_UpdateObject",
            S2CMessage::LoginAccountBooted => "Login_AccountBooted",
            S2CMessage::CommunicationTurbineChat => "Communication_TurbineChat",
            S2CMessage::LoginEnterGameServerReady => "Login_EnterGame_ServerReady",
            S2CMessage::CommunicationTextboxString => "Communication_TextboxString",
            S2CMessage::LoginWorldInfo => "Login_WorldInfo",
            S2CMessage::DDDDataMessage => "DDD_DataMessage",
            S2CMessage::DDDErrorMessage => "DDD_ErrorMessage",
            S2CMessage::DDDInterrogationMessage => "DDD_InterrogationMessage",
            S2CMessage::DDDBeginDDDMessage => "DDD_BeginDDDMessage",
            S2CMessage::DDDOnEndDDD => "DDD_OnEndDDD",
            S2CMessage::OrderedGameEvent => "Ordered_GameEvent",
        };
        write!(f, "{}", s)
    }
}

/// Ordered (0xF7B0) Server to Client opcodes
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum GameEvent {
    #[serde(rename = "Allegiance_AllegianceUpdateAborted")]
    AllegianceAllegianceUpdateAborted = 0x3,
    #[serde(rename = "Communication_PopUpString")]
    CommunicationPopUpString = 0x4,
    #[serde(rename = "Login_PlayerDescription")]
    LoginPlayerDescription = 0x13,
    #[serde(rename = "Allegiance_AllegianceUpdate")]
    AllegianceAllegianceUpdate = 0x20,
    #[serde(rename = "Social_FriendsUpdate")]
    SocialFriendsUpdate = 0x21,
    #[serde(rename = "Item_ServerSaysContainId")]
    ItemServerSaysContainId = 0x22,
    #[serde(rename = "Item_WearItem")]
    ItemWearItem = 0x23,
    #[serde(rename = "Social_CharacterTitleTable")]
    SocialCharacterTitleTable = 0x29,
    #[serde(rename = "Social_AddOrSetCharacterTitle")]
    SocialAddOrSetCharacterTitle = 0x2B,
    #[serde(rename = "Item_StopViewingObjectContents")]
    ItemStopViewingObjectContents = 0x52,
    #[serde(rename = "Vendor_VendorInfo")]
    VendorVendorInfo = 0x62,
    #[serde(rename = "Character_StartBarber")]
    CharacterStartBarber = 0x75,
    #[serde(rename = "Fellowship_Quit")]
    FellowshipQuit = 0xA3,
    #[serde(rename = "Fellowship_Dismiss")]
    FellowshipDismiss = 0xA4,
    #[serde(rename = "Writing_BookOpen")]
    WritingBookOpen = 0xB4,
    #[serde(rename = "Writing_BookAddPageResponse")]
    WritingBookAddPageResponse = 0xB6,
    #[serde(rename = "Writing_BookDeletePageResponse")]
    WritingBookDeletePageResponse = 0xB7,
    #[serde(rename = "Writing_BookPageDataResponse")]
    WritingBookPageDataResponse = 0xB8,
    #[serde(rename = "Item_GetInscriptionResponse")]
    ItemGetInscriptionResponse = 0xC3,
    #[serde(rename = "Item_SetAppraiseInfo")]
    ItemSetAppraiseInfo = 0xC9,
    #[serde(rename = "Communication_ChannelBroadcast")]
    CommunicationChannelBroadcast = 0x147,
    #[serde(rename = "Communication_ChannelList")]
    CommunicationChannelList = 0x148,
    #[serde(rename = "Communication_ChannelIndex")]
    CommunicationChannelIndex = 0x149,
    #[serde(rename = "Item_OnViewContents")]
    ItemOnViewContents = 0x196,
    #[serde(rename = "Item_ServerSaysMoveItem")]
    ItemServerSaysMoveItem = 0x19A,
    #[serde(rename = "Combat_HandleAttackDoneEvent")]
    CombatHandleAttackDoneEvent = 0x1A7,
    #[serde(rename = "Magic_RemoveSpell")]
    MagicRemoveSpell = 0x1A8,
    #[serde(rename = "Combat_HandleVictimNotificationEventSelf")]
    CombatHandleVictimNotificationEventSelf = 0x1AC,
    #[serde(rename = "Combat_HandleVictimNotificationEventOther")]
    CombatHandleVictimNotificationEventOther = 0x1AD,
    #[serde(rename = "Combat_HandleAttackerNotificationEvent")]
    CombatHandleAttackerNotificationEvent = 0x1B1,
    #[serde(rename = "Combat_HandleDefenderNotificationEvent")]
    CombatHandleDefenderNotificationEvent = 0x1B2,
    #[serde(rename = "Combat_HandleEvasionAttackerNotificationEvent")]
    CombatHandleEvasionAttackerNotificationEvent = 0x1B3,
    #[serde(rename = "Combat_HandleEvasionDefenderNotificationEvent")]
    CombatHandleEvasionDefenderNotificationEvent = 0x1B4,
    #[serde(rename = "Combat_HandleCommenceAttackEvent")]
    CombatHandleCommenceAttackEvent = 0x1B8,
    #[serde(rename = "Combat_QueryHealthResponse")]
    CombatQueryHealthResponse = 0x1C0,
    #[serde(rename = "Character_QueryAgeResponse")]
    CharacterQueryAgeResponse = 0x1C3,
    #[serde(rename = "Item_UseDone")]
    ItemUseDone = 0x1C7,
    #[serde(rename = "Allegiance_AllegianceUpdateDone")]
    AllegianceAllegianceUpdateDone = 0x1C8,
    #[serde(rename = "Fellowship_FellowUpdateDone")]
    FellowshipFellowUpdateDone = 0x1C9,
    #[serde(rename = "Fellowship_FellowStatsDone")]
    FellowshipFellowStatsDone = 0x1CA,
    #[serde(rename = "Item_AppraiseDone")]
    ItemAppraiseDone = 0x1CB,
    #[serde(rename = "Character_ReturnPing")]
    CharacterReturnPing = 0x1EA,
    #[serde(rename = "Communication_SetSquelchDB")]
    CommunicationSetSquelchDB = 0x1F4,
    #[serde(rename = "Trade_RegisterTrade")]
    TradeRegisterTrade = 0x1FD,
    #[serde(rename = "Trade_OpenTrade")]
    TradeOpenTrade = 0x1FE,
    #[serde(rename = "Trade_CloseTrade")]
    TradeCloseTrade = 0x1FF,
    #[serde(rename = "Trade_AddToTrade")]
    TradeAddToTrade = 0x200,
    #[serde(rename = "Trade_RemoveFromTrade")]
    TradeRemoveFromTrade = 0x201,
    #[serde(rename = "Trade_AcceptTrade")]
    TradeAcceptTrade = 0x202,
    #[serde(rename = "Trade_DeclineTrade")]
    TradeDeclineTrade = 0x203,
    #[serde(rename = "Trade_ResetTrade")]
    TradeResetTrade = 0x205,
    #[serde(rename = "Trade_TradeFailure")]
    TradeTradeFailure = 0x207,
    #[serde(rename = "Trade_ClearTradeAcceptance")]
    TradeClearTradeAcceptance = 0x208,
    #[serde(rename = "House_HouseProfile")]
    HouseHouseProfile = 0x21D,
    #[serde(rename = "House_HouseData")]
    HouseHouseData = 0x225,
    #[serde(rename = "House_HouseStatus")]
    HouseHouseStatus = 0x226,
    #[serde(rename = "House_UpdateRentTime")]
    HouseUpdateRentTime = 0x227,
    #[serde(rename = "House_UpdateRentPayment")]
    HouseUpdateRentPayment = 0x228,
    #[serde(rename = "House_UpdateRestrictions")]
    HouseUpdateRestrictions = 0x248,
    #[serde(rename = "House_UpdateHAR")]
    HouseUpdateHAR = 0x257,
    #[serde(rename = "House_HouseTransaction")]
    HouseHouseTransaction = 0x259,
    #[serde(rename = "Item_QueryItemManaResponse")]
    ItemQueryItemManaResponse = 0x264,
    #[serde(rename = "House_AvailableHouses")]
    HouseAvailableHouses = 0x271,
    #[serde(rename = "Character_ConfirmationRequest")]
    CharacterConfirmationRequest = 0x274,
    #[serde(rename = "Character_ConfirmationDone")]
    CharacterConfirmationDone = 0x276,
    #[serde(rename = "Allegiance_AllegianceLoginNotificationEvent")]
    AllegianceAllegianceLoginNotificationEvent = 0x27A,
    #[serde(rename = "Allegiance_AllegianceInfoResponseEvent")]
    AllegianceAllegianceInfoResponseEvent = 0x27C,
    #[serde(rename = "Game_JoinGameResponse")]
    GameJoinGameResponse = 0x281,
    #[serde(rename = "Game_StartGame")]
    GameStartGame = 0x282,
    #[serde(rename = "Game_MoveResponse")]
    GameMoveResponse = 0x283,
    #[serde(rename = "Game_OpponentTurn")]
    GameOpponentTurn = 0x284,
    #[serde(rename = "Game_OpponentStalemateState")]
    GameOpponentStalemateState = 0x285,
    #[serde(rename = "Communication_WeenieError")]
    CommunicationWeenieError = 0x28A,
    #[serde(rename = "Communication_WeenieErrorWithString")]
    CommunicationWeenieErrorWithString = 0x28B,
    #[serde(rename = "Game_GameOver")]
    GameGameOver = 0x28C,
    #[serde(rename = "Communication_ChatRoomTracker")]
    CommunicationChatRoomTracker = 0x295,
    #[serde(rename = "Admin_QueryPluginList")]
    AdminQueryPluginList = 0x2AE,
    #[serde(rename = "Admin_QueryPlugin")]
    AdminQueryPlugin = 0x2B1,
    #[serde(rename = "Admin_QueryPluginResponse2")]
    AdminQueryPluginResponse2 = 0x2B3,
    #[serde(rename = "Inventory_SalvageOperationsResultData")]
    InventorySalvageOperationsResultData = 0x2B4,
    #[serde(rename = "Communication_HearDirectSpeech")]
    CommunicationHearDirectSpeech = 0x2BD,
    #[serde(rename = "Fellowship_FullUpdate")]
    FellowshipFullUpdate = 0x2BE,
    #[serde(rename = "Fellowship_Disband")]
    FellowshipDisband = 0x2BF,
    #[serde(rename = "Fellowship_UpdateFellow")]
    FellowshipUpdateFellow = 0x2C0,
    #[serde(rename = "Magic_UpdateSpell")]
    MagicUpdateSpell = 0x2C1,
    #[serde(rename = "Magic_UpdateEnchantment")]
    MagicUpdateEnchantment = 0x2C2,
    #[serde(rename = "Magic_RemoveEnchantment")]
    MagicRemoveEnchantment = 0x2C3,
    #[serde(rename = "Magic_UpdateMultipleEnchantments")]
    MagicUpdateMultipleEnchantments = 0x2C4,
    #[serde(rename = "Magic_RemoveMultipleEnchantments")]
    MagicRemoveMultipleEnchantments = 0x2C5,
    #[serde(rename = "Magic_PurgeEnchantments")]
    MagicPurgeEnchantments = 0x2C6,
    #[serde(rename = "Magic_DispelEnchantment")]
    MagicDispelEnchantment = 0x2C7,
    #[serde(rename = "Magic_DispelMultipleEnchantments")]
    MagicDispelMultipleEnchantments = 0x2C8,
    #[serde(rename = "Misc_PortalStormBrewing")]
    MiscPortalStormBrewing = 0x2C9,
    #[serde(rename = "Misc_PortalStormImminent")]
    MiscPortalStormImminent = 0x2CA,
    #[serde(rename = "Misc_PortalStorm")]
    MiscPortalStorm = 0x2CB,
    #[serde(rename = "Misc_PortalStormSubsided")]
    MiscPortalStormSubsided = 0x2CC,
    #[serde(rename = "Communication_TransientString")]
    CommunicationTransientString = 0x2EB,
    #[serde(rename = "Magic_PurgeBadEnchantments")]
    MagicPurgeBadEnchantments = 0x312,
    #[serde(rename = "Social_SendClientContractTrackerTable")]
    SocialSendClientContractTrackerTable = 0x314,
    #[serde(rename = "Social_SendClientContractTracker")]
    SocialSendClientContractTracker = 0x315,
}

impl crate::readers::ACDataType for GameEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(GameEvent::try_from(value)?)
    }
}

impl crate::writers::ACWritable for GameEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for GameEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GameEvent::AllegianceAllegianceUpdateAborted => "Allegiance_AllegianceUpdateAborted",
            GameEvent::CommunicationPopUpString => "Communication_PopUpString",
            GameEvent::LoginPlayerDescription => "Login_PlayerDescription",
            GameEvent::AllegianceAllegianceUpdate => "Allegiance_AllegianceUpdate",
            GameEvent::SocialFriendsUpdate => "Social_FriendsUpdate",
            GameEvent::ItemServerSaysContainId => "Item_ServerSaysContainId",
            GameEvent::ItemWearItem => "Item_WearItem",
            GameEvent::SocialCharacterTitleTable => "Social_CharacterTitleTable",
            GameEvent::SocialAddOrSetCharacterTitle => "Social_AddOrSetCharacterTitle",
            GameEvent::ItemStopViewingObjectContents => "Item_StopViewingObjectContents",
            GameEvent::VendorVendorInfo => "Vendor_VendorInfo",
            GameEvent::CharacterStartBarber => "Character_StartBarber",
            GameEvent::FellowshipQuit => "Fellowship_Quit",
            GameEvent::FellowshipDismiss => "Fellowship_Dismiss",
            GameEvent::WritingBookOpen => "Writing_BookOpen",
            GameEvent::WritingBookAddPageResponse => "Writing_BookAddPageResponse",
            GameEvent::WritingBookDeletePageResponse => "Writing_BookDeletePageResponse",
            GameEvent::WritingBookPageDataResponse => "Writing_BookPageDataResponse",
            GameEvent::ItemGetInscriptionResponse => "Item_GetInscriptionResponse",
            GameEvent::ItemSetAppraiseInfo => "Item_SetAppraiseInfo",
            GameEvent::CommunicationChannelBroadcast => "Communication_ChannelBroadcast",
            GameEvent::CommunicationChannelList => "Communication_ChannelList",
            GameEvent::CommunicationChannelIndex => "Communication_ChannelIndex",
            GameEvent::ItemOnViewContents => "Item_OnViewContents",
            GameEvent::ItemServerSaysMoveItem => "Item_ServerSaysMoveItem",
            GameEvent::CombatHandleAttackDoneEvent => "Combat_HandleAttackDoneEvent",
            GameEvent::MagicRemoveSpell => "Magic_RemoveSpell",
            GameEvent::CombatHandleVictimNotificationEventSelf => "Combat_HandleVictimNotificationEventSelf",
            GameEvent::CombatHandleVictimNotificationEventOther => "Combat_HandleVictimNotificationEventOther",
            GameEvent::CombatHandleAttackerNotificationEvent => "Combat_HandleAttackerNotificationEvent",
            GameEvent::CombatHandleDefenderNotificationEvent => "Combat_HandleDefenderNotificationEvent",
            GameEvent::CombatHandleEvasionAttackerNotificationEvent => "Combat_HandleEvasionAttackerNotificationEvent",
            GameEvent::CombatHandleEvasionDefenderNotificationEvent => "Combat_HandleEvasionDefenderNotificationEvent",
            GameEvent::CombatHandleCommenceAttackEvent => "Combat_HandleCommenceAttackEvent",
            GameEvent::CombatQueryHealthResponse => "Combat_QueryHealthResponse",
            GameEvent::CharacterQueryAgeResponse => "Character_QueryAgeResponse",
            GameEvent::ItemUseDone => "Item_UseDone",
            GameEvent::AllegianceAllegianceUpdateDone => "Allegiance_AllegianceUpdateDone",
            GameEvent::FellowshipFellowUpdateDone => "Fellowship_FellowUpdateDone",
            GameEvent::FellowshipFellowStatsDone => "Fellowship_FellowStatsDone",
            GameEvent::ItemAppraiseDone => "Item_AppraiseDone",
            GameEvent::CharacterReturnPing => "Character_ReturnPing",
            GameEvent::CommunicationSetSquelchDB => "Communication_SetSquelchDB",
            GameEvent::TradeRegisterTrade => "Trade_RegisterTrade",
            GameEvent::TradeOpenTrade => "Trade_OpenTrade",
            GameEvent::TradeCloseTrade => "Trade_CloseTrade",
            GameEvent::TradeAddToTrade => "Trade_AddToTrade",
            GameEvent::TradeRemoveFromTrade => "Trade_RemoveFromTrade",
            GameEvent::TradeAcceptTrade => "Trade_AcceptTrade",
            GameEvent::TradeDeclineTrade => "Trade_DeclineTrade",
            GameEvent::TradeResetTrade => "Trade_ResetTrade",
            GameEvent::TradeTradeFailure => "Trade_TradeFailure",
            GameEvent::TradeClearTradeAcceptance => "Trade_ClearTradeAcceptance",
            GameEvent::HouseHouseProfile => "House_HouseProfile",
            GameEvent::HouseHouseData => "House_HouseData",
            GameEvent::HouseHouseStatus => "House_HouseStatus",
            GameEvent::HouseUpdateRentTime => "House_UpdateRentTime",
            GameEvent::HouseUpdateRentPayment => "House_UpdateRentPayment",
            GameEvent::HouseUpdateRestrictions => "House_UpdateRestrictions",
            GameEvent::HouseUpdateHAR => "House_UpdateHAR",
            GameEvent::HouseHouseTransaction => "House_HouseTransaction",
            GameEvent::ItemQueryItemManaResponse => "Item_QueryItemManaResponse",
            GameEvent::HouseAvailableHouses => "House_AvailableHouses",
            GameEvent::CharacterConfirmationRequest => "Character_ConfirmationRequest",
            GameEvent::CharacterConfirmationDone => "Character_ConfirmationDone",
            GameEvent::AllegianceAllegianceLoginNotificationEvent => "Allegiance_AllegianceLoginNotificationEvent",
            GameEvent::AllegianceAllegianceInfoResponseEvent => "Allegiance_AllegianceInfoResponseEvent",
            GameEvent::GameJoinGameResponse => "Game_JoinGameResponse",
            GameEvent::GameStartGame => "Game_StartGame",
            GameEvent::GameMoveResponse => "Game_MoveResponse",
            GameEvent::GameOpponentTurn => "Game_OpponentTurn",
            GameEvent::GameOpponentStalemateState => "Game_OpponentStalemateState",
            GameEvent::CommunicationWeenieError => "Communication_WeenieError",
            GameEvent::CommunicationWeenieErrorWithString => "Communication_WeenieErrorWithString",
            GameEvent::GameGameOver => "Game_GameOver",
            GameEvent::CommunicationChatRoomTracker => "Communication_ChatRoomTracker",
            GameEvent::AdminQueryPluginList => "Admin_QueryPluginList",
            GameEvent::AdminQueryPlugin => "Admin_QueryPlugin",
            GameEvent::AdminQueryPluginResponse2 => "Admin_QueryPluginResponse2",
            GameEvent::InventorySalvageOperationsResultData => "Inventory_SalvageOperationsResultData",
            GameEvent::CommunicationHearDirectSpeech => "Communication_HearDirectSpeech",
            GameEvent::FellowshipFullUpdate => "Fellowship_FullUpdate",
            GameEvent::FellowshipDisband => "Fellowship_Disband",
            GameEvent::FellowshipUpdateFellow => "Fellowship_UpdateFellow",
            GameEvent::MagicUpdateSpell => "Magic_UpdateSpell",
            GameEvent::MagicUpdateEnchantment => "Magic_UpdateEnchantment",
            GameEvent::MagicRemoveEnchantment => "Magic_RemoveEnchantment",
            GameEvent::MagicUpdateMultipleEnchantments => "Magic_UpdateMultipleEnchantments",
            GameEvent::MagicRemoveMultipleEnchantments => "Magic_RemoveMultipleEnchantments",
            GameEvent::MagicPurgeEnchantments => "Magic_PurgeEnchantments",
            GameEvent::MagicDispelEnchantment => "Magic_DispelEnchantment",
            GameEvent::MagicDispelMultipleEnchantments => "Magic_DispelMultipleEnchantments",
            GameEvent::MiscPortalStormBrewing => "Misc_PortalStormBrewing",
            GameEvent::MiscPortalStormImminent => "Misc_PortalStormImminent",
            GameEvent::MiscPortalStorm => "Misc_PortalStorm",
            GameEvent::MiscPortalStormSubsided => "Misc_PortalStormSubsided",
            GameEvent::CommunicationTransientString => "Communication_TransientString",
            GameEvent::MagicPurgeBadEnchantments => "Magic_PurgeBadEnchantments",
            GameEvent::SocialSendClientContractTrackerTable => "Social_SendClientContractTrackerTable",
            GameEvent::SocialSendClientContractTracker => "Social_SendClientContractTracker",
        };
        write!(f, "{}", s)
    }
}

/// Ordered (0xF7B1) Client to server opcodes
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum GameAction {
    #[serde(rename = "Character_PlayerOptionChangedEvent")]
    CharacterPlayerOptionChangedEvent = 0x5,
    #[serde(rename = "Combat_TargetedMeleeAttack")]
    CombatTargetedMeleeAttack = 0x8,
    #[serde(rename = "Combat_TargetedMissileAttack")]
    CombatTargetedMissileAttack = 0xA,
    #[serde(rename = "Communication_SetAFKMode")]
    CommunicationSetAFKMode = 0xF,
    #[serde(rename = "Communication_SetAFKMessage")]
    CommunicationSetAFKMessage = 0x10,
    #[serde(rename = "Communication_Talk")]
    CommunicationTalk = 0x15,
    #[serde(rename = "Social_RemoveFriend")]
    SocialRemoveFriend = 0x17,
    #[serde(rename = "Social_AddFriend")]
    SocialAddFriend = 0x18,
    #[serde(rename = "Inventory_PutItemInContainer")]
    InventoryPutItemInContainer = 0x19,
    #[serde(rename = "Inventory_GetAndWieldItem")]
    InventoryGetAndWieldItem = 0x1A,
    #[serde(rename = "Inventory_DropItem")]
    InventoryDropItem = 0x1B,
    #[serde(rename = "Allegiance_SwearAllegiance")]
    AllegianceSwearAllegiance = 0x1D,
    #[serde(rename = "Allegiance_BreakAllegiance")]
    AllegianceBreakAllegiance = 0x1E,
    #[serde(rename = "Allegiance_UpdateRequest")]
    AllegianceUpdateRequest = 0x1F,
    #[serde(rename = "Social_ClearFriends")]
    SocialClearFriends = 0x25,
    #[serde(rename = "Character_TeleToPKLArena")]
    CharacterTeleToPKLArena = 0x26,
    #[serde(rename = "Character_TeleToPKArena")]
    CharacterTeleToPKArena = 0x27,
    #[serde(rename = "Social_SetDisplayCharacterTitle")]
    SocialSetDisplayCharacterTitle = 0x2C,
    #[serde(rename = "Allegiance_QueryAllegianceName")]
    AllegianceQueryAllegianceName = 0x30,
    #[serde(rename = "Allegiance_ClearAllegianceName")]
    AllegianceClearAllegianceName = 0x31,
    #[serde(rename = "Communication_TalkDirect")]
    CommunicationTalkDirect = 0x32,
    #[serde(rename = "Allegiance_SetAllegianceName")]
    AllegianceSetAllegianceName = 0x33,
    #[serde(rename = "Inventory_UseWithTargetEvent")]
    InventoryUseWithTargetEvent = 0x35,
    #[serde(rename = "Inventory_UseEvent")]
    InventoryUseEvent = 0x36,
    #[serde(rename = "Allegiance_SetAllegianceOfficer")]
    AllegianceSetAllegianceOfficer = 0x3B,
    #[serde(rename = "Allegiance_SetAllegianceOfficerTitle")]
    AllegianceSetAllegianceOfficerTitle = 0x3C,
    #[serde(rename = "Allegiance_ListAllegianceOfficerTitles")]
    AllegianceListAllegianceOfficerTitles = 0x3D,
    #[serde(rename = "Allegiance_ClearAllegianceOfficerTitles")]
    AllegianceClearAllegianceOfficerTitles = 0x3E,
    #[serde(rename = "Allegiance_DoAllegianceLockAction")]
    AllegianceDoAllegianceLockAction = 0x3F,
    #[serde(rename = "Allegiance_SetAllegianceApprovedVassal")]
    AllegianceSetAllegianceApprovedVassal = 0x40,
    #[serde(rename = "Allegiance_AllegianceChatGag")]
    AllegianceAllegianceChatGag = 0x41,
    #[serde(rename = "Allegiance_DoAllegianceHouseAction")]
    AllegianceDoAllegianceHouseAction = 0x42,
    #[serde(rename = "Train_TrainAttribute2nd")]
    TrainTrainAttribute2nd = 0x44,
    #[serde(rename = "Train_TrainAttribute")]
    TrainTrainAttribute = 0x45,
    #[serde(rename = "Train_TrainSkill")]
    TrainTrainSkill = 0x46,
    #[serde(rename = "Train_TrainSkillAdvancementClass")]
    TrainTrainSkillAdvancementClass = 0x47,
    #[serde(rename = "Magic_CastUntargetedSpell")]
    MagicCastUntargetedSpell = 0x48,
    #[serde(rename = "Magic_CastTargetedSpell")]
    MagicCastTargetedSpell = 0x4A,
    #[serde(rename = "Combat_ChangeCombatMode")]
    CombatChangeCombatMode = 0x53,
    #[serde(rename = "Inventory_StackableMerge")]
    InventoryStackableMerge = 0x54,
    #[serde(rename = "Inventory_StackableSplitToContainer")]
    InventoryStackableSplitToContainer = 0x55,
    #[serde(rename = "Inventory_StackableSplitTo3D")]
    InventoryStackableSplitTo3D = 0x56,
    #[serde(rename = "Communication_ModifyCharacterSquelch")]
    CommunicationModifyCharacterSquelch = 0x58,
    #[serde(rename = "Communication_ModifyAccountSquelch")]
    CommunicationModifyAccountSquelch = 0x59,
    #[serde(rename = "Communication_ModifyGlobalSquelch")]
    CommunicationModifyGlobalSquelch = 0x5B,
    #[serde(rename = "Communication_TalkDirectByName")]
    CommunicationTalkDirectByName = 0x5D,
    #[serde(rename = "Vendor_Buy")]
    VendorBuy = 0x5F,
    #[serde(rename = "Vendor_Sell")]
    VendorSell = 0x60,
    #[serde(rename = "Character_TeleToLifestone")]
    CharacterTeleToLifestone = 0x63,
    #[serde(rename = "Character_LoginCompleteNotification")]
    CharacterLoginCompleteNotification = 0xA1,
    #[serde(rename = "Fellowship_Create")]
    FellowshipCreate = 0xA2,
    #[serde(rename = "Fellowship_Quit")]
    FellowshipQuit = 0xA3,
    #[serde(rename = "Fellowship_Dismiss")]
    FellowshipDismiss = 0xA4,
    #[serde(rename = "Fellowship_Recruit")]
    FellowshipRecruit = 0xA5,
    #[serde(rename = "Fellowship_UpdateRequest")]
    FellowshipUpdateRequest = 0xA6,
    #[serde(rename = "Writing_BookAddPage")]
    WritingBookAddPage = 0xAA,
    #[serde(rename = "Writing_BookModifyPage")]
    WritingBookModifyPage = 0xAB,
    #[serde(rename = "Writing_BookData")]
    WritingBookData = 0xAC,
    #[serde(rename = "Writing_BookDeletePage")]
    WritingBookDeletePage = 0xAD,
    #[serde(rename = "Writing_BookPageData")]
    WritingBookPageData = 0xAE,
    #[serde(rename = "Writing_SetInscription")]
    WritingSetInscription = 0xBF,
    #[serde(rename = "Item_Appraise")]
    ItemAppraise = 0xC8,
    #[serde(rename = "Inventory_GiveObjectRequest")]
    InventoryGiveObjectRequest = 0xCD,
    #[serde(rename = "Advocate_Teleport")]
    AdvocateTeleport = 0xD6,
    #[serde(rename = "Character_AbuseLogRequest")]
    CharacterAbuseLogRequest = 0x140,
    #[serde(rename = "Communication_AddToChannel")]
    CommunicationAddToChannel = 0x145,
    #[serde(rename = "Communication_RemoveFromChannel")]
    CommunicationRemoveFromChannel = 0x146,
    #[serde(rename = "Communication_ChannelBroadcast")]
    CommunicationChannelBroadcast = 0x147,
    #[serde(rename = "Communication_ChannelList")]
    CommunicationChannelList = 0x148,
    #[serde(rename = "Communication_ChannelIndex")]
    CommunicationChannelIndex = 0x149,
    #[serde(rename = "Inventory_NoLongerViewingContents")]
    InventoryNoLongerViewingContents = 0x195,
    #[serde(rename = "Inventory_StackableSplitToWield")]
    InventoryStackableSplitToWield = 0x19B,
    #[serde(rename = "Character_AddShortCut")]
    CharacterAddShortCut = 0x19C,
    #[serde(rename = "Character_RemoveShortCut")]
    CharacterRemoveShortCut = 0x19D,
    #[serde(rename = "Character_CharacterOptionsEvent")]
    CharacterCharacterOptionsEvent = 0x1A1,
    #[serde(rename = "Magic_RemoveSpell")]
    MagicRemoveSpell = 0x1A8,
    #[serde(rename = "Combat_CancelAttack")]
    CombatCancelAttack = 0x1B7,
    #[serde(rename = "Combat_QueryHealth")]
    CombatQueryHealth = 0x1BF,
    #[serde(rename = "Character_QueryAge")]
    CharacterQueryAge = 0x1C2,
    #[serde(rename = "Character_QueryBirth")]
    CharacterQueryBirth = 0x1C4,
    #[serde(rename = "Communication_Emote")]
    CommunicationEmote = 0x1DF,
    #[serde(rename = "Communication_SoulEmote")]
    CommunicationSoulEmote = 0x1E1,
    #[serde(rename = "Character_AddSpellFavorite")]
    CharacterAddSpellFavorite = 0x1E3,
    #[serde(rename = "Character_RemoveSpellFavorite")]
    CharacterRemoveSpellFavorite = 0x1E4,
    #[serde(rename = "Character_RequestPing")]
    CharacterRequestPing = 0x1E9,
    #[serde(rename = "Trade_OpenTradeNegotiations")]
    TradeOpenTradeNegotiations = 0x1F6,
    #[serde(rename = "Trade_CloseTradeNegotiations")]
    TradeCloseTradeNegotiations = 0x1F7,
    #[serde(rename = "Trade_AddToTrade")]
    TradeAddToTrade = 0x1F8,
    #[serde(rename = "Trade_AcceptTrade")]
    TradeAcceptTrade = 0x1FA,
    #[serde(rename = "Trade_DeclineTrade")]
    TradeDeclineTrade = 0x1FB,
    #[serde(rename = "Trade_ResetTrade")]
    TradeResetTrade = 0x204,
    #[serde(rename = "Character_ClearPlayerConsentList")]
    CharacterClearPlayerConsentList = 0x216,
    #[serde(rename = "Character_DisplayPlayerConsentList")]
    CharacterDisplayPlayerConsentList = 0x217,
    #[serde(rename = "Character_RemoveFromPlayerConsentList")]
    CharacterRemoveFromPlayerConsentList = 0x218,
    #[serde(rename = "Character_AddPlayerPermission")]
    CharacterAddPlayerPermission = 0x219,
    #[serde(rename = "House_BuyHouse")]
    HouseBuyHouse = 0x21C,
    #[serde(rename = "House_QueryHouse")]
    HouseQueryHouse = 0x21E,
    #[serde(rename = "House_AbandonHouse")]
    HouseAbandonHouse = 0x21F,
    #[serde(rename = "Character_RemovePlayerPermission")]
    CharacterRemovePlayerPermission = 0x220,
    #[serde(rename = "House_RentHouse")]
    HouseRentHouse = 0x221,
    #[serde(rename = "Character_SetDesiredComponentLevel")]
    CharacterSetDesiredComponentLevel = 0x224,
    #[serde(rename = "House_AddPermanentGuest")]
    HouseAddPermanentGuest = 0x245,
    #[serde(rename = "House_RemovePermanentGuest")]
    HouseRemovePermanentGuest = 0x246,
    #[serde(rename = "House_SetOpenHouseStatus")]
    HouseSetOpenHouseStatus = 0x247,
    #[serde(rename = "House_ChangeStoragePermission")]
    HouseChangeStoragePermission = 0x249,
    #[serde(rename = "House_BootSpecificHouseGuest")]
    HouseBootSpecificHouseGuest = 0x24A,
    #[serde(rename = "House_RemoveAllStoragePermission")]
    HouseRemoveAllStoragePermission = 0x24C,
    #[serde(rename = "House_RequestFullGuestList")]
    HouseRequestFullGuestList = 0x24D,
    #[serde(rename = "Allegiance_SetMotd")]
    AllegianceSetMotd = 0x254,
    #[serde(rename = "Allegiance_QueryMotd")]
    AllegianceQueryMotd = 0x255,
    #[serde(rename = "Allegiance_ClearMotd")]
    AllegianceClearMotd = 0x256,
    #[serde(rename = "House_QueryLord")]
    HouseQueryLord = 0x258,
    #[serde(rename = "House_AddAllStoragePermission")]
    HouseAddAllStoragePermission = 0x25C,
    #[serde(rename = "House_RemoveAllPermanentGuests")]
    HouseRemoveAllPermanentGuests = 0x25E,
    #[serde(rename = "House_BootEveryone")]
    HouseBootEveryone = 0x25F,
    #[serde(rename = "House_TeleToHouse")]
    HouseTeleToHouse = 0x262,
    #[serde(rename = "Item_QueryItemMana")]
    ItemQueryItemMana = 0x263,
    #[serde(rename = "House_SetHooksVisibility")]
    HouseSetHooksVisibility = 0x266,
    #[serde(rename = "House_ModifyAllegianceGuestPermission")]
    HouseModifyAllegianceGuestPermission = 0x267,
    #[serde(rename = "House_ModifyAllegianceStoragePermission")]
    HouseModifyAllegianceStoragePermission = 0x268,
    #[serde(rename = "Game_Join")]
    GameJoin = 0x269,
    #[serde(rename = "Game_Quit")]
    GameQuit = 0x26A,
    #[serde(rename = "Game_Move")]
    GameMove = 0x26B,
    #[serde(rename = "Game_MovePass")]
    GameMovePass = 0x26D,
    #[serde(rename = "Game_Stalemate")]
    GameStalemate = 0x26E,
    #[serde(rename = "House_ListAvailableHouses")]
    HouseListAvailableHouses = 0x270,
    #[serde(rename = "Character_ConfirmationResponse")]
    CharacterConfirmationResponse = 0x275,
    #[serde(rename = "Allegiance_BreakAllegianceBoot")]
    AllegianceBreakAllegianceBoot = 0x277,
    #[serde(rename = "House_TeleToMansion")]
    HouseTeleToMansion = 0x278,
    #[serde(rename = "Character_Suicide")]
    CharacterSuicide = 0x279,
    #[serde(rename = "Allegiance_AllegianceInfoRequest")]
    AllegianceAllegianceInfoRequest = 0x27B,
    #[serde(rename = "Inventory_CreateTinkeringTool")]
    InventoryCreateTinkeringTool = 0x27D,
    #[serde(rename = "Character_SpellbookFilterEvent")]
    CharacterSpellbookFilterEvent = 0x286,
    #[serde(rename = "Character_TeleToMarketplace")]
    CharacterTeleToMarketplace = 0x28D,
    #[serde(rename = "Character_EnterPKLite")]
    CharacterEnterPKLite = 0x28F,
    #[serde(rename = "Fellowship_AssignNewLeader")]
    FellowshipAssignNewLeader = 0x290,
    #[serde(rename = "Fellowship_ChangeFellowOpeness")]
    FellowshipChangeFellowOpeness = 0x291,
    #[serde(rename = "Allegiance_AllegianceChatBoot")]
    AllegianceAllegianceChatBoot = 0x2A0,
    #[serde(rename = "Allegiance_AddAllegianceBan")]
    AllegianceAddAllegianceBan = 0x2A1,
    #[serde(rename = "Allegiance_RemoveAllegianceBan")]
    AllegianceRemoveAllegianceBan = 0x2A2,
    #[serde(rename = "Allegiance_ListAllegianceBans")]
    AllegianceListAllegianceBans = 0x2A3,
    #[serde(rename = "Allegiance_RemoveAllegianceOfficer")]
    AllegianceRemoveAllegianceOfficer = 0x2A5,
    #[serde(rename = "Allegiance_ListAllegianceOfficers")]
    AllegianceListAllegianceOfficers = 0x2A6,
    #[serde(rename = "Allegiance_ClearAllegianceOfficers")]
    AllegianceClearAllegianceOfficers = 0x2A7,
    #[serde(rename = "Allegiance_RecallAllegianceHometown")]
    AllegianceRecallAllegianceHometown = 0x2AB,
    #[serde(rename = "Admin_QueryPluginListResponse")]
    AdminQueryPluginListResponse = 0x2AF,
    #[serde(rename = "Admin_QueryPluginResponse")]
    AdminQueryPluginResponse = 0x2B2,
    #[serde(rename = "Character_FinishBarber")]
    CharacterFinishBarber = 0x311,
    #[serde(rename = "Social_AbandonContract")]
    SocialAbandonContract = 0x316,
    #[serde(rename = "Movement_Jump")]
    MovementJump = 0xF61B,
    #[serde(rename = "Movement_MoveToState")]
    MovementMoveToState = 0xF61C,
    #[serde(rename = "Movement_DoMovementCommand")]
    MovementDoMovementCommand = 0xF61E,
    #[serde(rename = "Movement_StopMovementCommand")]
    MovementStopMovementCommand = 0xF661,
    #[serde(rename = "Movement_AutonomyLevel")]
    MovementAutonomyLevel = 0xF752,
    #[serde(rename = "Movement_AutonomousPosition")]
    MovementAutonomousPosition = 0xF753,
    #[serde(rename = "Movement_Jump_NonAutonomous")]
    MovementJumpNonAutonomous = 0xF7C9,
}

impl crate::readers::ACDataType for GameAction {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(GameAction::try_from(value)?)
    }
}

impl crate::writers::ACWritable for GameAction {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for GameAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GameAction::CharacterPlayerOptionChangedEvent => "Character_PlayerOptionChangedEvent",
            GameAction::CombatTargetedMeleeAttack => "Combat_TargetedMeleeAttack",
            GameAction::CombatTargetedMissileAttack => "Combat_TargetedMissileAttack",
            GameAction::CommunicationSetAFKMode => "Communication_SetAFKMode",
            GameAction::CommunicationSetAFKMessage => "Communication_SetAFKMessage",
            GameAction::CommunicationTalk => "Communication_Talk",
            GameAction::SocialRemoveFriend => "Social_RemoveFriend",
            GameAction::SocialAddFriend => "Social_AddFriend",
            GameAction::InventoryPutItemInContainer => "Inventory_PutItemInContainer",
            GameAction::InventoryGetAndWieldItem => "Inventory_GetAndWieldItem",
            GameAction::InventoryDropItem => "Inventory_DropItem",
            GameAction::AllegianceSwearAllegiance => "Allegiance_SwearAllegiance",
            GameAction::AllegianceBreakAllegiance => "Allegiance_BreakAllegiance",
            GameAction::AllegianceUpdateRequest => "Allegiance_UpdateRequest",
            GameAction::SocialClearFriends => "Social_ClearFriends",
            GameAction::CharacterTeleToPKLArena => "Character_TeleToPKLArena",
            GameAction::CharacterTeleToPKArena => "Character_TeleToPKArena",
            GameAction::SocialSetDisplayCharacterTitle => "Social_SetDisplayCharacterTitle",
            GameAction::AllegianceQueryAllegianceName => "Allegiance_QueryAllegianceName",
            GameAction::AllegianceClearAllegianceName => "Allegiance_ClearAllegianceName",
            GameAction::CommunicationTalkDirect => "Communication_TalkDirect",
            GameAction::AllegianceSetAllegianceName => "Allegiance_SetAllegianceName",
            GameAction::InventoryUseWithTargetEvent => "Inventory_UseWithTargetEvent",
            GameAction::InventoryUseEvent => "Inventory_UseEvent",
            GameAction::AllegianceSetAllegianceOfficer => "Allegiance_SetAllegianceOfficer",
            GameAction::AllegianceSetAllegianceOfficerTitle => "Allegiance_SetAllegianceOfficerTitle",
            GameAction::AllegianceListAllegianceOfficerTitles => "Allegiance_ListAllegianceOfficerTitles",
            GameAction::AllegianceClearAllegianceOfficerTitles => "Allegiance_ClearAllegianceOfficerTitles",
            GameAction::AllegianceDoAllegianceLockAction => "Allegiance_DoAllegianceLockAction",
            GameAction::AllegianceSetAllegianceApprovedVassal => "Allegiance_SetAllegianceApprovedVassal",
            GameAction::AllegianceAllegianceChatGag => "Allegiance_AllegianceChatGag",
            GameAction::AllegianceDoAllegianceHouseAction => "Allegiance_DoAllegianceHouseAction",
            GameAction::TrainTrainAttribute2nd => "Train_TrainAttribute2nd",
            GameAction::TrainTrainAttribute => "Train_TrainAttribute",
            GameAction::TrainTrainSkill => "Train_TrainSkill",
            GameAction::TrainTrainSkillAdvancementClass => "Train_TrainSkillAdvancementClass",
            GameAction::MagicCastUntargetedSpell => "Magic_CastUntargetedSpell",
            GameAction::MagicCastTargetedSpell => "Magic_CastTargetedSpell",
            GameAction::CombatChangeCombatMode => "Combat_ChangeCombatMode",
            GameAction::InventoryStackableMerge => "Inventory_StackableMerge",
            GameAction::InventoryStackableSplitToContainer => "Inventory_StackableSplitToContainer",
            GameAction::InventoryStackableSplitTo3D => "Inventory_StackableSplitTo3D",
            GameAction::CommunicationModifyCharacterSquelch => "Communication_ModifyCharacterSquelch",
            GameAction::CommunicationModifyAccountSquelch => "Communication_ModifyAccountSquelch",
            GameAction::CommunicationModifyGlobalSquelch => "Communication_ModifyGlobalSquelch",
            GameAction::CommunicationTalkDirectByName => "Communication_TalkDirectByName",
            GameAction::VendorBuy => "Vendor_Buy",
            GameAction::VendorSell => "Vendor_Sell",
            GameAction::CharacterTeleToLifestone => "Character_TeleToLifestone",
            GameAction::CharacterLoginCompleteNotification => "Character_LoginCompleteNotification",
            GameAction::FellowshipCreate => "Fellowship_Create",
            GameAction::FellowshipQuit => "Fellowship_Quit",
            GameAction::FellowshipDismiss => "Fellowship_Dismiss",
            GameAction::FellowshipRecruit => "Fellowship_Recruit",
            GameAction::FellowshipUpdateRequest => "Fellowship_UpdateRequest",
            GameAction::WritingBookAddPage => "Writing_BookAddPage",
            GameAction::WritingBookModifyPage => "Writing_BookModifyPage",
            GameAction::WritingBookData => "Writing_BookData",
            GameAction::WritingBookDeletePage => "Writing_BookDeletePage",
            GameAction::WritingBookPageData => "Writing_BookPageData",
            GameAction::WritingSetInscription => "Writing_SetInscription",
            GameAction::ItemAppraise => "Item_Appraise",
            GameAction::InventoryGiveObjectRequest => "Inventory_GiveObjectRequest",
            GameAction::AdvocateTeleport => "Advocate_Teleport",
            GameAction::CharacterAbuseLogRequest => "Character_AbuseLogRequest",
            GameAction::CommunicationAddToChannel => "Communication_AddToChannel",
            GameAction::CommunicationRemoveFromChannel => "Communication_RemoveFromChannel",
            GameAction::CommunicationChannelBroadcast => "Communication_ChannelBroadcast",
            GameAction::CommunicationChannelList => "Communication_ChannelList",
            GameAction::CommunicationChannelIndex => "Communication_ChannelIndex",
            GameAction::InventoryNoLongerViewingContents => "Inventory_NoLongerViewingContents",
            GameAction::InventoryStackableSplitToWield => "Inventory_StackableSplitToWield",
            GameAction::CharacterAddShortCut => "Character_AddShortCut",
            GameAction::CharacterRemoveShortCut => "Character_RemoveShortCut",
            GameAction::CharacterCharacterOptionsEvent => "Character_CharacterOptionsEvent",
            GameAction::MagicRemoveSpell => "Magic_RemoveSpell",
            GameAction::CombatCancelAttack => "Combat_CancelAttack",
            GameAction::CombatQueryHealth => "Combat_QueryHealth",
            GameAction::CharacterQueryAge => "Character_QueryAge",
            GameAction::CharacterQueryBirth => "Character_QueryBirth",
            GameAction::CommunicationEmote => "Communication_Emote",
            GameAction::CommunicationSoulEmote => "Communication_SoulEmote",
            GameAction::CharacterAddSpellFavorite => "Character_AddSpellFavorite",
            GameAction::CharacterRemoveSpellFavorite => "Character_RemoveSpellFavorite",
            GameAction::CharacterRequestPing => "Character_RequestPing",
            GameAction::TradeOpenTradeNegotiations => "Trade_OpenTradeNegotiations",
            GameAction::TradeCloseTradeNegotiations => "Trade_CloseTradeNegotiations",
            GameAction::TradeAddToTrade => "Trade_AddToTrade",
            GameAction::TradeAcceptTrade => "Trade_AcceptTrade",
            GameAction::TradeDeclineTrade => "Trade_DeclineTrade",
            GameAction::TradeResetTrade => "Trade_ResetTrade",
            GameAction::CharacterClearPlayerConsentList => "Character_ClearPlayerConsentList",
            GameAction::CharacterDisplayPlayerConsentList => "Character_DisplayPlayerConsentList",
            GameAction::CharacterRemoveFromPlayerConsentList => "Character_RemoveFromPlayerConsentList",
            GameAction::CharacterAddPlayerPermission => "Character_AddPlayerPermission",
            GameAction::HouseBuyHouse => "House_BuyHouse",
            GameAction::HouseQueryHouse => "House_QueryHouse",
            GameAction::HouseAbandonHouse => "House_AbandonHouse",
            GameAction::CharacterRemovePlayerPermission => "Character_RemovePlayerPermission",
            GameAction::HouseRentHouse => "House_RentHouse",
            GameAction::CharacterSetDesiredComponentLevel => "Character_SetDesiredComponentLevel",
            GameAction::HouseAddPermanentGuest => "House_AddPermanentGuest",
            GameAction::HouseRemovePermanentGuest => "House_RemovePermanentGuest",
            GameAction::HouseSetOpenHouseStatus => "House_SetOpenHouseStatus",
            GameAction::HouseChangeStoragePermission => "House_ChangeStoragePermission",
            GameAction::HouseBootSpecificHouseGuest => "House_BootSpecificHouseGuest",
            GameAction::HouseRemoveAllStoragePermission => "House_RemoveAllStoragePermission",
            GameAction::HouseRequestFullGuestList => "House_RequestFullGuestList",
            GameAction::AllegianceSetMotd => "Allegiance_SetMotd",
            GameAction::AllegianceQueryMotd => "Allegiance_QueryMotd",
            GameAction::AllegianceClearMotd => "Allegiance_ClearMotd",
            GameAction::HouseQueryLord => "House_QueryLord",
            GameAction::HouseAddAllStoragePermission => "House_AddAllStoragePermission",
            GameAction::HouseRemoveAllPermanentGuests => "House_RemoveAllPermanentGuests",
            GameAction::HouseBootEveryone => "House_BootEveryone",
            GameAction::HouseTeleToHouse => "House_TeleToHouse",
            GameAction::ItemQueryItemMana => "Item_QueryItemMana",
            GameAction::HouseSetHooksVisibility => "House_SetHooksVisibility",
            GameAction::HouseModifyAllegianceGuestPermission => "House_ModifyAllegianceGuestPermission",
            GameAction::HouseModifyAllegianceStoragePermission => "House_ModifyAllegianceStoragePermission",
            GameAction::GameJoin => "Game_Join",
            GameAction::GameQuit => "Game_Quit",
            GameAction::GameMove => "Game_Move",
            GameAction::GameMovePass => "Game_MovePass",
            GameAction::GameStalemate => "Game_Stalemate",
            GameAction::HouseListAvailableHouses => "House_ListAvailableHouses",
            GameAction::CharacterConfirmationResponse => "Character_ConfirmationResponse",
            GameAction::AllegianceBreakAllegianceBoot => "Allegiance_BreakAllegianceBoot",
            GameAction::HouseTeleToMansion => "House_TeleToMansion",
            GameAction::CharacterSuicide => "Character_Suicide",
            GameAction::AllegianceAllegianceInfoRequest => "Allegiance_AllegianceInfoRequest",
            GameAction::InventoryCreateTinkeringTool => "Inventory_CreateTinkeringTool",
            GameAction::CharacterSpellbookFilterEvent => "Character_SpellbookFilterEvent",
            GameAction::CharacterTeleToMarketplace => "Character_TeleToMarketplace",
            GameAction::CharacterEnterPKLite => "Character_EnterPKLite",
            GameAction::FellowshipAssignNewLeader => "Fellowship_AssignNewLeader",
            GameAction::FellowshipChangeFellowOpeness => "Fellowship_ChangeFellowOpeness",
            GameAction::AllegianceAllegianceChatBoot => "Allegiance_AllegianceChatBoot",
            GameAction::AllegianceAddAllegianceBan => "Allegiance_AddAllegianceBan",
            GameAction::AllegianceRemoveAllegianceBan => "Allegiance_RemoveAllegianceBan",
            GameAction::AllegianceListAllegianceBans => "Allegiance_ListAllegianceBans",
            GameAction::AllegianceRemoveAllegianceOfficer => "Allegiance_RemoveAllegianceOfficer",
            GameAction::AllegianceListAllegianceOfficers => "Allegiance_ListAllegianceOfficers",
            GameAction::AllegianceClearAllegianceOfficers => "Allegiance_ClearAllegianceOfficers",
            GameAction::AllegianceRecallAllegianceHometown => "Allegiance_RecallAllegianceHometown",
            GameAction::AdminQueryPluginListResponse => "Admin_QueryPluginListResponse",
            GameAction::AdminQueryPluginResponse => "Admin_QueryPluginResponse",
            GameAction::CharacterFinishBarber => "Character_FinishBarber",
            GameAction::SocialAbandonContract => "Social_AbandonContract",
            GameAction::MovementJump => "Movement_Jump",
            GameAction::MovementMoveToState => "Movement_MoveToState",
            GameAction::MovementDoMovementCommand => "Movement_DoMovementCommand",
            GameAction::MovementStopMovementCommand => "Movement_StopMovementCommand",
            GameAction::MovementAutonomyLevel => "Movement_AutonomyLevel",
            GameAction::MovementAutonomousPosition => "Movement_AutonomousPosition",
            GameAction::MovementJumpNonAutonomous => "Movement_Jump_NonAutonomous",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum WeenieType {
    Undef = 0x0,
    Generic = 0x1,
    Clothing = 0x2,
    MissileLauncher = 0x3,
    Missile = 0x4,
    Ammunition = 0x5,
    MeleeWeapon = 0x6,
    Portal = 0x7,
    Book = 0x8,
    Coin = 0x9,
    Creature = 0xA,
    Admin = 0xB,
    Vendor = 0xC,
    HotSpot = 0xD,
    Corpse = 0xE,
    Cow = 0xF,
    AI = 0x10,
    Machine = 0x11,
    Food = 0x12,
    Door = 0x13,
    Chest = 0x14,
    Container = 0x15,
    Key = 0x16,
    Lockpick = 0x17,
    PressurePlate = 0x18,
    LifeStone = 0x19,
    Switch = 0x1A,
    PKModifier = 0x1B,
    Healer = 0x1C,
    LightSource = 0x1D,
    Allegiance = 0x1E,
    #[serde(rename = "UNKNOWN__GUESSEDNAME32")]
    UNKNOWNGUESSEDNAME32 = 0x1F,
    SpellComponent = 0x20,
    ProjectileSpell = 0x21,
    Scroll = 0x22,
    Caster = 0x23,
    Channel = 0x24,
    ManaStone = 0x25,
    Gem = 0x26,
    AdvocateFane = 0x27,
    AdvocateItem = 0x28,
    Sentinel = 0x29,
    GSpellEconomy = 0x2A,
    LSpellEconomy = 0x2B,
    CraftTool = 0x2C,
    LScoreKeeper = 0x2D,
    GScoreKeeper = 0x2E,
    GScoreGatherer = 0x2F,
    ScoreBook = 0x30,
    EventCoordinator = 0x31,
    Entity = 0x32,
    Stackable = 0x33,
    HUD = 0x34,
    House = 0x35,
    Deed = 0x36,
    SlumLord = 0x37,
    Hook = 0x38,
    Storage = 0x39,
    BootSpot = 0x3A,
    HousePortal = 0x3B,
    Game = 0x3C,
    GamePiece = 0x3D,
    SkillAlterationDevice = 0x3E,
    AttributeTransferDevice = 0x3F,
    Hooker = 0x40,
    AllegianceBindstone = 0x41,
    InGameStatKeeper = 0x42,
    AugmentationDevice = 0x43,
    SocialManager = 0x44,
    Pet = 0x45,
    PetDevice = 0x46,
    CombatPet = 0x47,
}

impl crate::readers::ACDataType for WeenieType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(WeenieType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for WeenieType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for WeenieType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WeenieType::Undef => "Undef",
            WeenieType::Generic => "Generic",
            WeenieType::Clothing => "Clothing",
            WeenieType::MissileLauncher => "MissileLauncher",
            WeenieType::Missile => "Missile",
            WeenieType::Ammunition => "Ammunition",
            WeenieType::MeleeWeapon => "MeleeWeapon",
            WeenieType::Portal => "Portal",
            WeenieType::Book => "Book",
            WeenieType::Coin => "Coin",
            WeenieType::Creature => "Creature",
            WeenieType::Admin => "Admin",
            WeenieType::Vendor => "Vendor",
            WeenieType::HotSpot => "HotSpot",
            WeenieType::Corpse => "Corpse",
            WeenieType::Cow => "Cow",
            WeenieType::AI => "AI",
            WeenieType::Machine => "Machine",
            WeenieType::Food => "Food",
            WeenieType::Door => "Door",
            WeenieType::Chest => "Chest",
            WeenieType::Container => "Container",
            WeenieType::Key => "Key",
            WeenieType::Lockpick => "Lockpick",
            WeenieType::PressurePlate => "PressurePlate",
            WeenieType::LifeStone => "LifeStone",
            WeenieType::Switch => "Switch",
            WeenieType::PKModifier => "PKModifier",
            WeenieType::Healer => "Healer",
            WeenieType::LightSource => "LightSource",
            WeenieType::Allegiance => "Allegiance",
            WeenieType::UNKNOWNGUESSEDNAME32 => "UNKNOWN__GUESSEDNAME32",
            WeenieType::SpellComponent => "SpellComponent",
            WeenieType::ProjectileSpell => "ProjectileSpell",
            WeenieType::Scroll => "Scroll",
            WeenieType::Caster => "Caster",
            WeenieType::Channel => "Channel",
            WeenieType::ManaStone => "ManaStone",
            WeenieType::Gem => "Gem",
            WeenieType::AdvocateFane => "AdvocateFane",
            WeenieType::AdvocateItem => "AdvocateItem",
            WeenieType::Sentinel => "Sentinel",
            WeenieType::GSpellEconomy => "GSpellEconomy",
            WeenieType::LSpellEconomy => "LSpellEconomy",
            WeenieType::CraftTool => "CraftTool",
            WeenieType::LScoreKeeper => "LScoreKeeper",
            WeenieType::GScoreKeeper => "GScoreKeeper",
            WeenieType::GScoreGatherer => "GScoreGatherer",
            WeenieType::ScoreBook => "ScoreBook",
            WeenieType::EventCoordinator => "EventCoordinator",
            WeenieType::Entity => "Entity",
            WeenieType::Stackable => "Stackable",
            WeenieType::HUD => "HUD",
            WeenieType::House => "House",
            WeenieType::Deed => "Deed",
            WeenieType::SlumLord => "SlumLord",
            WeenieType::Hook => "Hook",
            WeenieType::Storage => "Storage",
            WeenieType::BootSpot => "BootSpot",
            WeenieType::HousePortal => "HousePortal",
            WeenieType::Game => "Game",
            WeenieType::GamePiece => "GamePiece",
            WeenieType::SkillAlterationDevice => "SkillAlterationDevice",
            WeenieType::AttributeTransferDevice => "AttributeTransferDevice",
            WeenieType::Hooker => "Hooker",
            WeenieType::AllegianceBindstone => "AllegianceBindstone",
            WeenieType::InGameStatKeeper => "InGameStatKeeper",
            WeenieType::AugmentationDevice => "AugmentationDevice",
            WeenieType::SocialManager => "SocialManager",
            WeenieType::Pet => "Pet",
            WeenieType::PetDevice => "PetDevice",
            WeenieType::CombatPet => "CombatPet",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    /// Flags that dictate what property tables are included with the ACBaseQuali
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ACBaseQualitiesFlags: u32 {
        const NONE = 0x0;
        const PROPERTY_INT = 0x1;
        const PROPERTY_BOOL = 0x2;
        const PROPERTY_FLOAT = 0x4;
        const PROPERTY_DATA_ID = 0x8;
        const PROPERTY_STRING = 0x10;
        const PROPERTY_POSITION = 0x20;
        const PROPERTY_INSTANCE_ID = 0x40;
        const PROPERTY_INT64 = 0x80;
    }
}

impl crate::readers::ACDataType for ACBaseQualitiesFlags {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ACBaseQualitiesFlags::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for ACBaseQualitiesFlags {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

/// Set of predefined error messages that accept interpolated string argument
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum WeenieErrorWithString {
    IsTooBusyToAcceptGifts = 0x1E,
    CannotCarryAnymore = 0x2B,
    #[serde(rename = "YouFailToAffect_YouCannotAffectAnyone")]
    YouFailToAffectYouCannotAffectAnyone = 0x4E,
    #[serde(rename = "YouFailToAffect_TheyCannotBeHarmed")]
    YouFailToAffectTheyCannotBeHarmed = 0x4F,
    #[serde(rename = "YouFailToAffect_WithBeneficialSpells")]
    YouFailToAffectWithBeneficialSpells = 0x50,
    #[serde(rename = "YouFailToAffect_YouAreNotPK")]
    YouFailToAffectYouAreNotPK = 0x51,
    #[serde(rename = "YouFailToAffect_TheyAreNotPK")]
    YouFailToAffectTheyAreNotPK = 0x52,
    #[serde(rename = "YouFailToAffect_NotSamePKType")]
    YouFailToAffectNotSamePKType = 0x53,
    #[serde(rename = "YouFailToAffect_AcrossHouseBoundary")]
    YouFailToAffectAcrossHouseBoundary = 0x54,
    IsNotAcceptingGiftsRightNow = 0x3EF,
    IsAlreadyOneOfYourFollowers = 0x413,
    CannotHaveAnyMoreVassals = 0x416,
    DoesntKnowWhatToDoWithThat = 0x46A,
    #[serde(rename = "YouMustBeAboveLevel_ToBuyHouse")]
    YouMustBeAboveLevelToBuyHouse = 0x488,
    #[serde(rename = "YouMustBeAtOrBelowLevel_ToBuyHouse")]
    YouMustBeAtOrBelowLevelToBuyHouse = 0x489,
    #[serde(rename = "YouMustBeAboveAllegianceRank_ToBuyHouse")]
    YouMustBeAboveAllegianceRankToBuyHouse = 0x48B,
    #[serde(rename = "YouMustBeAtOrBelowAllegianceRank_ToBuyHouse")]
    YouMustBeAtOrBelowAllegianceRankToBuyHouse = 0x48C,
    #[serde(rename = "The_WasNotSuitableForSalvaging")]
    TheWasNotSuitableForSalvaging = 0x4BF,
    #[serde(rename = "The_ContainseTheWrongMaterial")]
    TheContainseTheWrongMaterial = 0x4C0,
    #[serde(rename = "YouMustBe_ToUseItemMagic")]
    YouMustBeToUseItemMagic = 0x4C6,
    #[serde(rename = "Your_IsTooLowToUseItemMagic")]
    YourIsTooLowToUseItemMagic = 0x4C9,
    #[serde(rename = "Only_MayUseItemMagic")]
    OnlyMayUseItemMagic = 0x4CA,
    #[serde(rename = "YouMustSpecialize_ToUseItemMagic")]
    YouMustSpecializeToUseItemMagic = 0x4CB,
    AiRefuseItemDuringEmote = 0x4CE,
    CannotAcceptStackedItems = 0x4CF,
    #[serde(rename = "Your_SkillMustBeTrained")]
    YourSkillMustBeTrained = 0x4D1,
    NotEnoughSkillCreditsToSpecialize = 0x4D2,
    TooMuchXPToRecoverFromSkill = 0x4D3,
    #[serde(rename = "Your_SkillIsAlreadyUntrained")]
    YourSkillIsAlreadyUntrained = 0x4D4,
    CannotLowerSkillWhileWieldingItem = 0x4D5,
    #[serde(rename = "YouHaveSucceededSpecializing_Skill")]
    YouHaveSucceededSpecializingSkill = 0x4D6,
    #[serde(rename = "YouHaveSucceededUnspecializing_Skill")]
    YouHaveSucceededUnspecializingSkill = 0x4D7,
    #[serde(rename = "YouHaveSucceededUntraining_Skill")]
    YouHaveSucceededUntrainingSkill = 0x4D8,
    #[serde(rename = "CannotUntrain_SkillButRecoveredXP")]
    CannotUntrainSkillButRecoveredXP = 0x4D9,
    TooManyCreditsInSpecializedSkills = 0x4DA,
    AttributeTransferFromTooLow = 0x4DE,
    AttributeTransferToTooHigh = 0x4DF,
    #[serde(rename = "ItemUnusableOnHook_CannotOpen")]
    ItemUnusableOnHookCannotOpen = 0x4E8,
    #[serde(rename = "ItemUnusableOnHook_CanOpen")]
    ItemUnusableOnHookCanOpen = 0x4E9,
    ItemOnlyUsableOnHook = 0x4EA,
    #[serde(rename = "FailsToAffectYou_TheyCannotAffectAnyone")]
    FailsToAffectYouTheyCannotAffectAnyone = 0x4F4,
    #[serde(rename = "FailsToAffectYou_YouCannotBeHarmed")]
    FailsToAffectYouYouCannotBeHarmed = 0x4F5,
    #[serde(rename = "FailsToAffectYou_TheyAreNotPK")]
    FailsToAffectYouTheyAreNotPK = 0x4F6,
    #[serde(rename = "FailsToAffectYou_YouAreNotPK")]
    FailsToAffectYouYouAreNotPK = 0x4F7,
    #[serde(rename = "FailsToAffectYou_NotSamePKType")]
    FailsToAffectYouNotSamePKType = 0x4F8,
    FailsToAffectYouAcrossHouseBoundary = 0x4F9,
    IsAnInvalidTarget = 0x4FA,
    #[serde(rename = "YouAreInvalidTargetForSpellOf_")]
    YouAreInvalidTargetForSpellOf = 0x4FB,
    IsAtFullHealth = 0x4FF,
    HasNoSpellTargets = 0x509,
    #[serde(rename = "YouHaveNoTargetsForSpellOf_")]
    YouHaveNoTargetsForSpellOf = 0x50A,
    IsNowOpenFellowship = 0x50B,
    IsNowClosedFellowship = 0x50C,
    IsNowLeaderOfFellowship = 0x50D,
    #[serde(rename = "YouHavePassedFellowshipLeadershipTo_")]
    YouHavePassedFellowshipLeadershipTo = 0x50E,
    #[serde(rename = "MaxNumberOf_Hooked")]
    MaxNumberOfHooked = 0x510,
    #[serde(rename = "MaxNumberOf_HookedUntilOneIsRemoved")]
    MaxNumberOfHookedUntilOneIsRemoved = 0x514,
    #[serde(rename = "NoLongerMaxNumberOf_Hooked")]
    NoLongerMaxNumberOfHooked = 0x515,
    IsNotCloseEnoughToYourLevel = 0x517,
    #[serde(rename = "LockedFellowshipCannotRecruit_")]
    LockedFellowshipCannotRecruit = 0x518,
    #[serde(rename = "YouHaveEnteredThe_Channel")]
    YouHaveEnteredTheChannel = 0x51B,
    #[serde(rename = "YouHaveLeftThe_Channel")]
    YouHaveLeftTheChannel = 0x51C,
    WillNotReceiveMessage = 0x51E,
    #[serde(rename = "MessageBlocked_")]
    MessageBlocked = 0x51F,
    HasBeenAddedToHearList = 0x521,
    HasBeenRemovedFromHearList = 0x522,
    #[serde(rename = "FailToRemove_FromLoudList")]
    FailToRemoveFromLoudList = 0x525,
    YouCannotOpenLockedFellowship = 0x528,
    #[serde(rename = "YouAreNowSnoopingOn_")]
    YouAreNowSnoopingOn = 0x52C,
    #[serde(rename = "YouAreNoLongerSnoopingOn_")]
    YouAreNoLongerSnoopingOn = 0x52D,
    #[serde(rename = "YouFailToSnoopOn_")]
    YouFailToSnoopOn = 0x52E,
    AttemptedToSnoopOnYou = 0x52F,
    IsAlreadyBeingSnoopedOn = 0x530,
    IsInLimbo = 0x531,
    YouHaveBeenBootedFromAllegianceChat = 0x533,
    HasBeenBootedFromAllegianceChat = 0x534,
    #[serde(rename = "AccountOf_IsAlreadyBannedFromAllegiance")]
    AccountOfIsAlreadyBannedFromAllegiance = 0x536,
    #[serde(rename = "AccountOf_IsNotBannedFromAllegiance")]
    AccountOfIsNotBannedFromAllegiance = 0x537,
    #[serde(rename = "AccountOf_WasNotUnbannedFromAllegiance")]
    AccountOfWasNotUnbannedFromAllegiance = 0x538,
    #[serde(rename = "AccountOf_IsBannedFromAllegiance")]
    AccountOfIsBannedFromAllegiance = 0x539,
    #[serde(rename = "AccountOf_IsUnbannedFromAllegiance")]
    AccountOfIsUnbannedFromAllegiance = 0x53A,
    ListOfBannedCharacters = 0x53B,
    IsBannedFromAllegiance = 0x53E,
    YouAreBannedFromAllegiance = 0x53F,
    IsNowAllegianceOfficer = 0x541,
    #[serde(rename = "ErrorSetting_AsAllegianceOfficer")]
    ErrorSettingAsAllegianceOfficer = 0x542,
    IsNoLongerAllegianceOfficer = 0x543,
    #[serde(rename = "ErrorRemoving_AsAllegianceOFficer")]
    ErrorRemovingAsAllegianceOFficer = 0x544,
    #[serde(rename = "YouMustWait_BeforeCommunicating")]
    YouMustWaitBeforeCommunicating = 0x547,
    YourAllegianceOfficerStatusChanged = 0x549,
    IsAlreadyAllegianceOfficerOfThatLevel = 0x54B,
    #[serde(rename = "The_IsCurrentlyInUse")]
    TheIsCurrentlyInUse = 0x54D,
    #[serde(rename = "YouAreNotListeningTo_Channel")]
    YouAreNotListeningToChannel = 0x551,
    AugmentationSkillNotTrained = 0x55A,
    YouSuccededAcquiringAugmentation = 0x55B,
    #[serde(rename = "YouSucceededRecoveringXPFromSkill_AugmentationNotUntrainable")]
    YouSucceededRecoveringXPFromSkillAugmentationNotUntrainable = 0x55C,
    AFK = 0x55E,
    IsAlreadyOnYourFriendsList = 0x562,
    YouMayOnlyChangeAllegianceNameOnceEvery24Hours = 0x56A,
    IsTheMonarchAndCannotBePromotedOrDemoted = 0x56D,
    #[serde(rename = "ThatLevelOfAllegianceOfficerIsNowKnownAs_")]
    ThatLevelOfAllegianceOfficerIsNowKnownAs = 0x56E,
    #[serde(rename = "YourAllegianceIsCurrently_")]
    YourAllegianceIsCurrently = 0x574,
    #[serde(rename = "YourAllegianceIsNow_")]
    YourAllegianceIsNow = 0x575,
    #[serde(rename = "YouCannotAcceptAllegiance_YourAllegianceIsLocked")]
    YouCannotAcceptAllegianceYourAllegianceIsLocked = 0x576,
    #[serde(rename = "YouCannotSwearAllegiance_AllegianceOf_IsLocked")]
    YouCannotSwearAllegianceAllegianceOfIsLocked = 0x577,
    #[serde(rename = "YouHavePreApproved_ToJoinAllegiance")]
    YouHavePreApprovedToJoinAllegiance = 0x578,
    IsAlreadyMemberOfYourAllegiance = 0x57A,
    HasBeenPreApprovedToJoinYourAllegiance = 0x57B,
    YourAllegianceChatPrivilegesRemoved = 0x57F,
    IsTemporarilyGaggedInAllegianceChat = 0x580,
    #[serde(rename = "YourAllegianceChatPrivilegesRestoredBy_")]
    YourAllegianceChatPrivilegesRestoredBy = 0x582,
    #[serde(rename = "YouRestoreAllegianceChatPrivilegesTo_")]
    YouRestoreAllegianceChatPrivilegesTo = 0x583,
    CowersFromYou = 0x58A,
}

impl crate::readers::ACDataType for WeenieErrorWithString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(WeenieErrorWithString::try_from(value)?)
    }
}

impl crate::writers::ACWritable for WeenieErrorWithString {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for WeenieErrorWithString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WeenieErrorWithString::IsTooBusyToAcceptGifts => "IsTooBusyToAcceptGifts",
            WeenieErrorWithString::CannotCarryAnymore => "CannotCarryAnymore",
            WeenieErrorWithString::YouFailToAffectYouCannotAffectAnyone => "YouFailToAffect_YouCannotAffectAnyone",
            WeenieErrorWithString::YouFailToAffectTheyCannotBeHarmed => "YouFailToAffect_TheyCannotBeHarmed",
            WeenieErrorWithString::YouFailToAffectWithBeneficialSpells => "YouFailToAffect_WithBeneficialSpells",
            WeenieErrorWithString::YouFailToAffectYouAreNotPK => "YouFailToAffect_YouAreNotPK",
            WeenieErrorWithString::YouFailToAffectTheyAreNotPK => "YouFailToAffect_TheyAreNotPK",
            WeenieErrorWithString::YouFailToAffectNotSamePKType => "YouFailToAffect_NotSamePKType",
            WeenieErrorWithString::YouFailToAffectAcrossHouseBoundary => "YouFailToAffect_AcrossHouseBoundary",
            WeenieErrorWithString::IsNotAcceptingGiftsRightNow => "IsNotAcceptingGiftsRightNow",
            WeenieErrorWithString::IsAlreadyOneOfYourFollowers => "IsAlreadyOneOfYourFollowers",
            WeenieErrorWithString::CannotHaveAnyMoreVassals => "CannotHaveAnyMoreVassals",
            WeenieErrorWithString::DoesntKnowWhatToDoWithThat => "DoesntKnowWhatToDoWithThat",
            WeenieErrorWithString::YouMustBeAboveLevelToBuyHouse => "YouMustBeAboveLevel_ToBuyHouse",
            WeenieErrorWithString::YouMustBeAtOrBelowLevelToBuyHouse => "YouMustBeAtOrBelowLevel_ToBuyHouse",
            WeenieErrorWithString::YouMustBeAboveAllegianceRankToBuyHouse => "YouMustBeAboveAllegianceRank_ToBuyHouse",
            WeenieErrorWithString::YouMustBeAtOrBelowAllegianceRankToBuyHouse => "YouMustBeAtOrBelowAllegianceRank_ToBuyHouse",
            WeenieErrorWithString::TheWasNotSuitableForSalvaging => "The_WasNotSuitableForSalvaging",
            WeenieErrorWithString::TheContainseTheWrongMaterial => "The_ContainseTheWrongMaterial",
            WeenieErrorWithString::YouMustBeToUseItemMagic => "YouMustBe_ToUseItemMagic",
            WeenieErrorWithString::YourIsTooLowToUseItemMagic => "Your_IsTooLowToUseItemMagic",
            WeenieErrorWithString::OnlyMayUseItemMagic => "Only_MayUseItemMagic",
            WeenieErrorWithString::YouMustSpecializeToUseItemMagic => "YouMustSpecialize_ToUseItemMagic",
            WeenieErrorWithString::AiRefuseItemDuringEmote => "AiRefuseItemDuringEmote",
            WeenieErrorWithString::CannotAcceptStackedItems => "CannotAcceptStackedItems",
            WeenieErrorWithString::YourSkillMustBeTrained => "Your_SkillMustBeTrained",
            WeenieErrorWithString::NotEnoughSkillCreditsToSpecialize => "NotEnoughSkillCreditsToSpecialize",
            WeenieErrorWithString::TooMuchXPToRecoverFromSkill => "TooMuchXPToRecoverFromSkill",
            WeenieErrorWithString::YourSkillIsAlreadyUntrained => "Your_SkillIsAlreadyUntrained",
            WeenieErrorWithString::CannotLowerSkillWhileWieldingItem => "CannotLowerSkillWhileWieldingItem",
            WeenieErrorWithString::YouHaveSucceededSpecializingSkill => "YouHaveSucceededSpecializing_Skill",
            WeenieErrorWithString::YouHaveSucceededUnspecializingSkill => "YouHaveSucceededUnspecializing_Skill",
            WeenieErrorWithString::YouHaveSucceededUntrainingSkill => "YouHaveSucceededUntraining_Skill",
            WeenieErrorWithString::CannotUntrainSkillButRecoveredXP => "CannotUntrain_SkillButRecoveredXP",
            WeenieErrorWithString::TooManyCreditsInSpecializedSkills => "TooManyCreditsInSpecializedSkills",
            WeenieErrorWithString::AttributeTransferFromTooLow => "AttributeTransferFromTooLow",
            WeenieErrorWithString::AttributeTransferToTooHigh => "AttributeTransferToTooHigh",
            WeenieErrorWithString::ItemUnusableOnHookCannotOpen => "ItemUnusableOnHook_CannotOpen",
            WeenieErrorWithString::ItemUnusableOnHookCanOpen => "ItemUnusableOnHook_CanOpen",
            WeenieErrorWithString::ItemOnlyUsableOnHook => "ItemOnlyUsableOnHook",
            WeenieErrorWithString::FailsToAffectYouTheyCannotAffectAnyone => "FailsToAffectYou_TheyCannotAffectAnyone",
            WeenieErrorWithString::FailsToAffectYouYouCannotBeHarmed => "FailsToAffectYou_YouCannotBeHarmed",
            WeenieErrorWithString::FailsToAffectYouTheyAreNotPK => "FailsToAffectYou_TheyAreNotPK",
            WeenieErrorWithString::FailsToAffectYouYouAreNotPK => "FailsToAffectYou_YouAreNotPK",
            WeenieErrorWithString::FailsToAffectYouNotSamePKType => "FailsToAffectYou_NotSamePKType",
            WeenieErrorWithString::FailsToAffectYouAcrossHouseBoundary => "FailsToAffectYouAcrossHouseBoundary",
            WeenieErrorWithString::IsAnInvalidTarget => "IsAnInvalidTarget",
            WeenieErrorWithString::YouAreInvalidTargetForSpellOf => "YouAreInvalidTargetForSpellOf_",
            WeenieErrorWithString::IsAtFullHealth => "IsAtFullHealth",
            WeenieErrorWithString::HasNoSpellTargets => "HasNoSpellTargets",
            WeenieErrorWithString::YouHaveNoTargetsForSpellOf => "YouHaveNoTargetsForSpellOf_",
            WeenieErrorWithString::IsNowOpenFellowship => "IsNowOpenFellowship",
            WeenieErrorWithString::IsNowClosedFellowship => "IsNowClosedFellowship",
            WeenieErrorWithString::IsNowLeaderOfFellowship => "IsNowLeaderOfFellowship",
            WeenieErrorWithString::YouHavePassedFellowshipLeadershipTo => "YouHavePassedFellowshipLeadershipTo_",
            WeenieErrorWithString::MaxNumberOfHooked => "MaxNumberOf_Hooked",
            WeenieErrorWithString::MaxNumberOfHookedUntilOneIsRemoved => "MaxNumberOf_HookedUntilOneIsRemoved",
            WeenieErrorWithString::NoLongerMaxNumberOfHooked => "NoLongerMaxNumberOf_Hooked",
            WeenieErrorWithString::IsNotCloseEnoughToYourLevel => "IsNotCloseEnoughToYourLevel",
            WeenieErrorWithString::LockedFellowshipCannotRecruit => "LockedFellowshipCannotRecruit_",
            WeenieErrorWithString::YouHaveEnteredTheChannel => "YouHaveEnteredThe_Channel",
            WeenieErrorWithString::YouHaveLeftTheChannel => "YouHaveLeftThe_Channel",
            WeenieErrorWithString::WillNotReceiveMessage => "WillNotReceiveMessage",
            WeenieErrorWithString::MessageBlocked => "MessageBlocked_",
            WeenieErrorWithString::HasBeenAddedToHearList => "HasBeenAddedToHearList",
            WeenieErrorWithString::HasBeenRemovedFromHearList => "HasBeenRemovedFromHearList",
            WeenieErrorWithString::FailToRemoveFromLoudList => "FailToRemove_FromLoudList",
            WeenieErrorWithString::YouCannotOpenLockedFellowship => "YouCannotOpenLockedFellowship",
            WeenieErrorWithString::YouAreNowSnoopingOn => "YouAreNowSnoopingOn_",
            WeenieErrorWithString::YouAreNoLongerSnoopingOn => "YouAreNoLongerSnoopingOn_",
            WeenieErrorWithString::YouFailToSnoopOn => "YouFailToSnoopOn_",
            WeenieErrorWithString::AttemptedToSnoopOnYou => "AttemptedToSnoopOnYou",
            WeenieErrorWithString::IsAlreadyBeingSnoopedOn => "IsAlreadyBeingSnoopedOn",
            WeenieErrorWithString::IsInLimbo => "IsInLimbo",
            WeenieErrorWithString::YouHaveBeenBootedFromAllegianceChat => "YouHaveBeenBootedFromAllegianceChat",
            WeenieErrorWithString::HasBeenBootedFromAllegianceChat => "HasBeenBootedFromAllegianceChat",
            WeenieErrorWithString::AccountOfIsAlreadyBannedFromAllegiance => "AccountOf_IsAlreadyBannedFromAllegiance",
            WeenieErrorWithString::AccountOfIsNotBannedFromAllegiance => "AccountOf_IsNotBannedFromAllegiance",
            WeenieErrorWithString::AccountOfWasNotUnbannedFromAllegiance => "AccountOf_WasNotUnbannedFromAllegiance",
            WeenieErrorWithString::AccountOfIsBannedFromAllegiance => "AccountOf_IsBannedFromAllegiance",
            WeenieErrorWithString::AccountOfIsUnbannedFromAllegiance => "AccountOf_IsUnbannedFromAllegiance",
            WeenieErrorWithString::ListOfBannedCharacters => "ListOfBannedCharacters",
            WeenieErrorWithString::IsBannedFromAllegiance => "IsBannedFromAllegiance",
            WeenieErrorWithString::YouAreBannedFromAllegiance => "YouAreBannedFromAllegiance",
            WeenieErrorWithString::IsNowAllegianceOfficer => "IsNowAllegianceOfficer",
            WeenieErrorWithString::ErrorSettingAsAllegianceOfficer => "ErrorSetting_AsAllegianceOfficer",
            WeenieErrorWithString::IsNoLongerAllegianceOfficer => "IsNoLongerAllegianceOfficer",
            WeenieErrorWithString::ErrorRemovingAsAllegianceOFficer => "ErrorRemoving_AsAllegianceOFficer",
            WeenieErrorWithString::YouMustWaitBeforeCommunicating => "YouMustWait_BeforeCommunicating",
            WeenieErrorWithString::YourAllegianceOfficerStatusChanged => "YourAllegianceOfficerStatusChanged",
            WeenieErrorWithString::IsAlreadyAllegianceOfficerOfThatLevel => "IsAlreadyAllegianceOfficerOfThatLevel",
            WeenieErrorWithString::TheIsCurrentlyInUse => "The_IsCurrentlyInUse",
            WeenieErrorWithString::YouAreNotListeningToChannel => "YouAreNotListeningTo_Channel",
            WeenieErrorWithString::AugmentationSkillNotTrained => "AugmentationSkillNotTrained",
            WeenieErrorWithString::YouSuccededAcquiringAugmentation => "YouSuccededAcquiringAugmentation",
            WeenieErrorWithString::YouSucceededRecoveringXPFromSkillAugmentationNotUntrainable => "YouSucceededRecoveringXPFromSkill_AugmentationNotUntrainable",
            WeenieErrorWithString::AFK => "AFK",
            WeenieErrorWithString::IsAlreadyOnYourFriendsList => "IsAlreadyOnYourFriendsList",
            WeenieErrorWithString::YouMayOnlyChangeAllegianceNameOnceEvery24Hours => "YouMayOnlyChangeAllegianceNameOnceEvery24Hours",
            WeenieErrorWithString::IsTheMonarchAndCannotBePromotedOrDemoted => "IsTheMonarchAndCannotBePromotedOrDemoted",
            WeenieErrorWithString::ThatLevelOfAllegianceOfficerIsNowKnownAs => "ThatLevelOfAllegianceOfficerIsNowKnownAs_",
            WeenieErrorWithString::YourAllegianceIsCurrently => "YourAllegianceIsCurrently_",
            WeenieErrorWithString::YourAllegianceIsNow => "YourAllegianceIsNow_",
            WeenieErrorWithString::YouCannotAcceptAllegianceYourAllegianceIsLocked => "YouCannotAcceptAllegiance_YourAllegianceIsLocked",
            WeenieErrorWithString::YouCannotSwearAllegianceAllegianceOfIsLocked => "YouCannotSwearAllegiance_AllegianceOf_IsLocked",
            WeenieErrorWithString::YouHavePreApprovedToJoinAllegiance => "YouHavePreApproved_ToJoinAllegiance",
            WeenieErrorWithString::IsAlreadyMemberOfYourAllegiance => "IsAlreadyMemberOfYourAllegiance",
            WeenieErrorWithString::HasBeenPreApprovedToJoinYourAllegiance => "HasBeenPreApprovedToJoinYourAllegiance",
            WeenieErrorWithString::YourAllegianceChatPrivilegesRemoved => "YourAllegianceChatPrivilegesRemoved",
            WeenieErrorWithString::IsTemporarilyGaggedInAllegianceChat => "IsTemporarilyGaggedInAllegianceChat",
            WeenieErrorWithString::YourAllegianceChatPrivilegesRestoredBy => "YourAllegianceChatPrivilegesRestoredBy_",
            WeenieErrorWithString::YouRestoreAllegianceChatPrivilegesTo => "YouRestoreAllegianceChatPrivilegesTo_",
            WeenieErrorWithString::CowersFromYou => "CowersFromYou",
        };
        write!(f, "{}", s)
    }
}

/// Set of predefined error messages
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum WeenieError {
    None = 0x0,
    NoMem = 0x1,
    BadParam = 0x2,
    DivZero = 0x3,
    SegV = 0x4,
    Unimplemented = 0x5,
    UnknownMessageType = 0x6,
    NoAnimationTable = 0x7,
    NoPhysicsObject = 0x8,
    NoBookieObject = 0x9,
    NoWslObject = 0xA,
    NoMotionInterpreter = 0xB,
    UnhandledSwitch = 0xC,
    DefaultConstructorCalled = 0xD,
    InvalidCombatManeuver = 0xE,
    BadCast = 0xF,
    MissingQuality = 0x10,
    MissingDatabaseObject = 0x12,
    NoCallbackSet = 0x13,
    CorruptQuality = 0x14,
    BadContext = 0x15,
    NoEphseqManager = 0x16,
    BadMovementEvent = 0x17,
    CannotCreateNewObject = 0x18,
    NoControllerObject = 0x19,
    CannotSendEvent = 0x1A,
    PhysicsCantTransition = 0x1B,
    PhysicsMaxDistanceExceeded = 0x1C,
    YoureTooBusy = 0x1D,
    CannotSendMessage = 0x1F,
    IllegalInventoryTransaction = 0x20,
    ExternalWeenieObject = 0x21,
    InternalWeenieObject = 0x22,
    MotionFailure = 0x23,
    YouCantJumpWhileInTheAir = 0x24,
    InqCylSphereFailure = 0x25,
    ThatIsNotAValidCommand = 0x26,
    CarryingItem = 0x27,
    Frozen = 0x28,
    Stuck = 0x29,
    YouAreTooEncumbered = 0x2A,
    BadContain = 0x2C,
    BadParent = 0x2D,
    BadDrop = 0x2E,
    BadRelease = 0x2F,
    MsgBadMsg = 0x30,
    MsgUnpackFailed = 0x31,
    MsgNoMsg = 0x32,
    MsgUnderflow = 0x33,
    MsgOverflow = 0x34,
    MsgCallbackFailed = 0x35,
    ActionCancelled = 0x36,
    ObjectGone = 0x37,
    NoObject = 0x38,
    CantGetThere = 0x39,
    Dead = 0x3A,
    ILeftTheWorld = 0x3B,
    ITeleported = 0x3C,
    YouChargedTooFar = 0x3D,
    YouAreTooTiredToDoThat = 0x3E,
    CantCrouchInCombat = 0x3F,
    CantSitInCombat = 0x40,
    CantLieDownInCombat = 0x41,
    CantChatEmoteInCombat = 0x42,
    NoMtableData = 0x43,
    CantChatEmoteNotStanding = 0x44,
    TooManyActions = 0x45,
    Hidden = 0x46,
    GeneralMovementFailure = 0x47,
    YouCantJumpFromThisPosition = 0x48,
    CantJumpLoadedDown = 0x49,
    YouKilledYourself = 0x4A,
    MsgResponseFailure = 0x4B,
    ObjectIsStatic = 0x4C,
    InvalidPkStatus = 0x4D,
    InvalidXpAmount = 0x3E9,
    InvalidPpCalculation = 0x3EA,
    InvalidCpCalculation = 0x3EB,
    UnhandledStatAnswer = 0x3EC,
    HeartAttack = 0x3ED,
    TheContainerIsClosed = 0x3EE,
    InvalidInventoryLocation = 0x3F0,
    ChangeCombatModeFailure = 0x3F1,
    FullInventoryLocation = 0x3F2,
    ConflictingInventoryLocation = 0x3F3,
    ItemNotPending = 0x3F4,
    BeWieldedFailure = 0x3F5,
    BeDroppedFailure = 0x3F6,
    YouAreTooFatiguedToAttack = 0x3F7,
    YouAreOutOfAmmunition = 0x3F8,
    YourAttackMisfired = 0x3F9,
    YouveAttemptedAnImpossibleSpellPath = 0x3FA,
    MagicIncompleteAnimList = 0x3FB,
    MagicInvalidSpellType = 0x3FC,
    MagicInqPositionAndVelocityFailure = 0x3FD,
    YouDontKnowThatSpell = 0x3FE,
    IncorrectTargetType = 0x3FF,
    YouDontHaveAllTheComponents = 0x400,
    YouDontHaveEnoughManaToCast = 0x401,
    YourSpellFizzled = 0x402,
    YourSpellTargetIsMissing = 0x403,
    YourProjectileSpellMislaunched = 0x404,
    MagicSpellbookAddSpellFailure = 0x405,
    MagicTargetOutOfRange = 0x406,
    YourSpellCannotBeCastOutside = 0x407,
    YourSpellCannotBeCastInside = 0x408,
    MagicGeneralFailure = 0x409,
    YouAreUnpreparedToCastASpell = 0x40A,
    YouveAlreadySwornAllegiance = 0x40B,
    CantSwearAllegianceInsufficientXp = 0x40C,
    AllegianceIgnoringRequests = 0x40D,
    AllegianceSquelched = 0x40E,
    AllegianceMaxDistanceExceeded = 0x40F,
    AllegianceIllegalLevel = 0x410,
    AllegianceBadCreation = 0x411,
    AllegiancePatronBusy = 0x412,
    YouAreNotInAllegiance = 0x414,
    AllegianceRemoveHierarchyFailure = 0x415,
    FellowshipIgnoringRequests = 0x417,
    FellowshipSquelched = 0x418,
    FellowshipMaxDistanceExceeded = 0x419,
    FellowshipMember = 0x41A,
    FellowshipIllegalLevel = 0x41B,
    FellowshipRecruitBusy = 0x41C,
    YouMustBeLeaderOfFellowship = 0x41D,
    YourFellowshipIsFull = 0x41E,
    FellowshipNameIsNotPermitted = 0x41F,
    LevelTooLow = 0x420,
    LevelTooHigh = 0x421,
    ThatChannelDoesntExist = 0x422,
    YouCantUseThatChannel = 0x423,
    YouAreAlreadyOnThatChannel = 0x424,
    YouAreNotOnThatChannel = 0x425,
    AttunedItem = 0x426,
    YouCannotMergeDifferentStacks = 0x427,
    YouCannotMergeEnchantedItems = 0x428,
    YouMustControlAtLeastOneStack = 0x429,
    CurrentlyAttacking = 0x42A,
    MissileAttackNotOk = 0x42B,
    TargetNotAcquired = 0x42C,
    ImpossibleShot = 0x42D,
    BadWeaponSkill = 0x42E,
    UnwieldFailure = 0x42F,
    LaunchFailure = 0x430,
    ReloadFailure = 0x431,
    UnableToMakeCraftReq = 0x432,
    CraftAnimationFailed = 0x433,
    YouCantCraftWithThatNumberOfItems = 0x434,
    CraftGeneralErrorUiMsg = 0x435,
    CraftGeneralErrorNoUiMsg = 0x436,
    YouDoNotPassCraftingRequirements = 0x437,
    YouDoNotHaveAllTheNecessaryItems = 0x438,
    NotAllTheItemsAreAvailable = 0x439,
    YouMustBeInPeaceModeToTrade = 0x43A,
    YouAreNotTrainedInThatTradeSkill = 0x43B,
    YourHandsMustBeFree = 0x43C,
    YouCannotLinkToThatPortal = 0x43D,
    YouHaveSolvedThisQuestTooRecently = 0x43E,
    YouHaveSolvedThisQuestTooManyTimes = 0x43F,
    QuestUnknown = 0x440,
    QuestTableCorrupt = 0x441,
    QuestBad = 0x442,
    QuestDuplicate = 0x443,
    QuestUnsolved = 0x444,
    ItemRequiresQuestToBePickedUp = 0x445,
    QuestSolvedTooLongAgo = 0x446,
    TradeIgnoringRequests = 0x44C,
    TradeSquelched = 0x44D,
    TradeMaxDistanceExceeded = 0x44E,
    TradeAlreadyTrading = 0x44F,
    TradeBusy = 0x450,
    TradeClosed = 0x451,
    TradeExpired = 0x452,
    TradeItemBeingTraded = 0x453,
    TradeNonEmptyContainer = 0x454,
    TradeNonCombatMode = 0x455,
    TradeIncomplete = 0x456,
    TradeStampMismatch = 0x457,
    TradeUnopened = 0x458,
    TradeEmpty = 0x459,
    TradeAlreadyAccepted = 0x45A,
    TradeOutOfSync = 0x45B,
    PKsMayNotUsePortal = 0x45C,
    NonPKsMayNotUsePortal = 0x45D,
    HouseAbandoned = 0x45E,
    HouseEvicted = 0x45F,
    HouseAlreadyOwned = 0x460,
    HouseBuyFailed = 0x461,
    HouseRentFailed = 0x462,
    Hooked = 0x463,
    MagicInvalidPosition = 0x465,
    YouMustHaveDarkMajestyToUsePortal = 0x466,
    InvalidAmmoType = 0x467,
    SkillTooLow = 0x468,
    YouHaveUsedAllTheHooks = 0x469,
    TradeAiDoesntWant = 0x46A,
    HookHouseNotOwned = 0x46B,
    YouMustCompleteQuestToUsePortal = 0x474,
    HouseNoAllegiance = 0x47E,
    YouMustOwnHouseToUseCommand = 0x47F,
    YourMonarchDoesNotOwnAMansionOrVilla = 0x480,
    YourMonarchsHouseIsNotAMansionOrVilla = 0x481,
    YourMonarchHasClosedTheMansion = 0x482,
    YouMustBeMonarchToPurchaseDwelling = 0x48A,
    AllegianceTimeout = 0x48D,
    YourOfferOfAllegianceWasIgnored = 0x48E,
    ConfirmationInProgress = 0x48F,
    YouMustBeAMonarchToUseCommand = 0x490,
    YouMustSpecifyCharacterToBoot = 0x491,
    YouCantBootYourself = 0x492,
    ThatCharacterDoesNotExist = 0x493,
    ThatPersonIsNotInYourAllegiance = 0x494,
    CantBreakFromPatronNotInAllegiance = 0x495,
    YourAllegianceHasBeenDissolved = 0x496,
    YourPatronsAllegianceHasBeenBroken = 0x497,
    YouHaveMovedTooFar = 0x498,
    TeleToInvalidPosition = 0x499,
    MustHaveDarkMajestyToUse = 0x49A,
    YouFailToLinkWithLifestone = 0x49B,
    YouWanderedTooFarToLinkWithLifestone = 0x49C,
    YouSuccessfullyLinkWithLifestone = 0x49D,
    YouMustLinkToLifestoneToRecall = 0x49E,
    YouFailToRecallToLifestone = 0x49F,
    YouFailToLinkWithPortal = 0x4A0,
    YouSuccessfullyLinkWithPortal = 0x4A1,
    YouFailToRecallToPortal = 0x4A2,
    YouMustLinkToPortalToRecall = 0x4A3,
    YouFailToSummonPortal = 0x4A4,
    YouMustLinkToPortalToSummonIt = 0x4A5,
    YouFailToTeleport = 0x4A6,
    YouHaveBeenTeleportedTooRecently = 0x4A7,
    YouMustBeAnAdvocateToUsePortal = 0x4A8,
    PortalAisNotAllowed = 0x4A9,
    PlayersMayNotUsePortal = 0x4AA,
    YouAreNotPowerfulEnoughToUsePortal = 0x4AB,
    YouAreTooPowerfulToUsePortal = 0x4AC,
    YouCannotRecallPortal = 0x4AD,
    YouCannotSummonPortal = 0x4AE,
    LockAlreadyUnlocked = 0x4AF,
    YouCannotLockOrUnlockThat = 0x4B0,
    YouCannotLockWhatIsOpen = 0x4B1,
    KeyDoesntFitThisLock = 0x4B2,
    LockUsedTooRecently = 0x4B3,
    YouArentTrainedInLockpicking = 0x4B4,
    AllegianceInfoEmptyName = 0x4B5,
    AllegianceInfoSelf = 0x4B6,
    AllegianceInfoTooRecent = 0x4B7,
    AbuseNoSuchCharacter = 0x4B8,
    AbuseReportedSelf = 0x4B9,
    AbuseComplaintHandled = 0x4BA,
    YouDoNotOwnThatSalvageTool = 0x4BD,
    YouDoNotOwnThatItem = 0x4BE,
    MaterialCannotBeCreated = 0x4C1,
    ItemsAttemptingToSalvageIsInvalid = 0x4C2,
    YouCannotSalvageItemsInTrading = 0x4C3,
    YouMustBeHouseGuestToUsePortal = 0x4C4,
    YourAllegianceRankIsTooLowToUseMagic = 0x4C5,
    YourArcaneLoreIsTooLowToUseMagic = 0x4C7,
    ItemDoesntHaveEnoughMana = 0x4C8,
    YouHaveBeenInPKBattleTooRecently = 0x4CC,
    TradeAiRefuseEmote = 0x4CD,
    YouFailToAlterSkill = 0x4D0,
    FellowshipDeclined = 0x4DB,
    FellowshipTimeout = 0x4DC,
    YouHaveFailedToAlterAttributes = 0x4DD,
    CannotTransferAttributesWhileWieldingItem = 0x4E0,
    YouHaveSucceededTransferringAttributes = 0x4E1,
    HookIsDuplicated = 0x4E2,
    ItemIsWrongTypeForHook = 0x4E3,
    HousingChestIsDuplicated = 0x4E4,
    HookWillBeDeleted = 0x4E5,
    HousingChestWillBeDeleted = 0x4E6,
    CannotSwearAllegianceWhileOwningMansion = 0x4E7,
    YouCantDoThatWhileInTheAir = 0x4EB,
    CannotChangePKStatusWhileRecovering = 0x4EC,
    AdvocatesCannotChangePKStatus = 0x4ED,
    LevelTooLowToChangePKStatusWithObject = 0x4EE,
    LevelTooHighToChangePKStatusWithObject = 0x4EF,
    YouFeelAHarshDissonance = 0x4F0,
    YouArePKAgain = 0x4F1,
    YouAreTemporarilyNoLongerPK = 0x4F2,
    PKLiteMayNotUsePortal = 0x4F3,
    YouArentTrainedInHealing = 0x4FC,
    YouDontOwnThatHealingKit = 0x4FD,
    YouCantHealThat = 0x4FE,
    YouArentReadyToHeal = 0x500,
    YouCanOnlyHealPlayers = 0x501,
    LifestoneMagicProtectsYou = 0x502,
    PortalEnergyProtectsYou = 0x503,
    YouAreNonPKAgain = 0x504,
    YoureTooCloseToYourSanctuary = 0x505,
    CantDoThatTradeInProgress = 0x506,
    OnlyNonPKsMayEnterPKLite = 0x507,
    YouAreNowPKLite = 0x508,
    YouDoNotBelongToAFellowship = 0x50F,
    UsingMaxHooksSilent = 0x511,
    YouAreNowUsingMaxHooks = 0x512,
    YouAreNoLongerUsingMaxHooks = 0x513,
    YouAreNotPermittedToUseThatHook = 0x516,
    LockedFellowshipCannotRecruitYou = 0x519,
    ActivationNotAllowedNotOwner = 0x51A,
    TurbineChatIsEnabled = 0x51D,
    YouCannotAddPeopleToHearList = 0x520,
    #[serde(rename = "YouAreNowDeafTo_Screams")]
    YouAreNowDeafToScreams = 0x523,
    YouCanHearAllPlayersOnceAgain = 0x524,
    YouChickenOut = 0x526,
    YouCanPossiblySucceed = 0x527,
    FellowshipIsLocked = 0x528,
    TradeComplete = 0x529,
    NotASalvageTool = 0x52A,
    CharacterNotAvailable = 0x52B,
    YouMustWaitToPurchaseHouse = 0x532,
    YouDoNotHaveAuthorityInAllegiance = 0x535,
    YouHaveMaxAccountsBanned = 0x540,
    YouHaveMaxAllegianceOfficers = 0x545,
    YourAllegianceOfficersHaveBeenCleared = 0x546,
    YouCannotJoinChannelsWhileGagged = 0x548,
    YouAreNoLongerAllegianceOfficer = 0x54A,
    YourAllegianceDoesNotHaveHometown = 0x54C,
    #[serde(rename = "HookItemNotUsable_CannotOpen")]
    HookItemNotUsableCannotOpen = 0x54E,
    #[serde(rename = "HookItemNotUsable_CanOpen")]
    HookItemNotUsableCanOpen = 0x54F,
    MissileOutOfRange = 0x550,
    MustPurchaseThroneOfDestinyToUseFunction = 0x552,
    MustPurchaseThroneOfDestinyToUseItem = 0x553,
    MustPurchaseThroneOfDestinyToUsePortal = 0x554,
    MustPurchaseThroneOfDestinyToAccessQuest = 0x555,
    YouFailedToCompleteAugmentation = 0x556,
    AugmentationUsedTooManyTimes = 0x557,
    AugmentationTypeUsedTooManyTimes = 0x558,
    AugmentationNotEnoughExperience = 0x559,
    ExitTrainingAcademyToUseCommand = 0x55D,
    OnlyPKsMayUseCommand = 0x55F,
    OnlyPKLiteMayUseCommand = 0x560,
    MaxFriendsExceeded = 0x561,
    ThatCharacterNotOnYourFriendsList = 0x563,
    OnlyHouseOwnerCanUseCommand = 0x564,
    InvalidAllegianceNameCantBeEmpty = 0x565,
    InvalidAllegianceNameTooLong = 0x566,
    InvalidAllegianceNameBadCharacters = 0x567,
    InvalidAllegianceNameInappropriate = 0x568,
    InvalidAllegianceNameAlreadyInUse = 0x569,
    AllegianceNameCleared = 0x56B,
    InvalidAllegianceNameSameName = 0x56C,
    InvalidOfficerLevel = 0x56F,
    AllegianceOfficerTitleIsNotAppropriate = 0x570,
    AllegianceNameIsTooLong = 0x571,
    AllegianceOfficerTitlesCleared = 0x572,
    AllegianceTitleHasIllegalChars = 0x573,
    YouHaveNotPreApprovedVassals = 0x579,
    YouHaveClearedPreApprovedVassal = 0x57C,
    CharIsAlreadyGagged = 0x57D,
    CharIsNotCurrentlyGagged = 0x57E,
    YourAllegianceChatPrivilegesRestored = 0x581,
    TooManyUniqueItems = 0x584,
    HeritageRequiresSpecificArmor = 0x585,
    ArmorRequiresSpecificHeritage = 0x586,
    OlthoiCannotInteractWithThat = 0x587,
    OlthoiCannotUseLifestones = 0x588,
    OlthoiVendorLooksInHorror = 0x589,
    OlthoiCannotJoinFellowship = 0x58B,
    OlthoiCannotJoinAllegiance = 0x58C,
    YouCannotUseThatItem = 0x58D,
    ThisPersonWillNotInteractWithYou = 0x58E,
    OnlyOlthoiMayUsePortal = 0x58F,
    OlthoiMayNotUsePortal = 0x590,
    YouMayNotUsePortalWithVitae = 0x591,
    YouMustBeTwoWeeksOldToUsePortal = 0x592,
    OlthoiCanOnlyRecallToLifestone = 0x593,
    ContractError = 0x594,
}

impl crate::readers::ACDataType for WeenieError {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(WeenieError::try_from(value)?)
    }
}

impl crate::writers::ACWritable for WeenieError {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for WeenieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WeenieError::None => "None",
            WeenieError::NoMem => "NoMem",
            WeenieError::BadParam => "BadParam",
            WeenieError::DivZero => "DivZero",
            WeenieError::SegV => "SegV",
            WeenieError::Unimplemented => "Unimplemented",
            WeenieError::UnknownMessageType => "UnknownMessageType",
            WeenieError::NoAnimationTable => "NoAnimationTable",
            WeenieError::NoPhysicsObject => "NoPhysicsObject",
            WeenieError::NoBookieObject => "NoBookieObject",
            WeenieError::NoWslObject => "NoWslObject",
            WeenieError::NoMotionInterpreter => "NoMotionInterpreter",
            WeenieError::UnhandledSwitch => "UnhandledSwitch",
            WeenieError::DefaultConstructorCalled => "DefaultConstructorCalled",
            WeenieError::InvalidCombatManeuver => "InvalidCombatManeuver",
            WeenieError::BadCast => "BadCast",
            WeenieError::MissingQuality => "MissingQuality",
            WeenieError::MissingDatabaseObject => "MissingDatabaseObject",
            WeenieError::NoCallbackSet => "NoCallbackSet",
            WeenieError::CorruptQuality => "CorruptQuality",
            WeenieError::BadContext => "BadContext",
            WeenieError::NoEphseqManager => "NoEphseqManager",
            WeenieError::BadMovementEvent => "BadMovementEvent",
            WeenieError::CannotCreateNewObject => "CannotCreateNewObject",
            WeenieError::NoControllerObject => "NoControllerObject",
            WeenieError::CannotSendEvent => "CannotSendEvent",
            WeenieError::PhysicsCantTransition => "PhysicsCantTransition",
            WeenieError::PhysicsMaxDistanceExceeded => "PhysicsMaxDistanceExceeded",
            WeenieError::YoureTooBusy => "YoureTooBusy",
            WeenieError::CannotSendMessage => "CannotSendMessage",
            WeenieError::IllegalInventoryTransaction => "IllegalInventoryTransaction",
            WeenieError::ExternalWeenieObject => "ExternalWeenieObject",
            WeenieError::InternalWeenieObject => "InternalWeenieObject",
            WeenieError::MotionFailure => "MotionFailure",
            WeenieError::YouCantJumpWhileInTheAir => "YouCantJumpWhileInTheAir",
            WeenieError::InqCylSphereFailure => "InqCylSphereFailure",
            WeenieError::ThatIsNotAValidCommand => "ThatIsNotAValidCommand",
            WeenieError::CarryingItem => "CarryingItem",
            WeenieError::Frozen => "Frozen",
            WeenieError::Stuck => "Stuck",
            WeenieError::YouAreTooEncumbered => "YouAreTooEncumbered",
            WeenieError::BadContain => "BadContain",
            WeenieError::BadParent => "BadParent",
            WeenieError::BadDrop => "BadDrop",
            WeenieError::BadRelease => "BadRelease",
            WeenieError::MsgBadMsg => "MsgBadMsg",
            WeenieError::MsgUnpackFailed => "MsgUnpackFailed",
            WeenieError::MsgNoMsg => "MsgNoMsg",
            WeenieError::MsgUnderflow => "MsgUnderflow",
            WeenieError::MsgOverflow => "MsgOverflow",
            WeenieError::MsgCallbackFailed => "MsgCallbackFailed",
            WeenieError::ActionCancelled => "ActionCancelled",
            WeenieError::ObjectGone => "ObjectGone",
            WeenieError::NoObject => "NoObject",
            WeenieError::CantGetThere => "CantGetThere",
            WeenieError::Dead => "Dead",
            WeenieError::ILeftTheWorld => "ILeftTheWorld",
            WeenieError::ITeleported => "ITeleported",
            WeenieError::YouChargedTooFar => "YouChargedTooFar",
            WeenieError::YouAreTooTiredToDoThat => "YouAreTooTiredToDoThat",
            WeenieError::CantCrouchInCombat => "CantCrouchInCombat",
            WeenieError::CantSitInCombat => "CantSitInCombat",
            WeenieError::CantLieDownInCombat => "CantLieDownInCombat",
            WeenieError::CantChatEmoteInCombat => "CantChatEmoteInCombat",
            WeenieError::NoMtableData => "NoMtableData",
            WeenieError::CantChatEmoteNotStanding => "CantChatEmoteNotStanding",
            WeenieError::TooManyActions => "TooManyActions",
            WeenieError::Hidden => "Hidden",
            WeenieError::GeneralMovementFailure => "GeneralMovementFailure",
            WeenieError::YouCantJumpFromThisPosition => "YouCantJumpFromThisPosition",
            WeenieError::CantJumpLoadedDown => "CantJumpLoadedDown",
            WeenieError::YouKilledYourself => "YouKilledYourself",
            WeenieError::MsgResponseFailure => "MsgResponseFailure",
            WeenieError::ObjectIsStatic => "ObjectIsStatic",
            WeenieError::InvalidPkStatus => "InvalidPkStatus",
            WeenieError::InvalidXpAmount => "InvalidXpAmount",
            WeenieError::InvalidPpCalculation => "InvalidPpCalculation",
            WeenieError::InvalidCpCalculation => "InvalidCpCalculation",
            WeenieError::UnhandledStatAnswer => "UnhandledStatAnswer",
            WeenieError::HeartAttack => "HeartAttack",
            WeenieError::TheContainerIsClosed => "TheContainerIsClosed",
            WeenieError::InvalidInventoryLocation => "InvalidInventoryLocation",
            WeenieError::ChangeCombatModeFailure => "ChangeCombatModeFailure",
            WeenieError::FullInventoryLocation => "FullInventoryLocation",
            WeenieError::ConflictingInventoryLocation => "ConflictingInventoryLocation",
            WeenieError::ItemNotPending => "ItemNotPending",
            WeenieError::BeWieldedFailure => "BeWieldedFailure",
            WeenieError::BeDroppedFailure => "BeDroppedFailure",
            WeenieError::YouAreTooFatiguedToAttack => "YouAreTooFatiguedToAttack",
            WeenieError::YouAreOutOfAmmunition => "YouAreOutOfAmmunition",
            WeenieError::YourAttackMisfired => "YourAttackMisfired",
            WeenieError::YouveAttemptedAnImpossibleSpellPath => "YouveAttemptedAnImpossibleSpellPath",
            WeenieError::MagicIncompleteAnimList => "MagicIncompleteAnimList",
            WeenieError::MagicInvalidSpellType => "MagicInvalidSpellType",
            WeenieError::MagicInqPositionAndVelocityFailure => "MagicInqPositionAndVelocityFailure",
            WeenieError::YouDontKnowThatSpell => "YouDontKnowThatSpell",
            WeenieError::IncorrectTargetType => "IncorrectTargetType",
            WeenieError::YouDontHaveAllTheComponents => "YouDontHaveAllTheComponents",
            WeenieError::YouDontHaveEnoughManaToCast => "YouDontHaveEnoughManaToCast",
            WeenieError::YourSpellFizzled => "YourSpellFizzled",
            WeenieError::YourSpellTargetIsMissing => "YourSpellTargetIsMissing",
            WeenieError::YourProjectileSpellMislaunched => "YourProjectileSpellMislaunched",
            WeenieError::MagicSpellbookAddSpellFailure => "MagicSpellbookAddSpellFailure",
            WeenieError::MagicTargetOutOfRange => "MagicTargetOutOfRange",
            WeenieError::YourSpellCannotBeCastOutside => "YourSpellCannotBeCastOutside",
            WeenieError::YourSpellCannotBeCastInside => "YourSpellCannotBeCastInside",
            WeenieError::MagicGeneralFailure => "MagicGeneralFailure",
            WeenieError::YouAreUnpreparedToCastASpell => "YouAreUnpreparedToCastASpell",
            WeenieError::YouveAlreadySwornAllegiance => "YouveAlreadySwornAllegiance",
            WeenieError::CantSwearAllegianceInsufficientXp => "CantSwearAllegianceInsufficientXp",
            WeenieError::AllegianceIgnoringRequests => "AllegianceIgnoringRequests",
            WeenieError::AllegianceSquelched => "AllegianceSquelched",
            WeenieError::AllegianceMaxDistanceExceeded => "AllegianceMaxDistanceExceeded",
            WeenieError::AllegianceIllegalLevel => "AllegianceIllegalLevel",
            WeenieError::AllegianceBadCreation => "AllegianceBadCreation",
            WeenieError::AllegiancePatronBusy => "AllegiancePatronBusy",
            WeenieError::YouAreNotInAllegiance => "YouAreNotInAllegiance",
            WeenieError::AllegianceRemoveHierarchyFailure => "AllegianceRemoveHierarchyFailure",
            WeenieError::FellowshipIgnoringRequests => "FellowshipIgnoringRequests",
            WeenieError::FellowshipSquelched => "FellowshipSquelched",
            WeenieError::FellowshipMaxDistanceExceeded => "FellowshipMaxDistanceExceeded",
            WeenieError::FellowshipMember => "FellowshipMember",
            WeenieError::FellowshipIllegalLevel => "FellowshipIllegalLevel",
            WeenieError::FellowshipRecruitBusy => "FellowshipRecruitBusy",
            WeenieError::YouMustBeLeaderOfFellowship => "YouMustBeLeaderOfFellowship",
            WeenieError::YourFellowshipIsFull => "YourFellowshipIsFull",
            WeenieError::FellowshipNameIsNotPermitted => "FellowshipNameIsNotPermitted",
            WeenieError::LevelTooLow => "LevelTooLow",
            WeenieError::LevelTooHigh => "LevelTooHigh",
            WeenieError::ThatChannelDoesntExist => "ThatChannelDoesntExist",
            WeenieError::YouCantUseThatChannel => "YouCantUseThatChannel",
            WeenieError::YouAreAlreadyOnThatChannel => "YouAreAlreadyOnThatChannel",
            WeenieError::YouAreNotOnThatChannel => "YouAreNotOnThatChannel",
            WeenieError::AttunedItem => "AttunedItem",
            WeenieError::YouCannotMergeDifferentStacks => "YouCannotMergeDifferentStacks",
            WeenieError::YouCannotMergeEnchantedItems => "YouCannotMergeEnchantedItems",
            WeenieError::YouMustControlAtLeastOneStack => "YouMustControlAtLeastOneStack",
            WeenieError::CurrentlyAttacking => "CurrentlyAttacking",
            WeenieError::MissileAttackNotOk => "MissileAttackNotOk",
            WeenieError::TargetNotAcquired => "TargetNotAcquired",
            WeenieError::ImpossibleShot => "ImpossibleShot",
            WeenieError::BadWeaponSkill => "BadWeaponSkill",
            WeenieError::UnwieldFailure => "UnwieldFailure",
            WeenieError::LaunchFailure => "LaunchFailure",
            WeenieError::ReloadFailure => "ReloadFailure",
            WeenieError::UnableToMakeCraftReq => "UnableToMakeCraftReq",
            WeenieError::CraftAnimationFailed => "CraftAnimationFailed",
            WeenieError::YouCantCraftWithThatNumberOfItems => "YouCantCraftWithThatNumberOfItems",
            WeenieError::CraftGeneralErrorUiMsg => "CraftGeneralErrorUiMsg",
            WeenieError::CraftGeneralErrorNoUiMsg => "CraftGeneralErrorNoUiMsg",
            WeenieError::YouDoNotPassCraftingRequirements => "YouDoNotPassCraftingRequirements",
            WeenieError::YouDoNotHaveAllTheNecessaryItems => "YouDoNotHaveAllTheNecessaryItems",
            WeenieError::NotAllTheItemsAreAvailable => "NotAllTheItemsAreAvailable",
            WeenieError::YouMustBeInPeaceModeToTrade => "YouMustBeInPeaceModeToTrade",
            WeenieError::YouAreNotTrainedInThatTradeSkill => "YouAreNotTrainedInThatTradeSkill",
            WeenieError::YourHandsMustBeFree => "YourHandsMustBeFree",
            WeenieError::YouCannotLinkToThatPortal => "YouCannotLinkToThatPortal",
            WeenieError::YouHaveSolvedThisQuestTooRecently => "YouHaveSolvedThisQuestTooRecently",
            WeenieError::YouHaveSolvedThisQuestTooManyTimes => "YouHaveSolvedThisQuestTooManyTimes",
            WeenieError::QuestUnknown => "QuestUnknown",
            WeenieError::QuestTableCorrupt => "QuestTableCorrupt",
            WeenieError::QuestBad => "QuestBad",
            WeenieError::QuestDuplicate => "QuestDuplicate",
            WeenieError::QuestUnsolved => "QuestUnsolved",
            WeenieError::ItemRequiresQuestToBePickedUp => "ItemRequiresQuestToBePickedUp",
            WeenieError::QuestSolvedTooLongAgo => "QuestSolvedTooLongAgo",
            WeenieError::TradeIgnoringRequests => "TradeIgnoringRequests",
            WeenieError::TradeSquelched => "TradeSquelched",
            WeenieError::TradeMaxDistanceExceeded => "TradeMaxDistanceExceeded",
            WeenieError::TradeAlreadyTrading => "TradeAlreadyTrading",
            WeenieError::TradeBusy => "TradeBusy",
            WeenieError::TradeClosed => "TradeClosed",
            WeenieError::TradeExpired => "TradeExpired",
            WeenieError::TradeItemBeingTraded => "TradeItemBeingTraded",
            WeenieError::TradeNonEmptyContainer => "TradeNonEmptyContainer",
            WeenieError::TradeNonCombatMode => "TradeNonCombatMode",
            WeenieError::TradeIncomplete => "TradeIncomplete",
            WeenieError::TradeStampMismatch => "TradeStampMismatch",
            WeenieError::TradeUnopened => "TradeUnopened",
            WeenieError::TradeEmpty => "TradeEmpty",
            WeenieError::TradeAlreadyAccepted => "TradeAlreadyAccepted",
            WeenieError::TradeOutOfSync => "TradeOutOfSync",
            WeenieError::PKsMayNotUsePortal => "PKsMayNotUsePortal",
            WeenieError::NonPKsMayNotUsePortal => "NonPKsMayNotUsePortal",
            WeenieError::HouseAbandoned => "HouseAbandoned",
            WeenieError::HouseEvicted => "HouseEvicted",
            WeenieError::HouseAlreadyOwned => "HouseAlreadyOwned",
            WeenieError::HouseBuyFailed => "HouseBuyFailed",
            WeenieError::HouseRentFailed => "HouseRentFailed",
            WeenieError::Hooked => "Hooked",
            WeenieError::MagicInvalidPosition => "MagicInvalidPosition",
            WeenieError::YouMustHaveDarkMajestyToUsePortal => "YouMustHaveDarkMajestyToUsePortal",
            WeenieError::InvalidAmmoType => "InvalidAmmoType",
            WeenieError::SkillTooLow => "SkillTooLow",
            WeenieError::YouHaveUsedAllTheHooks => "YouHaveUsedAllTheHooks",
            WeenieError::TradeAiDoesntWant => "TradeAiDoesntWant",
            WeenieError::HookHouseNotOwned => "HookHouseNotOwned",
            WeenieError::YouMustCompleteQuestToUsePortal => "YouMustCompleteQuestToUsePortal",
            WeenieError::HouseNoAllegiance => "HouseNoAllegiance",
            WeenieError::YouMustOwnHouseToUseCommand => "YouMustOwnHouseToUseCommand",
            WeenieError::YourMonarchDoesNotOwnAMansionOrVilla => "YourMonarchDoesNotOwnAMansionOrVilla",
            WeenieError::YourMonarchsHouseIsNotAMansionOrVilla => "YourMonarchsHouseIsNotAMansionOrVilla",
            WeenieError::YourMonarchHasClosedTheMansion => "YourMonarchHasClosedTheMansion",
            WeenieError::YouMustBeMonarchToPurchaseDwelling => "YouMustBeMonarchToPurchaseDwelling",
            WeenieError::AllegianceTimeout => "AllegianceTimeout",
            WeenieError::YourOfferOfAllegianceWasIgnored => "YourOfferOfAllegianceWasIgnored",
            WeenieError::ConfirmationInProgress => "ConfirmationInProgress",
            WeenieError::YouMustBeAMonarchToUseCommand => "YouMustBeAMonarchToUseCommand",
            WeenieError::YouMustSpecifyCharacterToBoot => "YouMustSpecifyCharacterToBoot",
            WeenieError::YouCantBootYourself => "YouCantBootYourself",
            WeenieError::ThatCharacterDoesNotExist => "ThatCharacterDoesNotExist",
            WeenieError::ThatPersonIsNotInYourAllegiance => "ThatPersonIsNotInYourAllegiance",
            WeenieError::CantBreakFromPatronNotInAllegiance => "CantBreakFromPatronNotInAllegiance",
            WeenieError::YourAllegianceHasBeenDissolved => "YourAllegianceHasBeenDissolved",
            WeenieError::YourPatronsAllegianceHasBeenBroken => "YourPatronsAllegianceHasBeenBroken",
            WeenieError::YouHaveMovedTooFar => "YouHaveMovedTooFar",
            WeenieError::TeleToInvalidPosition => "TeleToInvalidPosition",
            WeenieError::MustHaveDarkMajestyToUse => "MustHaveDarkMajestyToUse",
            WeenieError::YouFailToLinkWithLifestone => "YouFailToLinkWithLifestone",
            WeenieError::YouWanderedTooFarToLinkWithLifestone => "YouWanderedTooFarToLinkWithLifestone",
            WeenieError::YouSuccessfullyLinkWithLifestone => "YouSuccessfullyLinkWithLifestone",
            WeenieError::YouMustLinkToLifestoneToRecall => "YouMustLinkToLifestoneToRecall",
            WeenieError::YouFailToRecallToLifestone => "YouFailToRecallToLifestone",
            WeenieError::YouFailToLinkWithPortal => "YouFailToLinkWithPortal",
            WeenieError::YouSuccessfullyLinkWithPortal => "YouSuccessfullyLinkWithPortal",
            WeenieError::YouFailToRecallToPortal => "YouFailToRecallToPortal",
            WeenieError::YouMustLinkToPortalToRecall => "YouMustLinkToPortalToRecall",
            WeenieError::YouFailToSummonPortal => "YouFailToSummonPortal",
            WeenieError::YouMustLinkToPortalToSummonIt => "YouMustLinkToPortalToSummonIt",
            WeenieError::YouFailToTeleport => "YouFailToTeleport",
            WeenieError::YouHaveBeenTeleportedTooRecently => "YouHaveBeenTeleportedTooRecently",
            WeenieError::YouMustBeAnAdvocateToUsePortal => "YouMustBeAnAdvocateToUsePortal",
            WeenieError::PortalAisNotAllowed => "PortalAisNotAllowed",
            WeenieError::PlayersMayNotUsePortal => "PlayersMayNotUsePortal",
            WeenieError::YouAreNotPowerfulEnoughToUsePortal => "YouAreNotPowerfulEnoughToUsePortal",
            WeenieError::YouAreTooPowerfulToUsePortal => "YouAreTooPowerfulToUsePortal",
            WeenieError::YouCannotRecallPortal => "YouCannotRecallPortal",
            WeenieError::YouCannotSummonPortal => "YouCannotSummonPortal",
            WeenieError::LockAlreadyUnlocked => "LockAlreadyUnlocked",
            WeenieError::YouCannotLockOrUnlockThat => "YouCannotLockOrUnlockThat",
            WeenieError::YouCannotLockWhatIsOpen => "YouCannotLockWhatIsOpen",
            WeenieError::KeyDoesntFitThisLock => "KeyDoesntFitThisLock",
            WeenieError::LockUsedTooRecently => "LockUsedTooRecently",
            WeenieError::YouArentTrainedInLockpicking => "YouArentTrainedInLockpicking",
            WeenieError::AllegianceInfoEmptyName => "AllegianceInfoEmptyName",
            WeenieError::AllegianceInfoSelf => "AllegianceInfoSelf",
            WeenieError::AllegianceInfoTooRecent => "AllegianceInfoTooRecent",
            WeenieError::AbuseNoSuchCharacter => "AbuseNoSuchCharacter",
            WeenieError::AbuseReportedSelf => "AbuseReportedSelf",
            WeenieError::AbuseComplaintHandled => "AbuseComplaintHandled",
            WeenieError::YouDoNotOwnThatSalvageTool => "YouDoNotOwnThatSalvageTool",
            WeenieError::YouDoNotOwnThatItem => "YouDoNotOwnThatItem",
            WeenieError::MaterialCannotBeCreated => "MaterialCannotBeCreated",
            WeenieError::ItemsAttemptingToSalvageIsInvalid => "ItemsAttemptingToSalvageIsInvalid",
            WeenieError::YouCannotSalvageItemsInTrading => "YouCannotSalvageItemsInTrading",
            WeenieError::YouMustBeHouseGuestToUsePortal => "YouMustBeHouseGuestToUsePortal",
            WeenieError::YourAllegianceRankIsTooLowToUseMagic => "YourAllegianceRankIsTooLowToUseMagic",
            WeenieError::YourArcaneLoreIsTooLowToUseMagic => "YourArcaneLoreIsTooLowToUseMagic",
            WeenieError::ItemDoesntHaveEnoughMana => "ItemDoesntHaveEnoughMana",
            WeenieError::YouHaveBeenInPKBattleTooRecently => "YouHaveBeenInPKBattleTooRecently",
            WeenieError::TradeAiRefuseEmote => "TradeAiRefuseEmote",
            WeenieError::YouFailToAlterSkill => "YouFailToAlterSkill",
            WeenieError::FellowshipDeclined => "FellowshipDeclined",
            WeenieError::FellowshipTimeout => "FellowshipTimeout",
            WeenieError::YouHaveFailedToAlterAttributes => "YouHaveFailedToAlterAttributes",
            WeenieError::CannotTransferAttributesWhileWieldingItem => "CannotTransferAttributesWhileWieldingItem",
            WeenieError::YouHaveSucceededTransferringAttributes => "YouHaveSucceededTransferringAttributes",
            WeenieError::HookIsDuplicated => "HookIsDuplicated",
            WeenieError::ItemIsWrongTypeForHook => "ItemIsWrongTypeForHook",
            WeenieError::HousingChestIsDuplicated => "HousingChestIsDuplicated",
            WeenieError::HookWillBeDeleted => "HookWillBeDeleted",
            WeenieError::HousingChestWillBeDeleted => "HousingChestWillBeDeleted",
            WeenieError::CannotSwearAllegianceWhileOwningMansion => "CannotSwearAllegianceWhileOwningMansion",
            WeenieError::YouCantDoThatWhileInTheAir => "YouCantDoThatWhileInTheAir",
            WeenieError::CannotChangePKStatusWhileRecovering => "CannotChangePKStatusWhileRecovering",
            WeenieError::AdvocatesCannotChangePKStatus => "AdvocatesCannotChangePKStatus",
            WeenieError::LevelTooLowToChangePKStatusWithObject => "LevelTooLowToChangePKStatusWithObject",
            WeenieError::LevelTooHighToChangePKStatusWithObject => "LevelTooHighToChangePKStatusWithObject",
            WeenieError::YouFeelAHarshDissonance => "YouFeelAHarshDissonance",
            WeenieError::YouArePKAgain => "YouArePKAgain",
            WeenieError::YouAreTemporarilyNoLongerPK => "YouAreTemporarilyNoLongerPK",
            WeenieError::PKLiteMayNotUsePortal => "PKLiteMayNotUsePortal",
            WeenieError::YouArentTrainedInHealing => "YouArentTrainedInHealing",
            WeenieError::YouDontOwnThatHealingKit => "YouDontOwnThatHealingKit",
            WeenieError::YouCantHealThat => "YouCantHealThat",
            WeenieError::YouArentReadyToHeal => "YouArentReadyToHeal",
            WeenieError::YouCanOnlyHealPlayers => "YouCanOnlyHealPlayers",
            WeenieError::LifestoneMagicProtectsYou => "LifestoneMagicProtectsYou",
            WeenieError::PortalEnergyProtectsYou => "PortalEnergyProtectsYou",
            WeenieError::YouAreNonPKAgain => "YouAreNonPKAgain",
            WeenieError::YoureTooCloseToYourSanctuary => "YoureTooCloseToYourSanctuary",
            WeenieError::CantDoThatTradeInProgress => "CantDoThatTradeInProgress",
            WeenieError::OnlyNonPKsMayEnterPKLite => "OnlyNonPKsMayEnterPKLite",
            WeenieError::YouAreNowPKLite => "YouAreNowPKLite",
            WeenieError::YouDoNotBelongToAFellowship => "YouDoNotBelongToAFellowship",
            WeenieError::UsingMaxHooksSilent => "UsingMaxHooksSilent",
            WeenieError::YouAreNowUsingMaxHooks => "YouAreNowUsingMaxHooks",
            WeenieError::YouAreNoLongerUsingMaxHooks => "YouAreNoLongerUsingMaxHooks",
            WeenieError::YouAreNotPermittedToUseThatHook => "YouAreNotPermittedToUseThatHook",
            WeenieError::LockedFellowshipCannotRecruitYou => "LockedFellowshipCannotRecruitYou",
            WeenieError::ActivationNotAllowedNotOwner => "ActivationNotAllowedNotOwner",
            WeenieError::TurbineChatIsEnabled => "TurbineChatIsEnabled",
            WeenieError::YouCannotAddPeopleToHearList => "YouCannotAddPeopleToHearList",
            WeenieError::YouAreNowDeafToScreams => "YouAreNowDeafTo_Screams",
            WeenieError::YouCanHearAllPlayersOnceAgain => "YouCanHearAllPlayersOnceAgain",
            WeenieError::YouChickenOut => "YouChickenOut",
            WeenieError::YouCanPossiblySucceed => "YouCanPossiblySucceed",
            WeenieError::FellowshipIsLocked => "FellowshipIsLocked",
            WeenieError::TradeComplete => "TradeComplete",
            WeenieError::NotASalvageTool => "NotASalvageTool",
            WeenieError::CharacterNotAvailable => "CharacterNotAvailable",
            WeenieError::YouMustWaitToPurchaseHouse => "YouMustWaitToPurchaseHouse",
            WeenieError::YouDoNotHaveAuthorityInAllegiance => "YouDoNotHaveAuthorityInAllegiance",
            WeenieError::YouHaveMaxAccountsBanned => "YouHaveMaxAccountsBanned",
            WeenieError::YouHaveMaxAllegianceOfficers => "YouHaveMaxAllegianceOfficers",
            WeenieError::YourAllegianceOfficersHaveBeenCleared => "YourAllegianceOfficersHaveBeenCleared",
            WeenieError::YouCannotJoinChannelsWhileGagged => "YouCannotJoinChannelsWhileGagged",
            WeenieError::YouAreNoLongerAllegianceOfficer => "YouAreNoLongerAllegianceOfficer",
            WeenieError::YourAllegianceDoesNotHaveHometown => "YourAllegianceDoesNotHaveHometown",
            WeenieError::HookItemNotUsableCannotOpen => "HookItemNotUsable_CannotOpen",
            WeenieError::HookItemNotUsableCanOpen => "HookItemNotUsable_CanOpen",
            WeenieError::MissileOutOfRange => "MissileOutOfRange",
            WeenieError::MustPurchaseThroneOfDestinyToUseFunction => "MustPurchaseThroneOfDestinyToUseFunction",
            WeenieError::MustPurchaseThroneOfDestinyToUseItem => "MustPurchaseThroneOfDestinyToUseItem",
            WeenieError::MustPurchaseThroneOfDestinyToUsePortal => "MustPurchaseThroneOfDestinyToUsePortal",
            WeenieError::MustPurchaseThroneOfDestinyToAccessQuest => "MustPurchaseThroneOfDestinyToAccessQuest",
            WeenieError::YouFailedToCompleteAugmentation => "YouFailedToCompleteAugmentation",
            WeenieError::AugmentationUsedTooManyTimes => "AugmentationUsedTooManyTimes",
            WeenieError::AugmentationTypeUsedTooManyTimes => "AugmentationTypeUsedTooManyTimes",
            WeenieError::AugmentationNotEnoughExperience => "AugmentationNotEnoughExperience",
            WeenieError::ExitTrainingAcademyToUseCommand => "ExitTrainingAcademyToUseCommand",
            WeenieError::OnlyPKsMayUseCommand => "OnlyPKsMayUseCommand",
            WeenieError::OnlyPKLiteMayUseCommand => "OnlyPKLiteMayUseCommand",
            WeenieError::MaxFriendsExceeded => "MaxFriendsExceeded",
            WeenieError::ThatCharacterNotOnYourFriendsList => "ThatCharacterNotOnYourFriendsList",
            WeenieError::OnlyHouseOwnerCanUseCommand => "OnlyHouseOwnerCanUseCommand",
            WeenieError::InvalidAllegianceNameCantBeEmpty => "InvalidAllegianceNameCantBeEmpty",
            WeenieError::InvalidAllegianceNameTooLong => "InvalidAllegianceNameTooLong",
            WeenieError::InvalidAllegianceNameBadCharacters => "InvalidAllegianceNameBadCharacters",
            WeenieError::InvalidAllegianceNameInappropriate => "InvalidAllegianceNameInappropriate",
            WeenieError::InvalidAllegianceNameAlreadyInUse => "InvalidAllegianceNameAlreadyInUse",
            WeenieError::AllegianceNameCleared => "AllegianceNameCleared",
            WeenieError::InvalidAllegianceNameSameName => "InvalidAllegianceNameSameName",
            WeenieError::InvalidOfficerLevel => "InvalidOfficerLevel",
            WeenieError::AllegianceOfficerTitleIsNotAppropriate => "AllegianceOfficerTitleIsNotAppropriate",
            WeenieError::AllegianceNameIsTooLong => "AllegianceNameIsTooLong",
            WeenieError::AllegianceOfficerTitlesCleared => "AllegianceOfficerTitlesCleared",
            WeenieError::AllegianceTitleHasIllegalChars => "AllegianceTitleHasIllegalChars",
            WeenieError::YouHaveNotPreApprovedVassals => "YouHaveNotPreApprovedVassals",
            WeenieError::YouHaveClearedPreApprovedVassal => "YouHaveClearedPreApprovedVassal",
            WeenieError::CharIsAlreadyGagged => "CharIsAlreadyGagged",
            WeenieError::CharIsNotCurrentlyGagged => "CharIsNotCurrentlyGagged",
            WeenieError::YourAllegianceChatPrivilegesRestored => "YourAllegianceChatPrivilegesRestored",
            WeenieError::TooManyUniqueItems => "TooManyUniqueItems",
            WeenieError::HeritageRequiresSpecificArmor => "HeritageRequiresSpecificArmor",
            WeenieError::ArmorRequiresSpecificHeritage => "ArmorRequiresSpecificHeritage",
            WeenieError::OlthoiCannotInteractWithThat => "OlthoiCannotInteractWithThat",
            WeenieError::OlthoiCannotUseLifestones => "OlthoiCannotUseLifestones",
            WeenieError::OlthoiVendorLooksInHorror => "OlthoiVendorLooksInHorror",
            WeenieError::OlthoiCannotJoinFellowship => "OlthoiCannotJoinFellowship",
            WeenieError::OlthoiCannotJoinAllegiance => "OlthoiCannotJoinAllegiance",
            WeenieError::YouCannotUseThatItem => "YouCannotUseThatItem",
            WeenieError::ThisPersonWillNotInteractWithYou => "ThisPersonWillNotInteractWithYou",
            WeenieError::OnlyOlthoiMayUsePortal => "OnlyOlthoiMayUsePortal",
            WeenieError::OlthoiMayNotUsePortal => "OlthoiMayNotUsePortal",
            WeenieError::YouMayNotUsePortalWithVitae => "YouMayNotUsePortalWithVitae",
            WeenieError::YouMustBeTwoWeeksOldToUsePortal => "YouMustBeTwoWeeksOldToUsePortal",
            WeenieError::OlthoiCanOnlyRecallToLifestone => "OlthoiCanOnlyRecallToLifestone",
            WeenieError::ContractError => "ContractError",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    /// The PositionFlags value defines the fields present in the Position structure.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PositionFlags: u32 {
        const HAS_VELOCITY = 0x1;
        const HAS_PLACEMENT_ID = 0x2;
        const IS_GROUNDED = 0x4;
        const ORIENTATION_HAS_NO_W = 0x8;
        const ORIENTATION_HAS_NO_X = 0x10;
        const ORIENTATION_HAS_NO_Y = 0x20;
        const ORIENTATION_HAS_NO_Z = 0x40;
    }
}

impl crate::readers::ACDataType for PositionFlags {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PositionFlags::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for PositionFlags {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

/// Height of the attack.  TODO these need to be verified.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AttackHeight {
    High = 0x1,
    Medium = 0x2,
    Low = 0x3,
}

impl crate::readers::ACDataType for AttackHeight {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AttackHeight::try_from(value)?)
    }
}

impl crate::writers::ACWritable for AttackHeight {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for AttackHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AttackHeight::High => "High",
            AttackHeight::Medium => "Medium",
            AttackHeight::Low => "Low",
        };
        write!(f, "{}", s)
    }
}

/// Container properties of an item
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ContainerProperties {
    None = 0x0,
    Container = 0x1,
    Foci = 0x2,
}

impl crate::readers::ACDataType for ContainerProperties {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ContainerProperties::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ContainerProperties {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ContainerProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ContainerProperties::None => "None",
            ContainerProperties::Container => "Container",
            ContainerProperties::Foci => "Foci",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AttackType {
    Undef = 0x0,
    Punch = 0x1,
    Thrust = 0x2,
    Slash = 0x4,
    Kick = 0x8,
    OffhandPunch = 0x10,
    DoubleSlash = 0x20,
    TripleSlash = 0x40,
    DoubleThrust = 0x80,
    TripleThrust = 0x100,
    OffhandThrust = 0x200,
    OffhandSlash = 0x400,
    OffhandDoubleSlash = 0x800,
    OffhandTripleSlash = 0x1000,
    OffhandDoubleThrust = 0x2000,
    OffhandTripleThrust = 0x4000,
}

impl crate::readers::ACDataType for AttackType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AttackType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for AttackType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for AttackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AttackType::Undef => "Undef",
            AttackType::Punch => "Punch",
            AttackType::Thrust => "Thrust",
            AttackType::Slash => "Slash",
            AttackType::Kick => "Kick",
            AttackType::OffhandPunch => "OffhandPunch",
            AttackType::DoubleSlash => "DoubleSlash",
            AttackType::TripleSlash => "TripleSlash",
            AttackType::DoubleThrust => "DoubleThrust",
            AttackType::TripleThrust => "TripleThrust",
            AttackType::OffhandThrust => "OffhandThrust",
            AttackType::OffhandSlash => "OffhandSlash",
            AttackType::OffhandDoubleSlash => "OffhandDoubleSlash",
            AttackType::OffhandTripleSlash => "OffhandTripleSlash",
            AttackType::OffhandDoubleThrust => "OffhandDoubleThrust",
            AttackType::OffhandTripleThrust => "OffhandTripleThrust",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    /// The objects type information
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ItemType: u32 {
        const MELEE_WEAPON = 0x1;
        const ARMOR = 0x2;
        const CLOTHING = 0x4;
        const JEWELRY = 0x8;
        const CREATURE = 0x10;
        const FOOD = 0x20;
        const MONEY = 0x40;
        const MISC = 0x80;
        const MISSILE_WEAPON = 0x100;
        const CONTAINER = 0x200;
        const USELESS = 0x400;
        const GEM = 0x800;
        const SPELL_COMPONENTS = 0x1000;
        const WRITABLE = 0x2000;
        const KEY = 0x4000;
        const CASTER = 0x8000;
        const PORTAL = 0x10000;
        const LOCKABLE = 0x20000;
        const PROMISSORY_NOTE = 0x40000;
        const MANA_STONE = 0x80000;
        const SERVICE = 0x100000;
        const MAGIC_WIELDABLE = 0x200000;
        const CRAFT_COOKING_BASE = 0x400000;
        const CRAFT_ALCHEMY_BASE = 0x800000;
        const CRAFT_FLETCHING_BASE = 0x2000000;
        const CRAFT_ALCHEMY_INTERMEDIATE = 0x4000000;
        const CRAFT_FLETCHING_INTERMEDIATE = 0x8000000;
        const LIFE_STONE = 0x10000000;
        const TINKERING_TOOL = 0x20000000;
        const TINKERING_MATERIAL = 0x40000000;
        const GAMEBOARD = 0x80000000;
    }
}

impl crate::readers::ACDataType for ItemType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ItemType::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for ItemType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

/// The Skill identifies a specific Character skill.
#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum SkillId {
    Axe = 0x1,
    Bow = 0x2,
    Crossbow = 0x3,
    Dagger = 0x4,
    Mace = 0x5,
    MeleeDefense = 0x6,
    MissileDefense = 0x7,
    Sling = 0x8,
    Spear = 0x9,
    Staff = 0xA,
    Sword = 0xB,
    ThrownWeapons = 0xC,
    UnarmedCombat = 0xD,
    ArcaneLore = 0xE,
    MagicDefense = 0xF,
    ManaConversion = 0x10,
    Spellcraft = 0x11,
    ItemTinkering = 0x12,
    AssessPerson = 0x13,
    Deception = 0x14,
    Healing = 0x15,
    Jump = 0x16,
    Lockpick = 0x17,
    Run = 0x18,
    Awareness = 0x19,
    ArmorRepair = 0x1A,
    AssessCreature = 0x1B,
    WeaponTinkering = 0x1C,
    ArmorTinkering = 0x1D,
    MagicItemTinkering = 0x1E,
    CreatureEnchantment = 0x1F,
    ItemEnchantment = 0x20,
    LifeMagic = 0x21,
    WarMagic = 0x22,
    Leadership = 0x23,
    Loyalty = 0x24,
    Fletching = 0x25,
    Alchemy = 0x26,
    Cooking = 0x27,
    Salvaging = 0x28,
    TwoHandedCombat = 0x29,
    Gearcraft = 0x2A,
    VoidMagic = 0x2B,
    HeavyWeapons = 0x2C,
    LightWeapons = 0x2D,
    FinesseWeapons = 0x2E,
    MissleWeapons = 0x2F,
    DualWield = 0x31,
    Recklessness = 0x32,
    SneakAttack = 0x33,
    DirtyFighting = 0x34,
    Challenge = 0x35,
    Summoning = 0x36,
}

impl crate::readers::ACDataType for SkillId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(SkillId::try_from(value)?)
    }
}

impl crate::writers::ACWritable for SkillId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_i32(writer, self.clone() as i32)?;
        Ok(())
    }
}

impl std::fmt::Display for SkillId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SkillId::Axe => "Axe",
            SkillId::Bow => "Bow",
            SkillId::Crossbow => "Crossbow",
            SkillId::Dagger => "Dagger",
            SkillId::Mace => "Mace",
            SkillId::MeleeDefense => "MeleeDefense",
            SkillId::MissileDefense => "MissileDefense",
            SkillId::Sling => "Sling",
            SkillId::Spear => "Spear",
            SkillId::Staff => "Staff",
            SkillId::Sword => "Sword",
            SkillId::ThrownWeapons => "ThrownWeapons",
            SkillId::UnarmedCombat => "UnarmedCombat",
            SkillId::ArcaneLore => "ArcaneLore",
            SkillId::MagicDefense => "MagicDefense",
            SkillId::ManaConversion => "ManaConversion",
            SkillId::Spellcraft => "Spellcraft",
            SkillId::ItemTinkering => "ItemTinkering",
            SkillId::AssessPerson => "AssessPerson",
            SkillId::Deception => "Deception",
            SkillId::Healing => "Healing",
            SkillId::Jump => "Jump",
            SkillId::Lockpick => "Lockpick",
            SkillId::Run => "Run",
            SkillId::Awareness => "Awareness",
            SkillId::ArmorRepair => "ArmorRepair",
            SkillId::AssessCreature => "AssessCreature",
            SkillId::WeaponTinkering => "WeaponTinkering",
            SkillId::ArmorTinkering => "ArmorTinkering",
            SkillId::MagicItemTinkering => "MagicItemTinkering",
            SkillId::CreatureEnchantment => "CreatureEnchantment",
            SkillId::ItemEnchantment => "ItemEnchantment",
            SkillId::LifeMagic => "LifeMagic",
            SkillId::WarMagic => "WarMagic",
            SkillId::Leadership => "Leadership",
            SkillId::Loyalty => "Loyalty",
            SkillId::Fletching => "Fletching",
            SkillId::Alchemy => "Alchemy",
            SkillId::Cooking => "Cooking",
            SkillId::Salvaging => "Salvaging",
            SkillId::TwoHandedCombat => "TwoHandedCombat",
            SkillId::Gearcraft => "Gearcraft",
            SkillId::VoidMagic => "VoidMagic",
            SkillId::HeavyWeapons => "HeavyWeapons",
            SkillId::LightWeapons => "LightWeapons",
            SkillId::FinesseWeapons => "FinesseWeapons",
            SkillId::MissleWeapons => "MissleWeapons",
            SkillId::DualWield => "DualWield",
            SkillId::Recklessness => "Recklessness",
            SkillId::SneakAttack => "SneakAttack",
            SkillId::DirtyFighting => "DirtyFighting",
            SkillId::Challenge => "Challenge",
            SkillId::Summoning => "Summoning",
        };
        write!(f, "{}", s)
    }
}

/// The SkillAdvancementClass identifies whether a skill is untrained, trained or specialized.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum SkillAdvancementClass {
    Untrained = 0x1,
    Trained = 0x2,
    Specialized = 0x3,
}

impl crate::readers::ACDataType for SkillAdvancementClass {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(SkillAdvancementClass::try_from(value)?)
    }
}

impl crate::writers::ACWritable for SkillAdvancementClass {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for SkillAdvancementClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SkillAdvancementClass::Untrained => "Untrained",
            SkillAdvancementClass::Trained => "Trained",
            SkillAdvancementClass::Specialized => "Specialized",
        };
        write!(f, "{}", s)
    }
}

#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum PropertyAttribute2nd {
    Undef = 0x0,
    MaxHealth = 0x1,
    Health = 0x2,
    MaxStamina = 0x3,
    Stamina = 0x4,
    MaxMana = 0x5,
    Mana = 0x6,
}

impl crate::readers::ACDataType for PropertyAttribute2nd {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(PropertyAttribute2nd::try_from(value)?)
    }
}

impl crate::writers::ACWritable for PropertyAttribute2nd {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.clone() as u16)?;
        Ok(())
    }
}

impl std::fmt::Display for PropertyAttribute2nd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PropertyAttribute2nd::Undef => "Undef",
            PropertyAttribute2nd::MaxHealth => "MaxHealth",
            PropertyAttribute2nd::Health => "Health",
            PropertyAttribute2nd::MaxStamina => "MaxStamina",
            PropertyAttribute2nd::Stamina => "Stamina",
            PropertyAttribute2nd::MaxMana => "MaxMana",
            PropertyAttribute2nd::Mana => "Mana",
        };
        write!(f, "{}", s)
    }
}

/// The EmoteType identifies the type of emote action
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum EmoteType {
    #[serde(rename = "Invalid_EmoteType")]
    InvalidEmoteType = 0x0,
    #[serde(rename = "Act_EmoteType")]
    ActEmoteType = 0x1,
    #[serde(rename = "AwardXP_EmoteType")]
    AwardXPEmoteType = 0x2,
    #[serde(rename = "Give_EmoteType")]
    GiveEmoteType = 0x3,
    #[serde(rename = "MoveHome_EmoteType")]
    MoveHomeEmoteType = 0x4,
    #[serde(rename = "Motion_EmoteType")]
    MotionEmoteType = 0x5,
    #[serde(rename = "Move_EmoteType")]
    MoveEmoteType = 0x6,
    #[serde(rename = "PhysScript_EmoteType")]
    PhysScriptEmoteType = 0x7,
    #[serde(rename = "Say_EmoteType")]
    SayEmoteType = 0x8,
    #[serde(rename = "Sound_EmoteType")]
    SoundEmoteType = 0x9,
    #[serde(rename = "Tell_EmoteType")]
    TellEmoteType = 0xA,
    #[serde(rename = "Turn_EmoteType")]
    TurnEmoteType = 0xB,
    #[serde(rename = "TurnToTarget_EmoteType")]
    TurnToTargetEmoteType = 0xC,
    #[serde(rename = "TextDirect_EmoteType")]
    TextDirectEmoteType = 0xD,
    #[serde(rename = "CastSpell_EmoteType")]
    CastSpellEmoteType = 0xE,
    #[serde(rename = "Activate_EmoteType")]
    ActivateEmoteType = 0xF,
    #[serde(rename = "WorldBroadcast_EmoteType")]
    WorldBroadcastEmoteType = 0x10,
    #[serde(rename = "LocalBroadcast_EmoteType")]
    LocalBroadcastEmoteType = 0x11,
    #[serde(rename = "DirectBroadcast_EmoteType")]
    DirectBroadcastEmoteType = 0x12,
    #[serde(rename = "CastSpellInstant_EmoteType")]
    CastSpellInstantEmoteType = 0x13,
    #[serde(rename = "UpdateQuest_EmoteType")]
    UpdateQuestEmoteType = 0x14,
    #[serde(rename = "InqQuest_EmoteType")]
    InqQuestEmoteType = 0x15,
    #[serde(rename = "StampQuest_EmoteType")]
    StampQuestEmoteType = 0x16,
    #[serde(rename = "StartEvent_EmoteType")]
    StartEventEmoteType = 0x17,
    #[serde(rename = "StopEvent_EmoteType")]
    StopEventEmoteType = 0x18,
    #[serde(rename = "BLog_EmoteType")]
    BLogEmoteType = 0x19,
    #[serde(rename = "AdminSpam_EmoteType")]
    AdminSpamEmoteType = 0x1A,
    #[serde(rename = "TeachSpell_EmoteType")]
    TeachSpellEmoteType = 0x1B,
    #[serde(rename = "AwardSkillXP_EmoteType")]
    AwardSkillXPEmoteType = 0x1C,
    #[serde(rename = "AwardSkillPoints_EmoteType")]
    AwardSkillPointsEmoteType = 0x1D,
    #[serde(rename = "InqQuestSolves_EmoteType")]
    InqQuestSolvesEmoteType = 0x1E,
    #[serde(rename = "EraseQuest_EmoteType")]
    EraseQuestEmoteType = 0x1F,
    #[serde(rename = "DecrementQuest_EmoteType")]
    DecrementQuestEmoteType = 0x20,
    #[serde(rename = "IncrementQuest_EmoteType")]
    IncrementQuestEmoteType = 0x21,
    #[serde(rename = "AddCharacterTitle_EmoteType")]
    AddCharacterTitleEmoteType = 0x22,
    #[serde(rename = "InqBoolStat_EmoteType")]
    InqBoolStatEmoteType = 0x23,
    #[serde(rename = "InqIntStat_EmoteType")]
    InqIntStatEmoteType = 0x24,
    #[serde(rename = "InqFloatStat_EmoteType")]
    InqFloatStatEmoteType = 0x25,
    #[serde(rename = "InqStringStat_EmoteType")]
    InqStringStatEmoteType = 0x26,
    #[serde(rename = "InqAttributeStat_EmoteType")]
    InqAttributeStatEmoteType = 0x27,
    #[serde(rename = "InqRawAttributeStat_EmoteType")]
    InqRawAttributeStatEmoteType = 0x28,
    #[serde(rename = "InqSecondaryAttributeStat_EmoteType")]
    InqSecondaryAttributeStatEmoteType = 0x29,
    #[serde(rename = "InqRawSecondaryAttributeStat_EmoteType")]
    InqRawSecondaryAttributeStatEmoteType = 0x2A,
    #[serde(rename = "InqSkillStat_EmoteType")]
    InqSkillStatEmoteType = 0x2B,
    #[serde(rename = "InqRawSkillStat_EmoteType")]
    InqRawSkillStatEmoteType = 0x2C,
    #[serde(rename = "InqSkillTrained_EmoteType")]
    InqSkillTrainedEmoteType = 0x2D,
    #[serde(rename = "InqSkillSpecialized_EmoteType")]
    InqSkillSpecializedEmoteType = 0x2E,
    #[serde(rename = "AwardTrainingCredits_EmoteType")]
    AwardTrainingCreditsEmoteType = 0x2F,
    #[serde(rename = "InflictVitaePenalty_EmoteType")]
    InflictVitaePenaltyEmoteType = 0x30,
    #[serde(rename = "AwardLevelProportionalXP_EmoteType")]
    AwardLevelProportionalXPEmoteType = 0x31,
    #[serde(rename = "AwardLevelProportionalSkillXP_EmoteType")]
    AwardLevelProportionalSkillXPEmoteType = 0x32,
    #[serde(rename = "InqEvent_EmoteType")]
    InqEventEmoteType = 0x33,
    #[serde(rename = "ForceMotion_EmoteType")]
    ForceMotionEmoteType = 0x34,
    #[serde(rename = "SetIntStat_EmoteType")]
    SetIntStatEmoteType = 0x35,
    #[serde(rename = "IncrementIntStat_EmoteType")]
    IncrementIntStatEmoteType = 0x36,
    #[serde(rename = "DecrementIntStat_EmoteType")]
    DecrementIntStatEmoteType = 0x37,
    #[serde(rename = "CreateTreasure_EmoteType")]
    CreateTreasureEmoteType = 0x38,
    #[serde(rename = "ResetHomePosition_EmoteType")]
    ResetHomePositionEmoteType = 0x39,
    #[serde(rename = "InqFellowQuest_EmoteType")]
    InqFellowQuestEmoteType = 0x3A,
    #[serde(rename = "InqFellowNum_EmoteType")]
    InqFellowNumEmoteType = 0x3B,
    #[serde(rename = "UpdateFellowQuest_EmoteType")]
    UpdateFellowQuestEmoteType = 0x3C,
    #[serde(rename = "StampFellowQuest_EmoteType")]
    StampFellowQuestEmoteType = 0x3D,
    #[serde(rename = "AwardNoShareXP_EmoteType")]
    AwardNoShareXPEmoteType = 0x3E,
    #[serde(rename = "SetSanctuaryPosition_EmoteType")]
    SetSanctuaryPositionEmoteType = 0x3F,
    #[serde(rename = "TellFellow_EmoteType")]
    TellFellowEmoteType = 0x40,
    #[serde(rename = "FellowBroadcast_EmoteType")]
    FellowBroadcastEmoteType = 0x41,
    #[serde(rename = "LockFellow_EmoteType")]
    LockFellowEmoteType = 0x42,
    #[serde(rename = "Goto_EmoteType")]
    GotoEmoteType = 0x43,
    #[serde(rename = "PopUp_EmoteType")]
    PopUpEmoteType = 0x44,
    #[serde(rename = "SetBoolStat_EmoteType")]
    SetBoolStatEmoteType = 0x45,
    #[serde(rename = "SetQuestCompletions_EmoteType")]
    SetQuestCompletionsEmoteType = 0x46,
    #[serde(rename = "InqNumCharacterTitles_EmoteType")]
    InqNumCharacterTitlesEmoteType = 0x47,
    #[serde(rename = "Generate_EmoteType")]
    GenerateEmoteType = 0x48,
    #[serde(rename = "PetCastSpellOnOwner_EmoteType")]
    PetCastSpellOnOwnerEmoteType = 0x49,
    #[serde(rename = "TakeItems_EmoteType")]
    TakeItemsEmoteType = 0x4A,
    #[serde(rename = "InqYesNo_EmoteType")]
    InqYesNoEmoteType = 0x4B,
    #[serde(rename = "InqOwnsItems_EmoteType")]
    InqOwnsItemsEmoteType = 0x4C,
    #[serde(rename = "DeleteSelf_EmoteType")]
    DeleteSelfEmoteType = 0x4D,
    #[serde(rename = "KillSelf_EmoteType")]
    KillSelfEmoteType = 0x4E,
    #[serde(rename = "UpdateMyQuest_EmoteType")]
    UpdateMyQuestEmoteType = 0x4F,
    #[serde(rename = "InqMyQuest_EmoteType")]
    InqMyQuestEmoteType = 0x50,
    #[serde(rename = "StampMyQuest_EmoteType")]
    StampMyQuestEmoteType = 0x51,
    #[serde(rename = "InqMyQuestSolves_EmoteType")]
    InqMyQuestSolvesEmoteType = 0x52,
    #[serde(rename = "EraseMyQuest_EmoteType")]
    EraseMyQuestEmoteType = 0x53,
    #[serde(rename = "DecrementMyQuest_EmoteType")]
    DecrementMyQuestEmoteType = 0x54,
    #[serde(rename = "IncrementMyQuest_EmoteType")]
    IncrementMyQuestEmoteType = 0x55,
    #[serde(rename = "SetMyQuestCompletions_EmoteType")]
    SetMyQuestCompletionsEmoteType = 0x56,
    #[serde(rename = "MoveToPos_EmoteType")]
    MoveToPosEmoteType = 0x57,
    #[serde(rename = "LocalSignal_EmoteType")]
    LocalSignalEmoteType = 0x58,
    #[serde(rename = "InqPackSpace_EmoteType")]
    InqPackSpaceEmoteType = 0x59,
    #[serde(rename = "RemoveVitaePenalty_EmoteType")]
    RemoveVitaePenaltyEmoteType = 0x5A,
    #[serde(rename = "SetEyeTexture_EmoteType")]
    SetEyeTextureEmoteType = 0x5B,
    #[serde(rename = "SetEyePalette_EmoteType")]
    SetEyePaletteEmoteType = 0x5C,
    #[serde(rename = "SetNoseTexture_EmoteType")]
    SetNoseTextureEmoteType = 0x5D,
    #[serde(rename = "SetNosePalette_EmoteType")]
    SetNosePaletteEmoteType = 0x5E,
    #[serde(rename = "SetMouthTexture_EmoteType")]
    SetMouthTextureEmoteType = 0x5F,
    #[serde(rename = "SetMouthPalette_EmoteType")]
    SetMouthPaletteEmoteType = 0x60,
    #[serde(rename = "SetHeadObject_EmoteType")]
    SetHeadObjectEmoteType = 0x61,
    #[serde(rename = "SetHeadPalette_EmoteType")]
    SetHeadPaletteEmoteType = 0x62,
    #[serde(rename = "TeleportTarget_EmoteType")]
    TeleportTargetEmoteType = 0x63,
    #[serde(rename = "TeleportSelf_EmoteType")]
    TeleportSelfEmoteType = 0x64,
    #[serde(rename = "StartBarber_EmoteType")]
    StartBarberEmoteType = 0x65,
    #[serde(rename = "InqQuestBitsOn_EmoteType")]
    InqQuestBitsOnEmoteType = 0x66,
    #[serde(rename = "InqQuestBitsOff_EmoteType")]
    InqQuestBitsOffEmoteType = 0x67,
    #[serde(rename = "InqMyQuestBitsOn_EmoteType")]
    InqMyQuestBitsOnEmoteType = 0x68,
    #[serde(rename = "InqMyQuestBitsOff_EmoteType")]
    InqMyQuestBitsOffEmoteType = 0x69,
    #[serde(rename = "SetQuestBitsOn_EmoteType")]
    SetQuestBitsOnEmoteType = 0x6A,
    #[serde(rename = "SetQuestBitsOff_EmoteType")]
    SetQuestBitsOffEmoteType = 0x6B,
    #[serde(rename = "SetMyQuestBitsOn_EmoteType")]
    SetMyQuestBitsOnEmoteType = 0x6C,
    #[serde(rename = "SetMyQuestBitsOff_EmoteType")]
    SetMyQuestBitsOffEmoteType = 0x6D,
    #[serde(rename = "UntrainSkill_EmoteType")]
    UntrainSkillEmoteType = 0x6E,
    #[serde(rename = "SetAltRacialSkills_EmoteType")]
    SetAltRacialSkillsEmoteType = 0x6F,
    #[serde(rename = "SpendLuminance_EmoteType")]
    SpendLuminanceEmoteType = 0x70,
    #[serde(rename = "AwardLuminance_EmoteType")]
    AwardLuminanceEmoteType = 0x71,
    #[serde(rename = "InqInt64Stat_EmoteType")]
    InqInt64StatEmoteType = 0x72,
    #[serde(rename = "SetInt64Stat_EmoteType")]
    SetInt64StatEmoteType = 0x73,
    #[serde(rename = "OpenMe_EmoteType")]
    OpenMeEmoteType = 0x74,
    #[serde(rename = "CloseMe_EmoteType")]
    CloseMeEmoteType = 0x75,
    #[serde(rename = "SetFloatStat_EmoteType")]
    SetFloatStatEmoteType = 0x76,
    #[serde(rename = "AddContract_EmoteType")]
    AddContractEmoteType = 0x77,
    #[serde(rename = "RemoveContract_EmoteType")]
    RemoveContractEmoteType = 0x78,
    #[serde(rename = "InqContractsFull_EmoteType")]
    InqContractsFullEmoteType = 0x79,
}

impl crate::readers::ACDataType for EmoteType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EmoteType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for EmoteType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for EmoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EmoteType::InvalidEmoteType => "Invalid_EmoteType",
            EmoteType::ActEmoteType => "Act_EmoteType",
            EmoteType::AwardXPEmoteType => "AwardXP_EmoteType",
            EmoteType::GiveEmoteType => "Give_EmoteType",
            EmoteType::MoveHomeEmoteType => "MoveHome_EmoteType",
            EmoteType::MotionEmoteType => "Motion_EmoteType",
            EmoteType::MoveEmoteType => "Move_EmoteType",
            EmoteType::PhysScriptEmoteType => "PhysScript_EmoteType",
            EmoteType::SayEmoteType => "Say_EmoteType",
            EmoteType::SoundEmoteType => "Sound_EmoteType",
            EmoteType::TellEmoteType => "Tell_EmoteType",
            EmoteType::TurnEmoteType => "Turn_EmoteType",
            EmoteType::TurnToTargetEmoteType => "TurnToTarget_EmoteType",
            EmoteType::TextDirectEmoteType => "TextDirect_EmoteType",
            EmoteType::CastSpellEmoteType => "CastSpell_EmoteType",
            EmoteType::ActivateEmoteType => "Activate_EmoteType",
            EmoteType::WorldBroadcastEmoteType => "WorldBroadcast_EmoteType",
            EmoteType::LocalBroadcastEmoteType => "LocalBroadcast_EmoteType",
            EmoteType::DirectBroadcastEmoteType => "DirectBroadcast_EmoteType",
            EmoteType::CastSpellInstantEmoteType => "CastSpellInstant_EmoteType",
            EmoteType::UpdateQuestEmoteType => "UpdateQuest_EmoteType",
            EmoteType::InqQuestEmoteType => "InqQuest_EmoteType",
            EmoteType::StampQuestEmoteType => "StampQuest_EmoteType",
            EmoteType::StartEventEmoteType => "StartEvent_EmoteType",
            EmoteType::StopEventEmoteType => "StopEvent_EmoteType",
            EmoteType::BLogEmoteType => "BLog_EmoteType",
            EmoteType::AdminSpamEmoteType => "AdminSpam_EmoteType",
            EmoteType::TeachSpellEmoteType => "TeachSpell_EmoteType",
            EmoteType::AwardSkillXPEmoteType => "AwardSkillXP_EmoteType",
            EmoteType::AwardSkillPointsEmoteType => "AwardSkillPoints_EmoteType",
            EmoteType::InqQuestSolvesEmoteType => "InqQuestSolves_EmoteType",
            EmoteType::EraseQuestEmoteType => "EraseQuest_EmoteType",
            EmoteType::DecrementQuestEmoteType => "DecrementQuest_EmoteType",
            EmoteType::IncrementQuestEmoteType => "IncrementQuest_EmoteType",
            EmoteType::AddCharacterTitleEmoteType => "AddCharacterTitle_EmoteType",
            EmoteType::InqBoolStatEmoteType => "InqBoolStat_EmoteType",
            EmoteType::InqIntStatEmoteType => "InqIntStat_EmoteType",
            EmoteType::InqFloatStatEmoteType => "InqFloatStat_EmoteType",
            EmoteType::InqStringStatEmoteType => "InqStringStat_EmoteType",
            EmoteType::InqAttributeStatEmoteType => "InqAttributeStat_EmoteType",
            EmoteType::InqRawAttributeStatEmoteType => "InqRawAttributeStat_EmoteType",
            EmoteType::InqSecondaryAttributeStatEmoteType => "InqSecondaryAttributeStat_EmoteType",
            EmoteType::InqRawSecondaryAttributeStatEmoteType => "InqRawSecondaryAttributeStat_EmoteType",
            EmoteType::InqSkillStatEmoteType => "InqSkillStat_EmoteType",
            EmoteType::InqRawSkillStatEmoteType => "InqRawSkillStat_EmoteType",
            EmoteType::InqSkillTrainedEmoteType => "InqSkillTrained_EmoteType",
            EmoteType::InqSkillSpecializedEmoteType => "InqSkillSpecialized_EmoteType",
            EmoteType::AwardTrainingCreditsEmoteType => "AwardTrainingCredits_EmoteType",
            EmoteType::InflictVitaePenaltyEmoteType => "InflictVitaePenalty_EmoteType",
            EmoteType::AwardLevelProportionalXPEmoteType => "AwardLevelProportionalXP_EmoteType",
            EmoteType::AwardLevelProportionalSkillXPEmoteType => "AwardLevelProportionalSkillXP_EmoteType",
            EmoteType::InqEventEmoteType => "InqEvent_EmoteType",
            EmoteType::ForceMotionEmoteType => "ForceMotion_EmoteType",
            EmoteType::SetIntStatEmoteType => "SetIntStat_EmoteType",
            EmoteType::IncrementIntStatEmoteType => "IncrementIntStat_EmoteType",
            EmoteType::DecrementIntStatEmoteType => "DecrementIntStat_EmoteType",
            EmoteType::CreateTreasureEmoteType => "CreateTreasure_EmoteType",
            EmoteType::ResetHomePositionEmoteType => "ResetHomePosition_EmoteType",
            EmoteType::InqFellowQuestEmoteType => "InqFellowQuest_EmoteType",
            EmoteType::InqFellowNumEmoteType => "InqFellowNum_EmoteType",
            EmoteType::UpdateFellowQuestEmoteType => "UpdateFellowQuest_EmoteType",
            EmoteType::StampFellowQuestEmoteType => "StampFellowQuest_EmoteType",
            EmoteType::AwardNoShareXPEmoteType => "AwardNoShareXP_EmoteType",
            EmoteType::SetSanctuaryPositionEmoteType => "SetSanctuaryPosition_EmoteType",
            EmoteType::TellFellowEmoteType => "TellFellow_EmoteType",
            EmoteType::FellowBroadcastEmoteType => "FellowBroadcast_EmoteType",
            EmoteType::LockFellowEmoteType => "LockFellow_EmoteType",
            EmoteType::GotoEmoteType => "Goto_EmoteType",
            EmoteType::PopUpEmoteType => "PopUp_EmoteType",
            EmoteType::SetBoolStatEmoteType => "SetBoolStat_EmoteType",
            EmoteType::SetQuestCompletionsEmoteType => "SetQuestCompletions_EmoteType",
            EmoteType::InqNumCharacterTitlesEmoteType => "InqNumCharacterTitles_EmoteType",
            EmoteType::GenerateEmoteType => "Generate_EmoteType",
            EmoteType::PetCastSpellOnOwnerEmoteType => "PetCastSpellOnOwner_EmoteType",
            EmoteType::TakeItemsEmoteType => "TakeItems_EmoteType",
            EmoteType::InqYesNoEmoteType => "InqYesNo_EmoteType",
            EmoteType::InqOwnsItemsEmoteType => "InqOwnsItems_EmoteType",
            EmoteType::DeleteSelfEmoteType => "DeleteSelf_EmoteType",
            EmoteType::KillSelfEmoteType => "KillSelf_EmoteType",
            EmoteType::UpdateMyQuestEmoteType => "UpdateMyQuest_EmoteType",
            EmoteType::InqMyQuestEmoteType => "InqMyQuest_EmoteType",
            EmoteType::StampMyQuestEmoteType => "StampMyQuest_EmoteType",
            EmoteType::InqMyQuestSolvesEmoteType => "InqMyQuestSolves_EmoteType",
            EmoteType::EraseMyQuestEmoteType => "EraseMyQuest_EmoteType",
            EmoteType::DecrementMyQuestEmoteType => "DecrementMyQuest_EmoteType",
            EmoteType::IncrementMyQuestEmoteType => "IncrementMyQuest_EmoteType",
            EmoteType::SetMyQuestCompletionsEmoteType => "SetMyQuestCompletions_EmoteType",
            EmoteType::MoveToPosEmoteType => "MoveToPos_EmoteType",
            EmoteType::LocalSignalEmoteType => "LocalSignal_EmoteType",
            EmoteType::InqPackSpaceEmoteType => "InqPackSpace_EmoteType",
            EmoteType::RemoveVitaePenaltyEmoteType => "RemoveVitaePenalty_EmoteType",
            EmoteType::SetEyeTextureEmoteType => "SetEyeTexture_EmoteType",
            EmoteType::SetEyePaletteEmoteType => "SetEyePalette_EmoteType",
            EmoteType::SetNoseTextureEmoteType => "SetNoseTexture_EmoteType",
            EmoteType::SetNosePaletteEmoteType => "SetNosePalette_EmoteType",
            EmoteType::SetMouthTextureEmoteType => "SetMouthTexture_EmoteType",
            EmoteType::SetMouthPaletteEmoteType => "SetMouthPalette_EmoteType",
            EmoteType::SetHeadObjectEmoteType => "SetHeadObject_EmoteType",
            EmoteType::SetHeadPaletteEmoteType => "SetHeadPalette_EmoteType",
            EmoteType::TeleportTargetEmoteType => "TeleportTarget_EmoteType",
            EmoteType::TeleportSelfEmoteType => "TeleportSelf_EmoteType",
            EmoteType::StartBarberEmoteType => "StartBarber_EmoteType",
            EmoteType::InqQuestBitsOnEmoteType => "InqQuestBitsOn_EmoteType",
            EmoteType::InqQuestBitsOffEmoteType => "InqQuestBitsOff_EmoteType",
            EmoteType::InqMyQuestBitsOnEmoteType => "InqMyQuestBitsOn_EmoteType",
            EmoteType::InqMyQuestBitsOffEmoteType => "InqMyQuestBitsOff_EmoteType",
            EmoteType::SetQuestBitsOnEmoteType => "SetQuestBitsOn_EmoteType",
            EmoteType::SetQuestBitsOffEmoteType => "SetQuestBitsOff_EmoteType",
            EmoteType::SetMyQuestBitsOnEmoteType => "SetMyQuestBitsOn_EmoteType",
            EmoteType::SetMyQuestBitsOffEmoteType => "SetMyQuestBitsOff_EmoteType",
            EmoteType::UntrainSkillEmoteType => "UntrainSkill_EmoteType",
            EmoteType::SetAltRacialSkillsEmoteType => "SetAltRacialSkills_EmoteType",
            EmoteType::SpendLuminanceEmoteType => "SpendLuminance_EmoteType",
            EmoteType::AwardLuminanceEmoteType => "AwardLuminance_EmoteType",
            EmoteType::InqInt64StatEmoteType => "InqInt64Stat_EmoteType",
            EmoteType::SetInt64StatEmoteType => "SetInt64Stat_EmoteType",
            EmoteType::OpenMeEmoteType => "OpenMe_EmoteType",
            EmoteType::CloseMeEmoteType => "CloseMe_EmoteType",
            EmoteType::SetFloatStatEmoteType => "SetFloatStat_EmoteType",
            EmoteType::AddContractEmoteType => "AddContract_EmoteType",
            EmoteType::RemoveContractEmoteType => "RemoveContract_EmoteType",
            EmoteType::InqContractsFullEmoteType => "InqContractsFull_EmoteType",
        };
        write!(f, "{}", s)
    }
}

/// The EmoteCategory identifies the category of an emote.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum EmoteCategory {
    #[serde(rename = "Invalid_EmoteCategory")]
    InvalidEmoteCategory = 0x0,
    #[serde(rename = "Refuse_EmoteCategory")]
    RefuseEmoteCategory = 0x1,
    #[serde(rename = "Vendor_EmoteCategory")]
    VendorEmoteCategory = 0x2,
    #[serde(rename = "Death_EmoteCategory")]
    DeathEmoteCategory = 0x3,
    #[serde(rename = "Portal_EmoteCategory")]
    PortalEmoteCategory = 0x4,
    #[serde(rename = "HeartBeat_EmoteCategory")]
    HeartBeatEmoteCategory = 0x5,
    #[serde(rename = "Give_EmoteCategory")]
    GiveEmoteCategory = 0x6,
    #[serde(rename = "Use_EmoteCategory")]
    UseEmoteCategory = 0x7,
    #[serde(rename = "Activation_EmoteCategory")]
    ActivationEmoteCategory = 0x8,
    #[serde(rename = "Generation_EmoteCategory")]
    GenerationEmoteCategory = 0x9,
    #[serde(rename = "PickUp_EmoteCategory")]
    PickUpEmoteCategory = 0xA,
    #[serde(rename = "Drop_EmoteCategory")]
    DropEmoteCategory = 0xB,
    #[serde(rename = "QuestSuccess_EmoteCategory")]
    QuestSuccessEmoteCategory = 0xC,
    #[serde(rename = "QuestFailure_EmoteCategory")]
    QuestFailureEmoteCategory = 0xD,
    #[serde(rename = "Taunt_EmoteCategory")]
    TauntEmoteCategory = 0xE,
    #[serde(rename = "WoundedTaunt_EmoteCategory")]
    WoundedTauntEmoteCategory = 0xF,
    #[serde(rename = "KillTaunt_EmoteCategory")]
    KillTauntEmoteCategory = 0x10,
    #[serde(rename = "NewEnemy_EmoteCategory")]
    NewEnemyEmoteCategory = 0x11,
    #[serde(rename = "Scream_EmoteCategory")]
    ScreamEmoteCategory = 0x12,
    #[serde(rename = "Homesick_EmoteCategory")]
    HomesickEmoteCategory = 0x13,
    #[serde(rename = "ReceiveCritical_EmoteCategory")]
    ReceiveCriticalEmoteCategory = 0x14,
    #[serde(rename = "ResistSpell_EmoteCategory")]
    ResistSpellEmoteCategory = 0x15,
    #[serde(rename = "TestSuccess_EmoteCategory")]
    TestSuccessEmoteCategory = 0x16,
    #[serde(rename = "TestFailure_EmoteCategory")]
    TestFailureEmoteCategory = 0x17,
    #[serde(rename = "HearChat_EmoteCategory")]
    HearChatEmoteCategory = 0x18,
    #[serde(rename = "Wield_EmoteCategory")]
    WieldEmoteCategory = 0x19,
    #[serde(rename = "UnWield_EmoteCategory")]
    UnWieldEmoteCategory = 0x1A,
    #[serde(rename = "EventSuccess_EmoteCategory")]
    EventSuccessEmoteCategory = 0x1B,
    #[serde(rename = "EventFailure_EmoteCategory")]
    EventFailureEmoteCategory = 0x1C,
    #[serde(rename = "TestNoQuality_EmoteCategory")]
    TestNoQualityEmoteCategory = 0x1D,
    #[serde(rename = "QuestNoFellow_EmoteCategory")]
    QuestNoFellowEmoteCategory = 0x1E,
    #[serde(rename = "TestNoFellow_EmoteCategory")]
    TestNoFellowEmoteCategory = 0x1F,
    #[serde(rename = "GotoSet_EmoteCategory")]
    GotoSetEmoteCategory = 0x20,
    #[serde(rename = "NumFellowsSuccess_EmoteCategory")]
    NumFellowsSuccessEmoteCategory = 0x21,
    #[serde(rename = "NumFellowsFailure_EmoteCategory")]
    NumFellowsFailureEmoteCategory = 0x22,
    #[serde(rename = "NumCharacterTitlesSuccess_EmoteCategory")]
    NumCharacterTitlesSuccessEmoteCategory = 0x23,
    #[serde(rename = "NumCharacterTitlesFailure_EmoteCategory")]
    NumCharacterTitlesFailureEmoteCategory = 0x24,
    #[serde(rename = "ReceiveLocalSignal_EmoteCategory")]
    ReceiveLocalSignalEmoteCategory = 0x25,
    #[serde(rename = "ReceiveTalkDirect_EmoteCategory")]
    ReceiveTalkDirectEmoteCategory = 0x26,
}

impl crate::readers::ACDataType for EmoteCategory {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EmoteCategory::try_from(value)?)
    }
}

impl crate::writers::ACWritable for EmoteCategory {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for EmoteCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EmoteCategory::InvalidEmoteCategory => "Invalid_EmoteCategory",
            EmoteCategory::RefuseEmoteCategory => "Refuse_EmoteCategory",
            EmoteCategory::VendorEmoteCategory => "Vendor_EmoteCategory",
            EmoteCategory::DeathEmoteCategory => "Death_EmoteCategory",
            EmoteCategory::PortalEmoteCategory => "Portal_EmoteCategory",
            EmoteCategory::HeartBeatEmoteCategory => "HeartBeat_EmoteCategory",
            EmoteCategory::GiveEmoteCategory => "Give_EmoteCategory",
            EmoteCategory::UseEmoteCategory => "Use_EmoteCategory",
            EmoteCategory::ActivationEmoteCategory => "Activation_EmoteCategory",
            EmoteCategory::GenerationEmoteCategory => "Generation_EmoteCategory",
            EmoteCategory::PickUpEmoteCategory => "PickUp_EmoteCategory",
            EmoteCategory::DropEmoteCategory => "Drop_EmoteCategory",
            EmoteCategory::QuestSuccessEmoteCategory => "QuestSuccess_EmoteCategory",
            EmoteCategory::QuestFailureEmoteCategory => "QuestFailure_EmoteCategory",
            EmoteCategory::TauntEmoteCategory => "Taunt_EmoteCategory",
            EmoteCategory::WoundedTauntEmoteCategory => "WoundedTaunt_EmoteCategory",
            EmoteCategory::KillTauntEmoteCategory => "KillTaunt_EmoteCategory",
            EmoteCategory::NewEnemyEmoteCategory => "NewEnemy_EmoteCategory",
            EmoteCategory::ScreamEmoteCategory => "Scream_EmoteCategory",
            EmoteCategory::HomesickEmoteCategory => "Homesick_EmoteCategory",
            EmoteCategory::ReceiveCriticalEmoteCategory => "ReceiveCritical_EmoteCategory",
            EmoteCategory::ResistSpellEmoteCategory => "ResistSpell_EmoteCategory",
            EmoteCategory::TestSuccessEmoteCategory => "TestSuccess_EmoteCategory",
            EmoteCategory::TestFailureEmoteCategory => "TestFailure_EmoteCategory",
            EmoteCategory::HearChatEmoteCategory => "HearChat_EmoteCategory",
            EmoteCategory::WieldEmoteCategory => "Wield_EmoteCategory",
            EmoteCategory::UnWieldEmoteCategory => "UnWield_EmoteCategory",
            EmoteCategory::EventSuccessEmoteCategory => "EventSuccess_EmoteCategory",
            EmoteCategory::EventFailureEmoteCategory => "EventFailure_EmoteCategory",
            EmoteCategory::TestNoQualityEmoteCategory => "TestNoQuality_EmoteCategory",
            EmoteCategory::QuestNoFellowEmoteCategory => "QuestNoFellow_EmoteCategory",
            EmoteCategory::TestNoFellowEmoteCategory => "TestNoFellow_EmoteCategory",
            EmoteCategory::GotoSetEmoteCategory => "GotoSet_EmoteCategory",
            EmoteCategory::NumFellowsSuccessEmoteCategory => "NumFellowsSuccess_EmoteCategory",
            EmoteCategory::NumFellowsFailureEmoteCategory => "NumFellowsFailure_EmoteCategory",
            EmoteCategory::NumCharacterTitlesSuccessEmoteCategory => "NumCharacterTitlesSuccess_EmoteCategory",
            EmoteCategory::NumCharacterTitlesFailureEmoteCategory => "NumCharacterTitlesFailure_EmoteCategory",
            EmoteCategory::ReceiveLocalSignalEmoteCategory => "ReceiveLocalSignal_EmoteCategory",
            EmoteCategory::ReceiveTalkDirectEmoteCategory => "ReceiveTalkDirect_EmoteCategory",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    /// The CharacterOptions1 word contains character options.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CharacterOptions1: u32 {
        const AUTO_REPEAT_ATTACK = 0x2;
        const IGNORE_ALLEGIANCE_REQUESTS = 0x4;
        const IGNORE_FELLOWSHIP_REQUESTS = 0x8;
        const NOT_USED2 = 0x10;
        const NOT_USED3 = 0x20;
        const ALLOW_GIVE = 0x40;
        const VIEW_COMBAT_TARGET = 0x80;
        const SHOW_TOOLTIPS = 0x100;
        const USE_DECEPTION = 0x200;
        const TOGGLE_RUN = 0x400;
        const STAY_IN_CHAT_MODE = 0x800;
        const ADVANCED_COMBAT_UI = 0x1000;
        const AUTO_TARGET = 0x2000;
        const NOT_USED4 = 0x4000;
        const VIVID_TARGETING_INDICATOR = 0x8000;
        const DISABLE_MOST_WEATHER_EFFECTS = 0x10000;
        const IGNORE_TRADE_REQUESTS = 0x20000;
        const FELLOWSHIP_SHARE_XP = 0x40000;
        const ACCEPT_LOOT_PERMITS = 0x80000;
        const FELLOWSHIP_SHARE_LOOT = 0x100000;
        const SIDE_BY_SIDE_VITALS = 0x200000;
        const COORDINATES_ON_RADAR = 0x400000;
        const SPELL_DURATION = 0x800000;
        const NOT_USED5 = 0x1000000;
        const DISABLE_HOUSE_RESTRICTION_EFFECTS = 0x2000000;
        const DRAG_ITEM_ON_PLAYER_OPENS_SECURE_TRADE = 0x4000000;
        const DISPLAY_ALLEGIANCE_LOGON_NOTIFICATIONS = 0x8000000;
        const USE_CHARGE_ATTACK = 0x10000000;
        const AUTO_ACCEPT_FELLOW_REQUEST = 0x20000000;
        const HEAR_ALLEGIANCE_CHAT = 0x40000000;
        const USE_CRAFT_SUCCESS_DIALOG = 0x80000000;
    }
}

impl crate::readers::ACDataType for CharacterOptions1 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CharacterOptions1::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for CharacterOptions1 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The CharacterOptions2 word contains additional character options.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CharacterOptions2: u32 {
        const PERSISTENT_AT_DAY = 0x1;
        const DISPLAY_DATE_OF_BIRTH = 0x2;
        const DISPLAY_CHESS_RANK = 0x4;
        const DISPLAY_FISHING_SKILL = 0x8;
        const DISPLAY_NUMBER_DEATHS = 0x10;
        const DISPLAY_AGE = 0x20;
        const TIME_STAMP = 0x40;
        const SALVAGE_MULTIPLE = 0x80;
        const HEAR_GENERAL_CHAT = 0x100;
        const HEAR_TRADE_CHAT = 0x200;
        const HEAR_LFGCHAT = 0x400;
        const HEAR_ROLEPLAY_CHAT = 0x800;
        const APPEAR_OFFLINE = 0x1000;
        const DISPLAY_NUMBER_CHARACTER_TITLES = 0x2000;
        const MAIN_PACK_PREFERRED = 0x4000;
        const LEAD_MISSILE_TARGETS = 0x8000;
        const USE_FAST_MISSILES = 0x10000;
        const FILTER_LANGUAGE = 0x20000;
        const CONFIRM_VOLATILE_RARE_USE = 0x40000;
        const HEAR_SOCIETY_CHAT = 0x80000;
        const SHOW_HELM = 0x100000;
        const DISABLE_DISTANCE_FOG = 0x200000;
        const USE_MOUSE_TURNING = 0x400000;
        const SHOW_CLOAK = 0x800000;
        const LOCK_UI = 0x1000000;
        const HEAR_PKDEATH = 0x2000000;
    }
}

impl crate::readers::ACDataType for CharacterOptions2 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CharacterOptions2::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for CharacterOptions2 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The various options for filtering the spellbook
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SpellBookFilterOptions: u32 {
        const NONE = 0x0;
        const CREATURE = 0x1;
        const ITEM = 0x2;
        const LIFE = 0x4;
        const WAR = 0x8;
        const LEVEL1 = 0x10;
        const LEVEL2 = 0x20;
        const LEVEL3 = 0x40;
        const LEVEL4 = 0x80;
        const LEVEL5 = 0x100;
        const LEVEL6 = 0x200;
        const LEVEL7 = 0x400;
        const LEVEL8 = 0x800;
        const LEVEL9 = 0x1000;
        const VOID = 0x2000;
    }
}

impl crate::readers::ACDataType for SpellBookFilterOptions {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(SpellBookFilterOptions::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for SpellBookFilterOptions {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The EquipMask value describes the equipment slots an item uses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EquipMask: u32 {
        const HEAD = 0x1;
        const CHEST_UNDERWEAR = 0x2;
        const ABDOMEN_UNDERWEAR = 0x4;
        const UPPER_ARMS_UNDERWEAR = 0x8;
        const LOWER_ARMS_UNDERWEAR = 0x10;
        const HANDS = 0x20;
        const UPPER_LEGS_UNDERWEAR = 0x40;
        const LOWER_LEGS_UNDERWEAR = 0x80;
        const FEET = 0x100;
        const CHEST = 0x200;
        const ABDOMEN = 0x400;
        const UPPER_ARMS = 0x800;
        const LOWER_ARMS = 0x1000;
        const UPPER_LEGS = 0x2000;
        const LOWER_LEGS = 0x4000;
        const NECKLACE = 0x8000;
        const RIGHT_BRACELET = 0x10000;
        const LEFT_BRACELET = 0x20000;
        const RIGHT_RING = 0x40000;
        const LEFT_RING = 0x80000;
        const MELEE_WEAPON = 0x100000;
        const SHIELD = 0x200000;
        const MISSILE_WEAPON = 0x400000;
        const AMMUNITION = 0x800000;
        const WAND = 0x1000000;
    }
}

impl crate::readers::ACDataType for EquipMask {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EquipMask::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for EquipMask {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The type of the friend change event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FriendsUpdateType: u32 {
        const FULL = 0x0;
        const ADDED = 0x1;
        const REMOVED = 0x2;
        const LOGIN_CHANGE = 0x4;
    }
}

impl crate::readers::ACDataType for FriendsUpdateType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(FriendsUpdateType::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for FriendsUpdateType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

/// The permission levels that can be given to an allegiance officer
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AllegianceOfficerLevel {
    Speaker = 0x1,
    Seneschal = 0x2,
    Castellan = 0x3,
}

impl crate::readers::ACDataType for AllegianceOfficerLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AllegianceOfficerLevel::try_from(value)?)
    }
}

impl crate::writers::ACWritable for AllegianceOfficerLevel {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for AllegianceOfficerLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AllegianceOfficerLevel::Speaker => "Speaker",
            AllegianceOfficerLevel::Seneschal => "Seneschal",
            AllegianceOfficerLevel::Castellan => "Castellan",
        };
        write!(f, "{}", s)
    }
}

/// Actions related to /allegiance lock
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AllegianceLockAction {
    LockedOff = 0x1,
    LockedOn = 0x2,
    ToggleLocked = 0x3,
    CheckLocked = 0x4,
    DisplayBypass = 0x5,
    ClearBypass = 0x6,
}

impl crate::readers::ACDataType for AllegianceLockAction {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AllegianceLockAction::try_from(value)?)
    }
}

impl crate::writers::ACWritable for AllegianceLockAction {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for AllegianceLockAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AllegianceLockAction::LockedOff => "LockedOff",
            AllegianceLockAction::LockedOn => "LockedOn",
            AllegianceLockAction::ToggleLocked => "ToggleLocked",
            AllegianceLockAction::CheckLocked => "CheckLocked",
            AllegianceLockAction::DisplayBypass => "DisplayBypass",
            AllegianceLockAction::ClearBypass => "ClearBypass",
        };
        write!(f, "{}", s)
    }
}

/// Actions related to /allegiance house
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AllegianceHouseAction {
    Help = 0x1,
    GuestOpen = 0x2,
    GuestClosed = 0x3,
    StorageOpen = 0x4,
    StorageClosed = 0x5,
}

impl crate::readers::ACDataType for AllegianceHouseAction {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AllegianceHouseAction::try_from(value)?)
    }
}

impl crate::writers::ACWritable for AllegianceHouseAction {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for AllegianceHouseAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AllegianceHouseAction::Help => "Help",
            AllegianceHouseAction::GuestOpen => "GuestOpen",
            AllegianceHouseAction::GuestClosed => "GuestClosed",
            AllegianceHouseAction::StorageOpen => "StorageOpen",
            AllegianceHouseAction::StorageClosed => "StorageClosed",
        };
        write!(f, "{}", s)
    }
}

/// The AttributeId identifies a specific Character attribute.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AttributeId {
    Strength = 0x1,
    Endurance = 0x2,
    Quickness = 0x3,
    Coordination = 0x4,
    Focus = 0x5,
    #[serde(rename = "Self")]
    Self_ = 0x6,
}

impl crate::readers::ACDataType for AttributeId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AttributeId::try_from(value)?)
    }
}

impl crate::writers::ACWritable for AttributeId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for AttributeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AttributeId::Strength => "Strength",
            AttributeId::Endurance => "Endurance",
            AttributeId::Quickness => "Quickness",
            AttributeId::Coordination => "Coordination",
            AttributeId::Focus => "Focus",
            AttributeId::Self_ => "Self",
        };
        write!(f, "{}", s)
    }
}

/// The VitalId identifies a specific Character vital (secondary attribute).
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum VitalId {
    MaximumHealth = 0x1,
    MaximumStamina = 0x3,
    MaximumMana = 0x5,
}

impl crate::readers::ACDataType for VitalId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(VitalId::try_from(value)?)
    }
}

impl crate::writers::ACWritable for VitalId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for VitalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            VitalId::MaximumHealth => "MaximumHealth",
            VitalId::MaximumStamina => "MaximumStamina",
            VitalId::MaximumMana => "MaximumMana",
        };
        write!(f, "{}", s)
    }
}

/// The CurVitalId identifies a specific Character vital (secondary attribute).
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CurVitalId {
    CurrentHealth = 0x2,
    CurrentStamina = 0x4,
    CurrentMana = 0x6,
}

impl crate::readers::ACDataType for CurVitalId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CurVitalId::try_from(value)?)
    }
}

impl crate::writers::ACWritable for CurVitalId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for CurVitalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CurVitalId::CurrentHealth => "CurrentHealth",
            CurVitalId::CurrentStamina => "CurrentStamina",
            CurVitalId::CurrentMana => "CurrentMana",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    /// The combat mode for a character or monster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CombatMode: u32 {
        const NON_COMBAT = 0x1;
        const MELEE = 0x2;
        const MISSLE = 0x4;
        const MAGIC = 0x8;
    }
}

impl crate::readers::ACDataType for CombatMode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CombatMode::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for CombatMode {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum Sound {
    Invalid = 0x0,
    Speak1 = 0x1,
    Random = 0x2,
    Attack1 = 0x3,
    Attack2 = 0x4,
    Attack3 = 0x5,
    SpecialAttack1 = 0x6,
    SpecialAttack2 = 0x7,
    SpecialAttack3 = 0x8,
    Damage1 = 0x9,
    Damage2 = 0xA,
    Damage3 = 0xB,
    Wound1 = 0xC,
    Wound2 = 0xD,
    Wound3 = 0xE,
    Death1 = 0xF,
    Death2 = 0x10,
    Death3 = 0x11,
    Grunt1 = 0x12,
    Grunt2 = 0x13,
    Grunt3 = 0x14,
    Oh1 = 0x15,
    Oh2 = 0x16,
    Oh3 = 0x17,
    Heave1 = 0x18,
    Heave2 = 0x19,
    Heave3 = 0x1A,
    Knockdown1 = 0x1B,
    Knockdown2 = 0x1C,
    Knockdown3 = 0x1D,
    Swoosh1 = 0x1E,
    Swoosh2 = 0x1F,
    Swoosh3 = 0x20,
    Thump1 = 0x21,
    Smash1 = 0x22,
    Scratch1 = 0x23,
    Spear = 0x24,
    Sling = 0x25,
    Dagger = 0x26,
    ArrowWhiz1 = 0x27,
    ArrowWhiz2 = 0x28,
    CrossbowPull = 0x29,
    CrossbowRelease = 0x2A,
    BowPull = 0x2B,
    BowRelease = 0x2C,
    ThrownWeaponRelease1 = 0x2D,
    ArrowLand = 0x2E,
    Collision = 0x2F,
    HitFlesh1 = 0x30,
    HitLeather1 = 0x31,
    HitChain1 = 0x32,
    HitPlate1 = 0x33,
    HitMissile1 = 0x34,
    HitMissile2 = 0x35,
    HitMissile3 = 0x36,
    Footstep1 = 0x37,
    Footstep2 = 0x38,
    Walk1 = 0x39,
    Dance1 = 0x3A,
    Dance2 = 0x3B,
    Dance3 = 0x3C,
    Hidden1 = 0x3D,
    Hidden2 = 0x3E,
    Hidden3 = 0x3F,
    Eat1 = 0x40,
    Drink1 = 0x41,
    Open = 0x42,
    Close = 0x43,
    OpenSlam = 0x44,
    CloseSlam = 0x45,
    Ambient1 = 0x46,
    Ambient2 = 0x47,
    Ambient3 = 0x48,
    Ambient4 = 0x49,
    Ambient5 = 0x4A,
    Ambient6 = 0x4B,
    Ambient7 = 0x4C,
    Ambient8 = 0x4D,
    Waterfall = 0x4E,
    LogOut = 0x4F,
    LogIn = 0x50,
    LifestoneOn = 0x51,
    AttribUp = 0x52,
    AttribDown = 0x53,
    SkillUp = 0x54,
    SkillDown = 0x55,
    HealthUp = 0x56,
    HealthDown = 0x57,
    ShieldUp = 0x58,
    ShieldDown = 0x59,
    EnchantUp = 0x5A,
    EnchantDown = 0x5B,
    VisionUp = 0x5C,
    VisionDown = 0x5D,
    Fizzle = 0x5E,
    Launch = 0x5F,
    Explode = 0x60,
    TransUp = 0x61,
    TransDown = 0x62,
    BreatheFlaem = 0x63,
    BreatheAcid = 0x64,
    BreatheFrost = 0x65,
    BreatheLightning = 0x66,
    Create = 0x67,
    Destroy = 0x68,
    Lockpicking = 0x69,
    #[serde(rename = "UI_EnterPortal")]
    UIEnterPortal = 0x6A,
    #[serde(rename = "UI_ExitPortal")]
    UIExitPortal = 0x6B,
    #[serde(rename = "UI_GeneralQuery")]
    UIGeneralQuery = 0x6C,
    #[serde(rename = "UI_GeneralError")]
    UIGeneralError = 0x6D,
    #[serde(rename = "UI_TransientMessage")]
    UITransientMessage = 0x6E,
    #[serde(rename = "UI_IconPickUp")]
    UIIconPickUp = 0x6F,
    #[serde(rename = "UI_IconSuccessfulDrop")]
    UIIconSuccessfulDrop = 0x70,
    #[serde(rename = "UI_IconInvalid_Drop")]
    UIIconInvalidDrop = 0x71,
    #[serde(rename = "UI_ButtonPress")]
    UIButtonPress = 0x72,
    #[serde(rename = "UI_GrabSlider")]
    UIGrabSlider = 0x73,
    #[serde(rename = "UI_ReleaseSlider")]
    UIReleaseSlider = 0x74,
    #[serde(rename = "UI_NewTargetSelected")]
    UINewTargetSelected = 0x75,
    #[serde(rename = "UI_Roar")]
    UIRoar = 0x76,
    #[serde(rename = "UI_Bell")]
    UIBell = 0x77,
    #[serde(rename = "UI_Chant1")]
    UIChant1 = 0x78,
    #[serde(rename = "UI_Chant2")]
    UIChant2 = 0x79,
    #[serde(rename = "UI_DarkWhispers1")]
    UIDarkWhispers1 = 0x7A,
    #[serde(rename = "UI_DarkWhispers2")]
    UIDarkWhispers2 = 0x7B,
    #[serde(rename = "UI_DarkLaugh")]
    UIDarkLaugh = 0x7C,
    #[serde(rename = "UI_DarkWind")]
    UIDarkWind = 0x7D,
    #[serde(rename = "UI_DarkSpeech")]
    UIDarkSpeech = 0x7E,
    #[serde(rename = "UI_Drums")]
    UIDrums = 0x7F,
    #[serde(rename = "UI_GhostSpeak")]
    UIGhostSpeak = 0x80,
    #[serde(rename = "UI_Breathing")]
    UIBreathing = 0x81,
    #[serde(rename = "UI_Howl")]
    UIHowl = 0x82,
    #[serde(rename = "UI_LostSouls")]
    UILostSouls = 0x83,
    #[serde(rename = "UI_Squeal")]
    UISqueal = 0x84,
    #[serde(rename = "UI_Thunder1")]
    UIThunder1 = 0x85,
    #[serde(rename = "UI_Thunder2")]
    UIThunder2 = 0x86,
    #[serde(rename = "UI_Thunder3")]
    UIThunder3 = 0x87,
    #[serde(rename = "UI_Thunder4")]
    UIThunder4 = 0x88,
    #[serde(rename = "UI_Thunder5")]
    UIThunder5 = 0x89,
    #[serde(rename = "UI_Thunder6")]
    UIThunder6 = 0x8A,
    RaiseTrait = 0x8B,
    WieldObject = 0x8C,
    UnwieldObject = 0x8D,
    ReceiveItem = 0x8E,
    PickUpItem = 0x8F,
    DropItem = 0x90,
    ResistSpell = 0x91,
    PicklockFail = 0x92,
    LockSuccess = 0x93,
    OpenFailDueToLock = 0x94,
    TriggerActivated = 0x95,
    SpellExpire = 0x96,
    ItemManaDepleted = 0x97,
    TriggerActivated1 = 0x98,
    TriggerActivated2 = 0x99,
    TriggerActivated3 = 0x9A,
    TriggerActivated4 = 0x9B,
    TriggerActivated5 = 0x9C,
    TriggerActivated6 = 0x9D,
    TriggerActivated7 = 0x9E,
    TriggerActivated8 = 0x9F,
    TriggerActivated9 = 0xA0,
    TriggerActivated10 = 0xA1,
    TriggerActivated11 = 0xA2,
    TriggerActivated12 = 0xA3,
    TriggerActivated13 = 0xA4,
    TriggerActivated14 = 0xA5,
    TriggerActivated15 = 0xA6,
    TriggerActivated16 = 0xA7,
    TriggerActivated17 = 0xA8,
    TriggerActivated18 = 0xA9,
    TriggerActivated19 = 0xAA,
    TriggerActivated20 = 0xAB,
    TriggerActivated21 = 0xAC,
    TriggerActivated22 = 0xAD,
    TriggerActivated23 = 0xAE,
    TriggerActivated24 = 0xAF,
    TriggerActivated25 = 0xB0,
    TriggerActivated26 = 0xB1,
    TriggerActivated27 = 0xB2,
    TriggerActivated28 = 0xB3,
    TriggerActivated29 = 0xB4,
    TriggerActivated30 = 0xB5,
    TriggerActivated31 = 0xB6,
    TriggerActivated32 = 0xB7,
    TriggerActivated33 = 0xB8,
    TriggerActivated34 = 0xB9,
    TriggerActivated35 = 0xBA,
    TriggerActivated36 = 0xBB,
    TriggerActivated37 = 0xBC,
    TriggerActivated38 = 0xBD,
    TriggerActivated39 = 0xBE,
    TriggerActivated40 = 0xBF,
    TriggerActivated41 = 0xC0,
    TriggerActivated42 = 0xC1,
    TriggerActivated43 = 0xC2,
    TriggerActivated44 = 0xC3,
    TriggerActivated45 = 0xC4,
    TriggerActivated46 = 0xC5,
    TriggerActivated47 = 0xC6,
    TriggerActivated48 = 0xC7,
    TriggerActivated49 = 0xC8,
    TriggerActivated50 = 0xC9,
    HealthDownVoid = 0xCA,
    RegenDownVoid = 0xCB,
    SkillDownVoid = 0xCC,
}

impl crate::readers::ACDataType for Sound {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(Sound::try_from(value)?)
    }
}

impl crate::writers::ACWritable for Sound {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_i32(writer, self.clone() as i32)?;
        Ok(())
    }
}

impl std::fmt::Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Sound::Invalid => "Invalid",
            Sound::Speak1 => "Speak1",
            Sound::Random => "Random",
            Sound::Attack1 => "Attack1",
            Sound::Attack2 => "Attack2",
            Sound::Attack3 => "Attack3",
            Sound::SpecialAttack1 => "SpecialAttack1",
            Sound::SpecialAttack2 => "SpecialAttack2",
            Sound::SpecialAttack3 => "SpecialAttack3",
            Sound::Damage1 => "Damage1",
            Sound::Damage2 => "Damage2",
            Sound::Damage3 => "Damage3",
            Sound::Wound1 => "Wound1",
            Sound::Wound2 => "Wound2",
            Sound::Wound3 => "Wound3",
            Sound::Death1 => "Death1",
            Sound::Death2 => "Death2",
            Sound::Death3 => "Death3",
            Sound::Grunt1 => "Grunt1",
            Sound::Grunt2 => "Grunt2",
            Sound::Grunt3 => "Grunt3",
            Sound::Oh1 => "Oh1",
            Sound::Oh2 => "Oh2",
            Sound::Oh3 => "Oh3",
            Sound::Heave1 => "Heave1",
            Sound::Heave2 => "Heave2",
            Sound::Heave3 => "Heave3",
            Sound::Knockdown1 => "Knockdown1",
            Sound::Knockdown2 => "Knockdown2",
            Sound::Knockdown3 => "Knockdown3",
            Sound::Swoosh1 => "Swoosh1",
            Sound::Swoosh2 => "Swoosh2",
            Sound::Swoosh3 => "Swoosh3",
            Sound::Thump1 => "Thump1",
            Sound::Smash1 => "Smash1",
            Sound::Scratch1 => "Scratch1",
            Sound::Spear => "Spear",
            Sound::Sling => "Sling",
            Sound::Dagger => "Dagger",
            Sound::ArrowWhiz1 => "ArrowWhiz1",
            Sound::ArrowWhiz2 => "ArrowWhiz2",
            Sound::CrossbowPull => "CrossbowPull",
            Sound::CrossbowRelease => "CrossbowRelease",
            Sound::BowPull => "BowPull",
            Sound::BowRelease => "BowRelease",
            Sound::ThrownWeaponRelease1 => "ThrownWeaponRelease1",
            Sound::ArrowLand => "ArrowLand",
            Sound::Collision => "Collision",
            Sound::HitFlesh1 => "HitFlesh1",
            Sound::HitLeather1 => "HitLeather1",
            Sound::HitChain1 => "HitChain1",
            Sound::HitPlate1 => "HitPlate1",
            Sound::HitMissile1 => "HitMissile1",
            Sound::HitMissile2 => "HitMissile2",
            Sound::HitMissile3 => "HitMissile3",
            Sound::Footstep1 => "Footstep1",
            Sound::Footstep2 => "Footstep2",
            Sound::Walk1 => "Walk1",
            Sound::Dance1 => "Dance1",
            Sound::Dance2 => "Dance2",
            Sound::Dance3 => "Dance3",
            Sound::Hidden1 => "Hidden1",
            Sound::Hidden2 => "Hidden2",
            Sound::Hidden3 => "Hidden3",
            Sound::Eat1 => "Eat1",
            Sound::Drink1 => "Drink1",
            Sound::Open => "Open",
            Sound::Close => "Close",
            Sound::OpenSlam => "OpenSlam",
            Sound::CloseSlam => "CloseSlam",
            Sound::Ambient1 => "Ambient1",
            Sound::Ambient2 => "Ambient2",
            Sound::Ambient3 => "Ambient3",
            Sound::Ambient4 => "Ambient4",
            Sound::Ambient5 => "Ambient5",
            Sound::Ambient6 => "Ambient6",
            Sound::Ambient7 => "Ambient7",
            Sound::Ambient8 => "Ambient8",
            Sound::Waterfall => "Waterfall",
            Sound::LogOut => "LogOut",
            Sound::LogIn => "LogIn",
            Sound::LifestoneOn => "LifestoneOn",
            Sound::AttribUp => "AttribUp",
            Sound::AttribDown => "AttribDown",
            Sound::SkillUp => "SkillUp",
            Sound::SkillDown => "SkillDown",
            Sound::HealthUp => "HealthUp",
            Sound::HealthDown => "HealthDown",
            Sound::ShieldUp => "ShieldUp",
            Sound::ShieldDown => "ShieldDown",
            Sound::EnchantUp => "EnchantUp",
            Sound::EnchantDown => "EnchantDown",
            Sound::VisionUp => "VisionUp",
            Sound::VisionDown => "VisionDown",
            Sound::Fizzle => "Fizzle",
            Sound::Launch => "Launch",
            Sound::Explode => "Explode",
            Sound::TransUp => "TransUp",
            Sound::TransDown => "TransDown",
            Sound::BreatheFlaem => "BreatheFlaem",
            Sound::BreatheAcid => "BreatheAcid",
            Sound::BreatheFrost => "BreatheFrost",
            Sound::BreatheLightning => "BreatheLightning",
            Sound::Create => "Create",
            Sound::Destroy => "Destroy",
            Sound::Lockpicking => "Lockpicking",
            Sound::UIEnterPortal => "UI_EnterPortal",
            Sound::UIExitPortal => "UI_ExitPortal",
            Sound::UIGeneralQuery => "UI_GeneralQuery",
            Sound::UIGeneralError => "UI_GeneralError",
            Sound::UITransientMessage => "UI_TransientMessage",
            Sound::UIIconPickUp => "UI_IconPickUp",
            Sound::UIIconSuccessfulDrop => "UI_IconSuccessfulDrop",
            Sound::UIIconInvalidDrop => "UI_IconInvalid_Drop",
            Sound::UIButtonPress => "UI_ButtonPress",
            Sound::UIGrabSlider => "UI_GrabSlider",
            Sound::UIReleaseSlider => "UI_ReleaseSlider",
            Sound::UINewTargetSelected => "UI_NewTargetSelected",
            Sound::UIRoar => "UI_Roar",
            Sound::UIBell => "UI_Bell",
            Sound::UIChant1 => "UI_Chant1",
            Sound::UIChant2 => "UI_Chant2",
            Sound::UIDarkWhispers1 => "UI_DarkWhispers1",
            Sound::UIDarkWhispers2 => "UI_DarkWhispers2",
            Sound::UIDarkLaugh => "UI_DarkLaugh",
            Sound::UIDarkWind => "UI_DarkWind",
            Sound::UIDarkSpeech => "UI_DarkSpeech",
            Sound::UIDrums => "UI_Drums",
            Sound::UIGhostSpeak => "UI_GhostSpeak",
            Sound::UIBreathing => "UI_Breathing",
            Sound::UIHowl => "UI_Howl",
            Sound::UILostSouls => "UI_LostSouls",
            Sound::UISqueal => "UI_Squeal",
            Sound::UIThunder1 => "UI_Thunder1",
            Sound::UIThunder2 => "UI_Thunder2",
            Sound::UIThunder3 => "UI_Thunder3",
            Sound::UIThunder4 => "UI_Thunder4",
            Sound::UIThunder5 => "UI_Thunder5",
            Sound::UIThunder6 => "UI_Thunder6",
            Sound::RaiseTrait => "RaiseTrait",
            Sound::WieldObject => "WieldObject",
            Sound::UnwieldObject => "UnwieldObject",
            Sound::ReceiveItem => "ReceiveItem",
            Sound::PickUpItem => "PickUpItem",
            Sound::DropItem => "DropItem",
            Sound::ResistSpell => "ResistSpell",
            Sound::PicklockFail => "PicklockFail",
            Sound::LockSuccess => "LockSuccess",
            Sound::OpenFailDueToLock => "OpenFailDueToLock",
            Sound::TriggerActivated => "TriggerActivated",
            Sound::SpellExpire => "SpellExpire",
            Sound::ItemManaDepleted => "ItemManaDepleted",
            Sound::TriggerActivated1 => "TriggerActivated1",
            Sound::TriggerActivated2 => "TriggerActivated2",
            Sound::TriggerActivated3 => "TriggerActivated3",
            Sound::TriggerActivated4 => "TriggerActivated4",
            Sound::TriggerActivated5 => "TriggerActivated5",
            Sound::TriggerActivated6 => "TriggerActivated6",
            Sound::TriggerActivated7 => "TriggerActivated7",
            Sound::TriggerActivated8 => "TriggerActivated8",
            Sound::TriggerActivated9 => "TriggerActivated9",
            Sound::TriggerActivated10 => "TriggerActivated10",
            Sound::TriggerActivated11 => "TriggerActivated11",
            Sound::TriggerActivated12 => "TriggerActivated12",
            Sound::TriggerActivated13 => "TriggerActivated13",
            Sound::TriggerActivated14 => "TriggerActivated14",
            Sound::TriggerActivated15 => "TriggerActivated15",
            Sound::TriggerActivated16 => "TriggerActivated16",
            Sound::TriggerActivated17 => "TriggerActivated17",
            Sound::TriggerActivated18 => "TriggerActivated18",
            Sound::TriggerActivated19 => "TriggerActivated19",
            Sound::TriggerActivated20 => "TriggerActivated20",
            Sound::TriggerActivated21 => "TriggerActivated21",
            Sound::TriggerActivated22 => "TriggerActivated22",
            Sound::TriggerActivated23 => "TriggerActivated23",
            Sound::TriggerActivated24 => "TriggerActivated24",
            Sound::TriggerActivated25 => "TriggerActivated25",
            Sound::TriggerActivated26 => "TriggerActivated26",
            Sound::TriggerActivated27 => "TriggerActivated27",
            Sound::TriggerActivated28 => "TriggerActivated28",
            Sound::TriggerActivated29 => "TriggerActivated29",
            Sound::TriggerActivated30 => "TriggerActivated30",
            Sound::TriggerActivated31 => "TriggerActivated31",
            Sound::TriggerActivated32 => "TriggerActivated32",
            Sound::TriggerActivated33 => "TriggerActivated33",
            Sound::TriggerActivated34 => "TriggerActivated34",
            Sound::TriggerActivated35 => "TriggerActivated35",
            Sound::TriggerActivated36 => "TriggerActivated36",
            Sound::TriggerActivated37 => "TriggerActivated37",
            Sound::TriggerActivated38 => "TriggerActivated38",
            Sound::TriggerActivated39 => "TriggerActivated39",
            Sound::TriggerActivated40 => "TriggerActivated40",
            Sound::TriggerActivated41 => "TriggerActivated41",
            Sound::TriggerActivated42 => "TriggerActivated42",
            Sound::TriggerActivated43 => "TriggerActivated43",
            Sound::TriggerActivated44 => "TriggerActivated44",
            Sound::TriggerActivated45 => "TriggerActivated45",
            Sound::TriggerActivated46 => "TriggerActivated46",
            Sound::TriggerActivated47 => "TriggerActivated47",
            Sound::TriggerActivated48 => "TriggerActivated48",
            Sound::TriggerActivated49 => "TriggerActivated49",
            Sound::TriggerActivated50 => "TriggerActivated50",
            Sound::HealthDownVoid => "HealthDownVoid",
            Sound::RegenDownVoid => "RegenDownVoid",
            Sound::SkillDownVoid => "SkillDownVoid",
        };
        write!(f, "{}", s)
    }
}

/// The ChatFragmentType categorizes chat window messages to control color and filtering.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ChatFragmentType {
    Default = 0x0,
    Speech = 0x2,
    Tell = 0x3,
    OutgoingTell = 0x4,
    System = 0x5,
    Combat = 0x6,
    Magic = 0x7,
    Channels = 0x8,
    OutgoingChannel = 0x9,
    Social = 0xA,
    OutgoingSocial = 0xB,
    Emote = 0xC,
    Advancement = 0xD,
    Abuse = 0xE,
    Help = 0xF,
    Appraisal = 0x10,
    Spellcasting = 0x11,
    Allegiance = 0x12,
    Fellowship = 0x13,
    WorldBroadcast = 0x14,
    CombatEnemy = 0x15,
    CombatSelf = 0x16,
    Recall = 0x17,
    Craft = 0x18,
    Salvaging = 0x19,
    AdminTell = 0x1F,
}

impl crate::readers::ACDataType for ChatFragmentType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ChatFragmentType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ChatFragmentType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ChatFragmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ChatFragmentType::Default => "Default",
            ChatFragmentType::Speech => "Speech",
            ChatFragmentType::Tell => "Tell",
            ChatFragmentType::OutgoingTell => "OutgoingTell",
            ChatFragmentType::System => "System",
            ChatFragmentType::Combat => "Combat",
            ChatFragmentType::Magic => "Magic",
            ChatFragmentType::Channels => "Channels",
            ChatFragmentType::OutgoingChannel => "OutgoingChannel",
            ChatFragmentType::Social => "Social",
            ChatFragmentType::OutgoingSocial => "OutgoingSocial",
            ChatFragmentType::Emote => "Emote",
            ChatFragmentType::Advancement => "Advancement",
            ChatFragmentType::Abuse => "Abuse",
            ChatFragmentType::Help => "Help",
            ChatFragmentType::Appraisal => "Appraisal",
            ChatFragmentType::Spellcasting => "Spellcasting",
            ChatFragmentType::Allegiance => "Allegiance",
            ChatFragmentType::Fellowship => "Fellowship",
            ChatFragmentType::WorldBroadcast => "WorldBroadcast",
            ChatFragmentType::CombatEnemy => "CombatEnemy",
            ChatFragmentType::CombatSelf => "CombatSelf",
            ChatFragmentType::Recall => "Recall",
            ChatFragmentType::Craft => "Craft",
            ChatFragmentType::Salvaging => "Salvaging",
            ChatFragmentType::AdminTell => "AdminTell",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    /// Flags related to the use of the item.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ObjectDescriptionFlag: u32 {
        const OPENABLE = 0x1;
        const INSCRIBABLE = 0x2;
        const STUCK = 0x4;
        const PLAYER = 0x8;
        const ATTACKABLE = 0x10;
        const PLAYER_KILLER = 0x20;
        const HIDDEN_ADMIN = 0x40;
        const UI_HIDDEN = 0x80;
        const BOOK = 0x100;
        const VENDOR = 0x200;
        const PK_SWITCH = 0x400;
        const NPK_SWITCH = 0x800;
        const DOOR = 0x1000;
        const CORPSE = 0x2000;
        const LIFE_STONE = 0x4000;
        const FOOD = 0x8000;
        const HEALER = 0x10000;
        const LOCKPICK = 0x20000;
        const PORTAL = 0x40000;
        const ADMIN = 0x100000;
        const FREE_PK_STATUS = 0x200000;
        const IMMUNE_CELL_RESTRICTIONS = 0x400000;
        const REQUIRES_PACK_SLOT = 0x800000;
        const RETAINED = 0x1000000;
        const PK_LITE_STATUS = 0x2000000;
        const INCLUDES_SECOND_HEADER = 0x4000000;
        const BIND_STONE = 0x8000000;
        const VOLATILE_RARE = 0x10000000;
        const WIELD_ON_USE = 0x20000000;
        const WIELD_LEFT = 0x40000000;
    }
}

impl crate::readers::ACDataType for ObjectDescriptionFlag {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ObjectDescriptionFlag::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for ObjectDescriptionFlag {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The AmmoType value describes the type of ammunition a missile weapon uses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AmmoType: u16 {
        const THROWN_WEAPON = 0x0;
        const ARROW = 0x1;
        const BOLT = 0x2;
        const DART = 0x4;
    }
}

impl crate::readers::ACDataType for AmmoType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(AmmoType::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for AmmoType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The useablilty flags of the object
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Usable: u32 {
        const SOURCE_UNUSABLE = 0x1;
        const SOURCE_SELF = 0x2;
        const SOURCE_WIELDED = 0x4;
        const SOURCE_CONTAINED = 0x8;
        const SOURCE_VIEWED = 0x10;
        const SOURCE_REMOTE = 0x20;
        const SOURCE_NO_APPROACH = 0x40;
        const SOURCE_OBJECT_SELF = 0x80;
        const TARGET_UNUSABLE = 0x10000;
        const TARGET_SELF = 0x20000;
        const TARGET_WIELDED = 0x40000;
        const TARGET_CONTAINED = 0x80000;
        const TARGET_VIEWED = 0x100000;
        const TARGET_REMOTE = 0x200000;
        const TARGET_NO_APPROACH = 0x400000;
        const TARGET_OBJECT_SELF = 0x800000;
    }
}

impl crate::readers::ACDataType for Usable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(Usable::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for Usable {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The CoverageMask value describes what parts of the body an item protects.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CoverageMask: u32 {
        const UPPER_LEGS_UNDERWEAR = 0x2;
        const LOWER_LEGS_UNDERWEAR = 0x4;
        const CHEST_UNDERWEAR = 0x8;
        const ABDOMEN_UNDERWEAR = 0x10;
        const UPPER_ARMS_UNDERWEAR = 0x20;
        const LOWER_ARMS_UNDERWEAR = 0x40;
        const UPPER_LEGS = 0x100;
        const LOWER_LEGS = 0x200;
        const CHEST = 0x400;
        const ABDOMEN = 0x800;
        const UPPER_ARMS = 0x1000;
        const LOWER_ARMS = 0x2000;
        const HEAD = 0x4000;
        const HANDS = 0x8000;
        const FEET = 0x10000;
    }
}

impl crate::readers::ACDataType for CoverageMask {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CoverageMask::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for CoverageMask {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The HookType identifies the types of dwelling hooks.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct HookType: u16 {
        const FLOOR = 0x1;
        const WALL = 0x2;
        const CEILING = 0x4;
        const YARD = 0x8;
        const ROOF = 0x10;
    }
}

impl crate::readers::ACDataType for HookType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(HookType::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for HookType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.bits())?;
        Ok(())
    }
}

/// The MaterialType identifies the material an object is made of.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum MaterialType {
    Ceramic = 0x1,
    Porcelain = 0x2,
    Linen = 0x4,
    Satin = 0x5,
    Silk = 0x6,
    Velvet = 0x7,
    Wool = 0x8,
    Agate = 0xA,
    Amber = 0xB,
    Amethyst = 0xC,
    Aquamarine = 0xD,
    Azurite = 0xE,
    BlackGarnet = 0xF,
    BlackOpal = 0x10,
    Bloodstone = 0x11,
    Carnelian = 0x12,
    Citrine = 0x13,
    Diamond = 0x14,
    Emerald = 0x15,
    FireOpal = 0x16,
    GreenGarnet = 0x17,
    GreenJade = 0x18,
    Hematite = 0x19,
    ImperialTopaz = 0x1A,
    Jet = 0x1B,
    LapisLazuli = 0x1C,
    LavenderJade = 0x1D,
    Malachite = 0x1E,
    Moonstone = 0x1F,
    Onyx = 0x20,
    Opal = 0x21,
    Peridot = 0x22,
    RedGarnet = 0x23,
    RedJade = 0x24,
    RoseQuartz = 0x25,
    Ruby = 0x26,
    Sapphire = 0x27,
    SmokeyQuartz = 0x28,
    Sunstone = 0x29,
    TigerEye = 0x2A,
    Tourmaline = 0x2B,
    Turquoise = 0x2C,
    WhiteJade = 0x2D,
    WhiteQuartz = 0x2E,
    WhiteSapphire = 0x2F,
    YellowGarnet = 0x30,
    YellowTopaz = 0x31,
    Zircon = 0x32,
    Ivory = 0x33,
    Leather = 0x34,
    ArmoredilloHide = 0x35,
    GromnieHide = 0x36,
    ReedSharkHide = 0x37,
    Brass = 0x39,
    Bronze = 0x3A,
    Copper = 0x3B,
    Gold = 0x3C,
    Iron = 0x3D,
    Pyreal = 0x3E,
    Silver = 0x3F,
    Steel = 0x40,
    Alabaster = 0x42,
    Granite = 0x43,
    Marble = 0x44,
    Obsidian = 0x45,
    Sandstone = 0x46,
    Serpentine = 0x47,
    Ebony = 0x49,
    Mahogany = 0x4A,
    Oak = 0x4B,
    Pine = 0x4C,
    Teak = 0x4D,
}

impl crate::readers::ACDataType for MaterialType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(MaterialType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for MaterialType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for MaterialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MaterialType::Ceramic => "Ceramic",
            MaterialType::Porcelain => "Porcelain",
            MaterialType::Linen => "Linen",
            MaterialType::Satin => "Satin",
            MaterialType::Silk => "Silk",
            MaterialType::Velvet => "Velvet",
            MaterialType::Wool => "Wool",
            MaterialType::Agate => "Agate",
            MaterialType::Amber => "Amber",
            MaterialType::Amethyst => "Amethyst",
            MaterialType::Aquamarine => "Aquamarine",
            MaterialType::Azurite => "Azurite",
            MaterialType::BlackGarnet => "BlackGarnet",
            MaterialType::BlackOpal => "BlackOpal",
            MaterialType::Bloodstone => "Bloodstone",
            MaterialType::Carnelian => "Carnelian",
            MaterialType::Citrine => "Citrine",
            MaterialType::Diamond => "Diamond",
            MaterialType::Emerald => "Emerald",
            MaterialType::FireOpal => "FireOpal",
            MaterialType::GreenGarnet => "GreenGarnet",
            MaterialType::GreenJade => "GreenJade",
            MaterialType::Hematite => "Hematite",
            MaterialType::ImperialTopaz => "ImperialTopaz",
            MaterialType::Jet => "Jet",
            MaterialType::LapisLazuli => "LapisLazuli",
            MaterialType::LavenderJade => "LavenderJade",
            MaterialType::Malachite => "Malachite",
            MaterialType::Moonstone => "Moonstone",
            MaterialType::Onyx => "Onyx",
            MaterialType::Opal => "Opal",
            MaterialType::Peridot => "Peridot",
            MaterialType::RedGarnet => "RedGarnet",
            MaterialType::RedJade => "RedJade",
            MaterialType::RoseQuartz => "RoseQuartz",
            MaterialType::Ruby => "Ruby",
            MaterialType::Sapphire => "Sapphire",
            MaterialType::SmokeyQuartz => "SmokeyQuartz",
            MaterialType::Sunstone => "Sunstone",
            MaterialType::TigerEye => "TigerEye",
            MaterialType::Tourmaline => "Tourmaline",
            MaterialType::Turquoise => "Turquoise",
            MaterialType::WhiteJade => "WhiteJade",
            MaterialType::WhiteQuartz => "WhiteQuartz",
            MaterialType::WhiteSapphire => "WhiteSapphire",
            MaterialType::YellowGarnet => "YellowGarnet",
            MaterialType::YellowTopaz => "YellowTopaz",
            MaterialType::Zircon => "Zircon",
            MaterialType::Ivory => "Ivory",
            MaterialType::Leather => "Leather",
            MaterialType::ArmoredilloHide => "ArmoredilloHide",
            MaterialType::GromnieHide => "GromnieHide",
            MaterialType::ReedSharkHide => "ReedSharkHide",
            MaterialType::Brass => "Brass",
            MaterialType::Bronze => "Bronze",
            MaterialType::Copper => "Copper",
            MaterialType::Gold => "Gold",
            MaterialType::Iron => "Iron",
            MaterialType::Pyreal => "Pyreal",
            MaterialType::Silver => "Silver",
            MaterialType::Steel => "Steel",
            MaterialType::Alabaster => "Alabaster",
            MaterialType::Granite => "Granite",
            MaterialType::Marble => "Marble",
            MaterialType::Obsidian => "Obsidian",
            MaterialType::Sandstone => "Sandstone",
            MaterialType::Serpentine => "Serpentine",
            MaterialType::Ebony => "Ebony",
            MaterialType::Mahogany => "Mahogany",
            MaterialType::Oak => "Oak",
            MaterialType::Pine => "Pine",
            MaterialType::Teak => "Teak",
        };
        write!(f, "{}", s)
    }
}

/// The ConfirmationType identifies the specific confirmation panel to be displayed.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ConfirmationType {
    SwearAllegiance = 0x1,
    AlterSkill = 0x2,
    AlterAttribute = 0x3,
    Fellowship = 0x4,
    Craft = 0x5,
    Augmentation = 0x6,
    YesNo = 0x7,
}

impl crate::readers::ACDataType for ConfirmationType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ConfirmationType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ConfirmationType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ConfirmationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ConfirmationType::SwearAllegiance => "SwearAllegiance",
            ConfirmationType::AlterSkill => "AlterSkill",
            ConfirmationType::AlterAttribute => "AlterAttribute",
            ConfirmationType::Fellowship => "Fellowship",
            ConfirmationType::Craft => "Craft",
            ConfirmationType::Augmentation => "Augmentation",
            ConfirmationType::YesNo => "YesNo",
        };
        write!(f, "{}", s)
    }
}

/// The EnvrionChangeType identifies the environment option set.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum EnvrionChangeType {
    Clear = 0x0,
    RedFog = 0x1,
    BlueFog = 0x2,
    WhiteFog = 0x3,
    GreenFog = 0x4,
    BlackFog = 0x5,
    BlackFog2 = 0x6,
    RoarSound = 0x65,
    BellSound = 0x66,
    Chant1Sound = 0x67,
    Chant2Sound = 0x68,
    DarkWhispers1Sound = 0x69,
    DarkWhispers2Sound = 0x6A,
    DarkLaughSound = 0x6B,
    DarkWindSound = 0x6C,
    DarkSpeechSound = 0x6D,
    DrumsSound = 0x6E,
    GhostSpeakSound = 0x6F,
    BreathingSound = 0x70,
    HowlSound = 0x71,
    LostSoulsSound = 0x72,
    SquealSound = 0x75,
    Thunder1Sound = 0x76,
    Thunder2Sound = 0x77,
    Thunder3Sound = 0x78,
    Thunder4Sound = 0x79,
    Thunder5Sound = 0x7A,
    Thunder6Sound = 0x7B,
}

impl crate::readers::ACDataType for EnvrionChangeType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EnvrionChangeType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for EnvrionChangeType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for EnvrionChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EnvrionChangeType::Clear => "Clear",
            EnvrionChangeType::RedFog => "RedFog",
            EnvrionChangeType::BlueFog => "BlueFog",
            EnvrionChangeType::WhiteFog => "WhiteFog",
            EnvrionChangeType::GreenFog => "GreenFog",
            EnvrionChangeType::BlackFog => "BlackFog",
            EnvrionChangeType::BlackFog2 => "BlackFog2",
            EnvrionChangeType::RoarSound => "RoarSound",
            EnvrionChangeType::BellSound => "BellSound",
            EnvrionChangeType::Chant1Sound => "Chant1Sound",
            EnvrionChangeType::Chant2Sound => "Chant2Sound",
            EnvrionChangeType::DarkWhispers1Sound => "DarkWhispers1Sound",
            EnvrionChangeType::DarkWhispers2Sound => "DarkWhispers2Sound",
            EnvrionChangeType::DarkLaughSound => "DarkLaughSound",
            EnvrionChangeType::DarkWindSound => "DarkWindSound",
            EnvrionChangeType::DarkSpeechSound => "DarkSpeechSound",
            EnvrionChangeType::DrumsSound => "DrumsSound",
            EnvrionChangeType::GhostSpeakSound => "GhostSpeakSound",
            EnvrionChangeType::BreathingSound => "BreathingSound",
            EnvrionChangeType::HowlSound => "HowlSound",
            EnvrionChangeType::LostSoulsSound => "LostSoulsSound",
            EnvrionChangeType::SquealSound => "SquealSound",
            EnvrionChangeType::Thunder1Sound => "Thunder1Sound",
            EnvrionChangeType::Thunder2Sound => "Thunder2Sound",
            EnvrionChangeType::Thunder3Sound => "Thunder3Sound",
            EnvrionChangeType::Thunder4Sound => "Thunder4Sound",
            EnvrionChangeType::Thunder5Sound => "Thunder5Sound",
            EnvrionChangeType::Thunder6Sound => "Thunder6Sound",
        };
        write!(f, "{}", s)
    }
}

/// The movement type defines the fields for the rest of the message
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum MovementType {
    InterpretedMotionState = 0x0,
    MoveToObject = 0x6,
    MoveToPosition = 0x7,
    TurnToObject = 0x8,
    TurnToPosition = 0x9,
}

impl crate::readers::ACDataType for MovementType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(MovementType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for MovementType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u8(writer, self.clone() as u8)?;
        Ok(())
    }
}

impl std::fmt::Display for MovementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MovementType::InterpretedMotionState => "InterpretedMotionState",
            MovementType::MoveToObject => "MoveToObject",
            MovementType::MoveToPosition => "MoveToPosition",
            MovementType::TurnToObject => "TurnToObject",
            MovementType::TurnToPosition => "TurnToPosition",
        };
        write!(f, "{}", s)
    }
}

/// Additional movement options
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum MovementOption {
    None = 0x0,
    StickToObject = 0x1,
    StandingLongJump = 0x2,
}

impl crate::readers::ACDataType for MovementOption {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(MovementOption::try_from(value)?)
    }
}

impl crate::writers::ACWritable for MovementOption {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u8(writer, self.clone() as u8)?;
        Ok(())
    }
}

impl std::fmt::Display for MovementOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MovementOption::None => "None",
            MovementOption::StickToObject => "StickToObject",
            MovementOption::StandingLongJump => "StandingLongJump",
        };
        write!(f, "{}", s)
    }
}

/// Command types
#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum Command {
    Invalid = 0x0,
    HoldRun = 0x1,
    HoldSidestep = 0x2,
    Ready = 0x3,
    Stop = 0x4,
    WalkForward = 0x5,
    WalkBackwards = 0x6,
    RunForward = 0x7,
    Fallen = 0x8,
    Interpolating = 0x9,
    Hover = 0xA,
    On = 0xB,
    Off = 0xC,
    TurnRight = 0xD,
    TurnLeft = 0xE,
    SideStepRight = 0xF,
    SideStepLeft = 0x10,
    Dead = 0x11,
    Crouch = 0x12,
    Sitting = 0x13,
    Sleeping = 0x14,
    Falling = 0x15,
    Reload = 0x16,
    Unload = 0x17,
    Pickup = 0x18,
    StoreInBackpack = 0x19,
    Eat = 0x1A,
    Drink = 0x1B,
    Reading = 0x1C,
    JumpCharging = 0x1D,
    AimLevel = 0x1E,
    AimHigh15 = 0x1F,
    AimHigh30 = 0x20,
    AimHigh45 = 0x21,
    AimHigh60 = 0x22,
    AimHigh75 = 0x23,
    AimHigh90 = 0x24,
    AimLow15 = 0x25,
    AimLow30 = 0x26,
    AimLow45 = 0x27,
    AimLow60 = 0x28,
    AimLow75 = 0x29,
    AimLow90 = 0x2A,
    MagicBlast = 0x2B,
    MagicSelfHead = 0x2C,
    MagicSelfHeart = 0x2D,
    MagicBonus = 0x2E,
    MagicClap = 0x2F,
    MagicHarm = 0x30,
    MagicHeal = 0x31,
    MagicThrowMissile = 0x32,
    MagicRecoilMissile = 0x33,
    MagicPenalty = 0x34,
    MagicTransfer = 0x35,
    MagicVision = 0x36,
    MagicEnchantItem = 0x37,
    MagicPortal = 0x38,
    MagicPray = 0x39,
    StopTurning = 0x3A,
    Jump = 0x3B,
    HandCombat = 0x3C,
    NonCombat = 0x3D,
    SwordCombat = 0x3E,
    BowCombat = 0x3F,
    SwordShieldCombat = 0x40,
    CrossbowCombat = 0x41,
    UnusedCombat = 0x42,
    SlingCombat = 0x43,
    TwoHandedSwordCombat = 0x44,
    TwoHandedStaffCombat = 0x45,
    DualWieldCombat = 0x46,
    ThrownWeaponCombat = 0x47,
    Graze = 0x48,
    Magi = 0x49,
    Hop = 0x4A,
    Jumpup = 0x4B,
    Cheer = 0x4C,
    ChestBeat = 0x4D,
    TippedLeft = 0x4E,
    TippedRight = 0x4F,
    FallDown = 0x50,
    Twitch1 = 0x51,
    Twitch2 = 0x52,
    Twitch3 = 0x53,
    Twitch4 = 0x54,
    StaggerBackward = 0x55,
    StaggerForward = 0x56,
    Sanctuary = 0x57,
    ThrustMed = 0x58,
    ThrustLow = 0x59,
    ThrustHigh = 0x5A,
    SlashHigh = 0x5B,
    SlashMed = 0x5C,
    SlashLow = 0x5D,
    BackhandHigh = 0x5E,
    BackhandMed = 0x5F,
    BackhandLow = 0x60,
    Shoot = 0x61,
    AttackHigh1 = 0x62,
    AttackMed1 = 0x63,
    AttackLow1 = 0x64,
    AttackHigh2 = 0x65,
    AttackMed2 = 0x66,
    AttackLow2 = 0x67,
    AttackHigh3 = 0x68,
    AttackMed3 = 0x69,
    AttackLow3 = 0x6A,
    HeadThrow = 0x6B,
    FistSlam = 0x6C,
    #[serde(rename = "BreatheFlame_")]
    BreatheFlame = 0x6D,
    SpinAttack = 0x6E,
    MagicPowerUp01 = 0x6F,
    MagicPowerUp02 = 0x70,
    MagicPowerUp03 = 0x71,
    MagicPowerUp04 = 0x72,
    MagicPowerUp05 = 0x73,
    MagicPowerUp06 = 0x74,
    MagicPowerUp07 = 0x75,
    MagicPowerUp08 = 0x76,
    MagicPowerUp09 = 0x77,
    MagicPowerUp10 = 0x78,
    ShakeFist = 0x79,
    Beckon = 0x7A,
    BeSeeingYou = 0x7B,
    BlowKiss = 0x7C,
    BowDeep = 0x7D,
    ClapHands = 0x7E,
    Cry = 0x7F,
    Laugh = 0x80,
    MimeEat = 0x81,
    MimeDrink = 0x82,
    Nod = 0x83,
    Point = 0x84,
    ShakeHead = 0x85,
    Shrug = 0x86,
    Wave = 0x87,
    Akimbo = 0x88,
    HeartyLaugh = 0x89,
    Salute = 0x8A,
    ScratchHead = 0x8B,
    SmackHead = 0x8C,
    TapFoot = 0x8D,
    WaveHigh = 0x8E,
    WaveLow = 0x8F,
    YawnStretch = 0x90,
    Cringe = 0x91,
    Kneel = 0x92,
    Plead = 0x93,
    Shiver = 0x94,
    Shoo = 0x95,
    Slouch = 0x96,
    Spit = 0x97,
    Surrender = 0x98,
    Woah = 0x99,
    Winded = 0x9A,
    YMCA = 0x9B,
    EnterGame = 0x9C,
    ExitGame = 0x9D,
    OnCreation = 0x9E,
    OnDestruction = 0x9F,
    EnterPortal = 0xA0,
    ExitPortal = 0xA1,
    Cancel = 0xA2,
    UseSelected = 0xA3,
    AutosortSelected = 0xA4,
    DropSelected = 0xA5,
    GiveSelected = 0xA6,
    SplitSelected = 0xA7,
    ExamineSelected = 0xA8,
    CreateShortcutToSelected = 0xA9,
    PreviousCompassItem = 0xAA,
    NextCompassItem = 0xAB,
    ClosestCompassItem = 0xAC,
    PreviousSelection = 0xAD,
    LastAttacker = 0xAE,
    PreviousFellow = 0xAF,
    NextFellow = 0xB0,
    ToggleCombat = 0xB1,
    HighAttack = 0xB2,
    MediumAttack = 0xB3,
    LowAttack = 0xB4,
    EnterChat = 0xB5,
    ToggleChat = 0xB6,
    SavePosition = 0xB7,
    OptionsPanel = 0xB8,
    ResetView = 0xB9,
    CameraLeftRotate = 0xBA,
    CameraRightRotate = 0xBB,
    CameraRaise = 0xBC,
    CameraLower = 0xBD,
    CameraCloser = 0xBE,
    CameraFarther = 0xBF,
    FloorView = 0xC0,
    MouseLook = 0xC1,
    PreviousItem = 0xC2,
    NextItem = 0xC3,
    ClosestItem = 0xC4,
    ShiftView = 0xC5,
    MapView = 0xC6,
    AutoRun = 0xC7,
    DecreasePowerSetting = 0xC8,
    IncreasePowerSetting = 0xC9,
    Pray = 0xCA,
    Mock = 0xCB,
    Teapot = 0xCC,
    SpecialAttack1 = 0xCD,
    SpecialAttack2 = 0xCE,
    SpecialAttack3 = 0xCF,
    MissileAttack1 = 0xD0,
    MissileAttack2 = 0xD1,
    MissileAttack3 = 0xD2,
    CastSpell = 0xD3,
    Flatulence = 0xD4,
    FirstPersonView = 0xD5,
    AllegiancePanel = 0xD6,
    FellowshipPanel = 0xD7,
    SpellbookPanel = 0xD8,
    SpellComponentsPanel = 0xD9,
    HousePanel = 0xDA,
    AttributesPanel = 0xDB,
    SkillsPanel = 0xDC,
    MapPanel = 0xDD,
    InventoryPanel = 0xDE,
    Demonet = 0xDF,
    UseMagicStaff = 0xE0,
    UseMagicWand = 0xE1,
    Blink = 0xE2,
    Bite = 0xE3,
    TwitchSubstate1 = 0xE4,
    TwitchSubstate2 = 0xE5,
    TwitchSubstate3 = 0xE6,
    CaptureScreenshotToFile = 0xE7,
    BowNoAmmo = 0xE8,
    CrossBowNoAmmo = 0xE9,
    ShakeFistState = 0xEA,
    PrayState = 0xEB,
    BowDeepState = 0xEC,
    ClapHandsState = 0xED,
    CrossArmsState = 0xEE,
    ShiverState = 0xEF,
    PointState = 0xF0,
    WaveState = 0xF1,
    AkimboState = 0xF2,
    SaluteState = 0xF3,
    ScratchHeadState = 0xF4,
    TapFootState = 0xF5,
    LeanState = 0xF6,
    KneelState = 0xF7,
    PleadState = 0xF8,
    ATOYOT = 0xF9,
    SlouchState = 0xFA,
    SurrenderState = 0xFB,
    WoahState = 0xFC,
    WindedState = 0xFD,
    AutoCreateShortcuts = 0xFE,
    AutoRepeatAttacks = 0xFF,
    AutoTarget = 0x100,
    AdvancedCombatInterface = 0x101,
    IgnoreAllegianceRequests = 0x102,
    IgnoreFellowshipRequests = 0x103,
    InvertMouseLook = 0x104,
    LetPlayersGiveYouItems = 0x105,
    AutoTrackCombatTargets = 0x106,
    DisplayTooltips = 0x107,
    AttemptToDeceivePlayers = 0x108,
    RunAsDefaultMovement = 0x109,
    StayInChatModeAfterSend = 0x10A,
    RightClickToMouseLook = 0x10B,
    VividTargetIndicator = 0x10C,
    SelectSelf = 0x10D,
    SkillHealSelf = 0x10E,
    NextMonster = 0x10F,
    PreviousMonster = 0x110,
    ClosestMonster = 0x111,
    NextPlayer = 0x112,
    PreviousPlayer = 0x113,
    ClosestPlayer = 0x114,
    SnowAngelState = 0x115,
    WarmHands = 0x116,
    CurtseyState = 0x117,
    AFKState = 0x118,
    MeditateState = 0x119,
    TradePanel = 0x11A,
    LogOut = 0x11B,
    DoubleSlashLow = 0x11C,
    DoubleSlashMed = 0x11D,
    DoubleSlashHigh = 0x11E,
    TripleSlashLow = 0x11F,
    TripleSlashMed = 0x120,
    TripleSlashHigh = 0x121,
    DoubleThrustLow = 0x122,
    DoubleThrustMed = 0x123,
    DoubleThrustHigh = 0x124,
    TripleThrustLow = 0x125,
    TripleThrustMed = 0x126,
    TripleThrustHigh = 0x127,
    MagicPowerUp01Purple = 0x128,
    MagicPowerUp02Purple = 0x129,
    MagicPowerUp03Purple = 0x12A,
    MagicPowerUp04Purple = 0x12B,
    MagicPowerUp05Purple = 0x12C,
    MagicPowerUp06Purple = 0x12D,
    MagicPowerUp07Purple = 0x12E,
    MagicPowerUp08Purple = 0x12F,
    MagicPowerUp09Purple = 0x130,
    MagicPowerUp10Purple = 0x131,
    Helper = 0x132,
    Pickup5 = 0x133,
    Pickup10 = 0x134,
    Pickup15 = 0x135,
    Pickup20 = 0x136,
    HouseRecall = 0x137,
    AtlatlCombat = 0x138,
    ThrownShieldCombat = 0x139,
    SitState = 0x13A,
    SitCrossleggedState = 0x13B,
    SitBackState = 0x13C,
    PointLeftState = 0x13D,
    PointRightState = 0x13E,
    TalktotheHandState = 0x13F,
    PointDownState = 0x140,
    DrudgeDanceState = 0x141,
    PossumState = 0x142,
    ReadState = 0x143,
    ThinkerState = 0x144,
    HaveASeatState = 0x145,
    AtEaseState = 0x146,
    NudgeLeft = 0x147,
    NudgeRight = 0x148,
    PointLeft = 0x149,
    PointRight = 0x14A,
    PointDown = 0x14B,
    Knock = 0x14C,
    ScanHorizon = 0x14D,
    DrudgeDance = 0x14E,
    HaveASeat = 0x14F,
    LifestoneRecall = 0x150,
    CharacterOptionsPanel = 0x151,
    SoundAndGraphicsPanel = 0x152,
    HelpfulSpellsPanel = 0x153,
    HarmfulSpellsPanel = 0x154,
    CharacterInformationPanel = 0x155,
    LinkStatusPanel = 0x156,
    VitaePanel = 0x157,
    ShareFellowshipXP = 0x158,
    ShareFellowshipLoot = 0x159,
    AcceptCorpseLooting = 0x15A,
    IgnoreTradeRequests = 0x15B,
    DisableWeather = 0x15C,
    DisableHouseEffect = 0x15D,
    SideBySideVitals = 0x15E,
    ShowRadarCoordinates = 0x15F,
    ShowSpellDurations = 0x160,
    MuteOnLosingFocus = 0x161,
    Fishing = 0x162,
    MarketplaceRecall = 0x163,
    EnterPKLite = 0x164,
    AllegianceChat = 0x165,
    AutomaticallyAcceptFellowshipRequests = 0x166,
    Reply = 0x167,
    MonarchReply = 0x168,
    PatronReply = 0x169,
    ToggleCraftingChanceOfSuccessDialog = 0x16A,
    UseClosestUnopenedCorpse = 0x16B,
    UseNextUnopenedCorpse = 0x16C,
    IssueSlashCommand = 0x16D,
    AllegianceHometownRecall = 0x16E,
    PKArenaRecall = 0x16F,
    OffhandSlashHigh = 0x170,
    OffhandSlashMed = 0x171,
    OffhandSlashLow = 0x172,
    OffhandThrustHigh = 0x173,
    OffhandThrustMed = 0x174,
    OffhandThrustLow = 0x175,
    OffhandDoubleSlashLow = 0x176,
    OffhandDoubleSlashMed = 0x177,
    OffhandDoubleSlashHigh = 0x178,
    OffhandTripleSlashLow = 0x179,
    OffhandTripleSlashMed = 0x17A,
    OffhandTripleSlashHigh = 0x17B,
    OffhandDoubleThrustLow = 0x17C,
    OffhandDoubleThrustMed = 0x17D,
    OffhandDoubleThrustHigh = 0x17E,
    OffhandTripleThrustLow = 0x17F,
    OffhandTripleThrustMed = 0x180,
    OffhandTripleThrustHigh = 0x181,
    OffhandKick = 0x182,
    AttackHigh4 = 0x183,
    AttackMed4 = 0x184,
    AttackLow4 = 0x185,
    AttackHigh5 = 0x186,
    AttackMed5 = 0x187,
    AttackLow5 = 0x188,
    AttackHigh6 = 0x189,
    AttackMed6 = 0x18A,
    AttackLow6 = 0x18B,
    PunchFastHigh = 0x18C,
    PunchFastMed = 0x18D,
    PunchFastLow = 0x18E,
    PunchSlowHigh = 0x18F,
    PunchSlowMed = 0x190,
    PunchSlowLow = 0x191,
    OffhandPunchFastHigh = 0x192,
    OffhandPunchFastMed = 0x193,
    OffhandPunchFastLow = 0x194,
    OffhandPunchSlowHigh = 0x195,
    OffhandPunchSlowMed = 0x196,
    OffhandPunchSlowLow = 0x197,
}

impl crate::readers::ACDataType for Command {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(Command::try_from(value)?)
    }
}

impl crate::writers::ACWritable for Command {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.clone() as u16)?;
        Ok(())
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Command::Invalid => "Invalid",
            Command::HoldRun => "HoldRun",
            Command::HoldSidestep => "HoldSidestep",
            Command::Ready => "Ready",
            Command::Stop => "Stop",
            Command::WalkForward => "WalkForward",
            Command::WalkBackwards => "WalkBackwards",
            Command::RunForward => "RunForward",
            Command::Fallen => "Fallen",
            Command::Interpolating => "Interpolating",
            Command::Hover => "Hover",
            Command::On => "On",
            Command::Off => "Off",
            Command::TurnRight => "TurnRight",
            Command::TurnLeft => "TurnLeft",
            Command::SideStepRight => "SideStepRight",
            Command::SideStepLeft => "SideStepLeft",
            Command::Dead => "Dead",
            Command::Crouch => "Crouch",
            Command::Sitting => "Sitting",
            Command::Sleeping => "Sleeping",
            Command::Falling => "Falling",
            Command::Reload => "Reload",
            Command::Unload => "Unload",
            Command::Pickup => "Pickup",
            Command::StoreInBackpack => "StoreInBackpack",
            Command::Eat => "Eat",
            Command::Drink => "Drink",
            Command::Reading => "Reading",
            Command::JumpCharging => "JumpCharging",
            Command::AimLevel => "AimLevel",
            Command::AimHigh15 => "AimHigh15",
            Command::AimHigh30 => "AimHigh30",
            Command::AimHigh45 => "AimHigh45",
            Command::AimHigh60 => "AimHigh60",
            Command::AimHigh75 => "AimHigh75",
            Command::AimHigh90 => "AimHigh90",
            Command::AimLow15 => "AimLow15",
            Command::AimLow30 => "AimLow30",
            Command::AimLow45 => "AimLow45",
            Command::AimLow60 => "AimLow60",
            Command::AimLow75 => "AimLow75",
            Command::AimLow90 => "AimLow90",
            Command::MagicBlast => "MagicBlast",
            Command::MagicSelfHead => "MagicSelfHead",
            Command::MagicSelfHeart => "MagicSelfHeart",
            Command::MagicBonus => "MagicBonus",
            Command::MagicClap => "MagicClap",
            Command::MagicHarm => "MagicHarm",
            Command::MagicHeal => "MagicHeal",
            Command::MagicThrowMissile => "MagicThrowMissile",
            Command::MagicRecoilMissile => "MagicRecoilMissile",
            Command::MagicPenalty => "MagicPenalty",
            Command::MagicTransfer => "MagicTransfer",
            Command::MagicVision => "MagicVision",
            Command::MagicEnchantItem => "MagicEnchantItem",
            Command::MagicPortal => "MagicPortal",
            Command::MagicPray => "MagicPray",
            Command::StopTurning => "StopTurning",
            Command::Jump => "Jump",
            Command::HandCombat => "HandCombat",
            Command::NonCombat => "NonCombat",
            Command::SwordCombat => "SwordCombat",
            Command::BowCombat => "BowCombat",
            Command::SwordShieldCombat => "SwordShieldCombat",
            Command::CrossbowCombat => "CrossbowCombat",
            Command::UnusedCombat => "UnusedCombat",
            Command::SlingCombat => "SlingCombat",
            Command::TwoHandedSwordCombat => "TwoHandedSwordCombat",
            Command::TwoHandedStaffCombat => "TwoHandedStaffCombat",
            Command::DualWieldCombat => "DualWieldCombat",
            Command::ThrownWeaponCombat => "ThrownWeaponCombat",
            Command::Graze => "Graze",
            Command::Magi => "Magi",
            Command::Hop => "Hop",
            Command::Jumpup => "Jumpup",
            Command::Cheer => "Cheer",
            Command::ChestBeat => "ChestBeat",
            Command::TippedLeft => "TippedLeft",
            Command::TippedRight => "TippedRight",
            Command::FallDown => "FallDown",
            Command::Twitch1 => "Twitch1",
            Command::Twitch2 => "Twitch2",
            Command::Twitch3 => "Twitch3",
            Command::Twitch4 => "Twitch4",
            Command::StaggerBackward => "StaggerBackward",
            Command::StaggerForward => "StaggerForward",
            Command::Sanctuary => "Sanctuary",
            Command::ThrustMed => "ThrustMed",
            Command::ThrustLow => "ThrustLow",
            Command::ThrustHigh => "ThrustHigh",
            Command::SlashHigh => "SlashHigh",
            Command::SlashMed => "SlashMed",
            Command::SlashLow => "SlashLow",
            Command::BackhandHigh => "BackhandHigh",
            Command::BackhandMed => "BackhandMed",
            Command::BackhandLow => "BackhandLow",
            Command::Shoot => "Shoot",
            Command::AttackHigh1 => "AttackHigh1",
            Command::AttackMed1 => "AttackMed1",
            Command::AttackLow1 => "AttackLow1",
            Command::AttackHigh2 => "AttackHigh2",
            Command::AttackMed2 => "AttackMed2",
            Command::AttackLow2 => "AttackLow2",
            Command::AttackHigh3 => "AttackHigh3",
            Command::AttackMed3 => "AttackMed3",
            Command::AttackLow3 => "AttackLow3",
            Command::HeadThrow => "HeadThrow",
            Command::FistSlam => "FistSlam",
            Command::BreatheFlame => "BreatheFlame_",
            Command::SpinAttack => "SpinAttack",
            Command::MagicPowerUp01 => "MagicPowerUp01",
            Command::MagicPowerUp02 => "MagicPowerUp02",
            Command::MagicPowerUp03 => "MagicPowerUp03",
            Command::MagicPowerUp04 => "MagicPowerUp04",
            Command::MagicPowerUp05 => "MagicPowerUp05",
            Command::MagicPowerUp06 => "MagicPowerUp06",
            Command::MagicPowerUp07 => "MagicPowerUp07",
            Command::MagicPowerUp08 => "MagicPowerUp08",
            Command::MagicPowerUp09 => "MagicPowerUp09",
            Command::MagicPowerUp10 => "MagicPowerUp10",
            Command::ShakeFist => "ShakeFist",
            Command::Beckon => "Beckon",
            Command::BeSeeingYou => "BeSeeingYou",
            Command::BlowKiss => "BlowKiss",
            Command::BowDeep => "BowDeep",
            Command::ClapHands => "ClapHands",
            Command::Cry => "Cry",
            Command::Laugh => "Laugh",
            Command::MimeEat => "MimeEat",
            Command::MimeDrink => "MimeDrink",
            Command::Nod => "Nod",
            Command::Point => "Point",
            Command::ShakeHead => "ShakeHead",
            Command::Shrug => "Shrug",
            Command::Wave => "Wave",
            Command::Akimbo => "Akimbo",
            Command::HeartyLaugh => "HeartyLaugh",
            Command::Salute => "Salute",
            Command::ScratchHead => "ScratchHead",
            Command::SmackHead => "SmackHead",
            Command::TapFoot => "TapFoot",
            Command::WaveHigh => "WaveHigh",
            Command::WaveLow => "WaveLow",
            Command::YawnStretch => "YawnStretch",
            Command::Cringe => "Cringe",
            Command::Kneel => "Kneel",
            Command::Plead => "Plead",
            Command::Shiver => "Shiver",
            Command::Shoo => "Shoo",
            Command::Slouch => "Slouch",
            Command::Spit => "Spit",
            Command::Surrender => "Surrender",
            Command::Woah => "Woah",
            Command::Winded => "Winded",
            Command::YMCA => "YMCA",
            Command::EnterGame => "EnterGame",
            Command::ExitGame => "ExitGame",
            Command::OnCreation => "OnCreation",
            Command::OnDestruction => "OnDestruction",
            Command::EnterPortal => "EnterPortal",
            Command::ExitPortal => "ExitPortal",
            Command::Cancel => "Cancel",
            Command::UseSelected => "UseSelected",
            Command::AutosortSelected => "AutosortSelected",
            Command::DropSelected => "DropSelected",
            Command::GiveSelected => "GiveSelected",
            Command::SplitSelected => "SplitSelected",
            Command::ExamineSelected => "ExamineSelected",
            Command::CreateShortcutToSelected => "CreateShortcutToSelected",
            Command::PreviousCompassItem => "PreviousCompassItem",
            Command::NextCompassItem => "NextCompassItem",
            Command::ClosestCompassItem => "ClosestCompassItem",
            Command::PreviousSelection => "PreviousSelection",
            Command::LastAttacker => "LastAttacker",
            Command::PreviousFellow => "PreviousFellow",
            Command::NextFellow => "NextFellow",
            Command::ToggleCombat => "ToggleCombat",
            Command::HighAttack => "HighAttack",
            Command::MediumAttack => "MediumAttack",
            Command::LowAttack => "LowAttack",
            Command::EnterChat => "EnterChat",
            Command::ToggleChat => "ToggleChat",
            Command::SavePosition => "SavePosition",
            Command::OptionsPanel => "OptionsPanel",
            Command::ResetView => "ResetView",
            Command::CameraLeftRotate => "CameraLeftRotate",
            Command::CameraRightRotate => "CameraRightRotate",
            Command::CameraRaise => "CameraRaise",
            Command::CameraLower => "CameraLower",
            Command::CameraCloser => "CameraCloser",
            Command::CameraFarther => "CameraFarther",
            Command::FloorView => "FloorView",
            Command::MouseLook => "MouseLook",
            Command::PreviousItem => "PreviousItem",
            Command::NextItem => "NextItem",
            Command::ClosestItem => "ClosestItem",
            Command::ShiftView => "ShiftView",
            Command::MapView => "MapView",
            Command::AutoRun => "AutoRun",
            Command::DecreasePowerSetting => "DecreasePowerSetting",
            Command::IncreasePowerSetting => "IncreasePowerSetting",
            Command::Pray => "Pray",
            Command::Mock => "Mock",
            Command::Teapot => "Teapot",
            Command::SpecialAttack1 => "SpecialAttack1",
            Command::SpecialAttack2 => "SpecialAttack2",
            Command::SpecialAttack3 => "SpecialAttack3",
            Command::MissileAttack1 => "MissileAttack1",
            Command::MissileAttack2 => "MissileAttack2",
            Command::MissileAttack3 => "MissileAttack3",
            Command::CastSpell => "CastSpell",
            Command::Flatulence => "Flatulence",
            Command::FirstPersonView => "FirstPersonView",
            Command::AllegiancePanel => "AllegiancePanel",
            Command::FellowshipPanel => "FellowshipPanel",
            Command::SpellbookPanel => "SpellbookPanel",
            Command::SpellComponentsPanel => "SpellComponentsPanel",
            Command::HousePanel => "HousePanel",
            Command::AttributesPanel => "AttributesPanel",
            Command::SkillsPanel => "SkillsPanel",
            Command::MapPanel => "MapPanel",
            Command::InventoryPanel => "InventoryPanel",
            Command::Demonet => "Demonet",
            Command::UseMagicStaff => "UseMagicStaff",
            Command::UseMagicWand => "UseMagicWand",
            Command::Blink => "Blink",
            Command::Bite => "Bite",
            Command::TwitchSubstate1 => "TwitchSubstate1",
            Command::TwitchSubstate2 => "TwitchSubstate2",
            Command::TwitchSubstate3 => "TwitchSubstate3",
            Command::CaptureScreenshotToFile => "CaptureScreenshotToFile",
            Command::BowNoAmmo => "BowNoAmmo",
            Command::CrossBowNoAmmo => "CrossBowNoAmmo",
            Command::ShakeFistState => "ShakeFistState",
            Command::PrayState => "PrayState",
            Command::BowDeepState => "BowDeepState",
            Command::ClapHandsState => "ClapHandsState",
            Command::CrossArmsState => "CrossArmsState",
            Command::ShiverState => "ShiverState",
            Command::PointState => "PointState",
            Command::WaveState => "WaveState",
            Command::AkimboState => "AkimboState",
            Command::SaluteState => "SaluteState",
            Command::ScratchHeadState => "ScratchHeadState",
            Command::TapFootState => "TapFootState",
            Command::LeanState => "LeanState",
            Command::KneelState => "KneelState",
            Command::PleadState => "PleadState",
            Command::ATOYOT => "ATOYOT",
            Command::SlouchState => "SlouchState",
            Command::SurrenderState => "SurrenderState",
            Command::WoahState => "WoahState",
            Command::WindedState => "WindedState",
            Command::AutoCreateShortcuts => "AutoCreateShortcuts",
            Command::AutoRepeatAttacks => "AutoRepeatAttacks",
            Command::AutoTarget => "AutoTarget",
            Command::AdvancedCombatInterface => "AdvancedCombatInterface",
            Command::IgnoreAllegianceRequests => "IgnoreAllegianceRequests",
            Command::IgnoreFellowshipRequests => "IgnoreFellowshipRequests",
            Command::InvertMouseLook => "InvertMouseLook",
            Command::LetPlayersGiveYouItems => "LetPlayersGiveYouItems",
            Command::AutoTrackCombatTargets => "AutoTrackCombatTargets",
            Command::DisplayTooltips => "DisplayTooltips",
            Command::AttemptToDeceivePlayers => "AttemptToDeceivePlayers",
            Command::RunAsDefaultMovement => "RunAsDefaultMovement",
            Command::StayInChatModeAfterSend => "StayInChatModeAfterSend",
            Command::RightClickToMouseLook => "RightClickToMouseLook",
            Command::VividTargetIndicator => "VividTargetIndicator",
            Command::SelectSelf => "SelectSelf",
            Command::SkillHealSelf => "SkillHealSelf",
            Command::NextMonster => "NextMonster",
            Command::PreviousMonster => "PreviousMonster",
            Command::ClosestMonster => "ClosestMonster",
            Command::NextPlayer => "NextPlayer",
            Command::PreviousPlayer => "PreviousPlayer",
            Command::ClosestPlayer => "ClosestPlayer",
            Command::SnowAngelState => "SnowAngelState",
            Command::WarmHands => "WarmHands",
            Command::CurtseyState => "CurtseyState",
            Command::AFKState => "AFKState",
            Command::MeditateState => "MeditateState",
            Command::TradePanel => "TradePanel",
            Command::LogOut => "LogOut",
            Command::DoubleSlashLow => "DoubleSlashLow",
            Command::DoubleSlashMed => "DoubleSlashMed",
            Command::DoubleSlashHigh => "DoubleSlashHigh",
            Command::TripleSlashLow => "TripleSlashLow",
            Command::TripleSlashMed => "TripleSlashMed",
            Command::TripleSlashHigh => "TripleSlashHigh",
            Command::DoubleThrustLow => "DoubleThrustLow",
            Command::DoubleThrustMed => "DoubleThrustMed",
            Command::DoubleThrustHigh => "DoubleThrustHigh",
            Command::TripleThrustLow => "TripleThrustLow",
            Command::TripleThrustMed => "TripleThrustMed",
            Command::TripleThrustHigh => "TripleThrustHigh",
            Command::MagicPowerUp01Purple => "MagicPowerUp01Purple",
            Command::MagicPowerUp02Purple => "MagicPowerUp02Purple",
            Command::MagicPowerUp03Purple => "MagicPowerUp03Purple",
            Command::MagicPowerUp04Purple => "MagicPowerUp04Purple",
            Command::MagicPowerUp05Purple => "MagicPowerUp05Purple",
            Command::MagicPowerUp06Purple => "MagicPowerUp06Purple",
            Command::MagicPowerUp07Purple => "MagicPowerUp07Purple",
            Command::MagicPowerUp08Purple => "MagicPowerUp08Purple",
            Command::MagicPowerUp09Purple => "MagicPowerUp09Purple",
            Command::MagicPowerUp10Purple => "MagicPowerUp10Purple",
            Command::Helper => "Helper",
            Command::Pickup5 => "Pickup5",
            Command::Pickup10 => "Pickup10",
            Command::Pickup15 => "Pickup15",
            Command::Pickup20 => "Pickup20",
            Command::HouseRecall => "HouseRecall",
            Command::AtlatlCombat => "AtlatlCombat",
            Command::ThrownShieldCombat => "ThrownShieldCombat",
            Command::SitState => "SitState",
            Command::SitCrossleggedState => "SitCrossleggedState",
            Command::SitBackState => "SitBackState",
            Command::PointLeftState => "PointLeftState",
            Command::PointRightState => "PointRightState",
            Command::TalktotheHandState => "TalktotheHandState",
            Command::PointDownState => "PointDownState",
            Command::DrudgeDanceState => "DrudgeDanceState",
            Command::PossumState => "PossumState",
            Command::ReadState => "ReadState",
            Command::ThinkerState => "ThinkerState",
            Command::HaveASeatState => "HaveASeatState",
            Command::AtEaseState => "AtEaseState",
            Command::NudgeLeft => "NudgeLeft",
            Command::NudgeRight => "NudgeRight",
            Command::PointLeft => "PointLeft",
            Command::PointRight => "PointRight",
            Command::PointDown => "PointDown",
            Command::Knock => "Knock",
            Command::ScanHorizon => "ScanHorizon",
            Command::DrudgeDance => "DrudgeDance",
            Command::HaveASeat => "HaveASeat",
            Command::LifestoneRecall => "LifestoneRecall",
            Command::CharacterOptionsPanel => "CharacterOptionsPanel",
            Command::SoundAndGraphicsPanel => "SoundAndGraphicsPanel",
            Command::HelpfulSpellsPanel => "HelpfulSpellsPanel",
            Command::HarmfulSpellsPanel => "HarmfulSpellsPanel",
            Command::CharacterInformationPanel => "CharacterInformationPanel",
            Command::LinkStatusPanel => "LinkStatusPanel",
            Command::VitaePanel => "VitaePanel",
            Command::ShareFellowshipXP => "ShareFellowshipXP",
            Command::ShareFellowshipLoot => "ShareFellowshipLoot",
            Command::AcceptCorpseLooting => "AcceptCorpseLooting",
            Command::IgnoreTradeRequests => "IgnoreTradeRequests",
            Command::DisableWeather => "DisableWeather",
            Command::DisableHouseEffect => "DisableHouseEffect",
            Command::SideBySideVitals => "SideBySideVitals",
            Command::ShowRadarCoordinates => "ShowRadarCoordinates",
            Command::ShowSpellDurations => "ShowSpellDurations",
            Command::MuteOnLosingFocus => "MuteOnLosingFocus",
            Command::Fishing => "Fishing",
            Command::MarketplaceRecall => "MarketplaceRecall",
            Command::EnterPKLite => "EnterPKLite",
            Command::AllegianceChat => "AllegianceChat",
            Command::AutomaticallyAcceptFellowshipRequests => "AutomaticallyAcceptFellowshipRequests",
            Command::Reply => "Reply",
            Command::MonarchReply => "MonarchReply",
            Command::PatronReply => "PatronReply",
            Command::ToggleCraftingChanceOfSuccessDialog => "ToggleCraftingChanceOfSuccessDialog",
            Command::UseClosestUnopenedCorpse => "UseClosestUnopenedCorpse",
            Command::UseNextUnopenedCorpse => "UseNextUnopenedCorpse",
            Command::IssueSlashCommand => "IssueSlashCommand",
            Command::AllegianceHometownRecall => "AllegianceHometownRecall",
            Command::PKArenaRecall => "PKArenaRecall",
            Command::OffhandSlashHigh => "OffhandSlashHigh",
            Command::OffhandSlashMed => "OffhandSlashMed",
            Command::OffhandSlashLow => "OffhandSlashLow",
            Command::OffhandThrustHigh => "OffhandThrustHigh",
            Command::OffhandThrustMed => "OffhandThrustMed",
            Command::OffhandThrustLow => "OffhandThrustLow",
            Command::OffhandDoubleSlashLow => "OffhandDoubleSlashLow",
            Command::OffhandDoubleSlashMed => "OffhandDoubleSlashMed",
            Command::OffhandDoubleSlashHigh => "OffhandDoubleSlashHigh",
            Command::OffhandTripleSlashLow => "OffhandTripleSlashLow",
            Command::OffhandTripleSlashMed => "OffhandTripleSlashMed",
            Command::OffhandTripleSlashHigh => "OffhandTripleSlashHigh",
            Command::OffhandDoubleThrustLow => "OffhandDoubleThrustLow",
            Command::OffhandDoubleThrustMed => "OffhandDoubleThrustMed",
            Command::OffhandDoubleThrustHigh => "OffhandDoubleThrustHigh",
            Command::OffhandTripleThrustLow => "OffhandTripleThrustLow",
            Command::OffhandTripleThrustMed => "OffhandTripleThrustMed",
            Command::OffhandTripleThrustHigh => "OffhandTripleThrustHigh",
            Command::OffhandKick => "OffhandKick",
            Command::AttackHigh4 => "AttackHigh4",
            Command::AttackMed4 => "AttackMed4",
            Command::AttackLow4 => "AttackLow4",
            Command::AttackHigh5 => "AttackHigh5",
            Command::AttackMed5 => "AttackMed5",
            Command::AttackLow5 => "AttackLow5",
            Command::AttackHigh6 => "AttackHigh6",
            Command::AttackMed6 => "AttackMed6",
            Command::AttackLow6 => "AttackLow6",
            Command::PunchFastHigh => "PunchFastHigh",
            Command::PunchFastMed => "PunchFastMed",
            Command::PunchFastLow => "PunchFastLow",
            Command::PunchSlowHigh => "PunchSlowHigh",
            Command::PunchSlowMed => "PunchSlowMed",
            Command::PunchSlowLow => "PunchSlowLow",
            Command::OffhandPunchFastHigh => "OffhandPunchFastHigh",
            Command::OffhandPunchFastMed => "OffhandPunchFastMed",
            Command::OffhandPunchFastLow => "OffhandPunchFastLow",
            Command::OffhandPunchSlowHigh => "OffhandPunchSlowHigh",
            Command::OffhandPunchSlowMed => "OffhandPunchSlowMed",
            Command::OffhandPunchSlowLow => "OffhandPunchSlowLow",
        };
        write!(f, "{}", s)
    }
}

/// The stance for a character or monster.
#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum StanceMode {
    HandCombat = 0x3C,
    NonCombat = 0x3D,
    SwordCombat = 0x3E,
    BowCombat = 0x3F,
    SwordShieldCombat = 0x40,
    CrossbowCombat = 0x41,
    UnusedCombat = 0x42,
    SlingCombat = 0x43,
    TwoHandedSwordCombat = 0x44,
    TwoHandedStaffCombat = 0x45,
    DualWieldCombat = 0x46,
    ThrownWeaponCombat = 0x47,
    BowNoAmmo = 0xE8,
    CrossBowNoAmmo = 0xE9,
    AtlatlCombat = 0x138,
    ThrownShieldCombat = 0x139,
}

impl crate::readers::ACDataType for StanceMode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(StanceMode::try_from(value)?)
    }
}

impl crate::writers::ACWritable for StanceMode {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.clone() as u16)?;
        Ok(())
    }
}

impl std::fmt::Display for StanceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            StanceMode::HandCombat => "HandCombat",
            StanceMode::NonCombat => "NonCombat",
            StanceMode::SwordCombat => "SwordCombat",
            StanceMode::BowCombat => "BowCombat",
            StanceMode::SwordShieldCombat => "SwordShieldCombat",
            StanceMode::CrossbowCombat => "CrossbowCombat",
            StanceMode::UnusedCombat => "UnusedCombat",
            StanceMode::SlingCombat => "SlingCombat",
            StanceMode::TwoHandedSwordCombat => "TwoHandedSwordCombat",
            StanceMode::TwoHandedStaffCombat => "TwoHandedStaffCombat",
            StanceMode::DualWieldCombat => "DualWieldCombat",
            StanceMode::ThrownWeaponCombat => "ThrownWeaponCombat",
            StanceMode::BowNoAmmo => "BowNoAmmo",
            StanceMode::CrossBowNoAmmo => "CrossBowNoAmmo",
            StanceMode::AtlatlCombat => "AtlatlCombat",
            StanceMode::ThrownShieldCombat => "ThrownShieldCombat",
        };
        write!(f, "{}", s)
    }
}

/// The movement (forward, side, turn) for a character or monster.
#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum MovementCommand {
    HoldRun = 0x1,
    HoldSidestep = 0x2,
    WalkForward = 0x5,
    WalkBackwards = 0x6,
    RunForward = 0x7,
    TurnRight = 0xD,
    TurnLeft = 0xE,
    SideStepRight = 0xF,
    SideStepLeft = 0x10,
}

impl crate::readers::ACDataType for MovementCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(MovementCommand::try_from(value)?)
    }
}

impl crate::writers::ACWritable for MovementCommand {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.clone() as u16)?;
        Ok(())
    }
}

impl std::fmt::Display for MovementCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MovementCommand::HoldRun => "HoldRun",
            MovementCommand::HoldSidestep => "HoldSidestep",
            MovementCommand::WalkForward => "WalkForward",
            MovementCommand::WalkBackwards => "WalkBackwards",
            MovementCommand::RunForward => "RunForward",
            MovementCommand::TurnRight => "TurnRight",
            MovementCommand::TurnLeft => "TurnLeft",
            MovementCommand::SideStepRight => "SideStepRight",
            MovementCommand::SideStepLeft => "SideStepLeft",
        };
        write!(f, "{}", s)
    }
}

/// House flags
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum HouseBitfield {
    Undef = 0x0,
    Active = 0x1,
    RequiresMonarch = 0x2,
}

impl crate::readers::ACDataType for HouseBitfield {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(HouseBitfield::try_from(value)?)
    }
}

impl crate::writers::ACWritable for HouseBitfield {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for HouseBitfield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HouseBitfield::Undef => "Undef",
            HouseBitfield::Active => "Active",
            HouseBitfield::RequiresMonarch => "RequiresMonarch",
        };
        write!(f, "{}", s)
    }
}

/// The type response to a chargen request
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CharGenResponseType {
    OK = 0x1,
    NameInUse = 0x3,
    NameBanned = 0x4,
    Corrupt = 0x5,
    #[serde(rename = "Corrupt_0x0006")]
    Corrupt0x0006 = 0x6,
    AdminPrivilegeDenied = 0x7,
}

impl crate::readers::ACDataType for CharGenResponseType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CharGenResponseType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for CharGenResponseType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for CharGenResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CharGenResponseType::OK => "OK",
            CharGenResponseType::NameInUse => "NameInUse",
            CharGenResponseType::NameBanned => "NameBanned",
            CharGenResponseType::Corrupt => "Corrupt",
            CharGenResponseType::Corrupt0x0006 => "Corrupt_0x0006",
            CharGenResponseType::AdminPrivilegeDenied => "AdminPrivilegeDenied",
        };
        write!(f, "{}", s)
    }
}

/// The CharacterErrorType identifies the type of character error that has occured.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CharacterErrorType {
    Logon = 0x1,
    AccountLogin = 0x3,
    ServerCrash = 0x4,
    Logoff = 0x5,
    Delete = 0x6,
    ServerCrash2 = 0x8,
    AccountInvalid = 0x9,
    AccountDoesntExist = 0xA,
    EnterGameGeneric = 0xB,
    EnterGameStressAccount = 0xC,
    EnterGameCharacterInWorld = 0xD,
    EnterGamePlayerAccountMissing = 0xE,
    EnterGameCharacterNotOwned = 0xF,
    EnterGameCharacterInWorldServer = 0x10,
    EnterGameOldCharacter = 0x11,
    EnterGameCorruptCharacter = 0x12,
    EnterGameStartServerDown = 0x13,
    EnterGameCouldntPlaceCharacter = 0x14,
    LogonServerFull = 0x15,
    EnterGameCharacterLocked = 0x17,
    SubscriptionExpired = 0x18,
}

impl crate::readers::ACDataType for CharacterErrorType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CharacterErrorType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for CharacterErrorType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for CharacterErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CharacterErrorType::Logon => "Logon",
            CharacterErrorType::AccountLogin => "AccountLogin",
            CharacterErrorType::ServerCrash => "ServerCrash",
            CharacterErrorType::Logoff => "Logoff",
            CharacterErrorType::Delete => "Delete",
            CharacterErrorType::ServerCrash2 => "ServerCrash2",
            CharacterErrorType::AccountInvalid => "AccountInvalid",
            CharacterErrorType::AccountDoesntExist => "AccountDoesntExist",
            CharacterErrorType::EnterGameGeneric => "EnterGameGeneric",
            CharacterErrorType::EnterGameStressAccount => "EnterGameStressAccount",
            CharacterErrorType::EnterGameCharacterInWorld => "EnterGameCharacterInWorld",
            CharacterErrorType::EnterGamePlayerAccountMissing => "EnterGamePlayerAccountMissing",
            CharacterErrorType::EnterGameCharacterNotOwned => "EnterGameCharacterNotOwned",
            CharacterErrorType::EnterGameCharacterInWorldServer => "EnterGameCharacterInWorldServer",
            CharacterErrorType::EnterGameOldCharacter => "EnterGameOldCharacter",
            CharacterErrorType::EnterGameCorruptCharacter => "EnterGameCorruptCharacter",
            CharacterErrorType::EnterGameStartServerDown => "EnterGameStartServerDown",
            CharacterErrorType::EnterGameCouldntPlaceCharacter => "EnterGameCouldntPlaceCharacter",
            CharacterErrorType::LogonServerFull => "LogonServerFull",
            CharacterErrorType::EnterGameCharacterLocked => "EnterGameCharacterLocked",
            CharacterErrorType::SubscriptionExpired => "SubscriptionExpired",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    /// The state flags for an object
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PhysicsState: u32 {
        const NONE = 0x0;
        const STATIC = 0x1;
        const ETHEREAL = 0x4;
        const REPORT_COLLISION = 0x8;
        const IGNORE_COLLISION = 0x10;
        const NO_DRAW = 0x20;
        const MISSLE = 0x40;
        const PUSHABLE = 0x80;
        const ALIGN_PATH = 0x100;
        const PATH_CLIPPED = 0x200;
        const GRAVITY = 0x400;
        const LIGHTING_ON = 0x800;
        const PARTICLE_EMITTER = 0x1000;
        const HIDDEN = 0x4000;
        const SCRIPTED_COLLISION = 0x8000;
        const HAS_PHYSICS_BSP = 0x10000;
        const INELASTIC = 0x20000;
        const HAS_DEFAULT_ANIM = 0x40000;
        const HAS_DEFAULT_SCRIPT = 0x80000;
        const CLOAKED = 0x100000;
        const REPORT_COLLISION_AS_ENVIRONMENT = 0x200000;
        const EDGE_SLIDE = 0x400000;
        const SLEDDING = 0x800000;
        const FROZEN = 0x1000000;
    }
}

impl crate::readers::ACDataType for PhysicsState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PhysicsState::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for PhysicsState {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

/// The TurbineChatType identifies the type of Turbine Chat message.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum TurbineChatType {
    ServerToClientMessage = 0x1,
    ClientToServerMessage = 0x3,
    AckClientToServerMessage = 0x5,
}

impl crate::readers::ACDataType for TurbineChatType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(TurbineChatType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for TurbineChatType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for TurbineChatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TurbineChatType::ServerToClientMessage => "ServerToClientMessage",
            TurbineChatType::ClientToServerMessage => "ClientToServerMessage",
            TurbineChatType::AckClientToServerMessage => "AckClientToServerMessage",
        };
        write!(f, "{}", s)
    }
}

/// The DatFileType identifies the dat file to be used.
#[repr(i64)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum DatFileType {
    #[serde(rename = "client_portal")]
    ClientPortal = 0x1,
    #[serde(rename = "client_cell_1")]
    ClientCell1 = 0x2,
    #[serde(rename = "client_local_English")]
    ClientLocalEnglish = 0x3,
}

impl crate::readers::ACDataType for DatFileType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i64(reader)?;
        Ok(DatFileType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for DatFileType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_i64(writer, self.clone() as i64)?;
        Ok(())
    }
}

impl std::fmt::Display for DatFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DatFileType::ClientPortal => "client_portal",
            DatFileType::ClientCell1 => "client_cell_1",
            DatFileType::ClientLocalEnglish => "client_local_English",
        };
        write!(f, "{}", s)
    }
}

/// The CompressionType identifies the type of data compression used.
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CompressionType {
    None = 0x0,
    ZLib = 0x1,
}

impl crate::readers::ACDataType for CompressionType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(CompressionType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for CompressionType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u8(writer, self.clone() as u8)?;
        Ok(())
    }
}

impl std::fmt::Display for CompressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CompressionType::None => "None",
            CompressionType::ZLib => "ZLib",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    /// The AttributeMask selects which creature attributes highlighting is applied to.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AttributeMask: u16 {
        const STRENGTH = 0x1;
        const ENDURANCE = 0x2;
        const QUICKNESS = 0x4;
        const COORDINATION = 0x8;
        const FOCUS = 0x10;
        const SELF = 0x20;
        const HEALTH = 0x40;
        const STAMINA = 0x80;
        const MANA = 0x100;
    }
}

impl crate::readers::ACDataType for AttributeMask {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(AttributeMask::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for AttributeMask {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The DamageType identifies the type of damage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DamageType: u32 {
        const SLASHING = 0x1;
        const PIERCING = 0x2;
        const BLUDGEONING = 0x4;
        const COLD = 0x8;
        const FIRE = 0x10;
        const ACID = 0x20;
        const ELECTRIC = 0x40;
    }
}

impl crate::readers::ACDataType for DamageType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(DamageType::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for DamageType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The HookAppraisalFlags identifies various properties for an item hooked.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct HookAppraisalFlags: u32 {
        const INSCRIBABLE = 0x1;
        const IS_HEALER = 0x2;
        const IS_LOCKPICK = 0x8;
    }
}

impl crate::readers::ACDataType for HookAppraisalFlags {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(HookAppraisalFlags::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for HookAppraisalFlags {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The ArmorHighlightMask selects which armor attributes highlighting is applied to.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ArmorHighlightMask: u16 {
        const ARMOR_LEVEL = 0x1;
        const SLASHING_PROTECTION = 0x2;
        const PIERCING_PROTECTION = 0x4;
        const BLUDGEONING_PROTECTION = 0x8;
        const COLD_PROTECTION = 0x10;
        const FIRE_PROTECTION = 0x20;
        const ACID_PROTECTION = 0x40;
        const ELECTRICAL_PROTECTION = 0x80;
    }
}

impl crate::readers::ACDataType for ArmorHighlightMask {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(ArmorHighlightMask::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for ArmorHighlightMask {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The ResistHighlightMask selects which wand attributes highlighting is applied to.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ResistHighlightMask: u16 {
        const RESIST_SLASH = 0x1;
        const RESIST_PIERCE = 0x2;
        const RESIST_BLUDGEON = 0x4;
        const RESIST_FIRE = 0x8;
        const RESIST_COLD = 0x10;
        const RESIST_ACID = 0x20;
        const RESIST_ELECTRIC = 0x40;
        const RESIST_HEALTH_BOOST = 0x80;
        const RESIST_STAMINA_DRAIN = 0x100;
        const RESIST_STAMINA_BOOST = 0x200;
        const RESIST_MANA_DRAIN = 0x400;
        const RESIST_MANA_BOOST = 0x800;
        const MANA_CONVERSION_MOD = 0x1000;
        const ELEMENTAL_DAMAGE_MOD = 0x2000;
        const RESIST_NETHER = 0x4000;
    }
}

impl crate::readers::ACDataType for ResistHighlightMask {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(ResistHighlightMask::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for ResistHighlightMask {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// The WeaponHighlightMask selects which weapon attributes highlighting is applied to.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct WeaponHighlightMask: u16 {
        const ATTACK_SKILL = 0x1;
        const MELEE_DEFENSE = 0x2;
        const SPEED = 0x4;
        const DAMAGE = 0x8;
        const DAMAGE_VARIANCE = 0x10;
        const DAMAGE_MOD = 0x20;
    }
}

impl crate::readers::ACDataType for WeaponHighlightMask {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(WeaponHighlightMask::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for WeaponHighlightMask {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// Additional attack information
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AttackConditionsMask: u32 {
        const CRITICAL_PROTECTION_AUGMENTATION = 0x1;
        const RECKLESSNESS = 0x2;
        const SNEAK_ATTACK = 0x4;
    }
}

impl crate::readers::ACDataType for AttackConditionsMask {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AttackConditionsMask::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for AttackConditionsMask {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

/// The DamageLocation indicates where damage was done.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum DamageLocation {
    Head = 0x0,
    Chest = 0x1,
    Abdomen = 0x2,
    UpperArm = 0x3,
    LowerArm = 0x4,
    Hand = 0x5,
    UpperLeg = 0x6,
    LowerLeg = 0x7,
    Foot = 0x8,
}

impl crate::readers::ACDataType for DamageLocation {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(DamageLocation::try_from(value)?)
    }
}

impl crate::writers::ACWritable for DamageLocation {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for DamageLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DamageLocation::Head => "Head",
            DamageLocation::Chest => "Chest",
            DamageLocation::Abdomen => "Abdomen",
            DamageLocation::UpperArm => "UpperArm",
            DamageLocation::LowerArm => "LowerArm",
            DamageLocation::Hand => "Hand",
            DamageLocation::UpperLeg => "UpperLeg",
            DamageLocation::LowerLeg => "LowerLeg",
            DamageLocation::Foot => "Foot",
        };
        write!(f, "{}", s)
    }
}

/// The LogTextType indicates the kind of text going to the chat area.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum LogTextType {
    Default = 0x0,
    Speech = 0x2,
    Tell = 0x3,
    SpeechDirectSend = 0x4,
    System = 0x5,
    Combat = 0x6,
    Magic = 0x7,
    Channel = 0x8,
    ChannelSend = 0x9,
    Social = 0xA,
    SocialSend = 0xB,
    Emote = 0xC,
    Advancement = 0xD,
    Abuse = 0xE,
    Help = 0xF,
    Appraisal = 0x10,
    Spellcasting = 0x11,
    Allegiance = 0x12,
    Fellowship = 0x13,
    WorldBroadcast = 0x14,
    CombatEnemy = 0x15,
    CombatSelf = 0x16,
    Recall = 0x17,
    Craft = 0x18,
    Salvaging = 0x19,
    AdminTell = 0x1F,
}

impl crate::readers::ACDataType for LogTextType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(LogTextType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for LogTextType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for LogTextType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LogTextType::Default => "Default",
            LogTextType::Speech => "Speech",
            LogTextType::Tell => "Tell",
            LogTextType::SpeechDirectSend => "SpeechDirectSend",
            LogTextType::System => "System",
            LogTextType::Combat => "Combat",
            LogTextType::Magic => "Magic",
            LogTextType::Channel => "Channel",
            LogTextType::ChannelSend => "ChannelSend",
            LogTextType::Social => "Social",
            LogTextType::SocialSend => "SocialSend",
            LogTextType::Emote => "Emote",
            LogTextType::Advancement => "Advancement",
            LogTextType::Abuse => "Abuse",
            LogTextType::Help => "Help",
            LogTextType::Appraisal => "Appraisal",
            LogTextType::Spellcasting => "Spellcasting",
            LogTextType::Allegiance => "Allegiance",
            LogTextType::Fellowship => "Fellowship",
            LogTextType::WorldBroadcast => "WorldBroadcast",
            LogTextType::CombatEnemy => "CombatEnemy",
            LogTextType::CombatSelf => "CombatSelf",
            LogTextType::Recall => "Recall",
            LogTextType::Craft => "Craft",
            LogTextType::Salvaging => "Salvaging",
            LogTextType::AdminTell => "AdminTell",
        };
        write!(f, "{}", s)
    }
}

/// The EndTradeReason identifies the reason trading was ended.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum EndTradeReason {
    Normal = 0x0,
    EnteredCombat = 0x2,
    Cancelled = 0x51,
}

impl crate::readers::ACDataType for EndTradeReason {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EndTradeReason::try_from(value)?)
    }
}

impl crate::writers::ACWritable for EndTradeReason {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for EndTradeReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EndTradeReason::Normal => "Normal",
            EndTradeReason::EnteredCombat => "EnteredCombat",
            EndTradeReason::Cancelled => "Cancelled",
        };
        write!(f, "{}", s)
    }
}

/// The TradeSide identifies the side of the trade window.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum TradeSide {
    #[serde(rename = "Self")]
    Self_ = 0x1,
    Partner = 0x2,
}

impl crate::readers::ACDataType for TradeSide {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(TradeSide::try_from(value)?)
    }
}

impl crate::writers::ACWritable for TradeSide {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for TradeSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TradeSide::Self_ => "Self",
            TradeSide::Partner => "Partner",
        };
        write!(f, "{}", s)
    }
}

/// The HouseType identifies the type of house.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum HouseType {
    Cottage = 0x1,
    Villa = 0x2,
    Mansion = 0x3,
    Apartment = 0x4,
}

impl crate::readers::ACDataType for HouseType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(HouseType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for HouseType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for HouseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HouseType::Cottage => "Cottage",
            HouseType::Villa => "Villa",
            HouseType::Mansion => "Mansion",
            HouseType::Apartment => "Apartment",
        };
        write!(f, "{}", s)
    }
}

/// Identifies the chess move attempt result.  Negative/0 values are failures.
#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ChessMoveResult {
    FailureNotYourTurn = -3,
    FailureInvalidDirection = -100,
    FailureInvalidDistance = -101,
    FailureMovingEmptySquare = -102,
    FailureMovingOpponentPiece = -103,
    FailureMovedPieceOffBoard = -104,
    FailureAttackingOwnPiece = -105,
    FailureCannotMoveIntoCheck = -106,
    FailurePathBlocked = -107,
    FailureCastleOutOfCheck = -108,
    FailureCastleThroughCheck = -109,
    FailureCastlePieceMoved = -110,
    FailureInvalidMove = 0x0,
    Success = 0x1,
    OpponentInCheck = 0x400,
    CheckMatedOpponent = 0x800,
}

impl crate::readers::ACDataType for ChessMoveResult {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(ChessMoveResult::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ChessMoveResult {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_i32(writer, self.clone() as i32)?;
        Ok(())
    }
}

impl std::fmt::Display for ChessMoveResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ChessMoveResult::FailureNotYourTurn => "FailureNotYourTurn",
            ChessMoveResult::FailureInvalidDirection => "FailureInvalidDirection",
            ChessMoveResult::FailureInvalidDistance => "FailureInvalidDistance",
            ChessMoveResult::FailureMovingEmptySquare => "FailureMovingEmptySquare",
            ChessMoveResult::FailureMovingOpponentPiece => "FailureMovingOpponentPiece",
            ChessMoveResult::FailureMovedPieceOffBoard => "FailureMovedPieceOffBoard",
            ChessMoveResult::FailureAttackingOwnPiece => "FailureAttackingOwnPiece",
            ChessMoveResult::FailureCannotMoveIntoCheck => "FailureCannotMoveIntoCheck",
            ChessMoveResult::FailurePathBlocked => "FailurePathBlocked",
            ChessMoveResult::FailureCastleOutOfCheck => "FailureCastleOutOfCheck",
            ChessMoveResult::FailureCastleThroughCheck => "FailureCastleThroughCheck",
            ChessMoveResult::FailureCastlePieceMoved => "FailureCastlePieceMoved",
            ChessMoveResult::FailureInvalidMove => "FailureInvalidMove",
            ChessMoveResult::Success => "Success",
            ChessMoveResult::OpponentInCheck => "OpponentInCheck",
            ChessMoveResult::CheckMatedOpponent => "CheckMatedOpponent",
        };
        write!(f, "{}", s)
    }
}

/// Type of fellow update
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum FellowUpdateType {
    FullUpdate = 0x1,
    UpdateStats = 0x2,
    UpdateVitals = 0x3,
}

impl crate::readers::ACDataType for FellowUpdateType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(FellowUpdateType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for FellowUpdateType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for FellowUpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FellowUpdateType::FullUpdate => "FullUpdate",
            FellowUpdateType::UpdateStats => "UpdateStats",
            FellowUpdateType::UpdateVitals => "UpdateVitals",
        };
        write!(f, "{}", s)
    }
}

/// Stage a contract is in.  Values 4+ appear to provide contract specific update messages
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ContractStage {
    New = 0x1,
    InProgress = 0x2,
    DoneOrPendingRepeat = 0x3,
}

impl crate::readers::ACDataType for ContractStage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ContractStage::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ContractStage {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ContractStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ContractStage::New => "New",
            ContractStage::InProgress => "InProgress",
            ContractStage::DoneOrPendingRepeat => "DoneOrPendingRepeat",
        };
        write!(f, "{}", s)
    }
}

/// Movement hold key
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum HoldKey {
    Invalid = 0x0,
    None = 0x1,
    Run = 0x2,
}

impl crate::readers::ACDataType for HoldKey {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(HoldKey::try_from(value)?)
    }
}

impl crate::writers::ACWritable for HoldKey {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for HoldKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HoldKey::Invalid => "Invalid",
            HoldKey::None => "None",
            HoldKey::Run => "Run",
        };
        write!(f, "{}", s)
    }
}

/// Radar behavior
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum RadarBehavior {
    Undefined = 0x0,
    ShowNever = 0x1,
    ShowMovement = 0x2,
    ShowAttacking = 0x3,
    ShowAlways = 0x4,
}

impl crate::readers::ACDataType for RadarBehavior {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(RadarBehavior::try_from(value)?)
    }
}

impl crate::writers::ACWritable for RadarBehavior {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u8(writer, self.clone() as u8)?;
        Ok(())
    }
}

impl std::fmt::Display for RadarBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RadarBehavior::Undefined => "Undefined",
            RadarBehavior::ShowNever => "ShowNever",
            RadarBehavior::ShowMovement => "ShowMovement",
            RadarBehavior::ShowAttacking => "ShowAttacking",
            RadarBehavior::ShowAlways => "ShowAlways",
        };
        write!(f, "{}", s)
    }
}

/// Gender of a player
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum Gender {
    Invalid = 0x0,
    Male = 0x1,
    Female = 0x2,
}

impl crate::readers::ACDataType for Gender {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(Gender::try_from(value)?)
    }
}

impl crate::writers::ACWritable for Gender {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u8(writer, self.clone() as u8)?;
        Ok(())
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Gender::Invalid => "Invalid",
            Gender::Male => "Male",
            Gender::Female => "Female",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FactionBits: u32 {
        const NONE = 0x0;
        const CELESTIAL_HAND = 0x1;
        const ELDRYTCH_WEB = 0x2;
        const RADIANT_BLOOD = 0x4;
    }
}

impl crate::readers::ACDataType for FactionBits {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(FactionBits::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for FactionBits {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

/// Creature type
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CreatureType {
    Olthoi = 0x1,
    Banderling = 0x2,
    Drudge = 0x3,
    Mosswart = 0x4,
    Lugian = 0x5,
    Tumerok = 0x6,
    Mite = 0x7,
    Tusker = 0x8,
    PhyntosWasp = 0x9,
    Rat = 0xA,
    Auroch = 0xB,
    Cow = 0xC,
    Golem = 0xD,
    Undead = 0xE,
    Gromnie = 0xF,
    Reedshark = 0x10,
    Armoredillo = 0x11,
    Fae = 0x12,
    Virindi = 0x13,
    Wisp = 0x14,
    Knathtead = 0x15,
    Shadow = 0x16,
    Mattekar = 0x17,
    Mumiyah = 0x18,
    Rabbit = 0x19,
    Sclavus = 0x1A,
    ShallowsShark = 0x1B,
    Monouga = 0x1C,
    Zefir = 0x1D,
    Skeleton = 0x1E,
    Human = 0x1F,
    Shreth = 0x20,
    Chittick = 0x21,
    Moarsman = 0x22,
    OlthoiLarvae = 0x23,
    Slithis = 0x24,
    Deru = 0x25,
    FireElemental = 0x26,
    Snowman = 0x27,
    Unknown = 0x28,
    Bunny = 0x29,
    LightningElemental = 0x2A,
    Rockslide = 0x2B,
    Grievver = 0x2C,
    Niffis = 0x2D,
    Ursuin = 0x2E,
    Crystal = 0x2F,
    HollowMinion = 0x30,
    Scarecrow = 0x31,
    Idol = 0x32,
    Empyrean = 0x33,
    Hopeslayer = 0x34,
    Doll = 0x35,
    Marionette = 0x36,
    Carenzi = 0x37,
    Siraluun = 0x38,
    AunTumerok = 0x39,
    HeaTumerok = 0x3A,
    Simulacrum = 0x3B,
    AcidElemental = 0x3C,
    FrostElemental = 0x3D,
    Elemental = 0x3E,
    Statue = 0x3F,
    Wall = 0x40,
    AlteredHuman = 0x41,
    Device = 0x42,
    Harbinger = 0x43,
    DarkSarcophagus = 0x44,
    Chicken = 0x45,
    GotrokLugian = 0x46,
    Margul = 0x47,
    BleachedRabbit = 0x48,
    NastyRabbit = 0x49,
    GrimacingRabbit = 0x4A,
    Burun = 0x4B,
    Target = 0x4C,
    Ghost = 0x4D,
    Fiun = 0x4E,
    Eater = 0x4F,
    Penguin = 0x50,
    Ruschk = 0x51,
    Thrungus = 0x52,
    ViamontianKnight = 0x53,
    Remoran = 0x54,
    Swarm = 0x55,
    Moar = 0x56,
    EnchantedArms = 0x57,
    Sleech = 0x58,
    Mukkir = 0x59,
    Merwart = 0x5A,
    Food = 0x5B,
    ParadoxOlthoi = 0x5C,
    Harvest = 0x5D,
    Energy = 0x5E,
    Apparition = 0x5F,
    Aerbax = 0x60,
    Touched = 0x61,
    BlightedMoarsman = 0x62,
    GearKnight = 0x63,
    Gurog = 0x64,
    Anekshay = 0x65,
}

impl crate::readers::ACDataType for CreatureType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CreatureType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for CreatureType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for CreatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CreatureType::Olthoi => "Olthoi",
            CreatureType::Banderling => "Banderling",
            CreatureType::Drudge => "Drudge",
            CreatureType::Mosswart => "Mosswart",
            CreatureType::Lugian => "Lugian",
            CreatureType::Tumerok => "Tumerok",
            CreatureType::Mite => "Mite",
            CreatureType::Tusker => "Tusker",
            CreatureType::PhyntosWasp => "PhyntosWasp",
            CreatureType::Rat => "Rat",
            CreatureType::Auroch => "Auroch",
            CreatureType::Cow => "Cow",
            CreatureType::Golem => "Golem",
            CreatureType::Undead => "Undead",
            CreatureType::Gromnie => "Gromnie",
            CreatureType::Reedshark => "Reedshark",
            CreatureType::Armoredillo => "Armoredillo",
            CreatureType::Fae => "Fae",
            CreatureType::Virindi => "Virindi",
            CreatureType::Wisp => "Wisp",
            CreatureType::Knathtead => "Knathtead",
            CreatureType::Shadow => "Shadow",
            CreatureType::Mattekar => "Mattekar",
            CreatureType::Mumiyah => "Mumiyah",
            CreatureType::Rabbit => "Rabbit",
            CreatureType::Sclavus => "Sclavus",
            CreatureType::ShallowsShark => "ShallowsShark",
            CreatureType::Monouga => "Monouga",
            CreatureType::Zefir => "Zefir",
            CreatureType::Skeleton => "Skeleton",
            CreatureType::Human => "Human",
            CreatureType::Shreth => "Shreth",
            CreatureType::Chittick => "Chittick",
            CreatureType::Moarsman => "Moarsman",
            CreatureType::OlthoiLarvae => "OlthoiLarvae",
            CreatureType::Slithis => "Slithis",
            CreatureType::Deru => "Deru",
            CreatureType::FireElemental => "FireElemental",
            CreatureType::Snowman => "Snowman",
            CreatureType::Unknown => "Unknown",
            CreatureType::Bunny => "Bunny",
            CreatureType::LightningElemental => "LightningElemental",
            CreatureType::Rockslide => "Rockslide",
            CreatureType::Grievver => "Grievver",
            CreatureType::Niffis => "Niffis",
            CreatureType::Ursuin => "Ursuin",
            CreatureType::Crystal => "Crystal",
            CreatureType::HollowMinion => "HollowMinion",
            CreatureType::Scarecrow => "Scarecrow",
            CreatureType::Idol => "Idol",
            CreatureType::Empyrean => "Empyrean",
            CreatureType::Hopeslayer => "Hopeslayer",
            CreatureType::Doll => "Doll",
            CreatureType::Marionette => "Marionette",
            CreatureType::Carenzi => "Carenzi",
            CreatureType::Siraluun => "Siraluun",
            CreatureType::AunTumerok => "AunTumerok",
            CreatureType::HeaTumerok => "HeaTumerok",
            CreatureType::Simulacrum => "Simulacrum",
            CreatureType::AcidElemental => "AcidElemental",
            CreatureType::FrostElemental => "FrostElemental",
            CreatureType::Elemental => "Elemental",
            CreatureType::Statue => "Statue",
            CreatureType::Wall => "Wall",
            CreatureType::AlteredHuman => "AlteredHuman",
            CreatureType::Device => "Device",
            CreatureType::Harbinger => "Harbinger",
            CreatureType::DarkSarcophagus => "DarkSarcophagus",
            CreatureType::Chicken => "Chicken",
            CreatureType::GotrokLugian => "GotrokLugian",
            CreatureType::Margul => "Margul",
            CreatureType::BleachedRabbit => "BleachedRabbit",
            CreatureType::NastyRabbit => "NastyRabbit",
            CreatureType::GrimacingRabbit => "GrimacingRabbit",
            CreatureType::Burun => "Burun",
            CreatureType::Target => "Target",
            CreatureType::Ghost => "Ghost",
            CreatureType::Fiun => "Fiun",
            CreatureType::Eater => "Eater",
            CreatureType::Penguin => "Penguin",
            CreatureType::Ruschk => "Ruschk",
            CreatureType::Thrungus => "Thrungus",
            CreatureType::ViamontianKnight => "ViamontianKnight",
            CreatureType::Remoran => "Remoran",
            CreatureType::Swarm => "Swarm",
            CreatureType::Moar => "Moar",
            CreatureType::EnchantedArms => "EnchantedArms",
            CreatureType::Sleech => "Sleech",
            CreatureType::Mukkir => "Mukkir",
            CreatureType::Merwart => "Merwart",
            CreatureType::Food => "Food",
            CreatureType::ParadoxOlthoi => "ParadoxOlthoi",
            CreatureType::Harvest => "Harvest",
            CreatureType::Energy => "Energy",
            CreatureType::Apparition => "Apparition",
            CreatureType::Aerbax => "Aerbax",
            CreatureType::Touched => "Touched",
            CreatureType::BlightedMoarsman => "BlightedMoarsman",
            CreatureType::GearKnight => "GearKnight",
            CreatureType::Gurog => "Gurog",
            CreatureType::Anekshay => "Anekshay",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CombatStyle: u32 {
        const UNDEF = 0x0;
        const UNARMED = 0x1;
        const ONE_HANDED = 0x2;
        const ONE_HANDED_AND_SHIELD = 0x4;
        const TWO_HANDED = 0x8;
        const BOW = 0x10;
        const CROSSBOW = 0x20;
        const SLING = 0x40;
        const THROWN_WEAPON = 0x80;
        const DUAL_WIELD = 0x100;
        const MAGIC = 0x200;
        const ATLATL = 0x400;
        const THROWN_SHIELD = 0x800;
        const RESERVED1 = 0x1000;
        const RESERVED2 = 0x2000;
        const RESERVED3 = 0x4000;
        const RESERVED4 = 0x8000;
        const STUBBORN_MAGIC = 0x10000;
        const STUBBORN_PROJECTILE = 0x20000;
        const STUBBORN_MELEE = 0x40000;
        const STUBBORN_MISSILE = 0x80000;
    }
}

impl crate::readers::ACDataType for CombatStyle {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CombatStyle::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for CombatStyle {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

bitflags::bitflags! {
    /// Indicates what data is present in the ACQualities data
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ACQualitiesFlags: u32 {
        const ATTRIBUTES = 0x1;
        const SKILLS = 0x2;
        const BODY = 0x4;
        const SPELL_BOOK = 0x100;
        const ENCHANTMENTS = 0x200;
        const EVENT_FILTER = 0x8;
        const EMOTES = 0x10;
        const CREATION_PROFILE = 0x20;
        const PAGE_DATA = 0x40;
        const GENERATORS = 0x80;
        const GENERATOR_REGISTRY = 0x400;
        const GENERATOR_QUEUE = 0x800;
    }
}

impl crate::readers::ACDataType for ACQualitiesFlags {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ACQualitiesFlags::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for ACQualitiesFlags {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum GeneratorDestruct {
    Undef = 0x0,
    Nothing = 0x1,
    Destroy = 0x2,
    Kill = 0x3,
}

impl crate::readers::ACDataType for GeneratorDestruct {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(GeneratorDestruct::try_from(value)?)
    }
}

impl crate::writers::ACWritable for GeneratorDestruct {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for GeneratorDestruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GeneratorDestruct::Undef => "Undef",
            GeneratorDestruct::Nothing => "Nothing",
            GeneratorDestruct::Destroy => "Destroy",
            GeneratorDestruct::Kill => "Kill",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum GeneratorTimeType {
    Undef = 0x0,
    RealTime = 0x1,
    Defined = 0x2,
    Event = 0x3,
    Night = 0x4,
    Day = 0x5,
}

impl crate::readers::ACDataType for GeneratorTimeType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(GeneratorTimeType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for GeneratorTimeType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for GeneratorTimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GeneratorTimeType::Undef => "Undef",
            GeneratorTimeType::RealTime => "RealTime",
            GeneratorTimeType::Defined => "Defined",
            GeneratorTimeType::Event => "Event",
            GeneratorTimeType::Night => "Night",
            GeneratorTimeType::Day => "Day",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum GeneratorType {
    Undef = 0x0,
    Relative = 0x1,
    Absolute = 0x2,
}

impl crate::readers::ACDataType for GeneratorType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(GeneratorType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for GeneratorType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for GeneratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GeneratorType::Undef => "Undef",
            GeneratorType::Relative => "Relative",
            GeneratorType::Absolute => "Absolute",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ImbuedEffectType {
    Undef = 0x0,
    CriticalStrike = 0x1,
    CripplingBlow = 0x2,
    ArmorRending = 0x4,
    SlashRending = 0x8,
    PierceRending = 0x10,
    BludgeonRending = 0x20,
    AcidRending = 0x40,
    ColdRending = 0x80,
    ElectricRending = 0x100,
    FireRending = 0x200,
    MeleeDefense = 0x400,
    MissileDefense = 0x800,
    MagicDefense = 0x1000,
    Spellbook = 0x2000,
    NetherRending = 0x4000,
    IgnoreSomeMagicProjectileDamage = 0x20000000,
    AlwaysCritical = 0x40000000,
    IgnoreAllArmor = 0x80000000,
}

impl crate::readers::ACDataType for ImbuedEffectType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ImbuedEffectType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ImbuedEffectType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ImbuedEffectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ImbuedEffectType::Undef => "Undef",
            ImbuedEffectType::CriticalStrike => "CriticalStrike",
            ImbuedEffectType::CripplingBlow => "CripplingBlow",
            ImbuedEffectType::ArmorRending => "ArmorRending",
            ImbuedEffectType::SlashRending => "SlashRending",
            ImbuedEffectType::PierceRending => "PierceRending",
            ImbuedEffectType::BludgeonRending => "BludgeonRending",
            ImbuedEffectType::AcidRending => "AcidRending",
            ImbuedEffectType::ColdRending => "ColdRending",
            ImbuedEffectType::ElectricRending => "ElectricRending",
            ImbuedEffectType::FireRending => "FireRending",
            ImbuedEffectType::MeleeDefense => "MeleeDefense",
            ImbuedEffectType::MissileDefense => "MissileDefense",
            ImbuedEffectType::MagicDefense => "MagicDefense",
            ImbuedEffectType::Spellbook => "Spellbook",
            ImbuedEffectType::NetherRending => "NetherRending",
            ImbuedEffectType::IgnoreSomeMagicProjectileDamage => "IgnoreSomeMagicProjectileDamage",
            ImbuedEffectType::AlwaysCritical => "AlwaysCritical",
            ImbuedEffectType::IgnoreAllArmor => "IgnoreAllArmor",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ItemXpStyle {
    Undef = 0x0,
    Fixed = 0x1,
    ScalesWithLevel = 0x2,
    FixedPlusBase = 0x3,
}

impl crate::readers::ACDataType for ItemXpStyle {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ItemXpStyle::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ItemXpStyle {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ItemXpStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ItemXpStyle::Undef => "Undef",
            ItemXpStyle::Fixed => "Fixed",
            ItemXpStyle::ScalesWithLevel => "ScalesWithLevel",
            ItemXpStyle::FixedPlusBase => "FixedPlusBase",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum SubscriptionStatus {
    #[serde(rename = "No_Subscription")]
    NoSubscription = 0x0,
    #[serde(rename = "AsheronsCall_Subscription")]
    AsheronsCallSubscription = 0x1,
    #[serde(rename = "DarkMajesty_Subscription")]
    DarkMajestySubscription = 0x2,
    #[serde(rename = "ThroneOfDestiny_Subscription")]
    ThroneOfDestinySubscription = 0x3,
    #[serde(rename = "ThroneOfDestiny_Preordered")]
    ThroneOfDestinyPreordered = 0x4,
}

impl crate::readers::ACDataType for SubscriptionStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(SubscriptionStatus::try_from(value)?)
    }
}

impl crate::writers::ACWritable for SubscriptionStatus {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SubscriptionStatus::NoSubscription => "No_Subscription",
            SubscriptionStatus::AsheronsCallSubscription => "AsheronsCall_Subscription",
            SubscriptionStatus::DarkMajestySubscription => "DarkMajesty_Subscription",
            SubscriptionStatus::ThroneOfDestinySubscription => "ThroneOfDestiny_Subscription",
            SubscriptionStatus::ThroneOfDestinyPreordered => "ThroneOfDestiny_Preordered",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum WeaponType {
    Undef = 0x0,
    Unarmed = 0x1,
    Sword = 0x2,
    Axe = 0x3,
    Mace = 0x4,
    Spear = 0x5,
    Dagger = 0x6,
    Staff = 0x7,
    Bow = 0x8,
    Crossbow = 0x9,
    Thrown = 0xA,
    TwoHanded = 0xB,
    Magic = 0xC,
}

impl crate::readers::ACDataType for WeaponType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(WeaponType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for WeaponType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WeaponType::Undef => "Undef",
            WeaponType::Unarmed => "Unarmed",
            WeaponType::Sword => "Sword",
            WeaponType::Axe => "Axe",
            WeaponType::Mace => "Mace",
            WeaponType::Spear => "Spear",
            WeaponType::Dagger => "Dagger",
            WeaponType::Staff => "Staff",
            WeaponType::Bow => "Bow",
            WeaponType::Crossbow => "Crossbow",
            WeaponType::Thrown => "Thrown",
            WeaponType::TwoHanded => "TwoHanded",
            WeaponType::Magic => "Magic",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ActivationResponse {
    Undef = 0x0,
    Use = 0x2,
    Animate = 0x4,
    Talk = 0x10,
    Emote = 0x800,
    CastSpell = 0x1000,
    Generate = 0x10000,
}

impl crate::readers::ACDataType for ActivationResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ActivationResponse::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ActivationResponse {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ActivationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ActivationResponse::Undef => "Undef",
            ActivationResponse::Use => "Use",
            ActivationResponse::Animate => "Animate",
            ActivationResponse::Talk => "Talk",
            ActivationResponse::Emote => "Emote",
            ActivationResponse::CastSpell => "CastSpell",
            ActivationResponse::Generate => "Generate",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AetheriaBitfield {
    None = 0x0,
    Blue = 0x1,
    Yellow = 0x2,
    Red = 0x4,
}

impl crate::readers::ACDataType for AetheriaBitfield {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AetheriaBitfield::try_from(value)?)
    }
}

impl crate::writers::ACWritable for AetheriaBitfield {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for AetheriaBitfield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AetheriaBitfield::None => "None",
            AetheriaBitfield::Blue => "Blue",
            AetheriaBitfield::Yellow => "Yellow",
            AetheriaBitfield::Red => "Red",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum HookGroupType {
    Undef = 0x0,
    NoisemakingItems = 0x1,
    TestItems = 0x2,
    PortalItems = 0x4,
    WritableItems = 0x8,
    SpellCastingItems = 0x10,
    SpellTeachingItems = 0x20,
}

impl crate::readers::ACDataType for HookGroupType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(HookGroupType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for HookGroupType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for HookGroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HookGroupType::Undef => "Undef",
            HookGroupType::NoisemakingItems => "NoisemakingItems",
            HookGroupType::TestItems => "TestItems",
            HookGroupType::PortalItems => "PortalItems",
            HookGroupType::WritableItems => "WritableItems",
            HookGroupType::SpellCastingItems => "SpellCastingItems",
            HookGroupType::SpellTeachingItems => "SpellTeachingItems",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ArmorType {
    None = 0x0,
    Cloth = 0x1,
    Leather = 0x2,
    StuddedLeather = 0x4,
    Scalemail = 0x8,
    Chainmail = 0x10,
    Metal = 0x20,
}

impl crate::readers::ACDataType for ArmorType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ArmorType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ArmorType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ArmorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ArmorType::None => "None",
            ArmorType::Cloth => "Cloth",
            ArmorType::Leather => "Leather",
            ArmorType::StuddedLeather => "StuddedLeather",
            ArmorType::Scalemail => "Scalemail",
            ArmorType::Chainmail => "Chainmail",
            ArmorType::Metal => "Metal",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AttunedStatus {
    Normal = 0x0,
    Attuned = 0x1,
    Sticky = 0x2,
}

impl crate::readers::ACDataType for AttunedStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AttunedStatus::try_from(value)?)
    }
}

impl crate::writers::ACWritable for AttunedStatus {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for AttunedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AttunedStatus::Normal => "Normal",
            AttunedStatus::Attuned => "Attuned",
            AttunedStatus::Sticky => "Sticky",
        };
        write!(f, "{}", s)
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum BondedStatus {
    Destroy = -2,
    Slippery = -1,
    Normal = 0x0,
    Bonded = 0x1,
    Sticky = 0x2,
}

impl crate::readers::ACDataType for BondedStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(BondedStatus::try_from(value)?)
    }
}

impl crate::writers::ACWritable for BondedStatus {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_i32(writer, self.clone() as i32)?;
        Ok(())
    }
}

impl std::fmt::Display for BondedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BondedStatus::Destroy => "Destroy",
            BondedStatus::Slippery => "Slippery",
            BondedStatus::Normal => "Normal",
            BondedStatus::Bonded => "Bonded",
            BondedStatus::Sticky => "Sticky",
        };
        write!(f, "{}", s)
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum HouseStatus {
    Disabled = -1,
    InActive = 0x0,
    Active = 0x1,
}

impl crate::readers::ACDataType for HouseStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(HouseStatus::try_from(value)?)
    }
}

impl crate::writers::ACWritable for HouseStatus {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_i32(writer, self.clone() as i32)?;
        Ok(())
    }
}

impl std::fmt::Display for HouseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HouseStatus::Disabled => "Disabled",
            HouseStatus::InActive => "InActive",
            HouseStatus::Active => "Active",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UiEffects: u32 {
        const UNDEF = 0x0;
        const MAGICAL = 0x1;
        const POISONED = 0x2;
        const BOOST_HEALTH = 0x4;
        const BOOST_MANA = 0x8;
        const BOOST_STAMINA = 0x10;
        const FIRE = 0x20;
        const LIGHTNING = 0x40;
        const FROST = 0x80;
        const ACID = 0x100;
        const BLUDGEONING = 0x200;
        const SLASHING = 0x400;
        const PIERCING = 0x800;
        const NETHER = 0x1000;
    }
}

impl crate::readers::ACDataType for UiEffects {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(UiEffects::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for UiEffects {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum PortalBitmask {
    NotPassable = 0x0,
    Unrestricted = 0x1,
    NoPk = 0x2,
    NoPKLite = 0x4,
    NoNPK = 0x8,
    NoSummon = 0x10,
    NoRecall = 0x20,
    OnlyOlthoiPCs = 0x40,
    NoOlthoiPCs = 0x80,
    NoVitae = 0x100,
    NoNewAccounts = 0x200,
}

impl crate::readers::ACDataType for PortalBitmask {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(PortalBitmask::try_from(value)?)
    }
}

impl crate::writers::ACWritable for PortalBitmask {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_i32(writer, self.clone() as i32)?;
        Ok(())
    }
}

impl std::fmt::Display for PortalBitmask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PortalBitmask::NotPassable => "NotPassable",
            PortalBitmask::Unrestricted => "Unrestricted",
            PortalBitmask::NoPk => "NoPk",
            PortalBitmask::NoPKLite => "NoPKLite",
            PortalBitmask::NoNPK => "NoNPK",
            PortalBitmask::NoSummon => "NoSummon",
            PortalBitmask::NoRecall => "NoRecall",
            PortalBitmask::OnlyOlthoiPCs => "OnlyOlthoiPCs",
            PortalBitmask::NoOlthoiPCs => "NoOlthoiPCs",
            PortalBitmask::NoVitae => "NoVitae",
            PortalBitmask::NoNewAccounts => "NoNewAccounts",
        };
        write!(f, "{}", s)
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum WieldRequirement {
    Undef = 0x0,
    Skill = 0x1,
    RawSkill = 0x2,
    Attrib = 0x3,
    RawAttrib = 0x4,
    SecondaryAttrib = 0x5,
    RawSecondaryAttrib = 0x6,
    Level = 0x7,
    Training = 0x8,
    IntStat = 0x9,
    BoolStat = 0xA,
    CreatureType = 0xB,
    HeritageType = 0xC,
}

impl crate::readers::ACDataType for WieldRequirement {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(WieldRequirement::try_from(value)?)
    }
}

impl crate::writers::ACWritable for WieldRequirement {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_i32(writer, self.clone() as i32)?;
        Ok(())
    }
}

impl std::fmt::Display for WieldRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WieldRequirement::Undef => "Undef",
            WieldRequirement::Skill => "Skill",
            WieldRequirement::RawSkill => "RawSkill",
            WieldRequirement::Attrib => "Attrib",
            WieldRequirement::RawAttrib => "RawAttrib",
            WieldRequirement::SecondaryAttrib => "SecondaryAttrib",
            WieldRequirement::RawSecondaryAttrib => "RawSecondaryAttrib",
            WieldRequirement::Level => "Level",
            WieldRequirement::Training => "Training",
            WieldRequirement::IntStat => "IntStat",
            WieldRequirement::BoolStat => "BoolStat",
            WieldRequirement::CreatureType => "CreatureType",
            WieldRequirement::HeritageType => "HeritageType",
        };
        write!(f, "{}", s)
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum PaletteTemplate {
    Undef = 0x0,
    AquaBlue = 0x1,
    Blue = 0x2,
    BluePurple = 0x3,
    Brown = 0x4,
    DarkBlue = 0x5,
    DeepBrown = 0x6,
    DeepGreen = 0x7,
    Green = 0x8,
    Grey = 0x9,
    LightBlue = 0xA,
    Maroon = 0xB,
    Navy = 0xC,
    Purple = 0xD,
    Red = 0xE,
    RedPurple = 0xF,
    Rose = 0x10,
    Yellow = 0x11,
    YellowBrown = 0x12,
    Copper = 0x13,
    Silver = 0x14,
    Gold = 0x15,
    Aqua = 0x16,
    DarkAquaMetal = 0x17,
    DarkBlueMetal = 0x18,
    DarkCopperMetal = 0x19,
    DarkGoldMetal = 0x1A,
    DarkGreenMetal = 0x1B,
    DarkPurpleMetal = 0x1C,
    DarkRedMetal = 0x1D,
    DarkSilverMetal = 0x1E,
    LightAquaMetal = 0x1F,
    LightBlueMetal = 0x20,
    LightCopperMetal = 0x21,
    LightGoldMetal = 0x22,
    LightGreenMetal = 0x23,
    LightPurpleMetal = 0x24,
    LightRedMetal = 0x25,
    LightSilverMetal = 0x26,
    Black = 0x27,
    Bronze = 0x28,
    SandyYellow = 0x29,
    DarkBrown = 0x2A,
    LightBrown = 0x2B,
    TanRed = 0x2C,
    PaleGreen = 0x2D,
    Tan = 0x2E,
    PastyYellow = 0x2F,
    SnowyWhite = 0x30,
    RuddyYellow = 0x31,
    RuddierYellow = 0x32,
    MidGrey = 0x33,
    DarkGrey = 0x34,
    BlueDullSilver = 0x35,
    YellowPaleSilver = 0x36,
    BrownBlueDark = 0x37,
    BrownBlueMed = 0x38,
    GreenSilver = 0x39,
    BrownGreen = 0x3A,
    YellowGreen = 0x3B,
    PalePurple = 0x3C,
    White = 0x3D,
    RedBrown = 0x3E,
    GreenBrown = 0x3F,
    OrangeBrown = 0x40,
    PaleGreenBrown = 0x41,
    PaleOrange = 0x42,
    GreenSlime = 0x43,
    BlueSlime = 0x44,
    YellowSlime = 0x45,
    PurpleSlime = 0x46,
    DullRed = 0x47,
    GreyWhite = 0x48,
    MediumGrey = 0x49,
    DullGreen = 0x4A,
    OliveGreen = 0x4B,
    Orange = 0x4C,
    BlueGreen = 0x4D,
    Olive = 0x4E,
    Lead = 0x4F,
    Iron = 0x50,
    LiteGreen = 0x51,
    PinkPurple = 0x52,
    Amber = 0x53,
    DyeDarkGreen = 0x54,
    DyeDarkRed = 0x55,
    DyeDarkYellow = 0x56,
    DyeBotched = 0x57,
    DyeWinterBlue = 0x58,
    DyeWinterGreen = 0x59,
    DyeWinterSilver = 0x5A,
    DyeSpringBlue = 0x5B,
    DyeSpringPurple = 0x5C,
    DyeSpringBlack = 0x5D,
}

impl crate::readers::ACDataType for PaletteTemplate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(PaletteTemplate::try_from(value)?)
    }
}

impl crate::writers::ACWritable for PaletteTemplate {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_i32(writer, self.clone() as i32)?;
        Ok(())
    }
}

impl std::fmt::Display for PaletteTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PaletteTemplate::Undef => "Undef",
            PaletteTemplate::AquaBlue => "AquaBlue",
            PaletteTemplate::Blue => "Blue",
            PaletteTemplate::BluePurple => "BluePurple",
            PaletteTemplate::Brown => "Brown",
            PaletteTemplate::DarkBlue => "DarkBlue",
            PaletteTemplate::DeepBrown => "DeepBrown",
            PaletteTemplate::DeepGreen => "DeepGreen",
            PaletteTemplate::Green => "Green",
            PaletteTemplate::Grey => "Grey",
            PaletteTemplate::LightBlue => "LightBlue",
            PaletteTemplate::Maroon => "Maroon",
            PaletteTemplate::Navy => "Navy",
            PaletteTemplate::Purple => "Purple",
            PaletteTemplate::Red => "Red",
            PaletteTemplate::RedPurple => "RedPurple",
            PaletteTemplate::Rose => "Rose",
            PaletteTemplate::Yellow => "Yellow",
            PaletteTemplate::YellowBrown => "YellowBrown",
            PaletteTemplate::Copper => "Copper",
            PaletteTemplate::Silver => "Silver",
            PaletteTemplate::Gold => "Gold",
            PaletteTemplate::Aqua => "Aqua",
            PaletteTemplate::DarkAquaMetal => "DarkAquaMetal",
            PaletteTemplate::DarkBlueMetal => "DarkBlueMetal",
            PaletteTemplate::DarkCopperMetal => "DarkCopperMetal",
            PaletteTemplate::DarkGoldMetal => "DarkGoldMetal",
            PaletteTemplate::DarkGreenMetal => "DarkGreenMetal",
            PaletteTemplate::DarkPurpleMetal => "DarkPurpleMetal",
            PaletteTemplate::DarkRedMetal => "DarkRedMetal",
            PaletteTemplate::DarkSilverMetal => "DarkSilverMetal",
            PaletteTemplate::LightAquaMetal => "LightAquaMetal",
            PaletteTemplate::LightBlueMetal => "LightBlueMetal",
            PaletteTemplate::LightCopperMetal => "LightCopperMetal",
            PaletteTemplate::LightGoldMetal => "LightGoldMetal",
            PaletteTemplate::LightGreenMetal => "LightGreenMetal",
            PaletteTemplate::LightPurpleMetal => "LightPurpleMetal",
            PaletteTemplate::LightRedMetal => "LightRedMetal",
            PaletteTemplate::LightSilverMetal => "LightSilverMetal",
            PaletteTemplate::Black => "Black",
            PaletteTemplate::Bronze => "Bronze",
            PaletteTemplate::SandyYellow => "SandyYellow",
            PaletteTemplate::DarkBrown => "DarkBrown",
            PaletteTemplate::LightBrown => "LightBrown",
            PaletteTemplate::TanRed => "TanRed",
            PaletteTemplate::PaleGreen => "PaleGreen",
            PaletteTemplate::Tan => "Tan",
            PaletteTemplate::PastyYellow => "PastyYellow",
            PaletteTemplate::SnowyWhite => "SnowyWhite",
            PaletteTemplate::RuddyYellow => "RuddyYellow",
            PaletteTemplate::RuddierYellow => "RuddierYellow",
            PaletteTemplate::MidGrey => "MidGrey",
            PaletteTemplate::DarkGrey => "DarkGrey",
            PaletteTemplate::BlueDullSilver => "BlueDullSilver",
            PaletteTemplate::YellowPaleSilver => "YellowPaleSilver",
            PaletteTemplate::BrownBlueDark => "BrownBlueDark",
            PaletteTemplate::BrownBlueMed => "BrownBlueMed",
            PaletteTemplate::GreenSilver => "GreenSilver",
            PaletteTemplate::BrownGreen => "BrownGreen",
            PaletteTemplate::YellowGreen => "YellowGreen",
            PaletteTemplate::PalePurple => "PalePurple",
            PaletteTemplate::White => "White",
            PaletteTemplate::RedBrown => "RedBrown",
            PaletteTemplate::GreenBrown => "GreenBrown",
            PaletteTemplate::OrangeBrown => "OrangeBrown",
            PaletteTemplate::PaleGreenBrown => "PaleGreenBrown",
            PaletteTemplate::PaleOrange => "PaleOrange",
            PaletteTemplate::GreenSlime => "GreenSlime",
            PaletteTemplate::BlueSlime => "BlueSlime",
            PaletteTemplate::YellowSlime => "YellowSlime",
            PaletteTemplate::PurpleSlime => "PurpleSlime",
            PaletteTemplate::DullRed => "DullRed",
            PaletteTemplate::GreyWhite => "GreyWhite",
            PaletteTemplate::MediumGrey => "MediumGrey",
            PaletteTemplate::DullGreen => "DullGreen",
            PaletteTemplate::OliveGreen => "OliveGreen",
            PaletteTemplate::Orange => "Orange",
            PaletteTemplate::BlueGreen => "BlueGreen",
            PaletteTemplate::Olive => "Olive",
            PaletteTemplate::Lead => "Lead",
            PaletteTemplate::Iron => "Iron",
            PaletteTemplate::LiteGreen => "LiteGreen",
            PaletteTemplate::PinkPurple => "PinkPurple",
            PaletteTemplate::Amber => "Amber",
            PaletteTemplate::DyeDarkGreen => "DyeDarkGreen",
            PaletteTemplate::DyeDarkRed => "DyeDarkRed",
            PaletteTemplate::DyeDarkYellow => "DyeDarkYellow",
            PaletteTemplate::DyeBotched => "DyeBotched",
            PaletteTemplate::DyeWinterBlue => "DyeWinterBlue",
            PaletteTemplate::DyeWinterGreen => "DyeWinterGreen",
            PaletteTemplate::DyeWinterSilver => "DyeWinterSilver",
            PaletteTemplate::DyeSpringBlue => "DyeSpringBlue",
            PaletteTemplate::DyeSpringPurple => "DyeSpringPurple",
            PaletteTemplate::DyeSpringBlack => "DyeSpringBlack",
        };
        write!(f, "{}", s)
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum SummoningMastery {
    Undef = 0x0,
    Primalist = 0x1,
    Necromancer = 0x2,
    Naturalist = 0x3,
}

impl crate::readers::ACDataType for SummoningMastery {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(SummoningMastery::try_from(value)?)
    }
}

impl crate::writers::ACWritable for SummoningMastery {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_i32(writer, self.clone() as i32)?;
        Ok(())
    }
}

impl std::fmt::Display for SummoningMastery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SummoningMastery::Undef => "Undef",
            SummoningMastery::Primalist => "Primalist",
            SummoningMastery::Necromancer => "Necromancer",
            SummoningMastery::Naturalist => "Naturalist",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ContractId {
    Undef = 0x0,
    #[serde(rename = "Contract_1_The_Shadows_of_Bitter_Winter")]
    Contract1TheShadowsOfBitterWinter = 0x1,
    #[serde(rename = "Contract_2_Test_Quest_Stamping")]
    Contract2TestQuestStamping = 0x2,
    #[serde(rename = "Contract_3_Test_Contract_3")]
    Contract3TestContract3 = 0x3,
    #[serde(rename = "Contract_4_Test_Contract_4")]
    Contract4TestContract4 = 0x4,
    #[serde(rename = "Contract_5_Reign_of_Terror")]
    Contract5ReignOfTerror = 0x5,
    #[serde(rename = "Contract_6_Glenden_Wood_Invasion_Low")]
    Contract6GlendenWoodInvasionLow = 0x6,
    #[serde(rename = "Contract_7_Glenden_Wood_Invasion_Mid")]
    Contract7GlendenWoodInvasionMid = 0x7,
    #[serde(rename = "Contract_8_Glenden_Wood_Invasion_High")]
    Contract8GlendenWoodInvasionHigh = 0x8,
    #[serde(rename = "Contract_9_Frozen_Fury")]
    Contract9FrozenFury = 0x9,
    #[serde(rename = "Contract_10_Defense_of_Zaikhal_Copper")]
    Contract10DefenseOfZaikhalCopper = 0xA,
    #[serde(rename = "Contract_11_Defense_of_Zaikhal_Silver")]
    Contract11DefenseOfZaikhalSilver = 0xB,
    #[serde(rename = "Contract_12_Defense_of_Zaikhal_Gold")]
    Contract12DefenseOfZaikhalGold = 0xC,
    #[serde(rename = "Contract_13_Defense_of_Zaikhal_Platinum")]
    Contract13DefenseOfZaikhalPlatinum = 0xD,
    #[serde(rename = "Contract_14_The_Caliginous_Bethel")]
    Contract14TheCaliginousBethel = 0xE,
    #[serde(rename = "Contract_15_The_Legend_of_the_Tusker_Paw")]
    Contract15TheLegendOfTheTuskerPaw = 0xF,
    #[serde(rename = "Contract_16_Oswalds_Lair")]
    Contract16OswaldsLair = 0x10,
    #[serde(rename = "Contract_17_The_Decrepit_Tower")]
    Contract17TheDecrepitTower = 0x11,
    #[serde(rename = "Contract_18_Banderling_Haunt")]
    Contract18BanderlingHaunt = 0x12,
    #[serde(rename = "Contract_19_Reconnaissance")]
    Contract19Reconnaissance = 0x13,
    #[serde(rename = "Contract_20_Assault_Low")]
    Contract20AssaultLow = 0x14,
    #[serde(rename = "Contract_21_Assault_Mid")]
    Contract21AssaultMid = 0x15,
    #[serde(rename = "Contract_22_Assault_High")]
    Contract22AssaultHigh = 0x16,
    #[serde(rename = "Contract_23_Assault_Expert")]
    Contract23AssaultExpert = 0x17,
    #[serde(rename = "Contract_24_Infiltration")]
    Contract24Infiltration = 0x18,
    #[serde(rename = "Contract_25_Of_Trust_and_Betrayal")]
    Contract25OfTrustAndBetrayal = 0x19,
    #[serde(rename = "Contract_26_Ishaqs_Lost_Key")]
    Contract26IshaqsLostKey = 0x1A,
    #[serde(rename = "Contract_27_The_Shadows_of_Bitter_Winter")]
    Contract27TheShadowsOfBitterWinter = 0x1B,
    #[serde(rename = "Contract_28_Suzuhara_Baijins_Delivery")]
    Contract28SuzuharaBaijinsDelivery = 0x1C,
    #[serde(rename = "Contract_29_Haleatan_Beach_Camps")]
    Contract29HaleatanBeachCamps = 0x1D,
    #[serde(rename = "Contract_30_Ricardos_Blood_Gem")]
    Contract30RicardosBloodGem = 0x1E,
    #[serde(rename = "Contract_31_Sawato_Extortion")]
    Contract31SawatoExtortion = 0x1F,
    #[serde(rename = "Contract_32_First_Contact")]
    Contract32FirstContact = 0x20,
    #[serde(rename = "Contract_33_Crafting_Forges_Low")]
    Contract33CraftingForgesLow = 0x21,
    #[serde(rename = "Contract_34_Crafting_Forges_Mid")]
    Contract34CraftingForgesMid = 0x22,
    #[serde(rename = "Contract_35_Crafting_Forges_High")]
    Contract35CraftingForgesHigh = 0x23,
    #[serde(rename = "Contract_36_Northern_Shroud_Cabal")]
    Contract36NorthernShroudCabal = 0x24,
    #[serde(rename = "Contract_37_Southern_Shroud_Cabal")]
    Contract37SouthernShroudCabal = 0x25,
    #[serde(rename = "Contract_38_Faces_of_the_Mukkir_Low")]
    Contract38FacesOfTheMukkirLow = 0x26,
    #[serde(rename = "Contract_39_Faces_of_the_Mukkir_Mid")]
    Contract39FacesOfTheMukkirMid = 0x27,
    #[serde(rename = "Contract_40_Faces_of_the_Mukkir_High")]
    Contract40FacesOfTheMukkirHigh = 0x28,
    #[serde(rename = "Contract_41_Faces_of_the_Mukkir_Expert")]
    Contract41FacesOfTheMukkirExpert = 0x29,
    #[serde(rename = "Contract_42_Fiun_Healing_Machine")]
    Contract42FiunHealingMachine = 0x2A,
    #[serde(rename = "Contract_43_Hamuds_Demise")]
    Contract43HamudsDemise = 0x2B,
    #[serde(rename = "Contract_44_Raising_Graels_Island")]
    Contract44RaisingGraelsIsland = 0x2C,
    #[serde(rename = "Contract_45_Enricos_Betrayal")]
    Contract45EnricosBetrayal = 0x2D,
    #[serde(rename = "Contract_46_Lost_Pet")]
    Contract46LostPet = 0x2E,
    #[serde(rename = "Contract_47_His_Masters_Voice")]
    Contract47HisMastersVoice = 0x2F,
    #[serde(rename = "Contract_48_Tentacles_of_Tthuun")]
    Contract48TentaclesOfTthuun = 0x30,
    #[serde(rename = "Contract_49_Reign_of_Terror")]
    Contract49ReignOfTerror = 0x31,
    #[serde(rename = "Contract_50_The_Crystal_Staff_of_the_Anekshay")]
    Contract50TheCrystalStaffOfTheAnekshay = 0x32,
    #[serde(rename = "Contract_51_The_Crystal_Sword_of_the_Anekshay")]
    Contract51TheCrystalSwordOfTheAnekshay = 0x33,
    #[serde(rename = "Contract_52_The_Crystal_Amulet_of_the_Anekshay")]
    Contract52TheCrystalAmuletOfTheAnekshay = 0x34,
    #[serde(rename = "Contract_53_The_Crystal_Idol_of_the_Anekshay")]
    Contract53TheCrystalIdolOfTheAnekshay = 0x35,
    #[serde(rename = "Contract_54_Armoredillo_Hunting__Lost_City_of_Neftet")]
    Contract54ArmoredilloHuntingLostCityOfNeftet = 0x36,
    #[serde(rename = "Contract_55_Golem_Hunting__Lost_City_of_Neftet")]
    Contract55GolemHuntingLostCityOfNeftet = 0x37,
    #[serde(rename = "Contract_56_Mu_miyah_Hunting__Lost_City_of_Neftet")]
    Contract56MuMiyahHuntingLostCityOfNeftet = 0x38,
    #[serde(rename = "Contract_57_Reedshark_Hunting__Lost_City_of_Neftet")]
    Contract57ReedsharkHuntingLostCityOfNeftet = 0x39,
    #[serde(rename = "Contract_58_Anekshay_Bracer_Collecting__Lost_City_of_Neftet")]
    Contract58AnekshayBracerCollectingLostCityOfNeftet = 0x3A,
    #[serde(rename = "Contract_59_Stone_Tablet_Collecting__Lost_City_of_Neftet")]
    Contract59StoneTabletCollectingLostCityOfNeftet = 0x3B,
    #[serde(rename = "Contract_60_Prickly_Pear_Collecting__Lost_City_of_Neftet")]
    Contract60PricklyPearCollectingLostCityOfNeftet = 0x3C,
    #[serde(rename = "Contract_61_Contracts__Brokers")]
    Contract61ContractsBrokers = 0x3D,
    #[serde(rename = "Contract_62_Aug__Sir_Bellas")]
    Contract62AugSirBellas = 0x3E,
    #[serde(rename = "Contract_63_Aug__Society")]
    Contract63AugSociety = 0x3F,
    #[serde(rename = "Contract_64_Aug__Diemos")]
    Contract64AugDiemos = 0x40,
    #[serde(rename = "Contract_65_Aug__Luminance")]
    Contract65AugLuminance = 0x41,
    #[serde(rename = "Contract_66_Colosseum")]
    Contract66Colosseum = 0x42,
    #[serde(rename = "Contract_67_Aerbaxs_Defeat")]
    Contract67AerbaxsDefeat = 0x43,
    #[serde(rename = "Contract_68_Summoning_Tthuun")]
    Contract68SummoningTthuun = 0x44,
    #[serde(rename = "Contract_69_Empyrean_Rescue")]
    Contract69EmpyreanRescue = 0x45,
    #[serde(rename = "Contract_70_Uncovering_the_Renegades")]
    Contract70UncoveringTheRenegades = 0x46,
    #[serde(rename = "Contract_71_Tumerok_Salted_Meat")]
    Contract71TumerokSaltedMeat = 0x47,
    #[serde(rename = "Contract_72_Deewains_Dark_Cavern")]
    Contract72DeewainsDarkCavern = 0x48,
    #[serde(rename = "Contract_73_Sealing_Away_the_Book_of_Eibhil")]
    Contract73SealingAwayTheBookOfEibhil = 0x49,
    #[serde(rename = "Contract_74_Soc__Dark_Isle_Delivery")]
    Contract74SocDarkIsleDelivery = 0x4A,
    #[serde(rename = "Contract_75_Soc__Vaeshok")]
    Contract75SocVaeshok = 0x4B,
    #[serde(rename = "Contract_76_Soc__Shambling_Archivist")]
    Contract76SocShamblingArchivist = 0x4C,
    #[serde(rename = "Contract_77_Soc__Undead_Jaw_Collection")]
    Contract77SocUndeadJawCollection = 0x4D,
    #[serde(rename = "Contract_78_Soc__Wight_Blade_Sorcerers")]
    Contract78SocWightBladeSorcerers = 0x4E,
    #[serde(rename = "Contract_79_Soc__Black_Coral_Collection")]
    Contract79SocBlackCoralCollection = 0x4F,
    #[serde(rename = "Contract_80_Soc__Dark_Isle_Scouting")]
    Contract80SocDarkIsleScouting = 0x50,
    #[serde(rename = "Contract_81_Soc__Bandit_Mana_Hunter_Boss")]
    Contract81SocBanditManaHunterBoss = 0x51,
    #[serde(rename = "Contract_82_Soc__Mana_Infused_Jungle_Flowers")]
    Contract82SocManaInfusedJungleFlowers = 0x52,
    #[serde(rename = "Contract_83_Soc__Jungle_Lilies")]
    Contract83SocJungleLilies = 0x53,
    #[serde(rename = "Contract_84_Soc__Moar_Glands")]
    Contract84SocMoarGlands = 0x54,
    #[serde(rename = "Contract_85_Soc__Blessed_Moarsmen")]
    Contract85SocBlessedMoarsmen = 0x55,
    #[serde(rename = "Contract_86_Soc__Phyntos_Hive_Splinters")]
    Contract86SocPhyntosHiveSplinters = 0x56,
    #[serde(rename = "Contract_87_Soc__Phyntos_Honey")]
    Contract87SocPhyntosHoney = 0x57,
    #[serde(rename = "Contract_88_Soc__Phyntos_Queen")]
    Contract88SocPhyntosQueen = 0x58,
    #[serde(rename = "Contract_89_Soc__Phyntos_Larvae")]
    Contract89SocPhyntosLarvae = 0x59,
    #[serde(rename = "Contract_90_Soc__Killer_Phyntos_Wasps")]
    Contract90SocKillerPhyntosWasps = 0x5A,
    #[serde(rename = "Contract_91_Soc__Coral_Towers")]
    Contract91SocCoralTowers = 0x5B,
    #[serde(rename = "Contract_92_Soc__Magshuth_Moarsmen")]
    Contract92SocMagshuthMoarsmen = 0x5C,
    #[serde(rename = "Contract_93_Soc__Moarsman_High_Priest")]
    Contract93SocMoarsmanHighPriest = 0x5D,
    #[serde(rename = "Contract_94_Soc__Artifact_Collection")]
    Contract94SocArtifactCollection = 0x5E,
    #[serde(rename = "Contract_95_Soc__Moguth_Moarsmen")]
    Contract95SocMoguthMoarsmen = 0x5F,
    #[serde(rename = "Contract_96_Soc__Shoguth_Moarsmen")]
    Contract96SocShoguthMoarsmen = 0x60,
    #[serde(rename = "Contract_97_Soc__Spawning_Pools")]
    Contract97SocSpawningPools = 0x61,
    #[serde(rename = "Contract_98_Soc__Graveyard_Delivery")]
    Contract98SocGraveyardDelivery = 0x62,
    #[serde(rename = "Contract_99_Soc__Stone_Tracings")]
    Contract99SocStoneTracings = 0x63,
    #[serde(rename = "Contract_100_Soc__Falatacot_Reports")]
    Contract100SocFalatacotReports = 0x64,
    #[serde(rename = "Contract_101_Soc__Dark_Isle_Delivery")]
    Contract101SocDarkIsleDelivery = 0x65,
    #[serde(rename = "Contract_102_Soc__Vaeshok")]
    Contract102SocVaeshok = 0x66,
    #[serde(rename = "Contract_103_Soc__Shambling_Archivist")]
    Contract103SocShamblingArchivist = 0x67,
    #[serde(rename = "Contract_104_Soc__Undead_Jaw_Collection")]
    Contract104SocUndeadJawCollection = 0x68,
    #[serde(rename = "Contract_105_Soc__Wight_Blade_Sorcerers")]
    Contract105SocWightBladeSorcerers = 0x69,
    #[serde(rename = "Contract_106_Soc__Black_Coral_Collection")]
    Contract106SocBlackCoralCollection = 0x6A,
    #[serde(rename = "Contract_107_Soc__Dark_Isle_Scouting")]
    Contract107SocDarkIsleScouting = 0x6B,
    #[serde(rename = "Contract_108_Soc__Bandit_Mana_Hunter_Boss")]
    Contract108SocBanditManaHunterBoss = 0x6C,
    #[serde(rename = "Contract_109_Soc__Mana_Infused_Jungle_Flowers")]
    Contract109SocManaInfusedJungleFlowers = 0x6D,
    #[serde(rename = "Contract_110_Soc__Jungle_Lilies")]
    Contract110SocJungleLilies = 0x6E,
    #[serde(rename = "Contract_111_Soc__Moar_Glands")]
    Contract111SocMoarGlands = 0x6F,
    #[serde(rename = "Contract_112_Soc__Blessed_Moarsmen")]
    Contract112SocBlessedMoarsmen = 0x70,
    #[serde(rename = "Contract_113_Soc__Phyntos_Hive_Splinters")]
    Contract113SocPhyntosHiveSplinters = 0x71,
    #[serde(rename = "Contract_114_Soc__Phyntos_Honey")]
    Contract114SocPhyntosHoney = 0x72,
    #[serde(rename = "Contract_115_Soc__Phyntos_Queen")]
    Contract115SocPhyntosQueen = 0x73,
    #[serde(rename = "Contract_116_Soc__Phyntos_Larvae")]
    Contract116SocPhyntosLarvae = 0x74,
    #[serde(rename = "Contract_117_Soc__Killer_Phyntos_Wasps")]
    Contract117SocKillerPhyntosWasps = 0x75,
    #[serde(rename = "Contract_118_Soc__Coral_Towers")]
    Contract118SocCoralTowers = 0x76,
    #[serde(rename = "Contract_119_Soc__Magshuth_Moarsmen")]
    Contract119SocMagshuthMoarsmen = 0x77,
    #[serde(rename = "Contract_120_Soc__Moarsman_High_Priest")]
    Contract120SocMoarsmanHighPriest = 0x78,
    #[serde(rename = "Contract_121_Soc__Artifact_Collection")]
    Contract121SocArtifactCollection = 0x79,
    #[serde(rename = "Contract_122_Soc__Moguth_Moarsmen")]
    Contract122SocMoguthMoarsmen = 0x7A,
    #[serde(rename = "Contract_123_Soc__Shoguth_Moarsmen")]
    Contract123SocShoguthMoarsmen = 0x7B,
    #[serde(rename = "Contract_124_Soc__Spawning_Pools")]
    Contract124SocSpawningPools = 0x7C,
    #[serde(rename = "Contract_125_Soc__Graveyard_Delivery")]
    Contract125SocGraveyardDelivery = 0x7D,
    #[serde(rename = "Contract_126_Soc__Stone_Tracings")]
    Contract126SocStoneTracings = 0x7E,
    #[serde(rename = "Contract_127_Soc__Falatacot_Reports")]
    Contract127SocFalatacotReports = 0x7F,
    #[serde(rename = "Contract_128_Soc__Dark_Isle_Delivery")]
    Contract128SocDarkIsleDelivery = 0x80,
    #[serde(rename = "Contract_129_Soc__Vaeshok")]
    Contract129SocVaeshok = 0x81,
    #[serde(rename = "Contract_130_Soc__Shambling_Archivist")]
    Contract130SocShamblingArchivist = 0x82,
    #[serde(rename = "Contract_131_Soc__Undead_Jaw_Collection")]
    Contract131SocUndeadJawCollection = 0x83,
    #[serde(rename = "Contract_132_Soc__Wight_Blade_Sorcerers")]
    Contract132SocWightBladeSorcerers = 0x84,
    #[serde(rename = "Contract_133_Soc__Black_Coral_Collection")]
    Contract133SocBlackCoralCollection = 0x85,
    #[serde(rename = "Contract_134_Soc__Dark_Isle_Scouting")]
    Contract134SocDarkIsleScouting = 0x86,
    #[serde(rename = "Contract_135_Soc__Bandit_Mana_Hunter_Boss")]
    Contract135SocBanditManaHunterBoss = 0x87,
    #[serde(rename = "Contract_136_Soc__Mana_Infused_Jungle_Flowers")]
    Contract136SocManaInfusedJungleFlowers = 0x88,
    #[serde(rename = "Contract_137_Soc__Jungle_Lilies")]
    Contract137SocJungleLilies = 0x89,
    #[serde(rename = "Contract_138_Soc__Moar_Glands")]
    Contract138SocMoarGlands = 0x8A,
    #[serde(rename = "Contract_139_Soc__Blessed_Moarsmen")]
    Contract139SocBlessedMoarsmen = 0x8B,
    #[serde(rename = "Contract_140_Soc__Phyntos_Hive_Splinters")]
    Contract140SocPhyntosHiveSplinters = 0x8C,
    #[serde(rename = "Contract_141_Soc__Phyntos_Honey")]
    Contract141SocPhyntosHoney = 0x8D,
    #[serde(rename = "Contract_142_Soc__Phyntos_Queen")]
    Contract142SocPhyntosQueen = 0x8E,
    #[serde(rename = "Contract_143_Soc__Phyntos_Larvae")]
    Contract143SocPhyntosLarvae = 0x8F,
    #[serde(rename = "Contract_144_Soc__Killer_Phyntos_Wasps")]
    Contract144SocKillerPhyntosWasps = 0x90,
    #[serde(rename = "Contract_145_Soc__Coral_Towers")]
    Contract145SocCoralTowers = 0x91,
    #[serde(rename = "Contract_146_Soc__Magshuth_Moarsmen")]
    Contract146SocMagshuthMoarsmen = 0x92,
    #[serde(rename = "Contract_147_Soc__Moarsman_High_Priest")]
    Contract147SocMoarsmanHighPriest = 0x93,
    #[serde(rename = "Contract_148_Soc__Artifact_Collection")]
    Contract148SocArtifactCollection = 0x94,
    #[serde(rename = "Contract_149_Soc__Moguth_Moarsmen")]
    Contract149SocMoguthMoarsmen = 0x95,
    #[serde(rename = "Contract_150_Soc__Shoguth_Moarsmen")]
    Contract150SocShoguthMoarsmen = 0x96,
    #[serde(rename = "Contract_151_Soc__Spawning_Pools")]
    Contract151SocSpawningPools = 0x97,
    #[serde(rename = "Contract_152_Soc__Graveyard_Delivery")]
    Contract152SocGraveyardDelivery = 0x98,
    #[serde(rename = "Contract_153_Soc__Stone_Tracings")]
    Contract153SocStoneTracings = 0x99,
    #[serde(rename = "Contract_154_Soc__Falatacot_Reports")]
    Contract154SocFalatacotReports = 0x9A,
    #[serde(rename = "Contract_155_Soc__Palm_Fort")]
    Contract155SocPalmFort = 0x9B,
    #[serde(rename = "Contract_156_Soc__Supply_Saboteur")]
    Contract156SocSupplySaboteur = 0x9C,
    #[serde(rename = "Contract_157_Soc__Forgotten_Tunnels_of_Nyrleha")]
    Contract157SocForgottenTunnelsOfNyrleha = 0x9D,
    #[serde(rename = "Contract_158_Soc__Palm_Fort")]
    Contract158SocPalmFort = 0x9E,
    #[serde(rename = "Contract_159_Soc__Supply_Saboteur")]
    Contract159SocSupplySaboteur = 0x9F,
    #[serde(rename = "Contract_160_Soc__Forgotten_Tunnels_of_Nyrleha")]
    Contract160SocForgottenTunnelsOfNyrleha = 0xA0,
    #[serde(rename = "Contract_161_Soc__Palm_Fort")]
    Contract161SocPalmFort = 0xA1,
    #[serde(rename = "Contract_162_Soc__Supply_Saboteur")]
    Contract162SocSupplySaboteur = 0xA2,
    #[serde(rename = "Contract_163_Soc__Forgotten_Tunnels_of_Nyrleha")]
    Contract163SocForgottenTunnelsOfNyrleha = 0xA3,
    #[serde(rename = "Contract_164_Kill__Tenebrous_Rifts")]
    Contract164KillTenebrousRifts = 0xA4,
    #[serde(rename = "Contract_165_Kill__Umbral_Rifts")]
    Contract165KillUmbralRifts = 0xA5,
    #[serde(rename = "Contract_166_Harlunes_Diplomacy")]
    Contract166HarlunesDiplomacy = 0xA6,
    #[serde(rename = "Contract_167_Saving_Asheron")]
    Contract167SavingAsheron = 0xA7,
    #[serde(rename = "Contract_168_Menhir_Research")]
    Contract168MenhirResearch = 0xA8,
    #[serde(rename = "Contract_169_Gear_Knight_Excavation")]
    Contract169GearKnightExcavation = 0xA9,
    #[serde(rename = "Contract_170_Nexus_Crawl")]
    Contract170NexusCrawl = 0xAA,
    #[serde(rename = "Contract_171_Jester_Released")]
    Contract171JesterReleased = 0xAB,
    #[serde(rename = "Contract_172_Vision_Quest")]
    Contract172VisionQuest = 0xAC,
    #[serde(rename = "Contract_173_Aerbaxs_Prodigal_Monouga")]
    Contract173AerbaxsProdigalMonouga = 0xAD,
    #[serde(rename = "Contract_174_QotM__Weekly_1")]
    Contract174QotMWeekly1 = 0xAE,
    #[serde(rename = "Contract_175_QotM__Weekly_2")]
    Contract175QotMWeekly2 = 0xAF,
    #[serde(rename = "Contract_176_QotM__Weekly_3")]
    Contract176QotMWeekly3 = 0xB0,
    #[serde(rename = "Contract_177_Deaths_Allure")]
    Contract177DeathsAllure = 0xB1,
    #[serde(rename = "Contract_178_Yanshi_Tunnels")]
    Contract178YanshiTunnels = 0xB2,
    #[serde(rename = "Contract_179_Kill__Gurog_Minions")]
    Contract179KillGurogMinions = 0xB3,
    #[serde(rename = "Contract_180_Kill__Gurog_Soldiers")]
    Contract180KillGurogSoldiers = 0xB4,
    #[serde(rename = "Contract_181_Kill__Gurog_Henchmen")]
    Contract181KillGurogHenchmen = 0xB5,
    #[serde(rename = "Contract_182_Aerbaxs_Prodigal_Tusker")]
    Contract182AerbaxsProdigalTusker = 0xB6,
    #[serde(rename = "Contract_183_Find_the_Barkeeper")]
    Contract183FindTheBarkeeper = 0xB7,
    #[serde(rename = "Contract_184_Find_the_Barkeeper")]
    Contract184FindTheBarkeeper = 0xB8,
    #[serde(rename = "Contract_185_Find_the_Barkeeper")]
    Contract185FindTheBarkeeper = 0xB9,
    #[serde(rename = "Contract_186_Find_the_Barkeeper")]
    Contract186FindTheBarkeeper = 0xBA,
    #[serde(rename = "Contract_187_Find_the_Pathwarden")]
    Contract187FindThePathwarden = 0xBB,
    #[serde(rename = "Contract_188_Find_the_Pathwarden")]
    Contract188FindThePathwarden = 0xBC,
    #[serde(rename = "Contract_189_Find_the_Pathwarden")]
    Contract189FindThePathwarden = 0xBD,
    #[serde(rename = "Contract_190_Find_the_Pathwarden")]
    Contract190FindThePathwarden = 0xBE,
    #[serde(rename = "Contract_191_Drudge_Hideout")]
    Contract191DrudgeHideout = 0xBF,
    #[serde(rename = "Contract_192_Holtburg_Redoubt")]
    Contract192HoltburgRedoubt = 0xC0,
    #[serde(rename = "Contract_193_The_Beacon")]
    Contract193TheBeacon = 0xC1,
    #[serde(rename = "Contract_194_The_Missing_Necklace")]
    Contract194TheMissingNecklace = 0xC2,
    #[serde(rename = "Contract_195_Braid_Mansion_Ruin")]
    Contract195BraidMansionRuin = 0xC3,
    #[serde(rename = "Contract_196_Nen_Ais_Pet_Drudge")]
    Contract196NenAisPetDrudge = 0xC4,
    #[serde(rename = "Contract_197_Sea_Temple_Catacombs")]
    Contract197SeaTempleCatacombs = 0xC5,
    #[serde(rename = "Contract_198_Under_Cove_Crypt")]
    Contract198UnderCoveCrypt = 0xC6,
    #[serde(rename = "Contract_199_Facility_Hub")]
    Contract199FacilityHub = 0xC7,
    #[serde(rename = "Contract_200_Jailbreak__Ardent_Leader")]
    Contract200JailbreakArdentLeader = 0xC8,
    #[serde(rename = "Contract_201_Jailbreak__Blessed_Leader")]
    Contract201JailbreakBlessedLeader = 0xC9,
    #[serde(rename = "Contract_202_Jailbreak__Verdant_Leader")]
    Contract202JailbreakVerdantLeader = 0xCA,
    #[serde(rename = "Contract_203_Jailbreak__General_Population")]
    Contract203JailbreakGeneralPopulation = 0xCB,
    #[serde(rename = "Contract_204_Gurog_Creation")]
    Contract204GurogCreation = 0xCC,
    #[serde(rename = "Contract_205_Wardley_and_the_Wights")]
    Contract205WardleyAndTheWights = 0xCD,
    #[serde(rename = "Contract_206_Aetherium_Ore_Collection")]
    Contract206AetheriumOreCollection = 0xCE,
    #[serde(rename = "Contract_207_Aetherium_Power_Core_Collection")]
    Contract207AetheriumPowerCoreCollection = 0xCF,
    #[serde(rename = "Contract_208_Aetherium_Raid_High")]
    Contract208AetheriumRaidHigh = 0xD0,
    #[serde(rename = "Contract_209_Soc__Mana_Siphon_Destruction")]
    Contract209SocManaSiphonDestruction = 0xD1,
    #[serde(rename = "Contract_210_Kill__Gear_Knight_Knights")]
    Contract210KillGearKnightKnights = 0xD2,
    #[serde(rename = "Contract_211_Kill__Gear_Knight_Commander")]
    Contract211KillGearKnightCommander = 0xD3,
    #[serde(rename = "Contract_212_Nalicanas_Test")]
    Contract212NalicanasTest = 0xD4,
    #[serde(rename = "Contract_213_Bloodstone_Investigation")]
    Contract213BloodstoneInvestigation = 0xD5,
    #[serde(rename = "Contract_214_Chasing_Oswald")]
    Contract214ChasingOswald = 0xD6,
    #[serde(rename = "Contract_215_Hunting_Aun_Ralirea")]
    Contract215HuntingAunRalirea = 0xD7,
    #[serde(rename = "Contract_216_Aerbaxs_Prodigal_Monouga")]
    Contract216AerbaxsProdigalMonouga = 0xD8,
    #[serde(rename = "Contract_217_Aerbaxs_Prodigal_Drudge")]
    Contract217AerbaxsProdigalDrudge = 0xD9,
    #[serde(rename = "Contract_218_Aerbaxs_Prodigal_Human")]
    Contract218AerbaxsProdigalHuman = 0xDA,
    #[serde(rename = "Contract_219_Kidnapped_Handmaiden")]
    Contract219KidnappedHandmaiden = 0xDB,
    #[serde(rename = "Contract_220_Sepulcher_of_Nightmares")]
    Contract220SepulcherOfNightmares = 0xDC,
    #[serde(rename = "Contract_221_Mhoire_Castle")]
    Contract221MhoireCastle = 0xDD,
    #[serde(rename = "Contract_222_Bobos_Medicine")]
    Contract222BobosMedicine = 0xDE,
    #[serde(rename = "Contract_223_Mhoire_Oubliette")]
    Contract223MhoireOubliette = 0xDF,
    #[serde(rename = "Contract_224_Geraines_Study")]
    Contract224GerainesStudy = 0xE0,
    #[serde(rename = "Contract_225_Geraines_Hosts")]
    Contract225GerainesHosts = 0xE1,
    #[serde(rename = "Contract_226_Splitting_Grael_High")]
    Contract226SplittingGraelHigh = 0xE2,
    #[serde(rename = "Contract_227_Splitting_Grael_Mid")]
    Contract227SplittingGraelMid = 0xE3,
    #[serde(rename = "Contract_228_Splitting_Grael_Low")]
    Contract228SplittingGraelLow = 0xE4,
    #[serde(rename = "Contract_229_Clutch_of_Kings__Reeshan")]
    Contract229ClutchOfKingsReeshan = 0xE5,
    #[serde(rename = "Contract_230_Clutch_of_Kings__Kiree")]
    Contract230ClutchOfKingsKiree = 0xE6,
    #[serde(rename = "Contract_231_Clutch_of_Kings__Broodu")]
    Contract231ClutchOfKingsBroodu = 0xE7,
    #[serde(rename = "Contract_232_Clutch_of_Kings__Keerik")]
    Contract232ClutchOfKingsKeerik = 0xE8,
    #[serde(rename = "Contract_233_Clutch_of_Kings__Rehir")]
    Contract233ClutchOfKingsRehir = 0xE9,
    #[serde(rename = "Contract_234_Clutch_of_Kings__Browerk")]
    Contract234ClutchOfKingsBrowerk = 0xEA,
    #[serde(rename = "Contract_235_Clutch_of_Kings__All")]
    Contract235ClutchOfKingsAll = 0xEB,
    #[serde(rename = "Contract_236_Kill__Spectral_Archers")]
    Contract236KillSpectralArchers = 0xEC,
    #[serde(rename = "Contract_237_Kill__Spectral_Minions")]
    Contract237KillSpectralMinions = 0xED,
    #[serde(rename = "Contract_238_Kill__Spectral_Nanjou_Shou_jen")]
    Contract238KillSpectralNanjouShouJen = 0xEE,
    #[serde(rename = "Contract_239_Kill__Spectral_Mages")]
    Contract239KillSpectralMages = 0xEF,
    #[serde(rename = "Contract_240_Kill__Spectral_Bushi")]
    Contract240KillSpectralBushi = 0xF0,
    #[serde(rename = "Contract_241_Kill__Spectral_Samurai")]
    Contract241KillSpectralSamurai = 0xF1,
    #[serde(rename = "Contract_242_Kill__Spectral_Blades_and_Claws")]
    Contract242KillSpectralBladesAndClaws = 0xF2,
    #[serde(rename = "Contract_243_Kill__Spectral_Samurai_Golems")]
    Contract243KillSpectralSamuraiGolems = 0xF3,
    #[serde(rename = "Contract_244_Hoshino_Fortress")]
    Contract244HoshinoFortress = 0xF4,
    #[serde(rename = "Contract_245_Stipend__General")]
    Contract245StipendGeneral = 0xF5,
    #[serde(rename = "Contract_246_Stipend__Celestial_Hand")]
    Contract246StipendCelestialHand = 0xF6,
    #[serde(rename = "Contract_247_Stipend__Radiant_Blood")]
    Contract247StipendRadiantBlood = 0xF7,
    #[serde(rename = "Contract_248_Stipend__Eldrytch_Web")]
    Contract248StipendEldrytchWeb = 0xF8,
    #[serde(rename = "Contract_249_Jester_Focuses")]
    Contract249JesterFocuses = 0xF9,
    #[serde(rename = "Contract_250_Unleash_the_Gearknights")]
    Contract250UnleashTheGearknights = 0xFA,
    #[serde(rename = "Contract_251_Virindi_Rescue")]
    Contract251VirindiRescue = 0xFB,
    #[serde(rename = "Contract_252_Ninja_Academy")]
    Contract252NinjaAcademy = 0xFC,
    #[serde(rename = "Contract_253_Tanada_Slaughter")]
    Contract253TanadaSlaughter = 0xFD,
    #[serde(rename = "Contract_254_Tanada_Intercept")]
    Contract254TanadaIntercept = 0xFE,
    #[serde(rename = "Contract_255_Crystalline_Adventurer")]
    Contract255CrystallineAdventurer = 0xFF,
    #[serde(rename = "Contract_256_Crystalline_Markers")]
    Contract256CrystallineMarkers = 0x100,
    #[serde(rename = "Contract_257_Crystalline_Killer")]
    Contract257CrystallineKiller = 0x101,
    #[serde(rename = "Contract_258_Crystalline_Bound_Wisp")]
    Contract258CrystallineBoundWisp = 0x102,
    #[serde(rename = "Contract_259_Nanjou_Stockade")]
    Contract259NanjouStockade = 0x103,
    #[serde(rename = "Contract_260_Mage_Academy")]
    Contract260MageAcademy = 0x104,
    #[serde(rename = "Contract_261_Apostate_Finale")]
    Contract261ApostateFinale = 0x105,
    #[serde(rename = "Contract_262_Lunnums_Return")]
    Contract262LunnumsReturn = 0x106,
    #[serde(rename = "Contract_263_Lunnums_Pyre")]
    Contract263LunnumsPyre = 0x107,
    #[serde(rename = "Contract_264_Lunnums_Disappearance")]
    Contract264LunnumsDisappearance = 0x108,
    #[serde(rename = "Contract_265_Lost_Lore")]
    Contract265LostLore = 0x109,
    #[serde(rename = "Contract_266_Sisters_of_Light")]
    Contract266SistersOfLight = 0x10A,
    #[serde(rename = "Contract_267_First_Sister")]
    Contract267FirstSister = 0x10B,
    #[serde(rename = "Contract_268_Second_Sister")]
    Contract268SecondSister = 0x10C,
    #[serde(rename = "Contract_269_Third_Sister")]
    Contract269ThirdSister = 0x10D,
    #[serde(rename = "Contract_270_Ritual_Investigation")]
    Contract270RitualInvestigation = 0x10E,
    #[serde(rename = "Contract_271_Ritual_Disruption")]
    Contract271RitualDisruption = 0x10F,
    #[serde(rename = "Contract_272_Defeat_Hoshino_Kei")]
    Contract272DefeatHoshinoKei = 0x110,
    #[serde(rename = "Contract_273_Protecting_Picketed_Pets")]
    Contract273ProtectingPicketedPets = 0x111,
    #[serde(rename = "Contract_274_Buried_Alive")]
    Contract274BuriedAlive = 0x112,
    #[serde(rename = "Contract_275_Graverobber")]
    Contract275Graverobber = 0x113,
    #[serde(rename = "Contract_276_Escape")]
    Contract276Escape = 0x114,
    #[serde(rename = "Contract_277_Deconstruction")]
    Contract277Deconstruction = 0x115,
    #[serde(rename = "Contract_278_Uziz_Abductions")]
    Contract278UzizAbductions = 0x116,
    #[serde(rename = "Contract_279_Golem_Hunters__Mud_Golem_Sludge_Lord")]
    Contract279GolemHuntersMudGolemSludgeLord = 0x117,
    #[serde(rename = "Contract_280_Golem_Hunters__Copper_Golem_Kingpin")]
    Contract280GolemHuntersCopperGolemKingpin = 0x118,
    #[serde(rename = "Contract_281_Golem_Hunters__Glacial_Golem_Margrave")]
    Contract281GolemHuntersGlacialGolemMargrave = 0x119,
    #[serde(rename = "Contract_282_Golem_Hunters__Magma_Golem_Exarch")]
    Contract282GolemHuntersMagmaGolemExarch = 0x11A,
    #[serde(rename = "Contract_283_Golem_Hunters__Coral_Golem_Viceroy")]
    Contract283GolemHuntersCoralGolemViceroy = 0x11B,
    #[serde(rename = "Contract_284_Golem_Hunters__Platinum_Golem_Mountain_King")]
    Contract284GolemHuntersPlatinumGolemMountainKing = 0x11C,
    #[serde(rename = "Contract_285_Olthoi_Hive_Queen")]
    Contract285OlthoiHiveQueen = 0x11D,
    #[serde(rename = "Contract_286_Soc__Mana_Siphon_Destruction")]
    Contract286SocManaSiphonDestruction = 0x11E,
    #[serde(rename = "Contract_287_Soc__Mana_Siphon_Destruction")]
    Contract287SocManaSiphonDestruction = 0x11F,
    #[serde(rename = "Contract_288_Soc__Destroy_The_Phalanx")]
    Contract288SocDestroyThePhalanx = 0x120,
    #[serde(rename = "Contract_289_Soc__Destroy_The_Phalanx")]
    Contract289SocDestroyThePhalanx = 0x121,
    #[serde(rename = "Contract_290_Soc__Destroy_The_Phalanx")]
    Contract290SocDestroyThePhalanx = 0x122,
    #[serde(rename = "Contract_291_Soc__Collect_Gear_Knight_Parts")]
    Contract291SocCollectGearKnightParts = 0x123,
    #[serde(rename = "Contract_292_Soc__Collect_Gear_Knight_Parts")]
    Contract292SocCollectGearKnightParts = 0x124,
    #[serde(rename = "Contract_293_Soc__Collect_Gear_Knight_Parts")]
    Contract293SocCollectGearKnightParts = 0x125,
    #[serde(rename = "Contract_294_Kill__Gear_Knight_Squires")]
    Contract294KillGearKnightSquires = 0x126,
    #[serde(rename = "Contract_295_Behind_The_Mask")]
    Contract295BehindTheMask = 0x127,
    #[serde(rename = "Contract_296_Frozen_Fortress_Laboratory")]
    Contract296FrozenFortressLaboratory = 0x128,
    #[serde(rename = "Contract_297_Frozen_Fortress_Testing_Grounds")]
    Contract297FrozenFortressTestingGrounds = 0x129,
    #[serde(rename = "Contract_298_Olthoi_Hive_Warrior_Pincer")]
    Contract298OlthoiHiveWarriorPincer = 0x12A,
    #[serde(rename = "Contract_299_Olthoi_Hive_Eviscerator_Pincer")]
    Contract299OlthoiHiveEvisceratorPincer = 0x12B,
    #[serde(rename = "Contract_300_Snow_Tusker_Leader_Tusk")]
    Contract300SnowTuskerLeaderTusk = 0x12C,
    #[serde(rename = "Contract_301_Journey_To_Madness")]
    Contract301JourneyToMadness = 0x12D,
    #[serde(rename = "Contract_302_Visitors")]
    Contract302Visitors = 0x12E,
    #[serde(rename = "Contract_303_Kill__Rynthid_Minions")]
    Contract303KillRynthidMinions = 0x12F,
    #[serde(rename = "Contract_304_Kill__Empowered_Wisps")]
    Contract304KillEmpoweredWisps = 0x130,
    #[serde(rename = "Contract_305_Kill__Rynthid_Rare_Boss")]
    Contract305KillRynthidRareBoss = 0x131,
    #[serde(rename = "Contract_306_Kill__Rynthid_Slayers")]
    Contract306KillRynthidSlayers = 0x132,
    #[serde(rename = "Contract_307_Kill__Rynthid_Ragers")]
    Contract307KillRynthidRagers = 0x133,
    #[serde(rename = "Contract_308_Kill__Rynthid_Sorcerers")]
    Contract308KillRynthidSorcerers = 0x134,
    #[serde(rename = "Contract_309_Kill__Rynthid_Rifts")]
    Contract309KillRynthidRifts = 0x135,
    #[serde(rename = "Contract_310_Legendary_Quests")]
    Contract310LegendaryQuests = 0x136,
    #[serde(rename = "Contract_311_Rynthid_Genesis")]
    Contract311RynthidGenesis = 0x137,
    #[serde(rename = "Contract_312_Changing_Gears")]
    Contract312ChangingGears = 0x138,
    #[serde(rename = "Contract_313_Fear_Factory")]
    Contract313FearFactory = 0x139,
    #[serde(rename = "Contract_314_Spirited_Halls")]
    Contract314SpiritedHalls = 0x13A,
    #[serde(rename = "Contract_315_End_of_Days")]
    Contract315EndOfDays = 0x13B,
    #[serde(rename = "Contract_316_Lugian_Assault")]
    Contract316LugianAssault = 0x13C,
    #[serde(rename = "Contract_317_Rynthid_Training")]
    Contract317RynthidTraining = 0x13D,
    #[serde(rename = "Contract_318_Kill__Tou_Tou_Shadow_Flyers")]
    Contract318KillTouTouShadowFlyers = 0x13E,
    #[serde(rename = "Contract_319_Kill__Tou_Tou_Grievver_Shredders")]
    Contract319KillTouTouGrievverShredders = 0x13F,
    #[serde(rename = "Contract_320_Kill__Tou_Tou_Devourer_Marguls")]
    Contract320KillTouTouDevourerMarguls = 0x140,
    #[serde(rename = "Contract_321_Kill__Tou_Tou_Shadows")]
    Contract321KillTouTouShadows = 0x141,
    #[serde(rename = "Contract_322_Kill__Tou_Tou_Void_Lords")]
    Contract322KillTouTouVoidLords = 0x142,
}

impl crate::readers::ACDataType for ContractId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ContractId::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ContractId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ContractId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ContractId::Undef => "Undef",
            ContractId::Contract1TheShadowsOfBitterWinter => "Contract_1_The_Shadows_of_Bitter_Winter",
            ContractId::Contract2TestQuestStamping => "Contract_2_Test_Quest_Stamping",
            ContractId::Contract3TestContract3 => "Contract_3_Test_Contract_3",
            ContractId::Contract4TestContract4 => "Contract_4_Test_Contract_4",
            ContractId::Contract5ReignOfTerror => "Contract_5_Reign_of_Terror",
            ContractId::Contract6GlendenWoodInvasionLow => "Contract_6_Glenden_Wood_Invasion_Low",
            ContractId::Contract7GlendenWoodInvasionMid => "Contract_7_Glenden_Wood_Invasion_Mid",
            ContractId::Contract8GlendenWoodInvasionHigh => "Contract_8_Glenden_Wood_Invasion_High",
            ContractId::Contract9FrozenFury => "Contract_9_Frozen_Fury",
            ContractId::Contract10DefenseOfZaikhalCopper => "Contract_10_Defense_of_Zaikhal_Copper",
            ContractId::Contract11DefenseOfZaikhalSilver => "Contract_11_Defense_of_Zaikhal_Silver",
            ContractId::Contract12DefenseOfZaikhalGold => "Contract_12_Defense_of_Zaikhal_Gold",
            ContractId::Contract13DefenseOfZaikhalPlatinum => "Contract_13_Defense_of_Zaikhal_Platinum",
            ContractId::Contract14TheCaliginousBethel => "Contract_14_The_Caliginous_Bethel",
            ContractId::Contract15TheLegendOfTheTuskerPaw => "Contract_15_The_Legend_of_the_Tusker_Paw",
            ContractId::Contract16OswaldsLair => "Contract_16_Oswalds_Lair",
            ContractId::Contract17TheDecrepitTower => "Contract_17_The_Decrepit_Tower",
            ContractId::Contract18BanderlingHaunt => "Contract_18_Banderling_Haunt",
            ContractId::Contract19Reconnaissance => "Contract_19_Reconnaissance",
            ContractId::Contract20AssaultLow => "Contract_20_Assault_Low",
            ContractId::Contract21AssaultMid => "Contract_21_Assault_Mid",
            ContractId::Contract22AssaultHigh => "Contract_22_Assault_High",
            ContractId::Contract23AssaultExpert => "Contract_23_Assault_Expert",
            ContractId::Contract24Infiltration => "Contract_24_Infiltration",
            ContractId::Contract25OfTrustAndBetrayal => "Contract_25_Of_Trust_and_Betrayal",
            ContractId::Contract26IshaqsLostKey => "Contract_26_Ishaqs_Lost_Key",
            ContractId::Contract27TheShadowsOfBitterWinter => "Contract_27_The_Shadows_of_Bitter_Winter",
            ContractId::Contract28SuzuharaBaijinsDelivery => "Contract_28_Suzuhara_Baijins_Delivery",
            ContractId::Contract29HaleatanBeachCamps => "Contract_29_Haleatan_Beach_Camps",
            ContractId::Contract30RicardosBloodGem => "Contract_30_Ricardos_Blood_Gem",
            ContractId::Contract31SawatoExtortion => "Contract_31_Sawato_Extortion",
            ContractId::Contract32FirstContact => "Contract_32_First_Contact",
            ContractId::Contract33CraftingForgesLow => "Contract_33_Crafting_Forges_Low",
            ContractId::Contract34CraftingForgesMid => "Contract_34_Crafting_Forges_Mid",
            ContractId::Contract35CraftingForgesHigh => "Contract_35_Crafting_Forges_High",
            ContractId::Contract36NorthernShroudCabal => "Contract_36_Northern_Shroud_Cabal",
            ContractId::Contract37SouthernShroudCabal => "Contract_37_Southern_Shroud_Cabal",
            ContractId::Contract38FacesOfTheMukkirLow => "Contract_38_Faces_of_the_Mukkir_Low",
            ContractId::Contract39FacesOfTheMukkirMid => "Contract_39_Faces_of_the_Mukkir_Mid",
            ContractId::Contract40FacesOfTheMukkirHigh => "Contract_40_Faces_of_the_Mukkir_High",
            ContractId::Contract41FacesOfTheMukkirExpert => "Contract_41_Faces_of_the_Mukkir_Expert",
            ContractId::Contract42FiunHealingMachine => "Contract_42_Fiun_Healing_Machine",
            ContractId::Contract43HamudsDemise => "Contract_43_Hamuds_Demise",
            ContractId::Contract44RaisingGraelsIsland => "Contract_44_Raising_Graels_Island",
            ContractId::Contract45EnricosBetrayal => "Contract_45_Enricos_Betrayal",
            ContractId::Contract46LostPet => "Contract_46_Lost_Pet",
            ContractId::Contract47HisMastersVoice => "Contract_47_His_Masters_Voice",
            ContractId::Contract48TentaclesOfTthuun => "Contract_48_Tentacles_of_Tthuun",
            ContractId::Contract49ReignOfTerror => "Contract_49_Reign_of_Terror",
            ContractId::Contract50TheCrystalStaffOfTheAnekshay => "Contract_50_The_Crystal_Staff_of_the_Anekshay",
            ContractId::Contract51TheCrystalSwordOfTheAnekshay => "Contract_51_The_Crystal_Sword_of_the_Anekshay",
            ContractId::Contract52TheCrystalAmuletOfTheAnekshay => "Contract_52_The_Crystal_Amulet_of_the_Anekshay",
            ContractId::Contract53TheCrystalIdolOfTheAnekshay => "Contract_53_The_Crystal_Idol_of_the_Anekshay",
            ContractId::Contract54ArmoredilloHuntingLostCityOfNeftet => "Contract_54_Armoredillo_Hunting__Lost_City_of_Neftet",
            ContractId::Contract55GolemHuntingLostCityOfNeftet => "Contract_55_Golem_Hunting__Lost_City_of_Neftet",
            ContractId::Contract56MuMiyahHuntingLostCityOfNeftet => "Contract_56_Mu_miyah_Hunting__Lost_City_of_Neftet",
            ContractId::Contract57ReedsharkHuntingLostCityOfNeftet => "Contract_57_Reedshark_Hunting__Lost_City_of_Neftet",
            ContractId::Contract58AnekshayBracerCollectingLostCityOfNeftet => "Contract_58_Anekshay_Bracer_Collecting__Lost_City_of_Neftet",
            ContractId::Contract59StoneTabletCollectingLostCityOfNeftet => "Contract_59_Stone_Tablet_Collecting__Lost_City_of_Neftet",
            ContractId::Contract60PricklyPearCollectingLostCityOfNeftet => "Contract_60_Prickly_Pear_Collecting__Lost_City_of_Neftet",
            ContractId::Contract61ContractsBrokers => "Contract_61_Contracts__Brokers",
            ContractId::Contract62AugSirBellas => "Contract_62_Aug__Sir_Bellas",
            ContractId::Contract63AugSociety => "Contract_63_Aug__Society",
            ContractId::Contract64AugDiemos => "Contract_64_Aug__Diemos",
            ContractId::Contract65AugLuminance => "Contract_65_Aug__Luminance",
            ContractId::Contract66Colosseum => "Contract_66_Colosseum",
            ContractId::Contract67AerbaxsDefeat => "Contract_67_Aerbaxs_Defeat",
            ContractId::Contract68SummoningTthuun => "Contract_68_Summoning_Tthuun",
            ContractId::Contract69EmpyreanRescue => "Contract_69_Empyrean_Rescue",
            ContractId::Contract70UncoveringTheRenegades => "Contract_70_Uncovering_the_Renegades",
            ContractId::Contract71TumerokSaltedMeat => "Contract_71_Tumerok_Salted_Meat",
            ContractId::Contract72DeewainsDarkCavern => "Contract_72_Deewains_Dark_Cavern",
            ContractId::Contract73SealingAwayTheBookOfEibhil => "Contract_73_Sealing_Away_the_Book_of_Eibhil",
            ContractId::Contract74SocDarkIsleDelivery => "Contract_74_Soc__Dark_Isle_Delivery",
            ContractId::Contract75SocVaeshok => "Contract_75_Soc__Vaeshok",
            ContractId::Contract76SocShamblingArchivist => "Contract_76_Soc__Shambling_Archivist",
            ContractId::Contract77SocUndeadJawCollection => "Contract_77_Soc__Undead_Jaw_Collection",
            ContractId::Contract78SocWightBladeSorcerers => "Contract_78_Soc__Wight_Blade_Sorcerers",
            ContractId::Contract79SocBlackCoralCollection => "Contract_79_Soc__Black_Coral_Collection",
            ContractId::Contract80SocDarkIsleScouting => "Contract_80_Soc__Dark_Isle_Scouting",
            ContractId::Contract81SocBanditManaHunterBoss => "Contract_81_Soc__Bandit_Mana_Hunter_Boss",
            ContractId::Contract82SocManaInfusedJungleFlowers => "Contract_82_Soc__Mana_Infused_Jungle_Flowers",
            ContractId::Contract83SocJungleLilies => "Contract_83_Soc__Jungle_Lilies",
            ContractId::Contract84SocMoarGlands => "Contract_84_Soc__Moar_Glands",
            ContractId::Contract85SocBlessedMoarsmen => "Contract_85_Soc__Blessed_Moarsmen",
            ContractId::Contract86SocPhyntosHiveSplinters => "Contract_86_Soc__Phyntos_Hive_Splinters",
            ContractId::Contract87SocPhyntosHoney => "Contract_87_Soc__Phyntos_Honey",
            ContractId::Contract88SocPhyntosQueen => "Contract_88_Soc__Phyntos_Queen",
            ContractId::Contract89SocPhyntosLarvae => "Contract_89_Soc__Phyntos_Larvae",
            ContractId::Contract90SocKillerPhyntosWasps => "Contract_90_Soc__Killer_Phyntos_Wasps",
            ContractId::Contract91SocCoralTowers => "Contract_91_Soc__Coral_Towers",
            ContractId::Contract92SocMagshuthMoarsmen => "Contract_92_Soc__Magshuth_Moarsmen",
            ContractId::Contract93SocMoarsmanHighPriest => "Contract_93_Soc__Moarsman_High_Priest",
            ContractId::Contract94SocArtifactCollection => "Contract_94_Soc__Artifact_Collection",
            ContractId::Contract95SocMoguthMoarsmen => "Contract_95_Soc__Moguth_Moarsmen",
            ContractId::Contract96SocShoguthMoarsmen => "Contract_96_Soc__Shoguth_Moarsmen",
            ContractId::Contract97SocSpawningPools => "Contract_97_Soc__Spawning_Pools",
            ContractId::Contract98SocGraveyardDelivery => "Contract_98_Soc__Graveyard_Delivery",
            ContractId::Contract99SocStoneTracings => "Contract_99_Soc__Stone_Tracings",
            ContractId::Contract100SocFalatacotReports => "Contract_100_Soc__Falatacot_Reports",
            ContractId::Contract101SocDarkIsleDelivery => "Contract_101_Soc__Dark_Isle_Delivery",
            ContractId::Contract102SocVaeshok => "Contract_102_Soc__Vaeshok",
            ContractId::Contract103SocShamblingArchivist => "Contract_103_Soc__Shambling_Archivist",
            ContractId::Contract104SocUndeadJawCollection => "Contract_104_Soc__Undead_Jaw_Collection",
            ContractId::Contract105SocWightBladeSorcerers => "Contract_105_Soc__Wight_Blade_Sorcerers",
            ContractId::Contract106SocBlackCoralCollection => "Contract_106_Soc__Black_Coral_Collection",
            ContractId::Contract107SocDarkIsleScouting => "Contract_107_Soc__Dark_Isle_Scouting",
            ContractId::Contract108SocBanditManaHunterBoss => "Contract_108_Soc__Bandit_Mana_Hunter_Boss",
            ContractId::Contract109SocManaInfusedJungleFlowers => "Contract_109_Soc__Mana_Infused_Jungle_Flowers",
            ContractId::Contract110SocJungleLilies => "Contract_110_Soc__Jungle_Lilies",
            ContractId::Contract111SocMoarGlands => "Contract_111_Soc__Moar_Glands",
            ContractId::Contract112SocBlessedMoarsmen => "Contract_112_Soc__Blessed_Moarsmen",
            ContractId::Contract113SocPhyntosHiveSplinters => "Contract_113_Soc__Phyntos_Hive_Splinters",
            ContractId::Contract114SocPhyntosHoney => "Contract_114_Soc__Phyntos_Honey",
            ContractId::Contract115SocPhyntosQueen => "Contract_115_Soc__Phyntos_Queen",
            ContractId::Contract116SocPhyntosLarvae => "Contract_116_Soc__Phyntos_Larvae",
            ContractId::Contract117SocKillerPhyntosWasps => "Contract_117_Soc__Killer_Phyntos_Wasps",
            ContractId::Contract118SocCoralTowers => "Contract_118_Soc__Coral_Towers",
            ContractId::Contract119SocMagshuthMoarsmen => "Contract_119_Soc__Magshuth_Moarsmen",
            ContractId::Contract120SocMoarsmanHighPriest => "Contract_120_Soc__Moarsman_High_Priest",
            ContractId::Contract121SocArtifactCollection => "Contract_121_Soc__Artifact_Collection",
            ContractId::Contract122SocMoguthMoarsmen => "Contract_122_Soc__Moguth_Moarsmen",
            ContractId::Contract123SocShoguthMoarsmen => "Contract_123_Soc__Shoguth_Moarsmen",
            ContractId::Contract124SocSpawningPools => "Contract_124_Soc__Spawning_Pools",
            ContractId::Contract125SocGraveyardDelivery => "Contract_125_Soc__Graveyard_Delivery",
            ContractId::Contract126SocStoneTracings => "Contract_126_Soc__Stone_Tracings",
            ContractId::Contract127SocFalatacotReports => "Contract_127_Soc__Falatacot_Reports",
            ContractId::Contract128SocDarkIsleDelivery => "Contract_128_Soc__Dark_Isle_Delivery",
            ContractId::Contract129SocVaeshok => "Contract_129_Soc__Vaeshok",
            ContractId::Contract130SocShamblingArchivist => "Contract_130_Soc__Shambling_Archivist",
            ContractId::Contract131SocUndeadJawCollection => "Contract_131_Soc__Undead_Jaw_Collection",
            ContractId::Contract132SocWightBladeSorcerers => "Contract_132_Soc__Wight_Blade_Sorcerers",
            ContractId::Contract133SocBlackCoralCollection => "Contract_133_Soc__Black_Coral_Collection",
            ContractId::Contract134SocDarkIsleScouting => "Contract_134_Soc__Dark_Isle_Scouting",
            ContractId::Contract135SocBanditManaHunterBoss => "Contract_135_Soc__Bandit_Mana_Hunter_Boss",
            ContractId::Contract136SocManaInfusedJungleFlowers => "Contract_136_Soc__Mana_Infused_Jungle_Flowers",
            ContractId::Contract137SocJungleLilies => "Contract_137_Soc__Jungle_Lilies",
            ContractId::Contract138SocMoarGlands => "Contract_138_Soc__Moar_Glands",
            ContractId::Contract139SocBlessedMoarsmen => "Contract_139_Soc__Blessed_Moarsmen",
            ContractId::Contract140SocPhyntosHiveSplinters => "Contract_140_Soc__Phyntos_Hive_Splinters",
            ContractId::Contract141SocPhyntosHoney => "Contract_141_Soc__Phyntos_Honey",
            ContractId::Contract142SocPhyntosQueen => "Contract_142_Soc__Phyntos_Queen",
            ContractId::Contract143SocPhyntosLarvae => "Contract_143_Soc__Phyntos_Larvae",
            ContractId::Contract144SocKillerPhyntosWasps => "Contract_144_Soc__Killer_Phyntos_Wasps",
            ContractId::Contract145SocCoralTowers => "Contract_145_Soc__Coral_Towers",
            ContractId::Contract146SocMagshuthMoarsmen => "Contract_146_Soc__Magshuth_Moarsmen",
            ContractId::Contract147SocMoarsmanHighPriest => "Contract_147_Soc__Moarsman_High_Priest",
            ContractId::Contract148SocArtifactCollection => "Contract_148_Soc__Artifact_Collection",
            ContractId::Contract149SocMoguthMoarsmen => "Contract_149_Soc__Moguth_Moarsmen",
            ContractId::Contract150SocShoguthMoarsmen => "Contract_150_Soc__Shoguth_Moarsmen",
            ContractId::Contract151SocSpawningPools => "Contract_151_Soc__Spawning_Pools",
            ContractId::Contract152SocGraveyardDelivery => "Contract_152_Soc__Graveyard_Delivery",
            ContractId::Contract153SocStoneTracings => "Contract_153_Soc__Stone_Tracings",
            ContractId::Contract154SocFalatacotReports => "Contract_154_Soc__Falatacot_Reports",
            ContractId::Contract155SocPalmFort => "Contract_155_Soc__Palm_Fort",
            ContractId::Contract156SocSupplySaboteur => "Contract_156_Soc__Supply_Saboteur",
            ContractId::Contract157SocForgottenTunnelsOfNyrleha => "Contract_157_Soc__Forgotten_Tunnels_of_Nyrleha",
            ContractId::Contract158SocPalmFort => "Contract_158_Soc__Palm_Fort",
            ContractId::Contract159SocSupplySaboteur => "Contract_159_Soc__Supply_Saboteur",
            ContractId::Contract160SocForgottenTunnelsOfNyrleha => "Contract_160_Soc__Forgotten_Tunnels_of_Nyrleha",
            ContractId::Contract161SocPalmFort => "Contract_161_Soc__Palm_Fort",
            ContractId::Contract162SocSupplySaboteur => "Contract_162_Soc__Supply_Saboteur",
            ContractId::Contract163SocForgottenTunnelsOfNyrleha => "Contract_163_Soc__Forgotten_Tunnels_of_Nyrleha",
            ContractId::Contract164KillTenebrousRifts => "Contract_164_Kill__Tenebrous_Rifts",
            ContractId::Contract165KillUmbralRifts => "Contract_165_Kill__Umbral_Rifts",
            ContractId::Contract166HarlunesDiplomacy => "Contract_166_Harlunes_Diplomacy",
            ContractId::Contract167SavingAsheron => "Contract_167_Saving_Asheron",
            ContractId::Contract168MenhirResearch => "Contract_168_Menhir_Research",
            ContractId::Contract169GearKnightExcavation => "Contract_169_Gear_Knight_Excavation",
            ContractId::Contract170NexusCrawl => "Contract_170_Nexus_Crawl",
            ContractId::Contract171JesterReleased => "Contract_171_Jester_Released",
            ContractId::Contract172VisionQuest => "Contract_172_Vision_Quest",
            ContractId::Contract173AerbaxsProdigalMonouga => "Contract_173_Aerbaxs_Prodigal_Monouga",
            ContractId::Contract174QotMWeekly1 => "Contract_174_QotM__Weekly_1",
            ContractId::Contract175QotMWeekly2 => "Contract_175_QotM__Weekly_2",
            ContractId::Contract176QotMWeekly3 => "Contract_176_QotM__Weekly_3",
            ContractId::Contract177DeathsAllure => "Contract_177_Deaths_Allure",
            ContractId::Contract178YanshiTunnels => "Contract_178_Yanshi_Tunnels",
            ContractId::Contract179KillGurogMinions => "Contract_179_Kill__Gurog_Minions",
            ContractId::Contract180KillGurogSoldiers => "Contract_180_Kill__Gurog_Soldiers",
            ContractId::Contract181KillGurogHenchmen => "Contract_181_Kill__Gurog_Henchmen",
            ContractId::Contract182AerbaxsProdigalTusker => "Contract_182_Aerbaxs_Prodigal_Tusker",
            ContractId::Contract183FindTheBarkeeper => "Contract_183_Find_the_Barkeeper",
            ContractId::Contract184FindTheBarkeeper => "Contract_184_Find_the_Barkeeper",
            ContractId::Contract185FindTheBarkeeper => "Contract_185_Find_the_Barkeeper",
            ContractId::Contract186FindTheBarkeeper => "Contract_186_Find_the_Barkeeper",
            ContractId::Contract187FindThePathwarden => "Contract_187_Find_the_Pathwarden",
            ContractId::Contract188FindThePathwarden => "Contract_188_Find_the_Pathwarden",
            ContractId::Contract189FindThePathwarden => "Contract_189_Find_the_Pathwarden",
            ContractId::Contract190FindThePathwarden => "Contract_190_Find_the_Pathwarden",
            ContractId::Contract191DrudgeHideout => "Contract_191_Drudge_Hideout",
            ContractId::Contract192HoltburgRedoubt => "Contract_192_Holtburg_Redoubt",
            ContractId::Contract193TheBeacon => "Contract_193_The_Beacon",
            ContractId::Contract194TheMissingNecklace => "Contract_194_The_Missing_Necklace",
            ContractId::Contract195BraidMansionRuin => "Contract_195_Braid_Mansion_Ruin",
            ContractId::Contract196NenAisPetDrudge => "Contract_196_Nen_Ais_Pet_Drudge",
            ContractId::Contract197SeaTempleCatacombs => "Contract_197_Sea_Temple_Catacombs",
            ContractId::Contract198UnderCoveCrypt => "Contract_198_Under_Cove_Crypt",
            ContractId::Contract199FacilityHub => "Contract_199_Facility_Hub",
            ContractId::Contract200JailbreakArdentLeader => "Contract_200_Jailbreak__Ardent_Leader",
            ContractId::Contract201JailbreakBlessedLeader => "Contract_201_Jailbreak__Blessed_Leader",
            ContractId::Contract202JailbreakVerdantLeader => "Contract_202_Jailbreak__Verdant_Leader",
            ContractId::Contract203JailbreakGeneralPopulation => "Contract_203_Jailbreak__General_Population",
            ContractId::Contract204GurogCreation => "Contract_204_Gurog_Creation",
            ContractId::Contract205WardleyAndTheWights => "Contract_205_Wardley_and_the_Wights",
            ContractId::Contract206AetheriumOreCollection => "Contract_206_Aetherium_Ore_Collection",
            ContractId::Contract207AetheriumPowerCoreCollection => "Contract_207_Aetherium_Power_Core_Collection",
            ContractId::Contract208AetheriumRaidHigh => "Contract_208_Aetherium_Raid_High",
            ContractId::Contract209SocManaSiphonDestruction => "Contract_209_Soc__Mana_Siphon_Destruction",
            ContractId::Contract210KillGearKnightKnights => "Contract_210_Kill__Gear_Knight_Knights",
            ContractId::Contract211KillGearKnightCommander => "Contract_211_Kill__Gear_Knight_Commander",
            ContractId::Contract212NalicanasTest => "Contract_212_Nalicanas_Test",
            ContractId::Contract213BloodstoneInvestigation => "Contract_213_Bloodstone_Investigation",
            ContractId::Contract214ChasingOswald => "Contract_214_Chasing_Oswald",
            ContractId::Contract215HuntingAunRalirea => "Contract_215_Hunting_Aun_Ralirea",
            ContractId::Contract216AerbaxsProdigalMonouga => "Contract_216_Aerbaxs_Prodigal_Monouga",
            ContractId::Contract217AerbaxsProdigalDrudge => "Contract_217_Aerbaxs_Prodigal_Drudge",
            ContractId::Contract218AerbaxsProdigalHuman => "Contract_218_Aerbaxs_Prodigal_Human",
            ContractId::Contract219KidnappedHandmaiden => "Contract_219_Kidnapped_Handmaiden",
            ContractId::Contract220SepulcherOfNightmares => "Contract_220_Sepulcher_of_Nightmares",
            ContractId::Contract221MhoireCastle => "Contract_221_Mhoire_Castle",
            ContractId::Contract222BobosMedicine => "Contract_222_Bobos_Medicine",
            ContractId::Contract223MhoireOubliette => "Contract_223_Mhoire_Oubliette",
            ContractId::Contract224GerainesStudy => "Contract_224_Geraines_Study",
            ContractId::Contract225GerainesHosts => "Contract_225_Geraines_Hosts",
            ContractId::Contract226SplittingGraelHigh => "Contract_226_Splitting_Grael_High",
            ContractId::Contract227SplittingGraelMid => "Contract_227_Splitting_Grael_Mid",
            ContractId::Contract228SplittingGraelLow => "Contract_228_Splitting_Grael_Low",
            ContractId::Contract229ClutchOfKingsReeshan => "Contract_229_Clutch_of_Kings__Reeshan",
            ContractId::Contract230ClutchOfKingsKiree => "Contract_230_Clutch_of_Kings__Kiree",
            ContractId::Contract231ClutchOfKingsBroodu => "Contract_231_Clutch_of_Kings__Broodu",
            ContractId::Contract232ClutchOfKingsKeerik => "Contract_232_Clutch_of_Kings__Keerik",
            ContractId::Contract233ClutchOfKingsRehir => "Contract_233_Clutch_of_Kings__Rehir",
            ContractId::Contract234ClutchOfKingsBrowerk => "Contract_234_Clutch_of_Kings__Browerk",
            ContractId::Contract235ClutchOfKingsAll => "Contract_235_Clutch_of_Kings__All",
            ContractId::Contract236KillSpectralArchers => "Contract_236_Kill__Spectral_Archers",
            ContractId::Contract237KillSpectralMinions => "Contract_237_Kill__Spectral_Minions",
            ContractId::Contract238KillSpectralNanjouShouJen => "Contract_238_Kill__Spectral_Nanjou_Shou_jen",
            ContractId::Contract239KillSpectralMages => "Contract_239_Kill__Spectral_Mages",
            ContractId::Contract240KillSpectralBushi => "Contract_240_Kill__Spectral_Bushi",
            ContractId::Contract241KillSpectralSamurai => "Contract_241_Kill__Spectral_Samurai",
            ContractId::Contract242KillSpectralBladesAndClaws => "Contract_242_Kill__Spectral_Blades_and_Claws",
            ContractId::Contract243KillSpectralSamuraiGolems => "Contract_243_Kill__Spectral_Samurai_Golems",
            ContractId::Contract244HoshinoFortress => "Contract_244_Hoshino_Fortress",
            ContractId::Contract245StipendGeneral => "Contract_245_Stipend__General",
            ContractId::Contract246StipendCelestialHand => "Contract_246_Stipend__Celestial_Hand",
            ContractId::Contract247StipendRadiantBlood => "Contract_247_Stipend__Radiant_Blood",
            ContractId::Contract248StipendEldrytchWeb => "Contract_248_Stipend__Eldrytch_Web",
            ContractId::Contract249JesterFocuses => "Contract_249_Jester_Focuses",
            ContractId::Contract250UnleashTheGearknights => "Contract_250_Unleash_the_Gearknights",
            ContractId::Contract251VirindiRescue => "Contract_251_Virindi_Rescue",
            ContractId::Contract252NinjaAcademy => "Contract_252_Ninja_Academy",
            ContractId::Contract253TanadaSlaughter => "Contract_253_Tanada_Slaughter",
            ContractId::Contract254TanadaIntercept => "Contract_254_Tanada_Intercept",
            ContractId::Contract255CrystallineAdventurer => "Contract_255_Crystalline_Adventurer",
            ContractId::Contract256CrystallineMarkers => "Contract_256_Crystalline_Markers",
            ContractId::Contract257CrystallineKiller => "Contract_257_Crystalline_Killer",
            ContractId::Contract258CrystallineBoundWisp => "Contract_258_Crystalline_Bound_Wisp",
            ContractId::Contract259NanjouStockade => "Contract_259_Nanjou_Stockade",
            ContractId::Contract260MageAcademy => "Contract_260_Mage_Academy",
            ContractId::Contract261ApostateFinale => "Contract_261_Apostate_Finale",
            ContractId::Contract262LunnumsReturn => "Contract_262_Lunnums_Return",
            ContractId::Contract263LunnumsPyre => "Contract_263_Lunnums_Pyre",
            ContractId::Contract264LunnumsDisappearance => "Contract_264_Lunnums_Disappearance",
            ContractId::Contract265LostLore => "Contract_265_Lost_Lore",
            ContractId::Contract266SistersOfLight => "Contract_266_Sisters_of_Light",
            ContractId::Contract267FirstSister => "Contract_267_First_Sister",
            ContractId::Contract268SecondSister => "Contract_268_Second_Sister",
            ContractId::Contract269ThirdSister => "Contract_269_Third_Sister",
            ContractId::Contract270RitualInvestigation => "Contract_270_Ritual_Investigation",
            ContractId::Contract271RitualDisruption => "Contract_271_Ritual_Disruption",
            ContractId::Contract272DefeatHoshinoKei => "Contract_272_Defeat_Hoshino_Kei",
            ContractId::Contract273ProtectingPicketedPets => "Contract_273_Protecting_Picketed_Pets",
            ContractId::Contract274BuriedAlive => "Contract_274_Buried_Alive",
            ContractId::Contract275Graverobber => "Contract_275_Graverobber",
            ContractId::Contract276Escape => "Contract_276_Escape",
            ContractId::Contract277Deconstruction => "Contract_277_Deconstruction",
            ContractId::Contract278UzizAbductions => "Contract_278_Uziz_Abductions",
            ContractId::Contract279GolemHuntersMudGolemSludgeLord => "Contract_279_Golem_Hunters__Mud_Golem_Sludge_Lord",
            ContractId::Contract280GolemHuntersCopperGolemKingpin => "Contract_280_Golem_Hunters__Copper_Golem_Kingpin",
            ContractId::Contract281GolemHuntersGlacialGolemMargrave => "Contract_281_Golem_Hunters__Glacial_Golem_Margrave",
            ContractId::Contract282GolemHuntersMagmaGolemExarch => "Contract_282_Golem_Hunters__Magma_Golem_Exarch",
            ContractId::Contract283GolemHuntersCoralGolemViceroy => "Contract_283_Golem_Hunters__Coral_Golem_Viceroy",
            ContractId::Contract284GolemHuntersPlatinumGolemMountainKing => "Contract_284_Golem_Hunters__Platinum_Golem_Mountain_King",
            ContractId::Contract285OlthoiHiveQueen => "Contract_285_Olthoi_Hive_Queen",
            ContractId::Contract286SocManaSiphonDestruction => "Contract_286_Soc__Mana_Siphon_Destruction",
            ContractId::Contract287SocManaSiphonDestruction => "Contract_287_Soc__Mana_Siphon_Destruction",
            ContractId::Contract288SocDestroyThePhalanx => "Contract_288_Soc__Destroy_The_Phalanx",
            ContractId::Contract289SocDestroyThePhalanx => "Contract_289_Soc__Destroy_The_Phalanx",
            ContractId::Contract290SocDestroyThePhalanx => "Contract_290_Soc__Destroy_The_Phalanx",
            ContractId::Contract291SocCollectGearKnightParts => "Contract_291_Soc__Collect_Gear_Knight_Parts",
            ContractId::Contract292SocCollectGearKnightParts => "Contract_292_Soc__Collect_Gear_Knight_Parts",
            ContractId::Contract293SocCollectGearKnightParts => "Contract_293_Soc__Collect_Gear_Knight_Parts",
            ContractId::Contract294KillGearKnightSquires => "Contract_294_Kill__Gear_Knight_Squires",
            ContractId::Contract295BehindTheMask => "Contract_295_Behind_The_Mask",
            ContractId::Contract296FrozenFortressLaboratory => "Contract_296_Frozen_Fortress_Laboratory",
            ContractId::Contract297FrozenFortressTestingGrounds => "Contract_297_Frozen_Fortress_Testing_Grounds",
            ContractId::Contract298OlthoiHiveWarriorPincer => "Contract_298_Olthoi_Hive_Warrior_Pincer",
            ContractId::Contract299OlthoiHiveEvisceratorPincer => "Contract_299_Olthoi_Hive_Eviscerator_Pincer",
            ContractId::Contract300SnowTuskerLeaderTusk => "Contract_300_Snow_Tusker_Leader_Tusk",
            ContractId::Contract301JourneyToMadness => "Contract_301_Journey_To_Madness",
            ContractId::Contract302Visitors => "Contract_302_Visitors",
            ContractId::Contract303KillRynthidMinions => "Contract_303_Kill__Rynthid_Minions",
            ContractId::Contract304KillEmpoweredWisps => "Contract_304_Kill__Empowered_Wisps",
            ContractId::Contract305KillRynthidRareBoss => "Contract_305_Kill__Rynthid_Rare_Boss",
            ContractId::Contract306KillRynthidSlayers => "Contract_306_Kill__Rynthid_Slayers",
            ContractId::Contract307KillRynthidRagers => "Contract_307_Kill__Rynthid_Ragers",
            ContractId::Contract308KillRynthidSorcerers => "Contract_308_Kill__Rynthid_Sorcerers",
            ContractId::Contract309KillRynthidRifts => "Contract_309_Kill__Rynthid_Rifts",
            ContractId::Contract310LegendaryQuests => "Contract_310_Legendary_Quests",
            ContractId::Contract311RynthidGenesis => "Contract_311_Rynthid_Genesis",
            ContractId::Contract312ChangingGears => "Contract_312_Changing_Gears",
            ContractId::Contract313FearFactory => "Contract_313_Fear_Factory",
            ContractId::Contract314SpiritedHalls => "Contract_314_Spirited_Halls",
            ContractId::Contract315EndOfDays => "Contract_315_End_of_Days",
            ContractId::Contract316LugianAssault => "Contract_316_Lugian_Assault",
            ContractId::Contract317RynthidTraining => "Contract_317_Rynthid_Training",
            ContractId::Contract318KillTouTouShadowFlyers => "Contract_318_Kill__Tou_Tou_Shadow_Flyers",
            ContractId::Contract319KillTouTouGrievverShredders => "Contract_319_Kill__Tou_Tou_Grievver_Shredders",
            ContractId::Contract320KillTouTouDevourerMarguls => "Contract_320_Kill__Tou_Tou_Devourer_Marguls",
            ContractId::Contract321KillTouTouShadows => "Contract_321_Kill__Tou_Tou_Shadows",
            ContractId::Contract322KillTouTouVoidLords => "Contract_322_Kill__Tou_Tou_Void_Lords",
        };
        write!(f, "{}", s)
    }
}

/// The PropertyInt64 identifies a specific Character or Object int64 property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyInt64 {
    TotalExperience = 0x1,
    AvailableExperience = 0x2,
    AugmentationCost = 0x3,
    ItemTotalXp = 0x4,
    ItemBaseXp = 0x5,
    AvailableLuminance = 0x6,
    MaximumLuminance = 0x7,
    InteractionReqs = 0x8,
}

impl crate::readers::ACDataType for PropertyInt64 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyInt64::try_from(value)?)
    }
}

impl crate::writers::ACWritable for PropertyInt64 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for PropertyInt64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PropertyInt64::TotalExperience => "TotalExperience",
            PropertyInt64::AvailableExperience => "AvailableExperience",
            PropertyInt64::AugmentationCost => "AugmentationCost",
            PropertyInt64::ItemTotalXp => "ItemTotalXp",
            PropertyInt64::ItemBaseXp => "ItemBaseXp",
            PropertyInt64::AvailableLuminance => "AvailableLuminance",
            PropertyInt64::MaximumLuminance => "MaximumLuminance",
            PropertyInt64::InteractionReqs => "InteractionReqs",
        };
        write!(f, "{}", s)
    }
}

/// The PropertyBool identifies a specific Character or Object boolean property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyBool {
    Undef = 0x0,
    Stuck = 0x1,
    Open = 0x2,
    Locked = 0x3,
    RotProof = 0x4,
    AllegianceUpdateRequest = 0x5,
    AiUsesMana = 0x6,
    AiUseHumanMagicAnimations = 0x7,
    AllowGive = 0x8,
    CurrentlyAttacking = 0x9,
    AttackerAi = 0xA,
    IgnoreCollisions = 0xB,
    ReportCollisions = 0xC,
    Ethereal = 0xD,
    GravityStatus = 0xE,
    LightsStatus = 0xF,
    ScriptedCollision = 0x10,
    Inelastic = 0x11,
    Visibility = 0x12,
    Attackable = 0x13,
    SafeSpellComponents = 0x14,
    AdvocateState = 0x15,
    Inscribable = 0x16,
    DestroyOnSell = 0x17,
    UiHidden = 0x18,
    IgnoreHouseBarriers = 0x19,
    HiddenAdmin = 0x1A,
    PkWounder = 0x1B,
    PkKiller = 0x1C,
    NoCorpse = 0x1D,
    UnderLifestoneProtection = 0x1E,
    ItemManaUpdatePending = 0x1F,
    GeneratorStatus = 0x20,
    ResetMessagePending = 0x21,
    DefaultOpen = 0x22,
    DefaultLocked = 0x23,
    DefaultOn = 0x24,
    OpenForBusiness = 0x25,
    IsFrozen = 0x26,
    DealMagicalItems = 0x27,
    LogoffImDead = 0x28,
    ReportCollisionsAsEnvironment = 0x29,
    AllowEdgeSlide = 0x2A,
    AdvocateQuest = 0x2B,
    IsAdmin = 0x2C,
    IsArch = 0x2D,
    IsSentinel = 0x2E,
    IsAdvocate = 0x2F,
    CurrentlyPoweringUp = 0x30,
    GeneratorEnteredWorld = 0x31,
    NeverFailCasting = 0x32,
    VendorService = 0x33,
    AiImmobile = 0x34,
    DamagedByCollisions = 0x35,
    IsDynamic = 0x36,
    IsHot = 0x37,
    IsAffecting = 0x38,
    AffectsAis = 0x39,
    SpellQueueActive = 0x3A,
    GeneratorDisabled = 0x3B,
    IsAcceptingTells = 0x3C,
    LoggingChannel = 0x3D,
    OpensAnyLock = 0x3E,
    UnlimitedUse = 0x3F,
    GeneratedTreasureItem = 0x40,
    IgnoreMagicResist = 0x41,
    IgnoreMagicArmor = 0x42,
    AiAllowTrade = 0x43,
    SpellComponentsRequired = 0x44,
    IsSellable = 0x45,
    IgnoreShieldsBySkill = 0x46,
    NoDraw = 0x47,
    ActivationUntargeted = 0x48,
    HouseHasGottenPriorityBootPos = 0x49,
    GeneratorAutomaticDestruction = 0x4A,
    HouseHooksVisible = 0x4B,
    HouseRequiresMonarch = 0x4C,
    HouseHooksEnabled = 0x4D,
    HouseNotifiedHudOfHookCount = 0x4E,
    AiAcceptEverything = 0x4F,
    IgnorePortalRestrictions = 0x50,
    RequiresBackpackSlot = 0x51,
    DontTurnOrMoveWhenGiving = 0x52,
    NpcLooksLikeObject = 0x53,
    IgnoreCloIcons = 0x54,
    AppraisalHasAllowedWielder = 0x55,
    ChestRegenOnClose = 0x56,
    LogoffInMinigame = 0x57,
    PortalShowDestination = 0x58,
    PortalIgnoresPkAttackTimer = 0x59,
    NpcInteractsSilently = 0x5A,
    Retained = 0x5B,
    IgnoreAuthor = 0x5C,
    Limbo = 0x5D,
    AppraisalHasAllowedActivator = 0x5E,
    ExistedBeforeAllegianceXpChanges = 0x5F,
    IsDeaf = 0x60,
    IsPsr = 0x61,
    Invincible = 0x62,
    Ivoryable = 0x63,
    Dyable = 0x64,
    CanGenerateRare = 0x65,
    CorpseGeneratedRare = 0x66,
    NonProjectileMagicImmune = 0x67,
    ActdReceivedItems = 0x68,
    Unknown105 = 0x69,
    FirstEnterWorldDone = 0x6A,
    RecallsDisabled = 0x6B,
    RareUsesTimer = 0x6C,
    ActdPreorderReceivedItems = 0x6D,
    Afk = 0x6E,
    IsGagged = 0x6F,
    ProcSpellSelfTargeted = 0x70,
    IsAllegianceGagged = 0x71,
    EquipmentSetTriggerPiece = 0x72,
    Uninscribe = 0x73,
    WieldOnUse = 0x74,
    ChestClearedWhenClosed = 0x75,
    NeverAttack = 0x76,
    SuppressGenerateEffect = 0x77,
    TreasureCorpse = 0x78,
    EquipmentSetAddLevel = 0x79,
    BarberActive = 0x7A,
    TopLayerPriority = 0x7B,
    NoHeldItemShown = 0x7C,
    LoginAtLifestone = 0x7D,
    OlthoiPk = 0x7E,
    Account15Days = 0x7F,
    HadNoVitae = 0x80,
    NoOlthoiTalk = 0x81,
    AutowieldLeft = 0x82,
}

impl crate::readers::ACDataType for PropertyBool {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyBool::try_from(value)?)
    }
}

impl crate::writers::ACWritable for PropertyBool {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for PropertyBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PropertyBool::Undef => "Undef",
            PropertyBool::Stuck => "Stuck",
            PropertyBool::Open => "Open",
            PropertyBool::Locked => "Locked",
            PropertyBool::RotProof => "RotProof",
            PropertyBool::AllegianceUpdateRequest => "AllegianceUpdateRequest",
            PropertyBool::AiUsesMana => "AiUsesMana",
            PropertyBool::AiUseHumanMagicAnimations => "AiUseHumanMagicAnimations",
            PropertyBool::AllowGive => "AllowGive",
            PropertyBool::CurrentlyAttacking => "CurrentlyAttacking",
            PropertyBool::AttackerAi => "AttackerAi",
            PropertyBool::IgnoreCollisions => "IgnoreCollisions",
            PropertyBool::ReportCollisions => "ReportCollisions",
            PropertyBool::Ethereal => "Ethereal",
            PropertyBool::GravityStatus => "GravityStatus",
            PropertyBool::LightsStatus => "LightsStatus",
            PropertyBool::ScriptedCollision => "ScriptedCollision",
            PropertyBool::Inelastic => "Inelastic",
            PropertyBool::Visibility => "Visibility",
            PropertyBool::Attackable => "Attackable",
            PropertyBool::SafeSpellComponents => "SafeSpellComponents",
            PropertyBool::AdvocateState => "AdvocateState",
            PropertyBool::Inscribable => "Inscribable",
            PropertyBool::DestroyOnSell => "DestroyOnSell",
            PropertyBool::UiHidden => "UiHidden",
            PropertyBool::IgnoreHouseBarriers => "IgnoreHouseBarriers",
            PropertyBool::HiddenAdmin => "HiddenAdmin",
            PropertyBool::PkWounder => "PkWounder",
            PropertyBool::PkKiller => "PkKiller",
            PropertyBool::NoCorpse => "NoCorpse",
            PropertyBool::UnderLifestoneProtection => "UnderLifestoneProtection",
            PropertyBool::ItemManaUpdatePending => "ItemManaUpdatePending",
            PropertyBool::GeneratorStatus => "GeneratorStatus",
            PropertyBool::ResetMessagePending => "ResetMessagePending",
            PropertyBool::DefaultOpen => "DefaultOpen",
            PropertyBool::DefaultLocked => "DefaultLocked",
            PropertyBool::DefaultOn => "DefaultOn",
            PropertyBool::OpenForBusiness => "OpenForBusiness",
            PropertyBool::IsFrozen => "IsFrozen",
            PropertyBool::DealMagicalItems => "DealMagicalItems",
            PropertyBool::LogoffImDead => "LogoffImDead",
            PropertyBool::ReportCollisionsAsEnvironment => "ReportCollisionsAsEnvironment",
            PropertyBool::AllowEdgeSlide => "AllowEdgeSlide",
            PropertyBool::AdvocateQuest => "AdvocateQuest",
            PropertyBool::IsAdmin => "IsAdmin",
            PropertyBool::IsArch => "IsArch",
            PropertyBool::IsSentinel => "IsSentinel",
            PropertyBool::IsAdvocate => "IsAdvocate",
            PropertyBool::CurrentlyPoweringUp => "CurrentlyPoweringUp",
            PropertyBool::GeneratorEnteredWorld => "GeneratorEnteredWorld",
            PropertyBool::NeverFailCasting => "NeverFailCasting",
            PropertyBool::VendorService => "VendorService",
            PropertyBool::AiImmobile => "AiImmobile",
            PropertyBool::DamagedByCollisions => "DamagedByCollisions",
            PropertyBool::IsDynamic => "IsDynamic",
            PropertyBool::IsHot => "IsHot",
            PropertyBool::IsAffecting => "IsAffecting",
            PropertyBool::AffectsAis => "AffectsAis",
            PropertyBool::SpellQueueActive => "SpellQueueActive",
            PropertyBool::GeneratorDisabled => "GeneratorDisabled",
            PropertyBool::IsAcceptingTells => "IsAcceptingTells",
            PropertyBool::LoggingChannel => "LoggingChannel",
            PropertyBool::OpensAnyLock => "OpensAnyLock",
            PropertyBool::UnlimitedUse => "UnlimitedUse",
            PropertyBool::GeneratedTreasureItem => "GeneratedTreasureItem",
            PropertyBool::IgnoreMagicResist => "IgnoreMagicResist",
            PropertyBool::IgnoreMagicArmor => "IgnoreMagicArmor",
            PropertyBool::AiAllowTrade => "AiAllowTrade",
            PropertyBool::SpellComponentsRequired => "SpellComponentsRequired",
            PropertyBool::IsSellable => "IsSellable",
            PropertyBool::IgnoreShieldsBySkill => "IgnoreShieldsBySkill",
            PropertyBool::NoDraw => "NoDraw",
            PropertyBool::ActivationUntargeted => "ActivationUntargeted",
            PropertyBool::HouseHasGottenPriorityBootPos => "HouseHasGottenPriorityBootPos",
            PropertyBool::GeneratorAutomaticDestruction => "GeneratorAutomaticDestruction",
            PropertyBool::HouseHooksVisible => "HouseHooksVisible",
            PropertyBool::HouseRequiresMonarch => "HouseRequiresMonarch",
            PropertyBool::HouseHooksEnabled => "HouseHooksEnabled",
            PropertyBool::HouseNotifiedHudOfHookCount => "HouseNotifiedHudOfHookCount",
            PropertyBool::AiAcceptEverything => "AiAcceptEverything",
            PropertyBool::IgnorePortalRestrictions => "IgnorePortalRestrictions",
            PropertyBool::RequiresBackpackSlot => "RequiresBackpackSlot",
            PropertyBool::DontTurnOrMoveWhenGiving => "DontTurnOrMoveWhenGiving",
            PropertyBool::NpcLooksLikeObject => "NpcLooksLikeObject",
            PropertyBool::IgnoreCloIcons => "IgnoreCloIcons",
            PropertyBool::AppraisalHasAllowedWielder => "AppraisalHasAllowedWielder",
            PropertyBool::ChestRegenOnClose => "ChestRegenOnClose",
            PropertyBool::LogoffInMinigame => "LogoffInMinigame",
            PropertyBool::PortalShowDestination => "PortalShowDestination",
            PropertyBool::PortalIgnoresPkAttackTimer => "PortalIgnoresPkAttackTimer",
            PropertyBool::NpcInteractsSilently => "NpcInteractsSilently",
            PropertyBool::Retained => "Retained",
            PropertyBool::IgnoreAuthor => "IgnoreAuthor",
            PropertyBool::Limbo => "Limbo",
            PropertyBool::AppraisalHasAllowedActivator => "AppraisalHasAllowedActivator",
            PropertyBool::ExistedBeforeAllegianceXpChanges => "ExistedBeforeAllegianceXpChanges",
            PropertyBool::IsDeaf => "IsDeaf",
            PropertyBool::IsPsr => "IsPsr",
            PropertyBool::Invincible => "Invincible",
            PropertyBool::Ivoryable => "Ivoryable",
            PropertyBool::Dyable => "Dyable",
            PropertyBool::CanGenerateRare => "CanGenerateRare",
            PropertyBool::CorpseGeneratedRare => "CorpseGeneratedRare",
            PropertyBool::NonProjectileMagicImmune => "NonProjectileMagicImmune",
            PropertyBool::ActdReceivedItems => "ActdReceivedItems",
            PropertyBool::Unknown105 => "Unknown105",
            PropertyBool::FirstEnterWorldDone => "FirstEnterWorldDone",
            PropertyBool::RecallsDisabled => "RecallsDisabled",
            PropertyBool::RareUsesTimer => "RareUsesTimer",
            PropertyBool::ActdPreorderReceivedItems => "ActdPreorderReceivedItems",
            PropertyBool::Afk => "Afk",
            PropertyBool::IsGagged => "IsGagged",
            PropertyBool::ProcSpellSelfTargeted => "ProcSpellSelfTargeted",
            PropertyBool::IsAllegianceGagged => "IsAllegianceGagged",
            PropertyBool::EquipmentSetTriggerPiece => "EquipmentSetTriggerPiece",
            PropertyBool::Uninscribe => "Uninscribe",
            PropertyBool::WieldOnUse => "WieldOnUse",
            PropertyBool::ChestClearedWhenClosed => "ChestClearedWhenClosed",
            PropertyBool::NeverAttack => "NeverAttack",
            PropertyBool::SuppressGenerateEffect => "SuppressGenerateEffect",
            PropertyBool::TreasureCorpse => "TreasureCorpse",
            PropertyBool::EquipmentSetAddLevel => "EquipmentSetAddLevel",
            PropertyBool::BarberActive => "BarberActive",
            PropertyBool::TopLayerPriority => "TopLayerPriority",
            PropertyBool::NoHeldItemShown => "NoHeldItemShown",
            PropertyBool::LoginAtLifestone => "LoginAtLifestone",
            PropertyBool::OlthoiPk => "OlthoiPk",
            PropertyBool::Account15Days => "Account15Days",
            PropertyBool::HadNoVitae => "HadNoVitae",
            PropertyBool::NoOlthoiTalk => "NoOlthoiTalk",
            PropertyBool::AutowieldLeft => "AutowieldLeft",
        };
        write!(f, "{}", s)
    }
}

/// The DataPropertyId identifies a specific Character or Object data property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyDataId {
    Setup = 0x1,
    MotionTable = 0x2,
    SoundTable = 0x3,
    CombatTable = 0x4,
    QualityFilter = 0x5,
    PaletteBase = 0x6,
    ClothingBase = 0x7,
    Icon = 0x8,
    EyesTexture = 0x9,
    NoseTexture = 0xA,
    MouthTexture = 0xB,
    DefaultEyesTexture = 0xC,
    DefaultNoseTexture = 0xD,
    DefaultMouthTexture = 0xE,
    HairPalette = 0xF,
    EyesPalette = 0x10,
    SkinPalette = 0x11,
    HeadObject = 0x12,
    ActivationAnimation = 0x13,
    InitMotion = 0x14,
    ActivationSound = 0x15,
    PhysicsEffectTable = 0x16,
    UseSound = 0x17,
    UseTargetAnimation = 0x18,
    UseTargetSuccessAnimation = 0x19,
    UseTargetFailureAnimation = 0x1A,
    UseUserAnimation = 0x1B,
    Spell = 0x1C,
    SpellComponent = 0x1D,
    PhysicsScript = 0x1E,
    LinkedPortalOne = 0x1F,
    WieldedTreasureType = 0x20,
    InventoryTreasureType = 0x21,
    ShopTreasureType = 0x22,
    DeathTreasureType = 0x23,
    MutateFilter = 0x24,
    ItemSkillLimit = 0x25,
    UseCreateItem = 0x26,
    DeathSpell = 0x27,
    VendorsClassId = 0x28,
    ItemSpecializedOnly = 0x29,
    HouseId = 0x2A,
    AccountHouseId = 0x2B,
    RestrictionEffect = 0x2C,
    CreationMutationFilter = 0x2D,
    TsysMutationFilter = 0x2E,
    LastPortal = 0x2F,
    LinkedPortalTwo = 0x30,
    OriginalPortal = 0x31,
    IconOverlay = 0x32,
    IconOverlaySecondary = 0x33,
    IconUnderlay = 0x34,
    AugmentationMutationFilter = 0x35,
    AugmentationEffect = 0x36,
    ProcSpell = 0x37,
    AugmentationCreateItem = 0x38,
    AlternateCurrency = 0x39,
    BlueSurgeSpell = 0x3A,
    YellowSurgeSpell = 0x3B,
    RedSurgeSpell = 0x3C,
    OlthoiDeathTreasureType = 0x3D,
}

impl crate::readers::ACDataType for PropertyDataId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyDataId::try_from(value)?)
    }
}

impl crate::writers::ACWritable for PropertyDataId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for PropertyDataId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PropertyDataId::Setup => "Setup",
            PropertyDataId::MotionTable => "MotionTable",
            PropertyDataId::SoundTable => "SoundTable",
            PropertyDataId::CombatTable => "CombatTable",
            PropertyDataId::QualityFilter => "QualityFilter",
            PropertyDataId::PaletteBase => "PaletteBase",
            PropertyDataId::ClothingBase => "ClothingBase",
            PropertyDataId::Icon => "Icon",
            PropertyDataId::EyesTexture => "EyesTexture",
            PropertyDataId::NoseTexture => "NoseTexture",
            PropertyDataId::MouthTexture => "MouthTexture",
            PropertyDataId::DefaultEyesTexture => "DefaultEyesTexture",
            PropertyDataId::DefaultNoseTexture => "DefaultNoseTexture",
            PropertyDataId::DefaultMouthTexture => "DefaultMouthTexture",
            PropertyDataId::HairPalette => "HairPalette",
            PropertyDataId::EyesPalette => "EyesPalette",
            PropertyDataId::SkinPalette => "SkinPalette",
            PropertyDataId::HeadObject => "HeadObject",
            PropertyDataId::ActivationAnimation => "ActivationAnimation",
            PropertyDataId::InitMotion => "InitMotion",
            PropertyDataId::ActivationSound => "ActivationSound",
            PropertyDataId::PhysicsEffectTable => "PhysicsEffectTable",
            PropertyDataId::UseSound => "UseSound",
            PropertyDataId::UseTargetAnimation => "UseTargetAnimation",
            PropertyDataId::UseTargetSuccessAnimation => "UseTargetSuccessAnimation",
            PropertyDataId::UseTargetFailureAnimation => "UseTargetFailureAnimation",
            PropertyDataId::UseUserAnimation => "UseUserAnimation",
            PropertyDataId::Spell => "Spell",
            PropertyDataId::SpellComponent => "SpellComponent",
            PropertyDataId::PhysicsScript => "PhysicsScript",
            PropertyDataId::LinkedPortalOne => "LinkedPortalOne",
            PropertyDataId::WieldedTreasureType => "WieldedTreasureType",
            PropertyDataId::InventoryTreasureType => "InventoryTreasureType",
            PropertyDataId::ShopTreasureType => "ShopTreasureType",
            PropertyDataId::DeathTreasureType => "DeathTreasureType",
            PropertyDataId::MutateFilter => "MutateFilter",
            PropertyDataId::ItemSkillLimit => "ItemSkillLimit",
            PropertyDataId::UseCreateItem => "UseCreateItem",
            PropertyDataId::DeathSpell => "DeathSpell",
            PropertyDataId::VendorsClassId => "VendorsClassId",
            PropertyDataId::ItemSpecializedOnly => "ItemSpecializedOnly",
            PropertyDataId::HouseId => "HouseId",
            PropertyDataId::AccountHouseId => "AccountHouseId",
            PropertyDataId::RestrictionEffect => "RestrictionEffect",
            PropertyDataId::CreationMutationFilter => "CreationMutationFilter",
            PropertyDataId::TsysMutationFilter => "TsysMutationFilter",
            PropertyDataId::LastPortal => "LastPortal",
            PropertyDataId::LinkedPortalTwo => "LinkedPortalTwo",
            PropertyDataId::OriginalPortal => "OriginalPortal",
            PropertyDataId::IconOverlay => "IconOverlay",
            PropertyDataId::IconOverlaySecondary => "IconOverlaySecondary",
            PropertyDataId::IconUnderlay => "IconUnderlay",
            PropertyDataId::AugmentationMutationFilter => "AugmentationMutationFilter",
            PropertyDataId::AugmentationEffect => "AugmentationEffect",
            PropertyDataId::ProcSpell => "ProcSpell",
            PropertyDataId::AugmentationCreateItem => "AugmentationCreateItem",
            PropertyDataId::AlternateCurrency => "AlternateCurrency",
            PropertyDataId::BlueSurgeSpell => "BlueSurgeSpell",
            PropertyDataId::YellowSurgeSpell => "YellowSurgeSpell",
            PropertyDataId::RedSurgeSpell => "RedSurgeSpell",
            PropertyDataId::OlthoiDeathTreasureType => "OlthoiDeathTreasureType",
        };
        write!(f, "{}", s)
    }
}

/// The PropertyInt identifies a specific Character or Object int property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyInt {
    ItemType = 0x1,
    CreatureType = 0x2,
    PaletteTemplate = 0x3,
    ClothingPriority = 0x4,
    EncumbranceVal = 0x5,
    ItemsCapacity = 0x6,
    ContainersCapacity = 0x7,
    Mass = 0x8,
    ValidLocations = 0x9,
    CurrentWieldedLocation = 0xA,
    MaxStackSize = 0xB,
    StackSize = 0xC,
    StackUnitEncumbrance = 0xD,
    StackUnitMass = 0xE,
    StackUnitValue = 0xF,
    ItemUseable = 0x10,
    RareId = 0x11,
    UiEffects = 0x12,
    Value = 0x13,
    CoinValue = 0x14,
    TotalExperience = 0x15,
    AvailableCharacter = 0x16,
    TotalSkillCredits = 0x17,
    AvailableSkillCredits = 0x18,
    Level = 0x19,
    AccountRequirements = 0x1A,
    ArmorType = 0x1B,
    ArmorLevel = 0x1C,
    AllegianceCpPool = 0x1D,
    AllegianceRank = 0x1E,
    ChannelsAllowed = 0x1F,
    ChannelsActive = 0x20,
    Bonded = 0x21,
    MonarchsRank = 0x22,
    AllegianceFollowers = 0x23,
    ResistMagic = 0x24,
    ResistItemAppraisal = 0x25,
    ResistLockpick = 0x26,
    DeprecatedResistRepair = 0x27,
    CombatMode = 0x28,
    CurrentAttackHeight = 0x29,
    CombatCollisions = 0x2A,
    NumDeaths = 0x2B,
    Damage = 0x2C,
    DamageType = 0x2D,
    DefaultCombatStyle = 0x2E,
    AttackType = 0x2F,
    WeaponSkill = 0x30,
    WeaponTime = 0x31,
    AmmoType = 0x32,
    CombatUse = 0x33,
    ParentLocation = 0x34,
    PlacementPosition = 0x35,
    WeaponEncumbrance = 0x36,
    WeaponMass = 0x37,
    ShieldValue = 0x38,
    ShieldEncumbrance = 0x39,
    MissileInventoryLocation = 0x3A,
    FullDamageType = 0x3B,
    WeaponRange = 0x3C,
    AttackersSkill = 0x3D,
    DefendersSkill = 0x3E,
    AttackersSkillValue = 0x3F,
    AttackersClass = 0x40,
    Placement = 0x41,
    CheckpointStatus = 0x42,
    Tolerance = 0x43,
    TargetingTactic = 0x44,
    CombatTactic = 0x45,
    HomesickTargetingTactic = 0x46,
    NumFollowFailures = 0x47,
    FriendType = 0x48,
    FoeType = 0x49,
    MerchandiseItemTypes = 0x4A,
    MerchandiseMinValue = 0x4B,
    MerchandiseMaxValue = 0x4C,
    NumItemsSold = 0x4D,
    NumItemsBought = 0x4E,
    MoneyIncome = 0x4F,
    MoneyOutflow = 0x50,
    MaxGeneratedObjects = 0x51,
    InitGeneratedObjects = 0x52,
    ActivationResponse = 0x53,
    OriginalValue = 0x54,
    NumMoveFailures = 0x55,
    MinLevel = 0x56,
    MaxLevel = 0x57,
    LockpickMod = 0x58,
    BoosterEnum = 0x59,
    BoostValue = 0x5A,
    MaxStructure = 0x5B,
    Structure = 0x5C,
    PhysicsState = 0x5D,
    TargetType = 0x5E,
    RadarBlipColor = 0x5F,
    EncumbranceCapacity = 0x60,
    LoginTimestamp = 0x61,
    CreationTimestamp = 0x62,
    PkLevelModifier = 0x63,
    GeneratorType = 0x64,
    AiAllowedCombatStyle = 0x65,
    LogoffTimestamp = 0x66,
    GeneratorDestructionType = 0x67,
    ActivationCreateClass = 0x68,
    ItemWorkmanship = 0x69,
    ItemSpellcraft = 0x6A,
    ItemCurMana = 0x6B,
    ItemMaxMana = 0x6C,
    ItemDifficulty = 0x6D,
    ItemAllegianceRankLimit = 0x6E,
    PortalBitmask = 0x6F,
    AdvocateLevel = 0x70,
    Gender = 0x71,
    Attuned = 0x72,
    ItemSkillLevelLimit = 0x73,
    GateLogic = 0x74,
    ItemManaCost = 0x75,
    Logoff = 0x76,
    Active = 0x77,
    AttackHeight = 0x78,
    NumAttackFailures = 0x79,
    AiCpThreshold = 0x7A,
    AiAdvancementStrategy = 0x7B,
    Version = 0x7C,
    Age = 0x7D,
    VendorHappyMean = 0x7E,
    VendorHappyVariance = 0x7F,
    CloakStatus = 0x80,
    VitaeCpPool = 0x81,
    NumServicesSold = 0x82,
    MaterialType = 0x83,
    NumAllegianceBreaks = 0x84,
    ShowableOnRadar = 0x85,
    PlayerKillerStatus = 0x86,
    VendorHappyMaxItems = 0x87,
    ScorePageNum = 0x88,
    ScoreConfigNum = 0x89,
    ScoreNumScores = 0x8A,
    DeathLevel = 0x8B,
    AiOptions = 0x8C,
    OpenToEveryone = 0x8D,
    GeneratorTimeType = 0x8E,
    GeneratorStartTime = 0x8F,
    GeneratorEndTime = 0x90,
    GeneratorEndDestructionType = 0x91,
    XpOverride = 0x92,
    NumCrashAndTurns = 0x93,
    ComponentWarningThreshold = 0x94,
    HouseStatus = 0x95,
    HookPlacement = 0x96,
    HookType = 0x97,
    HookItemType = 0x98,
    AiPpThreshold = 0x99,
    GeneratorVersion = 0x9A,
    HouseType = 0x9B,
    PickupEmoteOffset = 0x9C,
    WeenieIteration = 0x9D,
    WieldRequirements = 0x9E,
    WieldSkillType = 0x9F,
    WieldDifficulty = 0xA0,
    HouseMaxHooksUsable = 0xA1,
    HouseCurrentHooksUsable = 0xA2,
    AllegianceMinLevel = 0xA3,
    AllegianceMaxLevel = 0xA4,
    HouseRelinkHookCount = 0xA5,
    SlayerCreatureType = 0xA6,
    ConfirmationInProgress = 0xA7,
    ConfirmationTypeInProgress = 0xA8,
    TsysMutationData = 0xA9,
    NumItemsInMaterial = 0xAA,
    NumTimesTinkered = 0xAB,
    AppraisalLongDescDecoration = 0xAC,
    AppraisalLockpickSuccessPercent = 0xAD,
    AppraisalPages = 0xAE,
    AppraisalMaxPages = 0xAF,
    AppraisalItemSkill = 0xB0,
    GemCount = 0xB1,
    GemType = 0xB2,
    ImbuedEffect = 0xB3,
    AttackersRawSkillValue = 0xB4,
    ChessRank = 0xB5,
    ChessTotalGames = 0xB6,
    ChessGamesWon = 0xB7,
    ChessGamesLost = 0xB8,
    TypeOfAlteration = 0xB9,
    SkillToBeAltered = 0xBA,
    SkillAlterationCount = 0xBB,
    HeritageGroup = 0xBC,
    TransferFromAttribute = 0xBD,
    TransferToAttribute = 0xBE,
    AttributeTransferCount = 0xBF,
    FakeFishingSkill = 0xC0,
    NumKeys = 0xC1,
    DeathTimestamp = 0xC2,
    PkTimestamp = 0xC3,
    VictimTimestamp = 0xC4,
    HookGroup = 0xC5,
    AllegianceSwearTimestamp = 0xC6,
    HousePurchaseTimestamp = 0xC7,
    RedirectableEquippedArmorCount = 0xC8,
    MeleeDefenseImbuedEffectTypeCache = 0xC9,
    MissileDefenseImbuedEffectTypeCache = 0xCA,
    MagicDefenseImbuedEffectTypeCache = 0xCB,
    ElementalDamageBonus = 0xCC,
    ImbueAttempts = 0xCD,
    ImbueSuccesses = 0xCE,
    CreatureKills = 0xCF,
    PlayerKillsPk = 0xD0,
    PlayerKillsPkl = 0xD1,
    RaresTierOne = 0xD2,
    RaresTierTwo = 0xD3,
    RaresTierThree = 0xD4,
    RaresTierFour = 0xD5,
    RaresTierFive = 0xD6,
    AugmentationStat = 0xD7,
    AugmentationFamilyStat = 0xD8,
    AugmentationInnateFamily = 0xD9,
    AugmentationInnateStrength = 0xDA,
    AugmentationInnateEndurance = 0xDB,
    AugmentationInnateCoordination = 0xDC,
    AugmentationInnateQuickness = 0xDD,
    AugmentationInnateFocus = 0xDE,
    AugmentationInnateSelf = 0xDF,
    AugmentationSpecializeSalvaging = 0xE0,
    AugmentationSpecializeItemTinkering = 0xE1,
    AugmentationSpecializeArmorTinkering = 0xE2,
    AugmentationSpecializeMagicItemTinkering = 0xE3,
    AugmentationSpecializeWeaponTinkering = 0xE4,
    AugmentationExtraPackSlot = 0xE5,
    AugmentationIncreasedCarryingCapacity = 0xE6,
    AugmentationLessDeathItemLoss = 0xE7,
    AugmentationSpellsRemainPastDeath = 0xE8,
    AugmentationCriticalDefense = 0xE9,
    AugmentationBonusXp = 0xEA,
    AugmentationBonusSalvage = 0xEB,
    AugmentationBonusImbueChance = 0xEC,
    AugmentationFasterRegen = 0xED,
    AugmentationIncreasedSpellDuration = 0xEE,
    AugmentationResistanceFamily = 0xEF,
    AugmentationResistanceSlash = 0xF0,
    AugmentationResistancePierce = 0xF1,
    AugmentationResistanceBlunt = 0xF2,
    AugmentationResistanceAcid = 0xF3,
    AugmentationResistanceFire = 0xF4,
    AugmentationResistanceFrost = 0xF5,
    AugmentationResistanceLightning = 0xF6,
    RaresTierOneLogin = 0xF7,
    RaresTierTwoLogin = 0xF8,
    RaresTierThreeLogin = 0xF9,
    RaresTierFourLogin = 0xFA,
    RaresTierFiveLogin = 0xFB,
    RaresLoginTimestamp = 0xFC,
    RaresTierSix = 0xFD,
    RaresTierSeven = 0xFE,
    RaresTierSixLogin = 0xFF,
    RaresTierSevenLogin = 0x100,
    ItemAttributeLimit = 0x101,
    ItemAttributeLevelLimit = 0x102,
    ItemAttribute2ndLimit = 0x103,
    ItemAttribute2ndLevelLimit = 0x104,
    CharacterTitleId = 0x105,
    NumCharacterTitles = 0x106,
    ResistanceModifierType = 0x107,
    FreeTinkersBitfield = 0x108,
    EquipmentSetId = 0x109,
    PetClass = 0x10A,
    Lifespan = 0x10B,
    RemainingLifespan = 0x10C,
    UseCreateQuantity = 0x10D,
    WieldRequirements2 = 0x10E,
    WieldSkillType2 = 0x10F,
    WieldDifficulty2 = 0x110,
    WieldRequirements3 = 0x111,
    WieldSkillType3 = 0x112,
    WieldDifficulty3 = 0x113,
    WieldRequirements4 = 0x114,
    WieldSkillType4 = 0x115,
    WieldDifficulty4 = 0x116,
    Unique = 0x117,
    SharedCooldown = 0x118,
    Faction1Bits = 0x119,
    Faction2Bits = 0x11A,
    Faction3Bits = 0x11B,
    Hatred1Bits = 0x11C,
    Hatred2Bits = 0x11D,
    Hatred3Bits = 0x11E,
    SocietyRankCelhan = 0x11F,
    SocietyRankEldweb = 0x120,
    SocietyRankRadblo = 0x121,
    HearLocalSignals = 0x122,
    HearLocalSignalsRadius = 0x123,
    Cleaving = 0x124,
    AugmentationSpecializeGearcraft = 0x125,
    AugmentationInfusedCreatureMagic = 0x126,
    AugmentationInfusedItemMagic = 0x127,
    AugmentationInfusedLifeMagic = 0x128,
    AugmentationInfusedWarMagic = 0x129,
    AugmentationCriticalExpertise = 0x12A,
    AugmentationCriticalPower = 0x12B,
    AugmentationSkilledMelee = 0x12C,
    AugmentationSkilledMissile = 0x12D,
    AugmentationSkilledMagic = 0x12E,
    ImbuedEffect2 = 0x12F,
    ImbuedEffect3 = 0x130,
    ImbuedEffect4 = 0x131,
    ImbuedEffect5 = 0x132,
    DamageRating = 0x133,
    DamageResistRating = 0x134,
    AugmentationDamageBonus = 0x135,
    AugmentationDamageReduction = 0x136,
    ImbueStackingBits = 0x137,
    HealOverTime = 0x138,
    CritRating = 0x139,
    CritDamageRating = 0x13A,
    CritResistRating = 0x13B,
    CritDamageResistRating = 0x13C,
    HealingResistRating = 0x13D,
    DamageOverTime = 0x13E,
    ItemMaxLevel = 0x13F,
    ItemXpStyle = 0x140,
    EquipmentSetExtra = 0x141,
    AetheriaBitfield = 0x142,
    HealingBoostRating = 0x143,
    HeritageSpecificArmor = 0x144,
    AlternateRacialSkills = 0x145,
    AugmentationJackOfAllTrades = 0x146,
    AugmentationResistanceNether = 0x147,
    AugmentationInfusedVoidMagic = 0x148,
    WeaknessRating = 0x149,
    NetherOverTime = 0x14A,
    NetherResistRating = 0x14B,
    LuminanceAward = 0x14C,
    LumAugDamageRating = 0x14D,
    LumAugDamageReductionRating = 0x14E,
    LumAugCritDamageRating = 0x14F,
    LumAugCritReductionRating = 0x150,
    LumAugSurgeEffectRating = 0x151,
    LumAugSurgeChanceRating = 0x152,
    LumAugItemManaUsage = 0x153,
    LumAugItemManaGain = 0x154,
    LumAugVitality = 0x155,
    LumAugHealingRating = 0x156,
    LumAugSkilledCraft = 0x157,
    LumAugSkilledSpec = 0x158,
    LumAugNoDestroyCraft = 0x159,
    RestrictInteraction = 0x15A,
    OlthoiLootTimestamp = 0x15B,
    OlthoiLootStep = 0x15C,
    UseCreatesContractId = 0x15D,
    DotResistRating = 0x15E,
    LifeResistRating = 0x15F,
    CloakWeaveProc = 0x160,
    WeaponType = 0x161,
    MeleeMastery = 0x162,
    RangedMastery = 0x163,
    SneakAttackRating = 0x164,
    RecklessnessRating = 0x165,
    DeceptionRating = 0x166,
    CombatPetRange = 0x167,
    WeaponAuraDamage = 0x168,
    WeaponAuraSpeed = 0x169,
    SummoningMastery = 0x16A,
    HeartbeatLifespan = 0x16B,
    UseLevelRequirement = 0x16C,
    LumAugAllSkills = 0x16D,
    UseRequiresSkill = 0x16E,
    UseRequiresSkillLevel = 0x16F,
    UseRequiresSkillSpec = 0x170,
    UseRequiresLevel = 0x171,
    GearDamage = 0x172,
    GearDamageResist = 0x173,
    GearCrit = 0x174,
    GearCritResist = 0x175,
    GearCritDamage = 0x176,
    GearCritDamageResist = 0x177,
    GearHealingBoost = 0x178,
    GearNetherResist = 0x179,
    GearLifeResist = 0x17A,
    GearMaxHealth = 0x17B,
    Unknown380 = 0x17C,
    PKDamageRating = 0x17D,
    PKDamageResistRating = 0x17E,
    GearPKDamageRating = 0x17F,
    GearPKDamageResistRating = 0x180,
    Unknown385 = 0x181,
    Overpower = 0x182,
    OverpowerResist = 0x183,
    GearOverpower = 0x184,
    GearOverpowerResist = 0x185,
    Enlightenment = 0x186,
}

impl crate::readers::ACDataType for PropertyInt {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyInt::try_from(value)?)
    }
}

impl crate::writers::ACWritable for PropertyInt {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for PropertyInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PropertyInt::ItemType => "ItemType",
            PropertyInt::CreatureType => "CreatureType",
            PropertyInt::PaletteTemplate => "PaletteTemplate",
            PropertyInt::ClothingPriority => "ClothingPriority",
            PropertyInt::EncumbranceVal => "EncumbranceVal",
            PropertyInt::ItemsCapacity => "ItemsCapacity",
            PropertyInt::ContainersCapacity => "ContainersCapacity",
            PropertyInt::Mass => "Mass",
            PropertyInt::ValidLocations => "ValidLocations",
            PropertyInt::CurrentWieldedLocation => "CurrentWieldedLocation",
            PropertyInt::MaxStackSize => "MaxStackSize",
            PropertyInt::StackSize => "StackSize",
            PropertyInt::StackUnitEncumbrance => "StackUnitEncumbrance",
            PropertyInt::StackUnitMass => "StackUnitMass",
            PropertyInt::StackUnitValue => "StackUnitValue",
            PropertyInt::ItemUseable => "ItemUseable",
            PropertyInt::RareId => "RareId",
            PropertyInt::UiEffects => "UiEffects",
            PropertyInt::Value => "Value",
            PropertyInt::CoinValue => "CoinValue",
            PropertyInt::TotalExperience => "TotalExperience",
            PropertyInt::AvailableCharacter => "AvailableCharacter",
            PropertyInt::TotalSkillCredits => "TotalSkillCredits",
            PropertyInt::AvailableSkillCredits => "AvailableSkillCredits",
            PropertyInt::Level => "Level",
            PropertyInt::AccountRequirements => "AccountRequirements",
            PropertyInt::ArmorType => "ArmorType",
            PropertyInt::ArmorLevel => "ArmorLevel",
            PropertyInt::AllegianceCpPool => "AllegianceCpPool",
            PropertyInt::AllegianceRank => "AllegianceRank",
            PropertyInt::ChannelsAllowed => "ChannelsAllowed",
            PropertyInt::ChannelsActive => "ChannelsActive",
            PropertyInt::Bonded => "Bonded",
            PropertyInt::MonarchsRank => "MonarchsRank",
            PropertyInt::AllegianceFollowers => "AllegianceFollowers",
            PropertyInt::ResistMagic => "ResistMagic",
            PropertyInt::ResistItemAppraisal => "ResistItemAppraisal",
            PropertyInt::ResistLockpick => "ResistLockpick",
            PropertyInt::DeprecatedResistRepair => "DeprecatedResistRepair",
            PropertyInt::CombatMode => "CombatMode",
            PropertyInt::CurrentAttackHeight => "CurrentAttackHeight",
            PropertyInt::CombatCollisions => "CombatCollisions",
            PropertyInt::NumDeaths => "NumDeaths",
            PropertyInt::Damage => "Damage",
            PropertyInt::DamageType => "DamageType",
            PropertyInt::DefaultCombatStyle => "DefaultCombatStyle",
            PropertyInt::AttackType => "AttackType",
            PropertyInt::WeaponSkill => "WeaponSkill",
            PropertyInt::WeaponTime => "WeaponTime",
            PropertyInt::AmmoType => "AmmoType",
            PropertyInt::CombatUse => "CombatUse",
            PropertyInt::ParentLocation => "ParentLocation",
            PropertyInt::PlacementPosition => "PlacementPosition",
            PropertyInt::WeaponEncumbrance => "WeaponEncumbrance",
            PropertyInt::WeaponMass => "WeaponMass",
            PropertyInt::ShieldValue => "ShieldValue",
            PropertyInt::ShieldEncumbrance => "ShieldEncumbrance",
            PropertyInt::MissileInventoryLocation => "MissileInventoryLocation",
            PropertyInt::FullDamageType => "FullDamageType",
            PropertyInt::WeaponRange => "WeaponRange",
            PropertyInt::AttackersSkill => "AttackersSkill",
            PropertyInt::DefendersSkill => "DefendersSkill",
            PropertyInt::AttackersSkillValue => "AttackersSkillValue",
            PropertyInt::AttackersClass => "AttackersClass",
            PropertyInt::Placement => "Placement",
            PropertyInt::CheckpointStatus => "CheckpointStatus",
            PropertyInt::Tolerance => "Tolerance",
            PropertyInt::TargetingTactic => "TargetingTactic",
            PropertyInt::CombatTactic => "CombatTactic",
            PropertyInt::HomesickTargetingTactic => "HomesickTargetingTactic",
            PropertyInt::NumFollowFailures => "NumFollowFailures",
            PropertyInt::FriendType => "FriendType",
            PropertyInt::FoeType => "FoeType",
            PropertyInt::MerchandiseItemTypes => "MerchandiseItemTypes",
            PropertyInt::MerchandiseMinValue => "MerchandiseMinValue",
            PropertyInt::MerchandiseMaxValue => "MerchandiseMaxValue",
            PropertyInt::NumItemsSold => "NumItemsSold",
            PropertyInt::NumItemsBought => "NumItemsBought",
            PropertyInt::MoneyIncome => "MoneyIncome",
            PropertyInt::MoneyOutflow => "MoneyOutflow",
            PropertyInt::MaxGeneratedObjects => "MaxGeneratedObjects",
            PropertyInt::InitGeneratedObjects => "InitGeneratedObjects",
            PropertyInt::ActivationResponse => "ActivationResponse",
            PropertyInt::OriginalValue => "OriginalValue",
            PropertyInt::NumMoveFailures => "NumMoveFailures",
            PropertyInt::MinLevel => "MinLevel",
            PropertyInt::MaxLevel => "MaxLevel",
            PropertyInt::LockpickMod => "LockpickMod",
            PropertyInt::BoosterEnum => "BoosterEnum",
            PropertyInt::BoostValue => "BoostValue",
            PropertyInt::MaxStructure => "MaxStructure",
            PropertyInt::Structure => "Structure",
            PropertyInt::PhysicsState => "PhysicsState",
            PropertyInt::TargetType => "TargetType",
            PropertyInt::RadarBlipColor => "RadarBlipColor",
            PropertyInt::EncumbranceCapacity => "EncumbranceCapacity",
            PropertyInt::LoginTimestamp => "LoginTimestamp",
            PropertyInt::CreationTimestamp => "CreationTimestamp",
            PropertyInt::PkLevelModifier => "PkLevelModifier",
            PropertyInt::GeneratorType => "GeneratorType",
            PropertyInt::AiAllowedCombatStyle => "AiAllowedCombatStyle",
            PropertyInt::LogoffTimestamp => "LogoffTimestamp",
            PropertyInt::GeneratorDestructionType => "GeneratorDestructionType",
            PropertyInt::ActivationCreateClass => "ActivationCreateClass",
            PropertyInt::ItemWorkmanship => "ItemWorkmanship",
            PropertyInt::ItemSpellcraft => "ItemSpellcraft",
            PropertyInt::ItemCurMana => "ItemCurMana",
            PropertyInt::ItemMaxMana => "ItemMaxMana",
            PropertyInt::ItemDifficulty => "ItemDifficulty",
            PropertyInt::ItemAllegianceRankLimit => "ItemAllegianceRankLimit",
            PropertyInt::PortalBitmask => "PortalBitmask",
            PropertyInt::AdvocateLevel => "AdvocateLevel",
            PropertyInt::Gender => "Gender",
            PropertyInt::Attuned => "Attuned",
            PropertyInt::ItemSkillLevelLimit => "ItemSkillLevelLimit",
            PropertyInt::GateLogic => "GateLogic",
            PropertyInt::ItemManaCost => "ItemManaCost",
            PropertyInt::Logoff => "Logoff",
            PropertyInt::Active => "Active",
            PropertyInt::AttackHeight => "AttackHeight",
            PropertyInt::NumAttackFailures => "NumAttackFailures",
            PropertyInt::AiCpThreshold => "AiCpThreshold",
            PropertyInt::AiAdvancementStrategy => "AiAdvancementStrategy",
            PropertyInt::Version => "Version",
            PropertyInt::Age => "Age",
            PropertyInt::VendorHappyMean => "VendorHappyMean",
            PropertyInt::VendorHappyVariance => "VendorHappyVariance",
            PropertyInt::CloakStatus => "CloakStatus",
            PropertyInt::VitaeCpPool => "VitaeCpPool",
            PropertyInt::NumServicesSold => "NumServicesSold",
            PropertyInt::MaterialType => "MaterialType",
            PropertyInt::NumAllegianceBreaks => "NumAllegianceBreaks",
            PropertyInt::ShowableOnRadar => "ShowableOnRadar",
            PropertyInt::PlayerKillerStatus => "PlayerKillerStatus",
            PropertyInt::VendorHappyMaxItems => "VendorHappyMaxItems",
            PropertyInt::ScorePageNum => "ScorePageNum",
            PropertyInt::ScoreConfigNum => "ScoreConfigNum",
            PropertyInt::ScoreNumScores => "ScoreNumScores",
            PropertyInt::DeathLevel => "DeathLevel",
            PropertyInt::AiOptions => "AiOptions",
            PropertyInt::OpenToEveryone => "OpenToEveryone",
            PropertyInt::GeneratorTimeType => "GeneratorTimeType",
            PropertyInt::GeneratorStartTime => "GeneratorStartTime",
            PropertyInt::GeneratorEndTime => "GeneratorEndTime",
            PropertyInt::GeneratorEndDestructionType => "GeneratorEndDestructionType",
            PropertyInt::XpOverride => "XpOverride",
            PropertyInt::NumCrashAndTurns => "NumCrashAndTurns",
            PropertyInt::ComponentWarningThreshold => "ComponentWarningThreshold",
            PropertyInt::HouseStatus => "HouseStatus",
            PropertyInt::HookPlacement => "HookPlacement",
            PropertyInt::HookType => "HookType",
            PropertyInt::HookItemType => "HookItemType",
            PropertyInt::AiPpThreshold => "AiPpThreshold",
            PropertyInt::GeneratorVersion => "GeneratorVersion",
            PropertyInt::HouseType => "HouseType",
            PropertyInt::PickupEmoteOffset => "PickupEmoteOffset",
            PropertyInt::WeenieIteration => "WeenieIteration",
            PropertyInt::WieldRequirements => "WieldRequirements",
            PropertyInt::WieldSkillType => "WieldSkillType",
            PropertyInt::WieldDifficulty => "WieldDifficulty",
            PropertyInt::HouseMaxHooksUsable => "HouseMaxHooksUsable",
            PropertyInt::HouseCurrentHooksUsable => "HouseCurrentHooksUsable",
            PropertyInt::AllegianceMinLevel => "AllegianceMinLevel",
            PropertyInt::AllegianceMaxLevel => "AllegianceMaxLevel",
            PropertyInt::HouseRelinkHookCount => "HouseRelinkHookCount",
            PropertyInt::SlayerCreatureType => "SlayerCreatureType",
            PropertyInt::ConfirmationInProgress => "ConfirmationInProgress",
            PropertyInt::ConfirmationTypeInProgress => "ConfirmationTypeInProgress",
            PropertyInt::TsysMutationData => "TsysMutationData",
            PropertyInt::NumItemsInMaterial => "NumItemsInMaterial",
            PropertyInt::NumTimesTinkered => "NumTimesTinkered",
            PropertyInt::AppraisalLongDescDecoration => "AppraisalLongDescDecoration",
            PropertyInt::AppraisalLockpickSuccessPercent => "AppraisalLockpickSuccessPercent",
            PropertyInt::AppraisalPages => "AppraisalPages",
            PropertyInt::AppraisalMaxPages => "AppraisalMaxPages",
            PropertyInt::AppraisalItemSkill => "AppraisalItemSkill",
            PropertyInt::GemCount => "GemCount",
            PropertyInt::GemType => "GemType",
            PropertyInt::ImbuedEffect => "ImbuedEffect",
            PropertyInt::AttackersRawSkillValue => "AttackersRawSkillValue",
            PropertyInt::ChessRank => "ChessRank",
            PropertyInt::ChessTotalGames => "ChessTotalGames",
            PropertyInt::ChessGamesWon => "ChessGamesWon",
            PropertyInt::ChessGamesLost => "ChessGamesLost",
            PropertyInt::TypeOfAlteration => "TypeOfAlteration",
            PropertyInt::SkillToBeAltered => "SkillToBeAltered",
            PropertyInt::SkillAlterationCount => "SkillAlterationCount",
            PropertyInt::HeritageGroup => "HeritageGroup",
            PropertyInt::TransferFromAttribute => "TransferFromAttribute",
            PropertyInt::TransferToAttribute => "TransferToAttribute",
            PropertyInt::AttributeTransferCount => "AttributeTransferCount",
            PropertyInt::FakeFishingSkill => "FakeFishingSkill",
            PropertyInt::NumKeys => "NumKeys",
            PropertyInt::DeathTimestamp => "DeathTimestamp",
            PropertyInt::PkTimestamp => "PkTimestamp",
            PropertyInt::VictimTimestamp => "VictimTimestamp",
            PropertyInt::HookGroup => "HookGroup",
            PropertyInt::AllegianceSwearTimestamp => "AllegianceSwearTimestamp",
            PropertyInt::HousePurchaseTimestamp => "HousePurchaseTimestamp",
            PropertyInt::RedirectableEquippedArmorCount => "RedirectableEquippedArmorCount",
            PropertyInt::MeleeDefenseImbuedEffectTypeCache => "MeleeDefenseImbuedEffectTypeCache",
            PropertyInt::MissileDefenseImbuedEffectTypeCache => "MissileDefenseImbuedEffectTypeCache",
            PropertyInt::MagicDefenseImbuedEffectTypeCache => "MagicDefenseImbuedEffectTypeCache",
            PropertyInt::ElementalDamageBonus => "ElementalDamageBonus",
            PropertyInt::ImbueAttempts => "ImbueAttempts",
            PropertyInt::ImbueSuccesses => "ImbueSuccesses",
            PropertyInt::CreatureKills => "CreatureKills",
            PropertyInt::PlayerKillsPk => "PlayerKillsPk",
            PropertyInt::PlayerKillsPkl => "PlayerKillsPkl",
            PropertyInt::RaresTierOne => "RaresTierOne",
            PropertyInt::RaresTierTwo => "RaresTierTwo",
            PropertyInt::RaresTierThree => "RaresTierThree",
            PropertyInt::RaresTierFour => "RaresTierFour",
            PropertyInt::RaresTierFive => "RaresTierFive",
            PropertyInt::AugmentationStat => "AugmentationStat",
            PropertyInt::AugmentationFamilyStat => "AugmentationFamilyStat",
            PropertyInt::AugmentationInnateFamily => "AugmentationInnateFamily",
            PropertyInt::AugmentationInnateStrength => "AugmentationInnateStrength",
            PropertyInt::AugmentationInnateEndurance => "AugmentationInnateEndurance",
            PropertyInt::AugmentationInnateCoordination => "AugmentationInnateCoordination",
            PropertyInt::AugmentationInnateQuickness => "AugmentationInnateQuickness",
            PropertyInt::AugmentationInnateFocus => "AugmentationInnateFocus",
            PropertyInt::AugmentationInnateSelf => "AugmentationInnateSelf",
            PropertyInt::AugmentationSpecializeSalvaging => "AugmentationSpecializeSalvaging",
            PropertyInt::AugmentationSpecializeItemTinkering => "AugmentationSpecializeItemTinkering",
            PropertyInt::AugmentationSpecializeArmorTinkering => "AugmentationSpecializeArmorTinkering",
            PropertyInt::AugmentationSpecializeMagicItemTinkering => "AugmentationSpecializeMagicItemTinkering",
            PropertyInt::AugmentationSpecializeWeaponTinkering => "AugmentationSpecializeWeaponTinkering",
            PropertyInt::AugmentationExtraPackSlot => "AugmentationExtraPackSlot",
            PropertyInt::AugmentationIncreasedCarryingCapacity => "AugmentationIncreasedCarryingCapacity",
            PropertyInt::AugmentationLessDeathItemLoss => "AugmentationLessDeathItemLoss",
            PropertyInt::AugmentationSpellsRemainPastDeath => "AugmentationSpellsRemainPastDeath",
            PropertyInt::AugmentationCriticalDefense => "AugmentationCriticalDefense",
            PropertyInt::AugmentationBonusXp => "AugmentationBonusXp",
            PropertyInt::AugmentationBonusSalvage => "AugmentationBonusSalvage",
            PropertyInt::AugmentationBonusImbueChance => "AugmentationBonusImbueChance",
            PropertyInt::AugmentationFasterRegen => "AugmentationFasterRegen",
            PropertyInt::AugmentationIncreasedSpellDuration => "AugmentationIncreasedSpellDuration",
            PropertyInt::AugmentationResistanceFamily => "AugmentationResistanceFamily",
            PropertyInt::AugmentationResistanceSlash => "AugmentationResistanceSlash",
            PropertyInt::AugmentationResistancePierce => "AugmentationResistancePierce",
            PropertyInt::AugmentationResistanceBlunt => "AugmentationResistanceBlunt",
            PropertyInt::AugmentationResistanceAcid => "AugmentationResistanceAcid",
            PropertyInt::AugmentationResistanceFire => "AugmentationResistanceFire",
            PropertyInt::AugmentationResistanceFrost => "AugmentationResistanceFrost",
            PropertyInt::AugmentationResistanceLightning => "AugmentationResistanceLightning",
            PropertyInt::RaresTierOneLogin => "RaresTierOneLogin",
            PropertyInt::RaresTierTwoLogin => "RaresTierTwoLogin",
            PropertyInt::RaresTierThreeLogin => "RaresTierThreeLogin",
            PropertyInt::RaresTierFourLogin => "RaresTierFourLogin",
            PropertyInt::RaresTierFiveLogin => "RaresTierFiveLogin",
            PropertyInt::RaresLoginTimestamp => "RaresLoginTimestamp",
            PropertyInt::RaresTierSix => "RaresTierSix",
            PropertyInt::RaresTierSeven => "RaresTierSeven",
            PropertyInt::RaresTierSixLogin => "RaresTierSixLogin",
            PropertyInt::RaresTierSevenLogin => "RaresTierSevenLogin",
            PropertyInt::ItemAttributeLimit => "ItemAttributeLimit",
            PropertyInt::ItemAttributeLevelLimit => "ItemAttributeLevelLimit",
            PropertyInt::ItemAttribute2ndLimit => "ItemAttribute2ndLimit",
            PropertyInt::ItemAttribute2ndLevelLimit => "ItemAttribute2ndLevelLimit",
            PropertyInt::CharacterTitleId => "CharacterTitleId",
            PropertyInt::NumCharacterTitles => "NumCharacterTitles",
            PropertyInt::ResistanceModifierType => "ResistanceModifierType",
            PropertyInt::FreeTinkersBitfield => "FreeTinkersBitfield",
            PropertyInt::EquipmentSetId => "EquipmentSetId",
            PropertyInt::PetClass => "PetClass",
            PropertyInt::Lifespan => "Lifespan",
            PropertyInt::RemainingLifespan => "RemainingLifespan",
            PropertyInt::UseCreateQuantity => "UseCreateQuantity",
            PropertyInt::WieldRequirements2 => "WieldRequirements2",
            PropertyInt::WieldSkillType2 => "WieldSkillType2",
            PropertyInt::WieldDifficulty2 => "WieldDifficulty2",
            PropertyInt::WieldRequirements3 => "WieldRequirements3",
            PropertyInt::WieldSkillType3 => "WieldSkillType3",
            PropertyInt::WieldDifficulty3 => "WieldDifficulty3",
            PropertyInt::WieldRequirements4 => "WieldRequirements4",
            PropertyInt::WieldSkillType4 => "WieldSkillType4",
            PropertyInt::WieldDifficulty4 => "WieldDifficulty4",
            PropertyInt::Unique => "Unique",
            PropertyInt::SharedCooldown => "SharedCooldown",
            PropertyInt::Faction1Bits => "Faction1Bits",
            PropertyInt::Faction2Bits => "Faction2Bits",
            PropertyInt::Faction3Bits => "Faction3Bits",
            PropertyInt::Hatred1Bits => "Hatred1Bits",
            PropertyInt::Hatred2Bits => "Hatred2Bits",
            PropertyInt::Hatred3Bits => "Hatred3Bits",
            PropertyInt::SocietyRankCelhan => "SocietyRankCelhan",
            PropertyInt::SocietyRankEldweb => "SocietyRankEldweb",
            PropertyInt::SocietyRankRadblo => "SocietyRankRadblo",
            PropertyInt::HearLocalSignals => "HearLocalSignals",
            PropertyInt::HearLocalSignalsRadius => "HearLocalSignalsRadius",
            PropertyInt::Cleaving => "Cleaving",
            PropertyInt::AugmentationSpecializeGearcraft => "AugmentationSpecializeGearcraft",
            PropertyInt::AugmentationInfusedCreatureMagic => "AugmentationInfusedCreatureMagic",
            PropertyInt::AugmentationInfusedItemMagic => "AugmentationInfusedItemMagic",
            PropertyInt::AugmentationInfusedLifeMagic => "AugmentationInfusedLifeMagic",
            PropertyInt::AugmentationInfusedWarMagic => "AugmentationInfusedWarMagic",
            PropertyInt::AugmentationCriticalExpertise => "AugmentationCriticalExpertise",
            PropertyInt::AugmentationCriticalPower => "AugmentationCriticalPower",
            PropertyInt::AugmentationSkilledMelee => "AugmentationSkilledMelee",
            PropertyInt::AugmentationSkilledMissile => "AugmentationSkilledMissile",
            PropertyInt::AugmentationSkilledMagic => "AugmentationSkilledMagic",
            PropertyInt::ImbuedEffect2 => "ImbuedEffect2",
            PropertyInt::ImbuedEffect3 => "ImbuedEffect3",
            PropertyInt::ImbuedEffect4 => "ImbuedEffect4",
            PropertyInt::ImbuedEffect5 => "ImbuedEffect5",
            PropertyInt::DamageRating => "DamageRating",
            PropertyInt::DamageResistRating => "DamageResistRating",
            PropertyInt::AugmentationDamageBonus => "AugmentationDamageBonus",
            PropertyInt::AugmentationDamageReduction => "AugmentationDamageReduction",
            PropertyInt::ImbueStackingBits => "ImbueStackingBits",
            PropertyInt::HealOverTime => "HealOverTime",
            PropertyInt::CritRating => "CritRating",
            PropertyInt::CritDamageRating => "CritDamageRating",
            PropertyInt::CritResistRating => "CritResistRating",
            PropertyInt::CritDamageResistRating => "CritDamageResistRating",
            PropertyInt::HealingResistRating => "HealingResistRating",
            PropertyInt::DamageOverTime => "DamageOverTime",
            PropertyInt::ItemMaxLevel => "ItemMaxLevel",
            PropertyInt::ItemXpStyle => "ItemXpStyle",
            PropertyInt::EquipmentSetExtra => "EquipmentSetExtra",
            PropertyInt::AetheriaBitfield => "AetheriaBitfield",
            PropertyInt::HealingBoostRating => "HealingBoostRating",
            PropertyInt::HeritageSpecificArmor => "HeritageSpecificArmor",
            PropertyInt::AlternateRacialSkills => "AlternateRacialSkills",
            PropertyInt::AugmentationJackOfAllTrades => "AugmentationJackOfAllTrades",
            PropertyInt::AugmentationResistanceNether => "AugmentationResistanceNether",
            PropertyInt::AugmentationInfusedVoidMagic => "AugmentationInfusedVoidMagic",
            PropertyInt::WeaknessRating => "WeaknessRating",
            PropertyInt::NetherOverTime => "NetherOverTime",
            PropertyInt::NetherResistRating => "NetherResistRating",
            PropertyInt::LuminanceAward => "LuminanceAward",
            PropertyInt::LumAugDamageRating => "LumAugDamageRating",
            PropertyInt::LumAugDamageReductionRating => "LumAugDamageReductionRating",
            PropertyInt::LumAugCritDamageRating => "LumAugCritDamageRating",
            PropertyInt::LumAugCritReductionRating => "LumAugCritReductionRating",
            PropertyInt::LumAugSurgeEffectRating => "LumAugSurgeEffectRating",
            PropertyInt::LumAugSurgeChanceRating => "LumAugSurgeChanceRating",
            PropertyInt::LumAugItemManaUsage => "LumAugItemManaUsage",
            PropertyInt::LumAugItemManaGain => "LumAugItemManaGain",
            PropertyInt::LumAugVitality => "LumAugVitality",
            PropertyInt::LumAugHealingRating => "LumAugHealingRating",
            PropertyInt::LumAugSkilledCraft => "LumAugSkilledCraft",
            PropertyInt::LumAugSkilledSpec => "LumAugSkilledSpec",
            PropertyInt::LumAugNoDestroyCraft => "LumAugNoDestroyCraft",
            PropertyInt::RestrictInteraction => "RestrictInteraction",
            PropertyInt::OlthoiLootTimestamp => "OlthoiLootTimestamp",
            PropertyInt::OlthoiLootStep => "OlthoiLootStep",
            PropertyInt::UseCreatesContractId => "UseCreatesContractId",
            PropertyInt::DotResistRating => "DotResistRating",
            PropertyInt::LifeResistRating => "LifeResistRating",
            PropertyInt::CloakWeaveProc => "CloakWeaveProc",
            PropertyInt::WeaponType => "WeaponType",
            PropertyInt::MeleeMastery => "MeleeMastery",
            PropertyInt::RangedMastery => "RangedMastery",
            PropertyInt::SneakAttackRating => "SneakAttackRating",
            PropertyInt::RecklessnessRating => "RecklessnessRating",
            PropertyInt::DeceptionRating => "DeceptionRating",
            PropertyInt::CombatPetRange => "CombatPetRange",
            PropertyInt::WeaponAuraDamage => "WeaponAuraDamage",
            PropertyInt::WeaponAuraSpeed => "WeaponAuraSpeed",
            PropertyInt::SummoningMastery => "SummoningMastery",
            PropertyInt::HeartbeatLifespan => "HeartbeatLifespan",
            PropertyInt::UseLevelRequirement => "UseLevelRequirement",
            PropertyInt::LumAugAllSkills => "LumAugAllSkills",
            PropertyInt::UseRequiresSkill => "UseRequiresSkill",
            PropertyInt::UseRequiresSkillLevel => "UseRequiresSkillLevel",
            PropertyInt::UseRequiresSkillSpec => "UseRequiresSkillSpec",
            PropertyInt::UseRequiresLevel => "UseRequiresLevel",
            PropertyInt::GearDamage => "GearDamage",
            PropertyInt::GearDamageResist => "GearDamageResist",
            PropertyInt::GearCrit => "GearCrit",
            PropertyInt::GearCritResist => "GearCritResist",
            PropertyInt::GearCritDamage => "GearCritDamage",
            PropertyInt::GearCritDamageResist => "GearCritDamageResist",
            PropertyInt::GearHealingBoost => "GearHealingBoost",
            PropertyInt::GearNetherResist => "GearNetherResist",
            PropertyInt::GearLifeResist => "GearLifeResist",
            PropertyInt::GearMaxHealth => "GearMaxHealth",
            PropertyInt::Unknown380 => "Unknown380",
            PropertyInt::PKDamageRating => "PKDamageRating",
            PropertyInt::PKDamageResistRating => "PKDamageResistRating",
            PropertyInt::GearPKDamageRating => "GearPKDamageRating",
            PropertyInt::GearPKDamageResistRating => "GearPKDamageResistRating",
            PropertyInt::Unknown385 => "Unknown385",
            PropertyInt::Overpower => "Overpower",
            PropertyInt::OverpowerResist => "OverpowerResist",
            PropertyInt::GearOverpower => "GearOverpower",
            PropertyInt::GearOverpowerResist => "GearOverpowerResist",
            PropertyInt::Enlightenment => "Enlightenment",
        };
        write!(f, "{}", s)
    }
}

/// The PropertyInstanceId identifies a specific Character or Object instance property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyInstanceId {
    Owner = 0x1,
    Container = 0x2,
    Wielder = 0x3,
    Freezer = 0x4,
    Viewer = 0x5,
    Generator = 0x6,
    Scribe = 0x7,
    CurrentCombatTarget = 0x8,
    CurrentEnemy = 0x9,
    ProjectileLauncher = 0xA,
    CurrentAttacker = 0xB,
    CurrentDamager = 0xC,
    CurrentFollowTarget = 0xD,
    CurrentAppraisalTarget = 0xE,
    CurrentFellowshipAppraisalTarget = 0xF,
    ActivationTarget = 0x10,
    Creator = 0x11,
    Victim = 0x12,
    Killer = 0x13,
    Vendor = 0x14,
    Customer = 0x15,
    Bonded = 0x16,
    Wounder = 0x17,
    Allegiance = 0x18,
    Patron = 0x19,
    Monarch = 0x1A,
    CombatTarget = 0x1B,
    HealthQueryTarget = 0x1C,
    LastUnlocker = 0x1D,
    CrashAndTurnTarget = 0x1E,
    AllowedActivator = 0x1F,
    HouseOwner = 0x20,
    House = 0x21,
    Slumlord = 0x22,
    ManaQueryTarget = 0x23,
    CurrentGame = 0x24,
    RequestedAppraisalTarget = 0x25,
    AllowedWielder = 0x26,
    AssignedTarget = 0x27,
    LimboSource = 0x28,
    Snooper = 0x29,
    TeleportedCharacter = 0x2A,
    Pet = 0x2B,
    PetOwner = 0x2C,
    PetDevice = 0x2D,
}

impl crate::readers::ACDataType for PropertyInstanceId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyInstanceId::try_from(value)?)
    }
}

impl crate::writers::ACWritable for PropertyInstanceId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for PropertyInstanceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PropertyInstanceId::Owner => "Owner",
            PropertyInstanceId::Container => "Container",
            PropertyInstanceId::Wielder => "Wielder",
            PropertyInstanceId::Freezer => "Freezer",
            PropertyInstanceId::Viewer => "Viewer",
            PropertyInstanceId::Generator => "Generator",
            PropertyInstanceId::Scribe => "Scribe",
            PropertyInstanceId::CurrentCombatTarget => "CurrentCombatTarget",
            PropertyInstanceId::CurrentEnemy => "CurrentEnemy",
            PropertyInstanceId::ProjectileLauncher => "ProjectileLauncher",
            PropertyInstanceId::CurrentAttacker => "CurrentAttacker",
            PropertyInstanceId::CurrentDamager => "CurrentDamager",
            PropertyInstanceId::CurrentFollowTarget => "CurrentFollowTarget",
            PropertyInstanceId::CurrentAppraisalTarget => "CurrentAppraisalTarget",
            PropertyInstanceId::CurrentFellowshipAppraisalTarget => "CurrentFellowshipAppraisalTarget",
            PropertyInstanceId::ActivationTarget => "ActivationTarget",
            PropertyInstanceId::Creator => "Creator",
            PropertyInstanceId::Victim => "Victim",
            PropertyInstanceId::Killer => "Killer",
            PropertyInstanceId::Vendor => "Vendor",
            PropertyInstanceId::Customer => "Customer",
            PropertyInstanceId::Bonded => "Bonded",
            PropertyInstanceId::Wounder => "Wounder",
            PropertyInstanceId::Allegiance => "Allegiance",
            PropertyInstanceId::Patron => "Patron",
            PropertyInstanceId::Monarch => "Monarch",
            PropertyInstanceId::CombatTarget => "CombatTarget",
            PropertyInstanceId::HealthQueryTarget => "HealthQueryTarget",
            PropertyInstanceId::LastUnlocker => "LastUnlocker",
            PropertyInstanceId::CrashAndTurnTarget => "CrashAndTurnTarget",
            PropertyInstanceId::AllowedActivator => "AllowedActivator",
            PropertyInstanceId::HouseOwner => "HouseOwner",
            PropertyInstanceId::House => "House",
            PropertyInstanceId::Slumlord => "Slumlord",
            PropertyInstanceId::ManaQueryTarget => "ManaQueryTarget",
            PropertyInstanceId::CurrentGame => "CurrentGame",
            PropertyInstanceId::RequestedAppraisalTarget => "RequestedAppraisalTarget",
            PropertyInstanceId::AllowedWielder => "AllowedWielder",
            PropertyInstanceId::AssignedTarget => "AssignedTarget",
            PropertyInstanceId::LimboSource => "LimboSource",
            PropertyInstanceId::Snooper => "Snooper",
            PropertyInstanceId::TeleportedCharacter => "TeleportedCharacter",
            PropertyInstanceId::Pet => "Pet",
            PropertyInstanceId::PetOwner => "PetOwner",
            PropertyInstanceId::PetDevice => "PetDevice",
        };
        write!(f, "{}", s)
    }
}

/// The PropertyPosition identifies a specific Character or Object position property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyPosition {
    Location = 0x1,
    Destination = 0x2,
    Instantiation = 0x3,
    Sanctuary = 0x4,
    Home = 0x5,
    ActivationMove = 0x6,
    Target = 0x7,
    LinkedPortalOne = 0x8,
    LastPortal = 0x9,
    PortalStorm = 0xA,
    CrashAndTurn = 0xB,
    PortalSummonLoc = 0xC,
    HouseBoot = 0xD,
    LastOutsideDeath = 0xE,
    LinkedLifestone = 0xF,
    LinkedPortalTwo = 0x10,
    Save1 = 0x11,
    Save2 = 0x12,
    Save3 = 0x13,
    Save4 = 0x14,
    Save5 = 0x15,
    Save6 = 0x16,
    Save7 = 0x17,
    Save8 = 0x18,
    Save9 = 0x19,
    RelativeDestination = 0x1A,
    TeleportedCharacter = 0x1B,
}

impl crate::readers::ACDataType for PropertyPosition {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyPosition::try_from(value)?)
    }
}

impl crate::writers::ACWritable for PropertyPosition {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for PropertyPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PropertyPosition::Location => "Location",
            PropertyPosition::Destination => "Destination",
            PropertyPosition::Instantiation => "Instantiation",
            PropertyPosition::Sanctuary => "Sanctuary",
            PropertyPosition::Home => "Home",
            PropertyPosition::ActivationMove => "ActivationMove",
            PropertyPosition::Target => "Target",
            PropertyPosition::LinkedPortalOne => "LinkedPortalOne",
            PropertyPosition::LastPortal => "LastPortal",
            PropertyPosition::PortalStorm => "PortalStorm",
            PropertyPosition::CrashAndTurn => "CrashAndTurn",
            PropertyPosition::PortalSummonLoc => "PortalSummonLoc",
            PropertyPosition::HouseBoot => "HouseBoot",
            PropertyPosition::LastOutsideDeath => "LastOutsideDeath",
            PropertyPosition::LinkedLifestone => "LinkedLifestone",
            PropertyPosition::LinkedPortalTwo => "LinkedPortalTwo",
            PropertyPosition::Save1 => "Save1",
            PropertyPosition::Save2 => "Save2",
            PropertyPosition::Save3 => "Save3",
            PropertyPosition::Save4 => "Save4",
            PropertyPosition::Save5 => "Save5",
            PropertyPosition::Save6 => "Save6",
            PropertyPosition::Save7 => "Save7",
            PropertyPosition::Save8 => "Save8",
            PropertyPosition::Save9 => "Save9",
            PropertyPosition::RelativeDestination => "RelativeDestination",
            PropertyPosition::TeleportedCharacter => "TeleportedCharacter",
        };
        write!(f, "{}", s)
    }
}

/// The PropertyString identifies a specific Character or Object string property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyString {
    Name = 0x1,
    Title = 0x2,
    Sex = 0x3,
    HeritageGroup = 0x4,
    Template = 0x5,
    AttackersName = 0x6,
    Inscription = 0x7,
    ScribeName = 0x8,
    VendorsName = 0x9,
    Fellowship = 0xA,
    MonarchsName = 0xB,
    LockCode = 0xC,
    KeyCode = 0xD,
    Use = 0xE,
    ShortDesc = 0xF,
    LongDesc = 0x10,
    ActivationTalk = 0x11,
    UseMessage = 0x12,
    ItemHeritageGroupRestriction = 0x13,
    PluralName = 0x14,
    MonarchsTitle = 0x15,
    ActivationFailure = 0x16,
    ScribeAccount = 0x17,
    TownName = 0x18,
    CraftsmanName = 0x19,
    UsePkServerError = 0x1A,
    ScoreCachedText = 0x1B,
    ScoreDefaultEntryFormat = 0x1C,
    ScoreFirstEntryFormat = 0x1D,
    ScoreLastEntryFormat = 0x1E,
    ScoreOnlyEntryFormat = 0x1F,
    ScoreNoEntry = 0x20,
    Quest = 0x21,
    GeneratorEvent = 0x22,
    PatronsTitle = 0x23,
    HouseOwnerName = 0x24,
    QuestRestriction = 0x25,
    AppraisalPortalDestination = 0x26,
    TinkerName = 0x27,
    ImbuerName = 0x28,
    HouseOwnerAccount = 0x29,
    DisplayName = 0x2A,
    DateOfBirth = 0x2B,
    ThirdPartyApi = 0x2C,
    KillQuest = 0x2D,
    Afk = 0x2E,
    AllegianceName = 0x2F,
    AugmentationAddQuest = 0x30,
    KillQuest2 = 0x31,
    KillQuest3 = 0x32,
    UseSendsSignal = 0x33,
    GearPlatingName = 0x34,
}

impl crate::readers::ACDataType for PropertyString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyString::try_from(value)?)
    }
}

impl crate::writers::ACWritable for PropertyString {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for PropertyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PropertyString::Name => "Name",
            PropertyString::Title => "Title",
            PropertyString::Sex => "Sex",
            PropertyString::HeritageGroup => "HeritageGroup",
            PropertyString::Template => "Template",
            PropertyString::AttackersName => "AttackersName",
            PropertyString::Inscription => "Inscription",
            PropertyString::ScribeName => "ScribeName",
            PropertyString::VendorsName => "VendorsName",
            PropertyString::Fellowship => "Fellowship",
            PropertyString::MonarchsName => "MonarchsName",
            PropertyString::LockCode => "LockCode",
            PropertyString::KeyCode => "KeyCode",
            PropertyString::Use => "Use",
            PropertyString::ShortDesc => "ShortDesc",
            PropertyString::LongDesc => "LongDesc",
            PropertyString::ActivationTalk => "ActivationTalk",
            PropertyString::UseMessage => "UseMessage",
            PropertyString::ItemHeritageGroupRestriction => "ItemHeritageGroupRestriction",
            PropertyString::PluralName => "PluralName",
            PropertyString::MonarchsTitle => "MonarchsTitle",
            PropertyString::ActivationFailure => "ActivationFailure",
            PropertyString::ScribeAccount => "ScribeAccount",
            PropertyString::TownName => "TownName",
            PropertyString::CraftsmanName => "CraftsmanName",
            PropertyString::UsePkServerError => "UsePkServerError",
            PropertyString::ScoreCachedText => "ScoreCachedText",
            PropertyString::ScoreDefaultEntryFormat => "ScoreDefaultEntryFormat",
            PropertyString::ScoreFirstEntryFormat => "ScoreFirstEntryFormat",
            PropertyString::ScoreLastEntryFormat => "ScoreLastEntryFormat",
            PropertyString::ScoreOnlyEntryFormat => "ScoreOnlyEntryFormat",
            PropertyString::ScoreNoEntry => "ScoreNoEntry",
            PropertyString::Quest => "Quest",
            PropertyString::GeneratorEvent => "GeneratorEvent",
            PropertyString::PatronsTitle => "PatronsTitle",
            PropertyString::HouseOwnerName => "HouseOwnerName",
            PropertyString::QuestRestriction => "QuestRestriction",
            PropertyString::AppraisalPortalDestination => "AppraisalPortalDestination",
            PropertyString::TinkerName => "TinkerName",
            PropertyString::ImbuerName => "ImbuerName",
            PropertyString::HouseOwnerAccount => "HouseOwnerAccount",
            PropertyString::DisplayName => "DisplayName",
            PropertyString::DateOfBirth => "DateOfBirth",
            PropertyString::ThirdPartyApi => "ThirdPartyApi",
            PropertyString::KillQuest => "KillQuest",
            PropertyString::Afk => "Afk",
            PropertyString::AllegianceName => "AllegianceName",
            PropertyString::AugmentationAddQuest => "AugmentationAddQuest",
            PropertyString::KillQuest2 => "KillQuest2",
            PropertyString::KillQuest3 => "KillQuest3",
            PropertyString::UseSendsSignal => "UseSendsSignal",
            PropertyString::GearPlatingName => "GearPlatingName",
        };
        write!(f, "{}", s)
    }
}

/// The PropertyFloat identifies a specific Character or Object float property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyFloat {
    Undef = 0x0,
    HeartbeatInterval = 0x1,
    HeartbeatTimestamp = 0x2,
    HealthRate = 0x3,
    StaminaRate = 0x4,
    ManaRate = 0x5,
    HealthUponResurrection = 0x6,
    StaminaUponResurrection = 0x7,
    ManaUponResurrection = 0x8,
    StartTime = 0x9,
    StopTime = 0xA,
    ResetInterval = 0xB,
    Shade = 0xC,
    ArmorModVsSlash = 0xD,
    ArmorModVsPierce = 0xE,
    ArmorModVsBludgeon = 0xF,
    ArmorModVsCold = 0x10,
    ArmorModVsFire = 0x11,
    ArmorModVsAcid = 0x12,
    ArmorModVsElectric = 0x13,
    CombatSpeed = 0x14,
    WeaponLength = 0x15,
    DamageVariance = 0x16,
    CurrentPowerMod = 0x17,
    AccuracyMod = 0x18,
    StrengthMod = 0x19,
    MaximumVelocity = 0x1A,
    RotationSpeed = 0x1B,
    MotionTimestamp = 0x1C,
    WeaponDefense = 0x1D,
    WimpyLevel = 0x1E,
    VisualAwarenessRange = 0x1F,
    AuralAwarenessRange = 0x20,
    PerceptionLevel = 0x21,
    PowerupTime = 0x22,
    MaxChargeDistance = 0x23,
    ChargeSpeed = 0x24,
    BuyPrice = 0x25,
    SellPrice = 0x26,
    DefaultScale = 0x27,
    LockpickMod = 0x28,
    RegenerationInterval = 0x29,
    RegenerationTimestamp = 0x2A,
    GeneratorRadius = 0x2B,
    TimeToRot = 0x2C,
    DeathTimestamp = 0x2D,
    PkTimestamp = 0x2E,
    VictimTimestamp = 0x2F,
    LoginTimestamp = 0x30,
    CreationTimestamp = 0x31,
    MinimumTimeSincePk = 0x32,
    DeprecatedHousekeepingPriority = 0x33,
    AbuseLoggingTimestamp = 0x34,
    LastPortalTeleportTimestamp = 0x35,
    UseRadius = 0x36,
    HomeRadius = 0x37,
    ReleasedTimestamp = 0x38,
    MinHomeRadius = 0x39,
    Facing = 0x3A,
    ResetTimestamp = 0x3B,
    LogoffTimestamp = 0x3C,
    EconRecoveryInterval = 0x3D,
    WeaponOffense = 0x3E,
    DamageMod = 0x3F,
    ResistSlash = 0x40,
    ResistPierce = 0x41,
    ResistBludgeon = 0x42,
    ResistFire = 0x43,
    ResistCold = 0x44,
    ResistAcid = 0x45,
    ResistElectric = 0x46,
    ResistHealthBoost = 0x47,
    ResistStaminaDrain = 0x48,
    ResistStaminaBoost = 0x49,
    ResistManaDrain = 0x4A,
    ResistManaBoost = 0x4B,
    Translucency = 0x4C,
    PhysicsScriptIntensity = 0x4D,
    Friction = 0x4E,
    Elasticity = 0x4F,
    AiUseMagicDelay = 0x50,
    ItemMinSpellcraftMod = 0x51,
    ItemMaxSpellcraftMod = 0x52,
    ItemRankProbability = 0x53,
    Shade2 = 0x54,
    Shade3 = 0x55,
    Shade4 = 0x56,
    ItemEfficiency = 0x57,
    ItemManaUpdateTimestamp = 0x58,
    SpellGestureSpeedMod = 0x59,
    SpellStanceSpeedMod = 0x5A,
    AllegianceAppraisalTimestamp = 0x5B,
    PowerLevel = 0x5C,
    AccuracyLevel = 0x5D,
    AttackAngle = 0x5E,
    AttackTimestamp = 0x5F,
    CheckpointTimestamp = 0x60,
    SoldTimestamp = 0x61,
    UseTimestamp = 0x62,
    UseLockTimestamp = 0x63,
    HealkitMod = 0x64,
    FrozenTimestamp = 0x65,
    HealthRateMod = 0x66,
    AllegianceSwearTimestamp = 0x67,
    ObviousRadarRange = 0x68,
    HotspotCycleTime = 0x69,
    HotspotCycleTimeVariance = 0x6A,
    SpamTimestamp = 0x6B,
    SpamRate = 0x6C,
    BondWieldedTreasure = 0x6D,
    BulkMod = 0x6E,
    SizeMod = 0x6F,
    GagTimestamp = 0x70,
    GeneratorUpdateTimestamp = 0x71,
    DeathSpamTimestamp = 0x72,
    DeathSpamRate = 0x73,
    WildAttackProbability = 0x74,
    FocusedProbability = 0x75,
    CrashAndTurnProbability = 0x76,
    CrashAndTurnRadius = 0x77,
    CrashAndTurnBias = 0x78,
    GeneratorInitialDelay = 0x79,
    AiAcquireHealth = 0x7A,
    AiAcquireStamina = 0x7B,
    AiAcquireMana = 0x7C,
    ResistHealthDrain = 0x7D,
    LifestoneProtectionTimestamp = 0x7E,
    AiCounteractEnchantment = 0x7F,
    AiDispelEnchantment = 0x80,
    TradeTimestamp = 0x81,
    AiTargetedDetectionRadius = 0x82,
    EmotePriority = 0x83,
    LastTeleportStartTimestamp = 0x84,
    EventSpamTimestamp = 0x85,
    EventSpamRate = 0x86,
    InventoryOffset = 0x87,
    CriticalMultiplier = 0x88,
    ManaStoneDestroyChance = 0x89,
    SlayerDamageBonus = 0x8A,
    AllegianceInfoSpamTimestamp = 0x8B,
    AllegianceInfoSpamRate = 0x8C,
    NextSpellcastTimestamp = 0x8D,
    AppraisalRequestedTimestamp = 0x8E,
    AppraisalHeartbeatDueTimestamp = 0x8F,
    ManaConversionMod = 0x90,
    LastPkAttackTimestamp = 0x91,
    FellowshipUpdateTimestamp = 0x92,
    CriticalFrequency = 0x93,
    LimboStartTimestamp = 0x94,
    WeaponMissileDefense = 0x95,
    WeaponMagicDefense = 0x96,
    IgnoreShield = 0x97,
    ElementalDamageMod = 0x98,
    StartMissileAttackTimestamp = 0x99,
    LastRareUsedTimestamp = 0x9A,
    IgnoreArmor = 0x9B,
    ProcSpellRate = 0x9C,
    ResistanceModifier = 0x9D,
    AllegianceGagTimestamp = 0x9E,
    AbsorbMagicDamage = 0x9F,
    CachedMaxAbsorbMagicDamage = 0xA0,
    GagDuration = 0xA1,
    AllegianceGagDuration = 0xA2,
    GlobalXpMod = 0xA3,
    HealingModifier = 0xA4,
    ArmorModVsNether = 0xA5,
    ResistNether = 0xA6,
    CooldownDuration = 0xA7,
    WeaponAuraOffense = 0xA8,
    WeaponAuraDefense = 0xA9,
    WeaponAuraElemental = 0xAA,
    WeaponAuraManaConv = 0xAB,
}

impl crate::readers::ACDataType for PropertyFloat {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyFloat::try_from(value)?)
    }
}

impl crate::writers::ACWritable for PropertyFloat {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for PropertyFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PropertyFloat::Undef => "Undef",
            PropertyFloat::HeartbeatInterval => "HeartbeatInterval",
            PropertyFloat::HeartbeatTimestamp => "HeartbeatTimestamp",
            PropertyFloat::HealthRate => "HealthRate",
            PropertyFloat::StaminaRate => "StaminaRate",
            PropertyFloat::ManaRate => "ManaRate",
            PropertyFloat::HealthUponResurrection => "HealthUponResurrection",
            PropertyFloat::StaminaUponResurrection => "StaminaUponResurrection",
            PropertyFloat::ManaUponResurrection => "ManaUponResurrection",
            PropertyFloat::StartTime => "StartTime",
            PropertyFloat::StopTime => "StopTime",
            PropertyFloat::ResetInterval => "ResetInterval",
            PropertyFloat::Shade => "Shade",
            PropertyFloat::ArmorModVsSlash => "ArmorModVsSlash",
            PropertyFloat::ArmorModVsPierce => "ArmorModVsPierce",
            PropertyFloat::ArmorModVsBludgeon => "ArmorModVsBludgeon",
            PropertyFloat::ArmorModVsCold => "ArmorModVsCold",
            PropertyFloat::ArmorModVsFire => "ArmorModVsFire",
            PropertyFloat::ArmorModVsAcid => "ArmorModVsAcid",
            PropertyFloat::ArmorModVsElectric => "ArmorModVsElectric",
            PropertyFloat::CombatSpeed => "CombatSpeed",
            PropertyFloat::WeaponLength => "WeaponLength",
            PropertyFloat::DamageVariance => "DamageVariance",
            PropertyFloat::CurrentPowerMod => "CurrentPowerMod",
            PropertyFloat::AccuracyMod => "AccuracyMod",
            PropertyFloat::StrengthMod => "StrengthMod",
            PropertyFloat::MaximumVelocity => "MaximumVelocity",
            PropertyFloat::RotationSpeed => "RotationSpeed",
            PropertyFloat::MotionTimestamp => "MotionTimestamp",
            PropertyFloat::WeaponDefense => "WeaponDefense",
            PropertyFloat::WimpyLevel => "WimpyLevel",
            PropertyFloat::VisualAwarenessRange => "VisualAwarenessRange",
            PropertyFloat::AuralAwarenessRange => "AuralAwarenessRange",
            PropertyFloat::PerceptionLevel => "PerceptionLevel",
            PropertyFloat::PowerupTime => "PowerupTime",
            PropertyFloat::MaxChargeDistance => "MaxChargeDistance",
            PropertyFloat::ChargeSpeed => "ChargeSpeed",
            PropertyFloat::BuyPrice => "BuyPrice",
            PropertyFloat::SellPrice => "SellPrice",
            PropertyFloat::DefaultScale => "DefaultScale",
            PropertyFloat::LockpickMod => "LockpickMod",
            PropertyFloat::RegenerationInterval => "RegenerationInterval",
            PropertyFloat::RegenerationTimestamp => "RegenerationTimestamp",
            PropertyFloat::GeneratorRadius => "GeneratorRadius",
            PropertyFloat::TimeToRot => "TimeToRot",
            PropertyFloat::DeathTimestamp => "DeathTimestamp",
            PropertyFloat::PkTimestamp => "PkTimestamp",
            PropertyFloat::VictimTimestamp => "VictimTimestamp",
            PropertyFloat::LoginTimestamp => "LoginTimestamp",
            PropertyFloat::CreationTimestamp => "CreationTimestamp",
            PropertyFloat::MinimumTimeSincePk => "MinimumTimeSincePk",
            PropertyFloat::DeprecatedHousekeepingPriority => "DeprecatedHousekeepingPriority",
            PropertyFloat::AbuseLoggingTimestamp => "AbuseLoggingTimestamp",
            PropertyFloat::LastPortalTeleportTimestamp => "LastPortalTeleportTimestamp",
            PropertyFloat::UseRadius => "UseRadius",
            PropertyFloat::HomeRadius => "HomeRadius",
            PropertyFloat::ReleasedTimestamp => "ReleasedTimestamp",
            PropertyFloat::MinHomeRadius => "MinHomeRadius",
            PropertyFloat::Facing => "Facing",
            PropertyFloat::ResetTimestamp => "ResetTimestamp",
            PropertyFloat::LogoffTimestamp => "LogoffTimestamp",
            PropertyFloat::EconRecoveryInterval => "EconRecoveryInterval",
            PropertyFloat::WeaponOffense => "WeaponOffense",
            PropertyFloat::DamageMod => "DamageMod",
            PropertyFloat::ResistSlash => "ResistSlash",
            PropertyFloat::ResistPierce => "ResistPierce",
            PropertyFloat::ResistBludgeon => "ResistBludgeon",
            PropertyFloat::ResistFire => "ResistFire",
            PropertyFloat::ResistCold => "ResistCold",
            PropertyFloat::ResistAcid => "ResistAcid",
            PropertyFloat::ResistElectric => "ResistElectric",
            PropertyFloat::ResistHealthBoost => "ResistHealthBoost",
            PropertyFloat::ResistStaminaDrain => "ResistStaminaDrain",
            PropertyFloat::ResistStaminaBoost => "ResistStaminaBoost",
            PropertyFloat::ResistManaDrain => "ResistManaDrain",
            PropertyFloat::ResistManaBoost => "ResistManaBoost",
            PropertyFloat::Translucency => "Translucency",
            PropertyFloat::PhysicsScriptIntensity => "PhysicsScriptIntensity",
            PropertyFloat::Friction => "Friction",
            PropertyFloat::Elasticity => "Elasticity",
            PropertyFloat::AiUseMagicDelay => "AiUseMagicDelay",
            PropertyFloat::ItemMinSpellcraftMod => "ItemMinSpellcraftMod",
            PropertyFloat::ItemMaxSpellcraftMod => "ItemMaxSpellcraftMod",
            PropertyFloat::ItemRankProbability => "ItemRankProbability",
            PropertyFloat::Shade2 => "Shade2",
            PropertyFloat::Shade3 => "Shade3",
            PropertyFloat::Shade4 => "Shade4",
            PropertyFloat::ItemEfficiency => "ItemEfficiency",
            PropertyFloat::ItemManaUpdateTimestamp => "ItemManaUpdateTimestamp",
            PropertyFloat::SpellGestureSpeedMod => "SpellGestureSpeedMod",
            PropertyFloat::SpellStanceSpeedMod => "SpellStanceSpeedMod",
            PropertyFloat::AllegianceAppraisalTimestamp => "AllegianceAppraisalTimestamp",
            PropertyFloat::PowerLevel => "PowerLevel",
            PropertyFloat::AccuracyLevel => "AccuracyLevel",
            PropertyFloat::AttackAngle => "AttackAngle",
            PropertyFloat::AttackTimestamp => "AttackTimestamp",
            PropertyFloat::CheckpointTimestamp => "CheckpointTimestamp",
            PropertyFloat::SoldTimestamp => "SoldTimestamp",
            PropertyFloat::UseTimestamp => "UseTimestamp",
            PropertyFloat::UseLockTimestamp => "UseLockTimestamp",
            PropertyFloat::HealkitMod => "HealkitMod",
            PropertyFloat::FrozenTimestamp => "FrozenTimestamp",
            PropertyFloat::HealthRateMod => "HealthRateMod",
            PropertyFloat::AllegianceSwearTimestamp => "AllegianceSwearTimestamp",
            PropertyFloat::ObviousRadarRange => "ObviousRadarRange",
            PropertyFloat::HotspotCycleTime => "HotspotCycleTime",
            PropertyFloat::HotspotCycleTimeVariance => "HotspotCycleTimeVariance",
            PropertyFloat::SpamTimestamp => "SpamTimestamp",
            PropertyFloat::SpamRate => "SpamRate",
            PropertyFloat::BondWieldedTreasure => "BondWieldedTreasure",
            PropertyFloat::BulkMod => "BulkMod",
            PropertyFloat::SizeMod => "SizeMod",
            PropertyFloat::GagTimestamp => "GagTimestamp",
            PropertyFloat::GeneratorUpdateTimestamp => "GeneratorUpdateTimestamp",
            PropertyFloat::DeathSpamTimestamp => "DeathSpamTimestamp",
            PropertyFloat::DeathSpamRate => "DeathSpamRate",
            PropertyFloat::WildAttackProbability => "WildAttackProbability",
            PropertyFloat::FocusedProbability => "FocusedProbability",
            PropertyFloat::CrashAndTurnProbability => "CrashAndTurnProbability",
            PropertyFloat::CrashAndTurnRadius => "CrashAndTurnRadius",
            PropertyFloat::CrashAndTurnBias => "CrashAndTurnBias",
            PropertyFloat::GeneratorInitialDelay => "GeneratorInitialDelay",
            PropertyFloat::AiAcquireHealth => "AiAcquireHealth",
            PropertyFloat::AiAcquireStamina => "AiAcquireStamina",
            PropertyFloat::AiAcquireMana => "AiAcquireMana",
            PropertyFloat::ResistHealthDrain => "ResistHealthDrain",
            PropertyFloat::LifestoneProtectionTimestamp => "LifestoneProtectionTimestamp",
            PropertyFloat::AiCounteractEnchantment => "AiCounteractEnchantment",
            PropertyFloat::AiDispelEnchantment => "AiDispelEnchantment",
            PropertyFloat::TradeTimestamp => "TradeTimestamp",
            PropertyFloat::AiTargetedDetectionRadius => "AiTargetedDetectionRadius",
            PropertyFloat::EmotePriority => "EmotePriority",
            PropertyFloat::LastTeleportStartTimestamp => "LastTeleportStartTimestamp",
            PropertyFloat::EventSpamTimestamp => "EventSpamTimestamp",
            PropertyFloat::EventSpamRate => "EventSpamRate",
            PropertyFloat::InventoryOffset => "InventoryOffset",
            PropertyFloat::CriticalMultiplier => "CriticalMultiplier",
            PropertyFloat::ManaStoneDestroyChance => "ManaStoneDestroyChance",
            PropertyFloat::SlayerDamageBonus => "SlayerDamageBonus",
            PropertyFloat::AllegianceInfoSpamTimestamp => "AllegianceInfoSpamTimestamp",
            PropertyFloat::AllegianceInfoSpamRate => "AllegianceInfoSpamRate",
            PropertyFloat::NextSpellcastTimestamp => "NextSpellcastTimestamp",
            PropertyFloat::AppraisalRequestedTimestamp => "AppraisalRequestedTimestamp",
            PropertyFloat::AppraisalHeartbeatDueTimestamp => "AppraisalHeartbeatDueTimestamp",
            PropertyFloat::ManaConversionMod => "ManaConversionMod",
            PropertyFloat::LastPkAttackTimestamp => "LastPkAttackTimestamp",
            PropertyFloat::FellowshipUpdateTimestamp => "FellowshipUpdateTimestamp",
            PropertyFloat::CriticalFrequency => "CriticalFrequency",
            PropertyFloat::LimboStartTimestamp => "LimboStartTimestamp",
            PropertyFloat::WeaponMissileDefense => "WeaponMissileDefense",
            PropertyFloat::WeaponMagicDefense => "WeaponMagicDefense",
            PropertyFloat::IgnoreShield => "IgnoreShield",
            PropertyFloat::ElementalDamageMod => "ElementalDamageMod",
            PropertyFloat::StartMissileAttackTimestamp => "StartMissileAttackTimestamp",
            PropertyFloat::LastRareUsedTimestamp => "LastRareUsedTimestamp",
            PropertyFloat::IgnoreArmor => "IgnoreArmor",
            PropertyFloat::ProcSpellRate => "ProcSpellRate",
            PropertyFloat::ResistanceModifier => "ResistanceModifier",
            PropertyFloat::AllegianceGagTimestamp => "AllegianceGagTimestamp",
            PropertyFloat::AbsorbMagicDamage => "AbsorbMagicDamage",
            PropertyFloat::CachedMaxAbsorbMagicDamage => "CachedMaxAbsorbMagicDamage",
            PropertyFloat::GagDuration => "GagDuration",
            PropertyFloat::AllegianceGagDuration => "AllegianceGagDuration",
            PropertyFloat::GlobalXpMod => "GlobalXpMod",
            PropertyFloat::HealingModifier => "HealingModifier",
            PropertyFloat::ArmorModVsNether => "ArmorModVsNether",
            PropertyFloat::ResistNether => "ResistNether",
            PropertyFloat::CooldownDuration => "CooldownDuration",
            PropertyFloat::WeaponAuraOffense => "WeaponAuraOffense",
            PropertyFloat::WeaponAuraDefense => "WeaponAuraDefense",
            PropertyFloat::WeaponAuraElemental => "WeaponAuraElemental",
            PropertyFloat::WeaponAuraManaConv => "WeaponAuraManaConv",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    /// Chat channels
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Channel: u32 {
        const UNDEF = 0x0;
        const ABUSE = 0x1;
        const ADMIN = 0x2;
        const AUDIT = 0x4;
        const ADVOCATE1 = 0x8;
        const ADVOCATE2 = 0x10;
        const ADVOCATE3 = 0x20;
        const QA1 = 0x40;
        const QA2 = 0x80;
        const DEBUG = 0x100;
        const SENTINEL = 0x200;
        const HELP = 0x400;
        const ALL_BROADCAST = 0x401;
        const FELLOW = 0x800;
        const VASSALS = 0x1000;
        const PATRON = 0x2000;
        const MONARCH = 0x4000;
        const AL_ARQAS = 0x8000;
        const HOLTBURG = 0x10000;
        const LYTELTHORPE = 0x20000;
        const NANTO = 0x40000;
        const RITHWIC = 0x80000;
        const SAMSUR = 0x100000;
        const SHOUSHI = 0x200000;
        const YANSHI = 0x400000;
        const YARAQ = 0x800000;
        const TOWN_CHANS = 0xFF8000;
        const CO_VASSALS = 0x1000000;
        const ALLEGIANCE_BROADCAST = 0x2000000;
        const FELLOW_BROADCAST = 0x4000000;
        const SOCIETY_CEL_HAN_BROADCAST = 0x8000000;
        const SOCIETY_ELD_WEB_BROADCAST = 0x10000000;
        const SOCIETY_RAD_BLO_BROADCAST = 0x20000000;
        const OLTHOI = 0x40000000;
        const GHOST_CHANS = 0x7F007800;
    }
}

impl crate::readers::ACDataType for Channel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(Channel::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for Channel {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

/// Equipment Set Ids
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum EquipmentSet {
    None = 0x0,
    Test = 0x1,
    Test2 = 0x2,
    Unknown3 = 0x3,
    CarraidasBenediction = 0x4,
    NobleRelic = 0x5,
    AncientRelic = 0x6,
    AlduressaRelic = 0x7,
    Ninja = 0x8,
    EmpyreanRings = 0x9,
    ArmMindHeart = 0xA,
    ArmorPerfectLight = 0xB,
    ArmorPerfectLight2 = 0xC,
    Soldiers = 0xD,
    Adepts = 0xE,
    Archers = 0xF,
    Defenders = 0x10,
    Tinkers = 0x11,
    Crafters = 0x12,
    Hearty = 0x13,
    Dexterous = 0x14,
    Wise = 0x15,
    Swift = 0x16,
    Hardened = 0x17,
    Reinforced = 0x18,
    Interlocking = 0x19,
    Flameproof = 0x1A,
    Acidproof = 0x1B,
    Coldproof = 0x1C,
    Lightningproof = 0x1D,
    SocietyArmor = 0x1E,
    ColosseumClothing = 0x1F,
    GraveyardClothing = 0x20,
    OlthoiClothing = 0x21,
    NoobieArmor = 0x22,
    AetheriaDefense = 0x23,
    AetheriaDestruction = 0x24,
    AetheriaFury = 0x25,
    AetheriaGrowth = 0x26,
    AetheriaVigor = 0x27,
    RareDamageResistance = 0x28,
    RareDamageBoost = 0x29,
    OlthoiArmorDRed = 0x2A,
    OlthoiArmorCRat = 0x2B,
    OlthoiArmorCRed = 0x2C,
    OlthoiArmorDRat = 0x2D,
    AlduressaRelicUpgrade = 0x2E,
    AncientRelicUpgrade = 0x2F,
    NobleRelicUpgrade = 0x30,
    CloakAlchemy = 0x31,
    CloakArcaneLore = 0x32,
    CloakArmorTinkering = 0x33,
    CloakAssessPerson = 0x34,
    CloakLightWeapons = 0x35,
    CloakMissileWeapons = 0x36,
    CloakCooking = 0x37,
    CloakCreatureEnchantment = 0x38,
    CloakCrossbow = 0x39,
    CloakFinesseWeapons = 0x3A,
    CloakDeception = 0x3B,
    CloakFletching = 0x3C,
    CloakHealing = 0x3D,
    CloakItemEnchantment = 0x3E,
    CloakItemTinkering = 0x3F,
    CloakLeadership = 0x40,
    CloakLifeMagic = 0x41,
    CloakLoyalty = 0x42,
    CloakMace = 0x43,
    CloakMagicDefense = 0x44,
    CloakMagicItemTinkering = 0x45,
    CloakManaConversion = 0x46,
    CloakMeleeDefense = 0x47,
    CloakMissileDefense = 0x48,
    CloakSalvaging = 0x49,
    CloakSpear = 0x4A,
    CloakStaff = 0x4B,
    CloakHeavyWeapons = 0x4C,
    CloakThrownWeapon = 0x4D,
    CloakTwoHandedCombat = 0x4E,
    CloakUnarmedCombat = 0x4F,
    CloakVoidMagic = 0x50,
    CloakWarMagic = 0x51,
    CloakWeaponTinkering = 0x52,
    CloakAssessCreature = 0x53,
    CloakDirtyFighting = 0x54,
    CloakDualWield = 0x55,
    CloakRecklessness = 0x56,
    CloakShield = 0x57,
    CloakSneakAttack = 0x58,
    #[serde(rename = "Ninja_New")]
    NinjaNew = 0x59,
    CloakSummoning = 0x5A,
    ShroudedSoul = 0x5B,
    DarkenedMind = 0x5C,
    CloudedSpirit = 0x5D,
    MinorStingingShroudedSoul = 0x5E,
    MinorSparkingShroudedSoul = 0x5F,
    MinorSmolderingShroudedSoul = 0x60,
    MinorShiveringShroudedSoul = 0x61,
    MinorStingingDarkenedMind = 0x62,
    MinorSparkingDarkenedMind = 0x63,
    MinorSmolderingDarkenedMind = 0x64,
    MinorShiveringDarkenedMind = 0x65,
    MinorStingingCloudedSpirit = 0x66,
    MinorSparkingCloudedSpirit = 0x67,
    MinorSmolderingCloudedSpirit = 0x68,
    MinorShiveringCloudedSpirit = 0x69,
    MajorStingingShroudedSoul = 0x6A,
    MajorSparkingShroudedSoul = 0x6B,
    MajorSmolderingShroudedSoul = 0x6C,
    MajorShiveringShroudedSoul = 0x6D,
    MajorStingingDarkenedMind = 0x6E,
    MajorSparkingDarkenedMind = 0x6F,
    MajorSmolderingDarkenedMind = 0x70,
    MajorShiveringDarkenedMind = 0x71,
    MajorStingingCloudedSpirit = 0x72,
    MajorSparkingCloudedSpirit = 0x73,
    MajorSmolderingCloudedSpirit = 0x74,
    MajorShiveringCloudedSpirit = 0x75,
    BlackfireStingingShroudedSoul = 0x76,
    BlackfireSparkingShroudedSoul = 0x77,
    BlackfireSmolderingShroudedSoul = 0x78,
    BlackfireShiveringShroudedSoul = 0x79,
    BlackfireStingingDarkenedMind = 0x7A,
    BlackfireSparkingDarkenedMind = 0x7B,
    BlackfireSmolderingDarkenedMind = 0x7C,
    BlackfireShiveringDarkenedMind = 0x7D,
    BlackfireStingingCloudedSpirit = 0x7E,
    BlackfireSparkingCloudedSpirit = 0x7F,
    BlackfireSmolderingCloudedSpirit = 0x80,
    BlackfireShiveringCloudedSpirit = 0x81,
    ShimmeringShadowsSet = 0x82,
    BrownSocietyLocket = 0x83,
    YellowSocietyLocket = 0x84,
    RedSocietyBand = 0x85,
    GreenSocietyBand = 0x86,
    PurpleSocietyBand = 0x87,
    BlueSocietyBand = 0x88,
    GauntletGarb = 0x89,
    ParagonMissile = 0x8A,
    ParagonCaster = 0x8B,
    ParagonMelee = 0x8C,
}

impl crate::readers::ACDataType for EquipmentSet {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EquipmentSet::try_from(value)?)
    }
}

impl crate::writers::ACWritable for EquipmentSet {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for EquipmentSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EquipmentSet::None => "None",
            EquipmentSet::Test => "Test",
            EquipmentSet::Test2 => "Test2",
            EquipmentSet::Unknown3 => "Unknown3",
            EquipmentSet::CarraidasBenediction => "CarraidasBenediction",
            EquipmentSet::NobleRelic => "NobleRelic",
            EquipmentSet::AncientRelic => "AncientRelic",
            EquipmentSet::AlduressaRelic => "AlduressaRelic",
            EquipmentSet::Ninja => "Ninja",
            EquipmentSet::EmpyreanRings => "EmpyreanRings",
            EquipmentSet::ArmMindHeart => "ArmMindHeart",
            EquipmentSet::ArmorPerfectLight => "ArmorPerfectLight",
            EquipmentSet::ArmorPerfectLight2 => "ArmorPerfectLight2",
            EquipmentSet::Soldiers => "Soldiers",
            EquipmentSet::Adepts => "Adepts",
            EquipmentSet::Archers => "Archers",
            EquipmentSet::Defenders => "Defenders",
            EquipmentSet::Tinkers => "Tinkers",
            EquipmentSet::Crafters => "Crafters",
            EquipmentSet::Hearty => "Hearty",
            EquipmentSet::Dexterous => "Dexterous",
            EquipmentSet::Wise => "Wise",
            EquipmentSet::Swift => "Swift",
            EquipmentSet::Hardened => "Hardened",
            EquipmentSet::Reinforced => "Reinforced",
            EquipmentSet::Interlocking => "Interlocking",
            EquipmentSet::Flameproof => "Flameproof",
            EquipmentSet::Acidproof => "Acidproof",
            EquipmentSet::Coldproof => "Coldproof",
            EquipmentSet::Lightningproof => "Lightningproof",
            EquipmentSet::SocietyArmor => "SocietyArmor",
            EquipmentSet::ColosseumClothing => "ColosseumClothing",
            EquipmentSet::GraveyardClothing => "GraveyardClothing",
            EquipmentSet::OlthoiClothing => "OlthoiClothing",
            EquipmentSet::NoobieArmor => "NoobieArmor",
            EquipmentSet::AetheriaDefense => "AetheriaDefense",
            EquipmentSet::AetheriaDestruction => "AetheriaDestruction",
            EquipmentSet::AetheriaFury => "AetheriaFury",
            EquipmentSet::AetheriaGrowth => "AetheriaGrowth",
            EquipmentSet::AetheriaVigor => "AetheriaVigor",
            EquipmentSet::RareDamageResistance => "RareDamageResistance",
            EquipmentSet::RareDamageBoost => "RareDamageBoost",
            EquipmentSet::OlthoiArmorDRed => "OlthoiArmorDRed",
            EquipmentSet::OlthoiArmorCRat => "OlthoiArmorCRat",
            EquipmentSet::OlthoiArmorCRed => "OlthoiArmorCRed",
            EquipmentSet::OlthoiArmorDRat => "OlthoiArmorDRat",
            EquipmentSet::AlduressaRelicUpgrade => "AlduressaRelicUpgrade",
            EquipmentSet::AncientRelicUpgrade => "AncientRelicUpgrade",
            EquipmentSet::NobleRelicUpgrade => "NobleRelicUpgrade",
            EquipmentSet::CloakAlchemy => "CloakAlchemy",
            EquipmentSet::CloakArcaneLore => "CloakArcaneLore",
            EquipmentSet::CloakArmorTinkering => "CloakArmorTinkering",
            EquipmentSet::CloakAssessPerson => "CloakAssessPerson",
            EquipmentSet::CloakLightWeapons => "CloakLightWeapons",
            EquipmentSet::CloakMissileWeapons => "CloakMissileWeapons",
            EquipmentSet::CloakCooking => "CloakCooking",
            EquipmentSet::CloakCreatureEnchantment => "CloakCreatureEnchantment",
            EquipmentSet::CloakCrossbow => "CloakCrossbow",
            EquipmentSet::CloakFinesseWeapons => "CloakFinesseWeapons",
            EquipmentSet::CloakDeception => "CloakDeception",
            EquipmentSet::CloakFletching => "CloakFletching",
            EquipmentSet::CloakHealing => "CloakHealing",
            EquipmentSet::CloakItemEnchantment => "CloakItemEnchantment",
            EquipmentSet::CloakItemTinkering => "CloakItemTinkering",
            EquipmentSet::CloakLeadership => "CloakLeadership",
            EquipmentSet::CloakLifeMagic => "CloakLifeMagic",
            EquipmentSet::CloakLoyalty => "CloakLoyalty",
            EquipmentSet::CloakMace => "CloakMace",
            EquipmentSet::CloakMagicDefense => "CloakMagicDefense",
            EquipmentSet::CloakMagicItemTinkering => "CloakMagicItemTinkering",
            EquipmentSet::CloakManaConversion => "CloakManaConversion",
            EquipmentSet::CloakMeleeDefense => "CloakMeleeDefense",
            EquipmentSet::CloakMissileDefense => "CloakMissileDefense",
            EquipmentSet::CloakSalvaging => "CloakSalvaging",
            EquipmentSet::CloakSpear => "CloakSpear",
            EquipmentSet::CloakStaff => "CloakStaff",
            EquipmentSet::CloakHeavyWeapons => "CloakHeavyWeapons",
            EquipmentSet::CloakThrownWeapon => "CloakThrownWeapon",
            EquipmentSet::CloakTwoHandedCombat => "CloakTwoHandedCombat",
            EquipmentSet::CloakUnarmedCombat => "CloakUnarmedCombat",
            EquipmentSet::CloakVoidMagic => "CloakVoidMagic",
            EquipmentSet::CloakWarMagic => "CloakWarMagic",
            EquipmentSet::CloakWeaponTinkering => "CloakWeaponTinkering",
            EquipmentSet::CloakAssessCreature => "CloakAssessCreature",
            EquipmentSet::CloakDirtyFighting => "CloakDirtyFighting",
            EquipmentSet::CloakDualWield => "CloakDualWield",
            EquipmentSet::CloakRecklessness => "CloakRecklessness",
            EquipmentSet::CloakShield => "CloakShield",
            EquipmentSet::CloakSneakAttack => "CloakSneakAttack",
            EquipmentSet::NinjaNew => "Ninja_New",
            EquipmentSet::CloakSummoning => "CloakSummoning",
            EquipmentSet::ShroudedSoul => "ShroudedSoul",
            EquipmentSet::DarkenedMind => "DarkenedMind",
            EquipmentSet::CloudedSpirit => "CloudedSpirit",
            EquipmentSet::MinorStingingShroudedSoul => "MinorStingingShroudedSoul",
            EquipmentSet::MinorSparkingShroudedSoul => "MinorSparkingShroudedSoul",
            EquipmentSet::MinorSmolderingShroudedSoul => "MinorSmolderingShroudedSoul",
            EquipmentSet::MinorShiveringShroudedSoul => "MinorShiveringShroudedSoul",
            EquipmentSet::MinorStingingDarkenedMind => "MinorStingingDarkenedMind",
            EquipmentSet::MinorSparkingDarkenedMind => "MinorSparkingDarkenedMind",
            EquipmentSet::MinorSmolderingDarkenedMind => "MinorSmolderingDarkenedMind",
            EquipmentSet::MinorShiveringDarkenedMind => "MinorShiveringDarkenedMind",
            EquipmentSet::MinorStingingCloudedSpirit => "MinorStingingCloudedSpirit",
            EquipmentSet::MinorSparkingCloudedSpirit => "MinorSparkingCloudedSpirit",
            EquipmentSet::MinorSmolderingCloudedSpirit => "MinorSmolderingCloudedSpirit",
            EquipmentSet::MinorShiveringCloudedSpirit => "MinorShiveringCloudedSpirit",
            EquipmentSet::MajorStingingShroudedSoul => "MajorStingingShroudedSoul",
            EquipmentSet::MajorSparkingShroudedSoul => "MajorSparkingShroudedSoul",
            EquipmentSet::MajorSmolderingShroudedSoul => "MajorSmolderingShroudedSoul",
            EquipmentSet::MajorShiveringShroudedSoul => "MajorShiveringShroudedSoul",
            EquipmentSet::MajorStingingDarkenedMind => "MajorStingingDarkenedMind",
            EquipmentSet::MajorSparkingDarkenedMind => "MajorSparkingDarkenedMind",
            EquipmentSet::MajorSmolderingDarkenedMind => "MajorSmolderingDarkenedMind",
            EquipmentSet::MajorShiveringDarkenedMind => "MajorShiveringDarkenedMind",
            EquipmentSet::MajorStingingCloudedSpirit => "MajorStingingCloudedSpirit",
            EquipmentSet::MajorSparkingCloudedSpirit => "MajorSparkingCloudedSpirit",
            EquipmentSet::MajorSmolderingCloudedSpirit => "MajorSmolderingCloudedSpirit",
            EquipmentSet::MajorShiveringCloudedSpirit => "MajorShiveringCloudedSpirit",
            EquipmentSet::BlackfireStingingShroudedSoul => "BlackfireStingingShroudedSoul",
            EquipmentSet::BlackfireSparkingShroudedSoul => "BlackfireSparkingShroudedSoul",
            EquipmentSet::BlackfireSmolderingShroudedSoul => "BlackfireSmolderingShroudedSoul",
            EquipmentSet::BlackfireShiveringShroudedSoul => "BlackfireShiveringShroudedSoul",
            EquipmentSet::BlackfireStingingDarkenedMind => "BlackfireStingingDarkenedMind",
            EquipmentSet::BlackfireSparkingDarkenedMind => "BlackfireSparkingDarkenedMind",
            EquipmentSet::BlackfireSmolderingDarkenedMind => "BlackfireSmolderingDarkenedMind",
            EquipmentSet::BlackfireShiveringDarkenedMind => "BlackfireShiveringDarkenedMind",
            EquipmentSet::BlackfireStingingCloudedSpirit => "BlackfireStingingCloudedSpirit",
            EquipmentSet::BlackfireSparkingCloudedSpirit => "BlackfireSparkingCloudedSpirit",
            EquipmentSet::BlackfireSmolderingCloudedSpirit => "BlackfireSmolderingCloudedSpirit",
            EquipmentSet::BlackfireShiveringCloudedSpirit => "BlackfireShiveringCloudedSpirit",
            EquipmentSet::ShimmeringShadowsSet => "ShimmeringShadowsSet",
            EquipmentSet::BrownSocietyLocket => "BrownSocietyLocket",
            EquipmentSet::YellowSocietyLocket => "YellowSocietyLocket",
            EquipmentSet::RedSocietyBand => "RedSocietyBand",
            EquipmentSet::GreenSocietyBand => "GreenSocietyBand",
            EquipmentSet::PurpleSocietyBand => "PurpleSocietyBand",
            EquipmentSet::BlueSocietyBand => "BlueSocietyBand",
            EquipmentSet::GauntletGarb => "GauntletGarb",
            EquipmentSet::ParagonMissile => "ParagonMissile",
            EquipmentSet::ParagonCaster => "ParagonCaster",
            EquipmentSet::ParagonMelee => "ParagonMelee",
        };
        write!(f, "{}", s)
    }
}

/// Radar Color
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum RadarColor {
    Default = 0x0,
    Blue = 0x1,
    Gold = 0x2,
    White = 0x3,
    Purple = 0x4,
    Red = 0x5,
    Pink = 0x6,
    Green = 0x7,
    Yellow = 0x8,
    Cyan = 0x9,
    BrightGreen = 0x10,
}

impl crate::readers::ACDataType for RadarColor {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(RadarColor::try_from(value)?)
    }
}

impl crate::writers::ACWritable for RadarColor {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u8(writer, self.clone() as u8)?;
        Ok(())
    }
}

impl std::fmt::Display for RadarColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RadarColor::Default => "Default",
            RadarColor::Blue => "Blue",
            RadarColor::Gold => "Gold",
            RadarColor::White => "White",
            RadarColor::Purple => "Purple",
            RadarColor::Red => "Red",
            RadarColor::Pink => "Pink",
            RadarColor::Green => "Green",
            RadarColor::Yellow => "Yellow",
            RadarColor::Cyan => "Cyan",
            RadarColor::BrightGreen => "BrightGreen",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    /// Flags that determine what data is contained in the EnchantmentRegistry
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EnchantmentRegistryFlags: u32 {
        const LIFE_SPELLS = 0x1;
        const CREATURE_SPELLS = 0x2;
        const VITAE = 0x4;
        const COOLDOWNS = 0x8;
    }
}

impl crate::readers::ACDataType for EnchantmentRegistryFlags {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EnchantmentRegistryFlags::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for EnchantmentRegistryFlags {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum SpellCategory {
    Undef = 0x0,
    StrengthRaising = 0x1,
    StrengthLowering = 0x2,
    EnduranceRaising = 0x3,
    EnduranceLowering = 0x4,
    QuicknessRaising = 0x5,
    QuicknessLowering = 0x6,
    CoordinationRaising = 0x7,
    CoordinationLowering = 0x8,
    FocusRaising = 0x9,
    FocusLowering = 0xA,
    SelfRaising = 0xB,
    SelfLowering = 0xC,
    FocusConcentration = 0xD,
    FocusDisruption = 0xE,
    FocusBrilliance = 0xF,
    FocusDullness = 0x10,
    AxeRaising = 0x11,
    AxeLowering = 0x12,
    BowRaising = 0x13,
    BowLowering = 0x14,
    CrossbowRaising = 0x15,
    CrossbowLowering = 0x16,
    DaggerRaising = 0x17,
    DaggerLowering = 0x18,
    MaceRaising = 0x19,
    MaceLowering = 0x1A,
    SpearRaising = 0x1B,
    SpearLowering = 0x1C,
    StaffRaising = 0x1D,
    StaffLowering = 0x1E,
    SwordRaising = 0x1F,
    SwordLowering = 0x20,
    ThrownWeaponsRaising = 0x21,
    ThrownWeaponsLowering = 0x22,
    UnarmedCombatRaising = 0x23,
    UnarmedCombatLowering = 0x24,
    MeleeDefenseRaising = 0x25,
    MeleeDefenseLowering = 0x26,
    MissileDefenseRaising = 0x27,
    MissileDefenseLowering = 0x28,
    MagicDefenseRaising = 0x29,
    MagicDefenseLowering = 0x2A,
    CreatureEnchantmentRaising = 0x2B,
    CreatureEnchantmentLowering = 0x2C,
    ItemEnchantmentRaising = 0x2D,
    ItemEnchantmentLowering = 0x2E,
    LifeMagicRaising = 0x2F,
    LifeMagicLowering = 0x30,
    WarMagicRaising = 0x31,
    WarMagicLowering = 0x32,
    ManaConversionRaising = 0x33,
    ManaConversionLowering = 0x34,
    ArcaneLoreRaising = 0x35,
    ArcaneLoreLowering = 0x36,
    AppraiseArmorRaising = 0x37,
    AppraiseArmorLowering = 0x38,
    AppraiseItemRaising = 0x39,
    AppraiseItemLowering = 0x3A,
    AppraiseMagicItemRaising = 0x3B,
    AppraiseMagicItemLowering = 0x3C,
    AppraiseWeaponRaising = 0x3D,
    AppraiseWeaponLowering = 0x3E,
    AssessMonsterRaising = 0x3F,
    AssessMonsterLowering = 0x40,
    DeceptionRaising = 0x41,
    DeceptionLowering = 0x42,
    HealingRaising = 0x43,
    HealingLowering = 0x44,
    JumpRaising = 0x45,
    JumpLowering = 0x46,
    LeadershipRaising = 0x47,
    LeadershipLowering = 0x48,
    LockpickRaising = 0x49,
    LockpickLowering = 0x4A,
    LoyaltyRaising = 0x4B,
    LoyaltyLowering = 0x4C,
    RunRaising = 0x4D,
    RunLowering = 0x4E,
    HealthRaising = 0x4F,
    HealthLowering = 0x50,
    StaminaRaising = 0x51,
    StaminaLowering = 0x52,
    ManaRaising = 0x53,
    ManaLowering = 0x54,
    ManaRemedy = 0x55,
    ManaMalediction = 0x56,
    HealthTransfertocaster = 0x57,
    HealthTransferfromcaster = 0x58,
    StaminaTransfertocaster = 0x59,
    StaminaTransferfromcaster = 0x5A,
    ManaTransfertocaster = 0x5B,
    ManaTransferfromcaster = 0x5C,
    HealthAccelerating = 0x5D,
    HealthDecelerating = 0x5E,
    StaminaAccelerating = 0x5F,
    StaminaDecelerating = 0x60,
    ManaAccelerating = 0x61,
    ManaDecelerating = 0x62,
    VitaeRaising = 0x63,
    VitaeLowering = 0x64,
    AcidProtection = 0x65,
    AcidVulnerability = 0x66,
    BludgeonProtection = 0x67,
    BludgeonVulnerability = 0x68,
    ColdProtection = 0x69,
    ColdVulnerability = 0x6A,
    ElectricProtection = 0x6B,
    ElectricVulnerability = 0x6C,
    FireProtection = 0x6D,
    FireVulnerability = 0x6E,
    PierceProtection = 0x6F,
    PierceVulnerability = 0x70,
    SlashProtection = 0x71,
    SlashVulnerability = 0x72,
    ArmorRaising = 0x73,
    ArmorLowering = 0x74,
    AcidMissile = 0x75,
    BludgeoningMissile = 0x76,
    ColdMissile = 0x77,
    ElectricMissile = 0x78,
    FireMissile = 0x79,
    PiercingMissile = 0x7A,
    SlashingMissile = 0x7B,
    AcidSeeker = 0x7C,
    BludgeoningSeeker = 0x7D,
    ColdSeeker = 0x7E,
    ElectricSeeker = 0x7F,
    FireSeeker = 0x80,
    PiercingSeeker = 0x81,
    SlashingSeeker = 0x82,
    AcidBurst = 0x83,
    BludgeoningBurst = 0x84,
    ColdBurst = 0x85,
    ElectricBurst = 0x86,
    FireBurst = 0x87,
    PiercingBurst = 0x88,
    SlashingBurst = 0x89,
    AcidBlast = 0x8A,
    BludgeoningBlast = 0x8B,
    ColdBlast = 0x8C,
    ElectricBlast = 0x8D,
    FireBlast = 0x8E,
    PiercingBlast = 0x8F,
    SlashingBlast = 0x90,
    AcidScatter = 0x91,
    BludgeoningScatter = 0x92,
    ColdScatter = 0x93,
    ElectricScatter = 0x94,
    FireScatter = 0x95,
    PiercingScatter = 0x96,
    SlashingScatter = 0x97,
    AttackModRaising = 0x98,
    AttackModLowering = 0x99,
    DamageRaising = 0x9A,
    DamageLowering = 0x9B,
    DefenseModRaising = 0x9C,
    DefenseModLowering = 0x9D,
    WeaponTimeRaising = 0x9E,
    WeaponTimeLowering = 0x9F,
    ArmorValueRaising = 0xA0,
    ArmorValueLowering = 0xA1,
    AcidResistanceRaising = 0xA2,
    AcidResistanceLowering = 0xA3,
    BludgeonResistanceRaising = 0xA4,
    BludgeonResistanceLowering = 0xA5,
    ColdResistanceRaising = 0xA6,
    ColdResistanceLowering = 0xA7,
    ElectricResistanceRaising = 0xA8,
    ElectricResistanceLowering = 0xA9,
    FireResistanceRaising = 0xAA,
    FireResistanceLowering = 0xAB,
    PierceResistanceRaising = 0xAC,
    PierceResistanceLowering = 0xAD,
    SlashResistanceRaising = 0xAE,
    SlashResistanceLowering = 0xAF,
    BludgeoningResistanceRaising = 0xB0,
    BludgeoningResistanceLowering = 0xB1,
    SlashingResistanceRaising = 0xB2,
    SlashingResistanceLowering = 0xB3,
    PiercingResistanceRaising = 0xB4,
    PiercingResistanceLowering = 0xB5,
    ElectricalResistanceRaising = 0xB6,
    ElectricalResistanceLowering = 0xB7,
    FrostResistanceRaising = 0xB8,
    FrostResistanceLowering = 0xB9,
    FlameResistanceRaising = 0xBA,
    FlameResistanceLowering = 0xBB,
    AcidicResistanceRaising = 0xBC,
    AcidicResistanceLowering = 0xBD,
    ArmorLevelRaising = 0xBE,
    ArmorLevelLowering = 0xBF,
    LockpickResistanceRaising = 0xC0,
    LockpickResistanceLowering = 0xC1,
    ManaConversionModLowering = 0xC2,
    ManaConversionModRaising = 0xC3,
    VisionRaising = 0xC4,
    VisionLowering = 0xC5,
    TransparencyRaising = 0xC6,
    TransparencyLowering = 0xC7,
    PortalTie = 0xC8,
    PortalRecall = 0xC9,
    PortalCreation = 0xCA,
    PortalItemCreation = 0xCB,
    Vitae = 0xCC,
    AssessPersonRaising = 0xCD,
    AssessPersonLowering = 0xCE,
    AcidVolley = 0xCF,
    BludgeoningVolley = 0xD0,
    FrostVolley = 0xD1,
    LightningVolley = 0xD2,
    FlameVolley = 0xD3,
    ForceVolley = 0xD4,
    BladeVolley = 0xD5,
    PortalSending = 0xD6,
    LifestoneSending = 0xD7,
    CookingRaising = 0xD8,
    CookingLowering = 0xD9,
    FletchingRaising = 0xDA,
    FletchingLowering = 0xDB,
    AlchemyLowering = 0xDC,
    AlchemyRaising = 0xDD,
    AcidRing = 0xDE,
    BludgeoningRing = 0xDF,
    ColdRing = 0xE0,
    ElectricRing = 0xE1,
    FireRing = 0xE2,
    PiercingRing = 0xE3,
    SlashingRing = 0xE4,
    AcidWall = 0xE5,
    BludgeoningWall = 0xE6,
    ColdWall = 0xE7,
    ElectricWall = 0xE8,
    FireWall = 0xE9,
    PiercingWall = 0xEA,
    SlashingWall = 0xEB,
    AcidStrike = 0xEC,
    BludgeoningStrike = 0xED,
    ColdStrike = 0xEE,
    ElectricStrike = 0xEF,
    FireStrike = 0xF0,
    PiercingStrike = 0xF1,
    SlashingStrike = 0xF2,
    AcidStreak = 0xF3,
    BludgeoningStreak = 0xF4,
    ColdStreak = 0xF5,
    ElectricStreak = 0xF6,
    FireStreak = 0xF7,
    PiercingStreak = 0xF8,
    SlashingStreak = 0xF9,
    Dispel = 0xFA,
    CreatureMysticRaising = 0xFB,
    CreatureMysticLowering = 0xFC,
    ItemMysticRaising = 0xFD,
    ItemMysticLowering = 0xFE,
    WarMysticRaising = 0xFF,
    WarMysticLowering = 0x100,
    HealthRestoring = 0x101,
    HealthDepleting = 0x102,
    ManaRestoring = 0x103,
    ManaDepleting = 0x104,
    StrengthIncrease = 0x105,
    StrengthDecrease = 0x106,
    EnduranceIncrease = 0x107,
    EnduranceDecrease = 0x108,
    QuicknessIncrease = 0x109,
    QuicknessDecrease = 0x10A,
    CoordinationIncrease = 0x10B,
    CoordinationDecrease = 0x10C,
    FocusIncrease = 0x10D,
    FocusDecrease = 0x10E,
    SelfIncrease = 0x10F,
    SelfDecrease = 0x110,
    GreatVitalityRaising = 0x111,
    PoorVitalityLowering = 0x112,
    GreatVigorRaising = 0x113,
    PoorVigorLowering = 0x114,
    GreaterIntellectRaising = 0x115,
    LessorIntellectLowering = 0x116,
    LifeGiverRaising = 0x117,
    LifeTakerLowering = 0x118,
    StaminaGiverRaising = 0x119,
    StaminaTakerLowering = 0x11A,
    ManaGiverRaising = 0x11B,
    ManaTakerLowering = 0x11C,
    AcidWardProtection = 0x11D,
    AcidWardVulnerability = 0x11E,
    FireWardProtection = 0x11F,
    FireWardVulnerability = 0x120,
    ColdWardProtection = 0x121,
    ColdWardVulnerability = 0x122,
    ElectricWardProtection = 0x123,
    ElectricWardVulnerability = 0x124,
    LeadershipObedienceRaising = 0x125,
    LeadershipObedienceLowering = 0x126,
    MeleeDefenseShelterRaising = 0x127,
    MeleeDefenseShelterLowering = 0x128,
    MissileDefenseShelterRaising = 0x129,
    MissileDefenseShelterLowering = 0x12A,
    MagicDefenseShelterRaising = 0x12B,
    MagicDefenseShelterLowering = 0x12C,
    HuntersAcumenRaising = 0x12D,
    HuntersAcumenLowering = 0x12E,
    StillWaterRaising = 0x12F,
    StillWaterLowering = 0x130,
    StrengthofEarthRaising = 0x131,
    StrengthofEarthLowering = 0x132,
    TorrentRaising = 0x133,
    TorrentLowering = 0x134,
    GrowthRaising = 0x135,
    GrowthLowering = 0x136,
    CascadeAxeRaising = 0x137,
    CascadeAxeLowering = 0x138,
    CascadeDaggerRaising = 0x139,
    CascadeDaggerLowering = 0x13A,
    CascadeMaceRaising = 0x13B,
    CascadeMaceLowering = 0x13C,
    CascadeSpearRaising = 0x13D,
    CascadeSpearLowering = 0x13E,
    CascadeStaffRaising = 0x13F,
    CascadeStaffLowering = 0x140,
    StoneCliffsRaising = 0x141,
    StoneCliffsLowering = 0x142,
    MaxDamageRaising = 0x143,
    MaxDamageLowering = 0x144,
    BowDamageRaising = 0x145,
    BowDamageLowering = 0x146,
    BowRangeRaising = 0x147,
    BowRangeLowering = 0x148,
    ExtraDefenseModRaising = 0x149,
    ExtraDefenseModLowering = 0x14A,
    ExtraBowSkillRaising = 0x14B,
    ExtraBowSkillLowering = 0x14C,
    ExtraAlchemySkillRaising = 0x14D,
    ExtraAlchemySkillLowering = 0x14E,
    ExtraArcaneLoreSkillRaising = 0x14F,
    ExtraArcaneLoreSkillLowering = 0x150,
    ExtraAppraiseArmorSkillRaising = 0x151,
    ExtraAppraiseArmorSkillLowering = 0x152,
    ExtraCookingSkillRaising = 0x153,
    ExtraCookingSkillLowering = 0x154,
    ExtraCrossbowSkillRaising = 0x155,
    ExtraCrossbowSkillLowering = 0x156,
    ExtraDeceptionSkillRaising = 0x157,
    ExtraDeceptionSkillLowering = 0x158,
    ExtraLoyaltySkillRaising = 0x159,
    ExtraLoyaltySkillLowering = 0x15A,
    ExtraFletchingSkillRaising = 0x15B,
    ExtraFletchingSkillLowering = 0x15C,
    ExtraHealingSkillRaising = 0x15D,
    ExtraHealingSkillLowering = 0x15E,
    ExtraMeleeDefenseSkillRaising = 0x15F,
    ExtraMeleeDefenseSkillLowering = 0x160,
    ExtraAppraiseItemSkillRaising = 0x161,
    ExtraAppraiseItemSkillLowering = 0x162,
    ExtraJumpingSkillRaising = 0x163,
    ExtraJumpingSkillLowering = 0x164,
    ExtraLifeMagicSkillRaising = 0x165,
    ExtraLifeMagicSkillLowering = 0x166,
    ExtraLockpickSkillRaising = 0x167,
    ExtraLockpickSkillLowering = 0x168,
    ExtraAppraiseMagicItemSkillRaising = 0x169,
    ExtraAppraiseMagicItemSkillLowering = 0x16A,
    ExtraManaConversionSkillRaising = 0x16B,
    ExtraManaConversionSkillLowering = 0x16C,
    ExtraAssessCreatureSkillRaising = 0x16D,
    ExtraAssessCreatureSkillLowering = 0x16E,
    ExtraAssessPersonSkillRaising = 0x16F,
    ExtraAssessPersonSkillLowering = 0x170,
    ExtraRunSkillRaising = 0x171,
    ExtraRunSkillLowering = 0x172,
    ExtraSwordSkillRaising = 0x173,
    ExtraSwordSkillLowering = 0x174,
    ExtraThrownWeaponsSkillRaising = 0x175,
    ExtraThrownWeaponsSkillLowering = 0x176,
    ExtraUnarmedCombatSkillRaising = 0x177,
    ExtraUnarmedCombatSkillLowering = 0x178,
    ExtraAppraiseWeaponSkillRaising = 0x179,
    ExtraAppraiseWeaponSkillLowering = 0x17A,
    ArmorIncrease = 0x17B,
    ArmorDecrease = 0x17C,
    ExtraAcidResistanceRaising = 0x17D,
    ExtraAcidResistanceLowering = 0x17E,
    ExtraBludgeonResistanceRaising = 0x17F,
    ExtraBludgeonResistanceLowering = 0x180,
    ExtraFireResistanceRaising = 0x181,
    ExtraFireResistanceLowering = 0x182,
    ExtraColdResistanceRaising = 0x183,
    ExtraColdResistanceLowering = 0x184,
    ExtraAttackModRaising = 0x185,
    ExtraAttackModLowering = 0x186,
    ExtraArmorValueRaising = 0x187,
    ExtraArmorValueLowering = 0x188,
    ExtraPierceResistanceRaising = 0x189,
    ExtraPierceResistanceLowering = 0x18A,
    ExtraSlashResistanceRaising = 0x18B,
    ExtraSlashResistanceLowering = 0x18C,
    ExtraElectricResistanceRaising = 0x18D,
    ExtraElectricResistanceLowering = 0x18E,
    ExtraWeaponTimeRaising = 0x18F,
    ExtraWeaponTimeLowering = 0x190,
    BludgeonWardProtection = 0x191,
    BludgeonWardVulnerability = 0x192,
    SlashWardProtection = 0x193,
    SlashWardVulnerability = 0x194,
    PierceWardProtection = 0x195,
    PierceWardVulnerability = 0x196,
    StaminaRestoring = 0x197,
    StaminaDepleting = 0x198,
    Fireworks = 0x199,
    HealthDivide = 0x19A,
    StaminaDivide = 0x19B,
    ManaDivide = 0x19C,
    CoordinationIncrease2 = 0x19D,
    StrengthIncrease2 = 0x19E,
    FocusIncrease2 = 0x19F,
    EnduranceIncrease2 = 0x1A0,
    SelfIncrease2 = 0x1A1,
    MeleeDefenseMultiply = 0x1A2,
    MissileDefenseMultiply = 0x1A3,
    MagicDefenseMultiply = 0x1A4,
    AttributesDecrease = 0x1A5,
    LifeGiverRaising2 = 0x1A6,
    ItemEnchantmentRaising2 = 0x1A7,
    SkillsDecrease = 0x1A8,
    ExtraManaConversionBonus = 0x1A9,
    WarMysticRaising2 = 0x1AA,
    WarMysticLowering2 = 0x1AB,
    MagicDefenseShelterRaising2 = 0x1AC,
    ExtraLifeMagicSkillRaising2 = 0x1AD,
    CreatureMysticRaising2 = 0x1AE,
    ItemMysticRaising2 = 0x1AF,
    ManaRaising2 = 0x1B0,
    SelfRaising2 = 0x1B1,
    CreatureEnchantmentRaising2 = 0x1B2,
    SalvagingRaising = 0x1B3,
    ExtraSalvagingRaising = 0x1B4,
    ExtraSalvagingRaising2 = 0x1B5,
    CascadeAxeRaising2 = 0x1B6,
    ExtraBowSkillRaising2 = 0x1B7,
    ExtraThrownWeaponsSkillRaising2 = 0x1B8,
    ExtraCrossbowSkillRaising2 = 0x1B9,
    CascadeDaggerRaising2 = 0x1BA,
    CascadeMaceRaising2 = 0x1BB,
    ExtraUnarmedCombatSkillRaising2 = 0x1BC,
    CascadeSpearRaising2 = 0x1BD,
    CascadeStaffRaising2 = 0x1BE,
    ExtraSwordSkillRaising2 = 0x1BF,
    AcidProtectionRare = 0x1C0,
    AcidResistanceRaisingRare = 0x1C1,
    AlchemyRaisingRare = 0x1C2,
    AppraisalResistanceLoweringRare = 0x1C3,
    AppraiseArmorRaisingRare = 0x1C4,
    AppraiseItemRaisingRare = 0x1C5,
    AppraiseMagicItemRaisingRare = 0x1C6,
    AppraiseWeaponRaisingRare = 0x1C7,
    ArcaneLoreRaisingRare = 0x1C8,
    ArmorRaisingRare = 0x1C9,
    ArmorValueRaisingRare = 0x1CA,
    AssessMonsterRaisingRare = 0x1CB,
    AssessPersonRaisingRare = 0x1CC,
    AttackModRaisingRare = 0x1CD,
    AxeRaisingRare = 0x1CE,
    BludgeonProtectionRare = 0x1CF,
    BludgeonResistanceRaisingRare = 0x1D0,
    BowRaisingRare = 0x1D1,
    ColdProtectionRare = 0x1D2,
    ColdResistanceRaisingRare = 0x1D3,
    CookingRaisingRare = 0x1D4,
    CoordinationRaisingRare = 0x1D5,
    CreatureEnchantmentRaisingRare = 0x1D6,
    CrossbowRaisingRare = 0x1D7,
    DaggerRaisingRare = 0x1D8,
    DamageRaisingRare = 0x1D9,
    DeceptionRaisingRare = 0x1DA,
    DefenseModRaisingRare = 0x1DB,
    ElectricProtectionRare = 0x1DC,
    ElectricResistanceRaisingRare = 0x1DD,
    EnduranceRaisingRare = 0x1DE,
    FireProtectionRare = 0x1DF,
    FireResistanceRaisingRare = 0x1E0,
    FletchingRaisingRare = 0x1E1,
    FocusRaisingRare = 0x1E2,
    HealingRaisingRare = 0x1E3,
    HealthAcceleratingRare = 0x1E4,
    ItemEnchantmentRaisingRare = 0x1E5,
    JumpRaisingRare = 0x1E6,
    LeadershipRaisingRare = 0x1E7,
    LifeMagicRaisingRare = 0x1E8,
    LockpickRaisingRare = 0x1E9,
    LoyaltyRaisingRare = 0x1EA,
    MaceRaisingRare = 0x1EB,
    MagicDefenseRaisingRare = 0x1EC,
    ManaAcceleratingRare = 0x1ED,
    ManaConversionRaisingRare = 0x1EE,
    MeleeDefenseRaisingRare = 0x1EF,
    MissileDefenseRaisingRare = 0x1F0,
    PierceProtectionRare = 0x1F1,
    PierceResistanceRaisingRare = 0x1F2,
    QuicknessRaisingRare = 0x1F3,
    RunRaisingRare = 0x1F4,
    SelfRaisingRare = 0x1F5,
    SlashProtectionRare = 0x1F6,
    SlashResistanceRaisingRare = 0x1F7,
    SpearRaisingRare = 0x1F8,
    StaffRaisingRare = 0x1F9,
    StaminaAcceleratingRare = 0x1FA,
    StrengthRaisingRare = 0x1FB,
    SwordRaisingRare = 0x1FC,
    ThrownWeaponsRaisingRare = 0x1FD,
    UnarmedCombatRaisingRare = 0x1FE,
    WarMagicRaisingRare = 0x1FF,
    WeaponTimeRaisingRare = 0x200,
    ArmorIncreaseInkyArmor = 0x201,
    MagicDefenseShelterRaisingFiun = 0x202,
    ExtraRunSkillRaisingFiun = 0x203,
    ExtraManaConversionSkillRaisingFiun = 0x204,
    AttributesIncreaseCantrip1 = 0x205,
    ExtraMeleeDefenseSkillRaising2 = 0x206,
    ACTDPurchaseRewardSpell = 0x207,
    ACTDPurchaseRewardSpellHealth = 0x208,
    SaltAshAttackModRaising = 0x209,
    QuicknessIncrease2 = 0x20A,
    ExtraAlchemySkillRaising2 = 0x20B,
    ExtraCookingSkillRaising2 = 0x20C,
    ExtraFletchingSkillRaising2 = 0x20D,
    ExtraLockpickSkillRaising2 = 0x20E,
    MucorManaWell = 0x20F,
    StaminaRestoring2 = 0x210,
    AllegianceRaising = 0x211,
    HealthDoT = 0x212,
    HealthDoTSecondary = 0x213,
    HealthDoTTertiary = 0x214,
    HealthHoT = 0x215,
    HealthHoTSecondary = 0x216,
    HealthHoTTertiary = 0x217,
    HealthDivideSecondary = 0x218,
    HealthDivideTertiary = 0x219,
    SetSwordRaising = 0x21A,
    SetAxeRaising = 0x21B,
    SetDaggerRaising = 0x21C,
    SetMaceRaising = 0x21D,
    SetSpearRaising = 0x21E,
    SetStaffRaising = 0x21F,
    SetUnarmedRaising = 0x220,
    SetBowRaising = 0x221,
    SetCrossbowRaising = 0x222,
    SetThrownRaising = 0x223,
    SetItemEnchantmentRaising = 0x224,
    SetCreatureEnchantmentRaising = 0x225,
    SetWarMagicRaising = 0x226,
    SetLifeMagicRaising = 0x227,
    SetMeleeDefenseRaising = 0x228,
    SetMissileDefenseRaising = 0x229,
    SetMagicDefenseRaising = 0x22A,
    SetStaminaAccelerating = 0x22B,
    SetCookingRaising = 0x22C,
    SetFletchingRaising = 0x22D,
    SetLockpickRaising = 0x22E,
    SetAlchemyRaising = 0x22F,
    SetSalvagingRaising = 0x230,
    SetArmorExpertiseRaising = 0x231,
    SetWeaponExpertiseRaising = 0x232,
    SetItemTinkeringRaising = 0x233,
    SetMagicItemExpertiseRaising = 0x234,
    SetLoyaltyRaising = 0x235,
    SetStrengthRaising = 0x236,
    SetEnduranceRaising = 0x237,
    SetCoordinationRaising = 0x238,
    SetQuicknessRaising = 0x239,
    SetFocusRaising = 0x23A,
    SetWillpowerRaising = 0x23B,
    SetHealthRaising = 0x23C,
    SetStaminaRaising = 0x23D,
    SetManaRaising = 0x23E,
    SetSprintRaising = 0x23F,
    SetJumpingRaising = 0x240,
    SetSlashResistanceRaising = 0x241,
    SetBludgeonResistanceRaising = 0x242,
    SetPierceResistanceRaising = 0x243,
    SetFlameResistanceRaising = 0x244,
    SetAcidResistanceRaising = 0x245,
    SetFrostResistanceRaising = 0x246,
    SetLightningResistanceRaising = 0x247,
    CraftingLockPickRaising = 0x248,
    CraftingFletchingRaising = 0x249,
    CraftingCookingRaising = 0x24A,
    CraftingAlchemyRaising = 0x24B,
    CraftingArmorTinkeringRaising = 0x24C,
    CraftingWeaponTinkeringRaising = 0x24D,
    CraftingMagicTinkeringRaising = 0x24E,
    CraftingItemTinkeringRaising = 0x24F,
    SkillPercentAlchemyRaising = 0x250,
    TwoHandedRaising = 0x251,
    TwoHandedLowering = 0x252,
    ExtraTwoHandedSkillRaising = 0x253,
    ExtraTwoHandedSkillLowering = 0x254,
    ExtraTwoHandedSkillRaising2 = 0x255,
    TwoHandedRaisingRare = 0x256,
    SetTwoHandedRaising = 0x257,
    GearCraftRaising = 0x258,
    GearCraftLowering = 0x259,
    ExtraGearCraftSkillRaising = 0x25A,
    ExtraGearCraftSkillLowering = 0x25B,
    ExtraGearCraftSkillRaising2 = 0x25C,
    GearCraftRaisingRare = 0x25D,
    SetGearCraftRaising = 0x25E,
    LoyaltyManaRaising = 0x25F,
    LoyaltyStaminaRaising = 0x260,
    LeadershipHealthRaising = 0x261,
    TrinketDamageRaising = 0x262,
    TrinketDamageLowering = 0x263,
    TrinketHealthRaising = 0x264,
    TrinketStaminaRaising = 0x265,
    TrinketManaRaising = 0x266,
    TrinketXPRaising = 0x267,
    DeceptionArcaneLoreRaising = 0x268,
    HealOverTimeRaising = 0x269,
    DamageOverTimeRaising = 0x26A,
    HealingResistRatingRaising = 0x26B,
    AetheriaDamageRatingRaising = 0x26C,
    AetheriaDamageReductionRaising = 0x26D,
    AetheriaHealthRaising = 0x26F,
    AetheriaStaminaRaising = 0x270,
    AetheriaManaRaising = 0x271,
    AetheriaCriticalDamageRaising = 0x272,
    AetheriaHealingAmplificationRaising = 0x273,
    AetheriaProcDamageRatingRaising = 0x274,
    AetheriaProcDamageReductionRaising = 0x275,
    AetheriaProcHealthOverTimeRaising = 0x276,
    AetheriaProcDamageOverTimeRaising = 0x277,
    AetheriaProcHealingReductionRaising = 0x278,
    RareDamageRatingRaising = 0x279,
    RareDamageReductionRatingRaising = 0x27A,
    AetheriaEnduranceRaising = 0x27B,
    NetherDamageOverTimeRaising = 0x27C,
    NetherDamageOverTimeRaising2 = 0x27D,
    NetherDamageOverTimeRaising3 = 0x27E,
    NetherStreak = 0x27F,
    NetherMissile = 0x280,
    NetherRing = 0x281,
    NetherDamageRatingLowering = 0x282,
    NetherDamageHealingReductionRaising = 0x283,
    VoidMagicLowering = 0x284,
    VoidMagicRaising = 0x285,
    VoidMysticRaising = 0x286,
    SetVoidMagicRaising = 0x287,
    VoidMagicRaisingRare = 0x288,
    VoidMysticRaising2 = 0x289,
    LuminanceDamageRatingRaising = 0x28A,
    LuminanceDamageReductionRaising = 0x28B,
    LuminanceHealthRaising = 0x28C,
    AetheriaCriticalReductionRaising = 0x28D,
    ExtraMissileDefenseSkillRaising = 0x28E,
    ExtraMissileDefenseSkillLowering = 0x28F,
    ExtraMissileDefenseSkillRaising2 = 0x290,
    AetheriaHealthResistanceRaising = 0x291,
    AetheriaDotResistanceRaising = 0x292,
    CloakSkillRaising = 0x293,
    CloakAllSkillRaising = 0x294,
    CloakMagicDefenseLowering = 0x295,
    CloakMeleeDefenseLowering = 0x296,
    CloakMissileDefenseLowering = 0x297,
    DirtyFightingLowering = 0x298,
    DirtyFightingRaising = 0x299,
    ExtraDirtyFightingRaising = 0x29A,
    DualWieldLowering = 0x29B,
    DualWieldRaising = 0x29C,
    ExtraDualWieldRaising = 0x29D,
    RecklessnessLowering = 0x29E,
    RecklessnessRaising = 0x29F,
    ExtraRecklessnessRaising = 0x2A0,
    ShieldLowering = 0x2A1,
    ShieldRaising = 0x2A2,
    ExtraShieldRaising = 0x2A3,
    SneakAttackLowering = 0x2A4,
    SneakAttackRaising = 0x2A5,
    ExtraSneakAttackRaising = 0x2A6,
    RareDirtyFightingRaising = 0x2A7,
    RareDualWieldRaising = 0x2A8,
    RareRecklessnessRaising = 0x2A9,
    RareShieldRaising = 0x2AA,
    RareSneakAttackRaising = 0x2AB,
    DFAttackSkillDebuff = 0x2AC,
    DFBleedDamage = 0x2AD,
    DFDefenseSkillDebuff = 0x2AE,
    DFHealingDebuff = 0x2AF,
    SetDirtyFightingRaising = 0x2B0,
    SetDualWieldRaising = 0x2B1,
    SetRecklessnessRaising = 0x2B2,
    SetShieldRaising = 0x2B3,
    SetSneakAttackRaising = 0x2B4,
    LifeGiverMhoire = 0x2B5,
    RareDamageRatingRaising2 = 0x2B6,
    SpellDamageRaising = 0x2B7,
    SummoningRaising = 0x2B8,
    SummoningLowering = 0x2B9,
    ExtraSummoningSkillRaising = 0x2BA,
    SetSummoningRaising = 0x2BB,
    ParagonEnduranceRaising = 0x2C0,
    ParagonManaRaising = 0x2C1,
    ParagonStaminaRaising = 0x2C2,
    ParagonDirtyFightingRaising = 0x2C3,
    ParagonDualWieldRaising = 0x2C4,
    ParagonRecklessnessRaising = 0x2C5,
    ParagonSneakAttackRaising = 0x2C6,
    ParagonDamageRatingRaising = 0x2C7,
    ParagonDamageReductionRatingRaising = 0x2C8,
    ParagonCriticalDamageRatingRaising = 0x2C9,
    ParagonCriticalDamageReductionRatingRaising = 0x2CA,
    ParagonAxeRaising = 0x2CB,
    ParagonDaggerRaising = 0x2CC,
    ParagonSwordRaising = 0x2CD,
    ParagonWarMagicRaising = 0x2CE,
    ParagonLifeMagicRaising = 0x2CF,
    ParagonVoidMagicRaising = 0x2D0,
    ParagonBowRaising = 0x2D1,
    ParagonStrengthRaising = 0x2D2,
    ParagonCoordinationRaising = 0x2D3,
    ParagonQuicknessRaising = 0x2D4,
    ParagonFocusRaising = 0x2D5,
    ParagonWillpowerRaising = 0x2D6,
    ParagonTwoHandedRaising = 0x2D7,
    GauntletDamageReductionRatingRaising = 0x2D8,
    GauntletDamageRatingRaising = 0x2D9,
    GauntletHealingRatingRaising = 0x2DA,
    GauntletVitalityRaising = 0x2DB,
    GauntletCriticalDamageRatingRaising = 0x2DC,
    GauntletCriticalDamageReductionRatingRaising = 0x2DD,
}

impl crate::readers::ACDataType for SpellCategory {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(SpellCategory::try_from(value)?)
    }
}

impl crate::writers::ACWritable for SpellCategory {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u16(writer, self.clone() as u16)?;
        Ok(())
    }
}

impl std::fmt::Display for SpellCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SpellCategory::Undef => "Undef",
            SpellCategory::StrengthRaising => "StrengthRaising",
            SpellCategory::StrengthLowering => "StrengthLowering",
            SpellCategory::EnduranceRaising => "EnduranceRaising",
            SpellCategory::EnduranceLowering => "EnduranceLowering",
            SpellCategory::QuicknessRaising => "QuicknessRaising",
            SpellCategory::QuicknessLowering => "QuicknessLowering",
            SpellCategory::CoordinationRaising => "CoordinationRaising",
            SpellCategory::CoordinationLowering => "CoordinationLowering",
            SpellCategory::FocusRaising => "FocusRaising",
            SpellCategory::FocusLowering => "FocusLowering",
            SpellCategory::SelfRaising => "SelfRaising",
            SpellCategory::SelfLowering => "SelfLowering",
            SpellCategory::FocusConcentration => "FocusConcentration",
            SpellCategory::FocusDisruption => "FocusDisruption",
            SpellCategory::FocusBrilliance => "FocusBrilliance",
            SpellCategory::FocusDullness => "FocusDullness",
            SpellCategory::AxeRaising => "AxeRaising",
            SpellCategory::AxeLowering => "AxeLowering",
            SpellCategory::BowRaising => "BowRaising",
            SpellCategory::BowLowering => "BowLowering",
            SpellCategory::CrossbowRaising => "CrossbowRaising",
            SpellCategory::CrossbowLowering => "CrossbowLowering",
            SpellCategory::DaggerRaising => "DaggerRaising",
            SpellCategory::DaggerLowering => "DaggerLowering",
            SpellCategory::MaceRaising => "MaceRaising",
            SpellCategory::MaceLowering => "MaceLowering",
            SpellCategory::SpearRaising => "SpearRaising",
            SpellCategory::SpearLowering => "SpearLowering",
            SpellCategory::StaffRaising => "StaffRaising",
            SpellCategory::StaffLowering => "StaffLowering",
            SpellCategory::SwordRaising => "SwordRaising",
            SpellCategory::SwordLowering => "SwordLowering",
            SpellCategory::ThrownWeaponsRaising => "ThrownWeaponsRaising",
            SpellCategory::ThrownWeaponsLowering => "ThrownWeaponsLowering",
            SpellCategory::UnarmedCombatRaising => "UnarmedCombatRaising",
            SpellCategory::UnarmedCombatLowering => "UnarmedCombatLowering",
            SpellCategory::MeleeDefenseRaising => "MeleeDefenseRaising",
            SpellCategory::MeleeDefenseLowering => "MeleeDefenseLowering",
            SpellCategory::MissileDefenseRaising => "MissileDefenseRaising",
            SpellCategory::MissileDefenseLowering => "MissileDefenseLowering",
            SpellCategory::MagicDefenseRaising => "MagicDefenseRaising",
            SpellCategory::MagicDefenseLowering => "MagicDefenseLowering",
            SpellCategory::CreatureEnchantmentRaising => "CreatureEnchantmentRaising",
            SpellCategory::CreatureEnchantmentLowering => "CreatureEnchantmentLowering",
            SpellCategory::ItemEnchantmentRaising => "ItemEnchantmentRaising",
            SpellCategory::ItemEnchantmentLowering => "ItemEnchantmentLowering",
            SpellCategory::LifeMagicRaising => "LifeMagicRaising",
            SpellCategory::LifeMagicLowering => "LifeMagicLowering",
            SpellCategory::WarMagicRaising => "WarMagicRaising",
            SpellCategory::WarMagicLowering => "WarMagicLowering",
            SpellCategory::ManaConversionRaising => "ManaConversionRaising",
            SpellCategory::ManaConversionLowering => "ManaConversionLowering",
            SpellCategory::ArcaneLoreRaising => "ArcaneLoreRaising",
            SpellCategory::ArcaneLoreLowering => "ArcaneLoreLowering",
            SpellCategory::AppraiseArmorRaising => "AppraiseArmorRaising",
            SpellCategory::AppraiseArmorLowering => "AppraiseArmorLowering",
            SpellCategory::AppraiseItemRaising => "AppraiseItemRaising",
            SpellCategory::AppraiseItemLowering => "AppraiseItemLowering",
            SpellCategory::AppraiseMagicItemRaising => "AppraiseMagicItemRaising",
            SpellCategory::AppraiseMagicItemLowering => "AppraiseMagicItemLowering",
            SpellCategory::AppraiseWeaponRaising => "AppraiseWeaponRaising",
            SpellCategory::AppraiseWeaponLowering => "AppraiseWeaponLowering",
            SpellCategory::AssessMonsterRaising => "AssessMonsterRaising",
            SpellCategory::AssessMonsterLowering => "AssessMonsterLowering",
            SpellCategory::DeceptionRaising => "DeceptionRaising",
            SpellCategory::DeceptionLowering => "DeceptionLowering",
            SpellCategory::HealingRaising => "HealingRaising",
            SpellCategory::HealingLowering => "HealingLowering",
            SpellCategory::JumpRaising => "JumpRaising",
            SpellCategory::JumpLowering => "JumpLowering",
            SpellCategory::LeadershipRaising => "LeadershipRaising",
            SpellCategory::LeadershipLowering => "LeadershipLowering",
            SpellCategory::LockpickRaising => "LockpickRaising",
            SpellCategory::LockpickLowering => "LockpickLowering",
            SpellCategory::LoyaltyRaising => "LoyaltyRaising",
            SpellCategory::LoyaltyLowering => "LoyaltyLowering",
            SpellCategory::RunRaising => "RunRaising",
            SpellCategory::RunLowering => "RunLowering",
            SpellCategory::HealthRaising => "HealthRaising",
            SpellCategory::HealthLowering => "HealthLowering",
            SpellCategory::StaminaRaising => "StaminaRaising",
            SpellCategory::StaminaLowering => "StaminaLowering",
            SpellCategory::ManaRaising => "ManaRaising",
            SpellCategory::ManaLowering => "ManaLowering",
            SpellCategory::ManaRemedy => "ManaRemedy",
            SpellCategory::ManaMalediction => "ManaMalediction",
            SpellCategory::HealthTransfertocaster => "HealthTransfertocaster",
            SpellCategory::HealthTransferfromcaster => "HealthTransferfromcaster",
            SpellCategory::StaminaTransfertocaster => "StaminaTransfertocaster",
            SpellCategory::StaminaTransferfromcaster => "StaminaTransferfromcaster",
            SpellCategory::ManaTransfertocaster => "ManaTransfertocaster",
            SpellCategory::ManaTransferfromcaster => "ManaTransferfromcaster",
            SpellCategory::HealthAccelerating => "HealthAccelerating",
            SpellCategory::HealthDecelerating => "HealthDecelerating",
            SpellCategory::StaminaAccelerating => "StaminaAccelerating",
            SpellCategory::StaminaDecelerating => "StaminaDecelerating",
            SpellCategory::ManaAccelerating => "ManaAccelerating",
            SpellCategory::ManaDecelerating => "ManaDecelerating",
            SpellCategory::VitaeRaising => "VitaeRaising",
            SpellCategory::VitaeLowering => "VitaeLowering",
            SpellCategory::AcidProtection => "AcidProtection",
            SpellCategory::AcidVulnerability => "AcidVulnerability",
            SpellCategory::BludgeonProtection => "BludgeonProtection",
            SpellCategory::BludgeonVulnerability => "BludgeonVulnerability",
            SpellCategory::ColdProtection => "ColdProtection",
            SpellCategory::ColdVulnerability => "ColdVulnerability",
            SpellCategory::ElectricProtection => "ElectricProtection",
            SpellCategory::ElectricVulnerability => "ElectricVulnerability",
            SpellCategory::FireProtection => "FireProtection",
            SpellCategory::FireVulnerability => "FireVulnerability",
            SpellCategory::PierceProtection => "PierceProtection",
            SpellCategory::PierceVulnerability => "PierceVulnerability",
            SpellCategory::SlashProtection => "SlashProtection",
            SpellCategory::SlashVulnerability => "SlashVulnerability",
            SpellCategory::ArmorRaising => "ArmorRaising",
            SpellCategory::ArmorLowering => "ArmorLowering",
            SpellCategory::AcidMissile => "AcidMissile",
            SpellCategory::BludgeoningMissile => "BludgeoningMissile",
            SpellCategory::ColdMissile => "ColdMissile",
            SpellCategory::ElectricMissile => "ElectricMissile",
            SpellCategory::FireMissile => "FireMissile",
            SpellCategory::PiercingMissile => "PiercingMissile",
            SpellCategory::SlashingMissile => "SlashingMissile",
            SpellCategory::AcidSeeker => "AcidSeeker",
            SpellCategory::BludgeoningSeeker => "BludgeoningSeeker",
            SpellCategory::ColdSeeker => "ColdSeeker",
            SpellCategory::ElectricSeeker => "ElectricSeeker",
            SpellCategory::FireSeeker => "FireSeeker",
            SpellCategory::PiercingSeeker => "PiercingSeeker",
            SpellCategory::SlashingSeeker => "SlashingSeeker",
            SpellCategory::AcidBurst => "AcidBurst",
            SpellCategory::BludgeoningBurst => "BludgeoningBurst",
            SpellCategory::ColdBurst => "ColdBurst",
            SpellCategory::ElectricBurst => "ElectricBurst",
            SpellCategory::FireBurst => "FireBurst",
            SpellCategory::PiercingBurst => "PiercingBurst",
            SpellCategory::SlashingBurst => "SlashingBurst",
            SpellCategory::AcidBlast => "AcidBlast",
            SpellCategory::BludgeoningBlast => "BludgeoningBlast",
            SpellCategory::ColdBlast => "ColdBlast",
            SpellCategory::ElectricBlast => "ElectricBlast",
            SpellCategory::FireBlast => "FireBlast",
            SpellCategory::PiercingBlast => "PiercingBlast",
            SpellCategory::SlashingBlast => "SlashingBlast",
            SpellCategory::AcidScatter => "AcidScatter",
            SpellCategory::BludgeoningScatter => "BludgeoningScatter",
            SpellCategory::ColdScatter => "ColdScatter",
            SpellCategory::ElectricScatter => "ElectricScatter",
            SpellCategory::FireScatter => "FireScatter",
            SpellCategory::PiercingScatter => "PiercingScatter",
            SpellCategory::SlashingScatter => "SlashingScatter",
            SpellCategory::AttackModRaising => "AttackModRaising",
            SpellCategory::AttackModLowering => "AttackModLowering",
            SpellCategory::DamageRaising => "DamageRaising",
            SpellCategory::DamageLowering => "DamageLowering",
            SpellCategory::DefenseModRaising => "DefenseModRaising",
            SpellCategory::DefenseModLowering => "DefenseModLowering",
            SpellCategory::WeaponTimeRaising => "WeaponTimeRaising",
            SpellCategory::WeaponTimeLowering => "WeaponTimeLowering",
            SpellCategory::ArmorValueRaising => "ArmorValueRaising",
            SpellCategory::ArmorValueLowering => "ArmorValueLowering",
            SpellCategory::AcidResistanceRaising => "AcidResistanceRaising",
            SpellCategory::AcidResistanceLowering => "AcidResistanceLowering",
            SpellCategory::BludgeonResistanceRaising => "BludgeonResistanceRaising",
            SpellCategory::BludgeonResistanceLowering => "BludgeonResistanceLowering",
            SpellCategory::ColdResistanceRaising => "ColdResistanceRaising",
            SpellCategory::ColdResistanceLowering => "ColdResistanceLowering",
            SpellCategory::ElectricResistanceRaising => "ElectricResistanceRaising",
            SpellCategory::ElectricResistanceLowering => "ElectricResistanceLowering",
            SpellCategory::FireResistanceRaising => "FireResistanceRaising",
            SpellCategory::FireResistanceLowering => "FireResistanceLowering",
            SpellCategory::PierceResistanceRaising => "PierceResistanceRaising",
            SpellCategory::PierceResistanceLowering => "PierceResistanceLowering",
            SpellCategory::SlashResistanceRaising => "SlashResistanceRaising",
            SpellCategory::SlashResistanceLowering => "SlashResistanceLowering",
            SpellCategory::BludgeoningResistanceRaising => "BludgeoningResistanceRaising",
            SpellCategory::BludgeoningResistanceLowering => "BludgeoningResistanceLowering",
            SpellCategory::SlashingResistanceRaising => "SlashingResistanceRaising",
            SpellCategory::SlashingResistanceLowering => "SlashingResistanceLowering",
            SpellCategory::PiercingResistanceRaising => "PiercingResistanceRaising",
            SpellCategory::PiercingResistanceLowering => "PiercingResistanceLowering",
            SpellCategory::ElectricalResistanceRaising => "ElectricalResistanceRaising",
            SpellCategory::ElectricalResistanceLowering => "ElectricalResistanceLowering",
            SpellCategory::FrostResistanceRaising => "FrostResistanceRaising",
            SpellCategory::FrostResistanceLowering => "FrostResistanceLowering",
            SpellCategory::FlameResistanceRaising => "FlameResistanceRaising",
            SpellCategory::FlameResistanceLowering => "FlameResistanceLowering",
            SpellCategory::AcidicResistanceRaising => "AcidicResistanceRaising",
            SpellCategory::AcidicResistanceLowering => "AcidicResistanceLowering",
            SpellCategory::ArmorLevelRaising => "ArmorLevelRaising",
            SpellCategory::ArmorLevelLowering => "ArmorLevelLowering",
            SpellCategory::LockpickResistanceRaising => "LockpickResistanceRaising",
            SpellCategory::LockpickResistanceLowering => "LockpickResistanceLowering",
            SpellCategory::ManaConversionModLowering => "ManaConversionModLowering",
            SpellCategory::ManaConversionModRaising => "ManaConversionModRaising",
            SpellCategory::VisionRaising => "VisionRaising",
            SpellCategory::VisionLowering => "VisionLowering",
            SpellCategory::TransparencyRaising => "TransparencyRaising",
            SpellCategory::TransparencyLowering => "TransparencyLowering",
            SpellCategory::PortalTie => "PortalTie",
            SpellCategory::PortalRecall => "PortalRecall",
            SpellCategory::PortalCreation => "PortalCreation",
            SpellCategory::PortalItemCreation => "PortalItemCreation",
            SpellCategory::Vitae => "Vitae",
            SpellCategory::AssessPersonRaising => "AssessPersonRaising",
            SpellCategory::AssessPersonLowering => "AssessPersonLowering",
            SpellCategory::AcidVolley => "AcidVolley",
            SpellCategory::BludgeoningVolley => "BludgeoningVolley",
            SpellCategory::FrostVolley => "FrostVolley",
            SpellCategory::LightningVolley => "LightningVolley",
            SpellCategory::FlameVolley => "FlameVolley",
            SpellCategory::ForceVolley => "ForceVolley",
            SpellCategory::BladeVolley => "BladeVolley",
            SpellCategory::PortalSending => "PortalSending",
            SpellCategory::LifestoneSending => "LifestoneSending",
            SpellCategory::CookingRaising => "CookingRaising",
            SpellCategory::CookingLowering => "CookingLowering",
            SpellCategory::FletchingRaising => "FletchingRaising",
            SpellCategory::FletchingLowering => "FletchingLowering",
            SpellCategory::AlchemyLowering => "AlchemyLowering",
            SpellCategory::AlchemyRaising => "AlchemyRaising",
            SpellCategory::AcidRing => "AcidRing",
            SpellCategory::BludgeoningRing => "BludgeoningRing",
            SpellCategory::ColdRing => "ColdRing",
            SpellCategory::ElectricRing => "ElectricRing",
            SpellCategory::FireRing => "FireRing",
            SpellCategory::PiercingRing => "PiercingRing",
            SpellCategory::SlashingRing => "SlashingRing",
            SpellCategory::AcidWall => "AcidWall",
            SpellCategory::BludgeoningWall => "BludgeoningWall",
            SpellCategory::ColdWall => "ColdWall",
            SpellCategory::ElectricWall => "ElectricWall",
            SpellCategory::FireWall => "FireWall",
            SpellCategory::PiercingWall => "PiercingWall",
            SpellCategory::SlashingWall => "SlashingWall",
            SpellCategory::AcidStrike => "AcidStrike",
            SpellCategory::BludgeoningStrike => "BludgeoningStrike",
            SpellCategory::ColdStrike => "ColdStrike",
            SpellCategory::ElectricStrike => "ElectricStrike",
            SpellCategory::FireStrike => "FireStrike",
            SpellCategory::PiercingStrike => "PiercingStrike",
            SpellCategory::SlashingStrike => "SlashingStrike",
            SpellCategory::AcidStreak => "AcidStreak",
            SpellCategory::BludgeoningStreak => "BludgeoningStreak",
            SpellCategory::ColdStreak => "ColdStreak",
            SpellCategory::ElectricStreak => "ElectricStreak",
            SpellCategory::FireStreak => "FireStreak",
            SpellCategory::PiercingStreak => "PiercingStreak",
            SpellCategory::SlashingStreak => "SlashingStreak",
            SpellCategory::Dispel => "Dispel",
            SpellCategory::CreatureMysticRaising => "CreatureMysticRaising",
            SpellCategory::CreatureMysticLowering => "CreatureMysticLowering",
            SpellCategory::ItemMysticRaising => "ItemMysticRaising",
            SpellCategory::ItemMysticLowering => "ItemMysticLowering",
            SpellCategory::WarMysticRaising => "WarMysticRaising",
            SpellCategory::WarMysticLowering => "WarMysticLowering",
            SpellCategory::HealthRestoring => "HealthRestoring",
            SpellCategory::HealthDepleting => "HealthDepleting",
            SpellCategory::ManaRestoring => "ManaRestoring",
            SpellCategory::ManaDepleting => "ManaDepleting",
            SpellCategory::StrengthIncrease => "StrengthIncrease",
            SpellCategory::StrengthDecrease => "StrengthDecrease",
            SpellCategory::EnduranceIncrease => "EnduranceIncrease",
            SpellCategory::EnduranceDecrease => "EnduranceDecrease",
            SpellCategory::QuicknessIncrease => "QuicknessIncrease",
            SpellCategory::QuicknessDecrease => "QuicknessDecrease",
            SpellCategory::CoordinationIncrease => "CoordinationIncrease",
            SpellCategory::CoordinationDecrease => "CoordinationDecrease",
            SpellCategory::FocusIncrease => "FocusIncrease",
            SpellCategory::FocusDecrease => "FocusDecrease",
            SpellCategory::SelfIncrease => "SelfIncrease",
            SpellCategory::SelfDecrease => "SelfDecrease",
            SpellCategory::GreatVitalityRaising => "GreatVitalityRaising",
            SpellCategory::PoorVitalityLowering => "PoorVitalityLowering",
            SpellCategory::GreatVigorRaising => "GreatVigorRaising",
            SpellCategory::PoorVigorLowering => "PoorVigorLowering",
            SpellCategory::GreaterIntellectRaising => "GreaterIntellectRaising",
            SpellCategory::LessorIntellectLowering => "LessorIntellectLowering",
            SpellCategory::LifeGiverRaising => "LifeGiverRaising",
            SpellCategory::LifeTakerLowering => "LifeTakerLowering",
            SpellCategory::StaminaGiverRaising => "StaminaGiverRaising",
            SpellCategory::StaminaTakerLowering => "StaminaTakerLowering",
            SpellCategory::ManaGiverRaising => "ManaGiverRaising",
            SpellCategory::ManaTakerLowering => "ManaTakerLowering",
            SpellCategory::AcidWardProtection => "AcidWardProtection",
            SpellCategory::AcidWardVulnerability => "AcidWardVulnerability",
            SpellCategory::FireWardProtection => "FireWardProtection",
            SpellCategory::FireWardVulnerability => "FireWardVulnerability",
            SpellCategory::ColdWardProtection => "ColdWardProtection",
            SpellCategory::ColdWardVulnerability => "ColdWardVulnerability",
            SpellCategory::ElectricWardProtection => "ElectricWardProtection",
            SpellCategory::ElectricWardVulnerability => "ElectricWardVulnerability",
            SpellCategory::LeadershipObedienceRaising => "LeadershipObedienceRaising",
            SpellCategory::LeadershipObedienceLowering => "LeadershipObedienceLowering",
            SpellCategory::MeleeDefenseShelterRaising => "MeleeDefenseShelterRaising",
            SpellCategory::MeleeDefenseShelterLowering => "MeleeDefenseShelterLowering",
            SpellCategory::MissileDefenseShelterRaising => "MissileDefenseShelterRaising",
            SpellCategory::MissileDefenseShelterLowering => "MissileDefenseShelterLowering",
            SpellCategory::MagicDefenseShelterRaising => "MagicDefenseShelterRaising",
            SpellCategory::MagicDefenseShelterLowering => "MagicDefenseShelterLowering",
            SpellCategory::HuntersAcumenRaising => "HuntersAcumenRaising",
            SpellCategory::HuntersAcumenLowering => "HuntersAcumenLowering",
            SpellCategory::StillWaterRaising => "StillWaterRaising",
            SpellCategory::StillWaterLowering => "StillWaterLowering",
            SpellCategory::StrengthofEarthRaising => "StrengthofEarthRaising",
            SpellCategory::StrengthofEarthLowering => "StrengthofEarthLowering",
            SpellCategory::TorrentRaising => "TorrentRaising",
            SpellCategory::TorrentLowering => "TorrentLowering",
            SpellCategory::GrowthRaising => "GrowthRaising",
            SpellCategory::GrowthLowering => "GrowthLowering",
            SpellCategory::CascadeAxeRaising => "CascadeAxeRaising",
            SpellCategory::CascadeAxeLowering => "CascadeAxeLowering",
            SpellCategory::CascadeDaggerRaising => "CascadeDaggerRaising",
            SpellCategory::CascadeDaggerLowering => "CascadeDaggerLowering",
            SpellCategory::CascadeMaceRaising => "CascadeMaceRaising",
            SpellCategory::CascadeMaceLowering => "CascadeMaceLowering",
            SpellCategory::CascadeSpearRaising => "CascadeSpearRaising",
            SpellCategory::CascadeSpearLowering => "CascadeSpearLowering",
            SpellCategory::CascadeStaffRaising => "CascadeStaffRaising",
            SpellCategory::CascadeStaffLowering => "CascadeStaffLowering",
            SpellCategory::StoneCliffsRaising => "StoneCliffsRaising",
            SpellCategory::StoneCliffsLowering => "StoneCliffsLowering",
            SpellCategory::MaxDamageRaising => "MaxDamageRaising",
            SpellCategory::MaxDamageLowering => "MaxDamageLowering",
            SpellCategory::BowDamageRaising => "BowDamageRaising",
            SpellCategory::BowDamageLowering => "BowDamageLowering",
            SpellCategory::BowRangeRaising => "BowRangeRaising",
            SpellCategory::BowRangeLowering => "BowRangeLowering",
            SpellCategory::ExtraDefenseModRaising => "ExtraDefenseModRaising",
            SpellCategory::ExtraDefenseModLowering => "ExtraDefenseModLowering",
            SpellCategory::ExtraBowSkillRaising => "ExtraBowSkillRaising",
            SpellCategory::ExtraBowSkillLowering => "ExtraBowSkillLowering",
            SpellCategory::ExtraAlchemySkillRaising => "ExtraAlchemySkillRaising",
            SpellCategory::ExtraAlchemySkillLowering => "ExtraAlchemySkillLowering",
            SpellCategory::ExtraArcaneLoreSkillRaising => "ExtraArcaneLoreSkillRaising",
            SpellCategory::ExtraArcaneLoreSkillLowering => "ExtraArcaneLoreSkillLowering",
            SpellCategory::ExtraAppraiseArmorSkillRaising => "ExtraAppraiseArmorSkillRaising",
            SpellCategory::ExtraAppraiseArmorSkillLowering => "ExtraAppraiseArmorSkillLowering",
            SpellCategory::ExtraCookingSkillRaising => "ExtraCookingSkillRaising",
            SpellCategory::ExtraCookingSkillLowering => "ExtraCookingSkillLowering",
            SpellCategory::ExtraCrossbowSkillRaising => "ExtraCrossbowSkillRaising",
            SpellCategory::ExtraCrossbowSkillLowering => "ExtraCrossbowSkillLowering",
            SpellCategory::ExtraDeceptionSkillRaising => "ExtraDeceptionSkillRaising",
            SpellCategory::ExtraDeceptionSkillLowering => "ExtraDeceptionSkillLowering",
            SpellCategory::ExtraLoyaltySkillRaising => "ExtraLoyaltySkillRaising",
            SpellCategory::ExtraLoyaltySkillLowering => "ExtraLoyaltySkillLowering",
            SpellCategory::ExtraFletchingSkillRaising => "ExtraFletchingSkillRaising",
            SpellCategory::ExtraFletchingSkillLowering => "ExtraFletchingSkillLowering",
            SpellCategory::ExtraHealingSkillRaising => "ExtraHealingSkillRaising",
            SpellCategory::ExtraHealingSkillLowering => "ExtraHealingSkillLowering",
            SpellCategory::ExtraMeleeDefenseSkillRaising => "ExtraMeleeDefenseSkillRaising",
            SpellCategory::ExtraMeleeDefenseSkillLowering => "ExtraMeleeDefenseSkillLowering",
            SpellCategory::ExtraAppraiseItemSkillRaising => "ExtraAppraiseItemSkillRaising",
            SpellCategory::ExtraAppraiseItemSkillLowering => "ExtraAppraiseItemSkillLowering",
            SpellCategory::ExtraJumpingSkillRaising => "ExtraJumpingSkillRaising",
            SpellCategory::ExtraJumpingSkillLowering => "ExtraJumpingSkillLowering",
            SpellCategory::ExtraLifeMagicSkillRaising => "ExtraLifeMagicSkillRaising",
            SpellCategory::ExtraLifeMagicSkillLowering => "ExtraLifeMagicSkillLowering",
            SpellCategory::ExtraLockpickSkillRaising => "ExtraLockpickSkillRaising",
            SpellCategory::ExtraLockpickSkillLowering => "ExtraLockpickSkillLowering",
            SpellCategory::ExtraAppraiseMagicItemSkillRaising => "ExtraAppraiseMagicItemSkillRaising",
            SpellCategory::ExtraAppraiseMagicItemSkillLowering => "ExtraAppraiseMagicItemSkillLowering",
            SpellCategory::ExtraManaConversionSkillRaising => "ExtraManaConversionSkillRaising",
            SpellCategory::ExtraManaConversionSkillLowering => "ExtraManaConversionSkillLowering",
            SpellCategory::ExtraAssessCreatureSkillRaising => "ExtraAssessCreatureSkillRaising",
            SpellCategory::ExtraAssessCreatureSkillLowering => "ExtraAssessCreatureSkillLowering",
            SpellCategory::ExtraAssessPersonSkillRaising => "ExtraAssessPersonSkillRaising",
            SpellCategory::ExtraAssessPersonSkillLowering => "ExtraAssessPersonSkillLowering",
            SpellCategory::ExtraRunSkillRaising => "ExtraRunSkillRaising",
            SpellCategory::ExtraRunSkillLowering => "ExtraRunSkillLowering",
            SpellCategory::ExtraSwordSkillRaising => "ExtraSwordSkillRaising",
            SpellCategory::ExtraSwordSkillLowering => "ExtraSwordSkillLowering",
            SpellCategory::ExtraThrownWeaponsSkillRaising => "ExtraThrownWeaponsSkillRaising",
            SpellCategory::ExtraThrownWeaponsSkillLowering => "ExtraThrownWeaponsSkillLowering",
            SpellCategory::ExtraUnarmedCombatSkillRaising => "ExtraUnarmedCombatSkillRaising",
            SpellCategory::ExtraUnarmedCombatSkillLowering => "ExtraUnarmedCombatSkillLowering",
            SpellCategory::ExtraAppraiseWeaponSkillRaising => "ExtraAppraiseWeaponSkillRaising",
            SpellCategory::ExtraAppraiseWeaponSkillLowering => "ExtraAppraiseWeaponSkillLowering",
            SpellCategory::ArmorIncrease => "ArmorIncrease",
            SpellCategory::ArmorDecrease => "ArmorDecrease",
            SpellCategory::ExtraAcidResistanceRaising => "ExtraAcidResistanceRaising",
            SpellCategory::ExtraAcidResistanceLowering => "ExtraAcidResistanceLowering",
            SpellCategory::ExtraBludgeonResistanceRaising => "ExtraBludgeonResistanceRaising",
            SpellCategory::ExtraBludgeonResistanceLowering => "ExtraBludgeonResistanceLowering",
            SpellCategory::ExtraFireResistanceRaising => "ExtraFireResistanceRaising",
            SpellCategory::ExtraFireResistanceLowering => "ExtraFireResistanceLowering",
            SpellCategory::ExtraColdResistanceRaising => "ExtraColdResistanceRaising",
            SpellCategory::ExtraColdResistanceLowering => "ExtraColdResistanceLowering",
            SpellCategory::ExtraAttackModRaising => "ExtraAttackModRaising",
            SpellCategory::ExtraAttackModLowering => "ExtraAttackModLowering",
            SpellCategory::ExtraArmorValueRaising => "ExtraArmorValueRaising",
            SpellCategory::ExtraArmorValueLowering => "ExtraArmorValueLowering",
            SpellCategory::ExtraPierceResistanceRaising => "ExtraPierceResistanceRaising",
            SpellCategory::ExtraPierceResistanceLowering => "ExtraPierceResistanceLowering",
            SpellCategory::ExtraSlashResistanceRaising => "ExtraSlashResistanceRaising",
            SpellCategory::ExtraSlashResistanceLowering => "ExtraSlashResistanceLowering",
            SpellCategory::ExtraElectricResistanceRaising => "ExtraElectricResistanceRaising",
            SpellCategory::ExtraElectricResistanceLowering => "ExtraElectricResistanceLowering",
            SpellCategory::ExtraWeaponTimeRaising => "ExtraWeaponTimeRaising",
            SpellCategory::ExtraWeaponTimeLowering => "ExtraWeaponTimeLowering",
            SpellCategory::BludgeonWardProtection => "BludgeonWardProtection",
            SpellCategory::BludgeonWardVulnerability => "BludgeonWardVulnerability",
            SpellCategory::SlashWardProtection => "SlashWardProtection",
            SpellCategory::SlashWardVulnerability => "SlashWardVulnerability",
            SpellCategory::PierceWardProtection => "PierceWardProtection",
            SpellCategory::PierceWardVulnerability => "PierceWardVulnerability",
            SpellCategory::StaminaRestoring => "StaminaRestoring",
            SpellCategory::StaminaDepleting => "StaminaDepleting",
            SpellCategory::Fireworks => "Fireworks",
            SpellCategory::HealthDivide => "HealthDivide",
            SpellCategory::StaminaDivide => "StaminaDivide",
            SpellCategory::ManaDivide => "ManaDivide",
            SpellCategory::CoordinationIncrease2 => "CoordinationIncrease2",
            SpellCategory::StrengthIncrease2 => "StrengthIncrease2",
            SpellCategory::FocusIncrease2 => "FocusIncrease2",
            SpellCategory::EnduranceIncrease2 => "EnduranceIncrease2",
            SpellCategory::SelfIncrease2 => "SelfIncrease2",
            SpellCategory::MeleeDefenseMultiply => "MeleeDefenseMultiply",
            SpellCategory::MissileDefenseMultiply => "MissileDefenseMultiply",
            SpellCategory::MagicDefenseMultiply => "MagicDefenseMultiply",
            SpellCategory::AttributesDecrease => "AttributesDecrease",
            SpellCategory::LifeGiverRaising2 => "LifeGiverRaising2",
            SpellCategory::ItemEnchantmentRaising2 => "ItemEnchantmentRaising2",
            SpellCategory::SkillsDecrease => "SkillsDecrease",
            SpellCategory::ExtraManaConversionBonus => "ExtraManaConversionBonus",
            SpellCategory::WarMysticRaising2 => "WarMysticRaising2",
            SpellCategory::WarMysticLowering2 => "WarMysticLowering2",
            SpellCategory::MagicDefenseShelterRaising2 => "MagicDefenseShelterRaising2",
            SpellCategory::ExtraLifeMagicSkillRaising2 => "ExtraLifeMagicSkillRaising2",
            SpellCategory::CreatureMysticRaising2 => "CreatureMysticRaising2",
            SpellCategory::ItemMysticRaising2 => "ItemMysticRaising2",
            SpellCategory::ManaRaising2 => "ManaRaising2",
            SpellCategory::SelfRaising2 => "SelfRaising2",
            SpellCategory::CreatureEnchantmentRaising2 => "CreatureEnchantmentRaising2",
            SpellCategory::SalvagingRaising => "SalvagingRaising",
            SpellCategory::ExtraSalvagingRaising => "ExtraSalvagingRaising",
            SpellCategory::ExtraSalvagingRaising2 => "ExtraSalvagingRaising2",
            SpellCategory::CascadeAxeRaising2 => "CascadeAxeRaising2",
            SpellCategory::ExtraBowSkillRaising2 => "ExtraBowSkillRaising2",
            SpellCategory::ExtraThrownWeaponsSkillRaising2 => "ExtraThrownWeaponsSkillRaising2",
            SpellCategory::ExtraCrossbowSkillRaising2 => "ExtraCrossbowSkillRaising2",
            SpellCategory::CascadeDaggerRaising2 => "CascadeDaggerRaising2",
            SpellCategory::CascadeMaceRaising2 => "CascadeMaceRaising2",
            SpellCategory::ExtraUnarmedCombatSkillRaising2 => "ExtraUnarmedCombatSkillRaising2",
            SpellCategory::CascadeSpearRaising2 => "CascadeSpearRaising2",
            SpellCategory::CascadeStaffRaising2 => "CascadeStaffRaising2",
            SpellCategory::ExtraSwordSkillRaising2 => "ExtraSwordSkillRaising2",
            SpellCategory::AcidProtectionRare => "AcidProtectionRare",
            SpellCategory::AcidResistanceRaisingRare => "AcidResistanceRaisingRare",
            SpellCategory::AlchemyRaisingRare => "AlchemyRaisingRare",
            SpellCategory::AppraisalResistanceLoweringRare => "AppraisalResistanceLoweringRare",
            SpellCategory::AppraiseArmorRaisingRare => "AppraiseArmorRaisingRare",
            SpellCategory::AppraiseItemRaisingRare => "AppraiseItemRaisingRare",
            SpellCategory::AppraiseMagicItemRaisingRare => "AppraiseMagicItemRaisingRare",
            SpellCategory::AppraiseWeaponRaisingRare => "AppraiseWeaponRaisingRare",
            SpellCategory::ArcaneLoreRaisingRare => "ArcaneLoreRaisingRare",
            SpellCategory::ArmorRaisingRare => "ArmorRaisingRare",
            SpellCategory::ArmorValueRaisingRare => "ArmorValueRaisingRare",
            SpellCategory::AssessMonsterRaisingRare => "AssessMonsterRaisingRare",
            SpellCategory::AssessPersonRaisingRare => "AssessPersonRaisingRare",
            SpellCategory::AttackModRaisingRare => "AttackModRaisingRare",
            SpellCategory::AxeRaisingRare => "AxeRaisingRare",
            SpellCategory::BludgeonProtectionRare => "BludgeonProtectionRare",
            SpellCategory::BludgeonResistanceRaisingRare => "BludgeonResistanceRaisingRare",
            SpellCategory::BowRaisingRare => "BowRaisingRare",
            SpellCategory::ColdProtectionRare => "ColdProtectionRare",
            SpellCategory::ColdResistanceRaisingRare => "ColdResistanceRaisingRare",
            SpellCategory::CookingRaisingRare => "CookingRaisingRare",
            SpellCategory::CoordinationRaisingRare => "CoordinationRaisingRare",
            SpellCategory::CreatureEnchantmentRaisingRare => "CreatureEnchantmentRaisingRare",
            SpellCategory::CrossbowRaisingRare => "CrossbowRaisingRare",
            SpellCategory::DaggerRaisingRare => "DaggerRaisingRare",
            SpellCategory::DamageRaisingRare => "DamageRaisingRare",
            SpellCategory::DeceptionRaisingRare => "DeceptionRaisingRare",
            SpellCategory::DefenseModRaisingRare => "DefenseModRaisingRare",
            SpellCategory::ElectricProtectionRare => "ElectricProtectionRare",
            SpellCategory::ElectricResistanceRaisingRare => "ElectricResistanceRaisingRare",
            SpellCategory::EnduranceRaisingRare => "EnduranceRaisingRare",
            SpellCategory::FireProtectionRare => "FireProtectionRare",
            SpellCategory::FireResistanceRaisingRare => "FireResistanceRaisingRare",
            SpellCategory::FletchingRaisingRare => "FletchingRaisingRare",
            SpellCategory::FocusRaisingRare => "FocusRaisingRare",
            SpellCategory::HealingRaisingRare => "HealingRaisingRare",
            SpellCategory::HealthAcceleratingRare => "HealthAcceleratingRare",
            SpellCategory::ItemEnchantmentRaisingRare => "ItemEnchantmentRaisingRare",
            SpellCategory::JumpRaisingRare => "JumpRaisingRare",
            SpellCategory::LeadershipRaisingRare => "LeadershipRaisingRare",
            SpellCategory::LifeMagicRaisingRare => "LifeMagicRaisingRare",
            SpellCategory::LockpickRaisingRare => "LockpickRaisingRare",
            SpellCategory::LoyaltyRaisingRare => "LoyaltyRaisingRare",
            SpellCategory::MaceRaisingRare => "MaceRaisingRare",
            SpellCategory::MagicDefenseRaisingRare => "MagicDefenseRaisingRare",
            SpellCategory::ManaAcceleratingRare => "ManaAcceleratingRare",
            SpellCategory::ManaConversionRaisingRare => "ManaConversionRaisingRare",
            SpellCategory::MeleeDefenseRaisingRare => "MeleeDefenseRaisingRare",
            SpellCategory::MissileDefenseRaisingRare => "MissileDefenseRaisingRare",
            SpellCategory::PierceProtectionRare => "PierceProtectionRare",
            SpellCategory::PierceResistanceRaisingRare => "PierceResistanceRaisingRare",
            SpellCategory::QuicknessRaisingRare => "QuicknessRaisingRare",
            SpellCategory::RunRaisingRare => "RunRaisingRare",
            SpellCategory::SelfRaisingRare => "SelfRaisingRare",
            SpellCategory::SlashProtectionRare => "SlashProtectionRare",
            SpellCategory::SlashResistanceRaisingRare => "SlashResistanceRaisingRare",
            SpellCategory::SpearRaisingRare => "SpearRaisingRare",
            SpellCategory::StaffRaisingRare => "StaffRaisingRare",
            SpellCategory::StaminaAcceleratingRare => "StaminaAcceleratingRare",
            SpellCategory::StrengthRaisingRare => "StrengthRaisingRare",
            SpellCategory::SwordRaisingRare => "SwordRaisingRare",
            SpellCategory::ThrownWeaponsRaisingRare => "ThrownWeaponsRaisingRare",
            SpellCategory::UnarmedCombatRaisingRare => "UnarmedCombatRaisingRare",
            SpellCategory::WarMagicRaisingRare => "WarMagicRaisingRare",
            SpellCategory::WeaponTimeRaisingRare => "WeaponTimeRaisingRare",
            SpellCategory::ArmorIncreaseInkyArmor => "ArmorIncreaseInkyArmor",
            SpellCategory::MagicDefenseShelterRaisingFiun => "MagicDefenseShelterRaisingFiun",
            SpellCategory::ExtraRunSkillRaisingFiun => "ExtraRunSkillRaisingFiun",
            SpellCategory::ExtraManaConversionSkillRaisingFiun => "ExtraManaConversionSkillRaisingFiun",
            SpellCategory::AttributesIncreaseCantrip1 => "AttributesIncreaseCantrip1",
            SpellCategory::ExtraMeleeDefenseSkillRaising2 => "ExtraMeleeDefenseSkillRaising2",
            SpellCategory::ACTDPurchaseRewardSpell => "ACTDPurchaseRewardSpell",
            SpellCategory::ACTDPurchaseRewardSpellHealth => "ACTDPurchaseRewardSpellHealth",
            SpellCategory::SaltAshAttackModRaising => "SaltAshAttackModRaising",
            SpellCategory::QuicknessIncrease2 => "QuicknessIncrease2",
            SpellCategory::ExtraAlchemySkillRaising2 => "ExtraAlchemySkillRaising2",
            SpellCategory::ExtraCookingSkillRaising2 => "ExtraCookingSkillRaising2",
            SpellCategory::ExtraFletchingSkillRaising2 => "ExtraFletchingSkillRaising2",
            SpellCategory::ExtraLockpickSkillRaising2 => "ExtraLockpickSkillRaising2",
            SpellCategory::MucorManaWell => "MucorManaWell",
            SpellCategory::StaminaRestoring2 => "StaminaRestoring2",
            SpellCategory::AllegianceRaising => "AllegianceRaising",
            SpellCategory::HealthDoT => "HealthDoT",
            SpellCategory::HealthDoTSecondary => "HealthDoTSecondary",
            SpellCategory::HealthDoTTertiary => "HealthDoTTertiary",
            SpellCategory::HealthHoT => "HealthHoT",
            SpellCategory::HealthHoTSecondary => "HealthHoTSecondary",
            SpellCategory::HealthHoTTertiary => "HealthHoTTertiary",
            SpellCategory::HealthDivideSecondary => "HealthDivideSecondary",
            SpellCategory::HealthDivideTertiary => "HealthDivideTertiary",
            SpellCategory::SetSwordRaising => "SetSwordRaising",
            SpellCategory::SetAxeRaising => "SetAxeRaising",
            SpellCategory::SetDaggerRaising => "SetDaggerRaising",
            SpellCategory::SetMaceRaising => "SetMaceRaising",
            SpellCategory::SetSpearRaising => "SetSpearRaising",
            SpellCategory::SetStaffRaising => "SetStaffRaising",
            SpellCategory::SetUnarmedRaising => "SetUnarmedRaising",
            SpellCategory::SetBowRaising => "SetBowRaising",
            SpellCategory::SetCrossbowRaising => "SetCrossbowRaising",
            SpellCategory::SetThrownRaising => "SetThrownRaising",
            SpellCategory::SetItemEnchantmentRaising => "SetItemEnchantmentRaising",
            SpellCategory::SetCreatureEnchantmentRaising => "SetCreatureEnchantmentRaising",
            SpellCategory::SetWarMagicRaising => "SetWarMagicRaising",
            SpellCategory::SetLifeMagicRaising => "SetLifeMagicRaising",
            SpellCategory::SetMeleeDefenseRaising => "SetMeleeDefenseRaising",
            SpellCategory::SetMissileDefenseRaising => "SetMissileDefenseRaising",
            SpellCategory::SetMagicDefenseRaising => "SetMagicDefenseRaising",
            SpellCategory::SetStaminaAccelerating => "SetStaminaAccelerating",
            SpellCategory::SetCookingRaising => "SetCookingRaising",
            SpellCategory::SetFletchingRaising => "SetFletchingRaising",
            SpellCategory::SetLockpickRaising => "SetLockpickRaising",
            SpellCategory::SetAlchemyRaising => "SetAlchemyRaising",
            SpellCategory::SetSalvagingRaising => "SetSalvagingRaising",
            SpellCategory::SetArmorExpertiseRaising => "SetArmorExpertiseRaising",
            SpellCategory::SetWeaponExpertiseRaising => "SetWeaponExpertiseRaising",
            SpellCategory::SetItemTinkeringRaising => "SetItemTinkeringRaising",
            SpellCategory::SetMagicItemExpertiseRaising => "SetMagicItemExpertiseRaising",
            SpellCategory::SetLoyaltyRaising => "SetLoyaltyRaising",
            SpellCategory::SetStrengthRaising => "SetStrengthRaising",
            SpellCategory::SetEnduranceRaising => "SetEnduranceRaising",
            SpellCategory::SetCoordinationRaising => "SetCoordinationRaising",
            SpellCategory::SetQuicknessRaising => "SetQuicknessRaising",
            SpellCategory::SetFocusRaising => "SetFocusRaising",
            SpellCategory::SetWillpowerRaising => "SetWillpowerRaising",
            SpellCategory::SetHealthRaising => "SetHealthRaising",
            SpellCategory::SetStaminaRaising => "SetStaminaRaising",
            SpellCategory::SetManaRaising => "SetManaRaising",
            SpellCategory::SetSprintRaising => "SetSprintRaising",
            SpellCategory::SetJumpingRaising => "SetJumpingRaising",
            SpellCategory::SetSlashResistanceRaising => "SetSlashResistanceRaising",
            SpellCategory::SetBludgeonResistanceRaising => "SetBludgeonResistanceRaising",
            SpellCategory::SetPierceResistanceRaising => "SetPierceResistanceRaising",
            SpellCategory::SetFlameResistanceRaising => "SetFlameResistanceRaising",
            SpellCategory::SetAcidResistanceRaising => "SetAcidResistanceRaising",
            SpellCategory::SetFrostResistanceRaising => "SetFrostResistanceRaising",
            SpellCategory::SetLightningResistanceRaising => "SetLightningResistanceRaising",
            SpellCategory::CraftingLockPickRaising => "CraftingLockPickRaising",
            SpellCategory::CraftingFletchingRaising => "CraftingFletchingRaising",
            SpellCategory::CraftingCookingRaising => "CraftingCookingRaising",
            SpellCategory::CraftingAlchemyRaising => "CraftingAlchemyRaising",
            SpellCategory::CraftingArmorTinkeringRaising => "CraftingArmorTinkeringRaising",
            SpellCategory::CraftingWeaponTinkeringRaising => "CraftingWeaponTinkeringRaising",
            SpellCategory::CraftingMagicTinkeringRaising => "CraftingMagicTinkeringRaising",
            SpellCategory::CraftingItemTinkeringRaising => "CraftingItemTinkeringRaising",
            SpellCategory::SkillPercentAlchemyRaising => "SkillPercentAlchemyRaising",
            SpellCategory::TwoHandedRaising => "TwoHandedRaising",
            SpellCategory::TwoHandedLowering => "TwoHandedLowering",
            SpellCategory::ExtraTwoHandedSkillRaising => "ExtraTwoHandedSkillRaising",
            SpellCategory::ExtraTwoHandedSkillLowering => "ExtraTwoHandedSkillLowering",
            SpellCategory::ExtraTwoHandedSkillRaising2 => "ExtraTwoHandedSkillRaising2",
            SpellCategory::TwoHandedRaisingRare => "TwoHandedRaisingRare",
            SpellCategory::SetTwoHandedRaising => "SetTwoHandedRaising",
            SpellCategory::GearCraftRaising => "GearCraftRaising",
            SpellCategory::GearCraftLowering => "GearCraftLowering",
            SpellCategory::ExtraGearCraftSkillRaising => "ExtraGearCraftSkillRaising",
            SpellCategory::ExtraGearCraftSkillLowering => "ExtraGearCraftSkillLowering",
            SpellCategory::ExtraGearCraftSkillRaising2 => "ExtraGearCraftSkillRaising2",
            SpellCategory::GearCraftRaisingRare => "GearCraftRaisingRare",
            SpellCategory::SetGearCraftRaising => "SetGearCraftRaising",
            SpellCategory::LoyaltyManaRaising => "LoyaltyManaRaising",
            SpellCategory::LoyaltyStaminaRaising => "LoyaltyStaminaRaising",
            SpellCategory::LeadershipHealthRaising => "LeadershipHealthRaising",
            SpellCategory::TrinketDamageRaising => "TrinketDamageRaising",
            SpellCategory::TrinketDamageLowering => "TrinketDamageLowering",
            SpellCategory::TrinketHealthRaising => "TrinketHealthRaising",
            SpellCategory::TrinketStaminaRaising => "TrinketStaminaRaising",
            SpellCategory::TrinketManaRaising => "TrinketManaRaising",
            SpellCategory::TrinketXPRaising => "TrinketXPRaising",
            SpellCategory::DeceptionArcaneLoreRaising => "DeceptionArcaneLoreRaising",
            SpellCategory::HealOverTimeRaising => "HealOverTimeRaising",
            SpellCategory::DamageOverTimeRaising => "DamageOverTimeRaising",
            SpellCategory::HealingResistRatingRaising => "HealingResistRatingRaising",
            SpellCategory::AetheriaDamageRatingRaising => "AetheriaDamageRatingRaising",
            SpellCategory::AetheriaDamageReductionRaising => "AetheriaDamageReductionRaising",
            SpellCategory::AetheriaHealthRaising => "AetheriaHealthRaising",
            SpellCategory::AetheriaStaminaRaising => "AetheriaStaminaRaising",
            SpellCategory::AetheriaManaRaising => "AetheriaManaRaising",
            SpellCategory::AetheriaCriticalDamageRaising => "AetheriaCriticalDamageRaising",
            SpellCategory::AetheriaHealingAmplificationRaising => "AetheriaHealingAmplificationRaising",
            SpellCategory::AetheriaProcDamageRatingRaising => "AetheriaProcDamageRatingRaising",
            SpellCategory::AetheriaProcDamageReductionRaising => "AetheriaProcDamageReductionRaising",
            SpellCategory::AetheriaProcHealthOverTimeRaising => "AetheriaProcHealthOverTimeRaising",
            SpellCategory::AetheriaProcDamageOverTimeRaising => "AetheriaProcDamageOverTimeRaising",
            SpellCategory::AetheriaProcHealingReductionRaising => "AetheriaProcHealingReductionRaising",
            SpellCategory::RareDamageRatingRaising => "RareDamageRatingRaising",
            SpellCategory::RareDamageReductionRatingRaising => "RareDamageReductionRatingRaising",
            SpellCategory::AetheriaEnduranceRaising => "AetheriaEnduranceRaising",
            SpellCategory::NetherDamageOverTimeRaising => "NetherDamageOverTimeRaising",
            SpellCategory::NetherDamageOverTimeRaising2 => "NetherDamageOverTimeRaising2",
            SpellCategory::NetherDamageOverTimeRaising3 => "NetherDamageOverTimeRaising3",
            SpellCategory::NetherStreak => "NetherStreak",
            SpellCategory::NetherMissile => "NetherMissile",
            SpellCategory::NetherRing => "NetherRing",
            SpellCategory::NetherDamageRatingLowering => "NetherDamageRatingLowering",
            SpellCategory::NetherDamageHealingReductionRaising => "NetherDamageHealingReductionRaising",
            SpellCategory::VoidMagicLowering => "VoidMagicLowering",
            SpellCategory::VoidMagicRaising => "VoidMagicRaising",
            SpellCategory::VoidMysticRaising => "VoidMysticRaising",
            SpellCategory::SetVoidMagicRaising => "SetVoidMagicRaising",
            SpellCategory::VoidMagicRaisingRare => "VoidMagicRaisingRare",
            SpellCategory::VoidMysticRaising2 => "VoidMysticRaising2",
            SpellCategory::LuminanceDamageRatingRaising => "LuminanceDamageRatingRaising",
            SpellCategory::LuminanceDamageReductionRaising => "LuminanceDamageReductionRaising",
            SpellCategory::LuminanceHealthRaising => "LuminanceHealthRaising",
            SpellCategory::AetheriaCriticalReductionRaising => "AetheriaCriticalReductionRaising",
            SpellCategory::ExtraMissileDefenseSkillRaising => "ExtraMissileDefenseSkillRaising",
            SpellCategory::ExtraMissileDefenseSkillLowering => "ExtraMissileDefenseSkillLowering",
            SpellCategory::ExtraMissileDefenseSkillRaising2 => "ExtraMissileDefenseSkillRaising2",
            SpellCategory::AetheriaHealthResistanceRaising => "AetheriaHealthResistanceRaising",
            SpellCategory::AetheriaDotResistanceRaising => "AetheriaDotResistanceRaising",
            SpellCategory::CloakSkillRaising => "CloakSkillRaising",
            SpellCategory::CloakAllSkillRaising => "CloakAllSkillRaising",
            SpellCategory::CloakMagicDefenseLowering => "CloakMagicDefenseLowering",
            SpellCategory::CloakMeleeDefenseLowering => "CloakMeleeDefenseLowering",
            SpellCategory::CloakMissileDefenseLowering => "CloakMissileDefenseLowering",
            SpellCategory::DirtyFightingLowering => "DirtyFightingLowering",
            SpellCategory::DirtyFightingRaising => "DirtyFightingRaising",
            SpellCategory::ExtraDirtyFightingRaising => "ExtraDirtyFightingRaising",
            SpellCategory::DualWieldLowering => "DualWieldLowering",
            SpellCategory::DualWieldRaising => "DualWieldRaising",
            SpellCategory::ExtraDualWieldRaising => "ExtraDualWieldRaising",
            SpellCategory::RecklessnessLowering => "RecklessnessLowering",
            SpellCategory::RecklessnessRaising => "RecklessnessRaising",
            SpellCategory::ExtraRecklessnessRaising => "ExtraRecklessnessRaising",
            SpellCategory::ShieldLowering => "ShieldLowering",
            SpellCategory::ShieldRaising => "ShieldRaising",
            SpellCategory::ExtraShieldRaising => "ExtraShieldRaising",
            SpellCategory::SneakAttackLowering => "SneakAttackLowering",
            SpellCategory::SneakAttackRaising => "SneakAttackRaising",
            SpellCategory::ExtraSneakAttackRaising => "ExtraSneakAttackRaising",
            SpellCategory::RareDirtyFightingRaising => "RareDirtyFightingRaising",
            SpellCategory::RareDualWieldRaising => "RareDualWieldRaising",
            SpellCategory::RareRecklessnessRaising => "RareRecklessnessRaising",
            SpellCategory::RareShieldRaising => "RareShieldRaising",
            SpellCategory::RareSneakAttackRaising => "RareSneakAttackRaising",
            SpellCategory::DFAttackSkillDebuff => "DFAttackSkillDebuff",
            SpellCategory::DFBleedDamage => "DFBleedDamage",
            SpellCategory::DFDefenseSkillDebuff => "DFDefenseSkillDebuff",
            SpellCategory::DFHealingDebuff => "DFHealingDebuff",
            SpellCategory::SetDirtyFightingRaising => "SetDirtyFightingRaising",
            SpellCategory::SetDualWieldRaising => "SetDualWieldRaising",
            SpellCategory::SetRecklessnessRaising => "SetRecklessnessRaising",
            SpellCategory::SetShieldRaising => "SetShieldRaising",
            SpellCategory::SetSneakAttackRaising => "SetSneakAttackRaising",
            SpellCategory::LifeGiverMhoire => "LifeGiverMhoire",
            SpellCategory::RareDamageRatingRaising2 => "RareDamageRatingRaising2",
            SpellCategory::SpellDamageRaising => "SpellDamageRaising",
            SpellCategory::SummoningRaising => "SummoningRaising",
            SpellCategory::SummoningLowering => "SummoningLowering",
            SpellCategory::ExtraSummoningSkillRaising => "ExtraSummoningSkillRaising",
            SpellCategory::SetSummoningRaising => "SetSummoningRaising",
            SpellCategory::ParagonEnduranceRaising => "ParagonEnduranceRaising",
            SpellCategory::ParagonManaRaising => "ParagonManaRaising",
            SpellCategory::ParagonStaminaRaising => "ParagonStaminaRaising",
            SpellCategory::ParagonDirtyFightingRaising => "ParagonDirtyFightingRaising",
            SpellCategory::ParagonDualWieldRaising => "ParagonDualWieldRaising",
            SpellCategory::ParagonRecklessnessRaising => "ParagonRecklessnessRaising",
            SpellCategory::ParagonSneakAttackRaising => "ParagonSneakAttackRaising",
            SpellCategory::ParagonDamageRatingRaising => "ParagonDamageRatingRaising",
            SpellCategory::ParagonDamageReductionRatingRaising => "ParagonDamageReductionRatingRaising",
            SpellCategory::ParagonCriticalDamageRatingRaising => "ParagonCriticalDamageRatingRaising",
            SpellCategory::ParagonCriticalDamageReductionRatingRaising => "ParagonCriticalDamageReductionRatingRaising",
            SpellCategory::ParagonAxeRaising => "ParagonAxeRaising",
            SpellCategory::ParagonDaggerRaising => "ParagonDaggerRaising",
            SpellCategory::ParagonSwordRaising => "ParagonSwordRaising",
            SpellCategory::ParagonWarMagicRaising => "ParagonWarMagicRaising",
            SpellCategory::ParagonLifeMagicRaising => "ParagonLifeMagicRaising",
            SpellCategory::ParagonVoidMagicRaising => "ParagonVoidMagicRaising",
            SpellCategory::ParagonBowRaising => "ParagonBowRaising",
            SpellCategory::ParagonStrengthRaising => "ParagonStrengthRaising",
            SpellCategory::ParagonCoordinationRaising => "ParagonCoordinationRaising",
            SpellCategory::ParagonQuicknessRaising => "ParagonQuicknessRaising",
            SpellCategory::ParagonFocusRaising => "ParagonFocusRaising",
            SpellCategory::ParagonWillpowerRaising => "ParagonWillpowerRaising",
            SpellCategory::ParagonTwoHandedRaising => "ParagonTwoHandedRaising",
            SpellCategory::GauntletDamageReductionRatingRaising => "GauntletDamageReductionRatingRaising",
            SpellCategory::GauntletDamageRatingRaising => "GauntletDamageRatingRaising",
            SpellCategory::GauntletHealingRatingRaising => "GauntletHealingRatingRaising",
            SpellCategory::GauntletVitalityRaising => "GauntletVitalityRaising",
            SpellCategory::GauntletCriticalDamageRatingRaising => "GauntletCriticalDamageRatingRaising",
            SpellCategory::GauntletCriticalDamageReductionRatingRaising => "GauntletCriticalDamageReductionRatingRaising",
        };
        write!(f, "{}", s)
    }
}

/// Heritage of a player
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum HeritageGroup {
    Invalid = 0x0,
    Aluvian = 0x1,
    Gharundim = 0x2,
    Sho = 0x3,
    Viamontian = 0x4,
    Shadowbound = 0x5,
    Gearknight = 0x6,
    Tumerok = 0x7,
    Lugian = 0x8,
    Empyrean = 0x9,
    Penumbraen = 0xA,
    Undead = 0xB,
    Olthoi = 0xC,
    OlthoiAcid = 0xD,
}

impl crate::readers::ACDataType for HeritageGroup {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(HeritageGroup::try_from(value)?)
    }
}

impl crate::writers::ACWritable for HeritageGroup {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u8(writer, self.clone() as u8)?;
        Ok(())
    }
}

impl std::fmt::Display for HeritageGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HeritageGroup::Invalid => "Invalid",
            HeritageGroup::Aluvian => "Aluvian",
            HeritageGroup::Gharundim => "Gharundim",
            HeritageGroup::Sho => "Sho",
            HeritageGroup::Viamontian => "Viamontian",
            HeritageGroup::Shadowbound => "Shadowbound",
            HeritageGroup::Gearknight => "Gearknight",
            HeritageGroup::Tumerok => "Tumerok",
            HeritageGroup::Lugian => "Lugian",
            HeritageGroup::Empyrean => "Empyrean",
            HeritageGroup::Penumbraen => "Penumbraen",
            HeritageGroup::Undead => "Undead",
            HeritageGroup::Olthoi => "Olthoi",
            HeritageGroup::OlthoiAcid => "OlthoiAcid",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    /// the type of highlight (outline) applied to the object's icon
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct IconHighlight: u32 {
        const INVALID = 0x0;
        const MAGICAL = 0x1;
        const POISONED = 0x2;
        const BOOST_HEALTH = 0x4;
        const BOOST_MANA = 0x8;
        const BOOST_STAMINA = 0x10;
        const FIRE = 0x20;
        const LIGHTNING = 0x40;
        const FROST = 0x80;
        const ACID = 0x100;
        const BLUDGEONING = 0x200;
        const SLASHING = 0x400;
        const PIERCING = 0x800;
        const NETHER = 0x1000;
    }
}

impl crate::readers::ACDataType for IconHighlight {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(IconHighlight::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for IconHighlight {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CombatUse {
    None = 0x0,
    Melee = 0x1,
    Missile = 0x2,
    Ammo = 0x3,
    Shield = 0x4,
    TwoHanded = 0x5,
}

impl crate::readers::ACDataType for CombatUse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(CombatUse::try_from(value)?)
    }
}

impl crate::writers::ACWritable for CombatUse {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u8(writer, self.clone() as u8)?;
        Ok(())
    }
}

impl std::fmt::Display for CombatUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CombatUse::None => "None",
            CombatUse::Melee => "Melee",
            CombatUse::Missile => "Missile",
            CombatUse::Ammo => "Ammo",
            CombatUse::Shield => "Shield",
            CombatUse::TwoHanded => "TwoHanded",
        };
        write!(f, "{}", s)
    }
}

/// the type of wieldable item this is
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum WieldType {
    Invalid = 0x0,
    MeleeWeapon = 0x1,
    Armor = 0x2,
    Clothing = 0x4,
    Jewelry = 0x8,
}

impl crate::readers::ACDataType for WieldType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(WieldType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for WieldType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u8(writer, self.clone() as u8)?;
        Ok(())
    }
}

impl std::fmt::Display for WieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WieldType::Invalid => "Invalid",
            WieldType::MeleeWeapon => "MeleeWeapon",
            WieldType::Armor => "Armor",
            WieldType::Clothing => "Clothing",
            WieldType::Jewelry => "Jewelry",
        };
        write!(f, "{}", s)
    }
}

/// Chat channel type, for turbine chat
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ChatType {
    Undef = 0x0,
    Allegiance = 0x1,
    General = 0x2,
    Trade = 0x3,
    LFG = 0x4,
    Roleplay = 0x5,
    Society = 0x6,
    SocietyCelHan = 0x7,
    SocietyEldWeb = 0x8,
    SocietyRadBlo = 0x9,
    Olthoi = 0xA,
}

impl crate::readers::ACDataType for ChatType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ChatType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ChatType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ChatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ChatType::Undef => "Undef",
            ChatType::Allegiance => "Allegiance",
            ChatType::General => "General",
            ChatType::Trade => "Trade",
            ChatType::LFG => "LFG",
            ChatType::Roleplay => "Roleplay",
            ChatType::Society => "Society",
            ChatType::SocietyCelHan => "SocietyCelHan",
            ChatType::SocietyEldWeb => "SocietyEldWeb",
            ChatType::SocietyRadBlo => "SocietyRadBlo",
            ChatType::Olthoi => "Olthoi",
        };
        write!(f, "{}", s)
    }
}

/// The ChatDisplayMask identifies that types of chat that are displayed in each chat window. 
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ChatDisplayMask {
    Gameplay = 0x3912021,
    Mandatory = 0xC302,
    AreaChat = 0x1004,
    Tells = 0x18,
    Combat = 0x600040,
    Magic = 0x20080,
    Allegiance = 0x40C00,
    Fellowship = 0x80000,
    Errors = 0x4000000,
    TradeChannel = 0x10000000,
    LFGChannel = 0x20000000,
    RoleplayChannel = 0x40000000,
}

impl crate::readers::ACDataType for ChatDisplayMask {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ChatDisplayMask::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ChatDisplayMask {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ChatDisplayMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ChatDisplayMask::Gameplay => "Gameplay",
            ChatDisplayMask::Mandatory => "Mandatory",
            ChatDisplayMask::AreaChat => "AreaChat",
            ChatDisplayMask::Tells => "Tells",
            ChatDisplayMask::Combat => "Combat",
            ChatDisplayMask::Magic => "Magic",
            ChatDisplayMask::Allegiance => "Allegiance",
            ChatDisplayMask::Fellowship => "Fellowship",
            ChatDisplayMask::Errors => "Errors",
            ChatDisplayMask::TradeChannel => "TradeChannel",
            ChatDisplayMask::LFGChannel => "LFGChannel",
            ChatDisplayMask::RoleplayChannel => "RoleplayChannel",
        };
        write!(f, "{}", s)
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EnchantmentTypeFlags: u32 {
        const UNDEF = 0x0;
        const ATTRIBUTE = 0x1;
        const SECOND_ATT = 0x2;
        const INT = 0x4;
        const FLOAT = 0x8;
        const SKILL = 0x10;
        const BODY_DAMAGE_VALUE = 0x20;
        const BODY_DAMAGE_VARIANCE = 0x40;
        const BODY_ARMOR_VALUE = 0x80;
        const SINGLE_STAT = 0x1000;
        const MULTIPLE_STAT = 0x2000;
        const MULTIPLICATIVE = 0x4000;
        const ADDITIVE = 0x8000;
        const ATTACK_SKILLS = 0x10000;
        const DEFENSE_SKILLS = 0x20000;
        const MULTIPLICATIVE_DEGRADE = 0x100000;
        const ADDITIVE_DEGRADE = 0x200000;
        const VITAE = 0x800000;
        const COOLDOWN = 0x1000000;
        const BENEFICIAL = 0x2000000;
        const STAT_TYPES = 0xFF;
    }
}

impl crate::readers::ACDataType for EnchantmentTypeFlags {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EnchantmentTypeFlags::from_bits_retain(value))
    }
}

impl crate::writers::ACWritable for EnchantmentTypeFlags {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.bits())?;
        Ok(())
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ParentLocation {
    None = 0x0,
    RightHand = 0x1,
    LeftHand = 0x2,
    Shield = 0x3,
    Belt = 0x4,
    Quiver = 0x5,
    Hearldry = 0x6,
    Mouth = 0x7,
    LeftWeapon = 0x8,
    LeftUnarmed = 0x9,
}

impl crate::readers::ACDataType for ParentLocation {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ParentLocation::try_from(value)?)
    }
}

impl crate::writers::ACWritable for ParentLocation {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for ParentLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ParentLocation::None => "None",
            ParentLocation::RightHand => "RightHand",
            ParentLocation::LeftHand => "LeftHand",
            ParentLocation::Shield => "Shield",
            ParentLocation::Belt => "Belt",
            ParentLocation::Quiver => "Quiver",
            ParentLocation::Hearldry => "Hearldry",
            ParentLocation::Mouth => "Mouth",
            ParentLocation::LeftWeapon => "LeftWeapon",
            ParentLocation::LeftUnarmed => "LeftUnarmed",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum Placement {
    Default = 0x0,
    RightHandCombat = 0x1,
    RightHandNonCombat = 0x2,
    LeftHand = 0x3,
    Belt = 0x4,
    Quiver = 0x5,
    Shield = 0x6,
    LeftWeapon = 0x7,
    LeftUnarmed = 0x8,
    SpecialCrowssbowBolt = 0x33,
    MissileFlight = 0x34,
    Resting = 0x65,
    Other = 0x66,
    Hook = 0x67,
    Random1 = 0x79,
    Random2 = 0x7A,
    Random3 = 0x7B,
    Random4 = 0x7C,
    Random5 = 0x7D,
    Random6 = 0x7E,
    Random7 = 0x7F,
    Random8 = 0x80,
    Random9 = 0x81,
    Random10 = 0x82,
    XXXUnknownA = 0xA,
    XXXUnknownF = 0xF,
    XXXUnknown14 = 0x14,
    XXXUnknown1E = 0x1E,
    XXXUnknown20 = 0x20,
    XXXUnknown3C = 0x3C,
    XXXUnknown69 = 0x69,
    XXXUnknown6A = 0x6A,
    XXXUnknown63 = 0x63,
    XXXUnknown68 = 0x68,
    XXXUnknown78 = 0x78,
    XXXUnknown84 = 0x84,
    XXXUnknownF0 = 0xF0,
    XXXUnknown3F2 = 0x3F2,
}

impl crate::readers::ACDataType for Placement {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(Placement::try_from(value)?)
    }
}

impl crate::writers::ACWritable for Placement {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for Placement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Placement::Default => "Default",
            Placement::RightHandCombat => "RightHandCombat",
            Placement::RightHandNonCombat => "RightHandNonCombat",
            Placement::LeftHand => "LeftHand",
            Placement::Belt => "Belt",
            Placement::Quiver => "Quiver",
            Placement::Shield => "Shield",
            Placement::LeftWeapon => "LeftWeapon",
            Placement::LeftUnarmed => "LeftUnarmed",
            Placement::SpecialCrowssbowBolt => "SpecialCrowssbowBolt",
            Placement::MissileFlight => "MissileFlight",
            Placement::Resting => "Resting",
            Placement::Other => "Other",
            Placement::Hook => "Hook",
            Placement::Random1 => "Random1",
            Placement::Random2 => "Random2",
            Placement::Random3 => "Random3",
            Placement::Random4 => "Random4",
            Placement::Random5 => "Random5",
            Placement::Random6 => "Random6",
            Placement::Random7 => "Random7",
            Placement::Random8 => "Random8",
            Placement::Random9 => "Random9",
            Placement::Random10 => "Random10",
            Placement::XXXUnknownA => "XXXUnknownA",
            Placement::XXXUnknownF => "XXXUnknownF",
            Placement::XXXUnknown14 => "XXXUnknown14",
            Placement::XXXUnknown1E => "XXXUnknown1E",
            Placement::XXXUnknown20 => "XXXUnknown20",
            Placement::XXXUnknown3C => "XXXUnknown3C",
            Placement::XXXUnknown69 => "XXXUnknown69",
            Placement::XXXUnknown6A => "XXXUnknown6A",
            Placement::XXXUnknown63 => "XXXUnknown63",
            Placement::XXXUnknown68 => "XXXUnknown68",
            Placement::XXXUnknown78 => "XXXUnknown78",
            Placement::XXXUnknown84 => "XXXUnknown84",
            Placement::XXXUnknownF0 => "XXXUnknownF0",
            Placement::XXXUnknown3F2 => "XXXUnknown3F2",
        };
        write!(f, "{}", s)
    }
}

/// Message queue types from protocol.xml
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageQueue {
    CLCache,
    Control,
    Logon,
    SmartBox,
    UIQueue,
    Weenie,
}

