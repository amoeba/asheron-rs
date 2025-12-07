use serde::{Serialize, Deserialize};

// Allegiance update cancelled
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdateAborted")]
pub struct AllegianceAllegianceUpdateAborted {
        #[serde(rename = "FailureType")]
        failure_type: WeenieError
}

// Display a message in a popup message window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_PopUpString")]
pub struct CommunicationPopUpString {
        #[serde(rename = "Message")]
        message: String
}

// Information describing your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_PlayerDescription")]
pub struct LoginPlayerDescription {
        #[serde(rename = "BaseQualities")]
        base_qualities: ACBaseQualities,
        #[serde(rename = "Qualities")]
        qualities: ACQualities,
        #[serde(rename = "PlayerModule")]
        player_module: PlayerModule,
        #[serde(rename = "ContentProfile")]
        content_profile: PackableList,
        #[serde(rename = "InventoryPlacement")]
        inventory_placement: PackableList
}

// Returns info related to your monarch, patron and vassals.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdate")]
pub struct AllegianceAllegianceUpdate {
        #[serde(rename = "Rank")]
        rank: u32,
        #[serde(rename = "Profile")]
        profile: AllegianceProfile
}

// Friends list update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_FriendsUpdate")]
pub struct SocialFriendsUpdate {
        #[serde(rename = "Friends")]
        friends: PackableList,
        #[serde(rename = "Type")]
        type_: FriendsUpdateType
}

// Store an item in a container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ServerSaysContainId")]
pub struct ItemServerSaysContainId {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "SlotIndex")]
        slot_index: u32,
        #[serde(rename = "ContainerType")]
        container_type: ContainerProperties
}

// Equip an item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_WearItem")]
pub struct ItemWearItem {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Slot")]
        slot: EquipMask
}

// Titles for the current character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_CharacterTitleTable")]
pub struct SocialCharacterTitleTable {
        #[serde(rename = "DisplayTitle")]
        display_title: u32,
        #[serde(rename = "Titles")]
        titles: PackableList
}

// Set a title for the current character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_AddOrSetCharacterTitle")]
pub struct SocialAddOrSetCharacterTitle {
        #[serde(rename = "NewTitle")]
        new_title: u32,
        #[serde(rename = "SetAsDisplayTitle")]
        set_as_display_title: bool
}

// Close Container - Only sent when explicitly closed
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_StopViewingObjectContents")]
pub struct ItemStopViewingObjectContents {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Open the buy/sell panel for a merchant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Vendor_VendorInfo")]
pub struct VendorVendorInfo {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Profile")]
        profile: VendorProfile,
        #[serde(rename = "Items")]
        items: PackableList
}

// Opens barber UI
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_StartBarber")]
pub struct CharacterStartBarber {
        #[serde(rename = "BasePalette")]
        base_palette: DataId,
        #[serde(rename = "HeadObject")]
        head_object: DataId,
        #[serde(rename = "HeadTexture")]
        head_texture: DataId,
        #[serde(rename = "DefaultHeadTexture")]
        default_head_texture: DataId,
        #[serde(rename = "EyesTexture")]
        eyes_texture: DataId,
        #[serde(rename = "DefaultEyesTexture")]
        default_eyes_texture: DataId,
        #[serde(rename = "NoseTexture")]
        nose_texture: DataId,
        #[serde(rename = "DefaultNoseTexture")]
        default_nose_texture: DataId,
        #[serde(rename = "MouthTexture")]
        mouth_texture: DataId,
        #[serde(rename = "DefaultMouthTexture")]
        default_mouth_texture: DataId,
        #[serde(rename = "SkinPalette")]
        skin_palette: DataId,
        #[serde(rename = "HairPalette")]
        hair_palette: DataId,
        #[serde(rename = "EyesPalette")]
        eyes_palette: DataId,
        #[serde(rename = "SetupId")]
        setup_id: DataId,
        #[serde(rename = "Option1")]
        option1: i32,
        #[serde(rename = "Option2")]
        option2: i32
}

// Member left fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Quit")]
pub struct FellowshipQuit {
        #[serde(rename = "Disband")]
        disband: bool
}

// Member dismissed from fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Dismiss")]
pub struct FellowshipDismiss {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Sent when you first open a book, contains the entire table of contents.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookOpen")]
pub struct WritingBookOpen {
        #[serde(rename = "BookId")]
        book_id: ObjectId,
        #[serde(rename = "MaxNumPages")]
        max_num_pages: u32,
        #[serde(rename = "PageData")]
        page_data: PageDataList,
        #[serde(rename = "Inscription")]
        inscription: String,
        #[serde(rename = "ScribeId")]
        scribe_id: ObjectId,
        #[serde(rename = "ScribeName")]
        scribe_name: String
}

// Response to an attempt to add a page to a book.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookAddPageResponse")]
pub struct WritingBookAddPageResponse {
        #[serde(rename = "BookId")]
        book_id: ObjectId,
        #[serde(rename = "PageNumber")]
        page_number: u32,
        #[serde(rename = "Success")]
        success: bool
}

// Response to an attempt to delete a page from a book.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookDeletePageResponse")]
pub struct WritingBookDeletePageResponse {
        #[serde(rename = "BookId")]
        book_id: ObjectId,
        #[serde(rename = "PageNumber")]
        page_number: u32,
        #[serde(rename = "Success")]
        success: bool
}

// Contains the text of a single page of a book, parchment or sign.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookPageDataResponse")]
pub struct WritingBookPageDataResponse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Page")]
        page: u32,
        #[serde(rename = "PageData")]
        page_data: PageData
}

// Get Inscription Response, doesn't seem to be really used by client
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_GetInscriptionResponse")]
pub struct ItemGetInscriptionResponse {
        #[serde(rename = "Inscription")]
        inscription: String,
        #[serde(rename = "ScribeName")]
        scribe_name: String,
        #[serde(rename = "ScribeAccount")]
        scribe_account: String
}

// The result of an attempt to assess an item or creature.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_SetAppraiseInfo")]
pub struct ItemSetAppraiseInfo {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Success")]
        success: bool,
        #[serde(rename = "IntProperties")]
        int_properties: PackableHashTable,
        #[serde(rename = "Int64Properties")]
        int64_properties: PackableHashTable,
        #[serde(rename = "BoolProperties")]
        bool_properties: PackableHashTable,
        #[serde(rename = "FloatProperties")]
        float_properties: PackableHashTable,
        #[serde(rename = "StringProperties")]
        string_properties: PackableHashTable,
        #[serde(rename = "DataIdProperties")]
        data_id_properties: PackableHashTable,
        #[serde(rename = "SpellBook")]
        spell_book: PackableList,
        #[serde(rename = "ArmorProfile")]
        armor_profile: ArmorProfile,
        #[serde(rename = "CreatureProfile")]
        creature_profile: CreatureAppraisalProfile,
        #[serde(rename = "WeaponProfile")]
        weapon_profile: WeaponProfile,
        #[serde(rename = "HookProfile")]
        hook_profile: HookAppraisalProfile,
        #[serde(rename = "ArmorHighlight")]
        armor_highlight: ArmorHighlightMask,
        #[serde(rename = "ArmorColor")]
        armor_color: ArmorHighlightMask,
        #[serde(rename = "WeaponHighlight")]
        weapon_highlight: WeaponHighlightMask,
        #[serde(rename = "WeaponColor")]
        weapon_color: WeaponHighlightMask,
        #[serde(rename = "ResistHighlight")]
        resist_highlight: ResistHighlightMask,
        #[serde(rename = "ResistColor")]
        resist_color: ResistHighlightMask,
        #[serde(rename = "BaseArmorHead")]
        base_armor_head: u32,
        #[serde(rename = "BaseArmorChest")]
        base_armor_chest: u32,
        #[serde(rename = "BaseArmorGroin")]
        base_armor_groin: u32,
        #[serde(rename = "BaseArmorBicep")]
        base_armor_bicep: u32,
        #[serde(rename = "BaseArmorWrist")]
        base_armor_wrist: u32,
        #[serde(rename = "BaseArmorHand")]
        base_armor_hand: u32,
        #[serde(rename = "BaseArmorThigh")]
        base_armor_thigh: u32,
        #[serde(rename = "BaseArmorShin")]
        base_armor_shin: u32,
        #[serde(rename = "BaseArmorFoot")]
        base_armor_foot: u32
}

// ChannelBroadcast: Group Chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelBroadcast")]
pub struct CommunicationChannelBroadcast {
        #[serde(rename = "Channel")]
        channel: Channel,
        #[serde(rename = "Message")]
        message: String
}

// ChannelList: Provides list of characters listening to a channel, I assume in response to a command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelList")]
pub struct CommunicationChannelList {
        #[serde(rename = "Characters")]
        characters: PackableList
}

// ChannelIndex: Provides list of channels available to the player, I assume in response to a command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelIndex")]
pub struct CommunicationChannelIndex {
        #[serde(rename = "Channels")]
        channels: PackableList
}

// Set Pack Contents
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_OnViewContents")]
pub struct ItemOnViewContents {
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList
}

// ServerSaysMoveItem: Removes an item from inventory (when you place it on the ground or give it away)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ServerSaysMoveItem")]
pub struct ItemServerSaysMoveItem {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// HandleAttackDoneEvent: Melee attack completed
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleAttackDoneEvent")]
pub struct CombatHandleAttackDoneEvent {
        #[serde(rename = "Number")]
        number: u32
}

// RemoveSpell: Delete a spell from your spellbook.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_RemoveSpell")]
pub struct MagicRemoveSpell {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// You just died.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleVictimNotificationEventSelf")]
pub struct CombatHandleVictimNotificationEventSelf {
        #[serde(rename = "Message")]
        message: String
}

// Message for a death, something you killed or your own death message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleVictimNotificationEventOther")]
pub struct CombatHandleVictimNotificationEventOther {
        #[serde(rename = "Message")]
        message: String
}

// HandleAttackerNotificationEvent: You have hit your target with a melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleAttackerNotificationEvent")]
pub struct CombatHandleAttackerNotificationEvent {
        #[serde(rename = "DefenderName")]
        defender_name: String,
        #[serde(rename = "Type")]
        type_: DamageType,
        #[serde(rename = "DamagePercent")]
        damage_percent: f32,
        #[serde(rename = "Damage")]
        damage: u32,
        #[serde(rename = "Critical")]
        critical: bool,
        #[serde(rename = "AttackConditions")]
        attack_conditions: AttackConditionsMask
}

// HandleDefenderNotificationEvent: You have been hit by a creature's melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleDefenderNotificationEvent")]
pub struct CombatHandleDefenderNotificationEvent {
        #[serde(rename = "AttackerName")]
        attacker_name: String,
        #[serde(rename = "Type")]
        type_: DamageType,
        #[serde(rename = "DamagePercent")]
        damage_percent: f32,
        #[serde(rename = "Damage")]
        damage: u32,
        #[serde(rename = "Location")]
        location: DamageLocation,
        #[serde(rename = "Critical")]
        critical: bool,
        #[serde(rename = "AttackConditions")]
        attack_conditions: AttackConditionsMask
}

// HandleEvasionAttackerNotificationEvent: Your target has evaded your melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleEvasionAttackerNotificationEvent")]
pub struct CombatHandleEvasionAttackerNotificationEvent {
        #[serde(rename = "DefenderName")]
        defender_name: String
}

// HandleEvasionDefenderNotificationEvent: You have evaded a creature's melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleEvasionDefenderNotificationEvent")]
pub struct CombatHandleEvasionDefenderNotificationEvent {
        #[serde(rename = "AttackerName")]
        attacker_name: String
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
        object_id: ObjectId,
        #[serde(rename = "Health")]
        health: f32
}

// QueryAgeResponse: happens when you do /age in the game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_QueryAgeResponse")]
pub struct CharacterQueryAgeResponse {
        #[serde(rename = "TargetName")]
        target_name: String,
        #[serde(rename = "Age")]
        age: String
}

// UseDone: Ready. Previous action complete
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UseDone")]
pub struct ItemUseDone {
        #[serde(rename = "FailureType")]
        failure_type: WeenieError
}

// Allegiance update finished
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdateDone")]
pub struct AllegianceAllegianceUpdateDone {
        #[serde(rename = "FailureType")]
        failure_type: WeenieError
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
        unknown: u32
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
        squelch_db: SquelchDB
}

// RegisterTrade: Send to begin a trade and display the trade window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_RegisterTrade")]
pub struct TradeRegisterTrade {
        #[serde(rename = "InitiatorId")]
        initiator_id: ObjectId,
        #[serde(rename = "PartnerId")]
        partner_id: ObjectId,
        #[serde(rename = "Stamp")]
        stamp: i64
}

// OpenTrade: Open trade window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_OpenTrade")]
pub struct TradeOpenTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// CloseTrade: End trading
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_CloseTrade")]
pub struct TradeCloseTrade {
        #[serde(rename = "Reason")]
        reason: EndTradeReason
}

// RemoveFromTrade: Item was removed from trade window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_AddToTrade")]
pub struct TradeAddToTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Side")]
        side: TradeSide
}

// Removes an item from the trade window, not sure if this is used still?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_RemoveFromTrade")]
pub struct TradeRemoveFromTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Side")]
        side: TradeSide
}

// AcceptTrade: The trade was accepted
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_AcceptTrade")]
pub struct TradeAcceptTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// DeclineTrade: The trade was declined
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_DeclineTrade")]
pub struct TradeDeclineTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// ResetTrade: The trade window was reset
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_ResetTrade")]
pub struct TradeResetTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// TradeFailure: Failure to add a trade item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_TradeFailure")]
pub struct TradeTradeFailure {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Reason")]
        reason: u32
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
        object_id: ObjectId,
        #[serde(rename = "Profile")]
        profile: HouseProfile
}

// House panel information for owners.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseData")]
pub struct HouseHouseData {
        #[serde(rename = "Data")]
        data: HouseData
}

// House Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseStatus")]
pub struct HouseHouseStatus {
        #[serde(rename = "NoticeType")]
        notice_type: u32
}

// Update Rent Time
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRentTime")]
pub struct HouseUpdateRentTime {
        #[serde(rename = "RentTime")]
        rent_time: u32
}

// Update Rent Payment
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRentPayment")]
pub struct HouseUpdateRentPayment {
        #[serde(rename = "Rent")]
        rent: PackableList
}

// Update Restrictions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRestrictions")]
pub struct HouseUpdateRestrictions {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "Restrictions")]
        restrictions: RestrictionDB
}

// House Guest List
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateHAR")]
pub struct HouseUpdateHAR {
        #[serde(rename = "GuestList")]
        guest_list: HAR
}

// House Profile
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseTransaction")]
pub struct HouseHouseTransaction {
        #[serde(rename = "NoticeType")]
        notice_type: u32
}

// Update an item's mana bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_QueryItemManaResponse")]
pub struct ItemQueryItemManaResponse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Mana")]
        mana: f32,
        #[serde(rename = "Success")]
        success: bool
}

// Display a list of available dwellings in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AvailableHouses")]
pub struct HouseAvailableHouses {
        #[serde(rename = "Type")]
        type_: HouseType,
        #[serde(rename = "Houses")]
        houses: PackableList,
        #[serde(rename = "NumHouses")]
        num_houses: i32
}

// Display a confirmation panel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationRequest")]
pub struct CharacterConfirmationRequest {
        #[serde(rename = "ConfirmationType")]
        confirmation_type: ConfirmationType,
        #[serde(rename = "ContextId")]
        context_id: u32,
        #[serde(rename = "Text")]
        text: String
}

// Confirmation done
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationDone")]
pub struct CharacterConfirmationDone {
        #[serde(rename = "ConfirmationType")]
        confirmation_type: ConfirmationType,
        #[serde(rename = "ContextId")]
        context_id: u32
}

// Display an allegiance login/logout message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceLoginNotificationEvent")]
pub struct AllegianceAllegianceLoginNotificationEvent {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId,
        #[serde(rename = "IsLoggedIn")]
        is_logged_in: bool
}

// Returns data for a player's allegiance information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceInfoResponseEvent")]
pub struct AllegianceAllegianceInfoResponseEvent {
        #[serde(rename = "TargetId")]
        target_id: ObjectId,
        #[serde(rename = "Profile")]
        profile: AllegianceProfile
}

// Joining game response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_JoinGameResponse")]
pub struct GameJoinGameResponse {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32
}

// Start game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_StartGame")]
pub struct GameStartGame {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32
}

// Move response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_MoveResponse")]
pub struct GameMoveResponse {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "MoveResult")]
        move_result: ChessMoveResult
}

// Opponent Turn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_OpponentTurn")]
pub struct GameOpponentTurn {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32,
        #[serde(rename = "GameMove")]
        game_move: GameMoveData
}

// Opponent Stalemate State
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_OpponentStalemateState")]
pub struct GameOpponentStalemateState {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32,
        #[serde(rename = "On")]
        on: bool
}

// Display a status message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_WeenieError")]
pub struct CommunicationWeenieError {
        #[serde(rename = "Type")]
        type_: WeenieError
}

// Display a parameterized status message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_WeenieErrorWithString")]
pub struct CommunicationWeenieErrorWithString {
        #[serde(rename = "Type")]
        type_: WeenieErrorWithString,
        #[serde(rename = "Text")]
        text: String
}

// End of Chess game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_GameOver")]
pub struct GameGameOver {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "TeamWinner")]
        team_winner: i32
}

// Set Turbine Chat channel numbers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChatRoomTracker")]
pub struct CommunicationChatRoomTracker {
        #[serde(rename = "AllegianceRoomId")]
        allegiance_room_id: u32,
        #[serde(rename = "GeneralChatRoomId")]
        general_chat_room_id: u32,
        #[serde(rename = "TradeChatRoomId")]
        trade_chat_room_id: u32,
        #[serde(rename = "LFGChatRoomId")]
        lfg_chat_room_id: u32,
        #[serde(rename = "RoleplayChatRoomId")]
        roleplay_chat_room_id: u32,
        #[serde(rename = "OlthoiChatRoomId")]
        olthoi_chat_room_id: u32,
        #[serde(rename = "SocietyChatRoomId")]
        society_chat_room_id: u32,
        #[serde(rename = "SocietyCelestialHandChatRoomId")]
        society_celestial_hand_chat_room_id: u32,
        #[serde(rename = "SocietyEldrichWebChatRoomId")]
        society_eldrich_web_chat_room_id: u32,
        #[serde(rename = "SocietyRadiantBloodChatRoomId")]
        society_radiant_blood_chat_room_id: u32
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
        result: SalvageOperationsResultData
}

// Someone has sent you a @tell.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearDirectSpeech")]
pub struct CommunicationHearDirectSpeech {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "TargetId")]
        target_id: ObjectId,
        #[serde(rename = "Type")]
        type_: ChatFragmentType,
        #[serde(rename = "SecretFlags")]
        secret_flags: u32
}

// Create or join a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_FullUpdate")]
pub struct FellowshipFullUpdate {
        #[serde(rename = "Fellowship")]
        fellowship: Fellowship
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
        fellow: Fellow,
        #[serde(rename = "UpdateType")]
        update_type: FellowUpdateType
}

// Add a spell to your spellbook.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateSpell")]
pub struct MagicUpdateSpell {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Apply an enchantment to your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateEnchantment")]
pub struct MagicUpdateEnchantment {
        #[serde(rename = "Enchantment")]
        enchantment: Enchantment
}

// Remove an enchantment from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_RemoveEnchantment")]
pub struct MagicRemoveEnchantment {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Update multiple enchantments from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_UpdateMultipleEnchantments")]
pub struct MagicUpdateMultipleEnchantments {
        #[serde(rename = "Enchantments")]
        enchantments: PackableList
}

// Remove multiple enchantments from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_RemoveMultipleEnchantments")]
pub struct MagicRemoveMultipleEnchantments {
        #[serde(rename = "Enchantments")]
        enchantments: PackableList
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
        spell_id: LayeredSpellId
}

// Silently remove multiple enchantments from your character (no message in the chat window).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_DispelMultipleEnchantments")]
pub struct MagicDispelMultipleEnchantments {
        #[serde(rename = "Enchantments")]
        enchantments: PackableList
}

// A portal storm is brewing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Misc_PortalStormBrewing")]
pub struct MiscPortalStormBrewing {
        #[serde(rename = "Extent")]
        extent: f32
}

// A portal storm is imminent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Misc_PortalStormImminent")]
pub struct MiscPortalStormImminent {
        #[serde(rename = "Extent")]
        extent: f32
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
        message: String
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
        contract_tracker: ContractTrackerTable
}

// Updates a contract data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendClientContractTracker")]
pub struct SocialSendClientContractTracker {
        #[serde(rename = "ContractTracker")]
        contract_tracker: ContractTracker,
        #[serde(rename = "DeleteContract")]
        delete_contract: bool,
        #[serde(rename = "SetAsDisplayContract")]
        set_as_display_contract: bool
}

// Sent every time an object you are aware of ceases to exist. Merely running out of range does not generate this message - in that case, the client just automatically stops tracking it after receiving no updates for a while (which I presume is a very short while).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ServerSaysRemove")]
pub struct ItemServerSaysRemove {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Failure to give an item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ServerSaysAttemptFailed")]
pub struct CharacterServerSaysAttemptFailed {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Reason")]
        reason: WeenieError
}

// For stackable items, this changes the number of items in the stack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UpdateStackSize")]
pub struct ItemUpdateStackSize {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Amount")]
        amount: u32,
        #[serde(rename = "NewValue")]
        new_value: u32
}

// A Player Kill occurred nearby (also sent for suicides).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandlePlayerDeathEvent")]
pub struct CombatHandlePlayerDeathEvent {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "KilledId")]
        killed_id: ObjectId,
        #[serde(rename = "KillerId")]
        killer_id: ObjectId
}

// Remove an int property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveIntEvent")]
pub struct QualitiesPrivateRemoveIntEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyInt
}

// Remove an int property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveIntEvent")]
pub struct QualitiesRemoveIntEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyInt
}

// Remove a bool property from the charactert
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveBoolEvent")]
pub struct QualitiesPrivateRemoveBoolEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyBool
}

// Remove a bool property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveBoolEvent")]
pub struct QualitiesRemoveBoolEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyBool
}

// Remove a float property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveFloatEvent")]
pub struct QualitiesPrivateRemoveFloatEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyFloat
}

// Remove a float property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveFloatEvent")]
pub struct QualitiesRemoveFloatEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyFloat
}

// Remove a string property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveStringEvent")]
pub struct QualitiesPrivateRemoveStringEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyString
}

// Remove a string property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveStringEvent")]
pub struct QualitiesRemoveStringEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyString
}

// Remove an dataId property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveDataIdEvent")]
pub struct QualitiesPrivateRemoveDataIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyDataId
}

// Remove an dataId property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveDataIdEvent")]
pub struct QualitiesRemoveDataIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyDataId
}

// Remove an instanceId property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveInstanceIdEvent")]
pub struct QualitiesPrivateRemoveInstanceIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyInstanceId
}

// Remove an instanceId property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveInstanceIdEvent")]
pub struct QualitiesRemoveInstanceIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyInstanceId
}

// Remove a position property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemovePositionEvent")]
pub struct QualitiesPrivateRemovePositionEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyPosition
}

// Remove a position property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemovePositionEvent")]
pub struct QualitiesRemovePositionEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyPosition
}

// Remove an int64 property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemoveInt64Event")]
pub struct QualitiesPrivateRemoveInt64Event {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyInt64
}

// Remove an int64 property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveInt64Event")]
pub struct QualitiesRemoveInt64Event {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyInt64
}

// Set or update a Character Int property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateInt")]
pub struct QualitiesPrivateUpdateInt {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyInt,
        #[serde(rename = "Value")]
        value: i32
}

// Set or update an Object Int property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateInt")]
pub struct QualitiesUpdateInt {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyInt,
        #[serde(rename = "Value")]
        value: i32
}

// Set or update a Character Int64 property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateInt64")]
pub struct QualitiesPrivateUpdateInt64 {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyInt64,
        #[serde(rename = "Value")]
        value: i64
}

// Set or update a Character Int64 property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateInt64")]
pub struct QualitiesUpdateInt64 {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyInt64,
        #[serde(rename = "Value")]
        value: i64
}

// Set or update a Character Boolean property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateBool")]
pub struct QualitiesPrivateUpdateBool {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyBool,
        #[serde(rename = "Value")]
        value: bool
}

// Set or update an Object Boolean property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateBool")]
pub struct QualitiesUpdateBool {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyBool,
        #[serde(rename = "Value")]
        value: bool
}

// Set or update an Object float property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateFloat")]
pub struct QualitiesPrivateUpdateFloat {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyFloat,
        #[serde(rename = "Value")]
        value: f32
}

// Set or update an Object float property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateFloat")]
pub struct QualitiesUpdateFloat {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyFloat,
        #[serde(rename = "Value")]
        value: f32
}

// Set or update an Object String property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateString")]
pub struct QualitiesPrivateUpdateString {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyString,
        #[serde(rename = "Value")]
        value: String
}

// Set or update an Object String property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateString")]
pub struct QualitiesUpdateString {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyString,
        #[serde(rename = "Value")]
        value: String
}

// Set or update an Object DId property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateDataId")]
pub struct QualitiesPrivateUpdateDataId {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyDataId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update an Object DId property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateDataId")]
pub struct QualitiesUpdateDataId {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyDataId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a IId property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateInstanceId")]
pub struct QualitiesPrivateUpdateInstanceId {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyInstanceId,
        #[serde(rename = "Value")]
        value: ObjectId
}

// Set or update an Object IId property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateInstanceId")]
pub struct QualitiesUpdateInstanceId {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyInstanceId,
        #[serde(rename = "Value")]
        value: ObjectId
}

// Set or update a Character Position property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdatePosition")]
pub struct QualitiesPrivateUpdatePosition {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyPosition,
        #[serde(rename = "Value")]
        value: Position
}

// Set or update a Character Position property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdatePosition")]
pub struct QualitiesUpdatePosition {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyPosition,
        #[serde(rename = "Value")]
        value: Position
}

// Set or update a Character Skill value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateSkill")]
pub struct QualitiesPrivateUpdateSkill {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: Skill
}

// Set or update a Character Skill value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateSkill")]
pub struct QualitiesUpdateSkill {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: Skill
}

// Set or update a Character Skill Level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateSkillLevel")]
pub struct QualitiesPrivateUpdateSkillLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Skill Level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateSkillLevel")]
pub struct QualitiesUpdateSkillLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Skill state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateSkillAC")]
pub struct QualitiesPrivateUpdateSkillAC {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: SkillAdvancementClass
}

// Set or update a Character Skill state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateSkillAC")]
pub struct QualitiesUpdateSkillAC {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: SkillAdvancementClass
}

// Set or update a Character Attribute value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateAttribute")]
pub struct QualitiesPrivateUpdateAttribute {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: AttributeId,
        #[serde(rename = "Value")]
        value: AttributeInfo
}

// Set or update a Character Attribute value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateAttribute")]
pub struct QualitiesUpdateAttribute {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: AttributeId,
        #[serde(rename = "Value")]
        value: AttributeInfo
}

// Set or update a Character Attribute Level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateAttributeLevel")]
pub struct QualitiesPrivateUpdateAttributeLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: AttributeId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Attribute Level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateAttributeLevel")]
pub struct QualitiesUpdateAttributeLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: AttributeId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Vital value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateAttribute2nd")]
pub struct QualitiesPrivateUpdateAttribute2nd {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: VitalId,
        #[serde(rename = "Value")]
        value: SecondaryAttributeInfo
}

// Set or update a Character Vital value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateAttribute2nd")]
pub struct QualitiesUpdateAttribute2nd {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: VitalId,
        #[serde(rename = "Value")]
        value: SecondaryAttributeInfo
}

// Set or update a Character Vital value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateAttribute2ndLevel")]
pub struct QualitiesPrivateUpdateAttribute2ndLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: CurVitalId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Vital value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateAttribute2ndLevel")]
pub struct QualitiesUpdateAttribute2ndLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: CurVitalId,
        #[serde(rename = "Value")]
        value: u32
}

// Indirect '/e' text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearEmote")]
pub struct CommunicationHearEmote {
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "Text")]
        text: String
}

// Contains the text associated with an emote action.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearSoulEmote")]
pub struct CommunicationHearSoulEmote {
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "Text")]
        text: String
}

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearSpeech")]
pub struct CommunicationHearSpeech {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "Type")]
        type_: ChatFragmentType
}

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearRangedSpeech")]
pub struct CommunicationHearRangedSpeech {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "Range")]
        range: f32,
        #[serde(rename = "Type")]
        type_: ChatFragmentType
}

// This appears to be an admin command to change the environment (light, fog, sounds, colors)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_Environs")]
pub struct AdminEnvirons {
        #[serde(rename = "EnvrionOption")]
        envrion_option: EnvrionChangeType
}

// Sets both the position and movement, such as when materializing at a lifestone
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_PositionAndMovementEvent")]
pub struct MovementPositionAndMovementEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Position")]
        position: PositionPack,
        #[serde(rename = "MovementData")]
        movement_data: MovementData
}

// Sent whenever a character changes their clothes. It contains the entire description of what their wearing (and possibly their facial features as well). This message is only sent for changes, when the character is first created, the body of this message is included inside the creation message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ObjDescEvent")]
pub struct ItemObjDescEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectDescription")]
        object_description: ObjDesc,
        #[serde(rename = "InstanceSequence")]
        instance_sequence: u16,
        #[serde(rename = "VisualDescSequence")]
        visual_desc_sequence: u16
}

// Sets the player visual desc, TODO confirm this
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SetPlayerVisualDesc")]
pub struct CharacterSetPlayerVisualDesc {
        #[serde(rename = "ObjectDescription")]
        object_description: ObjDesc
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
        seconds_remaining: u32
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
        status: u32,
        #[serde(rename = "Characters")]
        characters: PackableList,
        #[serde(rename = "DeletedCharacters")]
        deleted_characters: PackableList,
        #[serde(rename = "NumAllowedCharacters")]
        num_allowed_characters: u32,
        #[serde(rename = "Account")]
        account: String,
        #[serde(rename = "UseTurbineChat")]
        use_turbine_chat: bool,
        #[serde(rename = "HasThroneofDestiny")]
        has_throneof_destiny: bool
}

// Failure to log in
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterError")]
pub struct CharacterCharacterError {
        #[serde(rename = "Reason")]
        reason: CharacterErrorType
}

// Create an object somewhere in the world
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_CreateObject")]
pub struct ItemCreateObject {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectDescription")]
        object_description: ObjDesc,
        #[serde(rename = "PhysicsDescription")]
        physics_description: PhysicsDesc,
        #[serde(rename = "WeenieDescription")]
        weenie_description: PublicWeenieDesc
}

// Login of player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_CreatePlayer")]
pub struct LoginCreatePlayer {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId
}

// Sent whenever an object is being deleted from the scene.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_DeleteObject")]
pub struct ItemDeleteObject {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16
}

// Sets the position/motion of an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_PositionEvent")]
pub struct MovementPositionEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Position")]
        position: PositionPack
}

// Sets the parent for an object, eg. equipting an object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ParentEvent")]
pub struct ItemParentEvent {
        #[serde(rename = "ParentId")]
        parent_id: ObjectId,
        #[serde(rename = "ChildId")]
        child_id: ObjectId,
        #[serde(rename = "Location")]
        location: ParentLocation,
        #[serde(rename = "Placement")]
        placement: Placement,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ChildPositionSequence")]
        child_position_sequence: u16
}

// Sent when picking up an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_PickupEvent")]
pub struct InventoryPickupEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectPositionSequence")]
        object_position_sequence: u16
}

// Set's the current state of the object. Client appears to only process the following state changes post creation: NoDraw, LightingOn, Hidden
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_SetState")]
pub struct ItemSetState {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "NewState")]
        new_state: PhysicsState,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectStateSequence")]
        object_state_sequence: u16
}

// These are animations. Whenever a human, monster or object moves - one of these little messages is sent. Even idle emotes (like head scratching and nodding) are sent in this manner.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_SetObjectMovement")]
pub struct MovementSetObjectMovement {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "MovementData")]
        movement_data: MovementData
}

// Changes an objects vector, for things like jumping
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_VectorUpdate")]
pub struct MovementVectorUpdate {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Velocity")]
        velocity: Vector3,
        #[serde(rename = "Omega")]
        omega: Vector3,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectVectorSequence")]
        object_vector_sequence: u16
}

// Applies a sound effect.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_SoundEvent")]
pub struct EffectsSoundEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "SoundType")]
        sound_type: Sound,
        #[serde(rename = "Volume")]
        volume: f32
}

// Instructs the client to show the portal graphic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayerTeleport")]
pub struct EffectsPlayerTeleport {
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16
}

// Instructs the client to play a script. (Not seen so far, maybe prefered PlayScriptType)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayScriptId")]
pub struct EffectsPlayScriptId {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ScriptId")]
        script_id: DataId
}

// Applies an effect with visual and sound by providing the type to be looked up in the Physics Script Table
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayScriptType")]
pub struct EffectsPlayScriptType {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ScriptType")]
        script_type: i32,
        #[serde(rename = "Speed")]
        speed: f32
}

// Account has been banned
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AccountBanned")]
pub struct LoginAccountBanned {
        #[serde(rename = "BannedUntil")]
        banned_until: u32,
        #[serde(rename = "Text")]
        text: String
}

// Admin Receive Account Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_ReceiveAccountData")]
pub struct AdminReceiveAccountData {
        #[serde(rename = "Unknown")]
        unknown: u32,
        #[serde(rename = "AdminAccountData")]
        admin_account_data: PackableList
}

// Admin Receive Player Data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_ReceivePlayerData")]
pub struct AdminReceivePlayerData {
        #[serde(rename = "Unknown")]
        unknown: i32,
        #[serde(rename = "AdminPlayerData")]
        admin_player_data: PackableList
}

// Update an existing object's data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UpdateObject")]
pub struct ItemUpdateObject {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectDesc")]
        object_desc: ObjDesc,
        #[serde(rename = "PhysicsDesc")]
        physics_desc: PhysicsDesc,
        #[serde(rename = "WeenieDesc")]
        weenie_desc: PublicWeenieDesc
}

// Account has been booted
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AccountBooted")]
pub struct LoginAccountBooted {
        #[serde(rename = "AdditionalReasonText")]
        additional_reason_text: String,
        #[serde(rename = "ReasonText")]
        reason_text: String
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
        text: String,
        #[serde(rename = "Type")]
        type_: ChatFragmentType
}

// The name of the current world.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_WorldInfo")]
pub struct LoginWorldInfo {
        #[serde(rename = "Connections")]
        connections: u32,
        #[serde(rename = "MaxConnections")]
        max_connections: u32,
        #[serde(rename = "WorldName")]
        world_name: String
}

// Add or update a dat file Resource.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_DataMessage")]
#[serde(tag = "Compression")]
pub enum DDDDataMessage {
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
    },
}

// DDD error
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_ErrorMessage")]
pub struct DDDErrorMessage {
        #[serde(rename = "ResourceType")]
        resource_type: u32,
        #[serde(rename = "ResourceId")]
        resource_id: DataId,
        #[serde(rename = "RError")]
        r_error: u32
}

// A list of dat files that need to be patched
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_BeginDDDMessage")]
pub struct DDDBeginDDDMessage {
        #[serde(rename = "DataExpected")]
        data_expected: u32,
        #[serde(rename = "Revisions")]
        revisions: PackableList
}

// Add or update a dat file Resource.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_InterrogationMessage")]
pub struct DDDInterrogationMessage {
        #[serde(rename = "ServersRegion")]
        servers_region: u32,
        #[serde(rename = "NameRuleLanguage")]
        name_rule_language: u32,
        #[serde(rename = "ProductId")]
        product_id: u32,
        #[serde(rename = "SupportedLanguages")]
        supported_languages: PackableList
}

// Ends DDD update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_OnEndDDD")]
pub struct DDDOnEndDDD {}

