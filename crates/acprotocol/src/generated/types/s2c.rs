use serde::{Serialize, Deserialize};

// Allegiance update cancelled
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdateAborted")]
pub struct AllegianceAllegianceUpdateAborted {
    #[serde(rename = "FailureType")]
    pub failure_type: WeenieError,
}

// Display a message in a popup message window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_PopUpString")]
pub struct CommunicationPopUpString {
    #[serde(rename = "Message")]
    pub message: String,
}

// Information describing your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_PlayerDescription")]
pub struct LoginPlayerDescription {
    #[serde(rename = "BaseQualities")]
    pub base_qualities: ACBaseQualities,
    #[serde(rename = "Qualities")]
    pub qualities: ACQualities,
    #[serde(rename = "PlayerModule")]
    pub player_module: PlayerModule,
    #[serde(rename = "ContentProfile")]
    pub content_profile: PackableList<ContentProfile>,
    #[serde(rename = "InventoryPlacement")]
    pub inventory_placement: PackableList<InventoryPlacement>,
}

// Returns info related to your monarch, patron and vassals.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdate")]
pub struct AllegianceAllegianceUpdate {
    #[serde(rename = "Rank")]
    pub rank: u32,
    #[serde(rename = "Profile")]
    pub profile: AllegianceProfile,
}

// Friends list update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_FriendsUpdate")]
pub struct SocialFriendsUpdate {
    #[serde(rename = "Friends")]
    pub friends: PackableList<FriendData>,
    #[serde(rename = "Type")]
    pub type_: FriendsUpdateType,
}

// Store an item in a container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ServerSaysContainId")]
pub struct ItemServerSaysContainId {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ContainerId")]
    pub container_id: ObjectId,
    #[serde(rename = "SlotIndex")]
    pub slot_index: u32,
    #[serde(rename = "ContainerType")]
    pub container_type: ContainerProperties,
}

// Equip an item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_WearItem")]
pub struct ItemWearItem {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Slot")]
    pub slot: EquipMask,
}

// Titles for the current character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_CharacterTitleTable")]
pub struct SocialCharacterTitleTable {
    #[serde(rename = "DisplayTitle")]
    pub display_title: u32,
    #[serde(rename = "Titles")]
    pub titles: PackableList<uint>,
}

// Set a title for the current character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_AddOrSetCharacterTitle")]
pub struct SocialAddOrSetCharacterTitle {
    #[serde(rename = "NewTitle")]
    pub new_title: u32,
    #[serde(rename = "SetAsDisplayTitle")]
    pub set_as_display_title: bool,
}

// Close Container - Only sent when explicitly closed
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_StopViewingObjectContents")]
pub struct ItemStopViewingObjectContents {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Open the buy/sell panel for a merchant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Vendor_VendorInfo")]
pub struct VendorVendorInfo {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Profile")]
    pub profile: VendorProfile,
    #[serde(rename = "Items")]
    pub items: PackableList<ItemProfile>,
}

// Opens barber UI
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_StartBarber")]
pub struct CharacterStartBarber {
    #[serde(rename = "BasePalette")]
    pub base_palette: DataId,
    #[serde(rename = "HeadObject")]
    pub head_object: DataId,
    #[serde(rename = "HeadTexture")]
    pub head_texture: DataId,
    #[serde(rename = "DefaultHeadTexture")]
    pub default_head_texture: DataId,
    #[serde(rename = "EyesTexture")]
    pub eyes_texture: DataId,
    #[serde(rename = "DefaultEyesTexture")]
    pub default_eyes_texture: DataId,
    #[serde(rename = "NoseTexture")]
    pub nose_texture: DataId,
    #[serde(rename = "DefaultNoseTexture")]
    pub default_nose_texture: DataId,
    #[serde(rename = "MouthTexture")]
    pub mouth_texture: DataId,
    #[serde(rename = "DefaultMouthTexture")]
    pub default_mouth_texture: DataId,
    #[serde(rename = "SkinPalette")]
    pub skin_palette: DataId,
    #[serde(rename = "HairPalette")]
    pub hair_palette: DataId,
    #[serde(rename = "EyesPalette")]
    pub eyes_palette: DataId,
    #[serde(rename = "SetupId")]
    pub setup_id: DataId,
    #[serde(rename = "Option1")]
    pub option1: i32,
    #[serde(rename = "Option2")]
    pub option2: i32,
}

// Member left fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Quit")]
pub struct FellowshipQuit {
    #[serde(rename = "Disband")]
    pub disband: bool,
}

// Member dismissed from fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Dismiss")]
pub struct FellowshipDismiss {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Sent when you first open a book, contains the entire table of contents.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookOpen")]
pub struct WritingBookOpen {
    #[serde(rename = "BookId")]
    pub book_id: ObjectId,
    #[serde(rename = "MaxNumPages")]
    pub max_num_pages: u32,
    #[serde(rename = "PageData")]
    pub page_data: PageDataList,
    #[serde(rename = "Inscription")]
    pub inscription: String,
    #[serde(rename = "ScribeId")]
    pub scribe_id: ObjectId,
    #[serde(rename = "ScribeName")]
    pub scribe_name: String,
}

// Response to an attempt to add a page to a book.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookAddPageResponse")]
pub struct WritingBookAddPageResponse {
    #[serde(rename = "BookId")]
    pub book_id: ObjectId,
    #[serde(rename = "PageNumber")]
    pub page_number: u32,
    #[serde(rename = "Success")]
    pub success: bool,
}

// Response to an attempt to delete a page from a book.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookDeletePageResponse")]
pub struct WritingBookDeletePageResponse {
    #[serde(rename = "BookId")]
    pub book_id: ObjectId,
    #[serde(rename = "PageNumber")]
    pub page_number: u32,
    #[serde(rename = "Success")]
    pub success: bool,
}

// Contains the text of a single page of a book, parchment or sign.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookPageDataResponse")]
pub struct WritingBookPageDataResponse {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Page")]
    pub page: u32,
    #[serde(rename = "PageData")]
    pub page_data: PageData,
}

// Get Inscription Response, doesn't seem to be really used by client
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_GetInscriptionResponse")]
pub struct ItemGetInscriptionResponse {
    #[serde(rename = "Inscription")]
    pub inscription: String,
    #[serde(rename = "ScribeName")]
    pub scribe_name: String,
    #[serde(rename = "ScribeAccount")]
    pub scribe_account: String,
}

// The result of an attempt to assess an item or creature.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_SetAppraiseInfo")]
pub struct ItemSetAppraiseInfo {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "IntProperties")]
    pub int_properties: Option<PackableHashTable<PropertyInt, int>>,
    #[serde(rename = "Int64Properties")]
    pub int64_properties: Option<PackableHashTable<PropertyInt64, long>>,
    #[serde(rename = "BoolProperties")]
    pub bool_properties: Option<PackableHashTable<PropertyBool, bool>>,
    #[serde(rename = "FloatProperties")]
    pub float_properties: Option<PackableHashTable<PropertyFloat, double>>,
    #[serde(rename = "StringProperties")]
    pub string_properties: Option<PackableHashTable<PropertyString, string>>,
    #[serde(rename = "DataIdProperties")]
    pub data_id_properties: Option<PackableHashTable<PropertyDataId, DataId>>,
    #[serde(rename = "SpellBook")]
    pub spell_book: Option<PackableList<LayeredSpellId>>,
    #[serde(rename = "ArmorProfile")]
    pub armor_profile: Option<ArmorProfile>,
    #[serde(rename = "CreatureProfile")]
    pub creature_profile: Option<CreatureAppraisalProfile>,
    #[serde(rename = "WeaponProfile")]
    pub weapon_profile: Option<WeaponProfile>,
    #[serde(rename = "HookProfile")]
    pub hook_profile: Option<HookAppraisalProfile>,
    #[serde(rename = "ArmorHighlight")]
    pub armor_highlight: Option<ArmorHighlightMask>,
    #[serde(rename = "ArmorColor")]
    pub armor_color: Option<ArmorHighlightMask>,
    #[serde(rename = "WeaponHighlight")]
    pub weapon_highlight: Option<WeaponHighlightMask>,
    #[serde(rename = "WeaponColor")]
    pub weapon_color: Option<WeaponHighlightMask>,
    #[serde(rename = "ResistHighlight")]
    pub resist_highlight: Option<ResistHighlightMask>,
    #[serde(rename = "ResistColor")]
    pub resist_color: Option<ResistHighlightMask>,
    #[serde(rename = "BaseArmorHead")]
    pub base_armor_head: Option<u32>,
    #[serde(rename = "BaseArmorChest")]
    pub base_armor_chest: Option<u32>,
    #[serde(rename = "BaseArmorGroin")]
    pub base_armor_groin: Option<u32>,
    #[serde(rename = "BaseArmorBicep")]
    pub base_armor_bicep: Option<u32>,
    #[serde(rename = "BaseArmorWrist")]
    pub base_armor_wrist: Option<u32>,
    #[serde(rename = "BaseArmorHand")]
    pub base_armor_hand: Option<u32>,
    #[serde(rename = "BaseArmorThigh")]
    pub base_armor_thigh: Option<u32>,
    #[serde(rename = "BaseArmorShin")]
    pub base_armor_shin: Option<u32>,
    #[serde(rename = "BaseArmorFoot")]
    pub base_armor_foot: Option<u32>,
}

// ChannelBroadcast: Group Chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelBroadcast")]
pub struct CommunicationChannelBroadcast {
    #[serde(rename = "Channel")]
    pub channel: Channel,
    #[serde(rename = "Message")]
    pub message: String,
}

// ChannelList: Provides list of characters listening to a channel, I assume in response to a command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelList")]
pub struct CommunicationChannelList {
    #[serde(rename = "Characters")]
    pub characters: PackableList<string>,
}

// ChannelIndex: Provides list of channels available to the player, I assume in response to a command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelIndex")]
pub struct CommunicationChannelIndex {
    #[serde(rename = "Channels")]
    pub channels: PackableList<string>,
}

// Set Pack Contents
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_OnViewContents")]
pub struct ItemOnViewContents {
    #[serde(rename = "ContainerId")]
    pub container_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ContentProfile>,
}

// ServerSaysMoveItem: Removes an item from inventory (when you place it on the ground or give it away)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ServerSaysMoveItem")]
pub struct ItemServerSaysMoveItem {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// HandleAttackDoneEvent: Melee attack completed
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleAttackDoneEvent")]
pub struct CombatHandleAttackDoneEvent {
    #[serde(rename = "Number")]
    pub number: u32,
}

// RemoveSpell: Delete a spell from your spellbook.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_RemoveSpell")]
pub struct MagicRemoveSpell {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

// You just died.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleVictimNotificationEventSelf")]
pub struct CombatHandleVictimNotificationEventSelf {
    #[serde(rename = "Message")]
    pub message: String,
}

// Message for a death, something you killed or your own death message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleVictimNotificationEventOther")]
pub struct CombatHandleVictimNotificationEventOther {
    #[serde(rename = "Message")]
    pub message: String,
}

// HandleAttackerNotificationEvent: You have hit your target with a melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleAttackerNotificationEvent")]
pub struct CombatHandleAttackerNotificationEvent {
    #[serde(rename = "DefenderName")]
    pub defender_name: String,
    #[serde(rename = "Type")]
    pub type_: DamageType,
    #[serde(rename = "DamagePercent")]
    pub damage_percent: f32,
    #[serde(rename = "Damage")]
    pub damage: u32,
    #[serde(rename = "Critical")]
    pub critical: bool,
    #[serde(rename = "AttackConditions")]
    pub attack_conditions: AttackConditionsMask,
}

// HandleDefenderNotificationEvent: You have been hit by a creature's melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleDefenderNotificationEvent")]
pub struct CombatHandleDefenderNotificationEvent {
    #[serde(rename = "AttackerName")]
    pub attacker_name: String,
    #[serde(rename = "Type")]
    pub type_: DamageType,
    #[serde(rename = "DamagePercent")]
    pub damage_percent: f32,
    #[serde(rename = "Damage")]
    pub damage: u32,
    #[serde(rename = "Location")]
    pub location: DamageLocation,
    #[serde(rename = "Critical")]
    pub critical: bool,
    #[serde(rename = "AttackConditions")]
    pub attack_conditions: AttackConditionsMask,
}

// HandleEvasionAttackerNotificationEvent: Your target has evaded your melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleEvasionAttackerNotificationEvent")]
pub struct CombatHandleEvasionAttackerNotificationEvent {
    #[serde(rename = "DefenderName")]
    pub defender_name: String,
}

// HandleEvasionDefenderNotificationEvent: You have evaded a creature's melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleEvasionDefenderNotificationEvent")]
pub struct CombatHandleEvasionDefenderNotificationEvent {
    #[serde(rename = "AttackerName")]
    pub attacker_name: String,
}

// HandleCommenceAttackEvent: Start melee attack
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleCommenceAttackEvent")]
pub struct CombatHandleCommenceAttackEvent {}

// QueryHealthResponse: Update a creature's health bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_QueryHealthResponse")]
pub struct CombatQueryHealthResponse {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Health")]
    pub health: f32,
}

// QueryAgeResponse: happens when you do /age in the game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_QueryAgeResponse")]
pub struct CharacterQueryAgeResponse {
    #[serde(rename = "TargetName")]
    pub target_name: String,
    #[serde(rename = "Age")]
    pub age: String,
}

// UseDone: Ready. Previous action complete
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UseDone")]
pub struct ItemUseDone {
    #[serde(rename = "FailureType")]
    pub failure_type: WeenieError,
}

// Allegiance update finished
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdateDone")]
pub struct AllegianceAllegianceUpdateDone {
    #[serde(rename = "FailureType")]
    pub failure_type: WeenieError,
}

// Fellow update is done
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_FellowUpdateDone")]
pub struct FellowshipFellowUpdateDone {}

// Fellow stats are done
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_FellowStatsDone")]
pub struct FellowshipFellowStatsDone {}

// Close Assess Panel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_AppraiseDone")]
pub struct ItemAppraiseDone {
    #[serde(rename = "Unknown")]
    pub unknown: u32,
}

// Ping Reply
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ReturnPing")]
pub struct CharacterReturnPing {}

// Squelch and Filter List
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_SetSquelchDB")]
pub struct CommunicationSetSquelchDB {
    #[serde(rename = "SquelchDB")]
    pub squelch_db: SquelchDB,
}

// RegisterTrade: Send to begin a trade and display the trade window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_RegisterTrade")]
pub struct TradeRegisterTrade {
    #[serde(rename = "InitiatorId")]
    pub initiator_id: ObjectId,
    #[serde(rename = "PartnerId")]
    pub partner_id: ObjectId,
    #[serde(rename = "Stamp")]
    pub stamp: i64,
}

// OpenTrade: Open trade window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_OpenTrade")]
pub struct TradeOpenTrade {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// CloseTrade: End trading
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_CloseTrade")]
pub struct TradeCloseTrade {
    #[serde(rename = "Reason")]
    pub reason: EndTradeReason,
}

// RemoveFromTrade: Item was removed from trade window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_AddToTrade")]
pub struct TradeAddToTrade {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Side")]
    pub side: TradeSide,
}

// Removes an item from the trade window, not sure if this is used still?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_RemoveFromTrade")]
pub struct TradeRemoveFromTrade {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Side")]
    pub side: TradeSide,
}

// AcceptTrade: The trade was accepted
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_AcceptTrade")]
pub struct TradeAcceptTrade {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// DeclineTrade: The trade was declined
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_DeclineTrade")]
pub struct TradeDeclineTrade {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// ResetTrade: The trade window was reset
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_ResetTrade")]
pub struct TradeResetTrade {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// TradeFailure: Failure to add a trade item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_TradeFailure")]
pub struct TradeTradeFailure {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Reason")]
    pub reason: u32,
}

// ClearTradeAcceptance: Failure to complete a trade
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_ClearTradeAcceptance")]
pub struct TradeClearTradeAcceptance {}

// Buy a dwelling or pay maintenance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseProfile")]
pub struct HouseHouseProfile {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Profile")]
    pub profile: HouseProfile,
}

// House panel information for owners.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseData")]
pub struct HouseHouseData {
    #[serde(rename = "Data")]
    pub data: HouseData,
}

// House Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseStatus")]
pub struct HouseHouseStatus {
    #[serde(rename = "NoticeType")]
    pub notice_type: u32,
}

// Update Rent Time
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRentTime")]
pub struct HouseUpdateRentTime {
    #[serde(rename = "RentTime")]
    pub rent_time: u32,
}

// Update Rent Payment
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRentPayment")]
pub struct HouseUpdateRentPayment {
    #[serde(rename = "Rent")]
    pub rent: PackableList<HousePayment>,
}

// Update Restrictions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRestrictions")]
pub struct HouseUpdateRestrictions {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "Restrictions")]
    pub restrictions: RestrictionDB,
}

// House Guest List
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateHAR")]
pub struct HouseUpdateHAR {
    #[serde(rename = "GuestList")]
    pub guest_list: HAR,
}

// House Profile
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseTransaction")]
pub struct HouseHouseTransaction {
    #[serde(rename = "NoticeType")]
    pub notice_type: u32,
}

// Update an item's mana bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_QueryItemManaResponse")]
pub struct ItemQueryItemManaResponse {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Mana")]
    pub mana: f32,
    #[serde(rename = "Success")]
    pub success: bool,
}

// Display a list of available dwellings in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AvailableHouses")]
pub struct HouseAvailableHouses {
    #[serde(rename = "Type")]
    pub type_: HouseType,
    #[serde(rename = "Houses")]
    pub houses: PackableList<uint>,
    #[serde(rename = "NumHouses")]
    pub num_houses: i32,
}

// Display a confirmation panel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationRequest")]
pub struct CharacterConfirmationRequest {
    #[serde(rename = "ConfirmationType")]
    pub confirmation_type: ConfirmationType,
    #[serde(rename = "ContextId")]
    pub context_id: u32,
    #[serde(rename = "Text")]
    pub text: String,
}

// Confirmation done
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationDone")]
pub struct CharacterConfirmationDone {
    #[serde(rename = "ConfirmationType")]
    pub confirmation_type: ConfirmationType,
    #[serde(rename = "ContextId")]
    pub context_id: u32,
}

// Display an allegiance login/logout message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceLoginNotificationEvent")]
pub struct AllegianceAllegianceLoginNotificationEvent {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
    #[serde(rename = "IsLoggedIn")]
    pub is_logged_in: bool,
}

// Returns data for a player's allegiance information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceInfoResponseEvent")]
pub struct AllegianceAllegianceInfoResponseEvent {
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "Profile")]
    pub profile: AllegianceProfile,
}

// Joining game response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_JoinGameResponse")]
pub struct GameJoinGameResponse {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "Team")]
    pub team: i32,
}

// Start game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_StartGame")]
pub struct GameStartGame {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "Team")]
    pub team: i32,
}

// Move response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_MoveResponse")]
pub struct GameMoveResponse {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "MoveResult")]
    pub move_result: ChessMoveResult,
}

// Opponent Turn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_OpponentTurn")]
pub struct GameOpponentTurn {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "Team")]
    pub team: i32,
    #[serde(rename = "GameMove")]
    pub game_move: GameMoveData,
}

// Opponent Stalemate State
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_OpponentStalemateState")]
pub struct GameOpponentStalemateState {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "Team")]
    pub team: i32,
    #[serde(rename = "On")]
    pub on: bool,
}

// Display a status message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_WeenieError")]
pub struct CommunicationWeenieError {
    #[serde(rename = "Type")]
    pub type_: WeenieError,
}

// Display a parameterized status message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_WeenieErrorWithString")]
pub struct CommunicationWeenieErrorWithString {
    #[serde(rename = "Type")]
    pub type_: WeenieErrorWithString,
    #[serde(rename = "Text")]
    pub text: String,
}

// End of Chess game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_GameOver")]
pub struct GameGameOver {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "TeamWinner")]
    pub team_winner: i32,
}

// Set Turbine Chat channel numbers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChatRoomTracker")]
pub struct CommunicationChatRoomTracker {
    #[serde(rename = "AllegianceRoomId")]
    pub allegiance_room_id: u32,
    #[serde(rename = "GeneralChatRoomId")]
    pub general_chat_room_id: u32,
    #[serde(rename = "TradeChatRoomId")]
    pub trade_chat_room_id: u32,
    #[serde(rename = "LFGChatRoomId")]
    pub lfg_chat_room_id: u32,
    #[serde(rename = "RoleplayChatRoomId")]
    pub roleplay_chat_room_id: u32,
    #[serde(rename = "OlthoiChatRoomId")]
    pub olthoi_chat_room_id: u32,
    #[serde(rename = "SocietyChatRoomId")]
    pub society_chat_room_id: u32,
    #[serde(rename = "SocietyCelestialHandChatRoomId")]
    pub society_celestial_hand_chat_room_id: u32,
    #[serde(rename = "SocietyEldrichWebChatRoomId")]
    pub society_eldrich_web_chat_room_id: u32,
    #[serde(rename = "SocietyRadiantBloodChatRoomId")]
    pub society_radiant_blood_chat_room_id: u32,
}

// TODO: QueryPluginList
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_QueryPluginList")]
pub struct AdminQueryPluginList {}

// TODO: QueryPlugin
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_QueryPlugin")]
pub struct AdminQueryPlugin {}

// TODO: QueryPluginResponse
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_QueryPluginResponse2")]
pub struct AdminQueryPluginResponse2 {}

// Salvage operation results
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_SalvageOperationsResultData")]
pub struct InventorySalvageOperationsResultData {
    #[serde(rename = "Result")]
    pub result: SalvageOperationsResultData,
}

// Someone has sent you a @tell.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearDirectSpeech")]
pub struct CommunicationHearDirectSpeech {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
    #[serde(rename = "SecretFlags")]
    pub secret_flags: u32,
}

// Create or join a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_FullUpdate")]
pub struct FellowshipFullUpdate {
    #[serde(rename = "Fellowship")]
    pub fellowship: Fellowship,
}

// Disband your fellowship.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Disband")]
pub struct FellowshipDisband {}

// Add/Update a member to your fellowship.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_UpdateFellow")]
pub struct FellowshipUpdateFellow {
    #[serde(rename = "Fellow")]
    pub fellow: Fellow,
    #[serde(rename = "UpdateType")]
    pub update_type: FellowUpdateType,
}

// Add a spell to your spellbook.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateSpell")]
pub struct MagicUpdateSpell {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

// Apply an enchantment to your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateEnchantment")]
pub struct MagicUpdateEnchantment {
    #[serde(rename = "Enchantment")]
    pub enchantment: Enchantment,
}

// Remove an enchantment from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_RemoveEnchantment")]
pub struct MagicRemoveEnchantment {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

// Update multiple enchantments from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateMultipleEnchantments")]
pub struct MagicUpdateMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    pub enchantments: PackableList<Enchantment>,
}

// Remove multiple enchantments from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_RemoveMultipleEnchantments")]
pub struct MagicRemoveMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    pub enchantments: PackableList<LayeredSpellId>,
}

// Silently remove all enchantments from your character, e.g. when you die (no message in the chat window).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_PurgeEnchantments")]
pub struct MagicPurgeEnchantments {}

// Silently remove An enchantment from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_DispelEnchantment")]
pub struct MagicDispelEnchantment {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

// Silently remove multiple enchantments from your character (no message in the chat window).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_DispelMultipleEnchantments")]
pub struct MagicDispelMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    pub enchantments: PackableList<LayeredSpellId>,
}

// A portal storm is brewing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Misc_PortalStormBrewing")]
pub struct MiscPortalStormBrewing {
    #[serde(rename = "Extent")]
    pub extent: f32,
}

// A portal storm is imminent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Misc_PortalStormImminent")]
pub struct MiscPortalStormImminent {
    #[serde(rename = "Extent")]
    pub extent: f32,
}

// You have been portal stormed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Misc_PortalStorm")]
pub struct MiscPortalStorm {}

// The portal storm has subsided.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Misc_PortalStormSubsided")]
pub struct MiscPortalStormSubsided {}

// Display a status message on the Action Viewscreen (the red text overlaid on the 3D area).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TransientString")]
pub struct CommunicationTransientString {
    #[serde(rename = "Message")]
    pub message: String,
}

// Remove all bad enchantments from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_PurgeBadEnchantments")]
pub struct MagicPurgeBadEnchantments {}

// Sends all contract data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendClientContractTrackerTable")]
pub struct SocialSendClientContractTrackerTable {
    #[serde(rename = "ContractTracker")]
    pub contract_tracker: ContractTrackerTable,
}

// Updates a contract data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendClientContractTracker")]
pub struct SocialSendClientContractTracker {
    #[serde(rename = "ContractTracker")]
    pub contract_tracker: ContractTracker,
    #[serde(rename = "DeleteContract")]
    pub delete_contract: bool,
    #[serde(rename = "SetAsDisplayContract")]
    pub set_as_display_contract: bool,
}

// Sent every time an object you are aware of ceases to exist. Merely running out of range does not generate this message - in that case, the client just automatically stops tracking it after receiving no updates for a while (which I presume is a very short while).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ServerSaysRemove")]
pub struct ItemServerSaysRemove {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Failure to give an item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ServerSaysAttemptFailed")]
pub struct CharacterServerSaysAttemptFailed {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Reason")]
    pub reason: WeenieError,
}

// For stackable items, this changes the number of items in the stack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UpdateStackSize")]
pub struct ItemUpdateStackSize {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Amount")]
    pub amount: u32,
    #[serde(rename = "NewValue")]
    pub new_value: u32,
}

// A Player Kill occurred nearby (also sent for suicides).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandlePlayerDeathEvent")]
pub struct CombatHandlePlayerDeathEvent {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "KilledId")]
    pub killed_id: ObjectId,
    #[serde(rename = "KillerId")]
    pub killer_id: ObjectId,
}

// Remove an int property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveIntEvent")]
pub struct QualitiesPrivateRemoveIntEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Type")]
    pub type_: PropertyInt,
}

// Remove an int property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveIntEvent")]
pub struct QualitiesRemoveIntEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: PropertyInt,
}

// Remove a bool property from the charactert
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveBoolEvent")]
pub struct QualitiesPrivateRemoveBoolEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Type")]
    pub type_: PropertyBool,
}

// Remove a bool property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveBoolEvent")]
pub struct QualitiesRemoveBoolEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: PropertyBool,
}

// Remove a float property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveFloatEvent")]
pub struct QualitiesPrivateRemoveFloatEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Type")]
    pub type_: PropertyFloat,
}

// Remove a float property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveFloatEvent")]
pub struct QualitiesRemoveFloatEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: PropertyFloat,
}

// Remove a string property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveStringEvent")]
pub struct QualitiesPrivateRemoveStringEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Type")]
    pub type_: PropertyString,
}

// Remove a string property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveStringEvent")]
pub struct QualitiesRemoveStringEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: PropertyString,
}

// Remove an dataId property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveDataIdEvent")]
pub struct QualitiesPrivateRemoveDataIdEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Type")]
    pub type_: PropertyDataId,
}

// Remove an dataId property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveDataIdEvent")]
pub struct QualitiesRemoveDataIdEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: PropertyDataId,
}

// Remove an instanceId property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveInstanceIdEvent")]
pub struct QualitiesPrivateRemoveInstanceIdEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Type")]
    pub type_: PropertyInstanceId,
}

// Remove an instanceId property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveInstanceIdEvent")]
pub struct QualitiesRemoveInstanceIdEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: PropertyInstanceId,
}

// Remove a position property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemovePositionEvent")]
pub struct QualitiesPrivateRemovePositionEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Type")]
    pub type_: PropertyPosition,
}

// Remove a position property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemovePositionEvent")]
pub struct QualitiesRemovePositionEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: PropertyPosition,
}

// Remove an int64 property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveInt64Event")]
pub struct QualitiesPrivateRemoveInt64Event {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Type")]
    pub type_: PropertyInt64,
}

// Remove an int64 property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveInt64Event")]
pub struct QualitiesRemoveInt64Event {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: PropertyInt64,
}

// Set or update a Character Int property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateInt")]
pub struct QualitiesPrivateUpdateInt {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: PropertyInt,
    #[serde(rename = "Value")]
    pub value: i32,
}

// Set or update an Object Int property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateInt")]
pub struct QualitiesUpdateInt {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: PropertyInt,
    #[serde(rename = "Value")]
    pub value: i32,
}

// Set or update a Character Int64 property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateInt64")]
pub struct QualitiesPrivateUpdateInt64 {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: PropertyInt64,
    #[serde(rename = "Value")]
    pub value: i64,
}

// Set or update a Character Int64 property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateInt64")]
pub struct QualitiesUpdateInt64 {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: PropertyInt64,
    #[serde(rename = "Value")]
    pub value: i64,
}

// Set or update a Character Boolean property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateBool")]
pub struct QualitiesPrivateUpdateBool {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: PropertyBool,
    #[serde(rename = "Value")]
    pub value: bool,
}

// Set or update an Object Boolean property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateBool")]
pub struct QualitiesUpdateBool {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: PropertyBool,
    #[serde(rename = "Value")]
    pub value: bool,
}

// Set or update an Object float property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateFloat")]
pub struct QualitiesPrivateUpdateFloat {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: PropertyFloat,
    #[serde(rename = "Value")]
    pub value: f32,
}

// Set or update an Object float property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateFloat")]
pub struct QualitiesUpdateFloat {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: PropertyFloat,
    #[serde(rename = "Value")]
    pub value: f32,
}

// Set or update an Object String property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateString")]
pub struct QualitiesPrivateUpdateString {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: PropertyString,
    #[serde(rename = "Value")]
    pub value: String,
}

// Set or update an Object String property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateString")]
pub struct QualitiesUpdateString {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: PropertyString,
    #[serde(rename = "Value")]
    pub value: String,
}

// Set or update an Object DId property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateDataId")]
pub struct QualitiesPrivateUpdateDataId {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: PropertyDataId,
    #[serde(rename = "Value")]
    pub value: u32,
}

// Set or update an Object DId property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateDataId")]
pub struct QualitiesUpdateDataId {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: PropertyDataId,
    #[serde(rename = "Value")]
    pub value: u32,
}

// Set or update a IId property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateInstanceId")]
pub struct QualitiesPrivateUpdateInstanceId {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: PropertyInstanceId,
    #[serde(rename = "Value")]
    pub value: ObjectId,
}

// Set or update an Object IId property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateInstanceId")]
pub struct QualitiesUpdateInstanceId {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: PropertyInstanceId,
    #[serde(rename = "Value")]
    pub value: ObjectId,
}

// Set or update a Character Position property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdatePosition")]
pub struct QualitiesPrivateUpdatePosition {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: PropertyPosition,
    #[serde(rename = "Value")]
    pub value: Position,
}

// Set or update a Character Position property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdatePosition")]
pub struct QualitiesUpdatePosition {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: PropertyPosition,
    #[serde(rename = "Value")]
    pub value: Position,
}

// Set or update a Character Skill value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateSkill")]
pub struct QualitiesPrivateUpdateSkill {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: SkillId,
    #[serde(rename = "Value")]
    pub value: Skill,
}

// Set or update a Character Skill value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateSkill")]
pub struct QualitiesUpdateSkill {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: SkillId,
    #[serde(rename = "Value")]
    pub value: Skill,
}

// Set or update a Character Skill Level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateSkillLevel")]
pub struct QualitiesPrivateUpdateSkillLevel {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: SkillId,
    #[serde(rename = "Value")]
    pub value: u32,
}

// Set or update a Character Skill Level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateSkillLevel")]
pub struct QualitiesUpdateSkillLevel {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: SkillId,
    #[serde(rename = "Value")]
    pub value: u32,
}

// Set or update a Character Skill state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateSkillAC")]
pub struct QualitiesPrivateUpdateSkillAC {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: SkillId,
    #[serde(rename = "Value")]
    pub value: SkillAdvancementClass,
}

// Set or update a Character Skill state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateSkillAC")]
pub struct QualitiesUpdateSkillAC {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: SkillId,
    #[serde(rename = "Value")]
    pub value: SkillAdvancementClass,
}

// Set or update a Character Attribute value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateAttribute")]
pub struct QualitiesPrivateUpdateAttribute {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: AttributeId,
    #[serde(rename = "Value")]
    pub value: AttributeInfo,
}

// Set or update a Character Attribute value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateAttribute")]
pub struct QualitiesUpdateAttribute {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: AttributeId,
    #[serde(rename = "Value")]
    pub value: AttributeInfo,
}

// Set or update a Character Attribute Level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateAttributeLevel")]
pub struct QualitiesPrivateUpdateAttributeLevel {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: AttributeId,
    #[serde(rename = "Value")]
    pub value: u32,
}

// Set or update a Character Attribute Level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateAttributeLevel")]
pub struct QualitiesUpdateAttributeLevel {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: AttributeId,
    #[serde(rename = "Value")]
    pub value: u32,
}

// Set or update a Character Vital value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateAttribute2nd")]
pub struct QualitiesPrivateUpdateAttribute2nd {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: VitalId,
    #[serde(rename = "Value")]
    pub value: SecondaryAttributeInfo,
}

// Set or update a Character Vital value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateAttribute2nd")]
pub struct QualitiesUpdateAttribute2nd {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: VitalId,
    #[serde(rename = "Value")]
    pub value: SecondaryAttributeInfo,
}

// Set or update a Character Vital value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateAttribute2ndLevel")]
pub struct QualitiesPrivateUpdateAttribute2ndLevel {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: CurVitalId,
    #[serde(rename = "Value")]
    pub value: u32,
}

// Set or update a Character Vital value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateAttribute2ndLevel")]
pub struct QualitiesUpdateAttribute2ndLevel {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: CurVitalId,
    #[serde(rename = "Value")]
    pub value: u32,
}

// Indirect '/e' text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearEmote")]
pub struct CommunicationHearEmote {
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "Text")]
    pub text: String,
}

// Contains the text associated with an emote action.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearSoulEmote")]
pub struct CommunicationHearSoulEmote {
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "Text")]
    pub text: String,
}

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearSpeech")]
pub struct CommunicationHearSpeech {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearRangedSpeech")]
pub struct CommunicationHearRangedSpeech {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "Range")]
    pub range: f32,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

// This appears to be an admin command to change the environment (light, fog, sounds, colors)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_Environs")]
pub struct AdminEnvirons {
    #[serde(rename = "EnvrionOption")]
    pub envrion_option: EnvrionChangeType,
}

// Sets both the position and movement, such as when materializing at a lifestone
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_PositionAndMovementEvent")]
pub struct MovementPositionAndMovementEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Position")]
    pub position: PositionPack,
    #[serde(rename = "MovementData")]
    pub movement_data: MovementData,
}

// Sent whenever a character changes their clothes. It contains the entire description of what their wearing (and possibly their facial features as well). This message is only sent for changes, when the character is first created, the body of this message is included inside the creation message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ObjDescEvent")]
pub struct ItemObjDescEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
    #[serde(rename = "InstanceSequence")]
    pub instance_sequence: u16,
    #[serde(rename = "VisualDescSequence")]
    pub visual_desc_sequence: u16,
}

// Sets the player visual desc, TODO confirm this
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SetPlayerVisualDesc")]
pub struct CharacterSetPlayerVisualDesc {
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
}

// Character creation screen initilised.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharGenVerificationResponse")]
#[serde(tag = "ResponseType")]
pub enum CharacterCharGenVerificationResponse {
    #[serde(rename = "0x1")]
    Type1 {
    #[serde(rename = "CharacterId")]
    character_id: ObjectId,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "SecondsUntilDeletion")]
    seconds_until_deletion: u32,
    },
}

// Sent when your subsciption is about to expire
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AwaitingSubscriptionExpiration")]
pub struct LoginAwaitingSubscriptionExpiration {
    #[serde(rename = "SecondsRemaining")]
    pub seconds_remaining: u32,
}

// Instructs the client to return to 2D mode - the character list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_LogOffCharacter")]
pub struct LoginLogOffCharacter {}

// A character was marked for deletetion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterDelete")]
pub struct CharacterCharacterDelete {}

// The list of characters on the current account.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_LoginCharacterSet")]
pub struct LoginLoginCharacterSet {
    #[serde(rename = "Status")]
    pub status: u32,
    #[serde(rename = "Characters")]
    pub characters: PackableList<CharacterIdentity>,
    #[serde(rename = "DeletedCharacters")]
    pub deleted_characters: PackableList<CharacterIdentity>,
    #[serde(rename = "NumAllowedCharacters")]
    pub num_allowed_characters: u32,
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "UseTurbineChat")]
    pub use_turbine_chat: bool,
    #[serde(rename = "HasThroneofDestiny")]
    pub has_throneof_destiny: bool,
}

// Failure to log in
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterError")]
pub struct CharacterCharacterError {
    #[serde(rename = "Reason")]
    pub reason: CharacterErrorType,
}

// Create an object somewhere in the world
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_CreateObject")]
pub struct ItemCreateObject {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
    #[serde(rename = "PhysicsDescription")]
    pub physics_description: PhysicsDesc,
    #[serde(rename = "WeenieDescription")]
    pub weenie_description: PublicWeenieDesc,
}

// Login of player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_CreatePlayer")]
pub struct LoginCreatePlayer {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
}

// Sent whenever an object is being deleted from the scene.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_DeleteObject")]
pub struct ItemDeleteObject {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
}

// Sets the position/motion of an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_PositionEvent")]
pub struct MovementPositionEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Position")]
    pub position: PositionPack,
}

// Sets the parent for an object, eg. equipting an object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ParentEvent")]
pub struct ItemParentEvent {
    #[serde(rename = "ParentId")]
    pub parent_id: ObjectId,
    #[serde(rename = "ChildId")]
    pub child_id: ObjectId,
    #[serde(rename = "Location")]
    pub location: ParentLocation,
    #[serde(rename = "Placement")]
    pub placement: Placement,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ChildPositionSequence")]
    pub child_position_sequence: u16,
}

// Sent when picking up an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_PickupEvent")]
pub struct InventoryPickupEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectPositionSequence")]
    pub object_position_sequence: u16,
}

// Set's the current state of the object. Client appears to only process the following state changes post creation: NoDraw, LightingOn, Hidden
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_SetState")]
pub struct ItemSetState {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "NewState")]
    pub new_state: PhysicsState,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectStateSequence")]
    pub object_state_sequence: u16,
}

// These are animations. Whenever a human, monster or object moves - one of these little messages is sent. Even idle emotes (like head scratching and nodding) are sent in this manner.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_SetObjectMovement")]
pub struct MovementSetObjectMovement {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "MovementData")]
    pub movement_data: MovementData,
}

// Changes an objects vector, for things like jumping
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_VectorUpdate")]
pub struct MovementVectorUpdate {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Velocity")]
    pub velocity: Vector3,
    #[serde(rename = "Omega")]
    pub omega: Vector3,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectVectorSequence")]
    pub object_vector_sequence: u16,
}

// Applies a sound effect.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_SoundEvent")]
pub struct EffectsSoundEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "SoundType")]
    pub sound_type: Sound,
    #[serde(rename = "Volume")]
    pub volume: f32,
}

// Instructs the client to show the portal graphic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayerTeleport")]
pub struct EffectsPlayerTeleport {
    #[serde(rename = "ObjectTeleportSequence")]
    pub object_teleport_sequence: u16,
}

// Instructs the client to play a script. (Not seen so far, maybe prefered PlayScriptType)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayScriptId")]
pub struct EffectsPlayScriptId {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ScriptId")]
    pub script_id: DataId,
}

// Applies an effect with visual and sound by providing the type to be looked up in the Physics Script Table
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayScriptType")]
pub struct EffectsPlayScriptType {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ScriptType")]
    pub script_type: i32,
    #[serde(rename = "Speed")]
    pub speed: f32,
}

// Account has been banned
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AccountBanned")]
pub struct LoginAccountBanned {
    #[serde(rename = "BannedUntil")]
    pub banned_until: u32,
    #[serde(rename = "Text")]
    pub text: String,
}

// Admin Receive Account Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_ReceiveAccountData")]
pub struct AdminReceiveAccountData {
    #[serde(rename = "Unknown")]
    pub unknown: u32,
    #[serde(rename = "AdminAccountData")]
    pub admin_account_data: PackableList<AdminAccountData>,
}

// Admin Receive Player Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_ReceivePlayerData")]
pub struct AdminReceivePlayerData {
    #[serde(rename = "Unknown")]
    pub unknown: i32,
    #[serde(rename = "AdminPlayerData")]
    pub admin_player_data: PackableList<AdminPlayerData>,
}

// Update an existing object's data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UpdateObject")]
pub struct ItemUpdateObject {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDesc")]
    pub object_desc: ObjDesc,
    #[serde(rename = "PhysicsDesc")]
    pub physics_desc: PhysicsDesc,
    #[serde(rename = "WeenieDesc")]
    pub weenie_desc: PublicWeenieDesc,
}

// Account has been booted
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AccountBooted")]
pub struct LoginAccountBooted {
    #[serde(rename = "AdditionalReasonText")]
    pub additional_reason_text: String,
    #[serde(rename = "ReasonText")]
    pub reason_text: String,
}

// Send or receive a message using Turbine Chat.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TurbineChat")]
#[serde(tag = "BlobDispatchType")]
pub enum CommunicationTurbineChat {
    #[serde(rename = "0x01")]
    #[serde(alias = "0x02")]
    Type01 {
    #[serde(rename = "MessageSize")]
    message_size: u32,
    #[serde(rename = "Type")]
    type_: TurbineChatType,
    #[serde(rename = "TargetType")]
    target_type: i32,
    #[serde(rename = "TargetId")]
    target_id: i32,
    #[serde(rename = "TransportType")]
    transport_type: i32,
    #[serde(rename = "TransportId")]
    transport_id: i32,
    #[serde(rename = "Cookie")]
    cookie: i32,
    #[serde(rename = "PayloadSize")]
    payload_size: u32,
    #[serde(rename = "ContextId")]
    context_id: u32,
    #[serde(rename = "ResponseId")]
    response_id: u32,
    #[serde(rename = "MethodId")]
    method_id: u32,
    #[serde(rename = "HResult")]
    h_result: i32,
    },
}

// Switch from the character display to the game display.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_EnterGame_ServerReady")]
pub struct LoginEnterGameServerReady {}

// Display a message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TextboxString")]
pub struct CommunicationTextboxString {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

// The name of the current world.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_WorldInfo")]
pub struct LoginWorldInfo {
    #[serde(rename = "Connections")]
    pub connections: u32,
    #[serde(rename = "MaxConnections")]
    pub max_connections: u32,
    #[serde(rename = "WorldName")]
    pub world_name: String,
}

// Add or update a dat file Resource.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_DataMessage")]
#[serde(tag = "Compression")]
pub enum DDDDataMessage {
    #[serde(rename = "0x00")]
    Type00 {
    #[serde(rename = "DatFile")]
    dat_file: DatFileType,
    #[serde(rename = "ResourceType")]
    resource_type: u32,
    #[serde(rename = "ResourceId")]
    resource_id: DataId,
    #[serde(rename = "Iteration")]
    iteration: u32,
    #[serde(rename = "Version")]
    version: u32,
    #[serde(rename = "DataSize")]
    data_size: u32,
    #[serde(rename = "Data")]
    data: Vec<byte>,
    },
    #[serde(rename = "0x01")]
    Type01 {
    #[serde(rename = "DatFile")]
    dat_file: DatFileType,
    #[serde(rename = "ResourceType")]
    resource_type: u32,
    #[serde(rename = "ResourceId")]
    resource_id: DataId,
    #[serde(rename = "Iteration")]
    iteration: u32,
    #[serde(rename = "Version")]
    version: u32,
    #[serde(rename = "DataSize")]
    data_size: u32,
    #[serde(rename = "FileSize")]
    file_size: u32,
    data: Vec<byte>,
    },
}

// DDD error
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_ErrorMessage")]
pub struct DDDErrorMessage {
    #[serde(rename = "ResourceType")]
    pub resource_type: u32,
    #[serde(rename = "ResourceId")]
    pub resource_id: DataId,
    #[serde(rename = "RError")]
    pub r_error: u32,
}

// A list of dat files that need to be patched
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_BeginDDDMessage")]
pub struct DDDBeginDDDMessage {
    #[serde(rename = "DataExpected")]
    pub data_expected: u32,
    #[serde(rename = "Revisions")]
    pub revisions: PackableList<DDDRevision>,
}

// Add or update a dat file Resource.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_InterrogationMessage")]
pub struct DDDInterrogationMessage {
    #[serde(rename = "ServersRegion")]
    pub servers_region: u32,
    #[serde(rename = "NameRuleLanguage")]
    pub name_rule_language: u32,
    #[serde(rename = "ProductId")]
    pub product_id: u32,
    #[serde(rename = "SupportedLanguages")]
    pub supported_languages: PackableList<uint>,
}

// Ends DDD update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_OnEndDDD")]
pub struct DDDOnEndDDD {}

