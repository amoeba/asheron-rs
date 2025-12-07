use serde::{Serialize, Deserialize};
use num_enum::TryFromPrimitive;

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum PacketHeaderFlags {
    None = 0x0,
    Retransmission = 0x1,
    EncryptedChecksum = 0x2,
    BlobFragments = 0x4,
    ServerSwitch = 0x100,
    LogonServerAddr = 0x200,
    EmptyHeader1 = 0x400,
    Referral = 0x800,
    RequestRetransmit = 0x1000,
    RejectRetransmit = 0x2000,
    AckSequence = 0x4000,
    Disconnect = 0x8000,
    LoginRequest = 0x10000,
    WorldLoginRequest = 0x20000,
    ConnectRequest = 0x40000,
    ConnectResponse = 0x80000,
    NetError = 0x100000,
    NetErrorDisconnect = 0x200000,
    CICMDCommand = 0x400000,
    TimeSync = 0x1000000,
    EchoRequest = 0x2000000,
    EchoResponse = 0x4000000,
    Flow = 0x8000000,
}

impl crate::readers::ACDataType for PacketHeaderFlags {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PacketHeaderFlags::try_from(value)?)
    }
}

#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum FragmentGroup {
}

impl crate::readers::ACDataType for FragmentGroup {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(FragmentGroup::try_from(value)?)
    }
}

/// The type of server to switch
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ServerSwitchType {
}

impl crate::readers::ACDataType for ServerSwitchType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(GameAction::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum WeenieType {
}

impl crate::readers::ACDataType for WeenieType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(WeenieType::try_from(value)?)
    }
}

/// Flags that dictate what property tables are included with the ACBaseQuali
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ACBaseQualitiesFlags {
    None = 0x0,
    PropertyInt = 0x1,
    PropertyBool = 0x2,
    PropertyFloat = 0x4,
    PropertyDataId = 0x8,
    PropertyString = 0x10,
    PropertyPosition = 0x20,
    PropertyInstanceId = 0x40,
    PropertyInt64 = 0x80,
}

impl crate::readers::ACDataType for ACBaseQualitiesFlags {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ACBaseQualitiesFlags::try_from(value)?)
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(WeenieError::try_from(value)?)
    }
}

/// The PositionFlags value defines the fields present in the Position structure.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum PositionFlags {
    HasVelocity = 0x1,
    HasPlacementId = 0x2,
    IsGrounded = 0x4,
    OrientationHasNoW = 0x8,
    OrientationHasNoX = 0x10,
    OrientationHasNoY = 0x20,
    OrientationHasNoZ = 0x40,
}

impl crate::readers::ACDataType for PositionFlags {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PositionFlags::try_from(value)?)
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AttackType::try_from(value)?)
    }
}

/// The objects type information
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ItemType {
    MeleeWeapon = 0x1,
    Armor = 0x2,
    Clothing = 0x4,
    Jewelry = 0x8,
    Creature = 0x10,
    Food = 0x20,
    Money = 0x40,
    Misc = 0x80,
    MissileWeapon = 0x100,
    Container = 0x200,
    Useless = 0x400,
    Gem = 0x800,
    SpellComponents = 0x1000,
    Writable = 0x2000,
    Key = 0x4000,
    Caster = 0x8000,
    Portal = 0x10000,
    Lockable = 0x20000,
    PromissoryNote = 0x40000,
    ManaStone = 0x80000,
    Service = 0x100000,
    MagicWieldable = 0x200000,
    CraftCookingBase = 0x400000,
    CraftAlchemyBase = 0x800000,
    CraftFletchingBase = 0x2000000,
    CraftAlchemyIntermediate = 0x4000000,
    CraftFletchingIntermediate = 0x8000000,
    LifeStone = 0x10000000,
    TinkeringTool = 0x20000000,
    TinkeringMaterial = 0x40000000,
    Gameboard = 0x80000000,
}

impl crate::readers::ACDataType for ItemType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ItemType::try_from(value)?)
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EmoteCategory::try_from(value)?)
    }
}

/// The CharacterOptions1 word contains character options.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CharacterOptions1 {
    AutoRepeatAttack = 0x2,
    IgnoreAllegianceRequests = 0x4,
    IgnoreFellowshipRequests = 0x8,
    NotUsed2 = 0x10,
    NotUsed3 = 0x20,
    AllowGive = 0x40,
    ViewCombatTarget = 0x80,
    ShowTooltips = 0x100,
    UseDeception = 0x200,
    ToggleRun = 0x400,
    StayInChatMode = 0x800,
    AdvancedCombatUI = 0x1000,
    AutoTarget = 0x2000,
    NotUsed4 = 0x4000,
    VividTargetingIndicator = 0x8000,
    DisableMostWeatherEffects = 0x10000,
    IgnoreTradeRequests = 0x20000,
    FellowshipShareXP = 0x40000,
    AcceptLootPermits = 0x80000,
    FellowshipShareLoot = 0x100000,
    SideBySideVitals = 0x200000,
    CoordinatesOnRadar = 0x400000,
    SpellDuration = 0x800000,
    NotUsed5 = 0x1000000,
    DisableHouseRestrictionEffects = 0x2000000,
    DragItemOnPlayerOpensSecureTrade = 0x4000000,
    DisplayAllegianceLogonNotifications = 0x8000000,
    UseChargeAttack = 0x10000000,
    AutoAcceptFellowRequest = 0x20000000,
    HearAllegianceChat = 0x40000000,
    UseCraftSuccessDialog = 0x80000000,
}

impl crate::readers::ACDataType for CharacterOptions1 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CharacterOptions1::try_from(value)?)
    }
}

/// The CharacterOptions2 word contains additional character options.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CharacterOptions2 {
    PersistentAtDay = 0x1,
    DisplayDateOfBirth = 0x2,
    DisplayChessRank = 0x4,
    DisplayFishingSkill = 0x8,
    DisplayNumberDeaths = 0x10,
    DisplayAge = 0x20,
    TimeStamp = 0x40,
    SalvageMultiple = 0x80,
    HearGeneralChat = 0x100,
    HearTradeChat = 0x200,
    HearLFGChat = 0x400,
    HearRoleplayChat = 0x800,
    AppearOffline = 0x1000,
    DisplayNumberCharacterTitles = 0x2000,
    MainPackPreferred = 0x4000,
    LeadMissileTargets = 0x8000,
    UseFastMissiles = 0x10000,
    FilterLanguage = 0x20000,
    ConfirmVolatileRareUse = 0x40000,
    HearSocietyChat = 0x80000,
    ShowHelm = 0x100000,
    DisableDistanceFog = 0x200000,
    UseMouseTurning = 0x400000,
    ShowCloak = 0x800000,
    LockUI = 0x1000000,
    HearPKDeath = 0x2000000,
}

impl crate::readers::ACDataType for CharacterOptions2 {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CharacterOptions2::try_from(value)?)
    }
}

/// The various options for filtering the spellbook
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum SpellBookFilterOptions {
    None = 0x0,
    Creature = 0x1,
    Item = 0x2,
    Life = 0x4,
    War = 0x8,
    Level1 = 0x10,
    Level2 = 0x20,
    Level3 = 0x40,
    Level4 = 0x80,
    Level5 = 0x100,
    Level6 = 0x200,
    Level7 = 0x400,
    Level8 = 0x800,
    Level9 = 0x1000,
    Void = 0x2000,
}

impl crate::readers::ACDataType for SpellBookFilterOptions {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(SpellBookFilterOptions::try_from(value)?)
    }
}

/// The EquipMask value describes the equipment slots an item uses.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum EquipMask {
    Head = 0x1,
    ChestUnderwear = 0x2,
    AbdomenUnderwear = 0x4,
    UpperArmsUnderwear = 0x8,
    LowerArmsUnderwear = 0x10,
    Hands = 0x20,
    UpperLegsUnderwear = 0x40,
    LowerLegsUnderwear = 0x80,
    Feet = 0x100,
    Chest = 0x200,
    Abdomen = 0x400,
    UpperArms = 0x800,
    LowerArms = 0x1000,
    UpperLegs = 0x2000,
    LowerLegs = 0x4000,
    Necklace = 0x8000,
    RightBracelet = 0x10000,
    LeftBracelet = 0x20000,
    RightRing = 0x40000,
    LeftRing = 0x80000,
    MeleeWeapon = 0x100000,
    Shield = 0x200000,
    MissileWeapon = 0x400000,
    Ammunition = 0x800000,
    Wand = 0x1000000,
}

impl crate::readers::ACDataType for EquipMask {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EquipMask::try_from(value)?)
    }
}

/// The type of the friend change event.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum FriendsUpdateType {
    Full = 0x0,
    Added = 0x1,
    Removed = 0x2,
    LoginChange = 0x4,
}

impl crate::readers::ACDataType for FriendsUpdateType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(FriendsUpdateType::try_from(value)?)
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CurVitalId::try_from(value)?)
    }
}

/// The combat mode for a character or monster.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CombatMode {
    NonCombat = 0x1,
    Melee = 0x2,
    Missle = 0x4,
    Magic = 0x8,
}

impl crate::readers::ACDataType for CombatMode {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CombatMode::try_from(value)?)
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ChatFragmentType::try_from(value)?)
    }
}

/// Flags related to the use of the item.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ObjectDescriptionFlag {
    Openable = 0x1,
    Inscribable = 0x2,
    Stuck = 0x4,
    Player = 0x8,
    Attackable = 0x10,
    PlayerKiller = 0x20,
    HiddenAdmin = 0x40,
    UiHidden = 0x80,
    Book = 0x100,
    Vendor = 0x200,
    PkSwitch = 0x400,
    NpkSwitch = 0x800,
    Door = 0x1000,
    Corpse = 0x2000,
    LifeStone = 0x4000,
    Food = 0x8000,
    Healer = 0x10000,
    Lockpick = 0x20000,
    Portal = 0x40000,
    Admin = 0x100000,
    FreePkStatus = 0x200000,
    ImmuneCellRestrictions = 0x400000,
    RequiresPackSlot = 0x800000,
    Retained = 0x1000000,
    PkLiteStatus = 0x2000000,
    IncludesSecondHeader = 0x4000000,
    BindStone = 0x8000000,
    VolatileRare = 0x10000000,
    WieldOnUse = 0x20000000,
    WieldLeft = 0x40000000,
}

impl crate::readers::ACDataType for ObjectDescriptionFlag {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ObjectDescriptionFlag::try_from(value)?)
    }
}

/// The AmmoType value describes the type of ammunition a missile weapon uses.
#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AmmoType {
    ThrownWeapon = 0x0,
    Arrow = 0x1,
    Bolt = 0x2,
    Dart = 0x4,
}

impl crate::readers::ACDataType for AmmoType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(AmmoType::try_from(value)?)
    }
}

/// The useablilty flags of the object
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum Usable {
    SourceUnusable = 0x1,
    SourceSelf = 0x2,
    SourceWielded = 0x4,
    SourceContained = 0x8,
    SourceViewed = 0x10,
    SourceRemote = 0x20,
    SourceNoApproach = 0x40,
    SourceObjectSelf = 0x80,
    TargetUnusable = 0x10000,
    TargetSelf = 0x20000,
    TargetWielded = 0x40000,
    TargetContained = 0x80000,
    TargetViewed = 0x100000,
    TargetRemote = 0x200000,
    TargetNoApproach = 0x400000,
    TargetObjectSelf = 0x800000,
}

impl crate::readers::ACDataType for Usable {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(Usable::try_from(value)?)
    }
}

/// The CoverageMask value describes what parts of the body an item protects.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CoverageMask {
    UpperLegsUnderwear = 0x2,
    LowerLegsUnderwear = 0x4,
    ChestUnderwear = 0x8,
    AbdomenUnderwear = 0x10,
    UpperArmsUnderwear = 0x20,
    LowerArmsUnderwear = 0x40,
    UpperLegs = 0x100,
    LowerLegs = 0x200,
    Chest = 0x400,
    Abdomen = 0x800,
    UpperArms = 0x1000,
    LowerArms = 0x2000,
    Head = 0x4000,
    Hands = 0x8000,
    Feet = 0x10000,
}

impl crate::readers::ACDataType for CoverageMask {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CoverageMask::try_from(value)?)
    }
}

/// The HookType identifies the types of dwelling hooks.
#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum HookType {
    Floor = 0x1,
    Wall = 0x2,
    Ceiling = 0x4,
    Yard = 0x8,
    Roof = 0x10,
}

impl crate::readers::ACDataType for HookType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(HookType::try_from(value)?)
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EnvrionChangeType::try_from(value)?)
    }
}

/// The movement type defines the fields for the rest of the message
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum MovementType {
    InterpertedMotionState = 0x0,
    MoveToObject = 0x6,
    MoveToPosition = 0x7,
    TurnToObject = 0x8,
    TurnToPosition = 0x9,
}

impl crate::readers::ACDataType for MovementType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CharacterErrorType::try_from(value)?)
    }
}

/// The state flags for an object
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum PhysicsState {
    None = 0x0,
    Static = 0x1,
    Ethereal = 0x4,
    ReportCollision = 0x8,
    IgnoreCollision = 0x10,
    NoDraw = 0x20,
    Missle = 0x40,
    Pushable = 0x80,
    AlignPath = 0x100,
    PathClipped = 0x200,
    Gravity = 0x400,
    LightingOn = 0x800,
    ParticleEmitter = 0x1000,
    Hidden = 0x4000,
    ScriptedCollision = 0x8000,
    HasPhysicsBsp = 0x10000,
    Inelastic = 0x20000,
    HasDefaultAnim = 0x40000,
    HasDefaultScript = 0x80000,
    Cloaked = 0x100000,
    ReportCollisionAsEnvironment = 0x200000,
    EdgeSlide = 0x400000,
    Sledding = 0x800000,
    Frozen = 0x1000000,
}

impl crate::readers::ACDataType for PhysicsState {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PhysicsState::try_from(value)?)
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(CompressionType::try_from(value)?)
    }
}

/// The AttributeMask selects which creature attributes highlighting is applied to.
#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AttributeMask {
    Strength = 0x1,
    Endurance = 0x2,
    Quickness = 0x4,
    Coordination = 0x8,
    Focus = 0x10,
    #[serde(rename = "Self")]
    Self_ = 0x20,
    Health = 0x40,
    Stamina = 0x80,
    Mana = 0x100,
}

impl crate::readers::ACDataType for AttributeMask {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(AttributeMask::try_from(value)?)
    }
}

/// The DamageType identifies the type of damage.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum DamageType {
    Slashing = 0x1,
    Piercing = 0x2,
    Bludgeoning = 0x4,
    Cold = 0x8,
    Fire = 0x10,
    Acid = 0x20,
    Electric = 0x40,
}

impl crate::readers::ACDataType for DamageType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(DamageType::try_from(value)?)
    }
}

/// The HookAppraisalFlags identifies various properties for an item hooked.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum HookAppraisalFlags {
    Inscribable = 0x1,
    IsHealer = 0x2,
    IsLockpick = 0x8,
}

impl crate::readers::ACDataType for HookAppraisalFlags {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(HookAppraisalFlags::try_from(value)?)
    }
}

/// The ArmorHighlightMask selects which armor attributes highlighting is applied to.
#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ArmorHighlightMask {
    ArmorLevel = 0x1,
    SlashingProtection = 0x2,
    PiercingProtection = 0x4,
    BludgeoningProtection = 0x8,
    ColdProtection = 0x10,
    FireProtection = 0x20,
    AcidProtection = 0x40,
    ElectricalProtection = 0x80,
}

impl crate::readers::ACDataType for ArmorHighlightMask {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(ArmorHighlightMask::try_from(value)?)
    }
}

/// The ResistHighlightMask selects which wand attributes highlighting is applied to.
#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ResistHighlightMask {
    ResistSlash = 0x1,
    ResistPierce = 0x2,
    ResistBludgeon = 0x4,
    ResistFire = 0x8,
    ResistCold = 0x10,
    ResistAcid = 0x20,
    ResistElectric = 0x40,
    ResistHealthBoost = 0x80,
    ResistStaminaDrain = 0x100,
    ResistStaminaBoost = 0x200,
    ResistManaDrain = 0x400,
    ResistManaBoost = 0x800,
    ManaConversionMod = 0x1000,
    ElementalDamageMod = 0x2000,
    ResistNether = 0x4000,
}

impl crate::readers::ACDataType for ResistHighlightMask {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(ResistHighlightMask::try_from(value)?)
    }
}

/// The WeaponHighlightMask selects which weapon attributes highlighting is applied to.
#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum WeaponHighlightMask {
    AttackSkill = 0x1,
    MeleeDefense = 0x2,
    Speed = 0x4,
    Damage = 0x8,
    DamageVariance = 0x10,
    DamageMod = 0x20,
}

impl crate::readers::ACDataType for WeaponHighlightMask {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u16(reader)?;
        Ok(WeaponHighlightMask::try_from(value)?)
    }
}

/// Additional attack information
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AttackConditionsMask {
    CriticalProtectionAugmentation = 0x1,
    Recklessness = 0x2,
    SneakAttack = 0x4,
}

impl crate::readers::ACDataType for AttackConditionsMask {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AttackConditionsMask::try_from(value)?)
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(HouseType::try_from(value)?)
    }
}

/// Identifies the chess move attempt result.  Negative/0 values are failures.
#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ChessMoveResult {
    Success = 0x1,
    OpponentInCheck = 0x400,
    CheckMatedOpponent = 0x800,
}

impl crate::readers::ACDataType for ChessMoveResult {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(Gender::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum FactionBits {
    None = 0x0,
    CelestialHand = 0x1,
    EldrytchWeb = 0x2,
    RadiantBlood = 0x4,
}

impl crate::readers::ACDataType for FactionBits {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(FactionBits::try_from(value)?)
    }
}

/// Creature type
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CreatureType {
}

impl crate::readers::ACDataType for CreatureType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CreatureType::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum CombatStyle {
    Undef = 0x0,
    Unarmed = 0x1,
    OneHanded = 0x2,
    OneHandedAndShield = 0x4,
    TwoHanded = 0x8,
    Bow = 0x10,
    Crossbow = 0x20,
    Sling = 0x40,
    ThrownWeapon = 0x80,
    DualWield = 0x100,
    Magic = 0x200,
    Atlatl = 0x400,
    ThrownShield = 0x800,
    Reserved1 = 0x1000,
    Reserved2 = 0x2000,
    Reserved3 = 0x4000,
    Reserved4 = 0x8000,
    StubbornMagic = 0x10000,
    StubbornProjectile = 0x20000,
    StubbornMelee = 0x40000,
    StubbornMissile = 0x80000,
}

impl crate::readers::ACDataType for CombatStyle {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(CombatStyle::try_from(value)?)
    }
}

/// Indicates what data is present in the ACQualities data
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ACQualitiesFlags {
    Attributes = 0x1,
    Skills = 0x2,
    Body = 0x4,
    SpellBook = 0x100,
    Enchantments = 0x200,
    EventFilter = 0x8,
    Emotes = 0x10,
    CreationProfile = 0x20,
    PageData = 0x40,
    Generators = 0x80,
    GeneratorRegistry = 0x400,
    GeneratorQueue = 0x800,
}

impl crate::readers::ACDataType for ACQualitiesFlags {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ACQualitiesFlags::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum GeneratorDestruct {
}

impl crate::readers::ACDataType for GeneratorDestruct {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(GeneratorDestruct::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum GeneratorTimeType {
}

impl crate::readers::ACDataType for GeneratorTimeType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(GeneratorTimeType::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum GeneratorType {
}

impl crate::readers::ACDataType for GeneratorType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(GeneratorType::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ImbuedEffectType {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ImbuedEffectType::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ItemXpStyle {
}

impl crate::readers::ACDataType for ItemXpStyle {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ItemXpStyle::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum SubscriptionStatus {
}

impl crate::readers::ACDataType for SubscriptionStatus {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(SubscriptionStatus::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum WeaponType {
}

impl crate::readers::ACDataType for WeaponType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(WeaponType::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ActivationResponse {
    Use = 0x2,
    Animate = 0x4,
    Talk = 0x10,
    Emote = 0x800,
    CastSpell = 0x1000,
    Generate = 0x10000,
}

impl crate::readers::ACDataType for ActivationResponse {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ActivationResponse::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AetheriaBitfield {
    Blue = 0x1,
    Yellow = 0x2,
    Red = 0x4,
}

impl crate::readers::ACDataType for AetheriaBitfield {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(HookGroupType::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ArmorType {
}

impl crate::readers::ACDataType for ArmorType {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ArmorType::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum AttunedStatus {
}

impl crate::readers::ACDataType for AttunedStatus {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(AttunedStatus::try_from(value)?)
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum BondedStatus {
}

impl crate::readers::ACDataType for BondedStatus {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(BondedStatus::try_from(value)?)
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum HouseStatus {
}

impl crate::readers::ACDataType for HouseStatus {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(HouseStatus::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum UiEffects {
    Undef = 0x0,
    Magical = 0x1,
    Poisoned = 0x2,
    BoostHealth = 0x4,
    BoostMana = 0x8,
    BoostStamina = 0x10,
    Fire = 0x20,
    Lightning = 0x40,
    Frost = 0x80,
    Acid = 0x100,
    Bludgeoning = 0x200,
    Slashing = 0x400,
    Piercing = 0x800,
    Nether = 0x1000,
}

impl crate::readers::ACDataType for UiEffects {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(UiEffects::try_from(value)?)
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(PortalBitmask::try_from(value)?)
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum WieldRequirement {
}

impl crate::readers::ACDataType for WieldRequirement {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(WieldRequirement::try_from(value)?)
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum PaletteTemplate {
}

impl crate::readers::ACDataType for PaletteTemplate {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(PaletteTemplate::try_from(value)?)
    }
}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum SummoningMastery {
}

impl crate::readers::ACDataType for SummoningMastery {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_i32(reader)?;
        Ok(SummoningMastery::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum ContractId {
}

impl crate::readers::ACDataType for ContractId {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyInt64::try_from(value)?)
    }
}

/// The PropertyBool identifies a specific Character or Object boolean property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyBool {
}

impl crate::readers::ACDataType for PropertyBool {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyBool::try_from(value)?)
    }
}

/// The DataPropertyId identifies a specific Character or Object data property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyDataId {
}

impl crate::readers::ACDataType for PropertyDataId {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyDataId::try_from(value)?)
    }
}

/// The PropertyInt identifies a specific Character or Object int property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyInt {
}

impl crate::readers::ACDataType for PropertyInt {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyInt::try_from(value)?)
    }
}

/// The PropertyInstanceId identifies a specific Character or Object instance property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyInstanceId {
}

impl crate::readers::ACDataType for PropertyInstanceId {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyInstanceId::try_from(value)?)
    }
}

/// The PropertyPosition identifies a specific Character or Object position property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyPosition {
}

impl crate::readers::ACDataType for PropertyPosition {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyPosition::try_from(value)?)
    }
}

/// The PropertyString identifies a specific Character or Object string property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyString {
}

impl crate::readers::ACDataType for PropertyString {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyString::try_from(value)?)
    }
}

/// The PropertyFloat identifies a specific Character or Object float property.
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive, Hash, Eq)]
pub enum PropertyFloat {
}

impl crate::readers::ACDataType for PropertyFloat {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(PropertyFloat::try_from(value)?)
    }
}

/// Chat channels
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum Channel {
    Undef = 0x0,
    Abuse = 0x1,
    Admin = 0x2,
    Audit = 0x4,
    Advocate1 = 0x8,
    Advocate2 = 0x10,
    Advocate3 = 0x20,
    QA1 = 0x40,
    QA2 = 0x80,
    Debug = 0x100,
    Sentinel = 0x200,
    Help = 0x400,
    AllBroadcast = 0x401,
    Fellow = 0x800,
    Vassals = 0x1000,
    Patron = 0x2000,
    Monarch = 0x4000,
    AlArqas = 0x8000,
    Holtburg = 0x10000,
    Lytelthorpe = 0x20000,
    Nanto = 0x40000,
    Rithwic = 0x80000,
    Samsur = 0x100000,
    Shoushi = 0x200000,
    Yanshi = 0x400000,
    Yaraq = 0x800000,
    TownChans = 0xFF8000,
    CoVassals = 0x1000000,
    AllegianceBroadcast = 0x2000000,
    FellowBroadcast = 0x4000000,
    SocietyCelHanBroadcast = 0x8000000,
    SocietyEldWebBroadcast = 0x10000000,
    SocietyRadBloBroadcast = 0x20000000,
    Olthoi = 0x40000000,
    GhostChans = 0x7F007800,
}

impl crate::readers::ACDataType for Channel {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(Channel::try_from(value)?)
    }
}

/// Equipment Set Ids
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum EquipmentSet {
}

impl crate::readers::ACDataType for EquipmentSet {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(RadarColor::try_from(value)?)
    }
}

/// Flags that determine what data is contained in the EnchantmentRegistry
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum EnchantmentRegistryFlags {
    LifeSpells = 0x1,
    CreatureSpells = 0x2,
    Vitae = 0x4,
    Cooldowns = 0x8,
}

impl crate::readers::ACDataType for EnchantmentRegistryFlags {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EnchantmentRegistryFlags::try_from(value)?)
    }
}

#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum SpellCategory {
}

impl crate::readers::ACDataType for SpellCategory {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u8(reader)?;
        Ok(HeritageGroup::try_from(value)?)
    }
}

/// the type of highlight (outline) applied to the object's icon
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum IconHighlight {
    Invalid = 0x0,
    Magical = 0x1,
    Poisoned = 0x2,
    BoostHealth = 0x4,
    BoostMana = 0x8,
    BoostStamina = 0x10,
    Fire = 0x20,
    Lightning = 0x40,
    Frost = 0x80,
    Acid = 0x100,
    Bludgeoning = 0x200,
    Slashing = 0x400,
    Piercing = 0x800,
    Nether = 0x1000,
}

impl crate::readers::ACDataType for IconHighlight {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(IconHighlight::try_from(value)?)
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(ChatDisplayMask::try_from(value)?)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum EnchantmentTypeFlags {
    Undef = 0x0,
    Attribute = 0x1,
    SecondAtt = 0x2,
    Int = 0x4,
    Float = 0x8,
    Skill = 0x10,
    BodyDamageValue = 0x20,
    BodyDamageVariance = 0x40,
    BodyArmorValue = 0x80,
    SingleStat = 0x1000,
    MultipleStat = 0x2000,
    Multiplicative = 0x4000,
    Additive = 0x8000,
    AttackSkills = 0x10000,
    DefenseSkills = 0x20000,
    MultiplicativeDegrade = 0x100000,
    #[serde(rename = "Additive_Degrade")]
    AdditiveDegrade = 0x200000,
    Vitae = 0x800000,
    Cooldown = 0x1000000,
    Beneficial = 0x2000000,
    StatTypes = 0xFF,
}

impl crate::readers::ACDataType for EnchantmentTypeFlags {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(EnchantmentTypeFlags::try_from(value)?)
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(Placement::try_from(value)?)
    }
}

