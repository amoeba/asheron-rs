use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::enums::*;

#[allow(non_camel_case_types)]
pub type byte = u8;

#[allow(non_camel_case_types)]
pub type short = i16;

#[allow(non_camel_case_types)]
pub type ushort = u16;

#[allow(non_camel_case_types)]
pub type int = i32;

#[allow(non_camel_case_types)]
pub type uint = u32;

#[allow(non_camel_case_types)]
pub type long = i64;

#[allow(non_camel_case_types)]
pub type ulong = u64;

#[allow(non_camel_case_types)]
pub type float = f32;

#[allow(non_camel_case_types)]
pub type double = f64;

#[allow(non_camel_case_types)]
pub type string = String;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct WString(pub String);

#[allow(non_camel_case_types)]
pub type WORD = u16;

#[allow(non_camel_case_types)]
pub type DWORD = u32;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackedWORD {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackedDWORD {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ObjectId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct LandcellId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SpellId(pub u16);

#[allow(non_camel_case_types)]
pub type DataId = PackedDWORD;

// Full spell Id combining the spell id with the spell layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub struct LayeredSpellId {
    #[serde(rename = "Id")]
    pub id: SpellId,
    #[serde(rename = "Layer")]
    pub layer: u16,
}

// List which is packable for network
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackableList<T> {
    #[serde(rename = "Count")]
    pub count: u32,
    #[serde(rename = "List")]
    pub list: Vec<T>,
}

// HashTable which is packable for network
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackableHashTable<T: std::cmp::Eq + std::hash::Hash, U> {
    #[serde(rename = "Count")]
    pub count: u16,
    #[serde(rename = "MaxSize")]
    pub max_size: u16,
    #[serde(rename = "Table")]
    pub table: std::collections::HashMap<T, U>,
}

// HashTable which is packable for network
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PHashTable<T: std::cmp::Eq + std::hash::Hash, U> {
    #[serde(rename = "PackedSize")]
    pub packed_size: u32,
    #[serde(rename = "Table")]
    pub table: std::collections::HashMap<T, U>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    #[serde(rename = "X")]
    pub x: f32,
    #[serde(rename = "Y")]
    pub y: f32,
    #[serde(rename = "Z")]
    pub z: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    #[serde(rename = "W")]
    pub w: f32,
    #[serde(rename = "X")]
    pub x: f32,
    #[serde(rename = "Y")]
    pub y: f32,
    #[serde(rename = "Z")]
    pub z: f32,
}

// Landcell location, without orientation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Origin {
    #[serde(rename = "Landcell")]
    pub landcell: LandcellId,
    #[serde(rename = "Location")]
    pub location: Vector3,
}

// Landcell location, including orientation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "Landcell")]
    pub landcell: LandcellId,
    #[serde(rename = "Frame")]
    pub frame: Frame,
}

// A the location and orientation of an object within a landcell
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    #[serde(rename = "Origin")]
    pub origin: Vector3,
    #[serde(rename = "Orientation")]
    pub orientation: Quaternion,
}

// Optional header data when PacketHeaderFlags includes ServerSwitch
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerSwitchHeader {
    #[serde(rename = "Sequence")]
    pub sequence: u32,
    #[serde(rename = "Type")]
    pub type_: ServerSwitchType,
}

// Optional header data when PacketHeaderFlags includes CICMDCommand
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CICMDCommandHeader {
    #[serde(rename = "Command")]
    pub command: u32,
    #[serde(rename = "Parameter")]
    pub parameter: u32,
}

// Optional header data when PacketHeaderFlags includes Flow
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlowHeader {
    #[serde(rename = "Bytes")]
    pub bytes: u32,
    #[serde(rename = "Interval")]
    pub interval: u16,
}

// Optional header data when PacketHeaderFlags includes LogonServerAddr
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocketAddress {
    #[serde(rename = "Family")]
    pub family: i16,
    #[serde(rename = "Port")]
    pub port: u16,
    #[serde(rename = "Address")]
    pub address: u32,
    #[serde(rename = "Empty")]
    pub empty: u64,
}

// Optional header data when PacketHeaderFlags includes LoginRequest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginRequestHeaderType2 {
    #[serde(rename = "ClientVersion")]
    pub client_version: String,
    #[serde(rename = "Length")]
    pub length: u32,
    #[serde(rename = "Flags")]
    pub flags: AuthFlags,
    #[serde(rename = "Sequence")]
    pub sequence: u32,
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "AccountToLoginAs")]
    pub account_to_login_as: String,
    #[serde(rename = "Password")]
    pub password: WString,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginRequestHeaderType40000002 {
    #[serde(rename = "ClientVersion")]
    pub client_version: String,
    #[serde(rename = "Length")]
    pub length: u32,
    #[serde(rename = "Flags")]
    pub flags: AuthFlags,
    #[serde(rename = "Sequence")]
    pub sequence: u32,
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "AccountToLoginAs")]
    pub account_to_login_as: String,
    #[serde(rename = "GlsTicket")]
    pub gls_ticket: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "AuthType")]
pub enum LoginRequestHeader {
    Type2(LoginRequestHeaderType2),
    Type40000002(LoginRequestHeaderType40000002),
}

// Optional header data when PacketHeaderFlags includes Referral
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferralHeader {
    #[serde(rename = "Cookie")]
    pub cookie: u64,
    #[serde(rename = "Address")]
    pub address: SocketAddress,
    #[serde(rename = "IdServer")]
    pub id_server: u16,
    #[serde(rename = "Unknown")]
    pub unknown: DWORD,
}

// Optional header data when PacketHeaderFlags includes ConnectRequest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectRequestHeader {
    #[serde(rename = "ServerTime")]
    pub server_time: f64,
    #[serde(rename = "Cookie")]
    pub cookie: u64,
    #[serde(rename = "NetID")]
    pub net_id: i32,
    #[serde(rename = "OutgoingSeed")]
    pub outgoing_seed: u32,
    #[serde(rename = "IncomingSeed")]
    pub incoming_seed: u32,
    #[serde(rename = "Unknown")]
    pub unknown: DWORD,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetError {
    #[serde(rename = "StringId")]
    pub string_id: DataId,
    #[serde(rename = "TableId")]
    pub table_id: DataId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EchoResponseHeader {
    #[serde(rename = "LocalTime")]
    pub local_time: f32,
    #[serde(rename = "HoldingTime")]
    pub holding_time: f32,
}

// A collection of property tables.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ACBaseQualities {
    #[serde(rename = "Flags")]
    pub flags: ACBaseQualitiesFlags,
    #[serde(rename = "WeenieType")]
    pub weenie_type: WeenieType,
    #[serde(rename = "IntProperties", skip_serializing_if = "Option::is_none")]
    pub int_properties: Option<PackableHashTable<PropertyInt, i32>>,
    #[serde(rename = "Int64Properties", skip_serializing_if = "Option::is_none")]
    pub int64_properties: Option<PackableHashTable<PropertyInt64, i64>>,
    #[serde(rename = "BoolProperties", skip_serializing_if = "Option::is_none")]
    pub bool_properties: Option<PackableHashTable<PropertyBool, bool>>,
    #[serde(rename = "FloatProperties", skip_serializing_if = "Option::is_none")]
    pub float_properties: Option<PackableHashTable<PropertyFloat, f64>>,
    #[serde(rename = "StringProperties", skip_serializing_if = "Option::is_none")]
    pub string_properties: Option<PackableHashTable<PropertyString, String>>,
    #[serde(rename = "DataProperties", skip_serializing_if = "Option::is_none")]
    pub data_properties: Option<PackableHashTable<PropertyDataId, DataId>>,
    #[serde(rename = "InstanceProperties", skip_serializing_if = "Option::is_none")]
    pub instance_properties: Option<PackableHashTable<PropertyInstanceId, ObjectId>>,
    #[serde(rename = "PositionProperties", skip_serializing_if = "Option::is_none")]
    pub position_properties: Option<PackableHashTable<PropertyPosition, Position>>,
}

// The ACQualities structure contains character property lists.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ACQualities {
    #[serde(rename = "Flags")]
    pub flags: ACQualitiesFlags,
    #[serde(rename = "HasHealth")]
    pub has_health: bool,
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributeCache>,
    #[serde(rename = "Skills", skip_serializing_if = "Option::is_none")]
    pub skills: Option<PackableHashTable<SkillId, Skill>>,
    #[serde(rename = "Body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
    #[serde(rename = "SpellBook", skip_serializing_if = "Option::is_none")]
    pub spell_book: Option<PackableHashTable<LayeredSpellId, SpellBookPage>>,
    #[serde(rename = "Enchantments", skip_serializing_if = "Option::is_none")]
    pub enchantments: Option<EnchantmentRegistry>,
    #[serde(rename = "EventFilter", skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<EventFilter>,
    #[serde(rename = "Emotes", skip_serializing_if = "Option::is_none")]
    pub emotes: Option<EmoteTable>,
    #[serde(rename = "CreationProfile", skip_serializing_if = "Option::is_none")]
    pub creation_profile: Option<PackableList<CreationProfile>>,
    #[serde(rename = "PageData", skip_serializing_if = "Option::is_none")]
    pub page_data: Option<PageDataList>,
    #[serde(rename = "Generators", skip_serializing_if = "Option::is_none")]
    pub generators: Option<GeneratorTable>,
    #[serde(rename = "GeneratorRegistry", skip_serializing_if = "Option::is_none")]
    pub generator_registry: Option<GeneratorRegistry>,
    #[serde(rename = "GeneratorQueue", skip_serializing_if = "Option::is_none")]
    pub generator_queue: Option<GeneratorQueue>,
}

// The AttributeCache structure contains information about a character attributes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeCache {
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "Strength", skip_serializing_if = "Option::is_none")]
    pub strength: Option<AttributeInfo>,
    #[serde(rename = "Endurance", skip_serializing_if = "Option::is_none")]
    pub endurance: Option<AttributeInfo>,
    #[serde(rename = "Quickness", skip_serializing_if = "Option::is_none")]
    pub quickness: Option<AttributeInfo>,
    #[serde(rename = "Coordination", skip_serializing_if = "Option::is_none")]
    pub coordination: Option<AttributeInfo>,
    #[serde(rename = "Focus", skip_serializing_if = "Option::is_none")]
    pub focus: Option<AttributeInfo>,
    #[serde(rename = "Self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<AttributeInfo>,
    #[serde(rename = "Health", skip_serializing_if = "Option::is_none")]
    pub health: Option<SecondaryAttributeInfo>,
    #[serde(rename = "Stamina", skip_serializing_if = "Option::is_none")]
    pub stamina: Option<SecondaryAttributeInfo>,
    #[serde(rename = "Mana", skip_serializing_if = "Option::is_none")]
    pub mana: Option<SecondaryAttributeInfo>,
}

// The Attribute structure contains information about a character attribute.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeInfo {
    #[serde(rename = "PointsRaised")]
    pub points_raised: u32,
    #[serde(rename = "InnatePoints")]
    pub innate_points: u32,
    #[serde(rename = "ExperienceSpent")]
    pub experience_spent: u32,
}

// The SecondaryAttribute structure contains information about a character vital.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecondaryAttributeInfo {
    #[serde(rename = "Attribute")]
    pub attribute: AttributeInfo,
    #[serde(rename = "Current")]
    pub current: u32,
}

// The Skill structure contains information about a character skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Skill {
    #[serde(rename = "PointsRaised")]
    pub points_raised: u16,
    #[serde(rename = "AdjustPP")]
    pub adjust_pp: u16,
    #[serde(rename = "TrainingLevel")]
    pub training_level: SkillAdvancementClass,
    #[serde(rename = "ExperienceSpent")]
    pub experience_spent: u32,
    #[serde(rename = "InnatePoints")]
    pub innate_points: u32,
    #[serde(rename = "ResistanceOfLastCheck")]
    pub resistance_of_last_check: u32,
    #[serde(rename = "LastUsedTime")]
    pub last_used_time: f64,
}

// Contains body part table
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "BodyParts")]
    pub body_parts: PackableHashTable<u32, BodyPart>,
}

// Information on individual body parts. (Needs to be confirmed if this was used in prod)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BodyPart {
    #[serde(rename = "HasBPSD")]
    pub has_bpsd: i32,
    #[serde(rename = "DamageType")]
    pub damage_type: DamageType,
    #[serde(rename = "DamageVal")]
    pub damage_val: i32,
    #[serde(rename = "DamageVar")]
    pub damage_var: i32,
    #[serde(rename = "ArmorCache")]
    pub armor_cache: ArmorCache,
    #[serde(rename = "BH")]
    pub bh: i32,
    #[serde(rename = "BPSD", skip_serializing_if = "Option::is_none")]
    pub bpsd: Option<BodyPartSelectionData>,
}

// Information on armor levels
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmorCache {
    #[serde(rename = "BaseArmor")]
    pub base_armor: i32,
    #[serde(rename = "ArmorVsSlash")]
    pub armor_vs_slash: i32,
    #[serde(rename = "ArmorVsPierce")]
    pub armor_vs_pierce: i32,
    #[serde(rename = "ArmorVsBludgeon")]
    pub armor_vs_bludgeon: i32,
    #[serde(rename = "ArmorVsCold")]
    pub armor_vs_cold: i32,
    #[serde(rename = "ArmorVsFire")]
    pub armor_vs_fire: i32,
    #[serde(rename = "ArmorVsAcid")]
    pub armor_vs_acid: i32,
    #[serde(rename = "ArmorVsElectric")]
    pub armor_vs_electric: i32,
    #[serde(rename = "ArmorVsNether")]
    pub armor_vs_nether: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BodyPartSelectionData {
    #[serde(rename = "HLF")]
    pub hlf: i32,
    #[serde(rename = "MLF")]
    pub mlf: i32,
    #[serde(rename = "LLF")]
    pub llf: i32,
    #[serde(rename = "HRF")]
    pub hrf: i32,
    #[serde(rename = "MRF")]
    pub mrf: i32,
    #[serde(rename = "LRF")]
    pub lrf: i32,
    #[serde(rename = "HLB")]
    pub hlb: i32,
    #[serde(rename = "MLB")]
    pub mlb: i32,
    #[serde(rename = "LLB")]
    pub llb: i32,
    #[serde(rename = "HRB")]
    pub hrb: i32,
    #[serde(rename = "MRB")]
    pub mrb: i32,
    #[serde(rename = "LRB")]
    pub lrb: i32,
}

// Contains information related to the spell in your spellbook
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpellBookPage {
    #[serde(rename = "CastingLikelihood")]
    pub casting_likelihood: f32,
    #[serde(rename = "CastingLikelihood2", skip_serializing_if = "Option::is_none")]
    pub casting_likelihood2: Option<f32>,
    #[serde(rename = "Unknown", skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i32>,
}

// Contains information related to the spells in effect on the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnchantmentRegistry {
    #[serde(rename = "Flags")]
    pub flags: EnchantmentRegistryFlags,
    #[serde(rename = "LifeSpells", skip_serializing_if = "Option::is_none")]
    pub life_spells: Option<PackableList<Enchantment>>,
    #[serde(rename = "CreatureSpells", skip_serializing_if = "Option::is_none")]
    pub creature_spells: Option<PackableList<Enchantment>>,
    #[serde(rename = "Vitae", skip_serializing_if = "Option::is_none")]
    pub vitae: Option<Enchantment>,
    #[serde(rename = "Cooldowns", skip_serializing_if = "Option::is_none")]
    pub cooldowns: Option<PackableList<Enchantment>>,
}

// The Enchantment structure describes an active enchantment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Enchantment {
    #[serde(rename = "Id")]
    pub id: LayeredSpellId,
    #[serde(rename = "HasEquipmentSet")]
    pub has_equipment_set: u16,
    #[serde(rename = "SpellCategory")]
    pub spell_category: SpellCategory,
    #[serde(rename = "PowerLevel")]
    pub power_level: u32,
    #[serde(rename = "StartTime")]
    pub start_time: f64,
    #[serde(rename = "Duration")]
    pub duration: f64,
    #[serde(rename = "CasterId")]
    pub caster_id: ObjectId,
    #[serde(rename = "DegradeModifier")]
    pub degrade_modifier: f32,
    #[serde(rename = "DegradeLimit")]
    pub degrade_limit: f32,
    #[serde(rename = "LastTimeDegraded")]
    pub last_time_degraded: f64,
    #[serde(rename = "StatMod")]
    pub stat_mod: StatMod,
    #[serde(rename = "EquipmentSet", skip_serializing_if = "Option::is_none")]
    pub equipment_set: Option<EquipmentSet>,
}

// Information on stat modification
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatMod {
    #[serde(rename = "Type")]
    pub type_: EnchantmentTypeFlags,
    #[serde(rename = "Key")]
    pub key: u32,
    #[serde(rename = "Value")]
    pub value: f32,
}

// Contains a list of events to filter? Unknown what this does currently.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventFilter {
    #[serde(rename = "Events")]
    pub events: PackableList<u32>,
}

// Contains a list of emotes for NPCs? Unknown what this does currently.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteTable {
    #[serde(rename = "Emotes")]
    pub emotes: PackableHashTable<EmoteCategory, EmoteSetList>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteSetList {
    #[serde(rename = "Emotes")]
    pub emotes: PackableList<EmoteSet>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteSetType1 {
    #[serde(rename = "Probability")]
    pub probability: f32,
    #[serde(rename = "Emotes")]
    pub emotes: PackableList<Emote>,
    #[serde(rename = "ClassId")]
    pub class_id: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteSetType2 {
    #[serde(rename = "Probability")]
    pub probability: f32,
    #[serde(rename = "Emotes")]
    pub emotes: PackableList<Emote>,
    #[serde(rename = "VendorType")]
    pub vendor_type: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteSetType5 {
    #[serde(rename = "Probability")]
    pub probability: f32,
    #[serde(rename = "Emotes")]
    pub emotes: PackableList<Emote>,
    #[serde(rename = "Style")]
    pub style: u32,
    #[serde(rename = "Substyle")]
    pub substyle: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteSetTypeC {
    #[serde(rename = "Probability")]
    pub probability: f32,
    #[serde(rename = "Emotes")]
    pub emotes: PackableList<Emote>,
    #[serde(rename = "Quest")]
    pub quest: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteSetTypeF {
    #[serde(rename = "Probability")]
    pub probability: f32,
    #[serde(rename = "Emotes")]
    pub emotes: PackableList<Emote>,
    #[serde(rename = "MinHealth")]
    pub min_health: f32,
    #[serde(rename = "MaxHealth")]
    pub max_health: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Category")]
pub enum EmoteSet {
    Type1(EmoteSetType1),
    Type2(EmoteSetType2),
    Type5(EmoteSetType5),
    TypeC(EmoteSetTypeC),
    TypeF(EmoteSetTypeF),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType1 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Message")]
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType2 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Amount64")]
    pub amount64: u64,
    #[serde(rename = "HeroXP64")]
    pub hero_xp64: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType3 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "CProfile")]
    pub c_profile: CreationProfile,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType4 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Frame")]
    pub frame: Frame,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType5 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Motion")]
    pub motion: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType7 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "PhysicsScript")]
    pub physics_script: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType9 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Sound")]
    pub sound: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteTypeE {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "SpellId")]
    pub spell_id: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType1C {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Amount")]
    pub amount: u32,
    #[serde(rename = "Stat")]
    pub stat: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType1E {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Min")]
    pub min: u32,
    #[serde(rename = "Max")]
    pub max: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType20 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType22 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType23 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Stat")]
    pub stat: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType24 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Min")]
    pub min: u32,
    #[serde(rename = "Max")]
    pub max: u32,
    #[serde(rename = "Stat")]
    pub stat: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType25 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "FMin")]
    pub f_min: f64,
    #[serde(rename = "FMax")]
    pub f_max: f64,
    #[serde(rename = "Stat")]
    pub stat: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType26 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "TestString")]
    pub test_string: String,
    #[serde(rename = "Stat")]
    pub stat: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType31 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Percent")]
    pub percent: f64,
    #[serde(rename = "Min64")]
    pub min64: u64,
    #[serde(rename = "Max64")]
    pub max64: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType32 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Stat")]
    pub stat: u32,
    #[serde(rename = "Percent")]
    pub percent: f64,
    #[serde(rename = "Min")]
    pub min: u32,
    #[serde(rename = "Max")]
    pub max: u32,
    #[serde(rename = "Display")]
    pub display: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType35 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Stat")]
    pub stat: u32,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType38 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "WealthRating")]
    pub wealth_rating: i32,
    #[serde(rename = "TreasureClass")]
    pub treasure_class: i32,
    #[serde(rename = "TreasureType")]
    pub treasure_type: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType3F {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Position")]
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType4C {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    pub msg: String,
    #[serde(rename = "CProfile")]
    pub c_profile: CreationProfile,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType6E {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Stat")]
    pub stat: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType70 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Amount64")]
    pub amount64: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType72 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Min64")]
    pub min64: u64,
    #[serde(rename = "Max64")]
    pub max64: u64,
    #[serde(rename = "Stat")]
    pub stat: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteType76 {
    #[serde(rename = "Delay")]
    pub delay: f32,
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Stat")]
    pub stat: u32,
    #[serde(rename = "Percent")]
    pub percent: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum Emote {
    Type1(EmoteType1),
    Type2(EmoteType2),
    Type3(EmoteType3),
    Type4(EmoteType4),
    Type5(EmoteType5),
    Type7(EmoteType7),
    Type9(EmoteType9),
    TypeE(EmoteTypeE),
    Type1C(EmoteType1C),
    Type1E(EmoteType1E),
    Type20(EmoteType20),
    Type22(EmoteType22),
    Type23(EmoteType23),
    Type24(EmoteType24),
    Type25(EmoteType25),
    Type26(EmoteType26),
    Type31(EmoteType31),
    Type32(EmoteType32),
    Type35(EmoteType35),
    Type38(EmoteType38),
    Type3F(EmoteType3F),
    Type4C(EmoteType4C),
    Type6E(EmoteType6E),
    Type70(EmoteType70),
    Type72(EmoteType72),
    Type76(EmoteType76),
}

// Set information about an item for creation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreationProfile {
    #[serde(rename = "WeenieClassId")]
    pub weenie_class_id: u32,
    #[serde(rename = "Palette")]
    pub palette: u32,
    #[serde(rename = "Shade")]
    pub shade: f32,
    #[serde(rename = "Destination")]
    pub destination: u32,
    #[serde(rename = "StackSize")]
    pub stack_size: i32,
    #[serde(rename = "TryToBond")]
    pub try_to_bond: bool,
}

// List of pages in a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageDataList {
    #[serde(rename = "MaxNumPages")]
    pub max_num_pages: u32,
    #[serde(rename = "MaxNumCharsPerPage")]
    pub max_num_chars_per_page: u32,
    #[serde(rename = "Pages")]
    pub pages: PackableList<PageData>,
}

// Data for an individual page
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageData {
    #[serde(rename = "AuthorId")]
    pub author_id: ObjectId,
    #[serde(rename = "AuthorName")]
    pub author_name: String,
    #[serde(rename = "AuthorAccount")]
    pub author_account: String,
    #[serde(rename = "Version")]
    pub version: u32,
    #[serde(rename = "TextIncluded")]
    pub text_included: bool,
    #[serde(rename = "IgnoreAuthor")]
    pub ignore_author: bool,
    #[serde(rename = "PageText", skip_serializing_if = "Option::is_none")]
    pub page_text: Option<String>,
}

// Blob fragment data used to contruct message data. These can be spread across multiple packets
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobFragments {
    #[serde(rename = "Sequence")]
    pub sequence: u32,
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "Count")]
    pub count: u16,
    #[serde(rename = "Size")]
    pub size: u16,
    #[serde(rename = "Index")]
    pub index: u16,
    #[serde(rename = "Group")]
    pub group: FragmentGroup,
    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorTable {
    #[serde(rename = "Generators")]
    pub generators: PackableList<GeneratorProfile>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorProfile {
    #[serde(rename = "Probability")]
    pub probability: f32,
    #[serde(rename = "TypeId")]
    pub type_id: u32,
    #[serde(rename = "Delay")]
    pub delay: f64,
    #[serde(rename = "InitCreate")]
    pub init_create: u32,
    #[serde(rename = "MaxNum")]
    pub max_num: u32,
    #[serde(rename = "WhenCreate")]
    pub when_create: u32,
    #[serde(rename = "WhereCreate")]
    pub where_create: u32,
    #[serde(rename = "StackSize")]
    pub stack_size: u32,
    #[serde(rename = "Ptid")]
    pub ptid: u32,
    #[serde(rename = "Shade")]
    pub shade: f32,
    #[serde(rename = "PosVal")]
    pub pos_val: Position,
    #[serde(rename = "Slot")]
    pub slot: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorRegistry {
    #[serde(rename = "Registry")]
    pub registry: PackableHashTable<u32, GeneratorRegistryNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorRegistryNode {
    #[serde(rename = "WcidOrType")]
    pub wcid_or_type: u32,
    #[serde(rename = "Ts")]
    pub ts: f64,
    #[serde(rename = "TreasureType")]
    pub treasure_type: u32,
    #[serde(rename = "Slot")]
    pub slot: u32,
    #[serde(rename = "Checkpointed")]
    pub checkpointed: u32,
    #[serde(rename = "Shop")]
    pub shop: u32,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

// Set of inventory items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorQueue {
    #[serde(rename = "Queue")]
    pub queue: PackableList<GeneratorQueueNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorQueueNode {
    #[serde(rename = "Slot")]
    pub slot: u32,
    #[serde(rename = "When")]
    pub when: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowPropertyType1000007F {
    #[serde(rename = "Unknown_j")]
    pub unknown_j: u32,
    #[serde(rename = "Value_j")]
    pub value_j: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowPropertyType10000086 {
    #[serde(rename = "Unknown_i")]
    pub unknown_i: u32,
    #[serde(rename = "Value_i")]
    pub value_i: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowPropertyType10000087 {
    #[serde(rename = "Unknown_h")]
    pub unknown_h: u32,
    #[serde(rename = "Value_h")]
    pub value_h: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowPropertyType10000088 {
    #[serde(rename = "Unknown_f")]
    pub unknown_f: u32,
    #[serde(rename = "Value_f")]
    pub value_f: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowPropertyType10000089 {
    #[serde(rename = "Unknown_e")]
    pub unknown_e: u32,
    #[serde(rename = "Value_e")]
    pub value_e: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowPropertyType1000008A {
    #[serde(rename = "Unknown_d")]
    pub unknown_d: u32,
    #[serde(rename = "Value_d")]
    pub value_d: u8,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowPropertyType1000008D {
    #[serde(rename = "Unknown_c")]
    pub unknown_c: u32,
    pub title_source: WindowPropertyType1000008DTitleSourceVariant,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowPropertyType1000008DTitleSourceVariantType0 {
    #[serde(rename = "StringId")]
    pub string_id: u32,
    #[serde(rename = "FileId")]
    pub file_id: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowPropertyType1000008DTitleSourceVariantType1 {
    #[serde(rename = "Value_a")]
    pub value_a: WString,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "TitleSource")]
pub enum WindowPropertyType1000008DTitleSourceVariant {
    #[serde(rename = "0x00")]
    Type0(WindowPropertyType1000008DTitleSourceVariantType0),
    #[serde(rename = "0x01")]
    Type1(WindowPropertyType1000008DTitleSourceVariantType1),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Key_a")]
pub enum WindowProperty {
    Type1000007F(WindowPropertyType1000007F),
    Type10000086(WindowPropertyType10000086),
    Type10000087(WindowPropertyType10000087),
    Type10000088(WindowPropertyType10000088),
    Type10000089(WindowPropertyType10000089),
    Type1000008A(WindowPropertyType1000008A),
    Type1000008D(WindowPropertyType1000008D),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowOptionType1000008B {
    #[serde(rename = "Unknown_b")]
    pub unknown_b: u8,
    #[serde(rename = "PropertyCount")]
    pub property_count: u8,
    #[serde(rename = "Properties")]
    pub properties: Vec<WindowProperty>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type_a")]
pub enum WindowOption {
    Type1000008B(WindowOptionType1000008B),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionPropertyType10000080 {
    pub unknown_l: u32,
    #[serde(rename = "inactiveOpacity")]
    pub inactive_opacity: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionPropertyType10000081 {
    pub unknown_k: u32,
    #[serde(rename = "activeOpacity")]
    pub active_opacity: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionPropertyType1000008C {
    #[serde(rename = "Unknown_a")]
    pub unknown_a: u32,
    #[serde(rename = "WindowOptions")]
    pub window_options: PackableList<WindowOption>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum OptionProperty {
    Type10000080(OptionPropertyType10000080),
    Type10000081(OptionPropertyType10000081),
    Type1000008C(OptionPropertyType1000008C),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameplayOptions {
    #[serde(rename = "Size")]
    pub size: u32,
    #[serde(rename = "Unknown200_2")]
    pub unknown200_2: u8,
    #[serde(rename = "OptionPropertyCount")]
    pub option_property_count: u8,
    #[serde(rename = "OptionProperties")]
    pub option_properties: Vec<OptionProperty>,
}

// The PlayerModule structure contains character options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerModule {
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "Options")]
    pub options: CharacterOptions1,
    #[serde(rename = "Shortcuts", skip_serializing_if = "Option::is_none")]
    pub shortcuts: Option<PackableList<ShortCutData>>,
    #[serde(rename = "Tab1Spells")]
    pub tab1_spells: PackableList<LayeredSpellId>,
    #[serde(rename = "Tab2Spells")]
    pub tab2_spells: PackableList<LayeredSpellId>,
    #[serde(rename = "Tab3Spells")]
    pub tab3_spells: PackableList<LayeredSpellId>,
    #[serde(rename = "Tab4Spells")]
    pub tab4_spells: PackableList<LayeredSpellId>,
    #[serde(rename = "Tab5Spells")]
    pub tab5_spells: PackableList<LayeredSpellId>,
    #[serde(rename = "Tab6Spells")]
    pub tab6_spells: PackableList<LayeredSpellId>,
    #[serde(rename = "Tab7Spells")]
    pub tab7_spells: PackableList<LayeredSpellId>,
    #[serde(rename = "Tab8Spells")]
    pub tab8_spells: PackableList<LayeredSpellId>,
    #[serde(rename = "FillComps", skip_serializing_if = "Option::is_none")]
    pub fill_comps: Option<PackableHashTable<u32, u32>>,
    #[serde(rename = "SpellBookFilters", skip_serializing_if = "Option::is_none")]
    pub spell_book_filters: Option<u32>,
    #[serde(rename = "OptionFlags", skip_serializing_if = "Option::is_none")]
    pub option_flags: Option<u32>,
    #[serde(rename = "Unknown100_1", skip_serializing_if = "Option::is_none")]
    pub unknown100_1: Option<u32>,
    #[serde(rename = "OptionStrings", skip_serializing_if = "Option::is_none")]
    pub option_strings: Option<PackableHashTable<u32, String>>,
    #[serde(rename = "GameplayOptions", skip_serializing_if = "Option::is_none")]
    pub gameplay_options: Option<GameplayOptions>,
}

// Set of shortcuts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortCutManager {
    #[serde(rename = "Shortcuts")]
    pub shortcuts: PackableList<ShortCutData>,
}

// Shortcut
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortCutData {
    #[serde(rename = "Index")]
    pub index: u32,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

// List of spells in spell tab
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpellTab {
    #[serde(rename = "Spells")]
    pub spells: PackableList<LayeredSpellId>,
}

// Set of inventory items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentProfile {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ContainerType")]
    pub container_type: ContainerProperties,
}

// Set of inventory items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryPlacement {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Location")]
    pub location: EquipMask,
    #[serde(rename = "Priority")]
    pub priority: CoverageMask,
}

// Allegience information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllegianceProfile {
    #[serde(rename = "TotalMembers")]
    pub total_members: u32,
    #[serde(rename = "TotalVassals")]
    pub total_vassals: u32,
    #[serde(rename = "Hierarchy")]
    pub hierarchy: AllegianceHierarchy,
}

// Allegience record
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllegianceRecord {
    #[serde(rename = "TreeParent")]
    pub tree_parent: ObjectId,
    #[serde(rename = "AllegianceData")]
    pub allegiance_data: AllegianceData,
}

// Allegience hierarchy information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllegianceHierarchy {
    #[serde(rename = "RecordCount")]
    pub record_count: u16,
    #[serde(rename = "OldVersion")]
    pub old_version: u16,
    #[serde(rename = "Officers")]
    pub officers: PHashTable<ObjectId, AllegianceOfficerLevel>,
    #[serde(rename = "OfficerTitles")]
    pub officer_titles: PackableList<String>,
    #[serde(rename = "MonarchBroadcastTime")]
    pub monarch_broadcast_time: u32,
    #[serde(rename = "MonarchBroadcastsToday")]
    pub monarch_broadcasts_today: u32,
    #[serde(rename = "SpokesBroadcastTime")]
    pub spokes_broadcast_time: u32,
    #[serde(rename = "SpokesBroadcastsToday")]
    pub spokes_broadcasts_today: u32,
    #[serde(rename = "Motd")]
    pub motd: String,
    #[serde(rename = "MotdSetBy")]
    pub motd_set_by: String,
    #[serde(rename = "ChatRoomId")]
    pub chat_room_id: u32,
    #[serde(rename = "Bindpoint")]
    pub bindpoint: Position,
    #[serde(rename = "AllegianceName")]
    pub allegiance_name: String,
    #[serde(rename = "NameLastSetTime")]
    pub name_last_set_time: u32,
    #[serde(rename = "IsLocked")]
    pub is_locked: bool,
    #[serde(rename = "ApprovedVassal")]
    pub approved_vassal: i32,
    #[serde(rename = "MonarchData", skip_serializing_if = "Option::is_none")]
    pub monarch_data: Option<AllegianceData>,
    #[serde(rename = "Records")]
    pub records: Vec<AllegianceRecord>,
}

// Set of allegiance data for a specific player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllegianceData {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
    #[serde(rename = "XPCached")]
    pub xp_cached: u32,
    #[serde(rename = "XPTithed")]
    pub xp_tithed: u32,
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "Gender")]
    pub gender: Gender,
    #[serde(rename = "Heritage")]
    pub heritage: HeritageGroup,
    #[serde(rename = "Rank")]
    pub rank: u16,
    #[serde(rename = "Level", skip_serializing_if = "Option::is_none")]
    pub level: Option<u32>,
    #[serde(rename = "Loyalty")]
    pub loyalty: u16,
    #[serde(rename = "Leadership")]
    pub leadership: u16,
    #[serde(rename = "AllegianceAge", skip_serializing_if = "Option::is_none")]
    pub allegiance_age: Option<u32>,
    #[serde(rename = "TimeOnline", skip_serializing_if = "Option::is_none")]
    pub time_online: Option<u64>,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendData {
    #[serde(rename = "FriendId")]
    pub friend_id: ObjectId,
    #[serde(rename = "Online")]
    pub online: bool,
    #[serde(rename = "AppearOffline")]
    pub appear_offline: bool,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OutFriends")]
    pub out_friends: PackableList<ObjectId>,
    #[serde(rename = "InFriends")]
    pub in_friends: PackableList<ObjectId>,
}

// Data related to an item, namely the amount and description
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemProfileTypeNeg1 {
    #[serde(rename = "PackedAmount")]
    pub packed_amount: u32,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "WeenieDescription")]
    pub weenie_description: PublicWeenieDesc,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemProfileType1 {
    #[serde(rename = "PackedAmount")]
    pub packed_amount: u32,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "OldWeenieDescription")]
    pub old_weenie_description: OldPublicWeenieDesc,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "PwdType")]
pub enum ItemProfile {
    TypeNeg1(ItemProfileTypeNeg1),
    Type1(ItemProfileType1),
}

// The PublicWeenieDesc structure defines an object's game behavior.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicWeenieDesc {
    #[serde(rename = "Header")]
    pub header: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "WeenieClassId")]
    pub weenie_class_id: PackedDWORD,
    #[serde(rename = "Icon")]
    pub icon: PackedDWORD,
    #[serde(rename = "Type")]
    pub type_: ItemType,
    #[serde(rename = "Behavior")]
    pub behavior: ObjectDescriptionFlag,
    #[serde(rename = "Header2", skip_serializing_if = "Option::is_none")]
    pub header2: Option<u32>,
    #[serde(rename = "PluralName", skip_serializing_if = "Option::is_none")]
    pub plural_name: Option<String>,
    #[serde(rename = "ItemsCapacity", skip_serializing_if = "Option::is_none")]
    pub items_capacity: Option<u8>,
    #[serde(rename = "ContainerCapacity", skip_serializing_if = "Option::is_none")]
    pub container_capacity: Option<u8>,
    #[serde(rename = "AmmunitionType", skip_serializing_if = "Option::is_none")]
    pub ammunition_type: Option<AmmoType>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<u32>,
    #[serde(rename = "Useability", skip_serializing_if = "Option::is_none")]
    pub useability: Option<Usable>,
    #[serde(rename = "UseRadius", skip_serializing_if = "Option::is_none")]
    pub use_radius: Option<f32>,
    #[serde(rename = "TargetType", skip_serializing_if = "Option::is_none")]
    pub target_type: Option<ItemType>,
    #[serde(rename = "Effects", skip_serializing_if = "Option::is_none")]
    pub effects: Option<IconHighlight>,
    #[serde(rename = "CombatUse", skip_serializing_if = "Option::is_none")]
    pub combat_use: Option<WieldType>,
    #[serde(rename = "Structure", skip_serializing_if = "Option::is_none")]
    pub structure: Option<u16>,
    #[serde(rename = "MaxStructure", skip_serializing_if = "Option::is_none")]
    pub max_structure: Option<u16>,
    #[serde(rename = "StackSize", skip_serializing_if = "Option::is_none")]
    pub stack_size: Option<u16>,
    #[serde(rename = "MaxStackSize", skip_serializing_if = "Option::is_none")]
    pub max_stack_size: Option<u16>,
    #[serde(rename = "ContainerId", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<ObjectId>,
    #[serde(rename = "WielderId", skip_serializing_if = "Option::is_none")]
    pub wielder_id: Option<ObjectId>,
    #[serde(rename = "ValidSlots", skip_serializing_if = "Option::is_none")]
    pub valid_slots: Option<EquipMask>,
    #[serde(rename = "Slot", skip_serializing_if = "Option::is_none")]
    pub slot: Option<EquipMask>,
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<CoverageMask>,
    #[serde(rename = "BlipColor", skip_serializing_if = "Option::is_none")]
    pub blip_color: Option<RadarColor>,
    #[serde(rename = "RadarEnum", skip_serializing_if = "Option::is_none")]
    pub radar_enum: Option<RadarBehavior>,
    #[serde(rename = "PhysicsScript", skip_serializing_if = "Option::is_none")]
    pub physics_script: Option<u16>,
    #[serde(rename = "Workmanship", skip_serializing_if = "Option::is_none")]
    pub workmanship: Option<f32>,
    #[serde(rename = "Burden", skip_serializing_if = "Option::is_none")]
    pub burden: Option<u16>,
    #[serde(rename = "SpellId", skip_serializing_if = "Option::is_none")]
    pub spell_id: Option<SpellId>,
    #[serde(rename = "OwnerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<ObjectId>,
    #[serde(rename = "Restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<RestrictionDB>,
    #[serde(rename = "HookItemTypes", skip_serializing_if = "Option::is_none")]
    pub hook_item_types: Option<HookType>,
    #[serde(rename = "MonarchId", skip_serializing_if = "Option::is_none")]
    pub monarch_id: Option<ObjectId>,
    #[serde(rename = "HookType", skip_serializing_if = "Option::is_none")]
    pub hook_type: Option<HookType>,
    #[serde(rename = "IconOverlay", skip_serializing_if = "Option::is_none")]
    pub icon_overlay: Option<PackedDWORD>,
    #[serde(rename = "IconUnderlay", skip_serializing_if = "Option::is_none")]
    pub icon_underlay: Option<PackedDWORD>,
    #[serde(rename = "Material", skip_serializing_if = "Option::is_none")]
    pub material: Option<MaterialType>,
    #[serde(rename = "CooldownId", skip_serializing_if = "Option::is_none")]
    pub cooldown_id: Option<u32>,
    #[serde(rename = "CooldownDuration", skip_serializing_if = "Option::is_none")]
    pub cooldown_duration: Option<u64>,
    #[serde(rename = "PetOwnerId", skip_serializing_if = "Option::is_none")]
    pub pet_owner_id: Option<ObjectId>,
}

// The RestrictionDB contains the access control list for a dwelling object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestrictionDB {
    #[serde(rename = "Version")]
    pub version: u32,
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "MonarchId")]
    pub monarch_id: ObjectId,
    #[serde(rename = "Permissions")]
    pub permissions: PHashTable<ObjectId, u32>,
}

// The OldPublicWeenieDesc structure defines an object's game behavior.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OldPublicWeenieDesc {
    #[serde(rename = "Header")]
    pub header: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "WeenieClassId")]
    pub weenie_class_id: PackedDWORD,
    #[serde(rename = "Icon")]
    pub icon: PackedDWORD,
    #[serde(rename = "Type")]
    pub type_: ItemType,
    #[serde(rename = "Bitfield")]
    pub bitfield: ObjectDescriptionFlag,
    #[serde(rename = "PluralName", skip_serializing_if = "Option::is_none")]
    pub plural_name: Option<String>,
    #[serde(rename = "ItemsCapacity", skip_serializing_if = "Option::is_none")]
    pub items_capacity: Option<u8>,
    #[serde(rename = "ContainerCapacity", skip_serializing_if = "Option::is_none")]
    pub container_capacity: Option<u8>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<u32>,
    #[serde(rename = "Useability", skip_serializing_if = "Option::is_none")]
    pub useability: Option<Usable>,
    #[serde(rename = "UseRadius", skip_serializing_if = "Option::is_none")]
    pub use_radius: Option<f32>,
    #[serde(rename = "tTargetType", skip_serializing_if = "Option::is_none")]
    pub t_target_type: Option<ItemType>,
    #[serde(rename = "Effects", skip_serializing_if = "Option::is_none")]
    pub effects: Option<IconHighlight>,
    #[serde(rename = "AmmunitionType", skip_serializing_if = "Option::is_none")]
    pub ammunition_type: Option<AmmoType>,
    #[serde(rename = "CombatUse", skip_serializing_if = "Option::is_none")]
    pub combat_use: Option<WieldType>,
    #[serde(rename = "Structure", skip_serializing_if = "Option::is_none")]
    pub structure: Option<u16>,
    #[serde(rename = "MaxStructure", skip_serializing_if = "Option::is_none")]
    pub max_structure: Option<u16>,
    #[serde(rename = "StackSize", skip_serializing_if = "Option::is_none")]
    pub stack_size: Option<u16>,
    #[serde(rename = "MaxStackSize", skip_serializing_if = "Option::is_none")]
    pub max_stack_size: Option<u16>,
    #[serde(rename = "ContainerId", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<ObjectId>,
    #[serde(rename = "WielderId", skip_serializing_if = "Option::is_none")]
    pub wielder_id: Option<ObjectId>,
    #[serde(rename = "ValidSlots", skip_serializing_if = "Option::is_none")]
    pub valid_slots: Option<EquipMask>,
    #[serde(rename = "Slots", skip_serializing_if = "Option::is_none")]
    pub slots: Option<EquipMask>,
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<CoverageMask>,
    #[serde(rename = "BlipColor", skip_serializing_if = "Option::is_none")]
    pub blip_color: Option<RadarColor>,
    #[serde(rename = "RadarEnum", skip_serializing_if = "Option::is_none")]
    pub radar_enum: Option<RadarBehavior>,
    #[serde(rename = "ObviousDistance", skip_serializing_if = "Option::is_none")]
    pub obvious_distance: Option<f32>,
    #[serde(rename = "Vndwcid", skip_serializing_if = "Option::is_none")]
    pub vndwcid: Option<u16>,
    #[serde(rename = "SpellId", skip_serializing_if = "Option::is_none")]
    pub spell_id: Option<SpellId>,
    #[serde(rename = "HouseOwnerId", skip_serializing_if = "Option::is_none")]
    pub house_owner_id: Option<ObjectId>,
    #[serde(rename = "PhysicsScript", skip_serializing_if = "Option::is_none")]
    pub physics_script: Option<u16>,
    #[serde(rename = "Restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<RestrictionDB>,
    #[serde(rename = "HookType", skip_serializing_if = "Option::is_none")]
    pub hook_type: Option<HookType>,
    #[serde(rename = "HookItemTypes", skip_serializing_if = "Option::is_none")]
    pub hook_item_types: Option<HookType>,
    #[serde(rename = "MonarchId", skip_serializing_if = "Option::is_none")]
    pub monarch_id: Option<ObjectId>,
    #[serde(rename = "IconOverlay", skip_serializing_if = "Option::is_none")]
    pub icon_overlay: Option<PackedDWORD>,
    #[serde(rename = "Material", skip_serializing_if = "Option::is_none")]
    pub material: Option<MaterialType>,
}

// Information related to a secure trade.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "PartnerId")]
    pub partner_id: ObjectId,
    #[serde(rename = "Sequence")]
    pub sequence: u64,
    #[serde(rename = "Status")]
    pub status: u32,
    #[serde(rename = "InitiatorId")]
    pub initiator_id: ObjectId,
    #[serde(rename = "Accepted")]
    pub accepted: bool,
    #[serde(rename = "PartnerAccepted")]
    pub partner_accepted: bool,
}

// A jump with sequences
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JumpPack {
    #[serde(rename = "Extent")]
    pub extent: f32,
    #[serde(rename = "Velocity")]
    pub velocity: Vector3,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    pub object_server_control_sequence: u16,
    #[serde(rename = "ObjectTeleportSequence")]
    pub object_teleport_sequence: u16,
    #[serde(rename = "ObjectForcePositionSequence")]
    pub object_force_position_sequence: u16,
}

// A set of data related to changing states with sequences
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveToStatePack {
    #[serde(rename = "RawMotionState")]
    pub raw_motion_state: RawMotionState,
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    pub object_server_control_sequence: u16,
    #[serde(rename = "ObjectTeleportSequence")]
    pub object_teleport_sequence: u16,
    #[serde(rename = "ObjectForcePositionSequence")]
    pub object_force_position_sequence: u16,
    #[serde(rename = "Contact")]
    pub contact: u8,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackedMotionCommand {
    #[serde(rename = "CommandId")]
    pub command_id: Command,
    #[serde(rename = "PackedSequence")]
    pub packed_sequence: u16,
    #[serde(rename = "Speed")]
    pub speed: f32,
}

// Data related to the movement of the object sent from a client
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawMotionState {
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "CurrentHoldkey", skip_serializing_if = "Option::is_none")]
    pub current_holdkey: Option<HoldKey>,
    #[serde(rename = "CurrentStyle", skip_serializing_if = "Option::is_none")]
    pub current_style: Option<StanceMode>,
    #[serde(rename = "ForwardCommand", skip_serializing_if = "Option::is_none")]
    pub forward_command: Option<Command>,
    #[serde(rename = "ForwardHoldkey", skip_serializing_if = "Option::is_none")]
    pub forward_holdkey: Option<HoldKey>,
    #[serde(rename = "ForwardSpeed", skip_serializing_if = "Option::is_none")]
    pub forward_speed: Option<f32>,
    #[serde(rename = "SidestepCommand", skip_serializing_if = "Option::is_none")]
    pub sidestep_command: Option<Command>,
    #[serde(rename = "SidestepHoldkey", skip_serializing_if = "Option::is_none")]
    pub sidestep_holdkey: Option<HoldKey>,
    #[serde(rename = "SidestepSpeed", skip_serializing_if = "Option::is_none")]
    pub sidestep_speed: Option<f32>,
    #[serde(rename = "TurnCommand", skip_serializing_if = "Option::is_none")]
    pub turn_command: Option<Command>,
    #[serde(rename = "TurnHoldkey", skip_serializing_if = "Option::is_none")]
    pub turn_holdkey: Option<u32>,
    #[serde(rename = "TurnSpeed", skip_serializing_if = "Option::is_none")]
    pub turn_speed: Option<f32>,
    #[serde(rename = "Commands")]
    pub commands: Vec<PackedMotionCommand>,
}

// An autonomous position with sequences
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousPositionPack {
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    pub object_server_control_sequence: u16,
    #[serde(rename = "ObjectTeleportSequence")]
    pub object_teleport_sequence: u16,
    #[serde(rename = "ObjectForcePositionSequence")]
    pub object_force_position_sequence: u16,
    #[serde(rename = "Contact")]
    pub contact: u8,
}

// A position with sequences
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PositionPack {
    #[serde(rename = "Flags")]
    pub flags: PositionFlags,
    #[serde(rename = "Origin")]
    pub origin: Origin,
    #[serde(rename = "WQuat", skip_serializing_if = "Option::is_none")]
    pub w_quat: Option<f32>,
    #[serde(rename = "XQuat", skip_serializing_if = "Option::is_none")]
    pub x_quat: Option<f32>,
    #[serde(rename = "YQuat", skip_serializing_if = "Option::is_none")]
    pub y_quat: Option<f32>,
    #[serde(rename = "ZQuat", skip_serializing_if = "Option::is_none")]
    pub z_quat: Option<f32>,
    #[serde(rename = "Velocity", skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Vector3>,
    #[serde(rename = "PlacementId", skip_serializing_if = "Option::is_none")]
    pub placement_id: Option<u32>,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectPositionSequence")]
    pub object_position_sequence: u16,
    #[serde(rename = "ObjectTeleportSequence")]
    pub object_teleport_sequence: u16,
    #[serde(rename = "ObjectForcePositionSequence")]
    pub object_force_position_sequence: u16,
}

// Data related to the movement and animation of the object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovementDataType0 {
    #[serde(rename = "ObjectMovementSequence")]
    pub object_movement_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    pub object_server_control_sequence: u16,
    #[serde(rename = "Autonomous")]
    pub autonomous: u16,
    #[serde(rename = "OptionFlags")]
    pub option_flags: MovementOption,
    #[serde(rename = "Stance")]
    pub stance: StanceMode,
    #[serde(rename = "State")]
    pub state: InterpretedMotionState,
    #[serde(rename = "StickyObject", skip_serializing_if = "Option::is_none")]
    pub sticky_object: Option<ObjectId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovementDataType6 {
    #[serde(rename = "ObjectMovementSequence")]
    pub object_movement_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    pub object_server_control_sequence: u16,
    #[serde(rename = "Autonomous")]
    pub autonomous: u16,
    #[serde(rename = "OptionFlags")]
    pub option_flags: MovementOption,
    #[serde(rename = "Stance")]
    pub stance: StanceMode,
    #[serde(rename = "Target")]
    pub target: ObjectId,
    #[serde(rename = "Origin")]
    pub origin: Origin,
    #[serde(rename = "MoveToParams")]
    pub move_to_params: MoveToMovementParameters,
    #[serde(rename = "MyRunRate")]
    pub my_run_rate: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovementDataType7 {
    #[serde(rename = "ObjectMovementSequence")]
    pub object_movement_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    pub object_server_control_sequence: u16,
    #[serde(rename = "Autonomous")]
    pub autonomous: u16,
    #[serde(rename = "OptionFlags")]
    pub option_flags: MovementOption,
    #[serde(rename = "Stance")]
    pub stance: StanceMode,
    #[serde(rename = "Origin")]
    pub origin: Origin,
    #[serde(rename = "MoveToParams")]
    pub move_to_params: MoveToMovementParameters,
    #[serde(rename = "MyRunRate")]
    pub my_run_rate: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovementDataType8 {
    #[serde(rename = "ObjectMovementSequence")]
    pub object_movement_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    pub object_server_control_sequence: u16,
    #[serde(rename = "Autonomous")]
    pub autonomous: u16,
    #[serde(rename = "OptionFlags")]
    pub option_flags: MovementOption,
    #[serde(rename = "Stance")]
    pub stance: StanceMode,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "DesiredHeading")]
    pub desired_heading: f32,
    #[serde(rename = "TurnToParams")]
    pub turn_to_params: TurnToMovementParameters,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovementDataType9 {
    #[serde(rename = "ObjectMovementSequence")]
    pub object_movement_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    pub object_server_control_sequence: u16,
    #[serde(rename = "Autonomous")]
    pub autonomous: u16,
    #[serde(rename = "OptionFlags")]
    pub option_flags: MovementOption,
    #[serde(rename = "Stance")]
    pub stance: StanceMode,
    #[serde(rename = "TurnToParams")]
    pub turn_to_params: TurnToMovementParameters,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "MovementType")]
pub enum MovementData {
    Type0(MovementDataType0),
    Type6(MovementDataType6),
    Type7(MovementDataType7),
    Type8(MovementDataType8),
    Type9(MovementDataType9),
}

// Contains information for animations and general free motion
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterpretedMotionState {
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "CurrentStyle", skip_serializing_if = "Option::is_none")]
    pub current_style: Option<StanceMode>,
    #[serde(rename = "ForwardCommand", skip_serializing_if = "Option::is_none")]
    pub forward_command: Option<Command>,
    #[serde(rename = "SidestepCommand", skip_serializing_if = "Option::is_none")]
    pub sidestep_command: Option<Command>,
    #[serde(rename = "TurnCommand", skip_serializing_if = "Option::is_none")]
    pub turn_command: Option<Command>,
    #[serde(rename = "ForwardSpeed", skip_serializing_if = "Option::is_none")]
    pub forward_speed: Option<f32>,
    #[serde(rename = "SidestepSpeed", skip_serializing_if = "Option::is_none")]
    pub sidestep_speed: Option<f32>,
    #[serde(rename = "TurnSpeed", skip_serializing_if = "Option::is_none")]
    pub turn_speed: Option<f32>,
    #[serde(rename = "Commands")]
    pub commands: Vec<PackedMotionCommand>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DDDRevision {
    #[serde(rename = "IdDatFile")]
    pub id_dat_file: u64,
    #[serde(rename = "Iteration")]
    pub iteration: u32,
    #[serde(rename = "IdsToDownload")]
    pub ids_to_download: PackableList<DataId>,
    #[serde(rename = "IdsToPurge")]
    pub ids_to_purge: PackableList<DataId>,
}

// Set of movement parameters required for a MoveTo movement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveToMovementParameters {
    #[serde(rename = "Bitmember")]
    pub bitmember: u32,
    #[serde(rename = "DistanceToObject")]
    pub distance_to_object: f32,
    #[serde(rename = "MinDistance")]
    pub min_distance: f32,
    #[serde(rename = "FailDistance")]
    pub fail_distance: f32,
    #[serde(rename = "AnimationSpeed")]
    pub animation_speed: f32,
    #[serde(rename = "WalkRunThreshold")]
    pub walk_run_threshold: f32,
    #[serde(rename = "DesiredHeading")]
    pub desired_heading: f32,
}

// Set of movement parameters required for a TurnTo motion
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TurnToMovementParameters {
    #[serde(rename = "Bitmember")]
    pub bitmember: u32,
    #[serde(rename = "AnimationSpeed")]
    pub animation_speed: f32,
    #[serde(rename = "DesiredHeading")]
    pub desired_heading: f32,
}

// The ObjDesc structure defines an object's visual appearance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjDesc {
    #[serde(rename = "Version")]
    pub version: u8,
    #[serde(rename = "PaletteCount")]
    pub palette_count: u8,
    #[serde(rename = "TextureCount")]
    pub texture_count: u8,
    #[serde(rename = "ModelCount")]
    pub model_count: u8,
    #[serde(rename = "Palette", skip_serializing_if = "Option::is_none")]
    pub palette: Option<DataId>,
    #[serde(rename = "Subpalettes")]
    pub subpalettes: Vec<Subpalette>,
    #[serde(rename = "TMChanges")]
    pub tm_changes: Vec<TextureMapChange>,
    #[serde(rename = "APChanges")]
    pub ap_changes: Vec<AnimPartChange>,
}

// Contains data for a subpalette
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subpalette {
    #[serde(rename = "Palette")]
    pub palette: DataId,
    #[serde(rename = "Offset")]
    pub offset: u8,
    #[serde(rename = "NumColors")]
    pub num_colors: u8,
}

// Contains data for texture map changes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextureMapChange {
    #[serde(rename = "PartIndex")]
    pub part_index: u8,
    #[serde(rename = "OldTexId")]
    pub old_tex_id: DataId,
    #[serde(rename = "NewTexId")]
    pub new_tex_id: DataId,
}

// Contains data for animation part changes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnimPartChange {
    #[serde(rename = "PartIndex")]
    pub part_index: u8,
    #[serde(rename = "PartId")]
    pub part_id: DataId,
}

// Data for a character creation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharGenResult {
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "One")]
    pub one: u32,
    #[serde(rename = "HeritageGroup")]
    pub heritage_group: HeritageGroup,
    #[serde(rename = "Gender")]
    pub gender: Gender,
    #[serde(rename = "EyesStrip")]
    pub eyes_strip: u32,
    #[serde(rename = "NoseStrip")]
    pub nose_strip: u32,
    #[serde(rename = "MouthStrip")]
    pub mouth_strip: u32,
    #[serde(rename = "HairColor")]
    pub hair_color: u32,
    #[serde(rename = "EyeColor")]
    pub eye_color: u32,
    #[serde(rename = "HairStyle")]
    pub hair_style: u32,
    #[serde(rename = "HeadgearStyle")]
    pub headgear_style: u32,
    #[serde(rename = "HeadgearColor")]
    pub headgear_color: u32,
    #[serde(rename = "ShirtStyle")]
    pub shirt_style: u32,
    #[serde(rename = "ShirtColor")]
    pub shirt_color: u32,
    #[serde(rename = "TrousersStyle")]
    pub trousers_style: u32,
    #[serde(rename = "TrousersColor")]
    pub trousers_color: u32,
    #[serde(rename = "FootwearStyle")]
    pub footwear_style: u32,
    #[serde(rename = "FootwearColor")]
    pub footwear_color: u32,
    #[serde(rename = "SkinShade")]
    pub skin_shade: u64,
    #[serde(rename = "HairShade")]
    pub hair_shade: u64,
    #[serde(rename = "HeadgearShade")]
    pub headgear_shade: u64,
    #[serde(rename = "ShirtShade")]
    pub shirt_shade: u64,
    #[serde(rename = "TrousersShade")]
    pub trousers_shade: u64,
    #[serde(rename = "TootwearShade")]
    pub tootwear_shade: u64,
    #[serde(rename = "TemplateNum")]
    pub template_num: u32,
    #[serde(rename = "Strength")]
    pub strength: u32,
    #[serde(rename = "Endurance")]
    pub endurance: u32,
    #[serde(rename = "Coordination")]
    pub coordination: u32,
    #[serde(rename = "Quickness")]
    pub quickness: u32,
    #[serde(rename = "Focus")]
    pub focus: u32,
    #[serde(rename = "Self")]
    pub self_: u32,
    #[serde(rename = "Slot")]
    pub slot: u32,
    #[serde(rename = "ClassId")]
    pub class_id: u32,
    #[serde(rename = "Skills")]
    pub skills: PackableList<SkillAdvancementClass>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "StartArea")]
    pub start_area: u32,
    #[serde(rename = "IsAdmin")]
    pub is_admin: u32,
    #[serde(rename = "IsEnvoy")]
    pub is_envoy: u32,
    #[serde(rename = "Validation")]
    pub validation: u32,
}

// Basic information for a character used at the Login screen
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterIdentity {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "SecondsGreyedOut")]
    pub seconds_greyed_out: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquipLocation {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Slot")]
    pub slot: EquipMask,
}

// The PhysicsDesc structure defines an object's physical behavior.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicsDesc {
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "State")]
    pub state: PhysicsState,
    #[serde(rename = "MovementBuffer", skip_serializing_if = "Option::is_none")]
    pub movement_buffer: Option<PackableList<u8>>,
    #[serde(rename = "Autonomous", skip_serializing_if = "Option::is_none")]
    pub autonomous: Option<bool>,
    #[serde(rename = "AnimationFrame", skip_serializing_if = "Option::is_none")]
    pub animation_frame: Option<u32>,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(rename = "MotionId", skip_serializing_if = "Option::is_none")]
    pub motion_id: Option<DataId>,
    #[serde(rename = "SoundId", skip_serializing_if = "Option::is_none")]
    pub sound_id: Option<DataId>,
    #[serde(rename = "PhysicsScriptId", skip_serializing_if = "Option::is_none")]
    pub physics_script_id: Option<DataId>,
    #[serde(rename = "SetupId", skip_serializing_if = "Option::is_none")]
    pub setup_id: Option<DataId>,
    #[serde(rename = "ParentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ObjectId>,
    #[serde(rename = "ParentLocation", skip_serializing_if = "Option::is_none")]
    pub parent_location: Option<ParentLocation>,
    #[serde(rename = "Children", skip_serializing_if = "Option::is_none")]
    pub children: Option<PackableList<EquipLocation>>,
    #[serde(rename = "Scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<f32>,
    #[serde(rename = "Friction", skip_serializing_if = "Option::is_none")]
    pub friction: Option<f32>,
    #[serde(rename = "Elasticity", skip_serializing_if = "Option::is_none")]
    pub elasticity: Option<f32>,
    #[serde(rename = "Translucency", skip_serializing_if = "Option::is_none")]
    pub translucency: Option<f32>,
    #[serde(rename = "Velocity", skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Vector3>,
    #[serde(rename = "Acceleration", skip_serializing_if = "Option::is_none")]
    pub acceleration: Option<Vector3>,
    #[serde(rename = "Omega", skip_serializing_if = "Option::is_none")]
    pub omega: Option<Vector3>,
    #[serde(rename = "DefaultScript", skip_serializing_if = "Option::is_none")]
    pub default_script: Option<u32>,
    #[serde(rename = "DefaultScriptIntensity", skip_serializing_if = "Option::is_none")]
    pub default_script_intensity: Option<f32>,
    #[serde(rename = "ObjectPositionSequence")]
    pub object_position_sequence: u16,
    #[serde(rename = "ObjectMovementSequence")]
    pub object_movement_sequence: u16,
    #[serde(rename = "ObjectStateSequence")]
    pub object_state_sequence: u16,
    #[serde(rename = "ObjectVectorSequence")]
    pub object_vector_sequence: u16,
    #[serde(rename = "ObjectTeleportSequence")]
    pub object_teleport_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    pub object_server_control_sequence: u16,
    #[serde(rename = "ObjectForcePositionSequence")]
    pub object_force_position_sequence: u16,
    #[serde(rename = "ObjectVisualDescSequence")]
    pub object_visual_desc_sequence: u16,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminAccountData {
    #[serde(rename = "AccountName")]
    pub account_name: String,
    #[serde(rename = "BookieId")]
    pub bookie_id: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminPlayerData {
    pub name: String,
    #[serde(rename = "bookieId")]
    pub bookie_id: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VendorProfile {
    #[serde(rename = "Categories")]
    pub categories: ItemType,
    #[serde(rename = "MinValue")]
    pub min_value: u32,
    #[serde(rename = "MaxValue")]
    pub max_value: u32,
    #[serde(rename = "DealsMagic")]
    pub deals_magic: bool,
    #[serde(rename = "BuyPrice")]
    pub buy_price: f32,
    #[serde(rename = "SellPrice")]
    pub sell_price: f32,
    #[serde(rename = "CurrencyId")]
    pub currency_id: u32,
    #[serde(rename = "CurrencyAmount")]
    pub currency_amount: u32,
    #[serde(rename = "CurrencyName")]
    pub currency_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmorProfile {
    #[serde(rename = "ProtSlashing")]
    pub prot_slashing: f32,
    #[serde(rename = "ProtPiercing")]
    pub prot_piercing: f32,
    #[serde(rename = "ProtBludgeoning")]
    pub prot_bludgeoning: f32,
    #[serde(rename = "ProtCold")]
    pub prot_cold: f32,
    #[serde(rename = "ProtFire")]
    pub prot_fire: f32,
    #[serde(rename = "ProtAcid")]
    pub prot_acid: f32,
    #[serde(rename = "ProtNether")]
    pub prot_nether: f32,
    #[serde(rename = "ProtLightning")]
    pub prot_lightning: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatureAppraisalProfile {
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "Health")]
    pub health: u32,
    #[serde(rename = "HealthMax")]
    pub health_max: u32,
    #[serde(rename = "Strength", skip_serializing_if = "Option::is_none")]
    pub strength: Option<u32>,
    #[serde(rename = "Endurance", skip_serializing_if = "Option::is_none")]
    pub endurance: Option<u32>,
    #[serde(rename = "Quickness", skip_serializing_if = "Option::is_none")]
    pub quickness: Option<u32>,
    #[serde(rename = "Coordination", skip_serializing_if = "Option::is_none")]
    pub coordination: Option<u32>,
    #[serde(rename = "Focus", skip_serializing_if = "Option::is_none")]
    pub focus: Option<u32>,
    #[serde(rename = "Self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<u32>,
    #[serde(rename = "Stamina", skip_serializing_if = "Option::is_none")]
    pub stamina: Option<u32>,
    #[serde(rename = "Mana", skip_serializing_if = "Option::is_none")]
    pub mana: Option<u32>,
    #[serde(rename = "StaminaMax", skip_serializing_if = "Option::is_none")]
    pub stamina_max: Option<u32>,
    #[serde(rename = "ManaMax", skip_serializing_if = "Option::is_none")]
    pub mana_max: Option<u32>,
    #[serde(rename = "AttrHighlight", skip_serializing_if = "Option::is_none")]
    pub attr_highlight: Option<AttributeMask>,
    #[serde(rename = "AttrColor", skip_serializing_if = "Option::is_none")]
    pub attr_color: Option<AttributeMask>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeaponProfile {
    #[serde(rename = "DamageType")]
    pub damage_type: DamageType,
    #[serde(rename = "Speed")]
    pub speed: u32,
    #[serde(rename = "Skill")]
    pub skill: SkillId,
    #[serde(rename = "Damage")]
    pub damage: u32,
    #[serde(rename = "Variance")]
    pub variance: f64,
    #[serde(rename = "Modifier")]
    pub modifier: f64,
    #[serde(rename = "Length")]
    pub length: f64,
    #[serde(rename = "MaxVelocity")]
    pub max_velocity: f64,
    #[serde(rename = "Offsense")]
    pub offsense: f64,
    #[serde(rename = "MaxVelocityEstimated")]
    pub max_velocity_estimated: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HookAppraisalProfile {
    #[serde(rename = "Flags")]
    pub flags: HookAppraisalFlags,
    #[serde(rename = "ValidLocations")]
    pub valid_locations: EquipMask,
    #[serde(rename = "AmmoType")]
    pub ammo_type: AmmoType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SquelchDB {
    #[serde(rename = "AccountHash")]
    pub account_hash: PackableHashTable<String, u32>,
    #[serde(rename = "CharacterHash")]
    pub character_hash: PackableHashTable<ObjectId, SquelchInfo>,
    #[serde(rename = "GlobalInfo")]
    pub global_info: SquelchInfo,
}

// Set of information related to a squelch entry
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SquelchInfo {
    #[serde(rename = "Filters")]
    pub filters: PackableList<LogTextType>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Account")]
    pub account: bool,
}

// Set of information related to purchasing a housing
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HouseProfile {
    #[serde(rename = "DwellingId")]
    pub dwelling_id: u32,
    #[serde(rename = "OwnerId")]
    pub owner_id: ObjectId,
    #[serde(rename = "Flags")]
    pub flags: HouseBitfield,
    #[serde(rename = "MinLevel")]
    pub min_level: i32,
    #[serde(rename = "MaxLevel")]
    pub max_level: i32,
    #[serde(rename = "MinAllegRank")]
    pub min_alleg_rank: i32,
    #[serde(rename = "MaxAllegRank")]
    pub max_alleg_rank: i32,
    #[serde(rename = "MaintenanceFree")]
    pub maintenance_free: bool,
    #[serde(rename = "Type")]
    pub type_: HouseType,
    #[serde(rename = "OwnerName")]
    pub owner_name: String,
    #[serde(rename = "Buy")]
    pub buy: PackableList<HousePayment>,
    #[serde(rename = "Rent")]
    pub rent: PackableList<HousePayment>,
}

// The HousePayment structure contains information about a house purchase or maintenance item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HousePayment {
    #[serde(rename = "Required")]
    pub required: u32,
    #[serde(rename = "Paid")]
    pub paid: u32,
    #[serde(rename = "WeenieClassId")]
    pub weenie_class_id: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "PluralName")]
    pub plural_name: String,
}

// Set of information related to owning a housing
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HouseData {
    #[serde(rename = "BuyTime")]
    pub buy_time: u32,
    #[serde(rename = "RentTime")]
    pub rent_time: u32,
    #[serde(rename = "Type")]
    pub type_: HouseType,
    #[serde(rename = "MaintenanceFree")]
    pub maintenance_free: bool,
    #[serde(rename = "Buy")]
    pub buy: PackableList<HousePayment>,
    #[serde(rename = "Rent")]
    pub rent: PackableList<HousePayment>,
    #[serde(rename = "Position")]
    pub position: Position,
}

// Set of information related to house access
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HAR {
    #[serde(rename = "Version")]
    pub version: u32,
    #[serde(rename = "Bitmask")]
    pub bitmask: u32,
    #[serde(rename = "MonarchId")]
    pub monarch_id: ObjectId,
    #[serde(rename = "GuestList")]
    pub guest_list: PackableHashTable<ObjectId, GuestInfo>,
    #[serde(rename = "RoommateList")]
    pub roommate_list: PackableList<ObjectId>,
}

// Set of information related to a house guest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestInfo {
    #[serde(rename = "HasStoragePermission")]
    pub has_storage_permission: bool,
    #[serde(rename = "GuestName")]
    pub guest_name: String,
}

// Set of information related to a chess game move
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameMoveDataType4 {
    #[serde(rename = "PlayerId")]
    pub player_id: ObjectId,
    #[serde(rename = "Team")]
    pub team: i32,
    #[serde(rename = "IdPieceToMove")]
    pub id_piece_to_move: i32,
    #[serde(rename = "YGrid")]
    pub y_grid: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameMoveDataType5 {
    #[serde(rename = "PlayerId")]
    pub player_id: ObjectId,
    #[serde(rename = "Team")]
    pub team: i32,
    #[serde(rename = "IdPieceToMove")]
    pub id_piece_to_move: i32,
    #[serde(rename = "YGrid")]
    pub y_grid: i32,
    #[serde(rename = "XTo")]
    pub x_to: i32,
    #[serde(rename = "YTo")]
    pub y_to: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameMoveDataType6 {
    #[serde(rename = "PlayerId")]
    pub player_id: ObjectId,
    #[serde(rename = "Team")]
    pub team: i32,
    #[serde(rename = "IdPieceToMove")]
    pub id_piece_to_move: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum GameMoveData {
    Type4(GameMoveDataType4),
    Type5(GameMoveDataType5),
    Type6(GameMoveDataType6),
}

// Set of information related to a salvage operation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SalvageOperationsResultData {
    #[serde(rename = "SkillUsed")]
    pub skill_used: SkillId,
    #[serde(rename = "NotSalvagable")]
    pub not_salvagable: PackableList<ObjectId>,
    #[serde(rename = "SalvageResults")]
    pub salvage_results: PackableList<SalvageResult>,
    #[serde(rename = "AugBonus")]
    pub aug_bonus: i32,
}

// Set of information related to a salvage of an item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SalvageResult {
    #[serde(rename = "Material")]
    pub material: MaterialType,
    #[serde(rename = "Workmanship")]
    pub workmanship: f64,
    #[serde(rename = "Units")]
    pub units: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FellowshipLockData {
    #[serde(rename = "Unknown1")]
    pub unknown1: u32,
    #[serde(rename = "Unknown2")]
    pub unknown2: u32,
    #[serde(rename = "Unknown3")]
    pub unknown3: u32,
    #[serde(rename = "Timestamp")]
    pub timestamp: u32,
    #[serde(rename = "Sequence")]
    pub sequence: u32,
}

// Set of information for a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship {
    #[serde(rename = "Members")]
    pub members: PackableHashTable<ObjectId, Fellow>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "LeaderId")]
    pub leader_id: ObjectId,
    #[serde(rename = "ShareXP")]
    pub share_xp: bool,
    #[serde(rename = "EvenXPSplit")]
    pub even_xp_split: bool,
    #[serde(rename = "Open")]
    pub open: bool,
    #[serde(rename = "Locked")]
    pub locked: bool,
    #[serde(rename = "RecentlyDeparted")]
    pub recently_departed: PackableHashTable<ObjectId, i32>,
    #[serde(rename = "Locks")]
    pub locks: PackableHashTable<String, FellowshipLockData>,
}

// The FellowInfo structure contains information about a fellowship member.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellow {
    #[serde(rename = "XPCached")]
    pub xp_cached: u32,
    #[serde(rename = "LumCached")]
    pub lum_cached: u32,
    #[serde(rename = "Level")]
    pub level: u32,
    #[serde(rename = "MaxHealth")]
    pub max_health: u32,
    #[serde(rename = "MaxStamina")]
    pub max_stamina: u32,
    #[serde(rename = "MaxMana")]
    pub max_mana: u32,
    #[serde(rename = "CurrentHealth")]
    pub current_health: u32,
    #[serde(rename = "CurrentStamina")]
    pub current_stamina: u32,
    #[serde(rename = "CurrentMana")]
    pub current_mana: u32,
    #[serde(rename = "ShareLoot")]
    pub share_loot: bool,
    #[serde(rename = "Name")]
    pub name: String,
}

// Contains information about a contract.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractTracker {
    #[serde(rename = "Version")]
    pub version: u32,
    #[serde(rename = "ContractId")]
    pub contract_id: ContractId,
    #[serde(rename = "ContractStage")]
    pub contract_stage: ContractStage,
    #[serde(rename = "TimeWhenDone")]
    pub time_when_done: i64,
    #[serde(rename = "TimeWhenRepeats")]
    pub time_when_repeats: i64,
}

// Contains table of ContractTrackers
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractTrackerTable {
    #[serde(rename = "ContactTrackers")]
    pub contact_trackers: PackableHashTable<u32, ContractTracker>,
}

impl WString {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_wstring(reader)?))
    }
}

impl crate::readers::ACDataType for WString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WString::read(reader)
    }
}

impl PackedWORD {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for PackedWORD {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PackedWORD::read(reader)
    }
}

impl PackedDWORD {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for PackedDWORD {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PackedDWORD::read(reader)
    }
}

impl ObjectId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u32(reader)?))
    }
}

impl crate::readers::ACDataType for ObjectId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ObjectId::read(reader)
    }
}

impl LandcellId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u32(reader)?))
    }
}

impl crate::readers::ACDataType for LandcellId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LandcellId::read(reader)
    }
}

impl SpellId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u16(reader)?))
    }
}

impl crate::readers::ACDataType for SpellId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SpellId::read(reader)
    }
}

impl crate::readers::ACDataType for LayeredSpellId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let id = SpellId::read(reader)?;
        let layer = read_u16(reader)?;

        Ok(Self {
            id,
            layer,
        })
    }
}

impl crate::readers::ACDataType for Vector3 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let x = read_f32(reader)?;
        let y = read_f32(reader)?;
        let z = read_f32(reader)?;

        Ok(Self {
            x,
            y,
            z,
        })
    }
}

impl crate::readers::ACDataType for Quaternion {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let w = read_f32(reader)?;
        let x = read_f32(reader)?;
        let y = read_f32(reader)?;
        let z = read_f32(reader)?;

        Ok(Self {
            w,
            x,
            y,
            z,
        })
    }
}

impl crate::readers::ACDataType for Origin {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let landcell = LandcellId::read(reader)?;
        let location = Vector3::read(reader)?;

        Ok(Self {
            landcell,
            location,
        })
    }
}

impl crate::readers::ACDataType for Position {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let landcell = LandcellId::read(reader)?;
        let frame = Frame::read(reader)?;

        Ok(Self {
            landcell,
            frame,
        })
    }
}

impl crate::readers::ACDataType for Frame {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let origin = Vector3::read(reader)?;
        let orientation = Quaternion::read(reader)?;

        Ok(Self {
            origin,
            orientation,
        })
    }
}

impl crate::readers::ACDataType for ServerSwitchHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u32(reader)?;
        let type_ = ServerSwitchType::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for CICMDCommandHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let command = read_u32(reader)?;
        let parameter = read_u32(reader)?;

        Ok(Self {
            command,
            parameter,
        })
    }
}

impl crate::readers::ACDataType for FlowHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = read_u32(reader)?;
        let interval = read_u16(reader)?;

        Ok(Self {
            bytes,
            interval,
        })
    }
}

impl crate::readers::ACDataType for SocketAddress {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let family = read_i16(reader)?;
        let port = read_u16(reader)?;
        let address = read_u32(reader)?;
        let empty = read_u64(reader)?;

        Ok(Self {
            family,
            port,
            address,
            empty,
        })
    }
}

impl LoginRequestHeaderType2 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, client_version: string, length: uint, flags: AuthFlags, sequence: uint, account: string, account_to_login_as: string) -> Result<Self, Box<dyn std::error::Error>> {
        let password = read_wstring(reader).map(WString)?;

        Ok(Self {
            client_version,
            length,
            flags,
            sequence,
            account,
            account_to_login_as,
            password,
        })
    }
}

impl LoginRequestHeaderType40000002 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, client_version: string, length: uint, flags: AuthFlags, sequence: uint, account: string, account_to_login_as: string) -> Result<Self, Box<dyn std::error::Error>> {
        let gls_ticket = read_string(reader)?;

        Ok(Self {
            client_version,
            length,
            flags,
            sequence,
            account,
            account_to_login_as,
            gls_ticket,
        })
    }
}

impl LoginRequestHeader {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let client_version = read_string(reader)?;
        let length = read_u32(reader)?;
        let auth_type = NetAuthType::try_from(read_u32(reader)?)?;
        let flags = AuthFlags::try_from(read_u32(reader)?)?;
        let sequence = read_u32(reader)?;
        let account = read_string(reader)?;
        let account_to_login_as = read_string(reader)?;

        match auth_type {
            NetAuthType::AccountPassword => {
                let variant_struct = LoginRequestHeaderType2::read(reader, client_version, length, flags, sequence, account, account_to_login_as)?;
                Ok(Self::Type2(variant_struct))
            },
            NetAuthType::GlsTicket => {
                let variant_struct = LoginRequestHeaderType40000002::read(reader, client_version, length, flags, sequence, account, account_to_login_as)?;
                Ok(Self::Type40000002(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "auth_type", auth_type).into()),
        }
    }
}

impl crate::readers::ACDataType for LoginRequestHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginRequestHeader::read(reader)
    }
}

impl crate::readers::ACDataType for ReferralHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let cookie = read_u64(reader)?;
        let address = SocketAddress::read(reader)?;
        let id_server = read_u16(reader)?;
        let _ = align_dword(reader)?;
        let unknown = DWORD::read(reader)?;

        Ok(Self {
            cookie,
            address,
            id_server,
            unknown,
        })
    }
}

impl crate::readers::ACDataType for ConnectRequestHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let server_time = read_f64(reader)?;
        let cookie = read_u64(reader)?;
        let net_id = read_i32(reader)?;
        let outgoing_seed = read_u32(reader)?;
        let incoming_seed = read_u32(reader)?;
        let unknown = DWORD::read(reader)?;

        Ok(Self {
            server_time,
            cookie,
            net_id,
            outgoing_seed,
            incoming_seed,
            unknown,
        })
    }
}

impl crate::readers::ACDataType for NetError {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let string_id = DataId::read(reader)?;
        let table_id = DataId::read(reader)?;

        Ok(Self {
            string_id,
            table_id,
        })
    }
}

impl crate::readers::ACDataType for EchoResponseHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let local_time = read_f32(reader)?;
        let holding_time = read_f32(reader)?;

        Ok(Self {
            local_time,
            holding_time,
        })
    }
}

impl crate::readers::ACDataType for ACBaseQualities {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = Ok::<_, Box<dyn std::error::Error>>(ACBaseQualitiesFlags::from_bits_retain(read_u32(reader)?))?;
        let weenie_type = WeenieType::try_from(read_u32(reader)?)?;
        let mut int_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_INT.bits()) != 0 {
            int_properties = Some(read_packable_hash_table::<PropertyInt, i32>(reader)?);
        }
        let mut int64_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_INT64.bits()) != 0 {
            int64_properties = Some(read_packable_hash_table::<PropertyInt64, i64>(reader)?);
        }
        let mut bool_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_BOOL.bits()) != 0 {
            bool_properties = Some(read_packable_hash_table::<PropertyBool, bool>(reader)?);
        }
        let mut float_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_FLOAT.bits()) != 0 {
            float_properties = Some(read_packable_hash_table::<PropertyFloat, f64>(reader)?);
        }
        let mut string_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_STRING.bits()) != 0 {
            string_properties = Some(read_packable_hash_table::<PropertyString, String>(reader)?);
        }
        let mut data_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_DATA_ID.bits()) != 0 {
            data_properties = Some(read_packable_hash_table::<PropertyDataId, DataId>(reader)?);
        }
        let mut instance_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_INSTANCE_ID.bits()) != 0 {
            instance_properties = Some(read_packable_hash_table::<PropertyInstanceId, ObjectId>(reader)?);
        }
        let mut position_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_POSITION.bits()) != 0 {
            position_properties = Some(read_packable_hash_table::<PropertyPosition, Position>(reader)?);
        }

        Ok(Self {
            flags,
            weenie_type,
            int_properties,
            int64_properties,
            bool_properties,
            float_properties,
            string_properties,
            data_properties,
            instance_properties,
            position_properties,
        })
    }
}

impl crate::readers::ACDataType for ACQualities {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = Ok::<_, Box<dyn std::error::Error>>(ACQualitiesFlags::from_bits_retain(read_u32(reader)?))?;
        let has_health = read_bool(reader)?;
        let mut attributes = None;
        if (flags.bits() & ACQualitiesFlags::ATTRIBUTES.bits()) != 0 {
            attributes = Some(AttributeCache::read(reader)?);
        }
        let mut skills = None;
        if (flags.bits() & ACQualitiesFlags::SKILLS.bits()) != 0 {
            skills = Some(read_packable_hash_table::<SkillId, Skill>(reader)?);
        }
        let mut body = None;
        if (flags.bits() & ACQualitiesFlags::BODY.bits()) != 0 {
            body = Some(Body::read(reader)?);
        }
        let mut spell_book = None;
        if (flags.bits() & ACQualitiesFlags::SPELL_BOOK.bits()) != 0 {
            spell_book = Some(read_packable_hash_table::<LayeredSpellId, SpellBookPage>(reader)?);
        }
        let mut enchantments = None;
        if (flags.bits() & ACQualitiesFlags::ENCHANTMENTS.bits()) != 0 {
            enchantments = Some(EnchantmentRegistry::read(reader)?);
        }
        let mut event_filter = None;
        if (flags.bits() & ACQualitiesFlags::EVENT_FILTER.bits()) != 0 {
            event_filter = Some(EventFilter::read(reader)?);
        }
        let mut emotes = None;
        if (flags.bits() & ACQualitiesFlags::EMOTES.bits()) != 0 {
            emotes = Some(EmoteTable::read(reader)?);
        }
        let mut creation_profile = None;
        if (flags.bits() & ACQualitiesFlags::CREATION_PROFILE.bits()) != 0 {
            creation_profile = Some(read_packable_list::<CreationProfile>(reader)?);
        }
        let mut page_data = None;
        if (flags.bits() & ACQualitiesFlags::PAGE_DATA.bits()) != 0 {
            page_data = Some(PageDataList::read(reader)?);
        }
        let mut generators = None;
        if (flags.bits() & ACQualitiesFlags::GENERATORS.bits()) != 0 {
            generators = Some(GeneratorTable::read(reader)?);
        }
        let mut generator_registry = None;
        if (flags.bits() & ACQualitiesFlags::GENERATOR_REGISTRY.bits()) != 0 {
            generator_registry = Some(GeneratorRegistry::read(reader)?);
        }
        let mut generator_queue = None;
        if (flags.bits() & ACQualitiesFlags::GENERATOR_QUEUE.bits()) != 0 {
            generator_queue = Some(GeneratorQueue::read(reader)?);
        }

        Ok(Self {
            flags,
            has_health,
            attributes,
            skills,
            body,
            spell_book,
            enchantments,
            event_filter,
            emotes,
            creation_profile,
            page_data,
            generators,
            generator_registry,
            generator_queue,
        })
    }
}

impl crate::readers::ACDataType for AttributeCache {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let mut strength = None;
        if (flags & 0x00000001) != 0 {
            strength = Some(AttributeInfo::read(reader)?);
        }
        let mut endurance = None;
        if (flags & 0x00000002) != 0 {
            endurance = Some(AttributeInfo::read(reader)?);
        }
        let mut quickness = None;
        if (flags & 0x00000004) != 0 {
            quickness = Some(AttributeInfo::read(reader)?);
        }
        let mut coordination = None;
        if (flags & 0x00000008) != 0 {
            coordination = Some(AttributeInfo::read(reader)?);
        }
        let mut focus = None;
        if (flags & 0x00000010) != 0 {
            focus = Some(AttributeInfo::read(reader)?);
        }
        let mut self_ = None;
        if (flags & 0x00000020) != 0 {
            self_ = Some(AttributeInfo::read(reader)?);
        }
        let mut health = None;
        if (flags & 0x00000040) != 0 {
            health = Some(SecondaryAttributeInfo::read(reader)?);
        }
        let mut stamina = None;
        if (flags & 0x00000080) != 0 {
            stamina = Some(SecondaryAttributeInfo::read(reader)?);
        }
        let mut mana = None;
        if (flags & 0x00000100) != 0 {
            mana = Some(SecondaryAttributeInfo::read(reader)?);
        }

        Ok(Self {
            flags,
            strength,
            endurance,
            quickness,
            coordination,
            focus,
            self_,
            health,
            stamina,
            mana,
        })
    }
}

impl crate::readers::ACDataType for AttributeInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let points_raised = read_u32(reader)?;
        let innate_points = read_u32(reader)?;
        let experience_spent = read_u32(reader)?;

        Ok(Self {
            points_raised,
            innate_points,
            experience_spent,
        })
    }
}

impl crate::readers::ACDataType for SecondaryAttributeInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let attribute = AttributeInfo::read(reader)?;
        let current = read_u32(reader)?;

        Ok(Self {
            attribute,
            current,
        })
    }
}

impl crate::readers::ACDataType for Skill {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let points_raised = read_u16(reader)?;
        let adjust_pp = read_u16(reader)?;
        let training_level = SkillAdvancementClass::try_from(read_u32(reader)?)?;
        let experience_spent = read_u32(reader)?;
        let innate_points = read_u32(reader)?;
        let resistance_of_last_check = read_u32(reader)?;
        let last_used_time = read_f64(reader)?;

        Ok(Self {
            points_raised,
            adjust_pp,
            training_level,
            experience_spent,
            innate_points,
            resistance_of_last_check,
            last_used_time,
        })
    }
}

impl crate::readers::ACDataType for Body {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let body_parts = read_packable_hash_table::<u32, BodyPart>(reader)?;

        Ok(Self {
            body_parts,
        })
    }
}

impl crate::readers::ACDataType for BodyPart {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let has_bpsd = read_i32(reader)?;
        let damage_type = Ok::<_, Box<dyn std::error::Error>>(DamageType::from_bits_retain(read_u32(reader)?))?;
        let damage_val = read_i32(reader)?;
        let damage_var = read_i32(reader)?;
        let armor_cache = ArmorCache::read(reader)?;
        let bh = read_i32(reader)?;
        let mut bpsd = None;
        if (has_bpsd & 0x00000001) != 0 {
            bpsd = Some(BodyPartSelectionData::read(reader)?);
        }

        Ok(Self {
            has_bpsd,
            damage_type,
            damage_val,
            damage_var,
            armor_cache,
            bh,
            bpsd,
        })
    }
}

impl crate::readers::ACDataType for ArmorCache {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let base_armor = read_i32(reader)?;
        let armor_vs_slash = read_i32(reader)?;
        let armor_vs_pierce = read_i32(reader)?;
        let armor_vs_bludgeon = read_i32(reader)?;
        let armor_vs_cold = read_i32(reader)?;
        let armor_vs_fire = read_i32(reader)?;
        let armor_vs_acid = read_i32(reader)?;
        let armor_vs_electric = read_i32(reader)?;
        let armor_vs_nether = read_i32(reader)?;

        Ok(Self {
            base_armor,
            armor_vs_slash,
            armor_vs_pierce,
            armor_vs_bludgeon,
            armor_vs_cold,
            armor_vs_fire,
            armor_vs_acid,
            armor_vs_electric,
            armor_vs_nether,
        })
    }
}

impl crate::readers::ACDataType for BodyPartSelectionData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let hlf = read_i32(reader)?;
        let mlf = read_i32(reader)?;
        let llf = read_i32(reader)?;
        let hrf = read_i32(reader)?;
        let mrf = read_i32(reader)?;
        let lrf = read_i32(reader)?;
        let hlb = read_i32(reader)?;
        let mlb = read_i32(reader)?;
        let llb = read_i32(reader)?;
        let hrb = read_i32(reader)?;
        let mrb = read_i32(reader)?;
        let lrb = read_i32(reader)?;

        Ok(Self {
            hlf,
            mlf,
            llf,
            hrf,
            mrf,
            lrf,
            hlb,
            mlb,
            llb,
            hrb,
            mrb,
            lrb,
        })
    }
}

impl crate::readers::ACDataType for SpellBookPage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let casting_likelihood = read_f32(reader)?;
        let mut casting_likelihood2 = None;
        let mut unknown = None;
        if casting_likelihood < 2.0 {
            casting_likelihood2 = if casting_likelihood < 2.0 { read_f32(reader).map(Some) } else { Ok(None) }?;
            unknown = if casting_likelihood < 2.0 { read_i32(reader).map(Some) } else { Ok(None) }?;
        }

        Ok(Self {
            casting_likelihood,
            casting_likelihood2,
            unknown,
        })
    }
}

impl crate::readers::ACDataType for EnchantmentRegistry {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = Ok::<_, Box<dyn std::error::Error>>(EnchantmentRegistryFlags::from_bits_retain(read_u32(reader)?))?;
        let mut life_spells = None;
        if (flags.bits() & EnchantmentRegistryFlags::LIFE_SPELLS.bits()) != 0 {
            life_spells = Some(read_packable_list::<Enchantment>(reader)?);
        }
        let mut creature_spells = None;
        if (flags.bits() & EnchantmentRegistryFlags::CREATURE_SPELLS.bits()) != 0 {
            creature_spells = Some(read_packable_list::<Enchantment>(reader)?);
        }
        let mut vitae = None;
        if (flags.bits() & EnchantmentRegistryFlags::VITAE.bits()) != 0 {
            vitae = Some(Enchantment::read(reader)?);
        }
        let mut cooldowns = None;
        if (flags.bits() & EnchantmentRegistryFlags::COOLDOWNS.bits()) != 0 {
            cooldowns = Some(read_packable_list::<Enchantment>(reader)?);
        }

        Ok(Self {
            flags,
            life_spells,
            creature_spells,
            vitae,
            cooldowns,
        })
    }
}

impl crate::readers::ACDataType for Enchantment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let id = LayeredSpellId::read(reader)?;
        let has_equipment_set = read_u16(reader)?;
        let spell_category = SpellCategory::try_from(read_u16(reader)?)?;
        let power_level = read_u32(reader)?;
        let start_time = read_f64(reader)?;
        let duration = read_f64(reader)?;
        let caster_id = ObjectId::read(reader)?;
        let degrade_modifier = read_f32(reader)?;
        let degrade_limit = read_f32(reader)?;
        let last_time_degraded = read_f64(reader)?;
        let stat_mod = StatMod::read(reader)?;
        let mut equipment_set = None;
        if has_equipment_set > 0 {
            equipment_set = if has_equipment_set > 0 { EquipmentSet::try_from(read_u32(reader)?).map(Some) } else { Ok(None) }?;
        }

        Ok(Self {
            id,
            has_equipment_set,
            spell_category,
            power_level,
            start_time,
            duration,
            caster_id,
            degrade_modifier,
            degrade_limit,
            last_time_degraded,
            stat_mod,
            equipment_set,
        })
    }
}

impl crate::readers::ACDataType for StatMod {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = Ok::<_, Box<dyn std::error::Error>>(EnchantmentTypeFlags::from_bits_retain(read_u32(reader)?))?;
        let key = read_u32(reader)?;
        let value = read_f32(reader)?;

        Ok(Self {
            type_,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for EventFilter {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let events = read_packable_list::<u32>(reader)?;

        Ok(Self {
            events,
        })
    }
}

impl crate::readers::ACDataType for EmoteTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let emotes = read_packable_hash_table::<EmoteCategory, EmoteSetList>(reader)?;

        Ok(Self {
            emotes,
        })
    }
}

impl crate::readers::ACDataType for EmoteSetList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let emotes = read_packable_list::<EmoteSet>(reader)?;

        Ok(Self {
            emotes,
        })
    }
}

impl EmoteSetType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, probability: float, emotes: PackableList<Emote>) -> Result<Self, Box<dyn std::error::Error>> {
        let class_id = read_u32(reader)?;

        Ok(Self {
            probability,
            emotes,
            class_id,
        })
    }
}

impl EmoteSetType2 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, probability: float, emotes: PackableList<Emote>) -> Result<Self, Box<dyn std::error::Error>> {
        let vendor_type = read_u32(reader)?;

        Ok(Self {
            probability,
            emotes,
            vendor_type,
        })
    }
}

impl EmoteSetType5 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, probability: float, emotes: PackableList<Emote>) -> Result<Self, Box<dyn std::error::Error>> {
        let style = read_u32(reader)?;
        let substyle = read_u32(reader)?;

        Ok(Self {
            probability,
            emotes,
            style,
            substyle,
        })
    }
}

impl EmoteSetTypeC {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, probability: float, emotes: PackableList<Emote>) -> Result<Self, Box<dyn std::error::Error>> {
        let quest = read_string(reader)?;

        Ok(Self {
            probability,
            emotes,
            quest,
        })
    }
}

impl EmoteSetTypeF {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, probability: float, emotes: PackableList<Emote>) -> Result<Self, Box<dyn std::error::Error>> {
        let min_health = read_f32(reader)?;
        let max_health = read_f32(reader)?;

        Ok(Self {
            probability,
            emotes,
            min_health,
            max_health,
        })
    }
}

impl EmoteSet {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let category = EmoteCategory::try_from(read_u32(reader)?)?;
        let probability = read_f32(reader)?;
        let emotes = read_packable_list::<Emote>(reader)?;

        match category {
            EmoteCategory::RefuseEmoteCategory | EmoteCategory::GiveEmoteCategory => {
                let variant_struct = EmoteSetType1::read(reader, probability, emotes)?;
                Ok(Self::Type1(variant_struct))
            },
            EmoteCategory::VendorEmoteCategory => {
                let variant_struct = EmoteSetType2::read(reader, probability, emotes)?;
                Ok(Self::Type2(variant_struct))
            },
            EmoteCategory::HeartBeatEmoteCategory => {
                let variant_struct = EmoteSetType5::read(reader, probability, emotes)?;
                Ok(Self::Type5(variant_struct))
            },
            EmoteCategory::QuestSuccessEmoteCategory | EmoteCategory::QuestFailureEmoteCategory | EmoteCategory::TestSuccessEmoteCategory | EmoteCategory::TestFailureEmoteCategory | EmoteCategory::EventSuccessEmoteCategory | EmoteCategory::EventFailureEmoteCategory | EmoteCategory::TestNoQualityEmoteCategory | EmoteCategory::QuestNoFellowEmoteCategory | EmoteCategory::TestNoFellowEmoteCategory | EmoteCategory::GotoSetEmoteCategory | EmoteCategory::NumFellowsSuccessEmoteCategory | EmoteCategory::NumFellowsFailureEmoteCategory | EmoteCategory::NumCharacterTitlesSuccessEmoteCategory | EmoteCategory::NumCharacterTitlesFailureEmoteCategory | EmoteCategory::ReceiveLocalSignalEmoteCategory | EmoteCategory::ReceiveTalkDirectEmoteCategory => {
                let variant_struct = EmoteSetTypeC::read(reader, probability, emotes)?;
                Ok(Self::TypeC(variant_struct))
            },
            EmoteCategory::WoundedTauntEmoteCategory => {
                let variant_struct = EmoteSetTypeF::read(reader, probability, emotes)?;
                Ok(Self::TypeF(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "category", category).into()),
        }
    }
}

impl crate::readers::ACDataType for EmoteSet {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EmoteSet::read(reader)
    }
}

impl EmoteType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
        })
    }
}

impl EmoteType2 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let amount64 = read_u64(reader)?;
        let hero_xp64 = read_u64(reader)?;

        Ok(Self {
            delay,
            extent,
            amount64,
            hero_xp64,
        })
    }
}

impl EmoteType3 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let c_profile = CreationProfile::read(reader)?;

        Ok(Self {
            delay,
            extent,
            c_profile,
        })
    }
}

impl EmoteType4 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let frame = Frame::read(reader)?;

        Ok(Self {
            delay,
            extent,
            frame,
        })
    }
}

impl EmoteType5 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let motion = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            motion,
        })
    }
}

impl EmoteType7 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let physics_script = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            physics_script,
        })
    }
}

impl EmoteType9 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let sound = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            sound,
        })
    }
}

impl EmoteTypeE {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            spell_id,
        })
    }
}

impl EmoteType1C {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let amount = read_u32(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            amount,
            stat,
        })
    }
}

impl EmoteType1E {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let min = read_u32(reader)?;
        let max = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            min,
            max,
        })
    }
}

impl EmoteType20 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            amount,
        })
    }
}

impl EmoteType22 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let amount = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            amount,
        })
    }
}

impl EmoteType23 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            stat,
        })
    }
}

impl EmoteType24 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let min = read_u32(reader)?;
        let max = read_u32(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            min,
            max,
            stat,
        })
    }
}

impl EmoteType25 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let f_min = read_f64(reader)?;
        let f_max = read_f64(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            f_min,
            f_max,
            stat,
        })
    }
}

impl EmoteType26 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let test_string = read_string(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            test_string,
            stat,
        })
    }
}

impl EmoteType31 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let percent = read_f64(reader)?;
        let min64 = read_u64(reader)?;
        let max64 = read_u64(reader)?;

        Ok(Self {
            delay,
            extent,
            percent,
            min64,
            max64,
        })
    }
}

impl EmoteType32 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let stat = read_u32(reader)?;
        let percent = read_f64(reader)?;
        let min = read_u32(reader)?;
        let max = read_u32(reader)?;
        let display = read_bool(reader)?;

        Ok(Self {
            delay,
            extent,
            stat,
            percent,
            min,
            max,
            display,
        })
    }
}

impl EmoteType35 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let stat = read_u32(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            stat,
            amount,
        })
    }
}

impl EmoteType38 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let wealth_rating = read_i32(reader)?;
        let treasure_class = read_i32(reader)?;
        let treasure_type = read_i32(reader)?;

        Ok(Self {
            delay,
            extent,
            wealth_rating,
            treasure_class,
            treasure_type,
        })
    }
}

impl EmoteType3F {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let position = Position::read(reader)?;

        Ok(Self {
            delay,
            extent,
            position,
        })
    }
}

impl EmoteType4C {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let msg = read_string(reader)?;
        let c_profile = CreationProfile::read(reader)?;

        Ok(Self {
            delay,
            extent,
            msg,
            c_profile,
        })
    }
}

impl EmoteType6E {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            stat,
        })
    }
}

impl EmoteType70 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let amount64 = read_u64(reader)?;

        Ok(Self {
            delay,
            extent,
            amount64,
        })
    }
}

impl EmoteType72 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let min64 = read_u64(reader)?;
        let max64 = read_u64(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            min64,
            max64,
            stat,
        })
    }
}

impl EmoteType76 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let stat = read_u32(reader)?;
        let percent = read_f64(reader)?;

        Ok(Self {
            delay,
            extent,
            stat,
            percent,
        })
    }
}

impl Emote {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = EmoteType::try_from(read_u32(reader)?)?;
        let delay = read_f32(reader)?;
        let extent = read_f32(reader)?;

        match type_ {
            EmoteType::ActEmoteType | EmoteType::SayEmoteType | EmoteType::TellEmoteType | EmoteType::TextDirectEmoteType | EmoteType::WorldBroadcastEmoteType | EmoteType::LocalBroadcastEmoteType | EmoteType::DirectBroadcastEmoteType | EmoteType::UpdateQuestEmoteType | EmoteType::InqQuestEmoteType | EmoteType::StampQuestEmoteType | EmoteType::StartEventEmoteType | EmoteType::StopEventEmoteType | EmoteType::BLogEmoteType | EmoteType::AdminSpamEmoteType | EmoteType::EraseQuestEmoteType | EmoteType::InqEventEmoteType | EmoteType::InqFellowQuestEmoteType | EmoteType::UpdateFellowQuestEmoteType | EmoteType::StampFellowQuestEmoteType | EmoteType::TellFellowEmoteType | EmoteType::FellowBroadcastEmoteType | EmoteType::GotoEmoteType | EmoteType::PopUpEmoteType | EmoteType::UpdateMyQuestEmoteType | EmoteType::InqMyQuestEmoteType | EmoteType::StampMyQuestEmoteType | EmoteType::EraseMyQuestEmoteType | EmoteType::LocalSignalEmoteType | EmoteType::InqContractsFullEmoteType => {
                let variant_struct = EmoteType1::read(reader, delay, extent)?;
                Ok(Self::Type1(variant_struct))
            },
            EmoteType::AwardXPEmoteType | EmoteType::AwardNoShareXPEmoteType => {
                let variant_struct = EmoteType2::read(reader, delay, extent)?;
                Ok(Self::Type2(variant_struct))
            },
            EmoteType::GiveEmoteType | EmoteType::TakeItemsEmoteType => {
                let variant_struct = EmoteType3::read(reader, delay, extent)?;
                Ok(Self::Type3(variant_struct))
            },
            EmoteType::MoveHomeEmoteType | EmoteType::MoveEmoteType | EmoteType::TurnEmoteType | EmoteType::MoveToPosEmoteType => {
                let variant_struct = EmoteType4::read(reader, delay, extent)?;
                Ok(Self::Type4(variant_struct))
            },
            EmoteType::MotionEmoteType | EmoteType::ForceMotionEmoteType => {
                let variant_struct = EmoteType5::read(reader, delay, extent)?;
                Ok(Self::Type5(variant_struct))
            },
            EmoteType::PhysScriptEmoteType => {
                let variant_struct = EmoteType7::read(reader, delay, extent)?;
                Ok(Self::Type7(variant_struct))
            },
            EmoteType::SoundEmoteType => {
                let variant_struct = EmoteType9::read(reader, delay, extent)?;
                Ok(Self::Type9(variant_struct))
            },
            EmoteType::CastSpellEmoteType | EmoteType::CastSpellInstantEmoteType | EmoteType::TeachSpellEmoteType | EmoteType::PetCastSpellOnOwnerEmoteType => {
                let variant_struct = EmoteTypeE::read(reader, delay, extent)?;
                Ok(Self::TypeE(variant_struct))
            },
            EmoteType::AwardSkillXPEmoteType | EmoteType::AwardSkillPointsEmoteType => {
                let variant_struct = EmoteType1C::read(reader, delay, extent)?;
                Ok(Self::Type1C(variant_struct))
            },
            EmoteType::InqQuestSolvesEmoteType | EmoteType::InqFellowNumEmoteType | EmoteType::InqNumCharacterTitlesEmoteType | EmoteType::InqMyQuestSolvesEmoteType => {
                let variant_struct = EmoteType1E::read(reader, delay, extent)?;
                Ok(Self::Type1E(variant_struct))
            },
            EmoteType::DecrementQuestEmoteType | EmoteType::IncrementQuestEmoteType | EmoteType::SetQuestCompletionsEmoteType | EmoteType::DecrementMyQuestEmoteType | EmoteType::IncrementMyQuestEmoteType | EmoteType::SetMyQuestCompletionsEmoteType | EmoteType::InqPackSpaceEmoteType | EmoteType::InqQuestBitsOnEmoteType | EmoteType::InqQuestBitsOffEmoteType | EmoteType::InqMyQuestBitsOnEmoteType | EmoteType::InqMyQuestBitsOffEmoteType | EmoteType::SetQuestBitsOnEmoteType | EmoteType::SetQuestBitsOffEmoteType | EmoteType::SetMyQuestBitsOnEmoteType | EmoteType::SetMyQuestBitsOffEmoteType => {
                let variant_struct = EmoteType20::read(reader, delay, extent)?;
                Ok(Self::Type20(variant_struct))
            },
            EmoteType::AddCharacterTitleEmoteType | EmoteType::AwardTrainingCreditsEmoteType | EmoteType::InflictVitaePenaltyEmoteType | EmoteType::RemoveVitaePenaltyEmoteType | EmoteType::SetAltRacialSkillsEmoteType | EmoteType::AddContractEmoteType | EmoteType::RemoveContractEmoteType => {
                let variant_struct = EmoteType22::read(reader, delay, extent)?;
                Ok(Self::Type22(variant_struct))
            },
            EmoteType::InqBoolStatEmoteType | EmoteType::InqSkillTrainedEmoteType | EmoteType::InqSkillSpecializedEmoteType => {
                let variant_struct = EmoteType23::read(reader, delay, extent)?;
                Ok(Self::Type23(variant_struct))
            },
            EmoteType::InqIntStatEmoteType | EmoteType::InqAttributeStatEmoteType | EmoteType::InqRawAttributeStatEmoteType | EmoteType::InqSecondaryAttributeStatEmoteType | EmoteType::InqRawSecondaryAttributeStatEmoteType | EmoteType::InqSkillStatEmoteType | EmoteType::InqRawSkillStatEmoteType => {
                let variant_struct = EmoteType24::read(reader, delay, extent)?;
                Ok(Self::Type24(variant_struct))
            },
            EmoteType::InqFloatStatEmoteType => {
                let variant_struct = EmoteType25::read(reader, delay, extent)?;
                Ok(Self::Type25(variant_struct))
            },
            EmoteType::InqStringStatEmoteType | EmoteType::InqYesNoEmoteType => {
                let variant_struct = EmoteType26::read(reader, delay, extent)?;
                Ok(Self::Type26(variant_struct))
            },
            EmoteType::AwardLevelProportionalXPEmoteType => {
                let variant_struct = EmoteType31::read(reader, delay, extent)?;
                Ok(Self::Type31(variant_struct))
            },
            EmoteType::AwardLevelProportionalSkillXPEmoteType => {
                let variant_struct = EmoteType32::read(reader, delay, extent)?;
                Ok(Self::Type32(variant_struct))
            },
            EmoteType::SetIntStatEmoteType | EmoteType::IncrementIntStatEmoteType | EmoteType::DecrementIntStatEmoteType | EmoteType::SetBoolStatEmoteType => {
                let variant_struct = EmoteType35::read(reader, delay, extent)?;
                Ok(Self::Type35(variant_struct))
            },
            EmoteType::CreateTreasureEmoteType => {
                let variant_struct = EmoteType38::read(reader, delay, extent)?;
                Ok(Self::Type38(variant_struct))
            },
            EmoteType::SetSanctuaryPositionEmoteType | EmoteType::TeleportTargetEmoteType | EmoteType::TeleportSelfEmoteType => {
                let variant_struct = EmoteType3F::read(reader, delay, extent)?;
                Ok(Self::Type3F(variant_struct))
            },
            EmoteType::InqOwnsItemsEmoteType => {
                let variant_struct = EmoteType4C::read(reader, delay, extent)?;
                Ok(Self::Type4C(variant_struct))
            },
            EmoteType::UntrainSkillEmoteType | EmoteType::SetInt64StatEmoteType => {
                let variant_struct = EmoteType6E::read(reader, delay, extent)?;
                Ok(Self::Type6E(variant_struct))
            },
            EmoteType::SpendLuminanceEmoteType | EmoteType::AwardLuminanceEmoteType => {
                let variant_struct = EmoteType70::read(reader, delay, extent)?;
                Ok(Self::Type70(variant_struct))
            },
            EmoteType::InqInt64StatEmoteType => {
                let variant_struct = EmoteType72::read(reader, delay, extent)?;
                Ok(Self::Type72(variant_struct))
            },
            EmoteType::SetFloatStatEmoteType => {
                let variant_struct = EmoteType76::read(reader, delay, extent)?;
                Ok(Self::Type76(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "type_", type_).into()),
        }
    }
}

impl crate::readers::ACDataType for Emote {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Emote::read(reader)
    }
}

impl crate::readers::ACDataType for CreationProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let weenie_class_id = read_u32(reader)?;
        let palette = read_u32(reader)?;
        let shade = read_f32(reader)?;
        let destination = read_u32(reader)?;
        let stack_size = read_i32(reader)?;
        let try_to_bond = read_bool(reader)?;

        Ok(Self {
            weenie_class_id,
            palette,
            shade,
            destination,
            stack_size,
            try_to_bond,
        })
    }
}

impl crate::readers::ACDataType for PageDataList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let max_num_pages = read_u32(reader)?;
        let max_num_chars_per_page = read_u32(reader)?;
        let pages = read_packable_list::<PageData>(reader)?;

        Ok(Self {
            max_num_pages,
            max_num_chars_per_page,
            pages,
        })
    }
}

impl crate::readers::ACDataType for PageData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let author_id = ObjectId::read(reader)?;
        let author_name = read_string(reader)?;
        let author_account = read_string(reader)?;
        let version = read_u32(reader)?;
        let text_included = read_bool(reader)?;
        let ignore_author = read_bool(reader)?;
        let mut page_text = None;
        if text_included {
            page_text = if text_included { read_string(reader).map(Some) } else { Ok(None) }?;
        }

        Ok(Self {
            author_id,
            author_name,
            author_account,
            version,
            text_included,
            ignore_author,
            page_text,
        })
    }
}

impl crate::readers::ACDataType for BlobFragments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u32(reader)?;
        let id = read_u32(reader)?;
        let count = read_u16(reader)?;
        let size = read_u16(reader)?;
        let body_size = (size - 16) as u16;
        let index = read_u16(reader)?;
        let group = FragmentGroup::try_from(read_u16(reader)?)?;
        let data = read_vec::<u8>(reader, body_size as usize)?;

        Ok(Self {
            sequence,
            id,
            count,
            size,
            index,
            group,
            data,
        })
    }
}

impl crate::readers::ACDataType for GeneratorTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let generators = read_packable_list::<GeneratorProfile>(reader)?;

        Ok(Self {
            generators,
        })
    }
}

impl crate::readers::ACDataType for GeneratorProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let probability = read_f32(reader)?;
        let type_id = read_u32(reader)?;
        let delay = read_f64(reader)?;
        let init_create = read_u32(reader)?;
        let max_num = read_u32(reader)?;
        let when_create = read_u32(reader)?;
        let where_create = read_u32(reader)?;
        let stack_size = read_u32(reader)?;
        let ptid = read_u32(reader)?;
        let shade = read_f32(reader)?;
        let pos_val = Position::read(reader)?;
        let slot = read_u32(reader)?;

        Ok(Self {
            probability,
            type_id,
            delay,
            init_create,
            max_num,
            when_create,
            where_create,
            stack_size,
            ptid,
            shade,
            pos_val,
            slot,
        })
    }
}

impl crate::readers::ACDataType for GeneratorRegistry {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = read_packable_hash_table::<u32, GeneratorRegistryNode>(reader)?;

        Ok(Self {
            registry,
        })
    }
}

impl crate::readers::ACDataType for GeneratorRegistryNode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let wcid_or_type = read_u32(reader)?;
        let ts = read_f64(reader)?;
        let treasure_type = read_u32(reader)?;
        let slot = read_u32(reader)?;
        let checkpointed = read_u32(reader)?;
        let shop = read_u32(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            wcid_or_type,
            ts,
            treasure_type,
            slot,
            checkpointed,
            shop,
            amount,
        })
    }
}

impl crate::readers::ACDataType for GeneratorQueue {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let queue = read_packable_list::<GeneratorQueueNode>(reader)?;

        Ok(Self {
            queue,
        })
    }
}

impl crate::readers::ACDataType for GeneratorQueueNode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let slot = read_u32(reader)?;
        let when = read_f64(reader)?;

        Ok(Self {
            slot,
            when,
        })
    }
}

impl WindowPropertyType1000007F {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_j = read_u32(reader)?;
        let value_j = read_u64(reader)?;

        Ok(Self {
            unknown_j,
            value_j,
        })
    }
}

impl WindowPropertyType10000086 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_i = read_u32(reader)?;
        let value_i = read_u32(reader)?;

        Ok(Self {
            unknown_i,
            value_i,
        })
    }
}

impl WindowPropertyType10000087 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_h = read_u32(reader)?;
        let value_h = read_u32(reader)?;

        Ok(Self {
            unknown_h,
            value_h,
        })
    }
}

impl WindowPropertyType10000088 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_f = read_u32(reader)?;
        let value_f = read_u32(reader)?;

        Ok(Self {
            unknown_f,
            value_f,
        })
    }
}

impl WindowPropertyType10000089 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_e = read_u32(reader)?;
        let value_e = read_u32(reader)?;

        Ok(Self {
            unknown_e,
            value_e,
        })
    }
}

impl WindowPropertyType1000008A {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_d = read_u32(reader)?;
        let value_d = read_u8(reader)?;

        Ok(Self {
            unknown_d,
            value_d,
        })
    }
}

impl WindowPropertyType1000008D {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_c = read_u32(reader)?;
        let title_source = WindowPropertyType1000008DTitleSourceVariant::read(reader)?;

        Ok(Self {
            unknown_c,
            title_source,
        })
    }
}

impl WindowPropertyType1000008DTitleSourceVariant {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let title_source = read_u8(reader)?;

        match title_source {
            0x00 => {
                let string_id = read_u32(reader)?;
                let file_id = read_u32(reader)?;
                Ok(Self::Type0(WindowPropertyType1000008DTitleSourceVariantType0 {
                    string_id,
                    file_id,
                }))
            },
            0x01 => {
                let value_a = read_wstring(reader).map(WString)?;
                Ok(Self::Type1(WindowPropertyType1000008DTitleSourceVariantType1 {
                    value_a,
                }))
            },
            _ => Err("Unknown nested switch value".into()),
        }
    }
}


impl WindowProperty {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let key_a = read_u32(reader)?;

        match key_a {
            0x1000007F => {
                let variant_struct = WindowPropertyType1000007F::read(reader)?;
                Ok(Self::Type1000007F(variant_struct))
            },
            0x10000086 => {
                let variant_struct = WindowPropertyType10000086::read(reader)?;
                Ok(Self::Type10000086(variant_struct))
            },
            0x10000087 => {
                let variant_struct = WindowPropertyType10000087::read(reader)?;
                Ok(Self::Type10000087(variant_struct))
            },
            0x10000088 => {
                let variant_struct = WindowPropertyType10000088::read(reader)?;
                Ok(Self::Type10000088(variant_struct))
            },
            0x10000089 => {
                let variant_struct = WindowPropertyType10000089::read(reader)?;
                Ok(Self::Type10000089(variant_struct))
            },
            0x1000008A => {
                let variant_struct = WindowPropertyType1000008A::read(reader)?;
                Ok(Self::Type1000008A(variant_struct))
            },
            0x1000008D => {
                let variant_struct = WindowPropertyType1000008D::read(reader)?;
                Ok(Self::Type1000008D(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "key_a", key_a).into()),
        }
    }
}

impl crate::readers::ACDataType for WindowProperty {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WindowProperty::read(reader)
    }
}

impl WindowOptionType1000008B {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_b = read_u8(reader)?;
        let property_count = read_u8(reader)?;
        let properties = read_vec::<WindowProperty>(reader, property_count as usize)?;

        Ok(Self {
            unknown_b,
            property_count,
            properties,
        })
    }
}

impl WindowOption {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_a = read_u32(reader)?;

        match type_a {
            0x1000008B => {
                let variant_struct = WindowOptionType1000008B::read(reader)?;
                Ok(Self::Type1000008B(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "type_a", type_a).into()),
        }
    }
}

impl crate::readers::ACDataType for WindowOption {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WindowOption::read(reader)
    }
}

impl OptionPropertyType10000080 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_l = read_u32(reader)?;
        let inactive_opacity = read_f32(reader)?;

        Ok(Self {
            unknown_l,
            inactive_opacity,
        })
    }
}

impl OptionPropertyType10000081 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_k = read_u32(reader)?;
        let active_opacity = read_f32(reader)?;

        Ok(Self {
            unknown_k,
            active_opacity,
        })
    }
}

impl OptionPropertyType1000008C {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_a = read_u32(reader)?;
        let window_options = read_packable_list::<WindowOption>(reader)?;

        Ok(Self {
            unknown_a,
            window_options,
        })
    }
}

impl OptionProperty {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = read_u32(reader)?;

        match type_ {
            0x10000080 => {
                let variant_struct = OptionPropertyType10000080::read(reader)?;
                Ok(Self::Type10000080(variant_struct))
            },
            0x10000081 => {
                let variant_struct = OptionPropertyType10000081::read(reader)?;
                Ok(Self::Type10000081(variant_struct))
            },
            0x1000008C => {
                let variant_struct = OptionPropertyType1000008C::read(reader)?;
                Ok(Self::Type1000008C(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "type_", type_).into()),
        }
    }
}

impl crate::readers::ACDataType for OptionProperty {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        OptionProperty::read(reader)
    }
}

impl crate::readers::ACDataType for GameplayOptions {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let size = read_u32(reader)?;
        let unknown200_2 = read_u8(reader)?;
        let option_property_count = read_u8(reader)?;
        let option_properties = read_vec::<OptionProperty>(reader, option_property_count as usize)?;
        let _ = align_dword(reader)?;

        Ok(Self {
            size,
            unknown200_2,
            option_property_count,
            option_properties,
        })
    }
}

impl crate::readers::ACDataType for PlayerModule {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let options = Ok::<_, Box<dyn std::error::Error>>(CharacterOptions1::from_bits_retain(read_u32(reader)?))?;
        let mut shortcuts = None;
        if (flags & 0x00000001) != 0 {
            shortcuts = Some(read_packable_list::<ShortCutData>(reader)?);
        }
        let tab1_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab2_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab3_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab4_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab5_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab6_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab7_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab8_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let mut fill_comps = None;
        if (flags & 0x00000008) != 0 {
            fill_comps = Some(read_packable_hash_table::<u32, u32>(reader)?);
        }
        let mut spell_book_filters = None;
        if (flags & 0x00000020) != 0 {
            spell_book_filters = Some(read_u32(reader)?);
        }
        let mut option_flags = None;
        if (flags & 0x00000040) != 0 {
            option_flags = Some(read_u32(reader)?);
        }
        let mut unknown100_1 = None;
        let mut option_strings = None;
        if (flags & 0x00000100) != 0 {
            unknown100_1 = Some(read_u32(reader)?);
            option_strings = Some(read_packable_hash_table::<u32, String>(reader)?);
        }
        let mut gameplay_options = None;
        if (flags & 0x00000200) != 0 {
            gameplay_options = Some(GameplayOptions::read(reader)?);
        }

        Ok(Self {
            flags,
            options,
            shortcuts,
            tab1_spells,
            tab2_spells,
            tab3_spells,
            tab4_spells,
            tab5_spells,
            tab6_spells,
            tab7_spells,
            tab8_spells,
            fill_comps,
            spell_book_filters,
            option_flags,
            unknown100_1,
            option_strings,
            gameplay_options,
        })
    }
}

impl crate::readers::ACDataType for ShortCutManager {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let shortcuts = read_packable_list::<ShortCutData>(reader)?;

        Ok(Self {
            shortcuts,
        })
    }
}

impl crate::readers::ACDataType for ShortCutData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let index = read_u32(reader)?;
        let object_id = ObjectId::read(reader)?;
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            index,
            object_id,
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for SpellTab {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spells = read_packable_list::<LayeredSpellId>(reader)?;

        Ok(Self {
            spells,
        })
    }
}

impl crate::readers::ACDataType for ContentProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let container_type = ContainerProperties::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            container_type,
        })
    }
}

impl crate::readers::ACDataType for InventoryPlacement {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let location = Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?;
        let priority = Ok::<_, Box<dyn std::error::Error>>(CoverageMask::from_bits_retain(read_u32(reader)?))?;

        Ok(Self {
            object_id,
            location,
            priority,
        })
    }
}

impl crate::readers::ACDataType for AllegianceProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let total_members = read_u32(reader)?;
        let total_vassals = read_u32(reader)?;
        let hierarchy = AllegianceHierarchy::read(reader)?;

        Ok(Self {
            total_members,
            total_vassals,
            hierarchy,
        })
    }
}

impl crate::readers::ACDataType for AllegianceRecord {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let tree_parent = ObjectId::read(reader)?;
        let allegiance_data = AllegianceData::read(reader)?;

        Ok(Self {
            tree_parent,
            allegiance_data,
        })
    }
}

impl crate::readers::ACDataType for AllegianceHierarchy {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let record_count = read_u16(reader)?;
        let old_version = read_u16(reader)?;
        let officers = read_phash_table::<ObjectId, AllegianceOfficerLevel>(reader)?;
        let officer_titles = read_packable_list::<String>(reader)?;
        let monarch_broadcast_time = read_u32(reader)?;
        let monarch_broadcasts_today = read_u32(reader)?;
        let spokes_broadcast_time = read_u32(reader)?;
        let spokes_broadcasts_today = read_u32(reader)?;
        let motd = read_string(reader)?;
        let motd_set_by = read_string(reader)?;
        let chat_room_id = read_u32(reader)?;
        let bindpoint = Position::read(reader)?;
        let allegiance_name = read_string(reader)?;
        let name_last_set_time = read_u32(reader)?;
        let is_locked = read_bool(reader)?;
        let approved_vassal = read_i32(reader)?;
        let mut monarch_data = None;
        if record_count > 0 {
            monarch_data = if record_count > 0 { AllegianceData::read(reader).map(Some) } else { Ok(None) }?;
        }
        let records = read_vec::<AllegianceRecord>(reader, record_count as usize - 1)?;

        Ok(Self {
            record_count,
            old_version,
            officers,
            officer_titles,
            monarch_broadcast_time,
            monarch_broadcasts_today,
            spokes_broadcast_time,
            spokes_broadcasts_today,
            motd,
            motd_set_by,
            chat_room_id,
            bindpoint,
            allegiance_name,
            name_last_set_time,
            is_locked,
            approved_vassal,
            monarch_data,
            records,
        })
    }
}

impl crate::readers::ACDataType for AllegianceData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;
        let xp_cached = read_u32(reader)?;
        let xp_tithed = read_u32(reader)?;
        let flags = read_u32(reader)?;
        let gender = Gender::try_from(read_u8(reader)?)?;
        let heritage = HeritageGroup::try_from(read_u8(reader)?)?;
        let rank = read_u16(reader)?;
        let mut level = None;
        if (flags & 0x8) != 0 {
            level = Some(read_u32(reader)?);
        }
        let loyalty = read_u16(reader)?;
        let leadership = read_u16(reader)?;
        let mut allegiance_age = None;
        let time_online;
        if flags == 0x4 {
            time_online = if flags == 0x4 { read_u64(reader).map(Some) } else { Ok(None) }?;
        } else {
            allegiance_age = if flags == 0x4 { read_u32(reader).map(Some) } else { Ok(None) }?;
            time_online = if flags == 0x4 { read_u64(reader).map(Some) } else { Ok(None) }?;
        }
        let name = read_string(reader)?;

        Ok(Self {
            character_id,
            xp_cached,
            xp_tithed,
            flags,
            gender,
            heritage,
            rank,
            level,
            loyalty,
            leadership,
            allegiance_age,
            time_online,
            name,
        })
    }
}

impl crate::readers::ACDataType for FriendData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let friend_id = ObjectId::read(reader)?;
        let online = read_bool(reader)?;
        let appear_offline = read_bool(reader)?;
        let name = read_string(reader)?;
        let out_friends = read_packable_list::<ObjectId>(reader)?;
        let in_friends = read_packable_list::<ObjectId>(reader)?;

        Ok(Self {
            friend_id,
            online,
            appear_offline,
            name,
            out_friends,
            in_friends,
        })
    }
}

impl ItemProfileTypeNeg1 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, packed_amount: uint, object_id: ObjectId) -> Result<Self, Box<dyn std::error::Error>> {
        let weenie_description = PublicWeenieDesc::read(reader)?;

        Ok(Self {
            packed_amount,
            object_id,
            weenie_description,
        })
    }
}

impl ItemProfileType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, packed_amount: uint, object_id: ObjectId) -> Result<Self, Box<dyn std::error::Error>> {
        let old_weenie_description = OldPublicWeenieDesc::read(reader)?;

        Ok(Self {
            packed_amount,
            object_id,
            old_weenie_description,
        })
    }
}

impl ItemProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let packed_amount = read_u32(reader)?;
        #[allow(unused_variables)]
        let amount = (packed_amount & 0x_ffffff) as i32;
        let pwd_type = (packed_amount >> 24) as i32;
        let object_id = ObjectId::read(reader)?;

        match pwd_type {
            -1 => {
                let variant_struct = ItemProfileTypeNeg1::read(reader, packed_amount, object_id)?;
                Ok(Self::TypeNeg1(variant_struct))
            },
            0x01 => {
                let variant_struct = ItemProfileType1::read(reader, packed_amount, object_id)?;
                Ok(Self::Type1(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "pwd_type", pwd_type).into()),
        }
    }
}

impl crate::readers::ACDataType for ItemProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemProfile::read(reader)
    }
}

impl crate::readers::ACDataType for PublicWeenieDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let header = read_u32(reader)?;
        let name = read_string(reader)?;
        let weenie_class_id = PackedDWORD::read(reader)?;
        let icon = PackedDWORD::read(reader)?;
        let type_ = Ok::<_, Box<dyn std::error::Error>>(ItemType::from_bits_retain(read_u32(reader)?))?;
        let behavior = Ok::<_, Box<dyn std::error::Error>>(ObjectDescriptionFlag::from_bits_retain(read_u32(reader)?))?;
        let _ = align_dword(reader)?;
        let mut header2 = None;
        if (behavior.bits() & 0x04000000) != 0 {
            header2 = Some(read_u32(reader)?);
        }
        let mut plural_name = None;
        if (header & 0x00000001) != 0 {
            plural_name = Some(read_string(reader)?);
        }
        let mut items_capacity = None;
        if (header & 0x00000002) != 0 {
            items_capacity = Some(read_u8(reader)?);
        }
        let mut container_capacity = None;
        if (header & 0x00000004) != 0 {
            container_capacity = Some(read_u8(reader)?);
        }
        let mut ammunition_type = None;
        if (header & 0x00000100) != 0 {
            ammunition_type = Some(Ok::<_, Box<dyn std::error::Error>>(AmmoType::from_bits_retain(read_u16(reader)?))?);
        }
        let mut value = None;
        if (header & 0x00000008) != 0 {
            value = Some(read_u32(reader)?);
        }
        let mut useability = None;
        if (header & 0x00000010) != 0 {
            useability = Some(Ok::<_, Box<dyn std::error::Error>>(Usable::from_bits_retain(read_u32(reader)?))?);
        }
        let mut use_radius = None;
        if (header & 0x00000020) != 0 {
            use_radius = Some(read_f32(reader)?);
        }
        let mut target_type = None;
        if (header & 0x00080000) != 0 {
            target_type = Some(Ok::<_, Box<dyn std::error::Error>>(ItemType::from_bits_retain(read_u32(reader)?))?);
        }
        let mut effects = None;
        if (header & 0x00000080) != 0 {
            effects = Some(Ok::<_, Box<dyn std::error::Error>>(IconHighlight::from_bits_retain(read_u32(reader)?))?);
        }
        let mut combat_use = None;
        if (header & 0x00000200) != 0 {
            combat_use = Some(WieldType::try_from(read_u8(reader)?)?);
        }
        let mut structure = None;
        if (header & 0x00000400) != 0 {
            structure = Some(read_u16(reader)?);
        }
        let mut max_structure = None;
        if (header & 0x00000800) != 0 {
            max_structure = Some(read_u16(reader)?);
        }
        let mut stack_size = None;
        if (header & 0x00001000) != 0 {
            stack_size = Some(read_u16(reader)?);
        }
        let mut max_stack_size = None;
        if (header & 0x00002000) != 0 {
            max_stack_size = Some(read_u16(reader)?);
        }
        let mut container_id = None;
        if (header & 0x00004000) != 0 {
            container_id = Some(ObjectId::read(reader)?);
        }
        let mut wielder_id = None;
        if (header & 0x00008000) != 0 {
            wielder_id = Some(ObjectId::read(reader)?);
        }
        let mut valid_slots = None;
        if (header & 0x00010000) != 0 {
            valid_slots = Some(Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?);
        }
        let mut slot = None;
        if (header & 0x00020000) != 0 {
            slot = Some(Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?);
        }
        let mut priority = None;
        if (header & 0x00040000) != 0 {
            priority = Some(Ok::<_, Box<dyn std::error::Error>>(CoverageMask::from_bits_retain(read_u32(reader)?))?);
        }
        let mut blip_color = None;
        if (header & 0x00100000) != 0 {
            blip_color = Some(RadarColor::try_from(read_u8(reader)?)?);
        }
        let mut radar_enum = None;
        if (header & 0x00800000) != 0 {
            radar_enum = Some(RadarBehavior::try_from(read_u8(reader)?)?);
        }
        let mut physics_script = None;
        if (header & 0x08000000) != 0 {
            physics_script = Some(read_u16(reader)?);
        }
        let mut workmanship = None;
        if (header & 0x01000000) != 0 {
            workmanship = Some(read_f32(reader)?);
        }
        let mut burden = None;
        if (header & 0x00200000) != 0 {
            burden = Some(read_u16(reader)?);
        }
        let mut spell_id = None;
        if (header & 0x00400000) != 0 {
            spell_id = Some(SpellId::read(reader)?);
        }
        let mut owner_id = None;
        if (header & 0x02000000) != 0 {
            owner_id = Some(ObjectId::read(reader)?);
        }
        let mut restrictions = None;
        if (header & 0x04000000) != 0 {
            restrictions = Some(RestrictionDB::read(reader)?);
        }
        let mut hook_item_types = None;
        if (header & 0x20000000) != 0 {
            hook_item_types = Some(Ok::<_, Box<dyn std::error::Error>>(HookType::from_bits_retain(read_u16(reader)?))?);
        }
        let mut monarch_id = None;
        if (header & 0x00000040) != 0 {
            monarch_id = Some(ObjectId::read(reader)?);
        }
        let mut hook_type = None;
        if (header & 0x10000000) != 0 {
            hook_type = Some(Ok::<_, Box<dyn std::error::Error>>(HookType::from_bits_retain(read_u16(reader)?))?);
        }
        let mut icon_overlay = None;
        if (header & 0x40000000) != 0 {
            icon_overlay = Some(PackedDWORD::read(reader)?);
        }
        let mut icon_underlay = None;
        if (header2.unwrap_or(0) & 0x00000001) != 0 {
            icon_underlay = Some(PackedDWORD::read(reader)?);
        }
        let mut material = None;
        if (header & 0x80000000) != 0 {
            material = Some(MaterialType::try_from(read_u32(reader)?)?);
        }
        let mut cooldown_id = None;
        if (header2.unwrap_or(0) & 0x00000002) != 0 {
            cooldown_id = Some(read_u32(reader)?);
        }
        let mut cooldown_duration = None;
        if (header2.unwrap_or(0) & 0x00000004) != 0 {
            cooldown_duration = Some(read_u64(reader)?);
        }
        let mut pet_owner_id = None;
        if (header2.unwrap_or(0) & 0x00000008) != 0 {
            pet_owner_id = Some(ObjectId::read(reader)?);
        }
        let _ = align_dword(reader)?;

        Ok(Self {
            header,
            name,
            weenie_class_id,
            icon,
            type_,
            behavior,
            header2,
            plural_name,
            items_capacity,
            container_capacity,
            ammunition_type,
            value,
            useability,
            use_radius,
            target_type,
            effects,
            combat_use,
            structure,
            max_structure,
            stack_size,
            max_stack_size,
            container_id,
            wielder_id,
            valid_slots,
            slot,
            priority,
            blip_color,
            radar_enum,
            physics_script,
            workmanship,
            burden,
            spell_id,
            owner_id,
            restrictions,
            hook_item_types,
            monarch_id,
            hook_type,
            icon_overlay,
            icon_underlay,
            material,
            cooldown_id,
            cooldown_duration,
            pet_owner_id,
        })
    }
}

impl crate::readers::ACDataType for RestrictionDB {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let version = read_u32(reader)?;
        let flags = read_u32(reader)?;
        let monarch_id = ObjectId::read(reader)?;
        let permissions = read_phash_table::<ObjectId, u32>(reader)?;

        Ok(Self {
            version,
            flags,
            monarch_id,
            permissions,
        })
    }
}

impl crate::readers::ACDataType for OldPublicWeenieDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let header = read_u32(reader)?;
        let name = read_string(reader)?;
        let weenie_class_id = PackedDWORD::read(reader)?;
        let icon = PackedDWORD::read(reader)?;
        let type_ = Ok::<_, Box<dyn std::error::Error>>(ItemType::from_bits_retain(read_u32(reader)?))?;
        let bitfield = Ok::<_, Box<dyn std::error::Error>>(ObjectDescriptionFlag::from_bits_retain(read_u32(reader)?))?;
        let mut plural_name = None;
        if (header & 0x00000001) != 0 {
            plural_name = Some(read_string(reader)?);
        }
        let mut items_capacity = None;
        if (header & 0x00000002) != 0 {
            items_capacity = Some(read_u8(reader)?);
        }
        let mut container_capacity = None;
        if (header & 0x00000004) != 0 {
            container_capacity = Some(read_u8(reader)?);
        }
        let mut value = None;
        if (header & 0x00000008) != 0 {
            value = Some(read_u32(reader)?);
        }
        let mut useability = None;
        if (header & 0x00000010) != 0 {
            useability = Some(Ok::<_, Box<dyn std::error::Error>>(Usable::from_bits_retain(read_u32(reader)?))?);
        }
        let mut use_radius = None;
        if (header & 0x00000020) != 0 {
            use_radius = Some(read_f32(reader)?);
        }
        let mut t_target_type = None;
        if (header & 0x00080000) != 0 {
            t_target_type = Some(Ok::<_, Box<dyn std::error::Error>>(ItemType::from_bits_retain(read_u32(reader)?))?);
        }
        let mut effects = None;
        if (header & 0x00000080) != 0 {
            effects = Some(Ok::<_, Box<dyn std::error::Error>>(IconHighlight::from_bits_retain(read_u32(reader)?))?);
        }
        let mut ammunition_type = None;
        if (header & 0x00000100) != 0 {
            ammunition_type = Some(Ok::<_, Box<dyn std::error::Error>>(AmmoType::from_bits_retain(read_u16(reader)?))?);
        }
        let mut combat_use = None;
        if (header & 0x00000200) != 0 {
            combat_use = Some(WieldType::try_from(read_u8(reader)?)?);
        }
        let mut structure = None;
        if (header & 0x00000400) != 0 {
            structure = Some(read_u16(reader)?);
        }
        let mut max_structure = None;
        if (header & 0x00000800) != 0 {
            max_structure = Some(read_u16(reader)?);
        }
        let mut stack_size = None;
        if (header & 0x00001000) != 0 {
            stack_size = Some(read_u16(reader)?);
        }
        let mut max_stack_size = None;
        if (header & 0x00002000) != 0 {
            max_stack_size = Some(read_u16(reader)?);
        }
        let mut container_id = None;
        if (header & 0x00004000) != 0 {
            container_id = Some(ObjectId::read(reader)?);
        }
        let mut wielder_id = None;
        if (header & 0x00008000) != 0 {
            wielder_id = Some(ObjectId::read(reader)?);
        }
        let mut valid_slots = None;
        if (header & 0x00010000) != 0 {
            valid_slots = Some(Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?);
        }
        let mut slots = None;
        if (header & 0x00020000) != 0 {
            slots = Some(Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?);
        }
        let mut priority = None;
        if (header & 0x00040000) != 0 {
            priority = Some(Ok::<_, Box<dyn std::error::Error>>(CoverageMask::from_bits_retain(read_u32(reader)?))?);
        }
        let mut blip_color = None;
        if (header & 0x00100000) != 0 {
            blip_color = Some(RadarColor::try_from(read_u8(reader)?)?);
        }
        let mut radar_enum = None;
        if (header & 0x00800000) != 0 {
            radar_enum = Some(RadarBehavior::try_from(read_u8(reader)?)?);
        }
        let mut obvious_distance = None;
        if (header & 0x01000000) != 0 {
            obvious_distance = Some(read_f32(reader)?);
        }
        let mut vndwcid = None;
        if (header & 0x00200000) != 0 {
            vndwcid = Some(read_u16(reader)?);
        }
        let mut spell_id = None;
        if (header & 0x00400000) != 0 {
            spell_id = Some(SpellId::read(reader)?);
        }
        let mut house_owner_id = None;
        if (header & 0x02000000) != 0 {
            house_owner_id = Some(ObjectId::read(reader)?);
        }
        let mut physics_script = None;
        if (header & 0x08000000) != 0 {
            physics_script = Some(read_u16(reader)?);
        }
        let mut restrictions = None;
        if (header & 0x04000000) != 0 {
            restrictions = Some(RestrictionDB::read(reader)?);
        }
        let mut hook_type = None;
        if (header & 0x10000000) != 0 {
            hook_type = Some(Ok::<_, Box<dyn std::error::Error>>(HookType::from_bits_retain(read_u16(reader)?))?);
        }
        let mut hook_item_types = None;
        if (header & 0x20000000) != 0 {
            hook_item_types = Some(Ok::<_, Box<dyn std::error::Error>>(HookType::from_bits_retain(read_u16(reader)?))?);
        }
        let mut monarch_id = None;
        if (header & 0x00000040) != 0 {
            monarch_id = Some(ObjectId::read(reader)?);
        }
        let mut icon_overlay = None;
        if (header & 0x40000000) != 0 {
            icon_overlay = Some(PackedDWORD::read(reader)?);
        }
        let mut material = None;
        if (header & 0x80000000) != 0 {
            material = Some(MaterialType::try_from(read_u32(reader)?)?);
        }
        let _ = align_dword(reader)?;

        Ok(Self {
            header,
            name,
            weenie_class_id,
            icon,
            type_,
            bitfield,
            plural_name,
            items_capacity,
            container_capacity,
            value,
            useability,
            use_radius,
            t_target_type,
            effects,
            ammunition_type,
            combat_use,
            structure,
            max_structure,
            stack_size,
            max_stack_size,
            container_id,
            wielder_id,
            valid_slots,
            slots,
            priority,
            blip_color,
            radar_enum,
            obvious_distance,
            vndwcid,
            spell_id,
            house_owner_id,
            physics_script,
            restrictions,
            hook_type,
            hook_item_types,
            monarch_id,
            icon_overlay,
            material,
        })
    }
}

impl crate::readers::ACDataType for Trade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let partner_id = ObjectId::read(reader)?;
        let sequence = read_u64(reader)?;
        let status = read_u32(reader)?;
        let initiator_id = ObjectId::read(reader)?;
        let accepted = read_bool(reader)?;
        let partner_accepted = read_bool(reader)?;

        Ok(Self {
            partner_id,
            sequence,
            status,
            initiator_id,
            accepted,
            partner_accepted,
        })
    }
}

impl crate::readers::ACDataType for JumpPack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let extent = read_f32(reader)?;
        let velocity = Vector3::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let object_server_control_sequence = read_u16(reader)?;
        let object_teleport_sequence = read_u16(reader)?;
        let object_force_position_sequence = read_u16(reader)?;
        let _ = align_dword(reader)?;

        Ok(Self {
            extent,
            velocity,
            object_instance_sequence,
            object_server_control_sequence,
            object_teleport_sequence,
            object_force_position_sequence,
        })
    }
}

impl crate::readers::ACDataType for MoveToStatePack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let raw_motion_state = RawMotionState::read(reader)?;
        let position = Position::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let object_server_control_sequence = read_u16(reader)?;
        let object_teleport_sequence = read_u16(reader)?;
        let object_force_position_sequence = read_u16(reader)?;
        let contact = read_u8(reader)?;
        let _ = align_dword(reader)?;

        Ok(Self {
            raw_motion_state,
            position,
            object_instance_sequence,
            object_server_control_sequence,
            object_teleport_sequence,
            object_force_position_sequence,
            contact,
        })
    }
}

impl crate::readers::ACDataType for PackedMotionCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let command_id = Command::try_from(read_u16(reader)?)?;
        let packed_sequence = read_u16(reader)?;
        #[allow(unused_variables)]
        let server_action_sequence = (packed_sequence & 0x7fff) as u16;
        #[allow(unused_variables)]
        let autonomous = ((packed_sequence >> 15) & 0x1) as u16;
        let speed = read_f32(reader)?;

        Ok(Self {
            command_id,
            packed_sequence,
            speed,
        })
    }
}

impl crate::readers::ACDataType for RawMotionState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let command_list_length = ((flags >> 11) & 0x_f8) as u16;
        let mut current_holdkey = None;
        if (flags & 0x00000001) != 0 {
            current_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
        }
        let mut current_style = None;
        if (flags & 0x00000002) != 0 {
            current_style = Some(StanceMode::try_from(read_u16(reader)?)?);
        }
        let mut forward_command = None;
        if (flags & 0x00000004) != 0 {
            forward_command = Some(Command::try_from(read_u16(reader)?)?);
        }
        let mut forward_holdkey = None;
        if (flags & 0x0000008) != 0 {
            forward_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
        }
        let mut forward_speed = None;
        if (flags & 0x00000010) != 0 {
            forward_speed = Some(read_f32(reader)?);
        }
        let mut sidestep_command = None;
        if (flags & 0x00000020) != 0 {
            sidestep_command = Some(Command::try_from(read_u16(reader)?)?);
        }
        let mut sidestep_holdkey = None;
        if (flags & 0x00000040) != 0 {
            sidestep_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
        }
        let mut sidestep_speed = None;
        if (flags & 0x00000080) != 0 {
            sidestep_speed = Some(read_f32(reader)?);
        }
        let mut turn_command = None;
        if (flags & 0x00000100) != 0 {
            turn_command = Some(Command::try_from(read_u16(reader)?)?);
        }
        let mut turn_holdkey = None;
        if (flags & 0x00000200) != 0 {
            turn_holdkey = Some(read_u32(reader)?);
        }
        let mut turn_speed = None;
        if (flags & 0x00000400) != 0 {
            turn_speed = Some(read_f32(reader)?);
        }
        let commands = read_vec::<PackedMotionCommand>(reader, command_list_length as usize)?;

        Ok(Self {
            flags,
            current_holdkey,
            current_style,
            forward_command,
            forward_holdkey,
            forward_speed,
            sidestep_command,
            sidestep_holdkey,
            sidestep_speed,
            turn_command,
            turn_holdkey,
            turn_speed,
            commands,
        })
    }
}

impl crate::readers::ACDataType for AutonomousPositionPack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let position = Position::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let object_server_control_sequence = read_u16(reader)?;
        let object_teleport_sequence = read_u16(reader)?;
        let object_force_position_sequence = read_u16(reader)?;
        let contact = read_u8(reader)?;
        let _ = align_dword(reader)?;

        Ok(Self {
            position,
            object_instance_sequence,
            object_server_control_sequence,
            object_teleport_sequence,
            object_force_position_sequence,
            contact,
        })
    }
}

impl crate::readers::ACDataType for PositionPack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = Ok::<_, Box<dyn std::error::Error>>(PositionFlags::from_bits_retain(read_u32(reader)?))?;
        let origin = Origin::read(reader)?;
        let mut w_quat = None;
        if (flags.bits() & 0x00000008) != 0 {
            w_quat = Some(read_f32(reader)?);
        }
        let mut x_quat = None;
        if (flags.bits() & 0x00000010) != 0 {
            x_quat = Some(read_f32(reader)?);
        }
        let mut y_quat = None;
        if (flags.bits() & 0x00000020) != 0 {
            y_quat = Some(read_f32(reader)?);
        }
        let mut z_quat = None;
        if (flags.bits() & 0x00000040) != 0 {
            z_quat = Some(read_f32(reader)?);
        }
        let mut velocity = None;
        if (flags.bits() & 0x00000001) != 0 {
            velocity = Some(Vector3::read(reader)?);
        }
        let mut placement_id = None;
        if (flags.bits() & 0x00000002) != 0 {
            placement_id = Some(read_u32(reader)?);
        }
        let object_instance_sequence = read_u16(reader)?;
        let object_position_sequence = read_u16(reader)?;
        let object_teleport_sequence = read_u16(reader)?;
        let object_force_position_sequence = read_u16(reader)?;

        Ok(Self {
            flags,
            origin,
            w_quat,
            x_quat,
            y_quat,
            z_quat,
            velocity,
            placement_id,
            object_instance_sequence,
            object_position_sequence,
            object_teleport_sequence,
            object_force_position_sequence,
        })
    }
}

impl MovementDataType0 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, object_movement_sequence: ushort, object_server_control_sequence: ushort, autonomous: ushort, option_flags: MovementOption, stance: StanceMode) -> Result<Self, Box<dyn std::error::Error>> {
        let state = InterpretedMotionState::read(reader)?;
        let sticky_object = if (option_flags.clone() as u32 & 0x01) != 0 { ObjectId::read(reader).map(Some) } else { Ok(None) }?;

        Ok(Self {
            object_movement_sequence,
            object_server_control_sequence,
            autonomous,
            option_flags,
            stance,
            state,
            sticky_object,
        })
    }
}

impl MovementDataType6 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, object_movement_sequence: ushort, object_server_control_sequence: ushort, autonomous: ushort, option_flags: MovementOption, stance: StanceMode) -> Result<Self, Box<dyn std::error::Error>> {
        let target = ObjectId::read(reader)?;
        let origin = Origin::read(reader)?;
        let move_to_params = MoveToMovementParameters::read(reader)?;
        let my_run_rate = read_f32(reader)?;

        Ok(Self {
            object_movement_sequence,
            object_server_control_sequence,
            autonomous,
            option_flags,
            stance,
            target,
            origin,
            move_to_params,
            my_run_rate,
        })
    }
}

impl MovementDataType7 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, object_movement_sequence: ushort, object_server_control_sequence: ushort, autonomous: ushort, option_flags: MovementOption, stance: StanceMode) -> Result<Self, Box<dyn std::error::Error>> {
        let origin = Origin::read(reader)?;
        let move_to_params = MoveToMovementParameters::read(reader)?;
        let my_run_rate = read_f32(reader)?;

        Ok(Self {
            object_movement_sequence,
            object_server_control_sequence,
            autonomous,
            option_flags,
            stance,
            origin,
            move_to_params,
            my_run_rate,
        })
    }
}

impl MovementDataType8 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, object_movement_sequence: ushort, object_server_control_sequence: ushort, autonomous: ushort, option_flags: MovementOption, stance: StanceMode) -> Result<Self, Box<dyn std::error::Error>> {
        let target_id = ObjectId::read(reader)?;
        let desired_heading = read_f32(reader)?;
        let turn_to_params = TurnToMovementParameters::read(reader)?;

        Ok(Self {
            object_movement_sequence,
            object_server_control_sequence,
            autonomous,
            option_flags,
            stance,
            target_id,
            desired_heading,
            turn_to_params,
        })
    }
}

impl MovementDataType9 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, object_movement_sequence: ushort, object_server_control_sequence: ushort, autonomous: ushort, option_flags: MovementOption, stance: StanceMode) -> Result<Self, Box<dyn std::error::Error>> {
        let turn_to_params = TurnToMovementParameters::read(reader)?;

        Ok(Self {
            object_movement_sequence,
            object_server_control_sequence,
            autonomous,
            option_flags,
            stance,
            turn_to_params,
        })
    }
}

impl MovementData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_movement_sequence = read_u16(reader)?;
        let object_server_control_sequence = read_u16(reader)?;
        let autonomous = read_u16(reader)?;
        let movement_type = MovementType::try_from(read_u8(reader)?)?;
        let option_flags = MovementOption::try_from(read_u8(reader)?)?;
        let stance = StanceMode::try_from(read_u16(reader)?)?;

        match movement_type {
            MovementType::InterpretedMotionState => {
                let variant_struct = MovementDataType0::read(reader, object_movement_sequence, object_server_control_sequence, autonomous, option_flags, stance)?;
                Ok(Self::Type0(variant_struct))
            },
            MovementType::MoveToObject => {
                let variant_struct = MovementDataType6::read(reader, object_movement_sequence, object_server_control_sequence, autonomous, option_flags, stance)?;
                Ok(Self::Type6(variant_struct))
            },
            MovementType::MoveToPosition => {
                let variant_struct = MovementDataType7::read(reader, object_movement_sequence, object_server_control_sequence, autonomous, option_flags, stance)?;
                Ok(Self::Type7(variant_struct))
            },
            MovementType::TurnToObject => {
                let variant_struct = MovementDataType8::read(reader, object_movement_sequence, object_server_control_sequence, autonomous, option_flags, stance)?;
                Ok(Self::Type8(variant_struct))
            },
            MovementType::TurnToPosition => {
                let variant_struct = MovementDataType9::read(reader, object_movement_sequence, object_server_control_sequence, autonomous, option_flags, stance)?;
                Ok(Self::Type9(variant_struct))
            },
        }
    }
}

impl crate::readers::ACDataType for MovementData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementData::read(reader)
    }
}

impl crate::readers::ACDataType for InterpretedMotionState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let command_list_length = ((flags >> 7) & 0x7f) as u32;
        let mut current_style = None;
        if (flags & 0x00000001) != 0 {
            current_style = Some(StanceMode::try_from(read_u16(reader)?)?);
        }
        let mut forward_command = None;
        if (flags & 0x00000002) != 0 {
            forward_command = Some(Command::try_from(read_u16(reader)?)?);
        }
        let mut sidestep_command = None;
        if (flags & 0x00000008) != 0 {
            sidestep_command = Some(Command::try_from(read_u16(reader)?)?);
        }
        let mut turn_command = None;
        if (flags & 0x00000020) != 0 {
            turn_command = Some(Command::try_from(read_u16(reader)?)?);
        }
        let mut forward_speed = None;
        if (flags & 0x00000004) != 0 {
            forward_speed = Some(read_f32(reader)?);
        }
        let mut sidestep_speed = None;
        if (flags & 0x00000010) != 0 {
            sidestep_speed = Some(read_f32(reader)?);
        }
        let mut turn_speed = None;
        if (flags & 0x00000040) != 0 {
            turn_speed = Some(read_f32(reader)?);
        }
        let commands = read_vec::<PackedMotionCommand>(reader, command_list_length as usize)?;
        let _ = align_dword(reader)?;

        Ok(Self {
            flags,
            current_style,
            forward_command,
            sidestep_command,
            turn_command,
            forward_speed,
            sidestep_speed,
            turn_speed,
            commands,
        })
    }
}

impl crate::readers::ACDataType for DDDRevision {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let id_dat_file = read_u64(reader)?;
        #[allow(unused_variables)]
        let dat_file_type = (id_dat_file >> 32) as u32;
        let iteration = read_u32(reader)?;
        let ids_to_download = read_packable_list::<DataId>(reader)?;
        let ids_to_purge = read_packable_list::<DataId>(reader)?;

        Ok(Self {
            id_dat_file,
            iteration,
            ids_to_download,
            ids_to_purge,
        })
    }
}

impl crate::readers::ACDataType for MoveToMovementParameters {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let bitmember = read_u32(reader)?;
        let distance_to_object = read_f32(reader)?;
        let min_distance = read_f32(reader)?;
        let fail_distance = read_f32(reader)?;
        let animation_speed = read_f32(reader)?;
        let walk_run_threshold = read_f32(reader)?;
        let desired_heading = read_f32(reader)?;

        Ok(Self {
            bitmember,
            distance_to_object,
            min_distance,
            fail_distance,
            animation_speed,
            walk_run_threshold,
            desired_heading,
        })
    }
}

impl crate::readers::ACDataType for TurnToMovementParameters {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let bitmember = read_u32(reader)?;
        let animation_speed = read_f32(reader)?;
        let desired_heading = read_f32(reader)?;

        Ok(Self {
            bitmember,
            animation_speed,
            desired_heading,
        })
    }
}

impl crate::readers::ACDataType for ObjDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let version = read_u8(reader)?;
        let palette_count = read_u8(reader)?;
        let texture_count = read_u8(reader)?;
        let model_count = read_u8(reader)?;
        let mut palette = None;
        if palette_count > 0 {
            palette = if palette_count > 0 { DataId::read(reader).map(Some) } else { Ok(None) }?;
        }
        let subpalettes = read_vec::<Subpalette>(reader, palette_count as usize)?;
        let tm_changes = read_vec::<TextureMapChange>(reader, texture_count as usize)?;
        let ap_changes = read_vec::<AnimPartChange>(reader, model_count as usize)?;
        let _ = align_dword(reader)?;

        Ok(Self {
            version,
            palette_count,
            texture_count,
            model_count,
            palette,
            subpalettes,
            tm_changes,
            ap_changes,
        })
    }
}

impl crate::readers::ACDataType for Subpalette {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let palette = DataId::read(reader)?;
        let offset = read_u8(reader)?;
        let num_colors = read_u8(reader)?;

        Ok(Self {
            palette,
            offset,
            num_colors,
        })
    }
}

impl crate::readers::ACDataType for TextureMapChange {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let part_index = read_u8(reader)?;
        let old_tex_id = DataId::read(reader)?;
        let new_tex_id = DataId::read(reader)?;

        Ok(Self {
            part_index,
            old_tex_id,
            new_tex_id,
        })
    }
}

impl crate::readers::ACDataType for AnimPartChange {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let part_index = read_u8(reader)?;
        let part_id = DataId::read(reader)?;

        Ok(Self {
            part_index,
            part_id,
        })
    }
}

impl crate::readers::ACDataType for CharGenResult {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let account = read_string(reader)?;
        let one = read_u32(reader)?;
        let heritage_group = HeritageGroup::try_from(read_u8(reader)?)?;
        let gender = Gender::try_from(read_u8(reader)?)?;
        let eyes_strip = read_u32(reader)?;
        let nose_strip = read_u32(reader)?;
        let mouth_strip = read_u32(reader)?;
        let hair_color = read_u32(reader)?;
        let eye_color = read_u32(reader)?;
        let hair_style = read_u32(reader)?;
        let headgear_style = read_u32(reader)?;
        let headgear_color = read_u32(reader)?;
        let shirt_style = read_u32(reader)?;
        let shirt_color = read_u32(reader)?;
        let trousers_style = read_u32(reader)?;
        let trousers_color = read_u32(reader)?;
        let footwear_style = read_u32(reader)?;
        let footwear_color = read_u32(reader)?;
        let skin_shade = read_u64(reader)?;
        let hair_shade = read_u64(reader)?;
        let headgear_shade = read_u64(reader)?;
        let shirt_shade = read_u64(reader)?;
        let trousers_shade = read_u64(reader)?;
        let tootwear_shade = read_u64(reader)?;
        let template_num = read_u32(reader)?;
        let strength = read_u32(reader)?;
        let endurance = read_u32(reader)?;
        let coordination = read_u32(reader)?;
        let quickness = read_u32(reader)?;
        let focus = read_u32(reader)?;
        let self_ = read_u32(reader)?;
        let slot = read_u32(reader)?;
        let class_id = read_u32(reader)?;
        let skills = read_packable_list::<SkillAdvancementClass>(reader)?;
        let name = read_string(reader)?;
        let start_area = read_u32(reader)?;
        let is_admin = read_u32(reader)?;
        let is_envoy = read_u32(reader)?;
        let validation = read_u32(reader)?;

        Ok(Self {
            account,
            one,
            heritage_group,
            gender,
            eyes_strip,
            nose_strip,
            mouth_strip,
            hair_color,
            eye_color,
            hair_style,
            headgear_style,
            headgear_color,
            shirt_style,
            shirt_color,
            trousers_style,
            trousers_color,
            footwear_style,
            footwear_color,
            skin_shade,
            hair_shade,
            headgear_shade,
            shirt_shade,
            trousers_shade,
            tootwear_shade,
            template_num,
            strength,
            endurance,
            coordination,
            quickness,
            focus,
            self_,
            slot,
            class_id,
            skills,
            name,
            start_area,
            is_admin,
            is_envoy,
            validation,
        })
    }
}

impl crate::readers::ACDataType for CharacterIdentity {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;
        let name = read_string(reader)?;
        let seconds_greyed_out = read_u32(reader)?;
        let _ = align_dword(reader)?;

        Ok(Self {
            character_id,
            name,
            seconds_greyed_out,
        })
    }
}

impl crate::readers::ACDataType for EquipLocation {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let slot = Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?;

        Ok(Self {
            object_id,
            slot,
        })
    }
}

impl crate::readers::ACDataType for PhysicsDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let state = Ok::<_, Box<dyn std::error::Error>>(PhysicsState::from_bits_retain(read_u32(reader)?))?;
        let mut movement_buffer = None;
        let mut autonomous = None;
        if (flags & 0x00010000) != 0 {
            movement_buffer = Some(read_packable_list::<u8>(reader)?);
            autonomous = Some(read_bool(reader)?);
        }
        let mut animation_frame = None;
        if (flags & 0x00020000) != 0 {
            animation_frame = Some(read_u32(reader)?);
        }
        let mut position = None;
        if (flags & 0x00008000) != 0 {
            position = Some(Position::read(reader)?);
        }
        let mut motion_id = None;
        if (flags & 0x00000002) != 0 {
            motion_id = Some(DataId::read(reader)?);
        }
        let mut sound_id = None;
        if (flags & 0x00000800) != 0 {
            sound_id = Some(DataId::read(reader)?);
        }
        let mut physics_script_id = None;
        if (flags & 0x00001000) != 0 {
            physics_script_id = Some(DataId::read(reader)?);
        }
        let mut setup_id = None;
        if (flags & 0x00000001) != 0 {
            setup_id = Some(DataId::read(reader)?);
        }
        let mut parent_id = None;
        let mut parent_location = None;
        if (flags & 0x00000020) != 0 {
            parent_id = Some(ObjectId::read(reader)?);
            parent_location = Some(ParentLocation::try_from(read_u32(reader)?)?);
        }
        let mut children = None;
        if (flags & 0x00000040) != 0 {
            children = Some(read_packable_list::<EquipLocation>(reader)?);
        }
        let mut scale = None;
        if (flags & 0x00000080) != 0 {
            scale = Some(read_f32(reader)?);
        }
        let mut friction = None;
        if (flags & 0x00000100) != 0 {
            friction = Some(read_f32(reader)?);
        }
        let mut elasticity = None;
        if (flags & 0x00000200) != 0 {
            elasticity = Some(read_f32(reader)?);
        }
        let mut translucency = None;
        if (flags & 0x00040000) != 0 {
            translucency = Some(read_f32(reader)?);
        }
        let mut velocity = None;
        if (flags & 0x00000004) != 0 {
            velocity = Some(Vector3::read(reader)?);
        }
        let mut acceleration = None;
        if (flags & 0x00000008) != 0 {
            acceleration = Some(Vector3::read(reader)?);
        }
        let mut omega = None;
        if (flags & 0x00000010) != 0 {
            omega = Some(Vector3::read(reader)?);
        }
        let mut default_script = None;
        if (flags & 0x00002000) != 0 {
            default_script = Some(read_u32(reader)?);
        }
        let mut default_script_intensity = None;
        if (flags & 0x00004000) != 0 {
            default_script_intensity = Some(read_f32(reader)?);
        }
        let object_position_sequence = read_u16(reader)?;
        let object_movement_sequence = read_u16(reader)?;
        let object_state_sequence = read_u16(reader)?;
        let object_vector_sequence = read_u16(reader)?;
        let object_teleport_sequence = read_u16(reader)?;
        let object_server_control_sequence = read_u16(reader)?;
        let object_force_position_sequence = read_u16(reader)?;
        let object_visual_desc_sequence = read_u16(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let _ = align_dword(reader)?;

        Ok(Self {
            flags,
            state,
            movement_buffer,
            autonomous,
            animation_frame,
            position,
            motion_id,
            sound_id,
            physics_script_id,
            setup_id,
            parent_id,
            parent_location,
            children,
            scale,
            friction,
            elasticity,
            translucency,
            velocity,
            acceleration,
            omega,
            default_script,
            default_script_intensity,
            object_position_sequence,
            object_movement_sequence,
            object_state_sequence,
            object_vector_sequence,
            object_teleport_sequence,
            object_server_control_sequence,
            object_force_position_sequence,
            object_visual_desc_sequence,
            object_instance_sequence,
        })
    }
}

impl crate::readers::ACDataType for AdminAccountData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let account_name = read_string(reader)?;
        let bookie_id = read_u32(reader)?;

        Ok(Self {
            account_name,
            bookie_id,
        })
    }
}

impl crate::readers::ACDataType for AdminPlayerData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let name = read_string(reader)?;
        let bookie_id = read_u32(reader)?;

        Ok(Self {
            name,
            bookie_id,
        })
    }
}

impl crate::readers::ACDataType for VendorProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let categories = Ok::<_, Box<dyn std::error::Error>>(ItemType::from_bits_retain(read_u32(reader)?))?;
        let min_value = read_u32(reader)?;
        let max_value = read_u32(reader)?;
        let deals_magic = read_bool(reader)?;
        let buy_price = read_f32(reader)?;
        let sell_price = read_f32(reader)?;
        let currency_id = read_u32(reader)?;
        let currency_amount = read_u32(reader)?;
        let currency_name = read_string(reader)?;

        Ok(Self {
            categories,
            min_value,
            max_value,
            deals_magic,
            buy_price,
            sell_price,
            currency_id,
            currency_amount,
            currency_name,
        })
    }
}

impl crate::readers::ACDataType for ArmorProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let prot_slashing = read_f32(reader)?;
        let prot_piercing = read_f32(reader)?;
        let prot_bludgeoning = read_f32(reader)?;
        let prot_cold = read_f32(reader)?;
        let prot_fire = read_f32(reader)?;
        let prot_acid = read_f32(reader)?;
        let prot_nether = read_f32(reader)?;
        let prot_lightning = read_f32(reader)?;

        Ok(Self {
            prot_slashing,
            prot_piercing,
            prot_bludgeoning,
            prot_cold,
            prot_fire,
            prot_acid,
            prot_nether,
            prot_lightning,
        })
    }
}

impl crate::readers::ACDataType for CreatureAppraisalProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let health = read_u32(reader)?;
        let health_max = read_u32(reader)?;
        let mut strength = None;
        let mut endurance = None;
        let mut quickness = None;
        let mut coordination = None;
        let mut focus = None;
        let mut self_ = None;
        let mut stamina = None;
        let mut mana = None;
        let mut stamina_max = None;
        let mut mana_max = None;
        if (flags & 0x00000008) != 0 {
            strength = Some(read_u32(reader)?);
            endurance = Some(read_u32(reader)?);
            quickness = Some(read_u32(reader)?);
            coordination = Some(read_u32(reader)?);
            focus = Some(read_u32(reader)?);
            self_ = Some(read_u32(reader)?);
            stamina = Some(read_u32(reader)?);
            mana = Some(read_u32(reader)?);
            stamina_max = Some(read_u32(reader)?);
            mana_max = Some(read_u32(reader)?);
        }
        let mut attr_highlight = None;
        let mut attr_color = None;
        if (flags & 0x00000001) != 0 {
            attr_highlight = Some(Ok::<_, Box<dyn std::error::Error>>(AttributeMask::from_bits_retain(read_u16(reader)?))?);
            attr_color = Some(Ok::<_, Box<dyn std::error::Error>>(AttributeMask::from_bits_retain(read_u16(reader)?))?);
        }

        Ok(Self {
            flags,
            health,
            health_max,
            strength,
            endurance,
            quickness,
            coordination,
            focus,
            self_,
            stamina,
            mana,
            stamina_max,
            mana_max,
            attr_highlight,
            attr_color,
        })
    }
}

impl crate::readers::ACDataType for WeaponProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let damage_type = Ok::<_, Box<dyn std::error::Error>>(DamageType::from_bits_retain(read_u32(reader)?))?;
        let speed = read_u32(reader)?;
        let skill = SkillId::try_from(read_i32(reader)?)?;
        let damage = read_u32(reader)?;
        let variance = read_f64(reader)?;
        let modifier = read_f64(reader)?;
        let length = read_f64(reader)?;
        let max_velocity = read_f64(reader)?;
        let offsense = read_f64(reader)?;
        let max_velocity_estimated = read_u32(reader)?;

        Ok(Self {
            damage_type,
            speed,
            skill,
            damage,
            variance,
            modifier,
            length,
            max_velocity,
            offsense,
            max_velocity_estimated,
        })
    }
}

impl crate::readers::ACDataType for HookAppraisalProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = Ok::<_, Box<dyn std::error::Error>>(HookAppraisalFlags::from_bits_retain(read_u32(reader)?))?;
        let valid_locations = Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?;
        let ammo_type = Ok::<_, Box<dyn std::error::Error>>(AmmoType::from_bits_retain(read_u16(reader)?))?;

        Ok(Self {
            flags,
            valid_locations,
            ammo_type,
        })
    }
}

impl crate::readers::ACDataType for SquelchDB {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let account_hash = read_packable_hash_table::<String, u32>(reader)?;
        let character_hash = read_packable_hash_table::<ObjectId, SquelchInfo>(reader)?;
        let global_info = SquelchInfo::read(reader)?;

        Ok(Self {
            account_hash,
            character_hash,
            global_info,
        })
    }
}

impl crate::readers::ACDataType for SquelchInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let filters = read_packable_list::<LogTextType>(reader)?;
        let name = read_string(reader)?;
        let account = read_bool(reader)?;

        Ok(Self {
            filters,
            name,
            account,
        })
    }
}

impl crate::readers::ACDataType for HouseProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let dwelling_id = read_u32(reader)?;
        let owner_id = ObjectId::read(reader)?;
        let flags = HouseBitfield::try_from(read_u32(reader)?)?;
        let min_level = read_i32(reader)?;
        let max_level = read_i32(reader)?;
        let min_alleg_rank = read_i32(reader)?;
        let max_alleg_rank = read_i32(reader)?;
        let maintenance_free = read_bool(reader)?;
        let type_ = HouseType::try_from(read_u32(reader)?)?;
        let owner_name = read_string(reader)?;
        let buy = read_packable_list::<HousePayment>(reader)?;
        let rent = read_packable_list::<HousePayment>(reader)?;

        Ok(Self {
            dwelling_id,
            owner_id,
            flags,
            min_level,
            max_level,
            min_alleg_rank,
            max_alleg_rank,
            maintenance_free,
            type_,
            owner_name,
            buy,
            rent,
        })
    }
}

impl crate::readers::ACDataType for HousePayment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let required = read_u32(reader)?;
        let paid = read_u32(reader)?;
        let weenie_class_id = read_u32(reader)?;
        let name = read_string(reader)?;
        let plural_name = read_string(reader)?;

        Ok(Self {
            required,
            paid,
            weenie_class_id,
            name,
            plural_name,
        })
    }
}

impl crate::readers::ACDataType for HouseData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let buy_time = read_u32(reader)?;
        let rent_time = read_u32(reader)?;
        let type_ = HouseType::try_from(read_u32(reader)?)?;
        let maintenance_free = read_bool(reader)?;
        let buy = read_packable_list::<HousePayment>(reader)?;
        let rent = read_packable_list::<HousePayment>(reader)?;
        let position = Position::read(reader)?;

        Ok(Self {
            buy_time,
            rent_time,
            type_,
            maintenance_free,
            buy,
            rent,
            position,
        })
    }
}

impl crate::readers::ACDataType for HAR {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let version = read_u32(reader)?;
        let bitmask = read_u32(reader)?;
        let monarch_id = ObjectId::read(reader)?;
        let guest_list = read_packable_hash_table::<ObjectId, GuestInfo>(reader)?;
        let roommate_list = read_packable_list::<ObjectId>(reader)?;

        Ok(Self {
            version,
            bitmask,
            monarch_id,
            guest_list,
            roommate_list,
        })
    }
}

impl crate::readers::ACDataType for GuestInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let has_storage_permission = read_bool(reader)?;
        let guest_name = read_string(reader)?;

        Ok(Self {
            has_storage_permission,
            guest_name,
        })
    }
}

impl GameMoveDataType4 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, player_id: ObjectId, team: int) -> Result<Self, Box<dyn std::error::Error>> {
        let id_piece_to_move = read_i32(reader)?;
        let y_grid = read_i32(reader)?;

        Ok(Self {
            player_id,
            team,
            id_piece_to_move,
            y_grid,
        })
    }
}

impl GameMoveDataType5 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, player_id: ObjectId, team: int) -> Result<Self, Box<dyn std::error::Error>> {
        let id_piece_to_move = read_i32(reader)?;
        let y_grid = read_i32(reader)?;
        let x_to = read_i32(reader)?;
        let y_to = read_i32(reader)?;

        Ok(Self {
            player_id,
            team,
            id_piece_to_move,
            y_grid,
            x_to,
            y_to,
        })
    }
}

impl GameMoveDataType6 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, player_id: ObjectId, team: int) -> Result<Self, Box<dyn std::error::Error>> {
        let id_piece_to_move = read_i32(reader)?;

        Ok(Self {
            player_id,
            team,
            id_piece_to_move,
        })
    }
}

impl GameMoveData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = read_i32(reader)?;
        let player_id = ObjectId::read(reader)?;
        let team = read_i32(reader)?;

        match type_ {
            0x04 => {
                let variant_struct = GameMoveDataType4::read(reader, player_id, team)?;
                Ok(Self::Type4(variant_struct))
            },
            0x05 => {
                let variant_struct = GameMoveDataType5::read(reader, player_id, team)?;
                Ok(Self::Type5(variant_struct))
            },
            0x06 => {
                let variant_struct = GameMoveDataType6::read(reader, player_id, team)?;
                Ok(Self::Type6(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "type_", type_).into()),
        }
    }
}

impl crate::readers::ACDataType for GameMoveData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameMoveData::read(reader)
    }
}

impl crate::readers::ACDataType for SalvageOperationsResultData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let skill_used = SkillId::try_from(read_i32(reader)?)?;
        let not_salvagable = read_packable_list::<ObjectId>(reader)?;
        let salvage_results = read_packable_list::<SalvageResult>(reader)?;
        let aug_bonus = read_i32(reader)?;

        Ok(Self {
            skill_used,
            not_salvagable,
            salvage_results,
            aug_bonus,
        })
    }
}

impl crate::readers::ACDataType for SalvageResult {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let material = MaterialType::try_from(read_u32(reader)?)?;
        let workmanship = read_f64(reader)?;
        let units = read_u32(reader)?;

        Ok(Self {
            material,
            workmanship,
            units,
        })
    }
}

impl crate::readers::ACDataType for FellowshipLockData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown1 = read_u32(reader)?;
        let unknown2 = read_u32(reader)?;
        let unknown3 = read_u32(reader)?;
        let timestamp = read_u32(reader)?;
        let sequence = read_u32(reader)?;

        Ok(Self {
            unknown1,
            unknown2,
            unknown3,
            timestamp,
            sequence,
        })
    }
}

impl crate::readers::ACDataType for Fellowship {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let members = read_packable_hash_table::<ObjectId, Fellow>(reader)?;
        let name = read_string(reader)?;
        let leader_id = ObjectId::read(reader)?;
        let share_xp = read_bool(reader)?;
        let even_xp_split = read_bool(reader)?;
        let open = read_bool(reader)?;
        let locked = read_bool(reader)?;
        let recently_departed = read_packable_hash_table::<ObjectId, i32>(reader)?;
        let locks = read_packable_hash_table::<String, FellowshipLockData>(reader)?;

        Ok(Self {
            members,
            name,
            leader_id,
            share_xp,
            even_xp_split,
            open,
            locked,
            recently_departed,
            locks,
        })
    }
}

impl crate::readers::ACDataType for Fellow {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let xp_cached = read_u32(reader)?;
        let lum_cached = read_u32(reader)?;
        let level = read_u32(reader)?;
        let max_health = read_u32(reader)?;
        let max_stamina = read_u32(reader)?;
        let max_mana = read_u32(reader)?;
        let current_health = read_u32(reader)?;
        let current_stamina = read_u32(reader)?;
        let current_mana = read_u32(reader)?;
        let share_loot = read_bool(reader)?;
        let name = read_string(reader)?;

        Ok(Self {
            xp_cached,
            lum_cached,
            level,
            max_health,
            max_stamina,
            max_mana,
            current_health,
            current_stamina,
            current_mana,
            share_loot,
            name,
        })
    }
}

impl crate::readers::ACDataType for ContractTracker {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let version = read_u32(reader)?;
        let contract_id = ContractId::try_from(read_u32(reader)?)?;
        let contract_stage = ContractStage::try_from(read_u32(reader)?)?;
        let time_when_done = read_i64(reader)?;
        let time_when_repeats = read_i64(reader)?;

        Ok(Self {
            version,
            contract_id,
            contract_stage,
            time_when_done,
            time_when_repeats,
        })
    }
}

impl crate::readers::ACDataType for ContractTrackerTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contact_trackers = read_packable_hash_table::<u32, ContractTracker>(reader)?;

        Ok(Self {
            contact_trackers,
        })
    }
}

