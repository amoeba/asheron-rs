use serde::{Serialize, Deserialize};

// Set a single character option.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_PlayerOptionChangedEvent")]
pub struct CharacterPlayerOptionChangedEvent {
    #[serde(rename = "Option")]
    pub option: CharacterOptions1,
    #[serde(rename = "Value")]
    pub value: bool,
}

// Starts a melee attack against a target
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_TargetedMeleeAttack")]
pub struct CombatTargetedMeleeAttack {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Height")]
    pub height: AttackHeight,
    #[serde(rename = "Power")]
    pub power: f32,
}

// Starts a missle attack against a target
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_TargetedMissileAttack")]
pub struct CombatTargetedMissileAttack {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Height")]
    pub height: AttackHeight,
    #[serde(rename = "Accuracy")]
    pub accuracy: f32,
}

// Set AFK mode.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_SetAFKMode")]
pub struct CommunicationSetAFKMode {
    #[serde(rename = "AFK")]
    pub afk: bool,
}

// Set AFK message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_SetAFKMessage")]
pub struct CommunicationSetAFKMessage {
    #[serde(rename = "Message")]
    pub message: String,
}

// Talking
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_Talk")]
pub struct CommunicationTalk {
    #[serde(rename = "Message")]
    pub message: String,
}

// Removes a friend
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_RemoveFriend")]
pub struct SocialRemoveFriend {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Adds a friend
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_AddFriend")]
pub struct SocialAddFriend {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

// Store an item in a container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_PutItemInContainer")]
pub struct InventoryPutItemInContainer {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ContainerId")]
    pub container_id: ObjectId,
    #[serde(rename = "SlotIndex")]
    pub slot_index: u32,
}

// Gets and weilds an item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_GetAndWieldItem")]
pub struct InventoryGetAndWieldItem {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Slot")]
    pub slot: EquipMask,
}

// Drop an item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_DropItem")]
pub struct InventoryDropItem {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Swear allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SwearAllegiance")]
pub struct AllegianceSwearAllegiance {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Break allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_BreakAllegiance")]
pub struct AllegianceBreakAllegiance {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Allegiance update request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_UpdateRequest")]
pub struct AllegianceUpdateRequest {
    #[serde(rename = "On")]
    pub on: bool,
}

// Clears friend list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_ClearFriends")]
pub struct SocialClearFriends {}

// Teleport to the PKLite Arena
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_TeleToPKLArena")]
pub struct CharacterTeleToPKLArena {}

// Teleport to the PK Arena
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_TeleToPKArena")]
pub struct CharacterTeleToPKArena {}

// Sets a character's display title
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SetDisplayCharacterTitle")]
pub struct SocialSetDisplayCharacterTitle {
    #[serde(rename = "TitleId")]
    pub title_id: u32,
}

// Query the allegiance name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_QueryAllegianceName")]
pub struct AllegianceQueryAllegianceName {}

// Clears the allegiance name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_ClearAllegianceName")]
pub struct AllegianceClearAllegianceName {}

// Direct message by Id
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TalkDirect")]
pub struct CommunicationTalkDirect {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
}

// Sets the allegiance name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceName")]
pub struct AllegianceSetAllegianceName {
    #[serde(rename = "Name")]
    pub name: String,
}

// Attempt to use an item with a target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_UseWithTargetEvent")]
pub struct InventoryUseWithTargetEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
}

// Attempt to use an item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_UseEvent")]
pub struct InventoryUseEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Sets an allegiance officer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceOfficer")]
pub struct AllegianceSetAllegianceOfficer {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "Level")]
    pub level: AllegianceOfficerLevel,
}

// Sets an allegiance officer title for a given level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceOfficerTitle")]
pub struct AllegianceSetAllegianceOfficerTitle {
    #[serde(rename = "Level")]
    pub level: AllegianceOfficerLevel,
    #[serde(rename = "Title")]
    pub title: String,
}

// List the allegiance officer titles
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_ListAllegianceOfficerTitles")]
pub struct AllegianceListAllegianceOfficerTitles {}

// Clears the allegiance officer titles
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_ClearAllegianceOfficerTitles")]
pub struct AllegianceClearAllegianceOfficerTitles {}

// Perform the allegiance lock action
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_DoAllegianceLockAction")]
pub struct AllegianceDoAllegianceLockAction {
    #[serde(rename = "Action")]
    pub action: AllegianceLockAction,
}

// Sets a person as an approved vassal
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceApprovedVassal")]
pub struct AllegianceSetAllegianceApprovedVassal {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

// Gags a person in allegiance chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceChatGag")]
pub struct AllegianceAllegianceChatGag {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "On")]
    pub on: bool,
}

// Perform the allegiance house action
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_DoAllegianceHouseAction")]
pub struct AllegianceDoAllegianceHouseAction {
    #[serde(rename = "Action")]
    pub action: AllegianceHouseAction,
}

// Spend XP to raise a vital.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainAttribute2nd")]
pub struct TrainTrainAttribute2nd {
    #[serde(rename = "Type")]
    pub type_: VitalId,
    #[serde(rename = "Experience")]
    pub experience: u32,
}

// Spend XP to raise an attribute.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainAttribute")]
pub struct TrainTrainAttribute {
    #[serde(rename = "Type")]
    pub type_: AttributeId,
    #[serde(rename = "Experience")]
    pub experience: u32,
}

// Spend XP to raise a skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainSkill")]
pub struct TrainTrainSkill {
    #[serde(rename = "Skill")]
    pub skill: SkillId,
    #[serde(rename = "Experience")]
    pub experience: u32,
}

// Spend skill credits to train a skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainSkillAdvancementClass")]
pub struct TrainTrainSkillAdvancementClass {
    #[serde(rename = "Skill")]
    pub skill: SkillId,
    #[serde(rename = "Credits")]
    pub credits: u32,
}

// Cast a spell with no target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_CastUntargetedSpell")]
pub struct MagicCastUntargetedSpell {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

// Cast a spell on a target
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_CastTargetedSpell")]
pub struct MagicCastTargetedSpell {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

// Changes the combat mode
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_ChangeCombatMode")]
pub struct CombatChangeCombatMode {
    #[serde(rename = "Mode")]
    pub mode: CombatMode,
}

// Merges one stack with another
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_StackableMerge")]
pub struct InventoryStackableMerge {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

// Split a stack and place it into a container
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_StackableSplitToContainer")]
pub struct InventoryStackableSplitToContainer {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ContainerId")]
    pub container_id: ObjectId,
    #[serde(rename = "SlotIndex")]
    pub slot_index: u32,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

// Split a stack and place it into the world
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_StackableSplitTo3D")]
pub struct InventoryStackableSplitTo3D {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

// Changes an account squelch
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ModifyCharacterSquelch")]
pub struct CommunicationModifyCharacterSquelch {
    #[serde(rename = "Add")]
    pub add: bool,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

// Changes an account squelch
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ModifyAccountSquelch")]
pub struct CommunicationModifyAccountSquelch {
    #[serde(rename = "Add")]
    pub add: bool,
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

// Changes the global filters, /filter -type as well as /chat and /notell
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ModifyGlobalSquelch")]
pub struct CommunicationModifyGlobalSquelch {
    #[serde(rename = "Add")]
    pub add: bool,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

// Direct message by name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TalkDirectByName")]
pub struct CommunicationTalkDirectByName {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

// Buy from a vendor
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Vendor_Buy")]
pub struct VendorBuy {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ItemProfile>,
    #[serde(rename = "AlternateCurrencyId")]
    pub alternate_currency_id: u32,
}

// Sell to a vendor
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Vendor_Sell")]
pub struct VendorSell {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ItemProfile>,
}

// Teleport to your lifestone. (/lifestone)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_TeleToLifestone")]
pub struct CharacterTeleToLifestone {}

// The client is ready for the character to materialize after portalling or logging on.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_LoginCompleteNotification")]
pub struct CharacterLoginCompleteNotification {}

// Create a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Create")]
pub struct FellowshipCreate {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ShareXP")]
    pub share_xp: bool,
}

// Quit the fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Quit")]
pub struct FellowshipQuit {
    #[serde(rename = "Disband")]
    pub disband: bool,
}

// Dismiss a player from the fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Dismiss")]
pub struct FellowshipDismiss {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Recruit a player to the fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Recruit")]
pub struct FellowshipRecruit {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Update request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_UpdateRequest")]
pub struct FellowshipUpdateRequest {
    #[serde(rename = "On")]
    pub on: bool,
}

// Request update to book data (seems to be sent after failed add page)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookAddPage")]
pub struct WritingBookAddPage {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Updates a page in a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookModifyPage")]
pub struct WritingBookModifyPage {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "PageNum")]
    pub page_num: i32,
    #[serde(rename = "PageText")]
    pub page_text: String,
}

// Add a page to a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookData")]
pub struct WritingBookData {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Removes a page from a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookDeletePage")]
pub struct WritingBookDeletePage {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "PageNum")]
    pub page_num: i32,
}

// Requests data for a page in a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookPageData")]
pub struct WritingBookPageData {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "PageNum")]
    pub page_num: i32,
}

// Sets the inscription on an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_SetInscription")]
pub struct WritingSetInscription {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Inscription")]
    pub inscription: String,
}

// Appraise something
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_Appraise")]
pub struct ItemAppraise {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Give an item to someone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_GiveObjectRequest")]
pub struct InventoryGiveObjectRequest {
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

// Advocate Teleport
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Advocate_Teleport")]
pub struct AdvocateTeleport {
    #[serde(rename = "ObjectId")]
    pub object_id: String,
    #[serde(rename = "Destination")]
    pub destination: Position,
}

// Sends an abuse report.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AbuseLogRequest")]
pub struct CharacterAbuseLogRequest {
    #[serde(rename = "Character")]
    pub character: String,
    #[serde(rename = "Status")]
    pub status: u32,
    #[serde(rename = "Complaint")]
    pub complaint: String,
}

// Joins a chat channel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_AddToChannel")]
pub struct CommunicationAddToChannel {
    #[serde(rename = "Channel")]
    pub channel: Channel,
}

// Leaves a chat channel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_RemoveFromChannel")]
pub struct CommunicationRemoveFromChannel {
    #[serde(rename = "Channel")]
    pub channel: Channel,
}

// Sends a message to a chat channel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelBroadcast")]
pub struct CommunicationChannelBroadcast {
    #[serde(rename = "Channel")]
    pub channel: Channel,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "Message")]
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelList")]
pub struct CommunicationChannelList {
    #[serde(rename = "Channel")]
    pub channel: Channel,
}

// Requests a channel index
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelIndex")]
pub struct CommunicationChannelIndex {}

// Stop viewing the contents of a container
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_NoLongerViewingContents")]
pub struct InventoryNoLongerViewingContents {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Splits an item to a wield location.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_StackableSplitToWield")]
pub struct InventoryStackableSplitToWield {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Slot")]
    pub slot: EquipMask,
    #[serde(rename = "Amount")]
    pub amount: i32,
}

// Add an item to the shortcut bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AddShortCut")]
pub struct CharacterAddShortCut {
    #[serde(rename = "Shortcut")]
    pub shortcut: ShortCutData,
}

// Remove an item from the shortcut bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RemoveShortCut")]
pub struct CharacterRemoveShortCut {
    #[serde(rename = "Index")]
    pub index: u32,
}

// Set multiple character options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterOptionsEvent")]
pub struct CharacterCharacterOptionsEvent {
    #[serde(rename = "Options")]
    pub options: PlayerModule,
}

// Removes a spell from the spell book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_RemoveSpell")]
pub struct MagicRemoveSpell {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

// Cancels an attack
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_CancelAttack")]
pub struct CombatCancelAttack {}

// Query's a creatures health
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_QueryHealth")]
pub struct CombatQueryHealth {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Query a character's age
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_QueryAge")]
pub struct CharacterQueryAge {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Query a character's birth day
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_QueryBirth")]
pub struct CharacterQueryBirth {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Emote message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_Emote")]
pub struct CommunicationEmote {
    #[serde(rename = "Message")]
    pub message: String,
}

// Soul emote message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_SoulEmote")]
pub struct CommunicationSoulEmote {
    #[serde(rename = "Message")]
    pub message: String,
}

// Add a spell to a spell bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AddSpellFavorite")]
pub struct CharacterAddSpellFavorite {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
    #[serde(rename = "Index")]
    pub index: u32,
    #[serde(rename = "SpellBar")]
    pub spell_bar: u32,
}

// Remove a spell from a spell bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RemoveSpellFavorite")]
pub struct CharacterRemoveSpellFavorite {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
    #[serde(rename = "SpellBar")]
    pub spell_bar: u32,
}

// Request a ping
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RequestPing")]
pub struct CharacterRequestPing {}

// Starts trading with another player.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_OpenTradeNegotiations")]
pub struct TradeOpenTradeNegotiations {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Ends trading, when trade window is closed?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_CloseTradeNegotiations")]
pub struct TradeCloseTradeNegotiations {}

// Adds an object to the trade.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_AddToTrade")]
pub struct TradeAddToTrade {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "SlotIndex")]
    pub slot_index: u32,
}

// Accepts a trade.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_AcceptTrade")]
pub struct TradeAcceptTrade {
    #[serde(rename = "Contents")]
    pub contents: Trade,
}

// Declines a trade, when cancel is clicked?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_DeclineTrade")]
pub struct TradeDeclineTrade {}

// Resets the trade, when clear all is clicked?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_ResetTrade")]
pub struct TradeResetTrade {}

// Clears the player's corpse looting consent list, /consent clear
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ClearPlayerConsentList")]
pub struct CharacterClearPlayerConsentList {}

// Display the player's corpse looting consent list, /consent who 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_DisplayPlayerConsentList")]
pub struct CharacterDisplayPlayerConsentList {}

// Remove your corpse looting permission for the given player, /consent remove 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RemoveFromPlayerConsentList")]
pub struct CharacterRemoveFromPlayerConsentList {
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

// Grants a player corpse looting permission, /permit add
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AddPlayerPermission")]
pub struct CharacterAddPlayerPermission {
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

// Buy a house
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_BuyHouse")]
pub struct HouseBuyHouse {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ObjectId>,
}

// Query your house info, during signin
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_QueryHouse")]
pub struct HouseQueryHouse {}

// Abandons your house
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AbandonHouse")]
pub struct HouseAbandonHouse {}

// Revokes a player's corpse looting permission, /permit remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RemovePlayerPermission")]
pub struct CharacterRemovePlayerPermission {
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

// Pay rent for a house
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_RentHouse")]
pub struct HouseRentHouse {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ObjectId>,
}

// Sets a new fill complevel for a component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SetDesiredComponentLevel")]
pub struct CharacterSetDesiredComponentLevel {
    #[serde(rename = "Wcid")]
    pub wcid: u32,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

// Adds a guest to your house guest list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AddPermanentGuest")]
pub struct HouseAddPermanentGuest {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
}

// Removes a specific player from your house guest list, /house guest remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_RemovePermanentGuest")]
pub struct HouseRemovePermanentGuest {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
}

// Sets your house open status
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_SetOpenHouseStatus")]
pub struct HouseSetOpenHouseStatus {
    #[serde(rename = "OpenHouse")]
    pub open_house: bool,
}

// Changes a specific players storage permission, /house storage add/remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ChangeStoragePermission")]
pub struct HouseChangeStoragePermission {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
    #[serde(rename = "HasPermission")]
    pub has_permission: bool,
}

// Boots a specific player from your house /house boot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_BootSpecificHouseGuest")]
pub struct HouseBootSpecificHouseGuest {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
}

// Removes all storage permissions, /house storage remove_all
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_RemoveAllStoragePermission")]
pub struct HouseRemoveAllStoragePermission {}

// Requests your full guest list, /house guest list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_RequestFullGuestList")]
pub struct HouseRequestFullGuestList {}

// Sets the allegiance message of the day, /allegiance motd set
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetMotd")]
pub struct AllegianceSetMotd {
    #[serde(rename = "Message")]
    pub message: String,
}

// Query the motd, /allegiance motd
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_QueryMotd")]
pub struct AllegianceQueryMotd {}

// Clear the motd, /allegiance motd clear
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_ClearMotd")]
pub struct AllegianceClearMotd {}

// Gets SlumLord info, sent after getting a failed house transaction
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_QueryLord")]
pub struct HouseQueryLord {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Adds all to your storage permissions, /house storage add -all
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AddAllStoragePermission")]
pub struct HouseAddAllStoragePermission {}

// Removes all guests, /house guest remove_all
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_RemoveAllPermanentGuests")]
pub struct HouseRemoveAllPermanentGuests {}

// Boot everyone from your house, /house boot -all
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_BootEveryone")]
pub struct HouseBootEveryone {}

// Teleports you to your house, /house recall
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_TeleToHouse")]
pub struct HouseTeleToHouse {}

// Queries an item's mana
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_QueryItemMana")]
pub struct ItemQueryItemMana {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Modify whether house hooks are visibile or not, /house hooks on/off
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_SetHooksVisibility")]
pub struct HouseSetHooksVisibility {
    #[serde(rename = "Visible")]
    pub visible: bool,
}

// Modify whether allegiance members are guests, /house guest add_allegiance/remove_allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ModifyAllegianceGuestPermission")]
pub struct HouseModifyAllegianceGuestPermission {
    #[serde(rename = "Add")]
    pub add: bool,
}

// Modify whether allegiance members can access storage, /house storage add_allegiance/remove_allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ModifyAllegianceStoragePermission")]
pub struct HouseModifyAllegianceStoragePermission {
    #[serde(rename = "Add")]
    pub add: bool,
}

// Joins a chess game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_Join")]
pub struct GameJoin {
    #[serde(rename = "GameId")]
    pub game_id: u32,
    #[serde(rename = "Team")]
    pub team: u32,
}

// Quits a chess game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_Quit")]
pub struct GameQuit {}

// Makes a chess move
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_Move")]
pub struct GameMove {
    #[serde(rename = "XFrom")]
    pub x_from: i32,
    #[serde(rename = "YFrom")]
    pub y_from: i32,
    #[serde(rename = "XTo")]
    pub x_to: i32,
    #[serde(rename = "YTo")]
    pub y_to: i32,
}

// Skip a move?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_MovePass")]
pub struct GameMovePass {}

// Offer or confirm stalemate
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Game_Stalemate")]
pub struct GameStalemate {
    #[serde(rename = "On")]
    pub on: bool,
}

// Lists available house /house available
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ListAvailableHouses")]
pub struct HouseListAvailableHouses {
    #[serde(rename = "Type")]
    pub type_: HouseType,
}

// Confirms a dialog
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationResponse")]
pub struct CharacterConfirmationResponse {
    #[serde(rename = "Type")]
    pub type_: ConfirmationType,
    #[serde(rename = "Context")]
    pub context: u32,
    #[serde(rename = "Accepted")]
    pub accepted: bool,
}

// Boots a player from the allegiance, optionally all characters on their account
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_BreakAllegianceBoot")]
pub struct AllegianceBreakAllegianceBoot {
    #[serde(rename = "BooteeName")]
    pub bootee_name: String,
    #[serde(rename = "AccountBoot")]
    pub account_boot: bool,
}

// Teleports player to their allegiance housing, /house mansion_recall
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_TeleToMansion")]
pub struct HouseTeleToMansion {}

// Player is commiting suicide
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_Suicide")]
pub struct CharacterSuicide {}

// Request allegiance info for a player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceInfoRequest")]
pub struct AllegianceAllegianceInfoRequest {
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

// Salvages items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_CreateTinkeringTool")]
pub struct InventoryCreateTinkeringTool {
    #[serde(rename = "ToolId")]
    pub tool_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ObjectId>,
}

// Changes the spell book filter
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SpellbookFilterEvent")]
pub struct CharacterSpellbookFilterEvent {
    #[serde(rename = "Options")]
    pub options: SpellBookFilterOptions,
}

// Teleport to the marketplace
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_TeleToMarketplace")]
pub struct CharacterTeleToMarketplace {}

// Enter PKLite mode
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_EnterPKLite")]
pub struct CharacterEnterPKLite {}

// Fellowship Assign a new leader
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_AssignNewLeader")]
pub struct FellowshipAssignNewLeader {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// Fellowship Change openness
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_ChangeFellowOpeness")]
pub struct FellowshipChangeFellowOpeness {
    #[serde(rename = "Open")]
    pub open: bool,
}

// Boots a player from the allegiance chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceChatBoot")]
pub struct AllegianceAllegianceChatBoot {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "Reason")]
    pub reason: String,
}

// Bans a player from the allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AddAllegianceBan")]
pub struct AllegianceAddAllegianceBan {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

// Removes a player ban from the allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_RemoveAllegianceBan")]
pub struct AllegianceRemoveAllegianceBan {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

// Display allegiance bans
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_ListAllegianceBans")]
pub struct AllegianceListAllegianceBans {}

// Removes an allegiance officer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_RemoveAllegianceOfficer")]
pub struct AllegianceRemoveAllegianceOfficer {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

// List allegiance officers
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_ListAllegianceOfficers")]
pub struct AllegianceListAllegianceOfficers {}

// Clear allegiance officers
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_ClearAllegianceOfficers")]
pub struct AllegianceClearAllegianceOfficers {}

// Recalls to players allegiance hometown
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_RecallAllegianceHometown")]
pub struct AllegianceRecallAllegianceHometown {}

// Admin Returns a plugin list response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_QueryPluginListResponse")]
pub struct AdminQueryPluginListResponse {
    #[serde(rename = "Context")]
    pub context: u32,
    #[serde(rename = "PluginList")]
    pub plugin_list: String,
}

// Admin Returns plugin info
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_QueryPluginResponse")]
pub struct AdminQueryPluginResponse {
    #[serde(rename = "Context")]
    pub context: u32,
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "PluginName")]
    pub plugin_name: String,
    #[serde(rename = "PluginAuthor")]
    pub plugin_author: String,
    #[serde(rename = "PluginEmail")]
    pub plugin_email: String,
    #[serde(rename = "PluginWebpage")]
    pub plugin_webpage: String,
}

// Completes the barber interaction
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_FinishBarber")]
pub struct CharacterFinishBarber {
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

// Abandons a contract
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_AbandonContract")]
pub struct SocialAbandonContract {
    #[serde(rename = "ContractId")]
    pub contract_id: ContractId,
}

// Performs a jump
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_Jump")]
pub struct MovementJump {
    #[serde(rename = "Jump")]
    pub jump: JumpPack,
}

// Move to state data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_MoveToState")]
pub struct MovementMoveToState {
    #[serde(rename = "MoveToState")]
    pub move_to_state: MoveToStatePack,
}

// Performs a movement based on input
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_DoMovementCommand")]
pub struct MovementDoMovementCommand {
    #[serde(rename = "Motion")]
    pub motion: u32,
    #[serde(rename = "Speed")]
    pub speed: f32,
    #[serde(rename = "HoldKey")]
    pub hold_key: HoldKey,
}

// Stops a movement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_StopMovementCommand")]
pub struct MovementStopMovementCommand {
    #[serde(rename = "Motion")]
    pub motion: u32,
    #[serde(rename = "HoldKey")]
    pub hold_key: HoldKey,
}

// Sets an autonomy level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_AutonomyLevel")]
pub struct MovementAutonomyLevel {
    #[serde(rename = "AutonomyLevel")]
    pub autonomy_level: u32,
}

// Sends an autonomous position
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_AutonomousPosition")]
pub struct MovementAutonomousPosition {
    #[serde(rename = "Position")]
    pub position: AutonomousPositionPack,
}

// Performs a non autonomous jump
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_Jump_NonAutonomous")]
pub struct MovementJumpNonAutonomous {
    #[serde(rename = "Extent")]
    pub extent: f32,
}

// Instructs the client to return to 2D mode - the character list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_LogOffCharacter")]
pub struct LoginLogOffCharacter {}

// Mark a character for deletetion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterDelete")]
pub struct CharacterCharacterDelete {
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "Slot")]
    pub slot: i32,
}

// Character creation result
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SendCharGenResult")]
pub struct CharacterSendCharGenResult {
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "Result")]
    pub result: CharGenResult,
}

// The character to log in.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_SendEnterWorld")]
pub struct LoginSendEnterWorld {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
    #[serde(rename = "Account")]
    pub account: String,
}

// Asks server for a new object description
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Object_SendForceObjdesc")]
pub struct ObjectSendForceObjdesc {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

// The user has clicked 'Enter'. This message does not contain the Id of the character logging on; that comes later.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_SendEnterWorldRequest")]
pub struct LoginSendEnterWorldRequest {}

// Sent if player is an PSR, I assume it displays the server version number
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_SendAdminGetServerVersion")]
pub struct AdminSendAdminGetServerVersion {}

// Seems to be a legacy friends command, /friends old, for when Jan 2006 event changed the friends list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendFriendsCommand")]
pub struct SocialSendFriendsCommand {
    #[serde(rename = "Command")]
    pub command: u32,
    #[serde(rename = "Player")]
    pub player: String,
}

// Admin command to restore a character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_SendAdminRestoreCharacter")]
pub struct AdminSendAdminRestoreCharacter {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "RestoredCharName")]
    pub restored_char_name: String,
    #[serde(rename = "AccountToRestoreTo")]
    pub account_to_restore_to: String,
}

// Send or receive a message using Turbine Chat.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TurbineChat")]
#[serde(tag = "BlobDispatchType")]
pub enum CommunicationTurbineChat {
    #[serde(rename = "0x01")]
    #[serde(alias = "0x02")]
    Type01 {
    #[serde(rename = "MmessageSize")]
    mmessage_size: u32,
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

// DDD request for data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_RequestDataMessage")]
pub struct DDDRequestDataMessage {
    #[serde(rename = "ResourceType")]
    pub resource_type: u32,
    #[serde(rename = "ResourceId")]
    pub resource_id: DataId,
}

// TODO
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_InterrogationResponseMessage")]
pub struct DDDInterrogationResponseMessage {
    #[serde(rename = "Language")]
    pub language: u32,
    #[serde(rename = "Files")]
    pub files: PackableList<long>,
}

// Ends DDD message update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_EndDDDMessage")]
pub struct DDDEndDDDMessage {}

// Ends DDD update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_OnEndDDD")]
pub struct DDDOnEndDDD {}

