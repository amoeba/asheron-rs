use serde::{Serialize, Deserialize};
use num_enum::TryFromPrimitive;
use crate::readers::ACReader;

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

/// Flags that dictate what property tables are included with the ACBaseQuali
bitflags::bitflags! {
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

/// The PositionFlags value defines the fields present in the Position structure.
bitflags::bitflags! {
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

/// The objects type information
bitflags::bitflags! {
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

/// The CharacterOptions1 word contains character options.
bitflags::bitflags! {
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

/// The CharacterOptions2 word contains additional character options.
bitflags::bitflags! {
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

/// The various options for filtering the spellbook
bitflags::bitflags! {
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

/// The EquipMask value describes the equipment slots an item uses.
bitflags::bitflags! {
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

/// The type of the friend change event.
bitflags::bitflags! {
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

/// The combat mode for a character or monster.
bitflags::bitflags! {
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

/// Flags related to the use of the item.
bitflags::bitflags! {
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

/// The AmmoType value describes the type of ammunition a missile weapon uses.
bitflags::bitflags! {
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

/// The useablilty flags of the object
bitflags::bitflags! {
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

/// The CoverageMask value describes what parts of the body an item protects.
bitflags::bitflags! {
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

/// The HookType identifies the types of dwelling hooks.
bitflags::bitflags! {
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

/// The state flags for an object
bitflags::bitflags! {
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

/// The AttributeMask selects which creature attributes highlighting is applied to.
bitflags::bitflags! {
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

/// The DamageType identifies the type of damage.
bitflags::bitflags! {
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

/// The HookAppraisalFlags identifies various properties for an item hooked.
bitflags::bitflags! {
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

/// The ArmorHighlightMask selects which armor attributes highlighting is applied to.
bitflags::bitflags! {
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

/// The ResistHighlightMask selects which wand attributes highlighting is applied to.
bitflags::bitflags! {
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

/// The WeaponHighlightMask selects which weapon attributes highlighting is applied to.
bitflags::bitflags! {
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

/// Additional attack information
bitflags::bitflags! {
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

/// Indicates what data is present in the ACQualities data
bitflags::bitflags! {
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

/// Chat channels
bitflags::bitflags! {
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

/// Flags that determine what data is contained in the EnchantmentRegistry
bitflags::bitflags! {
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

/// the type of highlight (outline) applied to the object's icon
bitflags::bitflags! {
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

/// Message queue types from protocol.xml
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageQueue {
    ClCache,
    Control,
    Logon,
    SmartBox,
    UiQueue,
    Weenie,
}

