use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
use crate::enums::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DataId(pub u32);

impl DataId {
    pub fn read(reader: &mut dyn crate::readers::ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(crate::readers::read_u32(reader)?))
    }
}

impl crate::readers::ACDataType for DataId {
    fn read(reader: &mut dyn crate::readers::ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DataId::read(reader)
    }
}

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

impl WString {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        write_wstring(writer, &self.0)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for WString {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WString::write(self, writer)
    }
}

impl PackedWORD {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        crate::readers::read_packed_word(reader)?;
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for PackedWORD {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PackedWORD::read(reader)
    }
}

impl PackedWORD {
    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl crate::writers::ACWritable for PackedWORD {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        PackedWORD::write(self, writer)
    }
}

impl PackedDWORD {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        crate::readers::read_packed_dword(reader)?;
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for PackedDWORD {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PackedDWORD::read(reader)
    }
}

impl PackedDWORD {
    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl crate::writers::ACWritable for PackedDWORD {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        PackedDWORD::write(self, writer)
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

impl ObjectId {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        write_u32(writer, self.0)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for ObjectId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        ObjectId::write(self, writer)
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

impl LandcellId {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        write_u32(writer, self.0)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for LandcellId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        LandcellId::write(self, writer)
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

impl SpellId {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        write_u16(writer, self.0)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for SpellId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        SpellId::write(self, writer)
    }
}

impl DataId {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        write_u32(writer, self.0)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for DataId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        DataId::write(self, writer)
    }
}

impl crate::readers::ACDataType for LayeredSpellId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "LayeredSpellId").entered();

        #[cfg(feature = "tracing")]
        let _field_span_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Id", position = pos).entered()
        };
        let id = SpellId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_id);
        #[cfg(feature = "tracing")]
        let _field_span_layer = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Layer", position = pos).entered()
        };
        let layer = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_layer);

        Ok(Self {
            id,
            layer,
        })
    }
}

impl crate::writers::ACWritable for LayeredSpellId {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "LayeredSpellId").entered();

        self.id.write(writer)?;
        write_u16(writer, self.layer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for Vector3 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Vector3").entered();

        #[cfg(feature = "tracing")]
        let _field_span_x = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "X", position = pos).entered()
        };
        let x = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_x);
        #[cfg(feature = "tracing")]
        let _field_span_y = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Y", position = pos).entered()
        };
        let y = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_y);
        #[cfg(feature = "tracing")]
        let _field_span_z = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Z", position = pos).entered()
        };
        let z = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_z);

        Ok(Self {
            x,
            y,
            z,
        })
    }
}

impl crate::writers::ACWritable for Vector3 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Vector3").entered();

        write_f32(writer, self.x)?;
        write_f32(writer, self.y)?;
        write_f32(writer, self.z)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for Quaternion {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Quaternion").entered();

        #[cfg(feature = "tracing")]
        let _field_span_w = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "W", position = pos).entered()
        };
        let w = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_w);
        #[cfg(feature = "tracing")]
        let _field_span_x = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "X", position = pos).entered()
        };
        let x = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_x);
        #[cfg(feature = "tracing")]
        let _field_span_y = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Y", position = pos).entered()
        };
        let y = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_y);
        #[cfg(feature = "tracing")]
        let _field_span_z = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Z", position = pos).entered()
        };
        let z = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_z);

        Ok(Self {
            w,
            x,
            y,
            z,
        })
    }
}

impl crate::writers::ACWritable for Quaternion {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Quaternion").entered();

        write_f32(writer, self.w)?;
        write_f32(writer, self.x)?;
        write_f32(writer, self.y)?;
        write_f32(writer, self.z)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for Origin {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Origin").entered();

        #[cfg(feature = "tracing")]
        let _field_span_landcell = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Landcell", position = pos).entered()
        };
        let landcell = LandcellId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_landcell);
        #[cfg(feature = "tracing")]
        let _field_span_location = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Location", position = pos).entered()
        };
        let location = Vector3::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_location);

        Ok(Self {
            landcell,
            location,
        })
    }
}

impl crate::writers::ACWritable for Origin {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Origin").entered();

        self.landcell.write(writer)?;
        self.location.write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for Position {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Position").entered();

        #[cfg(feature = "tracing")]
        let _field_span_landcell = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Landcell", position = pos).entered()
        };
        let landcell = LandcellId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_landcell);
        #[cfg(feature = "tracing")]
        let _field_span_frame = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Frame", position = pos).entered()
        };
        let frame = Frame::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_frame);

        Ok(Self {
            landcell,
            frame,
        })
    }
}

impl crate::writers::ACWritable for Position {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Position").entered();

        self.landcell.write(writer)?;
        self.frame.write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for Frame {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Frame").entered();

        #[cfg(feature = "tracing")]
        let _field_span_origin = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Origin", position = pos).entered()
        };
        let origin = Vector3::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_origin);
        #[cfg(feature = "tracing")]
        let _field_span_orientation = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Orientation", position = pos).entered()
        };
        let orientation = Quaternion::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_orientation);

        Ok(Self {
            origin,
            orientation,
        })
    }
}

impl crate::writers::ACWritable for Frame {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Frame").entered();

        self.origin.write(writer)?;
        self.orientation.write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for ServerSwitchHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ServerSwitchHeader").entered();

        #[cfg(feature = "tracing")]
        let _field_span_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Sequence", position = pos).entered()
        };
        let sequence = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = ServerSwitchType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::writers::ACWritable for ServerSwitchHeader {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ServerSwitchHeader").entered();

        write_u32(writer, self.sequence)?;
        write_u32(writer, self.type_.clone() as u32)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for CICMDCommandHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CICMDCommandHeader").entered();

        #[cfg(feature = "tracing")]
        let _field_span_command = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Command", position = pos).entered()
        };
        let command = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_command);
        #[cfg(feature = "tracing")]
        let _field_span_parameter = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Parameter", position = pos).entered()
        };
        let parameter = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_parameter);

        Ok(Self {
            command,
            parameter,
        })
    }
}

impl crate::writers::ACWritable for CICMDCommandHeader {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CICMDCommandHeader").entered();

        write_u32(writer, self.command)?;
        write_u32(writer, self.parameter)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for FlowHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "FlowHeader").entered();

        #[cfg(feature = "tracing")]
        let _field_span_bytes = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Bytes", position = pos).entered()
        };
        let bytes = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_bytes);
        #[cfg(feature = "tracing")]
        let _field_span_interval = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Interval", position = pos).entered()
        };
        let interval = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_interval);

        Ok(Self {
            bytes,
            interval,
        })
    }
}

impl crate::writers::ACWritable for FlowHeader {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "FlowHeader").entered();

        write_u32(writer, self.bytes)?;
        write_u16(writer, self.interval)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for SocketAddress {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SocketAddress").entered();

        #[cfg(feature = "tracing")]
        let _field_span_family = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Family", position = pos).entered()
        };
        let family = read_i16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_family);
        #[cfg(feature = "tracing")]
        let _field_span_port = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Port", position = pos).entered()
        };
        let port = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_port);
        #[cfg(feature = "tracing")]
        let _field_span_address = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Address", position = pos).entered()
        };
        let address = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_address);
        #[cfg(feature = "tracing")]
        let _field_span_empty = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Empty", position = pos).entered()
        };
        let empty = read_u64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_empty);

        Ok(Self {
            family,
            port,
            address,
            empty,
        })
    }
}

impl crate::writers::ACWritable for SocketAddress {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "SocketAddress").entered();

        write_i16(writer, self.family)?;
        write_u16(writer, self.port)?;
        write_u32(writer, self.address)?;
        write_u64(writer, self.empty)?;
        Ok(())
    }
}

impl LoginRequestHeaderType2 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, client_version: string, length: uint, flags: AuthFlags, sequence: uint, account: string, account_to_login_as: string) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "LoginRequestHeaderType2").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "LoginRequestHeaderType40000002").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "LoginRequestHeader").entered();

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

impl LoginRequestHeaderType2 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "LoginRequestHeaderType2").entered();

        write_wstring(writer, &self.password.0)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for LoginRequestHeaderType2 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        LoginRequestHeaderType2::write(self, writer)
    }
}

impl LoginRequestHeaderType40000002 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "LoginRequestHeaderType40000002").entered();

        write_string(writer, &self.gls_ticket)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for LoginRequestHeaderType40000002 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        LoginRequestHeaderType40000002::write(self, writer)
    }
}

impl LoginRequestHeader {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "LoginRequestHeader").entered();


        match self {
            Self::Type2(variant_struct) => {
                LoginRequestHeaderType2::write(variant_struct, writer)?;
            },
            Self::Type40000002(variant_struct) => {
                LoginRequestHeaderType40000002::write(variant_struct, writer)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for LoginRequestHeader {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        LoginRequestHeader::write(self, writer)
    }
}

impl crate::readers::ACDataType for ReferralHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ReferralHeader").entered();

        #[cfg(feature = "tracing")]
        let _field_span_cookie = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Cookie", position = pos).entered()
        };
        let cookie = read_u64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_cookie);
        #[cfg(feature = "tracing")]
        let _field_span_address = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Address", position = pos).entered()
        };
        let address = SocketAddress::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_address);
        #[cfg(feature = "tracing")]
        let _field_span_id_server = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "IdServer", position = pos).entered()
        };
        let id_server = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_id_server);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);
        #[cfg(feature = "tracing")]
        let _field_span_unknown = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Unknown", position = pos).entered()
        };
        let unknown = DWORD::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_unknown);

        Ok(Self {
            cookie,
            address,
            id_server,
            unknown,
        })
    }
}

impl crate::writers::ACWritable for ReferralHeader {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ReferralHeader").entered();

        write_u64(writer, self.cookie)?;
        self.address.write(writer)?;
        write_u16(writer, self.id_server)?;
        align_dword_write(writer)?;
        self.unknown.write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for ConnectRequestHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ConnectRequestHeader").entered();

        #[cfg(feature = "tracing")]
        let _field_span_server_time = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ServerTime", position = pos).entered()
        };
        let server_time = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_server_time);
        #[cfg(feature = "tracing")]
        let _field_span_cookie = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Cookie", position = pos).entered()
        };
        let cookie = read_u64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_cookie);
        #[cfg(feature = "tracing")]
        let _field_span_net_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NetID", position = pos).entered()
        };
        let net_id = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_net_id);
        #[cfg(feature = "tracing")]
        let _field_span_outgoing_seed = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "OutgoingSeed", position = pos).entered()
        };
        let outgoing_seed = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_outgoing_seed);
        #[cfg(feature = "tracing")]
        let _field_span_incoming_seed = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "IncomingSeed", position = pos).entered()
        };
        let incoming_seed = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_incoming_seed);
        #[cfg(feature = "tracing")]
        let _field_span_unknown = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Unknown", position = pos).entered()
        };
        let unknown = DWORD::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_unknown);

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

impl crate::writers::ACWritable for ConnectRequestHeader {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ConnectRequestHeader").entered();

        write_f64(writer, self.server_time)?;
        write_u64(writer, self.cookie)?;
        write_i32(writer, self.net_id)?;
        write_u32(writer, self.outgoing_seed)?;
        write_u32(writer, self.incoming_seed)?;
        self.unknown.write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for NetError {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "NetError").entered();

        #[cfg(feature = "tracing")]
        let _field_span_string_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "StringId", position = pos).entered()
        };
        let string_id = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_string_id);
        #[cfg(feature = "tracing")]
        let _field_span_table_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TableId", position = pos).entered()
        };
        let table_id = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_table_id);

        Ok(Self {
            string_id,
            table_id,
        })
    }
}

impl crate::writers::ACWritable for NetError {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "NetError").entered();

        self.string_id.write(writer)?;
        self.table_id.write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for EchoResponseHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EchoResponseHeader").entered();

        #[cfg(feature = "tracing")]
        let _field_span_local_time = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "LocalTime", position = pos).entered()
        };
        let local_time = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_local_time);
        #[cfg(feature = "tracing")]
        let _field_span_holding_time = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HoldingTime", position = pos).entered()
        };
        let holding_time = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_holding_time);

        Ok(Self {
            local_time,
            holding_time,
        })
    }
}

impl crate::writers::ACWritable for EchoResponseHeader {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EchoResponseHeader").entered();

        write_f32(writer, self.local_time)?;
        write_f32(writer, self.holding_time)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for ACBaseQualities {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ACBaseQualities").entered();

        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = Ok::<_, Box<dyn std::error::Error>>(ACBaseQualitiesFlags::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_weenie_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WeenieType", position = pos).entered()
        };
        let weenie_type = WeenieType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_weenie_type);
        let mut int_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_INT.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_int_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "IntProperties", position = pos).entered()
            };
            int_properties = Some(read_packable_hash_table::<PropertyInt, i32>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_int_properties);
        }
        let mut int64_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_INT64.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_int64_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Int64Properties", position = pos).entered()
            };
            int64_properties = Some(read_packable_hash_table::<PropertyInt64, i64>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_int64_properties);
        }
        let mut bool_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_BOOL.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_bool_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BoolProperties", position = pos).entered()
            };
            bool_properties = Some(read_packable_hash_table::<PropertyBool, bool>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_bool_properties);
        }
        let mut float_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_FLOAT.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_float_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "FloatProperties", position = pos).entered()
            };
            float_properties = Some(read_packable_hash_table::<PropertyFloat, f64>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_float_properties);
        }
        let mut string_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_STRING.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_string_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "StringProperties", position = pos).entered()
            };
            string_properties = Some(read_packable_hash_table::<PropertyString, String>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_string_properties);
        }
        let mut data_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_DATA_ID.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_data_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "DataProperties", position = pos).entered()
            };
            data_properties = Some(read_packable_hash_table::<PropertyDataId, DataId>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_data_properties);
        }
        let mut instance_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_INSTANCE_ID.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_instance_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "InstanceProperties", position = pos).entered()
            };
            instance_properties = Some(read_packable_hash_table::<PropertyInstanceId, ObjectId>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_instance_properties);
        }
        let mut position_properties = None;
        if (flags.bits() & ACBaseQualitiesFlags::PROPERTY_POSITION.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_position_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "PositionProperties", position = pos).entered()
            };
            position_properties = Some(read_packable_hash_table::<PropertyPosition, Position>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_position_properties);
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

impl crate::writers::ACWritable for ACBaseQualities {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ACBaseQualities").entered();

        write_u32(writer, self.flags.bits())?;
        write_u32(writer, self.weenie_type.clone() as u32)?;
        if (self.flags.bits() & ACBaseQualitiesFlags::PROPERTY_INT.bits()) != 0 {
            if let Some(ref value) = self.int_properties {
                write_packable_hash_table::<PropertyInt, i32>(writer, &value)?;
            }
        }
        if (self.flags.bits() & ACBaseQualitiesFlags::PROPERTY_INT64.bits()) != 0 {
            if let Some(ref value) = self.int64_properties {
                write_packable_hash_table::<PropertyInt64, i64>(writer, &value)?;
            }
        }
        if (self.flags.bits() & ACBaseQualitiesFlags::PROPERTY_BOOL.bits()) != 0 {
            if let Some(ref value) = self.bool_properties {
                write_packable_hash_table::<PropertyBool, bool>(writer, &value)?;
            }
        }
        if (self.flags.bits() & ACBaseQualitiesFlags::PROPERTY_FLOAT.bits()) != 0 {
            if let Some(ref value) = self.float_properties {
                write_packable_hash_table::<PropertyFloat, f64>(writer, &value)?;
            }
        }
        if (self.flags.bits() & ACBaseQualitiesFlags::PROPERTY_STRING.bits()) != 0 {
            if let Some(ref value) = self.string_properties {
                write_packable_hash_table::<PropertyString, String>(writer, &value)?;
            }
        }
        if (self.flags.bits() & ACBaseQualitiesFlags::PROPERTY_DATA_ID.bits()) != 0 {
            if let Some(ref value) = self.data_properties {
                write_packable_hash_table::<PropertyDataId, DataId>(writer, &value)?;
            }
        }
        if (self.flags.bits() & ACBaseQualitiesFlags::PROPERTY_INSTANCE_ID.bits()) != 0 {
            if let Some(ref value) = self.instance_properties {
                write_packable_hash_table::<PropertyInstanceId, ObjectId>(writer, &value)?;
            }
        }
        if (self.flags.bits() & ACBaseQualitiesFlags::PROPERTY_POSITION.bits()) != 0 {
            if let Some(ref value) = self.position_properties {
                write_packable_hash_table::<PropertyPosition, Position>(writer, &value)?;
            }
        }
        Ok(())
    }
}

impl crate::readers::ACDataType for ACQualities {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ACQualities").entered();

        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = Ok::<_, Box<dyn std::error::Error>>(ACQualitiesFlags::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_has_health = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HasHealth", position = pos).entered()
        };
        let has_health = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_has_health);
        let mut attributes = None;
        if (flags.bits() & ACQualitiesFlags::ATTRIBUTES.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_attributes = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Attributes", position = pos).entered()
            };
            attributes = Some(AttributeCache::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_attributes);
        }
        let mut skills = None;
        if (flags.bits() & ACQualitiesFlags::SKILLS.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_skills = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Skills", position = pos).entered()
            };
            skills = Some(read_packable_hash_table::<SkillId, Skill>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_skills);
        }
        let mut body = None;
        if (flags.bits() & ACQualitiesFlags::BODY.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_body = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Body", position = pos).entered()
            };
            body = Some(Body::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_body);
        }
        let mut spell_book = None;
        if (flags.bits() & ACQualitiesFlags::SPELL_BOOK.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_spell_book = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SpellBook", position = pos).entered()
            };
            spell_book = Some(read_packable_hash_table::<LayeredSpellId, SpellBookPage>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_spell_book);
        }
        let mut enchantments = None;
        if (flags.bits() & ACQualitiesFlags::ENCHANTMENTS.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_enchantments = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Enchantments", position = pos).entered()
            };
            enchantments = Some(EnchantmentRegistry::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_enchantments);
        }
        let mut event_filter = None;
        if (flags.bits() & ACQualitiesFlags::EVENT_FILTER.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_event_filter = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "EventFilter", position = pos).entered()
            };
            event_filter = Some(EventFilter::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_event_filter);
        }
        let mut emotes = None;
        if (flags.bits() & ACQualitiesFlags::EMOTES.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_emotes = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Emotes", position = pos).entered()
            };
            emotes = Some(EmoteTable::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_emotes);
        }
        let mut creation_profile = None;
        if (flags.bits() & ACQualitiesFlags::CREATION_PROFILE.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_creation_profile = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CreationProfile", position = pos).entered()
            };
            creation_profile = Some(read_packable_list::<CreationProfile>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_creation_profile);
        }
        let mut page_data = None;
        if (flags.bits() & ACQualitiesFlags::PAGE_DATA.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_page_data = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "PageData", position = pos).entered()
            };
            page_data = Some(PageDataList::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_page_data);
        }
        let mut generators = None;
        if (flags.bits() & ACQualitiesFlags::GENERATORS.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_generators = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Generators", position = pos).entered()
            };
            generators = Some(GeneratorTable::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_generators);
        }
        let mut generator_registry = None;
        if (flags.bits() & ACQualitiesFlags::GENERATOR_REGISTRY.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_generator_registry = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "GeneratorRegistry", position = pos).entered()
            };
            generator_registry = Some(GeneratorRegistry::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_generator_registry);
        }
        let mut generator_queue = None;
        if (flags.bits() & ACQualitiesFlags::GENERATOR_QUEUE.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_generator_queue = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "GeneratorQueue", position = pos).entered()
            };
            generator_queue = Some(GeneratorQueue::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_generator_queue);
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

impl crate::writers::ACWritable for ACQualities {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ACQualities").entered();

        write_u32(writer, self.flags.bits())?;
        write_bool(writer, self.has_health)?;
        if (self.flags.bits() & ACQualitiesFlags::ATTRIBUTES.bits()) != 0 {
            if let Some(ref value) = self.attributes {
                value.write(writer)?;
            }
        }
        if (self.flags.bits() & ACQualitiesFlags::SKILLS.bits()) != 0 {
            if let Some(ref value) = self.skills {
                write_packable_hash_table::<SkillId, Skill>(writer, &value)?;
            }
        }
        if (self.flags.bits() & ACQualitiesFlags::BODY.bits()) != 0 {
            if let Some(ref value) = self.body {
                value.write(writer)?;
            }
        }
        if (self.flags.bits() & ACQualitiesFlags::SPELL_BOOK.bits()) != 0 {
            if let Some(ref value) = self.spell_book {
                write_packable_hash_table::<LayeredSpellId, SpellBookPage>(writer, &value)?;
            }
        }
        if (self.flags.bits() & ACQualitiesFlags::ENCHANTMENTS.bits()) != 0 {
            if let Some(ref value) = self.enchantments {
                value.write(writer)?;
            }
        }
        if (self.flags.bits() & ACQualitiesFlags::EVENT_FILTER.bits()) != 0 {
            if let Some(ref value) = self.event_filter {
                value.write(writer)?;
            }
        }
        if (self.flags.bits() & ACQualitiesFlags::EMOTES.bits()) != 0 {
            if let Some(ref value) = self.emotes {
                value.write(writer)?;
            }
        }
        if (self.flags.bits() & ACQualitiesFlags::CREATION_PROFILE.bits()) != 0 {
            if let Some(ref value) = self.creation_profile {
                write_packable_list::<CreationProfile>(writer, &value)?;
            }
        }
        if (self.flags.bits() & ACQualitiesFlags::PAGE_DATA.bits()) != 0 {
            if let Some(ref value) = self.page_data {
                value.write(writer)?;
            }
        }
        if (self.flags.bits() & ACQualitiesFlags::GENERATORS.bits()) != 0 {
            if let Some(ref value) = self.generators {
                value.write(writer)?;
            }
        }
        if (self.flags.bits() & ACQualitiesFlags::GENERATOR_REGISTRY.bits()) != 0 {
            if let Some(ref value) = self.generator_registry {
                value.write(writer)?;
            }
        }
        if (self.flags.bits() & ACQualitiesFlags::GENERATOR_QUEUE.bits()) != 0 {
            if let Some(ref value) = self.generator_queue {
                value.write(writer)?;
            }
        }
        Ok(())
    }
}

impl crate::readers::ACDataType for AttributeCache {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AttributeCache").entered();

        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        let mut strength = None;
        if (flags & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_strength = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Strength", position = pos).entered()
            };
            strength = Some(AttributeInfo::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_strength);
        }
        let mut endurance = None;
        if (flags & 0x00000002) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_endurance = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Endurance", position = pos).entered()
            };
            endurance = Some(AttributeInfo::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_endurance);
        }
        let mut quickness = None;
        if (flags & 0x00000004) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_quickness = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Quickness", position = pos).entered()
            };
            quickness = Some(AttributeInfo::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_quickness);
        }
        let mut coordination = None;
        if (flags & 0x00000008) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_coordination = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Coordination", position = pos).entered()
            };
            coordination = Some(AttributeInfo::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_coordination);
        }
        let mut focus = None;
        if (flags & 0x00000010) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_focus = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Focus", position = pos).entered()
            };
            focus = Some(AttributeInfo::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_focus);
        }
        let mut self_ = None;
        if (flags & 0x00000020) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_self_ = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Self", position = pos).entered()
            };
            self_ = Some(AttributeInfo::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_self_);
        }
        let mut health = None;
        if (flags & 0x00000040) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_health = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Health", position = pos).entered()
            };
            health = Some(SecondaryAttributeInfo::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_health);
        }
        let mut stamina = None;
        if (flags & 0x00000080) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_stamina = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Stamina", position = pos).entered()
            };
            stamina = Some(SecondaryAttributeInfo::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_stamina);
        }
        let mut mana = None;
        if (flags & 0x00000100) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_mana = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Mana", position = pos).entered()
            };
            mana = Some(SecondaryAttributeInfo::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_mana);
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

impl crate::writers::ACWritable for AttributeCache {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AttributeCache").entered();

        write_u32(writer, self.flags)?;
        if (self.flags & 0x00000001) != 0 {
            if let Some(ref value) = self.strength {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000002) != 0 {
            if let Some(ref value) = self.endurance {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000004) != 0 {
            if let Some(ref value) = self.quickness {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.coordination {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000010) != 0 {
            if let Some(ref value) = self.focus {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000020) != 0 {
            if let Some(ref value) = self.self_ {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000040) != 0 {
            if let Some(ref value) = self.health {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000080) != 0 {
            if let Some(ref value) = self.stamina {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000100) != 0 {
            if let Some(ref value) = self.mana {
                value.write(writer)?;
            }
        }
        Ok(())
    }
}

impl crate::readers::ACDataType for AttributeInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AttributeInfo").entered();

        #[cfg(feature = "tracing")]
        let _field_span_points_raised = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PointsRaised", position = pos).entered()
        };
        let points_raised = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_points_raised);
        #[cfg(feature = "tracing")]
        let _field_span_innate_points = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "InnatePoints", position = pos).entered()
        };
        let innate_points = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_innate_points);
        #[cfg(feature = "tracing")]
        let _field_span_experience_spent = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ExperienceSpent", position = pos).entered()
        };
        let experience_spent = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_experience_spent);

        Ok(Self {
            points_raised,
            innate_points,
            experience_spent,
        })
    }
}

impl crate::writers::ACWritable for AttributeInfo {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AttributeInfo").entered();

        write_u32(writer, self.points_raised)?;
        write_u32(writer, self.innate_points)?;
        write_u32(writer, self.experience_spent)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for SecondaryAttributeInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SecondaryAttributeInfo").entered();

        #[cfg(feature = "tracing")]
        let _field_span_attribute = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Attribute", position = pos).entered()
        };
        let attribute = AttributeInfo::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_attribute);
        #[cfg(feature = "tracing")]
        let _field_span_current = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Current", position = pos).entered()
        };
        let current = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_current);

        Ok(Self {
            attribute,
            current,
        })
    }
}

impl crate::writers::ACWritable for SecondaryAttributeInfo {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "SecondaryAttributeInfo").entered();

        self.attribute.write(writer)?;
        write_u32(writer, self.current)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for Skill {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Skill").entered();

        #[cfg(feature = "tracing")]
        let _field_span_points_raised = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PointsRaised", position = pos).entered()
        };
        let points_raised = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_points_raised);
        #[cfg(feature = "tracing")]
        let _field_span_adjust_pp = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AdjustPP", position = pos).entered()
        };
        let adjust_pp = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_adjust_pp);
        #[cfg(feature = "tracing")]
        let _field_span_training_level = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TrainingLevel", position = pos).entered()
        };
        let training_level = SkillAdvancementClass::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_training_level);
        #[cfg(feature = "tracing")]
        let _field_span_experience_spent = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ExperienceSpent", position = pos).entered()
        };
        let experience_spent = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_experience_spent);
        #[cfg(feature = "tracing")]
        let _field_span_innate_points = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "InnatePoints", position = pos).entered()
        };
        let innate_points = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_innate_points);
        #[cfg(feature = "tracing")]
        let _field_span_resistance_of_last_check = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ResistanceOfLastCheck", position = pos).entered()
        };
        let resistance_of_last_check = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_resistance_of_last_check);
        #[cfg(feature = "tracing")]
        let _field_span_last_used_time = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "LastUsedTime", position = pos).entered()
        };
        let last_used_time = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_last_used_time);

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

impl crate::writers::ACWritable for Skill {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Skill").entered();

        write_u16(writer, self.points_raised)?;
        write_u16(writer, self.adjust_pp)?;
        write_u32(writer, self.training_level.clone() as u32)?;
        write_u32(writer, self.experience_spent)?;
        write_u32(writer, self.innate_points)?;
        write_u32(writer, self.resistance_of_last_check)?;
        write_f64(writer, self.last_used_time)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for Body {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Body").entered();

        #[cfg(feature = "tracing")]
        let _field_span_body_parts = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BodyParts", position = pos).entered()
        };
        let body_parts = read_packable_hash_table::<u32, BodyPart>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_body_parts);

        Ok(Self {
            body_parts,
        })
    }
}

impl crate::writers::ACWritable for Body {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Body").entered();

        write_packable_hash_table::<u32, BodyPart>(writer, &self.body_parts)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for BodyPart {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "BodyPart").entered();

        #[cfg(feature = "tracing")]
        let _field_span_has_bpsd = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HasBPSD", position = pos).entered()
        };
        let has_bpsd = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_has_bpsd);
        #[cfg(feature = "tracing")]
        let _field_span_damage_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DamageType", position = pos).entered()
        };
        let damage_type = Ok::<_, Box<dyn std::error::Error>>(DamageType::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_damage_type);
        #[cfg(feature = "tracing")]
        let _field_span_damage_val = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DamageVal", position = pos).entered()
        };
        let damage_val = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_damage_val);
        #[cfg(feature = "tracing")]
        let _field_span_damage_var = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DamageVar", position = pos).entered()
        };
        let damage_var = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_damage_var);
        #[cfg(feature = "tracing")]
        let _field_span_armor_cache = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ArmorCache", position = pos).entered()
        };
        let armor_cache = ArmorCache::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_armor_cache);
        #[cfg(feature = "tracing")]
        let _field_span_bh = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BH", position = pos).entered()
        };
        let bh = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_bh);
        let mut bpsd = None;
        if (has_bpsd & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_bpsd = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BPSD", position = pos).entered()
            };
            bpsd = Some(BodyPartSelectionData::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_bpsd);
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

impl crate::writers::ACWritable for BodyPart {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "BodyPart").entered();

        write_i32(writer, self.has_bpsd)?;
        write_u32(writer, self.damage_type.bits())?;
        write_i32(writer, self.damage_val)?;
        write_i32(writer, self.damage_var)?;
        self.armor_cache.write(writer)?;
        write_i32(writer, self.bh)?;
        if (self.has_bpsd & 0x00000001) != 0 {
            if let Some(ref value) = self.bpsd {
                value.write(writer)?;
            }
        }
        Ok(())
    }
}

impl crate::readers::ACDataType for ArmorCache {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ArmorCache").entered();

        #[cfg(feature = "tracing")]
        let _field_span_base_armor = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BaseArmor", position = pos).entered()
        };
        let base_armor = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_base_armor);
        #[cfg(feature = "tracing")]
        let _field_span_armor_vs_slash = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ArmorVsSlash", position = pos).entered()
        };
        let armor_vs_slash = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_armor_vs_slash);
        #[cfg(feature = "tracing")]
        let _field_span_armor_vs_pierce = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ArmorVsPierce", position = pos).entered()
        };
        let armor_vs_pierce = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_armor_vs_pierce);
        #[cfg(feature = "tracing")]
        let _field_span_armor_vs_bludgeon = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ArmorVsBludgeon", position = pos).entered()
        };
        let armor_vs_bludgeon = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_armor_vs_bludgeon);
        #[cfg(feature = "tracing")]
        let _field_span_armor_vs_cold = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ArmorVsCold", position = pos).entered()
        };
        let armor_vs_cold = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_armor_vs_cold);
        #[cfg(feature = "tracing")]
        let _field_span_armor_vs_fire = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ArmorVsFire", position = pos).entered()
        };
        let armor_vs_fire = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_armor_vs_fire);
        #[cfg(feature = "tracing")]
        let _field_span_armor_vs_acid = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ArmorVsAcid", position = pos).entered()
        };
        let armor_vs_acid = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_armor_vs_acid);
        #[cfg(feature = "tracing")]
        let _field_span_armor_vs_electric = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ArmorVsElectric", position = pos).entered()
        };
        let armor_vs_electric = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_armor_vs_electric);
        #[cfg(feature = "tracing")]
        let _field_span_armor_vs_nether = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ArmorVsNether", position = pos).entered()
        };
        let armor_vs_nether = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_armor_vs_nether);

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

impl crate::writers::ACWritable for ArmorCache {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ArmorCache").entered();

        write_i32(writer, self.base_armor)?;
        write_i32(writer, self.armor_vs_slash)?;
        write_i32(writer, self.armor_vs_pierce)?;
        write_i32(writer, self.armor_vs_bludgeon)?;
        write_i32(writer, self.armor_vs_cold)?;
        write_i32(writer, self.armor_vs_fire)?;
        write_i32(writer, self.armor_vs_acid)?;
        write_i32(writer, self.armor_vs_electric)?;
        write_i32(writer, self.armor_vs_nether)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for BodyPartSelectionData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "BodyPartSelectionData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_hlf = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HLF", position = pos).entered()
        };
        let hlf = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_hlf);
        #[cfg(feature = "tracing")]
        let _field_span_mlf = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MLF", position = pos).entered()
        };
        let mlf = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_mlf);
        #[cfg(feature = "tracing")]
        let _field_span_llf = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "LLF", position = pos).entered()
        };
        let llf = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_llf);
        #[cfg(feature = "tracing")]
        let _field_span_hrf = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HRF", position = pos).entered()
        };
        let hrf = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_hrf);
        #[cfg(feature = "tracing")]
        let _field_span_mrf = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MRF", position = pos).entered()
        };
        let mrf = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_mrf);
        #[cfg(feature = "tracing")]
        let _field_span_lrf = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "LRF", position = pos).entered()
        };
        let lrf = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_lrf);
        #[cfg(feature = "tracing")]
        let _field_span_hlb = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HLB", position = pos).entered()
        };
        let hlb = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_hlb);
        #[cfg(feature = "tracing")]
        let _field_span_mlb = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MLB", position = pos).entered()
        };
        let mlb = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_mlb);
        #[cfg(feature = "tracing")]
        let _field_span_llb = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "LLB", position = pos).entered()
        };
        let llb = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_llb);
        #[cfg(feature = "tracing")]
        let _field_span_hrb = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HRB", position = pos).entered()
        };
        let hrb = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_hrb);
        #[cfg(feature = "tracing")]
        let _field_span_mrb = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MRB", position = pos).entered()
        };
        let mrb = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_mrb);
        #[cfg(feature = "tracing")]
        let _field_span_lrb = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "LRB", position = pos).entered()
        };
        let lrb = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_lrb);

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

impl crate::writers::ACWritable for BodyPartSelectionData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "BodyPartSelectionData").entered();

        write_i32(writer, self.hlf)?;
        write_i32(writer, self.mlf)?;
        write_i32(writer, self.llf)?;
        write_i32(writer, self.hrf)?;
        write_i32(writer, self.mrf)?;
        write_i32(writer, self.lrf)?;
        write_i32(writer, self.hlb)?;
        write_i32(writer, self.mlb)?;
        write_i32(writer, self.llb)?;
        write_i32(writer, self.hrb)?;
        write_i32(writer, self.mrb)?;
        write_i32(writer, self.lrb)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for SpellBookPage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SpellBookPage").entered();

        #[cfg(feature = "tracing")]
        let _field_span_casting_likelihood = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CastingLikelihood", position = pos).entered()
        };
        let casting_likelihood = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_casting_likelihood);
        let mut casting_likelihood2 = None;
        let mut unknown = None;
        if casting_likelihood < 2.0 {
            #[cfg(feature = "tracing")]
            let _field_span_casting_likelihood2 = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CastingLikelihood2", position = pos).entered()
            };
            casting_likelihood2 = if casting_likelihood < 2.0 { read_f32(reader).map(Some) } else { Ok(None) }?;
            #[cfg(feature = "tracing")]
            drop(_field_span_casting_likelihood2);
            #[cfg(feature = "tracing")]
            let _field_span_unknown = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Unknown", position = pos).entered()
            };
            unknown = if casting_likelihood < 2.0 { read_i32(reader).map(Some) } else { Ok(None) }?;
            #[cfg(feature = "tracing")]
            drop(_field_span_unknown);
        }

        Ok(Self {
            casting_likelihood,
            casting_likelihood2,
            unknown,
        })
    }
}

impl crate::writers::ACWritable for SpellBookPage {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "SpellBookPage").entered();

        write_f32(writer, self.casting_likelihood)?;
        if self.casting_likelihood < 2.0 {
            if let Some(ref value) = self.casting_likelihood2 {
                write_f32(writer, *value)?;
            }
        }
        if self.casting_likelihood < 2.0 {
            if let Some(ref value) = self.unknown {
                write_i32(writer, *value)?;
            }
        }
        Ok(())
    }
}

impl crate::readers::ACDataType for EnchantmentRegistry {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EnchantmentRegistry").entered();

        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = Ok::<_, Box<dyn std::error::Error>>(EnchantmentRegistryFlags::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        let mut life_spells = None;
        if (flags.bits() & EnchantmentRegistryFlags::LIFE_SPELLS.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_life_spells = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "LifeSpells", position = pos).entered()
            };
            life_spells = Some(read_packable_list::<Enchantment>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_life_spells);
        }
        let mut creature_spells = None;
        if (flags.bits() & EnchantmentRegistryFlags::CREATURE_SPELLS.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_creature_spells = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CreatureSpells", position = pos).entered()
            };
            creature_spells = Some(read_packable_list::<Enchantment>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_creature_spells);
        }
        let mut vitae = None;
        if (flags.bits() & EnchantmentRegistryFlags::VITAE.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_vitae = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Vitae", position = pos).entered()
            };
            vitae = Some(Enchantment::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_vitae);
        }
        let mut cooldowns = None;
        if (flags.bits() & EnchantmentRegistryFlags::COOLDOWNS.bits()) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_cooldowns = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Cooldowns", position = pos).entered()
            };
            cooldowns = Some(read_packable_list::<Enchantment>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_cooldowns);
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

impl crate::writers::ACWritable for EnchantmentRegistry {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EnchantmentRegistry").entered();

        write_u32(writer, self.flags.bits())?;
        if (self.flags.bits() & EnchantmentRegistryFlags::LIFE_SPELLS.bits()) != 0 {
            if let Some(ref value) = self.life_spells {
                write_packable_list::<Enchantment>(writer, &value)?;
            }
        }
        if (self.flags.bits() & EnchantmentRegistryFlags::CREATURE_SPELLS.bits()) != 0 {
            if let Some(ref value) = self.creature_spells {
                write_packable_list::<Enchantment>(writer, &value)?;
            }
        }
        if (self.flags.bits() & EnchantmentRegistryFlags::VITAE.bits()) != 0 {
            if let Some(ref value) = self.vitae {
                value.write(writer)?;
            }
        }
        if (self.flags.bits() & EnchantmentRegistryFlags::COOLDOWNS.bits()) != 0 {
            if let Some(ref value) = self.cooldowns {
                write_packable_list::<Enchantment>(writer, &value)?;
            }
        }
        Ok(())
    }
}

impl crate::readers::ACDataType for Enchantment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Enchantment").entered();

        #[cfg(feature = "tracing")]
        let _field_span_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Id", position = pos).entered()
        };
        let id = LayeredSpellId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_id);
        #[cfg(feature = "tracing")]
        let _field_span_has_equipment_set = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HasEquipmentSet", position = pos).entered()
        };
        let has_equipment_set = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_has_equipment_set);
        #[cfg(feature = "tracing")]
        let _field_span_spell_category = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SpellCategory", position = pos).entered()
        };
        let spell_category = SpellCategory::try_from(read_u16(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_spell_category);
        #[cfg(feature = "tracing")]
        let _field_span_power_level = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PowerLevel", position = pos).entered()
        };
        let power_level = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_power_level);
        #[cfg(feature = "tracing")]
        let _field_span_start_time = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "StartTime", position = pos).entered()
        };
        let start_time = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_start_time);
        #[cfg(feature = "tracing")]
        let _field_span_duration = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Duration", position = pos).entered()
        };
        let duration = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_duration);
        #[cfg(feature = "tracing")]
        let _field_span_caster_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CasterId", position = pos).entered()
        };
        let caster_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_caster_id);
        #[cfg(feature = "tracing")]
        let _field_span_degrade_modifier = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DegradeModifier", position = pos).entered()
        };
        let degrade_modifier = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_degrade_modifier);
        #[cfg(feature = "tracing")]
        let _field_span_degrade_limit = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DegradeLimit", position = pos).entered()
        };
        let degrade_limit = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_degrade_limit);
        #[cfg(feature = "tracing")]
        let _field_span_last_time_degraded = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "LastTimeDegraded", position = pos).entered()
        };
        let last_time_degraded = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_last_time_degraded);
        #[cfg(feature = "tracing")]
        let _field_span_stat_mod = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "StatMod", position = pos).entered()
        };
        let stat_mod = StatMod::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_stat_mod);
        let mut equipment_set = None;
        if has_equipment_set > 0 {
            #[cfg(feature = "tracing")]
            let _field_span_equipment_set = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "EquipmentSet", position = pos).entered()
            };
            equipment_set = if has_equipment_set > 0 { EquipmentSet::try_from(read_u32(reader)?).map(Some) } else { Ok(None) }?;
            #[cfg(feature = "tracing")]
            drop(_field_span_equipment_set);
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

impl crate::writers::ACWritable for Enchantment {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Enchantment").entered();

        self.id.write(writer)?;
        write_u16(writer, self.has_equipment_set)?;
        write_u16(writer, self.spell_category.clone() as u16)?;
        write_u32(writer, self.power_level)?;
        write_f64(writer, self.start_time)?;
        write_f64(writer, self.duration)?;
        self.caster_id.write(writer)?;
        write_f32(writer, self.degrade_modifier)?;
        write_f32(writer, self.degrade_limit)?;
        write_f64(writer, self.last_time_degraded)?;
        self.stat_mod.write(writer)?;
        if self.has_equipment_set > 0 {
            if let Some(ref value) = self.equipment_set {
                write_u32(writer, value.clone() as u32)?;
            }
        }
        Ok(())
    }
}

impl crate::readers::ACDataType for StatMod {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "StatMod").entered();

        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = Ok::<_, Box<dyn std::error::Error>>(EnchantmentTypeFlags::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);
        #[cfg(feature = "tracing")]
        let _field_span_key = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Key", position = pos).entered()
        };
        let key = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_key);
        #[cfg(feature = "tracing")]
        let _field_span_value = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Value", position = pos).entered()
        };
        let value = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_value);

        Ok(Self {
            type_,
            key,
            value,
        })
    }
}

impl crate::writers::ACWritable for StatMod {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "StatMod").entered();

        write_u32(writer, self.type_.bits())?;
        write_u32(writer, self.key)?;
        write_f32(writer, self.value)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for EventFilter {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EventFilter").entered();

        #[cfg(feature = "tracing")]
        let _field_span_events = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Events", position = pos).entered()
        };
        let events = read_packable_list::<u32>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_events);

        Ok(Self {
            events,
        })
    }
}

impl crate::writers::ACWritable for EventFilter {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EventFilter").entered();

        write_packable_list::<u32>(writer, &self.events)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for EmoteTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteTable").entered();

        #[cfg(feature = "tracing")]
        let _field_span_emotes = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Emotes", position = pos).entered()
        };
        let emotes = read_packable_hash_table::<EmoteCategory, EmoteSetList>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_emotes);

        Ok(Self {
            emotes,
        })
    }
}

impl crate::writers::ACWritable for EmoteTable {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteTable").entered();

        write_packable_hash_table::<EmoteCategory, EmoteSetList>(writer, &self.emotes)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for EmoteSetList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteSetList").entered();

        #[cfg(feature = "tracing")]
        let _field_span_emotes = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Emotes", position = pos).entered()
        };
        let emotes = read_packable_list::<EmoteSet>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_emotes);

        Ok(Self {
            emotes,
        })
    }
}

impl crate::writers::ACWritable for EmoteSetList {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteSetList").entered();

        write_packable_list::<EmoteSet>(writer, &self.emotes)?;
        Ok(())
    }
}

impl EmoteSetType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, probability: float, emotes: PackableList<Emote>) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteSetType1").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteSetType2").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteSetType5").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteSetTypeC").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteSetTypeF").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteSet").entered();

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

impl EmoteSetType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteSetType1").entered();

        write_u32(writer, self.class_id)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteSetType1 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteSetType1::write(self, writer)
    }
}

impl EmoteSetType2 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteSetType2").entered();

        write_u32(writer, self.vendor_type)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteSetType2 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteSetType2::write(self, writer)
    }
}

impl EmoteSetType5 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteSetType5").entered();

        write_u32(writer, self.style)?;
        write_u32(writer, self.substyle)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteSetType5 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteSetType5::write(self, writer)
    }
}

impl EmoteSetTypeC {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteSetTypeC").entered();

        write_string(writer, &self.quest)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteSetTypeC {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteSetTypeC::write(self, writer)
    }
}

impl EmoteSetTypeF {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteSetTypeF").entered();

        write_f32(writer, self.min_health)?;
        write_f32(writer, self.max_health)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteSetTypeF {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteSetTypeF::write(self, writer)
    }
}

impl EmoteSet {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteSet").entered();


        match self {
            Self::Type1(variant_struct) => {
                EmoteSetType1::write(variant_struct, writer)?;
            },
            Self::Type2(variant_struct) => {
                EmoteSetType2::write(variant_struct, writer)?;
            },
            Self::Type5(variant_struct) => {
                EmoteSetType5::write(variant_struct, writer)?;
            },
            Self::TypeC(variant_struct) => {
                EmoteSetTypeC::write(variant_struct, writer)?;
            },
            Self::TypeF(variant_struct) => {
                EmoteSetTypeF::write(variant_struct, writer)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteSet {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteSet::write(self, writer)
    }
}

impl EmoteType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType1").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType2").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType3").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType4").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType5").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType7").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType9").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteTypeE").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType1C").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType1E").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType20").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType22").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType23").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType24").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType25").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType26").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType31").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType32").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType35").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType38").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType3F").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType4C").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType6E").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType70").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType72").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EmoteType76").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Emote").entered();

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

impl EmoteType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType1").entered();

        write_string(writer, &self.message)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType1 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType1::write(self, writer)
    }
}

impl EmoteType2 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType2").entered();

        write_u64(writer, self.amount64)?;
        write_u64(writer, self.hero_xp64)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType2 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType2::write(self, writer)
    }
}

impl EmoteType3 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType3").entered();

        self.c_profile.write(writer)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType3 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType3::write(self, writer)
    }
}

impl EmoteType4 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType4").entered();

        self.frame.write(writer)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType4 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType4::write(self, writer)
    }
}

impl EmoteType5 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType5").entered();

        write_u32(writer, self.motion)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType5 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType5::write(self, writer)
    }
}

impl EmoteType7 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType7").entered();

        write_u32(writer, self.physics_script)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType7 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType7::write(self, writer)
    }
}

impl EmoteType9 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType9").entered();

        write_u32(writer, self.sound)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType9 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType9::write(self, writer)
    }
}

impl EmoteTypeE {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteTypeE").entered();

        write_u32(writer, self.spell_id)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteTypeE {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteTypeE::write(self, writer)
    }
}

impl EmoteType1C {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType1C").entered();

        write_u32(writer, self.amount)?;
        write_u32(writer, self.stat)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType1C {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType1C::write(self, writer)
    }
}

impl EmoteType1E {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType1E").entered();

        write_string(writer, &self.message)?;
        write_u32(writer, self.min)?;
        write_u32(writer, self.max)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType1E {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType1E::write(self, writer)
    }
}

impl EmoteType20 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType20").entered();

        write_string(writer, &self.message)?;
        write_u32(writer, self.amount)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType20 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType20::write(self, writer)
    }
}

impl EmoteType22 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType22").entered();

        write_u32(writer, self.amount)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType22 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType22::write(self, writer)
    }
}

impl EmoteType23 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType23").entered();

        write_string(writer, &self.message)?;
        write_u32(writer, self.stat)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType23 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType23::write(self, writer)
    }
}

impl EmoteType24 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType24").entered();

        write_string(writer, &self.message)?;
        write_u32(writer, self.min)?;
        write_u32(writer, self.max)?;
        write_u32(writer, self.stat)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType24 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType24::write(self, writer)
    }
}

impl EmoteType25 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType25").entered();

        write_string(writer, &self.message)?;
        write_f64(writer, self.f_min)?;
        write_f64(writer, self.f_max)?;
        write_u32(writer, self.stat)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType25 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType25::write(self, writer)
    }
}

impl EmoteType26 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType26").entered();

        write_string(writer, &self.message)?;
        write_string(writer, &self.test_string)?;
        write_u32(writer, self.stat)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType26 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType26::write(self, writer)
    }
}

impl EmoteType31 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType31").entered();

        write_f64(writer, self.percent)?;
        write_u64(writer, self.min64)?;
        write_u64(writer, self.max64)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType31 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType31::write(self, writer)
    }
}

impl EmoteType32 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType32").entered();

        write_u32(writer, self.stat)?;
        write_f64(writer, self.percent)?;
        write_u32(writer, self.min)?;
        write_u32(writer, self.max)?;
        write_bool(writer, self.display)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType32 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType32::write(self, writer)
    }
}

impl EmoteType35 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType35").entered();

        write_u32(writer, self.stat)?;
        write_u32(writer, self.amount)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType35 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType35::write(self, writer)
    }
}

impl EmoteType38 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType38").entered();

        write_i32(writer, self.wealth_rating)?;
        write_i32(writer, self.treasure_class)?;
        write_i32(writer, self.treasure_type)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType38 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType38::write(self, writer)
    }
}

impl EmoteType3F {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType3F").entered();

        self.position.write(writer)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType3F {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType3F::write(self, writer)
    }
}

impl EmoteType4C {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType4C").entered();

        write_string(writer, &self.msg)?;
        self.c_profile.write(writer)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType4C {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType4C::write(self, writer)
    }
}

impl EmoteType6E {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType6E").entered();

        write_u32(writer, self.stat)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType6E {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType6E::write(self, writer)
    }
}

impl EmoteType70 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType70").entered();

        write_u64(writer, self.amount64)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType70 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType70::write(self, writer)
    }
}

impl EmoteType72 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType72").entered();

        write_string(writer, &self.message)?;
        write_u64(writer, self.min64)?;
        write_u64(writer, self.max64)?;
        write_u32(writer, self.stat)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType72 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType72::write(self, writer)
    }
}

impl EmoteType76 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EmoteType76").entered();

        write_u32(writer, self.stat)?;
        write_f64(writer, self.percent)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for EmoteType76 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        EmoteType76::write(self, writer)
    }
}

impl Emote {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Emote").entered();


        match self {
            Self::Type1(variant_struct) => {
                EmoteType1::write(variant_struct, writer)?;
            },
            Self::Type2(variant_struct) => {
                EmoteType2::write(variant_struct, writer)?;
            },
            Self::Type3(variant_struct) => {
                EmoteType3::write(variant_struct, writer)?;
            },
            Self::Type4(variant_struct) => {
                EmoteType4::write(variant_struct, writer)?;
            },
            Self::Type5(variant_struct) => {
                EmoteType5::write(variant_struct, writer)?;
            },
            Self::Type7(variant_struct) => {
                EmoteType7::write(variant_struct, writer)?;
            },
            Self::Type9(variant_struct) => {
                EmoteType9::write(variant_struct, writer)?;
            },
            Self::TypeE(variant_struct) => {
                EmoteTypeE::write(variant_struct, writer)?;
            },
            Self::Type1C(variant_struct) => {
                EmoteType1C::write(variant_struct, writer)?;
            },
            Self::Type1E(variant_struct) => {
                EmoteType1E::write(variant_struct, writer)?;
            },
            Self::Type20(variant_struct) => {
                EmoteType20::write(variant_struct, writer)?;
            },
            Self::Type22(variant_struct) => {
                EmoteType22::write(variant_struct, writer)?;
            },
            Self::Type23(variant_struct) => {
                EmoteType23::write(variant_struct, writer)?;
            },
            Self::Type24(variant_struct) => {
                EmoteType24::write(variant_struct, writer)?;
            },
            Self::Type25(variant_struct) => {
                EmoteType25::write(variant_struct, writer)?;
            },
            Self::Type26(variant_struct) => {
                EmoteType26::write(variant_struct, writer)?;
            },
            Self::Type31(variant_struct) => {
                EmoteType31::write(variant_struct, writer)?;
            },
            Self::Type32(variant_struct) => {
                EmoteType32::write(variant_struct, writer)?;
            },
            Self::Type35(variant_struct) => {
                EmoteType35::write(variant_struct, writer)?;
            },
            Self::Type38(variant_struct) => {
                EmoteType38::write(variant_struct, writer)?;
            },
            Self::Type3F(variant_struct) => {
                EmoteType3F::write(variant_struct, writer)?;
            },
            Self::Type4C(variant_struct) => {
                EmoteType4C::write(variant_struct, writer)?;
            },
            Self::Type6E(variant_struct) => {
                EmoteType6E::write(variant_struct, writer)?;
            },
            Self::Type70(variant_struct) => {
                EmoteType70::write(variant_struct, writer)?;
            },
            Self::Type72(variant_struct) => {
                EmoteType72::write(variant_struct, writer)?;
            },
            Self::Type76(variant_struct) => {
                EmoteType76::write(variant_struct, writer)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for Emote {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        Emote::write(self, writer)
    }
}

impl crate::readers::ACDataType for CreationProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CreationProfile").entered();

        #[cfg(feature = "tracing")]
        let _field_span_weenie_class_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WeenieClassId", position = pos).entered()
        };
        let weenie_class_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_weenie_class_id);
        #[cfg(feature = "tracing")]
        let _field_span_palette = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Palette", position = pos).entered()
        };
        let palette = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_palette);
        #[cfg(feature = "tracing")]
        let _field_span_shade = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Shade", position = pos).entered()
        };
        let shade = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_shade);
        #[cfg(feature = "tracing")]
        let _field_span_destination = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Destination", position = pos).entered()
        };
        let destination = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_destination);
        #[cfg(feature = "tracing")]
        let _field_span_stack_size = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "StackSize", position = pos).entered()
        };
        let stack_size = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_stack_size);
        #[cfg(feature = "tracing")]
        let _field_span_try_to_bond = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TryToBond", position = pos).entered()
        };
        let try_to_bond = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_try_to_bond);

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

impl crate::writers::ACWritable for CreationProfile {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CreationProfile").entered();

        write_u32(writer, self.weenie_class_id)?;
        write_u32(writer, self.palette)?;
        write_f32(writer, self.shade)?;
        write_u32(writer, self.destination)?;
        write_i32(writer, self.stack_size)?;
        write_bool(writer, self.try_to_bond)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for PageDataList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "PageDataList").entered();

        #[cfg(feature = "tracing")]
        let _field_span_max_num_pages = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxNumPages", position = pos).entered()
        };
        let max_num_pages = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_num_pages);
        #[cfg(feature = "tracing")]
        let _field_span_max_num_chars_per_page = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxNumCharsPerPage", position = pos).entered()
        };
        let max_num_chars_per_page = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_num_chars_per_page);
        #[cfg(feature = "tracing")]
        let _field_span_pages = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Pages", position = pos).entered()
        };
        let pages = read_packable_list::<PageData>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_pages);

        Ok(Self {
            max_num_pages,
            max_num_chars_per_page,
            pages,
        })
    }
}

impl crate::writers::ACWritable for PageDataList {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "PageDataList").entered();

        write_u32(writer, self.max_num_pages)?;
        write_u32(writer, self.max_num_chars_per_page)?;
        write_packable_list::<PageData>(writer, &self.pages)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for PageData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "PageData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_author_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AuthorId", position = pos).entered()
        };
        let author_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_author_id);
        #[cfg(feature = "tracing")]
        let _field_span_author_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AuthorName", position = pos).entered()
        };
        let author_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_author_name);
        #[cfg(feature = "tracing")]
        let _field_span_author_account = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AuthorAccount", position = pos).entered()
        };
        let author_account = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_author_account);
        #[cfg(feature = "tracing")]
        let _field_span_version = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Version", position = pos).entered()
        };
        let version = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_version);
        #[cfg(feature = "tracing")]
        let _field_span_text_included = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TextIncluded", position = pos).entered()
        };
        let text_included = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_text_included);
        #[cfg(feature = "tracing")]
        let _field_span_ignore_author = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "IgnoreAuthor", position = pos).entered()
        };
        let ignore_author = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_ignore_author);
        let mut page_text = None;
        if text_included {
            #[cfg(feature = "tracing")]
            let _field_span_page_text = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "PageText", position = pos).entered()
            };
            page_text = if text_included { read_string(reader).map(Some) } else { Ok(None) }?;
            #[cfg(feature = "tracing")]
            drop(_field_span_page_text);
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

impl crate::writers::ACWritable for PageData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "PageData").entered();

        self.author_id.write(writer)?;
        write_string(writer, &self.author_name)?;
        write_string(writer, &self.author_account)?;
        write_u32(writer, self.version)?;
        write_bool(writer, self.text_included)?;
        write_bool(writer, self.ignore_author)?;
        if self.text_included {
            if let Some(ref value) = self.page_text {
                write_string(writer, &value)?;
            }
        }
        Ok(())
    }
}

impl crate::readers::ACDataType for BlobFragments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "BlobFragments").entered();

        #[cfg(feature = "tracing")]
        let _field_span_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Sequence", position = pos).entered()
        };
        let sequence = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Id", position = pos).entered()
        };
        let id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_id);
        #[cfg(feature = "tracing")]
        let _field_span_count = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Count", position = pos).entered()
        };
        let count = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_count);
        #[cfg(feature = "tracing")]
        let _field_span_size = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Size", position = pos).entered()
        };
        let size = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_size);
        let body_size = (size - 16) as u16;
        #[cfg(feature = "tracing")]
        let _field_span_index = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Index", position = pos).entered()
        };
        let index = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_index);
        #[cfg(feature = "tracing")]
        let _field_span_group = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Group", position = pos).entered()
        };
        let group = FragmentGroup::try_from(read_u16(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_group);
        #[cfg(feature = "tracing")]
        let _field_span_data = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Data", position = pos).entered()
        };
        let data = read_vec::<u8>(reader, body_size as usize)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_data);

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

impl crate::writers::ACWritable for BlobFragments {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "BlobFragments").entered();

        write_u32(writer, self.sequence)?;
        write_u32(writer, self.id)?;
        write_u16(writer, self.count)?;
        write_u16(writer, self.size)?;
        write_u16(writer, self.index)?;
        write_u16(writer, self.group.clone() as u16)?;
        write_vec::<u8>(writer, &self.data)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for GeneratorTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GeneratorTable").entered();

        #[cfg(feature = "tracing")]
        let _field_span_generators = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Generators", position = pos).entered()
        };
        let generators = read_packable_list::<GeneratorProfile>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_generators);

        Ok(Self {
            generators,
        })
    }
}

impl crate::writers::ACWritable for GeneratorTable {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GeneratorTable").entered();

        write_packable_list::<GeneratorProfile>(writer, &self.generators)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for GeneratorProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GeneratorProfile").entered();

        #[cfg(feature = "tracing")]
        let _field_span_probability = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Probability", position = pos).entered()
        };
        let probability = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_probability);
        #[cfg(feature = "tracing")]
        let _field_span_type_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TypeId", position = pos).entered()
        };
        let type_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_id);
        #[cfg(feature = "tracing")]
        let _field_span_delay = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Delay", position = pos).entered()
        };
        let delay = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_delay);
        #[cfg(feature = "tracing")]
        let _field_span_init_create = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "InitCreate", position = pos).entered()
        };
        let init_create = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_init_create);
        #[cfg(feature = "tracing")]
        let _field_span_max_num = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxNum", position = pos).entered()
        };
        let max_num = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_num);
        #[cfg(feature = "tracing")]
        let _field_span_when_create = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WhenCreate", position = pos).entered()
        };
        let when_create = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_when_create);
        #[cfg(feature = "tracing")]
        let _field_span_where_create = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WhereCreate", position = pos).entered()
        };
        let where_create = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_where_create);
        #[cfg(feature = "tracing")]
        let _field_span_stack_size = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "StackSize", position = pos).entered()
        };
        let stack_size = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_stack_size);
        #[cfg(feature = "tracing")]
        let _field_span_ptid = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Ptid", position = pos).entered()
        };
        let ptid = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_ptid);
        #[cfg(feature = "tracing")]
        let _field_span_shade = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Shade", position = pos).entered()
        };
        let shade = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_shade);
        #[cfg(feature = "tracing")]
        let _field_span_pos_val = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PosVal", position = pos).entered()
        };
        let pos_val = Position::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_pos_val);
        #[cfg(feature = "tracing")]
        let _field_span_slot = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Slot", position = pos).entered()
        };
        let slot = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_slot);

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

impl crate::writers::ACWritable for GeneratorProfile {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GeneratorProfile").entered();

        write_f32(writer, self.probability)?;
        write_u32(writer, self.type_id)?;
        write_f64(writer, self.delay)?;
        write_u32(writer, self.init_create)?;
        write_u32(writer, self.max_num)?;
        write_u32(writer, self.when_create)?;
        write_u32(writer, self.where_create)?;
        write_u32(writer, self.stack_size)?;
        write_u32(writer, self.ptid)?;
        write_f32(writer, self.shade)?;
        self.pos_val.write(writer)?;
        write_u32(writer, self.slot)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for GeneratorRegistry {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GeneratorRegistry").entered();

        #[cfg(feature = "tracing")]
        let _field_span_registry = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Registry", position = pos).entered()
        };
        let registry = read_packable_hash_table::<u32, GeneratorRegistryNode>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_registry);

        Ok(Self {
            registry,
        })
    }
}

impl crate::writers::ACWritable for GeneratorRegistry {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GeneratorRegistry").entered();

        write_packable_hash_table::<u32, GeneratorRegistryNode>(writer, &self.registry)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for GeneratorRegistryNode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GeneratorRegistryNode").entered();

        #[cfg(feature = "tracing")]
        let _field_span_wcid_or_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WcidOrType", position = pos).entered()
        };
        let wcid_or_type = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_wcid_or_type);
        #[cfg(feature = "tracing")]
        let _field_span_ts = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Ts", position = pos).entered()
        };
        let ts = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_ts);
        #[cfg(feature = "tracing")]
        let _field_span_treasure_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TreasureType", position = pos).entered()
        };
        let treasure_type = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_treasure_type);
        #[cfg(feature = "tracing")]
        let _field_span_slot = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Slot", position = pos).entered()
        };
        let slot = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_slot);
        #[cfg(feature = "tracing")]
        let _field_span_checkpointed = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Checkpointed", position = pos).entered()
        };
        let checkpointed = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_checkpointed);
        #[cfg(feature = "tracing")]
        let _field_span_shop = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Shop", position = pos).entered()
        };
        let shop = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_shop);
        #[cfg(feature = "tracing")]
        let _field_span_amount = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Amount", position = pos).entered()
        };
        let amount = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_amount);

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

impl crate::writers::ACWritable for GeneratorRegistryNode {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GeneratorRegistryNode").entered();

        write_u32(writer, self.wcid_or_type)?;
        write_f64(writer, self.ts)?;
        write_u32(writer, self.treasure_type)?;
        write_u32(writer, self.slot)?;
        write_u32(writer, self.checkpointed)?;
        write_u32(writer, self.shop)?;
        write_u32(writer, self.amount)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for GeneratorQueue {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GeneratorQueue").entered();

        #[cfg(feature = "tracing")]
        let _field_span_queue = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Queue", position = pos).entered()
        };
        let queue = read_packable_list::<GeneratorQueueNode>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_queue);

        Ok(Self {
            queue,
        })
    }
}

impl crate::writers::ACWritable for GeneratorQueue {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GeneratorQueue").entered();

        write_packable_list::<GeneratorQueueNode>(writer, &self.queue)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for GeneratorQueueNode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GeneratorQueueNode").entered();

        #[cfg(feature = "tracing")]
        let _field_span_slot = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Slot", position = pos).entered()
        };
        let slot = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_slot);
        #[cfg(feature = "tracing")]
        let _field_span_when = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "When", position = pos).entered()
        };
        let when = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_when);

        Ok(Self {
            slot,
            when,
        })
    }
}

impl crate::writers::ACWritable for GeneratorQueueNode {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GeneratorQueueNode").entered();

        write_u32(writer, self.slot)?;
        write_f64(writer, self.when)?;
        Ok(())
    }
}

impl WindowPropertyType1000007F {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WindowPropertyType1000007F").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WindowPropertyType10000086").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WindowPropertyType10000087").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WindowPropertyType10000088").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WindowPropertyType10000089").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WindowPropertyType1000008A").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WindowPropertyType1000008D").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WindowPropertyType1000008DTitleSourceVariant").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WindowProperty").entered();

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

impl WindowPropertyType1000007F {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WindowPropertyType1000007F").entered();

        write_u32(writer, self.unknown_j)?;
        write_u64(writer, self.value_j)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for WindowPropertyType1000007F {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WindowPropertyType1000007F::write(self, writer)
    }
}

impl WindowPropertyType10000086 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WindowPropertyType10000086").entered();

        write_u32(writer, self.unknown_i)?;
        write_u32(writer, self.value_i)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for WindowPropertyType10000086 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WindowPropertyType10000086::write(self, writer)
    }
}

impl WindowPropertyType10000087 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WindowPropertyType10000087").entered();

        write_u32(writer, self.unknown_h)?;
        write_u32(writer, self.value_h)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for WindowPropertyType10000087 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WindowPropertyType10000087::write(self, writer)
    }
}

impl WindowPropertyType10000088 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WindowPropertyType10000088").entered();

        write_u32(writer, self.unknown_f)?;
        write_u32(writer, self.value_f)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for WindowPropertyType10000088 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WindowPropertyType10000088::write(self, writer)
    }
}

impl WindowPropertyType10000089 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WindowPropertyType10000089").entered();

        write_u32(writer, self.unknown_e)?;
        write_u32(writer, self.value_e)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for WindowPropertyType10000089 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WindowPropertyType10000089::write(self, writer)
    }
}

impl WindowPropertyType1000008A {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WindowPropertyType1000008A").entered();

        write_u32(writer, self.unknown_d)?;
        write_u8(writer, self.value_d)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for WindowPropertyType1000008A {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WindowPropertyType1000008A::write(self, writer)
    }
}

impl WindowPropertyType1000008D {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WindowPropertyType1000008D").entered();

        write_u32(writer, self.unknown_c)?;
        self.title_source.write(writer)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for WindowPropertyType1000008D {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WindowPropertyType1000008D::write(self, writer)
    }
}

impl WindowPropertyType1000008DTitleSourceVariant {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WindowPropertyType1000008DTitleSourceVariant").entered();

        match self {
            Self::Type0(variant_struct) => {
                write_u8(writer, 0x00)?;
                write_u32(writer, variant_struct.string_id)?;
                write_u32(writer, variant_struct.file_id)?;
            },
            Self::Type1(variant_struct) => {
                write_u8(writer, 0x01)?;
                write_wstring(writer, &variant_struct.value_a.0)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for WindowPropertyType1000008DTitleSourceVariant {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WindowPropertyType1000008DTitleSourceVariant::write(self, writer)
    }
}


impl WindowProperty {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WindowProperty").entered();


        match self {
            Self::Type1000007F(variant_struct) => {
                WindowPropertyType1000007F::write(variant_struct, writer)?;
            },
            Self::Type10000086(variant_struct) => {
                WindowPropertyType10000086::write(variant_struct, writer)?;
            },
            Self::Type10000087(variant_struct) => {
                WindowPropertyType10000087::write(variant_struct, writer)?;
            },
            Self::Type10000088(variant_struct) => {
                WindowPropertyType10000088::write(variant_struct, writer)?;
            },
            Self::Type10000089(variant_struct) => {
                WindowPropertyType10000089::write(variant_struct, writer)?;
            },
            Self::Type1000008A(variant_struct) => {
                WindowPropertyType1000008A::write(variant_struct, writer)?;
            },
            Self::Type1000008D(variant_struct) => {
                WindowPropertyType1000008D::write(variant_struct, writer)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for WindowProperty {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WindowProperty::write(self, writer)
    }
}

impl WindowOptionType1000008B {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WindowOptionType1000008B").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WindowOption").entered();

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

impl WindowOptionType1000008B {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WindowOptionType1000008B").entered();

        write_u8(writer, self.unknown_b)?;
        write_u8(writer, self.property_count)?;
        write_vec::<WindowProperty>(writer, &self.properties)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for WindowOptionType1000008B {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WindowOptionType1000008B::write(self, writer)
    }
}

impl WindowOption {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WindowOption").entered();


        match self {
            Self::Type1000008B(variant_struct) => {
                WindowOptionType1000008B::write(variant_struct, writer)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for WindowOption {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        WindowOption::write(self, writer)
    }
}

impl OptionPropertyType10000080 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "OptionPropertyType10000080").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "OptionPropertyType10000081").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "OptionPropertyType1000008C").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "OptionProperty").entered();

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

impl OptionPropertyType10000080 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "OptionPropertyType10000080").entered();

        write_u32(writer, self.unknown_l)?;
        write_f32(writer, self.inactive_opacity)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for OptionPropertyType10000080 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        OptionPropertyType10000080::write(self, writer)
    }
}

impl OptionPropertyType10000081 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "OptionPropertyType10000081").entered();

        write_u32(writer, self.unknown_k)?;
        write_f32(writer, self.active_opacity)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for OptionPropertyType10000081 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        OptionPropertyType10000081::write(self, writer)
    }
}

impl OptionPropertyType1000008C {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "OptionPropertyType1000008C").entered();

        write_u32(writer, self.unknown_a)?;
        write_packable_list::<WindowOption>(writer, &self.window_options)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for OptionPropertyType1000008C {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        OptionPropertyType1000008C::write(self, writer)
    }
}

impl OptionProperty {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "OptionProperty").entered();


        match self {
            Self::Type10000080(variant_struct) => {
                OptionPropertyType10000080::write(variant_struct, writer)?;
            },
            Self::Type10000081(variant_struct) => {
                OptionPropertyType10000081::write(variant_struct, writer)?;
            },
            Self::Type1000008C(variant_struct) => {
                OptionPropertyType1000008C::write(variant_struct, writer)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for OptionProperty {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        OptionProperty::write(self, writer)
    }
}

impl crate::readers::ACDataType for GameplayOptions {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GameplayOptions").entered();

        #[cfg(feature = "tracing")]
        let _field_span_size = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Size", position = pos).entered()
        };
        let size = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_size);
        #[cfg(feature = "tracing")]
        let _field_span_unknown200_2 = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Unknown200_2", position = pos).entered()
        };
        let unknown200_2 = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_unknown200_2);
        #[cfg(feature = "tracing")]
        let _field_span_option_property_count = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "OptionPropertyCount", position = pos).entered()
        };
        let option_property_count = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_option_property_count);
        #[cfg(feature = "tracing")]
        let _field_span_option_properties = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "OptionProperties", position = pos).entered()
        };
        let option_properties = read_vec::<OptionProperty>(reader, option_property_count as usize)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_option_properties);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

        Ok(Self {
            size,
            unknown200_2,
            option_property_count,
            option_properties,
        })
    }
}

impl crate::writers::ACWritable for GameplayOptions {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GameplayOptions").entered();

        write_u32(writer, self.size)?;
        write_u8(writer, self.unknown200_2)?;
        write_u8(writer, self.option_property_count)?;
        write_vec::<OptionProperty>(writer, &self.option_properties)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for PlayerModule {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "PlayerModule").entered();

        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_options = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Options", position = pos).entered()
        };
        let options = Ok::<_, Box<dyn std::error::Error>>(CharacterOptions1::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_options);
        let mut shortcuts = None;
        if (flags & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_shortcuts = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Shortcuts", position = pos).entered()
            };
            shortcuts = Some(read_packable_list::<ShortCutData>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_shortcuts);
        }
        #[cfg(feature = "tracing")]
        let _field_span_tab1_spells = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Tab1Spells", position = pos).entered()
        };
        let tab1_spells = read_packable_list::<LayeredSpellId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tab1_spells);
        #[cfg(feature = "tracing")]
        let _field_span_tab2_spells = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Tab2Spells", position = pos).entered()
        };
        let tab2_spells = read_packable_list::<LayeredSpellId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tab2_spells);
        #[cfg(feature = "tracing")]
        let _field_span_tab3_spells = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Tab3Spells", position = pos).entered()
        };
        let tab3_spells = read_packable_list::<LayeredSpellId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tab3_spells);
        #[cfg(feature = "tracing")]
        let _field_span_tab4_spells = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Tab4Spells", position = pos).entered()
        };
        let tab4_spells = read_packable_list::<LayeredSpellId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tab4_spells);
        #[cfg(feature = "tracing")]
        let _field_span_tab5_spells = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Tab5Spells", position = pos).entered()
        };
        let tab5_spells = read_packable_list::<LayeredSpellId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tab5_spells);
        #[cfg(feature = "tracing")]
        let _field_span_tab6_spells = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Tab6Spells", position = pos).entered()
        };
        let tab6_spells = read_packable_list::<LayeredSpellId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tab6_spells);
        #[cfg(feature = "tracing")]
        let _field_span_tab7_spells = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Tab7Spells", position = pos).entered()
        };
        let tab7_spells = read_packable_list::<LayeredSpellId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tab7_spells);
        #[cfg(feature = "tracing")]
        let _field_span_tab8_spells = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Tab8Spells", position = pos).entered()
        };
        let tab8_spells = read_packable_list::<LayeredSpellId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tab8_spells);
        let mut fill_comps = None;
        if (flags & 0x00000008) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_fill_comps = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "FillComps", position = pos).entered()
            };
            fill_comps = Some(read_packable_hash_table::<u32, u32>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_fill_comps);
        }
        let mut spell_book_filters = None;
        if (flags & 0x00000020) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_spell_book_filters = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SpellBookFilters", position = pos).entered()
            };
            spell_book_filters = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_spell_book_filters);
        }
        let mut option_flags = None;
        if (flags & 0x00000040) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_option_flags = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "OptionFlags", position = pos).entered()
            };
            option_flags = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_option_flags);
        }
        let mut unknown100_1 = None;
        let mut option_strings = None;
        if (flags & 0x00000100) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_unknown100_1 = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Unknown100_1", position = pos).entered()
            };
            unknown100_1 = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_unknown100_1);
            #[cfg(feature = "tracing")]
            let _field_span_option_strings = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "OptionStrings", position = pos).entered()
            };
            option_strings = Some(read_packable_hash_table::<u32, String>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_option_strings);
        }
        let mut gameplay_options = None;
        if (flags & 0x00000200) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_gameplay_options = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "GameplayOptions", position = pos).entered()
            };
            gameplay_options = Some(GameplayOptions::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_gameplay_options);
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

impl crate::writers::ACWritable for PlayerModule {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "PlayerModule").entered();

        write_u32(writer, self.flags)?;
        write_u32(writer, self.options.bits())?;
        if (self.flags & 0x00000001) != 0 {
            if let Some(ref value) = self.shortcuts {
                write_packable_list::<ShortCutData>(writer, &value)?;
            }
        }
        write_packable_list::<LayeredSpellId>(writer, &self.tab1_spells)?;
        write_packable_list::<LayeredSpellId>(writer, &self.tab2_spells)?;
        write_packable_list::<LayeredSpellId>(writer, &self.tab3_spells)?;
        write_packable_list::<LayeredSpellId>(writer, &self.tab4_spells)?;
        write_packable_list::<LayeredSpellId>(writer, &self.tab5_spells)?;
        write_packable_list::<LayeredSpellId>(writer, &self.tab6_spells)?;
        write_packable_list::<LayeredSpellId>(writer, &self.tab7_spells)?;
        write_packable_list::<LayeredSpellId>(writer, &self.tab8_spells)?;
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.fill_comps {
                write_packable_hash_table::<u32, u32>(writer, &value)?;
            }
        }
        if (self.flags & 0x00000020) != 0 {
            if let Some(ref value) = self.spell_book_filters {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000040) != 0 {
            if let Some(ref value) = self.option_flags {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000100) != 0 {
            if let Some(ref value) = self.unknown100_1 {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000100) != 0 {
            if let Some(ref value) = self.option_strings {
                write_packable_hash_table::<u32, String>(writer, &value)?;
            }
        }
        if (self.flags & 0x00000200) != 0 {
            if let Some(ref value) = self.gameplay_options {
                value.write(writer)?;
            }
        }
        Ok(())
    }
}

impl crate::readers::ACDataType for ShortCutManager {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ShortCutManager").entered();

        #[cfg(feature = "tracing")]
        let _field_span_shortcuts = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Shortcuts", position = pos).entered()
        };
        let shortcuts = read_packable_list::<ShortCutData>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_shortcuts);

        Ok(Self {
            shortcuts,
        })
    }
}

impl crate::writers::ACWritable for ShortCutManager {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ShortCutManager").entered();

        write_packable_list::<ShortCutData>(writer, &self.shortcuts)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for ShortCutData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ShortCutData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_index = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Index", position = pos).entered()
        };
        let index = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_index);
        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_spell_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SpellId", position = pos).entered()
        };
        let spell_id = LayeredSpellId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_spell_id);

        Ok(Self {
            index,
            object_id,
            spell_id,
        })
    }
}

impl crate::writers::ACWritable for ShortCutData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ShortCutData").entered();

        write_u32(writer, self.index)?;
        self.object_id.write(writer)?;
        self.spell_id.write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for SpellTab {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SpellTab").entered();

        #[cfg(feature = "tracing")]
        let _field_span_spells = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Spells", position = pos).entered()
        };
        let spells = read_packable_list::<LayeredSpellId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_spells);

        Ok(Self {
            spells,
        })
    }
}

impl crate::writers::ACWritable for SpellTab {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "SpellTab").entered();

        write_packable_list::<LayeredSpellId>(writer, &self.spells)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for ContentProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ContentProfile").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_container_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContainerType", position = pos).entered()
        };
        let container_type = ContainerProperties::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_container_type);

        Ok(Self {
            object_id,
            container_type,
        })
    }
}

impl crate::writers::ACWritable for ContentProfile {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ContentProfile").entered();

        self.object_id.write(writer)?;
        write_u32(writer, self.container_type.clone() as u32)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for InventoryPlacement {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "InventoryPlacement").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_location = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Location", position = pos).entered()
        };
        let location = Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_location);
        #[cfg(feature = "tracing")]
        let _field_span_priority = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Priority", position = pos).entered()
        };
        let priority = Ok::<_, Box<dyn std::error::Error>>(CoverageMask::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_priority);

        Ok(Self {
            object_id,
            location,
            priority,
        })
    }
}

impl crate::writers::ACWritable for InventoryPlacement {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "InventoryPlacement").entered();

        self.object_id.write(writer)?;
        write_u32(writer, self.location.bits())?;
        write_u32(writer, self.priority.bits())?;
        Ok(())
    }
}

impl crate::readers::ACDataType for AllegianceProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceProfile").entered();

        #[cfg(feature = "tracing")]
        let _field_span_total_members = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TotalMembers", position = pos).entered()
        };
        let total_members = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_total_members);
        #[cfg(feature = "tracing")]
        let _field_span_total_vassals = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TotalVassals", position = pos).entered()
        };
        let total_vassals = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_total_vassals);
        #[cfg(feature = "tracing")]
        let _field_span_hierarchy = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Hierarchy", position = pos).entered()
        };
        let hierarchy = AllegianceHierarchy::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_hierarchy);

        Ok(Self {
            total_members,
            total_vassals,
            hierarchy,
        })
    }
}

impl crate::writers::ACWritable for AllegianceProfile {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AllegianceProfile").entered();

        write_u32(writer, self.total_members)?;
        write_u32(writer, self.total_vassals)?;
        self.hierarchy.write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for AllegianceRecord {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceRecord").entered();

        #[cfg(feature = "tracing")]
        let _field_span_tree_parent = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TreeParent", position = pos).entered()
        };
        let tree_parent = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tree_parent);
        #[cfg(feature = "tracing")]
        let _field_span_allegiance_data = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AllegianceData", position = pos).entered()
        };
        let allegiance_data = AllegianceData::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_allegiance_data);

        Ok(Self {
            tree_parent,
            allegiance_data,
        })
    }
}

impl crate::writers::ACWritable for AllegianceRecord {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AllegianceRecord").entered();

        self.tree_parent.write(writer)?;
        self.allegiance_data.write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for AllegianceHierarchy {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceHierarchy").entered();

        #[cfg(feature = "tracing")]
        let _field_span_record_count = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "RecordCount", position = pos).entered()
        };
        let record_count = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_record_count);
        #[cfg(feature = "tracing")]
        let _field_span_old_version = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "OldVersion", position = pos).entered()
        };
        let old_version = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_old_version);
        #[cfg(feature = "tracing")]
        let _field_span_officers = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Officers", position = pos).entered()
        };
        let officers = read_phash_table::<ObjectId, AllegianceOfficerLevel>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_officers);
        #[cfg(feature = "tracing")]
        let _field_span_officer_titles = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "OfficerTitles", position = pos).entered()
        };
        let officer_titles = read_packable_list::<String>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_officer_titles);
        #[cfg(feature = "tracing")]
        let _field_span_monarch_broadcast_time = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MonarchBroadcastTime", position = pos).entered()
        };
        let monarch_broadcast_time = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_monarch_broadcast_time);
        #[cfg(feature = "tracing")]
        let _field_span_monarch_broadcasts_today = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MonarchBroadcastsToday", position = pos).entered()
        };
        let monarch_broadcasts_today = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_monarch_broadcasts_today);
        #[cfg(feature = "tracing")]
        let _field_span_spokes_broadcast_time = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SpokesBroadcastTime", position = pos).entered()
        };
        let spokes_broadcast_time = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_spokes_broadcast_time);
        #[cfg(feature = "tracing")]
        let _field_span_spokes_broadcasts_today = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SpokesBroadcastsToday", position = pos).entered()
        };
        let spokes_broadcasts_today = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_spokes_broadcasts_today);
        #[cfg(feature = "tracing")]
        let _field_span_motd = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Motd", position = pos).entered()
        };
        let motd = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_motd);
        #[cfg(feature = "tracing")]
        let _field_span_motd_set_by = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MotdSetBy", position = pos).entered()
        };
        let motd_set_by = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_motd_set_by);
        #[cfg(feature = "tracing")]
        let _field_span_chat_room_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ChatRoomId", position = pos).entered()
        };
        let chat_room_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_chat_room_id);
        #[cfg(feature = "tracing")]
        let _field_span_bindpoint = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Bindpoint", position = pos).entered()
        };
        let bindpoint = Position::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_bindpoint);
        #[cfg(feature = "tracing")]
        let _field_span_allegiance_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AllegianceName", position = pos).entered()
        };
        let allegiance_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_allegiance_name);
        #[cfg(feature = "tracing")]
        let _field_span_name_last_set_time = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NameLastSetTime", position = pos).entered()
        };
        let name_last_set_time = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name_last_set_time);
        #[cfg(feature = "tracing")]
        let _field_span_is_locked = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "IsLocked", position = pos).entered()
        };
        let is_locked = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_is_locked);
        #[cfg(feature = "tracing")]
        let _field_span_approved_vassal = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ApprovedVassal", position = pos).entered()
        };
        let approved_vassal = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_approved_vassal);
        let mut monarch_data = None;
        if record_count > 0 {
            #[cfg(feature = "tracing")]
            let _field_span_monarch_data = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "MonarchData", position = pos).entered()
            };
            monarch_data = if record_count > 0 { AllegianceData::read(reader).map(Some) } else { Ok(None) }?;
            #[cfg(feature = "tracing")]
            drop(_field_span_monarch_data);
        }
        #[cfg(feature = "tracing")]
        let _field_span_records = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Records", position = pos).entered()
        };
        let records = read_vec::<AllegianceRecord>(reader, record_count as usize - 1)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_records);

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

impl crate::writers::ACWritable for AllegianceHierarchy {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AllegianceHierarchy").entered();

        write_u16(writer, self.record_count)?;
        write_u16(writer, self.old_version)?;
        write_phash_table::<ObjectId, AllegianceOfficerLevel>(writer, &self.officers)?;
        write_packable_list::<String>(writer, &self.officer_titles)?;
        write_u32(writer, self.monarch_broadcast_time)?;
        write_u32(writer, self.monarch_broadcasts_today)?;
        write_u32(writer, self.spokes_broadcast_time)?;
        write_u32(writer, self.spokes_broadcasts_today)?;
        write_string(writer, &self.motd)?;
        write_string(writer, &self.motd_set_by)?;
        write_u32(writer, self.chat_room_id)?;
        self.bindpoint.write(writer)?;
        write_string(writer, &self.allegiance_name)?;
        write_u32(writer, self.name_last_set_time)?;
        write_bool(writer, self.is_locked)?;
        write_i32(writer, self.approved_vassal)?;
        if self.record_count > 0 {
            if let Some(ref value) = self.monarch_data {
                value.write(writer)?;
            }
        }
        write_vec::<AllegianceRecord>(writer, &self.records)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for AllegianceData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_character_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CharacterId", position = pos).entered()
        };
        let character_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_character_id);
        #[cfg(feature = "tracing")]
        let _field_span_xp_cached = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "XPCached", position = pos).entered()
        };
        let xp_cached = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_xp_cached);
        #[cfg(feature = "tracing")]
        let _field_span_xp_tithed = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "XPTithed", position = pos).entered()
        };
        let xp_tithed = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_xp_tithed);
        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_gender = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Gender", position = pos).entered()
        };
        let gender = Gender::try_from(read_u8(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_gender);
        #[cfg(feature = "tracing")]
        let _field_span_heritage = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Heritage", position = pos).entered()
        };
        let heritage = HeritageGroup::try_from(read_u8(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_heritage);
        #[cfg(feature = "tracing")]
        let _field_span_rank = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Rank", position = pos).entered()
        };
        let rank = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_rank);
        let mut level = None;
        if (flags & 0x8) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_level = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Level", position = pos).entered()
            };
            level = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_level);
        }
        #[cfg(feature = "tracing")]
        let _field_span_loyalty = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Loyalty", position = pos).entered()
        };
        let loyalty = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_loyalty);
        #[cfg(feature = "tracing")]
        let _field_span_leadership = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Leadership", position = pos).entered()
        };
        let leadership = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_leadership);
        let mut allegiance_age = None;
        let time_online;
        if flags == 0x4 {
            #[cfg(feature = "tracing")]
            let _field_span_time_online = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "TimeOnline", position = pos).entered()
            };
            time_online = if flags == 0x4 { read_u64(reader).map(Some) } else { Ok(None) }?;
            #[cfg(feature = "tracing")]
            drop(_field_span_time_online);
        } else {
            #[cfg(feature = "tracing")]
            let _field_span_allegiance_age = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "AllegianceAge", position = pos).entered()
            };
            allegiance_age = if flags == 0x4 { read_u32(reader).map(Some) } else { Ok(None) }?;
            #[cfg(feature = "tracing")]
            drop(_field_span_allegiance_age);
            #[cfg(feature = "tracing")]
            let _field_span_time_online = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "TimeOnline", position = pos).entered()
            };
            time_online = if flags == 0x4 { read_u64(reader).map(Some) } else { Ok(None) }?;
            #[cfg(feature = "tracing")]
            drop(_field_span_time_online);
        }
        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);

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

impl crate::writers::ACWritable for AllegianceData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AllegianceData").entered();

        self.character_id.write(writer)?;
        write_u32(writer, self.xp_cached)?;
        write_u32(writer, self.xp_tithed)?;
        write_u32(writer, self.flags)?;
        write_u8(writer, self.gender.clone() as u8)?;
        write_u8(writer, self.heritage.clone() as u8)?;
        write_u16(writer, self.rank)?;
        if (self.flags & 0x8) != 0 {
            if let Some(ref value) = self.level {
                write_u32(writer, *value)?;
            }
        }
        write_u16(writer, self.loyalty)?;
        write_u16(writer, self.leadership)?;
        if self.flags == 0x4 {
            if let Some(ref value) = self.allegiance_age {
                write_u32(writer, *value)?;
            }
        }
        if self.flags == 0x4 {
            if let Some(ref value) = self.time_online {
                write_u64(writer, *value)?;
            }
        }
        write_string(writer, &self.name)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for FriendData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "FriendData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_friend_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "FriendId", position = pos).entered()
        };
        let friend_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_friend_id);
        #[cfg(feature = "tracing")]
        let _field_span_online = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Online", position = pos).entered()
        };
        let online = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_online);
        #[cfg(feature = "tracing")]
        let _field_span_appear_offline = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AppearOffline", position = pos).entered()
        };
        let appear_offline = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_appear_offline);
        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);
        #[cfg(feature = "tracing")]
        let _field_span_out_friends = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "OutFriends", position = pos).entered()
        };
        let out_friends = read_packable_list::<ObjectId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_out_friends);
        #[cfg(feature = "tracing")]
        let _field_span_in_friends = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "InFriends", position = pos).entered()
        };
        let in_friends = read_packable_list::<ObjectId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_in_friends);

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

impl crate::writers::ACWritable for FriendData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "FriendData").entered();

        self.friend_id.write(writer)?;
        write_bool(writer, self.online)?;
        write_bool(writer, self.appear_offline)?;
        write_string(writer, &self.name)?;
        write_packable_list::<ObjectId>(writer, &self.out_friends)?;
        write_packable_list::<ObjectId>(writer, &self.in_friends)?;
        Ok(())
    }
}

impl ItemProfileTypeNeg1 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, packed_amount: uint, object_id: ObjectId) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemProfileTypeNeg1").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemProfileType1").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemProfile").entered();

        let packed_amount = read_u32(reader)?;
        #[allow(unused_variables)]
        let amount = (packed_amount & 0xFFFFFF) as i32;
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

impl ItemProfileTypeNeg1 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ItemProfileTypeNeg1").entered();

        self.weenie_description.write(writer)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for ItemProfileTypeNeg1 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        ItemProfileTypeNeg1::write(self, writer)
    }
}

impl ItemProfileType1 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ItemProfileType1").entered();

        self.old_weenie_description.write(writer)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for ItemProfileType1 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        ItemProfileType1::write(self, writer)
    }
}

impl ItemProfile {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ItemProfile").entered();


        match self {
            Self::TypeNeg1(variant_struct) => {
                ItemProfileTypeNeg1::write(variant_struct, writer)?;
            },
            Self::Type1(variant_struct) => {
                ItemProfileType1::write(variant_struct, writer)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for ItemProfile {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        ItemProfile::write(self, writer)
    }
}

impl crate::readers::ACDataType for PublicWeenieDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "PublicWeenieDesc").entered();

        #[cfg(feature = "tracing")]
        let _field_span_header = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Header", position = pos).entered()
        };
        let header = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_header);
        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);
        #[cfg(feature = "tracing")]
        let _field_span_weenie_class_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WeenieClassId", position = pos).entered()
        };
        let weenie_class_id = PackedDWORD::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_weenie_class_id);
        #[cfg(feature = "tracing")]
        let _field_span_icon = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Icon", position = pos).entered()
        };
        let icon = PackedDWORD::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_icon);
        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = Ok::<_, Box<dyn std::error::Error>>(ItemType::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);
        #[cfg(feature = "tracing")]
        let _field_span_behavior = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Behavior", position = pos).entered()
        };
        let behavior = Ok::<_, Box<dyn std::error::Error>>(ObjectDescriptionFlag::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_behavior);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);
        let mut header2 = None;
        if (behavior.bits() & 0x04000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_header2 = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Header2", position = pos).entered()
            };
            header2 = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_header2);
        }
        let mut plural_name = None;
        if (header & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_plural_name = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "PluralName", position = pos).entered()
            };
            plural_name = Some(read_string(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_plural_name);
        }
        let mut items_capacity = None;
        if (header & 0x00000002) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_items_capacity = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ItemsCapacity", position = pos).entered()
            };
            items_capacity = Some(read_u8(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_items_capacity);
        }
        let mut container_capacity = None;
        if (header & 0x00000004) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_container_capacity = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ContainerCapacity", position = pos).entered()
            };
            container_capacity = Some(read_u8(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_container_capacity);
        }
        let mut ammunition_type = None;
        if (header & 0x00000100) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_ammunition_type = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "AmmunitionType", position = pos).entered()
            };
            ammunition_type = Some(Ok::<_, Box<dyn std::error::Error>>(AmmoType::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_ammunition_type);
        }
        let mut value = None;
        if (header & 0x00000008) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_value = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Value", position = pos).entered()
            };
            value = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_value);
        }
        let mut useability = None;
        if (header & 0x00000010) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_useability = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Useability", position = pos).entered()
            };
            useability = Some(Ok::<_, Box<dyn std::error::Error>>(Usable::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_useability);
        }
        let mut use_radius = None;
        if (header & 0x00000020) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_use_radius = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "UseRadius", position = pos).entered()
            };
            use_radius = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_use_radius);
        }
        let mut target_type = None;
        if (header & 0x00080000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_target_type = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "TargetType", position = pos).entered()
            };
            target_type = Some(Ok::<_, Box<dyn std::error::Error>>(ItemType::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_target_type);
        }
        let mut effects = None;
        if (header & 0x00000080) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_effects = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Effects", position = pos).entered()
            };
            effects = Some(Ok::<_, Box<dyn std::error::Error>>(IconHighlight::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_effects);
        }
        let mut combat_use = None;
        if (header & 0x00000200) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_combat_use = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CombatUse", position = pos).entered()
            };
            combat_use = Some(WieldType::try_from(read_u8(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_combat_use);
        }
        let mut structure = None;
        if (header & 0x00000400) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_structure = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Structure", position = pos).entered()
            };
            structure = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_structure);
        }
        let mut max_structure = None;
        if (header & 0x00000800) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_max_structure = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "MaxStructure", position = pos).entered()
            };
            max_structure = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_max_structure);
        }
        let mut stack_size = None;
        if (header & 0x00001000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_stack_size = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "StackSize", position = pos).entered()
            };
            stack_size = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_stack_size);
        }
        let mut max_stack_size = None;
        if (header & 0x00002000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_max_stack_size = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "MaxStackSize", position = pos).entered()
            };
            max_stack_size = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_max_stack_size);
        }
        let mut container_id = None;
        if (header & 0x00004000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_container_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ContainerId", position = pos).entered()
            };
            container_id = Some(ObjectId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_container_id);
        }
        let mut wielder_id = None;
        if (header & 0x00008000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_wielder_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "WielderId", position = pos).entered()
            };
            wielder_id = Some(ObjectId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_wielder_id);
        }
        let mut valid_slots = None;
        if (header & 0x00010000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_valid_slots = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ValidSlots", position = pos).entered()
            };
            valid_slots = Some(Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_valid_slots);
        }
        let mut slot = None;
        if (header & 0x00020000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_slot = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Slot", position = pos).entered()
            };
            slot = Some(Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_slot);
        }
        let mut priority = None;
        if (header & 0x00040000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_priority = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Priority", position = pos).entered()
            };
            priority = Some(Ok::<_, Box<dyn std::error::Error>>(CoverageMask::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_priority);
        }
        let mut blip_color = None;
        if (header & 0x00100000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_blip_color = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BlipColor", position = pos).entered()
            };
            blip_color = Some(RadarColor::try_from(read_u8(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_blip_color);
        }
        let mut radar_enum = None;
        if (header & 0x00800000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_radar_enum = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "RadarEnum", position = pos).entered()
            };
            radar_enum = Some(RadarBehavior::try_from(read_u8(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_radar_enum);
        }
        let mut physics_script = None;
        if (header & 0x08000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_physics_script = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "PhysicsScript", position = pos).entered()
            };
            physics_script = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_physics_script);
        }
        let mut workmanship = None;
        if (header & 0x01000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_workmanship = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Workmanship", position = pos).entered()
            };
            workmanship = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_workmanship);
        }
        let mut burden = None;
        if (header & 0x00200000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_burden = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Burden", position = pos).entered()
            };
            burden = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_burden);
        }
        let mut spell_id = None;
        if (header & 0x00400000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_spell_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SpellId", position = pos).entered()
            };
            spell_id = Some(SpellId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_spell_id);
        }
        let mut owner_id = None;
        if (header & 0x02000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_owner_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "OwnerId", position = pos).entered()
            };
            owner_id = Some(ObjectId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_owner_id);
        }
        let mut restrictions = None;
        if (header & 0x04000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_restrictions = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Restrictions", position = pos).entered()
            };
            restrictions = Some(RestrictionDB::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_restrictions);
        }
        let mut hook_item_types = None;
        if (header & 0x20000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_hook_item_types = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "HookItemTypes", position = pos).entered()
            };
            hook_item_types = Some(Ok::<_, Box<dyn std::error::Error>>(HookType::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_hook_item_types);
        }
        let mut monarch_id = None;
        if (header & 0x00000040) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_monarch_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "MonarchId", position = pos).entered()
            };
            monarch_id = Some(ObjectId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_monarch_id);
        }
        let mut hook_type = None;
        if (header & 0x10000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_hook_type = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "HookType", position = pos).entered()
            };
            hook_type = Some(Ok::<_, Box<dyn std::error::Error>>(HookType::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_hook_type);
        }
        let mut icon_overlay = None;
        if (header & 0x40000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_icon_overlay = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "IconOverlay", position = pos).entered()
            };
            icon_overlay = Some(PackedDWORD::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_icon_overlay);
        }
        let mut icon_underlay = None;
        if (header2.unwrap_or(0) & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_icon_underlay = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "IconUnderlay", position = pos).entered()
            };
            icon_underlay = Some(PackedDWORD::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_icon_underlay);
        }
        let mut material = None;
        if (header & 0x80000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_material = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Material", position = pos).entered()
            };
            material = Some(MaterialType::try_from(read_u32(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_material);
        }
        let mut cooldown_id = None;
        if (header2.unwrap_or(0) & 0x00000002) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_cooldown_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CooldownId", position = pos).entered()
            };
            cooldown_id = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_cooldown_id);
        }
        let mut cooldown_duration = None;
        if (header2.unwrap_or(0) & 0x00000004) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_cooldown_duration = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CooldownDuration", position = pos).entered()
            };
            cooldown_duration = Some(read_u64(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_cooldown_duration);
        }
        let mut pet_owner_id = None;
        if (header2.unwrap_or(0) & 0x00000008) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_pet_owner_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "PetOwnerId", position = pos).entered()
            };
            pet_owner_id = Some(ObjectId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_pet_owner_id);
        }
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

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

impl crate::writers::ACWritable for PublicWeenieDesc {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "PublicWeenieDesc").entered();

        write_u32(writer, self.header)?;
        write_string(writer, &self.name)?;
        self.weenie_class_id.write(writer)?;
        self.icon.write(writer)?;
        write_u32(writer, self.type_.bits())?;
        write_u32(writer, self.behavior.bits())?;
        align_dword_write(writer)?;
        if (self.behavior.bits() & 0x04000000) != 0 {
            if let Some(ref value) = self.header2 {
                write_u32(writer, *value)?;
            }
        }
        if (self.header & 0x00000001) != 0 {
            if let Some(ref value) = self.plural_name {
                write_string(writer, &value)?;
            }
        }
        if (self.header & 0x00000002) != 0 {
            if let Some(ref value) = self.items_capacity {
                write_u8(writer, *value)?;
            }
        }
        if (self.header & 0x00000004) != 0 {
            if let Some(ref value) = self.container_capacity {
                write_u8(writer, *value)?;
            }
        }
        if (self.header & 0x00000100) != 0 {
            if let Some(ref value) = self.ammunition_type {
                write_u16(writer, value.bits())?;
            }
        }
        if (self.header & 0x00000008) != 0 {
            if let Some(ref value) = self.value {
                write_u32(writer, *value)?;
            }
        }
        if (self.header & 0x00000010) != 0 {
            if let Some(ref value) = self.useability {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00000020) != 0 {
            if let Some(ref value) = self.use_radius {
                write_f32(writer, *value)?;
            }
        }
        if (self.header & 0x00080000) != 0 {
            if let Some(ref value) = self.target_type {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00000080) != 0 {
            if let Some(ref value) = self.effects {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00000200) != 0 {
            if let Some(ref value) = self.combat_use {
                write_u8(writer, value.clone() as u8)?;
            }
        }
        if (self.header & 0x00000400) != 0 {
            if let Some(ref value) = self.structure {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x00000800) != 0 {
            if let Some(ref value) = self.max_structure {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x00001000) != 0 {
            if let Some(ref value) = self.stack_size {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x00002000) != 0 {
            if let Some(ref value) = self.max_stack_size {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x00004000) != 0 {
            if let Some(ref value) = self.container_id {
                value.write(writer)?;
            }
        }
        if (self.header & 0x00008000) != 0 {
            if let Some(ref value) = self.wielder_id {
                value.write(writer)?;
            }
        }
        if (self.header & 0x00010000) != 0 {
            if let Some(ref value) = self.valid_slots {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00020000) != 0 {
            if let Some(ref value) = self.slot {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00040000) != 0 {
            if let Some(ref value) = self.priority {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00100000) != 0 {
            if let Some(ref value) = self.blip_color {
                write_u8(writer, value.clone() as u8)?;
            }
        }
        if (self.header & 0x00800000) != 0 {
            if let Some(ref value) = self.radar_enum {
                write_u8(writer, value.clone() as u8)?;
            }
        }
        if (self.header & 0x08000000) != 0 {
            if let Some(ref value) = self.physics_script {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x01000000) != 0 {
            if let Some(ref value) = self.workmanship {
                write_f32(writer, *value)?;
            }
        }
        if (self.header & 0x00200000) != 0 {
            if let Some(ref value) = self.burden {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x00400000) != 0 {
            if let Some(ref value) = self.spell_id {
                value.write(writer)?;
            }
        }
        if (self.header & 0x02000000) != 0 {
            if let Some(ref value) = self.owner_id {
                value.write(writer)?;
            }
        }
        if (self.header & 0x04000000) != 0 {
            if let Some(ref value) = self.restrictions {
                value.write(writer)?;
            }
        }
        if (self.header & 0x20000000) != 0 {
            if let Some(ref value) = self.hook_item_types {
                write_u16(writer, value.bits())?;
            }
        }
        if (self.header & 0x00000040) != 0 {
            if let Some(ref value) = self.monarch_id {
                value.write(writer)?;
            }
        }
        if (self.header & 0x10000000) != 0 {
            if let Some(ref value) = self.hook_type {
                write_u16(writer, value.bits())?;
            }
        }
        if (self.header & 0x40000000) != 0 {
            if let Some(ref value) = self.icon_overlay {
                value.write(writer)?;
            }
        }
        if (self.header2.unwrap_or(0) & 0x00000001) != 0 {
            if let Some(ref value) = self.icon_underlay {
                value.write(writer)?;
            }
        }
        if (self.header & 0x80000000) != 0 {
            if let Some(ref value) = self.material {
                write_u32(writer, value.clone() as u32)?;
            }
        }
        if (self.header2.unwrap_or(0) & 0x00000002) != 0 {
            if let Some(ref value) = self.cooldown_id {
                write_u32(writer, *value)?;
            }
        }
        if (self.header2.unwrap_or(0) & 0x00000004) != 0 {
            if let Some(ref value) = self.cooldown_duration {
                write_u64(writer, *value)?;
            }
        }
        if (self.header2.unwrap_or(0) & 0x00000008) != 0 {
            if let Some(ref value) = self.pet_owner_id {
                value.write(writer)?;
            }
        }
        align_dword_write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for RestrictionDB {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "RestrictionDB").entered();

        #[cfg(feature = "tracing")]
        let _field_span_version = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Version", position = pos).entered()
        };
        let version = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_version);
        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_monarch_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MonarchId", position = pos).entered()
        };
        let monarch_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_monarch_id);
        #[cfg(feature = "tracing")]
        let _field_span_permissions = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Permissions", position = pos).entered()
        };
        let permissions = read_phash_table::<ObjectId, u32>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_permissions);

        Ok(Self {
            version,
            flags,
            monarch_id,
            permissions,
        })
    }
}

impl crate::writers::ACWritable for RestrictionDB {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "RestrictionDB").entered();

        write_u32(writer, self.version)?;
        write_u32(writer, self.flags)?;
        self.monarch_id.write(writer)?;
        write_phash_table::<ObjectId, u32>(writer, &self.permissions)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for OldPublicWeenieDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "OldPublicWeenieDesc").entered();

        #[cfg(feature = "tracing")]
        let _field_span_header = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Header", position = pos).entered()
        };
        let header = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_header);
        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);
        #[cfg(feature = "tracing")]
        let _field_span_weenie_class_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WeenieClassId", position = pos).entered()
        };
        let weenie_class_id = PackedDWORD::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_weenie_class_id);
        #[cfg(feature = "tracing")]
        let _field_span_icon = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Icon", position = pos).entered()
        };
        let icon = PackedDWORD::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_icon);
        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = Ok::<_, Box<dyn std::error::Error>>(ItemType::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);
        #[cfg(feature = "tracing")]
        let _field_span_bitfield = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Bitfield", position = pos).entered()
        };
        let bitfield = Ok::<_, Box<dyn std::error::Error>>(ObjectDescriptionFlag::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_bitfield);
        let mut plural_name = None;
        if (header & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_plural_name = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "PluralName", position = pos).entered()
            };
            plural_name = Some(read_string(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_plural_name);
        }
        let mut items_capacity = None;
        if (header & 0x00000002) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_items_capacity = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ItemsCapacity", position = pos).entered()
            };
            items_capacity = Some(read_u8(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_items_capacity);
        }
        let mut container_capacity = None;
        if (header & 0x00000004) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_container_capacity = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ContainerCapacity", position = pos).entered()
            };
            container_capacity = Some(read_u8(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_container_capacity);
        }
        let mut value = None;
        if (header & 0x00000008) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_value = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Value", position = pos).entered()
            };
            value = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_value);
        }
        let mut useability = None;
        if (header & 0x00000010) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_useability = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Useability", position = pos).entered()
            };
            useability = Some(Ok::<_, Box<dyn std::error::Error>>(Usable::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_useability);
        }
        let mut use_radius = None;
        if (header & 0x00000020) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_use_radius = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "UseRadius", position = pos).entered()
            };
            use_radius = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_use_radius);
        }
        let mut t_target_type = None;
        if (header & 0x00080000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_t_target_type = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "tTargetType", position = pos).entered()
            };
            t_target_type = Some(Ok::<_, Box<dyn std::error::Error>>(ItemType::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_t_target_type);
        }
        let mut effects = None;
        if (header & 0x00000080) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_effects = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Effects", position = pos).entered()
            };
            effects = Some(Ok::<_, Box<dyn std::error::Error>>(IconHighlight::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_effects);
        }
        let mut ammunition_type = None;
        if (header & 0x00000100) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_ammunition_type = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "AmmunitionType", position = pos).entered()
            };
            ammunition_type = Some(Ok::<_, Box<dyn std::error::Error>>(AmmoType::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_ammunition_type);
        }
        let mut combat_use = None;
        if (header & 0x00000200) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_combat_use = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CombatUse", position = pos).entered()
            };
            combat_use = Some(WieldType::try_from(read_u8(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_combat_use);
        }
        let mut structure = None;
        if (header & 0x00000400) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_structure = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Structure", position = pos).entered()
            };
            structure = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_structure);
        }
        let mut max_structure = None;
        if (header & 0x00000800) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_max_structure = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "MaxStructure", position = pos).entered()
            };
            max_structure = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_max_structure);
        }
        let mut stack_size = None;
        if (header & 0x00001000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_stack_size = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "StackSize", position = pos).entered()
            };
            stack_size = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_stack_size);
        }
        let mut max_stack_size = None;
        if (header & 0x00002000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_max_stack_size = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "MaxStackSize", position = pos).entered()
            };
            max_stack_size = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_max_stack_size);
        }
        let mut container_id = None;
        if (header & 0x00004000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_container_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ContainerId", position = pos).entered()
            };
            container_id = Some(ObjectId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_container_id);
        }
        let mut wielder_id = None;
        if (header & 0x00008000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_wielder_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "WielderId", position = pos).entered()
            };
            wielder_id = Some(ObjectId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_wielder_id);
        }
        let mut valid_slots = None;
        if (header & 0x00010000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_valid_slots = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ValidSlots", position = pos).entered()
            };
            valid_slots = Some(Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_valid_slots);
        }
        let mut slots = None;
        if (header & 0x00020000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_slots = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Slots", position = pos).entered()
            };
            slots = Some(Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_slots);
        }
        let mut priority = None;
        if (header & 0x00040000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_priority = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Priority", position = pos).entered()
            };
            priority = Some(Ok::<_, Box<dyn std::error::Error>>(CoverageMask::from_bits_retain(read_u32(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_priority);
        }
        let mut blip_color = None;
        if (header & 0x00100000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_blip_color = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BlipColor", position = pos).entered()
            };
            blip_color = Some(RadarColor::try_from(read_u8(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_blip_color);
        }
        let mut radar_enum = None;
        if (header & 0x00800000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_radar_enum = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "RadarEnum", position = pos).entered()
            };
            radar_enum = Some(RadarBehavior::try_from(read_u8(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_radar_enum);
        }
        let mut obvious_distance = None;
        if (header & 0x01000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_obvious_distance = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ObviousDistance", position = pos).entered()
            };
            obvious_distance = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_obvious_distance);
        }
        let mut vndwcid = None;
        if (header & 0x00200000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_vndwcid = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Vndwcid", position = pos).entered()
            };
            vndwcid = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_vndwcid);
        }
        let mut spell_id = None;
        if (header & 0x00400000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_spell_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SpellId", position = pos).entered()
            };
            spell_id = Some(SpellId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_spell_id);
        }
        let mut house_owner_id = None;
        if (header & 0x02000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_house_owner_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "HouseOwnerId", position = pos).entered()
            };
            house_owner_id = Some(ObjectId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_house_owner_id);
        }
        let mut physics_script = None;
        if (header & 0x08000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_physics_script = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "PhysicsScript", position = pos).entered()
            };
            physics_script = Some(read_u16(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_physics_script);
        }
        let mut restrictions = None;
        if (header & 0x04000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_restrictions = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Restrictions", position = pos).entered()
            };
            restrictions = Some(RestrictionDB::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_restrictions);
        }
        let mut hook_type = None;
        if (header & 0x10000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_hook_type = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "HookType", position = pos).entered()
            };
            hook_type = Some(Ok::<_, Box<dyn std::error::Error>>(HookType::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_hook_type);
        }
        let mut hook_item_types = None;
        if (header & 0x20000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_hook_item_types = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "HookItemTypes", position = pos).entered()
            };
            hook_item_types = Some(Ok::<_, Box<dyn std::error::Error>>(HookType::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_hook_item_types);
        }
        let mut monarch_id = None;
        if (header & 0x00000040) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_monarch_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "MonarchId", position = pos).entered()
            };
            monarch_id = Some(ObjectId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_monarch_id);
        }
        let mut icon_overlay = None;
        if (header & 0x40000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_icon_overlay = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "IconOverlay", position = pos).entered()
            };
            icon_overlay = Some(PackedDWORD::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_icon_overlay);
        }
        let mut material = None;
        if (header & 0x80000000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_material = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Material", position = pos).entered()
            };
            material = Some(MaterialType::try_from(read_u32(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_material);
        }
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

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

impl crate::writers::ACWritable for OldPublicWeenieDesc {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "OldPublicWeenieDesc").entered();

        write_u32(writer, self.header)?;
        write_string(writer, &self.name)?;
        self.weenie_class_id.write(writer)?;
        self.icon.write(writer)?;
        write_u32(writer, self.type_.bits())?;
        write_u32(writer, self.bitfield.bits())?;
        if (self.header & 0x00000001) != 0 {
            if let Some(ref value) = self.plural_name {
                write_string(writer, &value)?;
            }
        }
        if (self.header & 0x00000002) != 0 {
            if let Some(ref value) = self.items_capacity {
                write_u8(writer, *value)?;
            }
        }
        if (self.header & 0x00000004) != 0 {
            if let Some(ref value) = self.container_capacity {
                write_u8(writer, *value)?;
            }
        }
        if (self.header & 0x00000008) != 0 {
            if let Some(ref value) = self.value {
                write_u32(writer, *value)?;
            }
        }
        if (self.header & 0x00000010) != 0 {
            if let Some(ref value) = self.useability {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00000020) != 0 {
            if let Some(ref value) = self.use_radius {
                write_f32(writer, *value)?;
            }
        }
        if (self.header & 0x00080000) != 0 {
            if let Some(ref value) = self.t_target_type {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00000080) != 0 {
            if let Some(ref value) = self.effects {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00000100) != 0 {
            if let Some(ref value) = self.ammunition_type {
                write_u16(writer, value.bits())?;
            }
        }
        if (self.header & 0x00000200) != 0 {
            if let Some(ref value) = self.combat_use {
                write_u8(writer, value.clone() as u8)?;
            }
        }
        if (self.header & 0x00000400) != 0 {
            if let Some(ref value) = self.structure {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x00000800) != 0 {
            if let Some(ref value) = self.max_structure {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x00001000) != 0 {
            if let Some(ref value) = self.stack_size {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x00002000) != 0 {
            if let Some(ref value) = self.max_stack_size {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x00004000) != 0 {
            if let Some(ref value) = self.container_id {
                value.write(writer)?;
            }
        }
        if (self.header & 0x00008000) != 0 {
            if let Some(ref value) = self.wielder_id {
                value.write(writer)?;
            }
        }
        if (self.header & 0x00010000) != 0 {
            if let Some(ref value) = self.valid_slots {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00020000) != 0 {
            if let Some(ref value) = self.slots {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00040000) != 0 {
            if let Some(ref value) = self.priority {
                write_u32(writer, value.bits())?;
            }
        }
        if (self.header & 0x00100000) != 0 {
            if let Some(ref value) = self.blip_color {
                write_u8(writer, value.clone() as u8)?;
            }
        }
        if (self.header & 0x00800000) != 0 {
            if let Some(ref value) = self.radar_enum {
                write_u8(writer, value.clone() as u8)?;
            }
        }
        if (self.header & 0x01000000) != 0 {
            if let Some(ref value) = self.obvious_distance {
                write_f32(writer, *value)?;
            }
        }
        if (self.header & 0x00200000) != 0 {
            if let Some(ref value) = self.vndwcid {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x00400000) != 0 {
            if let Some(ref value) = self.spell_id {
                value.write(writer)?;
            }
        }
        if (self.header & 0x02000000) != 0 {
            if let Some(ref value) = self.house_owner_id {
                value.write(writer)?;
            }
        }
        if (self.header & 0x08000000) != 0 {
            if let Some(ref value) = self.physics_script {
                write_u16(writer, *value)?;
            }
        }
        if (self.header & 0x04000000) != 0 {
            if let Some(ref value) = self.restrictions {
                value.write(writer)?;
            }
        }
        if (self.header & 0x10000000) != 0 {
            if let Some(ref value) = self.hook_type {
                write_u16(writer, value.bits())?;
            }
        }
        if (self.header & 0x20000000) != 0 {
            if let Some(ref value) = self.hook_item_types {
                write_u16(writer, value.bits())?;
            }
        }
        if (self.header & 0x00000040) != 0 {
            if let Some(ref value) = self.monarch_id {
                value.write(writer)?;
            }
        }
        if (self.header & 0x40000000) != 0 {
            if let Some(ref value) = self.icon_overlay {
                value.write(writer)?;
            }
        }
        if (self.header & 0x80000000) != 0 {
            if let Some(ref value) = self.material {
                write_u32(writer, value.clone() as u32)?;
            }
        }
        align_dword_write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for Trade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Trade").entered();

        #[cfg(feature = "tracing")]
        let _field_span_partner_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PartnerId", position = pos).entered()
        };
        let partner_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_partner_id);
        #[cfg(feature = "tracing")]
        let _field_span_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Sequence", position = pos).entered()
        };
        let sequence = read_u64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_status = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Status", position = pos).entered()
        };
        let status = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_status);
        #[cfg(feature = "tracing")]
        let _field_span_initiator_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "InitiatorId", position = pos).entered()
        };
        let initiator_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_initiator_id);
        #[cfg(feature = "tracing")]
        let _field_span_accepted = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Accepted", position = pos).entered()
        };
        let accepted = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_accepted);
        #[cfg(feature = "tracing")]
        let _field_span_partner_accepted = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PartnerAccepted", position = pos).entered()
        };
        let partner_accepted = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_partner_accepted);

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

impl crate::writers::ACWritable for Trade {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Trade").entered();

        self.partner_id.write(writer)?;
        write_u64(writer, self.sequence)?;
        write_u32(writer, self.status)?;
        self.initiator_id.write(writer)?;
        write_bool(writer, self.accepted)?;
        write_bool(writer, self.partner_accepted)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for JumpPack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "JumpPack").entered();

        #[cfg(feature = "tracing")]
        let _field_span_extent = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Extent", position = pos).entered()
        };
        let extent = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_extent);
        #[cfg(feature = "tracing")]
        let _field_span_velocity = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Velocity", position = pos).entered()
        };
        let velocity = Vector3::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_velocity);
        #[cfg(feature = "tracing")]
        let _field_span_object_instance_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectInstanceSequence", position = pos).entered()
        };
        let object_instance_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_instance_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_server_control_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectServerControlSequence", position = pos).entered()
        };
        let object_server_control_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_server_control_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_teleport_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectTeleportSequence", position = pos).entered()
        };
        let object_teleport_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_teleport_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_force_position_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectForcePositionSequence", position = pos).entered()
        };
        let object_force_position_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_force_position_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

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

impl crate::writers::ACWritable for JumpPack {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "JumpPack").entered();

        write_f32(writer, self.extent)?;
        self.velocity.write(writer)?;
        write_u16(writer, self.object_instance_sequence)?;
        write_u16(writer, self.object_server_control_sequence)?;
        write_u16(writer, self.object_teleport_sequence)?;
        write_u16(writer, self.object_force_position_sequence)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for MoveToStatePack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MoveToStatePack").entered();

        #[cfg(feature = "tracing")]
        let _field_span_raw_motion_state = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "RawMotionState", position = pos).entered()
        };
        let raw_motion_state = RawMotionState::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_raw_motion_state);
        #[cfg(feature = "tracing")]
        let _field_span_position = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Position", position = pos).entered()
        };
        let position = Position::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_position);
        #[cfg(feature = "tracing")]
        let _field_span_object_instance_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectInstanceSequence", position = pos).entered()
        };
        let object_instance_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_instance_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_server_control_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectServerControlSequence", position = pos).entered()
        };
        let object_server_control_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_server_control_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_teleport_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectTeleportSequence", position = pos).entered()
        };
        let object_teleport_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_teleport_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_force_position_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectForcePositionSequence", position = pos).entered()
        };
        let object_force_position_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_force_position_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_contact = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Contact", position = pos).entered()
        };
        let contact = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_contact);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

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

impl crate::writers::ACWritable for MoveToStatePack {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MoveToStatePack").entered();

        self.raw_motion_state.write(writer)?;
        self.position.write(writer)?;
        write_u16(writer, self.object_instance_sequence)?;
        write_u16(writer, self.object_server_control_sequence)?;
        write_u16(writer, self.object_teleport_sequence)?;
        write_u16(writer, self.object_force_position_sequence)?;
        write_u8(writer, self.contact)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for PackedMotionCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "PackedMotionCommand").entered();

        #[cfg(feature = "tracing")]
        let _field_span_command_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CommandId", position = pos).entered()
        };
        let command_id = Command::try_from(read_u16(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_command_id);
        #[cfg(feature = "tracing")]
        let _field_span_packed_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PackedSequence", position = pos).entered()
        };
        let packed_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_packed_sequence);
        #[allow(unused_variables)]
        let server_action_sequence = (packed_sequence & 0x7FFF) as u16;
        #[allow(unused_variables)]
        let autonomous = ((packed_sequence >> 15) & 0x1) as u16;
        #[cfg(feature = "tracing")]
        let _field_span_speed = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Speed", position = pos).entered()
        };
        let speed = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_speed);

        Ok(Self {
            command_id,
            packed_sequence,
            speed,
        })
    }
}

impl crate::writers::ACWritable for PackedMotionCommand {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "PackedMotionCommand").entered();

        write_u16(writer, self.command_id.clone() as u16)?;
        write_u16(writer, self.packed_sequence)?;
        write_f32(writer, self.speed)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for RawMotionState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "RawMotionState").entered();

        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        let command_list_length = ((flags >> 11) & 0xF8) as u16;
        let mut current_holdkey = None;
        if (flags & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_current_holdkey = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CurrentHoldkey", position = pos).entered()
            };
            current_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_current_holdkey);
        }
        let mut current_style = None;
        if (flags & 0x00000002) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_current_style = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CurrentStyle", position = pos).entered()
            };
            current_style = Some(StanceMode::try_from(read_u16(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_current_style);
        }
        let mut forward_command = None;
        if (flags & 0x00000004) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_forward_command = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ForwardCommand", position = pos).entered()
            };
            forward_command = Some(Command::try_from(read_u16(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_forward_command);
        }
        let mut forward_holdkey = None;
        if (flags & 0x0000008) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_forward_holdkey = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ForwardHoldkey", position = pos).entered()
            };
            forward_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_forward_holdkey);
        }
        let mut forward_speed = None;
        if (flags & 0x00000010) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_forward_speed = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ForwardSpeed", position = pos).entered()
            };
            forward_speed = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_forward_speed);
        }
        let mut sidestep_command = None;
        if (flags & 0x00000020) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_sidestep_command = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SidestepCommand", position = pos).entered()
            };
            sidestep_command = Some(Command::try_from(read_u16(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_sidestep_command);
        }
        let mut sidestep_holdkey = None;
        if (flags & 0x00000040) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_sidestep_holdkey = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SidestepHoldkey", position = pos).entered()
            };
            sidestep_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_sidestep_holdkey);
        }
        let mut sidestep_speed = None;
        if (flags & 0x00000080) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_sidestep_speed = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SidestepSpeed", position = pos).entered()
            };
            sidestep_speed = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_sidestep_speed);
        }
        let mut turn_command = None;
        if (flags & 0x00000100) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_turn_command = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "TurnCommand", position = pos).entered()
            };
            turn_command = Some(Command::try_from(read_u16(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_turn_command);
        }
        let mut turn_holdkey = None;
        if (flags & 0x00000200) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_turn_holdkey = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "TurnHoldkey", position = pos).entered()
            };
            turn_holdkey = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_turn_holdkey);
        }
        let mut turn_speed = None;
        if (flags & 0x00000400) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_turn_speed = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "TurnSpeed", position = pos).entered()
            };
            turn_speed = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_turn_speed);
        }
        #[cfg(feature = "tracing")]
        let _field_span_commands = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Commands", position = pos).entered()
        };
        let commands = read_vec::<PackedMotionCommand>(reader, command_list_length as usize)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_commands);

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

impl crate::writers::ACWritable for RawMotionState {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "RawMotionState").entered();

        write_u32(writer, self.flags)?;
        if (self.flags & 0x00000001) != 0 {
            if let Some(ref value) = self.current_holdkey {
                write_u32(writer, value.clone() as u32)?;
            }
        }
        if (self.flags & 0x00000002) != 0 {
            if let Some(ref value) = self.current_style {
                write_u16(writer, value.clone() as u16)?;
            }
        }
        if (self.flags & 0x00000004) != 0 {
            if let Some(ref value) = self.forward_command {
                write_u16(writer, value.clone() as u16)?;
            }
        }
        if (self.flags & 0x0000008) != 0 {
            if let Some(ref value) = self.forward_holdkey {
                write_u32(writer, value.clone() as u32)?;
            }
        }
        if (self.flags & 0x00000010) != 0 {
            if let Some(ref value) = self.forward_speed {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000020) != 0 {
            if let Some(ref value) = self.sidestep_command {
                write_u16(writer, value.clone() as u16)?;
            }
        }
        if (self.flags & 0x00000040) != 0 {
            if let Some(ref value) = self.sidestep_holdkey {
                write_u32(writer, value.clone() as u32)?;
            }
        }
        if (self.flags & 0x00000080) != 0 {
            if let Some(ref value) = self.sidestep_speed {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000100) != 0 {
            if let Some(ref value) = self.turn_command {
                write_u16(writer, value.clone() as u16)?;
            }
        }
        if (self.flags & 0x00000200) != 0 {
            if let Some(ref value) = self.turn_holdkey {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000400) != 0 {
            if let Some(ref value) = self.turn_speed {
                write_f32(writer, *value)?;
            }
        }
        write_vec::<PackedMotionCommand>(writer, &self.commands)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for AutonomousPositionPack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AutonomousPositionPack").entered();

        #[cfg(feature = "tracing")]
        let _field_span_position = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Position", position = pos).entered()
        };
        let position = Position::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_position);
        #[cfg(feature = "tracing")]
        let _field_span_object_instance_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectInstanceSequence", position = pos).entered()
        };
        let object_instance_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_instance_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_server_control_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectServerControlSequence", position = pos).entered()
        };
        let object_server_control_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_server_control_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_teleport_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectTeleportSequence", position = pos).entered()
        };
        let object_teleport_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_teleport_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_force_position_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectForcePositionSequence", position = pos).entered()
        };
        let object_force_position_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_force_position_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_contact = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Contact", position = pos).entered()
        };
        let contact = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_contact);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

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

impl crate::writers::ACWritable for AutonomousPositionPack {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AutonomousPositionPack").entered();

        self.position.write(writer)?;
        write_u16(writer, self.object_instance_sequence)?;
        write_u16(writer, self.object_server_control_sequence)?;
        write_u16(writer, self.object_teleport_sequence)?;
        write_u16(writer, self.object_force_position_sequence)?;
        write_u8(writer, self.contact)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for PositionPack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "PositionPack").entered();

        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = Ok::<_, Box<dyn std::error::Error>>(PositionFlags::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_origin = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Origin", position = pos).entered()
        };
        let origin = Origin::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_origin);
        let mut w_quat = None;
        if (flags.bits() & 0x00000008) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_w_quat = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "WQuat", position = pos).entered()
            };
            w_quat = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_w_quat);
        }
        let mut x_quat = None;
        if (flags.bits() & 0x00000010) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_x_quat = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "XQuat", position = pos).entered()
            };
            x_quat = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_x_quat);
        }
        let mut y_quat = None;
        if (flags.bits() & 0x00000020) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_y_quat = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "YQuat", position = pos).entered()
            };
            y_quat = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_y_quat);
        }
        let mut z_quat = None;
        if (flags.bits() & 0x00000040) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_z_quat = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ZQuat", position = pos).entered()
            };
            z_quat = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_z_quat);
        }
        let mut velocity = None;
        if (flags.bits() & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_velocity = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Velocity", position = pos).entered()
            };
            velocity = Some(Vector3::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_velocity);
        }
        let mut placement_id = None;
        if (flags.bits() & 0x00000002) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_placement_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "PlacementId", position = pos).entered()
            };
            placement_id = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_placement_id);
        }
        #[cfg(feature = "tracing")]
        let _field_span_object_instance_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectInstanceSequence", position = pos).entered()
        };
        let object_instance_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_instance_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_position_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectPositionSequence", position = pos).entered()
        };
        let object_position_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_position_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_teleport_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectTeleportSequence", position = pos).entered()
        };
        let object_teleport_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_teleport_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_force_position_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectForcePositionSequence", position = pos).entered()
        };
        let object_force_position_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_force_position_sequence);

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

impl crate::writers::ACWritable for PositionPack {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "PositionPack").entered();

        write_u32(writer, self.flags.bits())?;
        self.origin.write(writer)?;
        if (self.flags.bits() & 0x00000008) != 0 {
            if let Some(ref value) = self.w_quat {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags.bits() & 0x00000010) != 0 {
            if let Some(ref value) = self.x_quat {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags.bits() & 0x00000020) != 0 {
            if let Some(ref value) = self.y_quat {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags.bits() & 0x00000040) != 0 {
            if let Some(ref value) = self.z_quat {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags.bits() & 0x00000001) != 0 {
            if let Some(ref value) = self.velocity {
                value.write(writer)?;
            }
        }
        if (self.flags.bits() & 0x00000002) != 0 {
            if let Some(ref value) = self.placement_id {
                write_u32(writer, *value)?;
            }
        }
        write_u16(writer, self.object_instance_sequence)?;
        write_u16(writer, self.object_position_sequence)?;
        write_u16(writer, self.object_teleport_sequence)?;
        write_u16(writer, self.object_force_position_sequence)?;
        Ok(())
    }
}

impl MovementDataType0 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, object_movement_sequence: ushort, object_server_control_sequence: ushort, autonomous: ushort, option_flags: MovementOption, stance: StanceMode) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementDataType0").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementDataType6").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementDataType7").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementDataType8").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementDataType9").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementData").entered();

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

impl MovementDataType0 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementDataType0").entered();

        self.state.write(writer)?;
        if (self.option_flags.clone() as u32 & 0x01) != 0 {
            if let Some(ref value) = self.sticky_object {
                value.write(writer)?;
            }
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for MovementDataType0 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        MovementDataType0::write(self, writer)
    }
}

impl MovementDataType6 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementDataType6").entered();

        self.target.write(writer)?;
        self.origin.write(writer)?;
        self.move_to_params.write(writer)?;
        write_f32(writer, self.my_run_rate)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for MovementDataType6 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        MovementDataType6::write(self, writer)
    }
}

impl MovementDataType7 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementDataType7").entered();

        self.origin.write(writer)?;
        self.move_to_params.write(writer)?;
        write_f32(writer, self.my_run_rate)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for MovementDataType7 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        MovementDataType7::write(self, writer)
    }
}

impl MovementDataType8 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementDataType8").entered();

        self.target_id.write(writer)?;
        write_f32(writer, self.desired_heading)?;
        self.turn_to_params.write(writer)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for MovementDataType8 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        MovementDataType8::write(self, writer)
    }
}

impl MovementDataType9 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementDataType9").entered();

        self.turn_to_params.write(writer)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for MovementDataType9 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        MovementDataType9::write(self, writer)
    }
}

impl MovementData {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementData").entered();


        match self {
            Self::Type0(variant_struct) => {
                MovementDataType0::write(variant_struct, writer)?;
            },
            Self::Type6(variant_struct) => {
                MovementDataType6::write(variant_struct, writer)?;
            },
            Self::Type7(variant_struct) => {
                MovementDataType7::write(variant_struct, writer)?;
            },
            Self::Type8(variant_struct) => {
                MovementDataType8::write(variant_struct, writer)?;
            },
            Self::Type9(variant_struct) => {
                MovementDataType9::write(variant_struct, writer)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for MovementData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        MovementData::write(self, writer)
    }
}

impl crate::readers::ACDataType for InterpretedMotionState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "InterpretedMotionState").entered();

        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        let command_list_length = ((flags >> 7) & 0x7F) as u32;
        let mut current_style = None;
        if (flags & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_current_style = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CurrentStyle", position = pos).entered()
            };
            current_style = Some(StanceMode::try_from(read_u16(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_current_style);
        }
        let mut forward_command = None;
        if (flags & 0x00000002) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_forward_command = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ForwardCommand", position = pos).entered()
            };
            forward_command = Some(Command::try_from(read_u16(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_forward_command);
        }
        let mut sidestep_command = None;
        if (flags & 0x00000008) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_sidestep_command = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SidestepCommand", position = pos).entered()
            };
            sidestep_command = Some(Command::try_from(read_u16(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_sidestep_command);
        }
        let mut turn_command = None;
        if (flags & 0x00000020) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_turn_command = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "TurnCommand", position = pos).entered()
            };
            turn_command = Some(Command::try_from(read_u16(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_turn_command);
        }
        let mut forward_speed = None;
        if (flags & 0x00000004) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_forward_speed = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ForwardSpeed", position = pos).entered()
            };
            forward_speed = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_forward_speed);
        }
        let mut sidestep_speed = None;
        if (flags & 0x00000010) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_sidestep_speed = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SidestepSpeed", position = pos).entered()
            };
            sidestep_speed = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_sidestep_speed);
        }
        let mut turn_speed = None;
        if (flags & 0x00000040) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_turn_speed = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "TurnSpeed", position = pos).entered()
            };
            turn_speed = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_turn_speed);
        }
        #[cfg(feature = "tracing")]
        let _field_span_commands = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Commands", position = pos).entered()
        };
        let commands = read_vec::<PackedMotionCommand>(reader, command_list_length as usize)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_commands);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

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

impl crate::writers::ACWritable for InterpretedMotionState {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "InterpretedMotionState").entered();

        write_u32(writer, self.flags)?;
        if (self.flags & 0x00000001) != 0 {
            if let Some(ref value) = self.current_style {
                write_u16(writer, value.clone() as u16)?;
            }
        }
        if (self.flags & 0x00000002) != 0 {
            if let Some(ref value) = self.forward_command {
                write_u16(writer, value.clone() as u16)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.sidestep_command {
                write_u16(writer, value.clone() as u16)?;
            }
        }
        if (self.flags & 0x00000020) != 0 {
            if let Some(ref value) = self.turn_command {
                write_u16(writer, value.clone() as u16)?;
            }
        }
        if (self.flags & 0x00000004) != 0 {
            if let Some(ref value) = self.forward_speed {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000010) != 0 {
            if let Some(ref value) = self.sidestep_speed {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000040) != 0 {
            if let Some(ref value) = self.turn_speed {
                write_f32(writer, *value)?;
            }
        }
        write_vec::<PackedMotionCommand>(writer, &self.commands)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for DDDRevision {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "DDDRevision").entered();

        #[cfg(feature = "tracing")]
        let _field_span_id_dat_file = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "IdDatFile", position = pos).entered()
        };
        let id_dat_file = read_u64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_id_dat_file);
        #[allow(unused_variables)]
        let dat_file_type = (id_dat_file >> 32) as u32;
        #[cfg(feature = "tracing")]
        let _field_span_iteration = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Iteration", position = pos).entered()
        };
        let iteration = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_iteration);
        #[cfg(feature = "tracing")]
        let _field_span_ids_to_download = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "IdsToDownload", position = pos).entered()
        };
        let ids_to_download = read_packable_list::<DataId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_ids_to_download);
        #[cfg(feature = "tracing")]
        let _field_span_ids_to_purge = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "IdsToPurge", position = pos).entered()
        };
        let ids_to_purge = read_packable_list::<DataId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_ids_to_purge);

        Ok(Self {
            id_dat_file,
            iteration,
            ids_to_download,
            ids_to_purge,
        })
    }
}

impl crate::writers::ACWritable for DDDRevision {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "DDDRevision").entered();

        write_u64(writer, self.id_dat_file)?;
        write_u32(writer, self.iteration)?;
        write_packable_list::<DataId>(writer, &self.ids_to_download)?;
        write_packable_list::<DataId>(writer, &self.ids_to_purge)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for MoveToMovementParameters {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MoveToMovementParameters").entered();

        #[cfg(feature = "tracing")]
        let _field_span_bitmember = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Bitmember", position = pos).entered()
        };
        let bitmember = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_bitmember);
        #[cfg(feature = "tracing")]
        let _field_span_distance_to_object = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DistanceToObject", position = pos).entered()
        };
        let distance_to_object = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_distance_to_object);
        #[cfg(feature = "tracing")]
        let _field_span_min_distance = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MinDistance", position = pos).entered()
        };
        let min_distance = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_min_distance);
        #[cfg(feature = "tracing")]
        let _field_span_fail_distance = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "FailDistance", position = pos).entered()
        };
        let fail_distance = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_fail_distance);
        #[cfg(feature = "tracing")]
        let _field_span_animation_speed = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AnimationSpeed", position = pos).entered()
        };
        let animation_speed = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_animation_speed);
        #[cfg(feature = "tracing")]
        let _field_span_walk_run_threshold = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WalkRunThreshold", position = pos).entered()
        };
        let walk_run_threshold = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_walk_run_threshold);
        #[cfg(feature = "tracing")]
        let _field_span_desired_heading = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DesiredHeading", position = pos).entered()
        };
        let desired_heading = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_desired_heading);

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

impl crate::writers::ACWritable for MoveToMovementParameters {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MoveToMovementParameters").entered();

        write_u32(writer, self.bitmember)?;
        write_f32(writer, self.distance_to_object)?;
        write_f32(writer, self.min_distance)?;
        write_f32(writer, self.fail_distance)?;
        write_f32(writer, self.animation_speed)?;
        write_f32(writer, self.walk_run_threshold)?;
        write_f32(writer, self.desired_heading)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for TurnToMovementParameters {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "TurnToMovementParameters").entered();

        #[cfg(feature = "tracing")]
        let _field_span_bitmember = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Bitmember", position = pos).entered()
        };
        let bitmember = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_bitmember);
        #[cfg(feature = "tracing")]
        let _field_span_animation_speed = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AnimationSpeed", position = pos).entered()
        };
        let animation_speed = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_animation_speed);
        #[cfg(feature = "tracing")]
        let _field_span_desired_heading = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DesiredHeading", position = pos).entered()
        };
        let desired_heading = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_desired_heading);

        Ok(Self {
            bitmember,
            animation_speed,
            desired_heading,
        })
    }
}

impl crate::writers::ACWritable for TurnToMovementParameters {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "TurnToMovementParameters").entered();

        write_u32(writer, self.bitmember)?;
        write_f32(writer, self.animation_speed)?;
        write_f32(writer, self.desired_heading)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for ObjDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ObjDesc").entered();

        #[cfg(feature = "tracing")]
        let _field_span_version = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Version", position = pos).entered()
        };
        let version = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_version);
        #[cfg(feature = "tracing")]
        let _field_span_palette_count = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PaletteCount", position = pos).entered()
        };
        let palette_count = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_palette_count);
        #[cfg(feature = "tracing")]
        let _field_span_texture_count = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TextureCount", position = pos).entered()
        };
        let texture_count = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_texture_count);
        #[cfg(feature = "tracing")]
        let _field_span_model_count = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ModelCount", position = pos).entered()
        };
        let model_count = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_model_count);
        let mut palette = None;
        if palette_count > 0 {
            #[cfg(feature = "tracing")]
            let _field_span_palette = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Palette", position = pos).entered()
            };
            palette = if palette_count > 0 { read_packed_dword(reader).map(DataId).map(Some) } else { Ok(None) }?;
            #[cfg(feature = "tracing")]
            drop(_field_span_palette);
        }
        #[cfg(feature = "tracing")]
        let _field_span_subpalettes = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Subpalettes", position = pos).entered()
        };
        let subpalettes = read_vec::<Subpalette>(reader, palette_count as usize)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_subpalettes);
        #[cfg(feature = "tracing")]
        let _field_span_tm_changes = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TMChanges", position = pos).entered()
        };
        let tm_changes = read_vec::<TextureMapChange>(reader, texture_count as usize)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tm_changes);
        #[cfg(feature = "tracing")]
        let _field_span_ap_changes = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "APChanges", position = pos).entered()
        };
        let ap_changes = read_vec::<AnimPartChange>(reader, model_count as usize)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_ap_changes);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

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

impl crate::writers::ACWritable for ObjDesc {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ObjDesc").entered();

        write_u8(writer, self.version)?;
        write_u8(writer, self.palette_count)?;
        write_u8(writer, self.texture_count)?;
        write_u8(writer, self.model_count)?;
        if self.palette_count > 0 {
            if let Some(ref value) = self.palette {
                write_packed_dword(writer, value.0)?;
            }
        }
        write_vec::<Subpalette>(writer, &self.subpalettes)?;
        write_vec::<TextureMapChange>(writer, &self.tm_changes)?;
        write_vec::<AnimPartChange>(writer, &self.ap_changes)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for Subpalette {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Subpalette").entered();

        #[cfg(feature = "tracing")]
        let _field_span_palette = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Palette", position = pos).entered()
        };
        let palette = read_packed_dword(reader).map(DataId)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_palette);
        #[cfg(feature = "tracing")]
        let _field_span_offset = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Offset", position = pos).entered()
        };
        let offset = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_offset);
        #[cfg(feature = "tracing")]
        let _field_span_num_colors = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NumColors", position = pos).entered()
        };
        let num_colors = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_num_colors);

        Ok(Self {
            palette,
            offset,
            num_colors,
        })
    }
}

impl crate::writers::ACWritable for Subpalette {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Subpalette").entered();

        write_packed_dword(writer, self.palette.0)?;
        write_u8(writer, self.offset)?;
        write_u8(writer, self.num_colors)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for TextureMapChange {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "TextureMapChange").entered();

        #[cfg(feature = "tracing")]
        let _field_span_part_index = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PartIndex", position = pos).entered()
        };
        let part_index = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_part_index);
        #[cfg(feature = "tracing")]
        let _field_span_old_tex_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "OldTexId", position = pos).entered()
        };
        let old_tex_id = read_packed_dword(reader).map(DataId)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_old_tex_id);
        #[cfg(feature = "tracing")]
        let _field_span_new_tex_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NewTexId", position = pos).entered()
        };
        let new_tex_id = read_packed_dword(reader).map(DataId)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_new_tex_id);

        Ok(Self {
            part_index,
            old_tex_id,
            new_tex_id,
        })
    }
}

impl crate::writers::ACWritable for TextureMapChange {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "TextureMapChange").entered();

        write_u8(writer, self.part_index)?;
        write_packed_dword(writer, self.old_tex_id.0)?;
        write_packed_dword(writer, self.new_tex_id.0)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for AnimPartChange {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AnimPartChange").entered();

        #[cfg(feature = "tracing")]
        let _field_span_part_index = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PartIndex", position = pos).entered()
        };
        let part_index = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_part_index);
        #[cfg(feature = "tracing")]
        let _field_span_part_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PartId", position = pos).entered()
        };
        let part_id = read_packed_dword(reader).map(DataId)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_part_id);

        Ok(Self {
            part_index,
            part_id,
        })
    }
}

impl crate::writers::ACWritable for AnimPartChange {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AnimPartChange").entered();

        write_u8(writer, self.part_index)?;
        write_packed_dword(writer, self.part_id.0)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for CharGenResult {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharGenResult").entered();

        #[cfg(feature = "tracing")]
        let _field_span_account = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Account", position = pos).entered()
        };
        let account = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_account);
        #[cfg(feature = "tracing")]
        let _field_span_one = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "One", position = pos).entered()
        };
        let one = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_one);
        #[cfg(feature = "tracing")]
        let _field_span_heritage_group = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HeritageGroup", position = pos).entered()
        };
        let heritage_group = HeritageGroup::try_from(read_u8(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_heritage_group);
        #[cfg(feature = "tracing")]
        let _field_span_gender = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Gender", position = pos).entered()
        };
        let gender = Gender::try_from(read_u8(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_gender);
        #[cfg(feature = "tracing")]
        let _field_span_eyes_strip = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "EyesStrip", position = pos).entered()
        };
        let eyes_strip = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_eyes_strip);
        #[cfg(feature = "tracing")]
        let _field_span_nose_strip = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NoseStrip", position = pos).entered()
        };
        let nose_strip = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_nose_strip);
        #[cfg(feature = "tracing")]
        let _field_span_mouth_strip = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MouthStrip", position = pos).entered()
        };
        let mouth_strip = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_mouth_strip);
        #[cfg(feature = "tracing")]
        let _field_span_hair_color = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HairColor", position = pos).entered()
        };
        let hair_color = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_hair_color);
        #[cfg(feature = "tracing")]
        let _field_span_eye_color = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "EyeColor", position = pos).entered()
        };
        let eye_color = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_eye_color);
        #[cfg(feature = "tracing")]
        let _field_span_hair_style = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HairStyle", position = pos).entered()
        };
        let hair_style = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_hair_style);
        #[cfg(feature = "tracing")]
        let _field_span_headgear_style = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HeadgearStyle", position = pos).entered()
        };
        let headgear_style = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_headgear_style);
        #[cfg(feature = "tracing")]
        let _field_span_headgear_color = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HeadgearColor", position = pos).entered()
        };
        let headgear_color = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_headgear_color);
        #[cfg(feature = "tracing")]
        let _field_span_shirt_style = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ShirtStyle", position = pos).entered()
        };
        let shirt_style = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_shirt_style);
        #[cfg(feature = "tracing")]
        let _field_span_shirt_color = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ShirtColor", position = pos).entered()
        };
        let shirt_color = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_shirt_color);
        #[cfg(feature = "tracing")]
        let _field_span_trousers_style = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TrousersStyle", position = pos).entered()
        };
        let trousers_style = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_trousers_style);
        #[cfg(feature = "tracing")]
        let _field_span_trousers_color = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TrousersColor", position = pos).entered()
        };
        let trousers_color = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_trousers_color);
        #[cfg(feature = "tracing")]
        let _field_span_footwear_style = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "FootwearStyle", position = pos).entered()
        };
        let footwear_style = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_footwear_style);
        #[cfg(feature = "tracing")]
        let _field_span_footwear_color = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "FootwearColor", position = pos).entered()
        };
        let footwear_color = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_footwear_color);
        #[cfg(feature = "tracing")]
        let _field_span_skin_shade = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SkinShade", position = pos).entered()
        };
        let skin_shade = read_u64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_skin_shade);
        #[cfg(feature = "tracing")]
        let _field_span_hair_shade = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HairShade", position = pos).entered()
        };
        let hair_shade = read_u64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_hair_shade);
        #[cfg(feature = "tracing")]
        let _field_span_headgear_shade = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HeadgearShade", position = pos).entered()
        };
        let headgear_shade = read_u64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_headgear_shade);
        #[cfg(feature = "tracing")]
        let _field_span_shirt_shade = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ShirtShade", position = pos).entered()
        };
        let shirt_shade = read_u64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_shirt_shade);
        #[cfg(feature = "tracing")]
        let _field_span_trousers_shade = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TrousersShade", position = pos).entered()
        };
        let trousers_shade = read_u64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_trousers_shade);
        #[cfg(feature = "tracing")]
        let _field_span_tootwear_shade = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TootwearShade", position = pos).entered()
        };
        let tootwear_shade = read_u64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tootwear_shade);
        #[cfg(feature = "tracing")]
        let _field_span_template_num = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TemplateNum", position = pos).entered()
        };
        let template_num = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_template_num);
        #[cfg(feature = "tracing")]
        let _field_span_strength = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Strength", position = pos).entered()
        };
        let strength = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_strength);
        #[cfg(feature = "tracing")]
        let _field_span_endurance = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Endurance", position = pos).entered()
        };
        let endurance = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_endurance);
        #[cfg(feature = "tracing")]
        let _field_span_coordination = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Coordination", position = pos).entered()
        };
        let coordination = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_coordination);
        #[cfg(feature = "tracing")]
        let _field_span_quickness = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Quickness", position = pos).entered()
        };
        let quickness = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_quickness);
        #[cfg(feature = "tracing")]
        let _field_span_focus = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Focus", position = pos).entered()
        };
        let focus = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_focus);
        #[cfg(feature = "tracing")]
        let _field_span_self_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Self", position = pos).entered()
        };
        let self_ = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_self_);
        #[cfg(feature = "tracing")]
        let _field_span_slot = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Slot", position = pos).entered()
        };
        let slot = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_slot);
        #[cfg(feature = "tracing")]
        let _field_span_class_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ClassId", position = pos).entered()
        };
        let class_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_class_id);
        #[cfg(feature = "tracing")]
        let _field_span_skills = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Skills", position = pos).entered()
        };
        let skills = read_packable_list::<SkillAdvancementClass>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_skills);
        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);
        #[cfg(feature = "tracing")]
        let _field_span_start_area = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "StartArea", position = pos).entered()
        };
        let start_area = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_start_area);
        #[cfg(feature = "tracing")]
        let _field_span_is_admin = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "IsAdmin", position = pos).entered()
        };
        let is_admin = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_is_admin);
        #[cfg(feature = "tracing")]
        let _field_span_is_envoy = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "IsEnvoy", position = pos).entered()
        };
        let is_envoy = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_is_envoy);
        #[cfg(feature = "tracing")]
        let _field_span_validation = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Validation", position = pos).entered()
        };
        let validation = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_validation);

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

impl crate::writers::ACWritable for CharGenResult {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharGenResult").entered();

        write_string(writer, &self.account)?;
        write_u32(writer, self.one)?;
        write_u8(writer, self.heritage_group.clone() as u8)?;
        write_u8(writer, self.gender.clone() as u8)?;
        write_u32(writer, self.eyes_strip)?;
        write_u32(writer, self.nose_strip)?;
        write_u32(writer, self.mouth_strip)?;
        write_u32(writer, self.hair_color)?;
        write_u32(writer, self.eye_color)?;
        write_u32(writer, self.hair_style)?;
        write_u32(writer, self.headgear_style)?;
        write_u32(writer, self.headgear_color)?;
        write_u32(writer, self.shirt_style)?;
        write_u32(writer, self.shirt_color)?;
        write_u32(writer, self.trousers_style)?;
        write_u32(writer, self.trousers_color)?;
        write_u32(writer, self.footwear_style)?;
        write_u32(writer, self.footwear_color)?;
        write_u64(writer, self.skin_shade)?;
        write_u64(writer, self.hair_shade)?;
        write_u64(writer, self.headgear_shade)?;
        write_u64(writer, self.shirt_shade)?;
        write_u64(writer, self.trousers_shade)?;
        write_u64(writer, self.tootwear_shade)?;
        write_u32(writer, self.template_num)?;
        write_u32(writer, self.strength)?;
        write_u32(writer, self.endurance)?;
        write_u32(writer, self.coordination)?;
        write_u32(writer, self.quickness)?;
        write_u32(writer, self.focus)?;
        write_u32(writer, self.self_)?;
        write_u32(writer, self.slot)?;
        write_u32(writer, self.class_id)?;
        write_packable_list::<SkillAdvancementClass>(writer, &self.skills)?;
        write_string(writer, &self.name)?;
        write_u32(writer, self.start_area)?;
        write_u32(writer, self.is_admin)?;
        write_u32(writer, self.is_envoy)?;
        write_u32(writer, self.validation)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for CharacterIdentity {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterIdentity").entered();

        #[cfg(feature = "tracing")]
        let _field_span_character_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CharacterId", position = pos).entered()
        };
        let character_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_character_id);
        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);
        #[cfg(feature = "tracing")]
        let _field_span_seconds_greyed_out = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SecondsGreyedOut", position = pos).entered()
        };
        let seconds_greyed_out = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_seconds_greyed_out);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

        Ok(Self {
            character_id,
            name,
            seconds_greyed_out,
        })
    }
}

impl crate::writers::ACWritable for CharacterIdentity {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterIdentity").entered();

        self.character_id.write(writer)?;
        write_string(writer, &self.name)?;
        write_u32(writer, self.seconds_greyed_out)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for EquipLocation {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EquipLocation").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_slot = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Slot", position = pos).entered()
        };
        let slot = Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_slot);

        Ok(Self {
            object_id,
            slot,
        })
    }
}

impl crate::writers::ACWritable for EquipLocation {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EquipLocation").entered();

        self.object_id.write(writer)?;
        write_u32(writer, self.slot.bits())?;
        Ok(())
    }
}

impl crate::readers::ACDataType for PhysicsDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "PhysicsDesc").entered();

        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_state = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "State", position = pos).entered()
        };
        let state = Ok::<_, Box<dyn std::error::Error>>(PhysicsState::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_state);
        let mut movement_buffer = None;
        let mut autonomous = None;
        if (flags & 0x00010000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_movement_buffer = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "MovementBuffer", position = pos).entered()
            };
            movement_buffer = Some(read_packable_list::<u8>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_movement_buffer);
            #[cfg(feature = "tracing")]
            let _field_span_autonomous = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Autonomous", position = pos).entered()
            };
            autonomous = Some(read_bool(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_autonomous);
        }
        let mut animation_frame = None;
        if (flags & 0x00020000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_animation_frame = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "AnimationFrame", position = pos).entered()
            };
            animation_frame = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_animation_frame);
        }
        let mut position = None;
        if (flags & 0x00008000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_position = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Position", position = pos).entered()
            };
            position = Some(Position::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_position);
        }
        let mut motion_id = None;
        if (flags & 0x00000002) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_motion_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "MotionId", position = pos).entered()
            };
            motion_id = Some(DataId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_motion_id);
        }
        let mut sound_id = None;
        if (flags & 0x00000800) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_sound_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SoundId", position = pos).entered()
            };
            sound_id = Some(DataId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_sound_id);
        }
        let mut physics_script_id = None;
        if (flags & 0x00001000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_physics_script_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "PhysicsScriptId", position = pos).entered()
            };
            physics_script_id = Some(DataId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_physics_script_id);
        }
        let mut setup_id = None;
        if (flags & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_setup_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SetupId", position = pos).entered()
            };
            setup_id = Some(DataId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_setup_id);
        }
        let mut parent_id = None;
        let mut parent_location = None;
        if (flags & 0x00000020) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_parent_id = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ParentId", position = pos).entered()
            };
            parent_id = Some(ObjectId::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_parent_id);
            #[cfg(feature = "tracing")]
            let _field_span_parent_location = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ParentLocation", position = pos).entered()
            };
            parent_location = Some(ParentLocation::try_from(read_u32(reader)?)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_parent_location);
        }
        let mut children = None;
        if (flags & 0x00000040) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_children = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Children", position = pos).entered()
            };
            children = Some(read_packable_list::<EquipLocation>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_children);
        }
        let mut scale = None;
        if (flags & 0x00000080) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_scale = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Scale", position = pos).entered()
            };
            scale = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_scale);
        }
        let mut friction = None;
        if (flags & 0x00000100) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_friction = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Friction", position = pos).entered()
            };
            friction = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_friction);
        }
        let mut elasticity = None;
        if (flags & 0x00000200) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_elasticity = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Elasticity", position = pos).entered()
            };
            elasticity = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_elasticity);
        }
        let mut translucency = None;
        if (flags & 0x00040000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_translucency = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Translucency", position = pos).entered()
            };
            translucency = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_translucency);
        }
        let mut velocity = None;
        if (flags & 0x00000004) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_velocity = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Velocity", position = pos).entered()
            };
            velocity = Some(Vector3::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_velocity);
        }
        let mut acceleration = None;
        if (flags & 0x00000008) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_acceleration = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Acceleration", position = pos).entered()
            };
            acceleration = Some(Vector3::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_acceleration);
        }
        let mut omega = None;
        if (flags & 0x00000010) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_omega = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Omega", position = pos).entered()
            };
            omega = Some(Vector3::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_omega);
        }
        let mut default_script = None;
        if (flags & 0x00002000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_default_script = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "DefaultScript", position = pos).entered()
            };
            default_script = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_default_script);
        }
        let mut default_script_intensity = None;
        if (flags & 0x00004000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_default_script_intensity = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "DefaultScriptIntensity", position = pos).entered()
            };
            default_script_intensity = Some(read_f32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_default_script_intensity);
        }
        #[cfg(feature = "tracing")]
        let _field_span_object_position_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectPositionSequence", position = pos).entered()
        };
        let object_position_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_position_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_movement_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectMovementSequence", position = pos).entered()
        };
        let object_movement_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_movement_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_state_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectStateSequence", position = pos).entered()
        };
        let object_state_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_state_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_vector_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectVectorSequence", position = pos).entered()
        };
        let object_vector_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_vector_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_teleport_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectTeleportSequence", position = pos).entered()
        };
        let object_teleport_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_teleport_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_server_control_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectServerControlSequence", position = pos).entered()
        };
        let object_server_control_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_server_control_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_force_position_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectForcePositionSequence", position = pos).entered()
        };
        let object_force_position_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_force_position_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_visual_desc_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectVisualDescSequence", position = pos).entered()
        };
        let object_visual_desc_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_visual_desc_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_instance_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectInstanceSequence", position = pos).entered()
        };
        let object_instance_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_instance_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

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

impl crate::writers::ACWritable for PhysicsDesc {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "PhysicsDesc").entered();

        write_u32(writer, self.flags)?;
        write_u32(writer, self.state.bits())?;
        if (self.flags & 0x00010000) != 0 {
            if let Some(ref value) = self.movement_buffer {
                write_packable_list::<u8>(writer, &value)?;
            }
        }
        if (self.flags & 0x00010000) != 0 {
            if let Some(ref value) = self.autonomous {
                write_bool(writer, *value)?;
            }
        }
        if (self.flags & 0x00020000) != 0 {
            if let Some(ref value) = self.animation_frame {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00008000) != 0 {
            if let Some(ref value) = self.position {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000002) != 0 {
            if let Some(ref value) = self.motion_id {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000800) != 0 {
            if let Some(ref value) = self.sound_id {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00001000) != 0 {
            if let Some(ref value) = self.physics_script_id {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000001) != 0 {
            if let Some(ref value) = self.setup_id {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000020) != 0 {
            if let Some(ref value) = self.parent_id {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000020) != 0 {
            if let Some(ref value) = self.parent_location {
                write_u32(writer, value.clone() as u32)?;
            }
        }
        if (self.flags & 0x00000040) != 0 {
            if let Some(ref value) = self.children {
                write_packable_list::<EquipLocation>(writer, &value)?;
            }
        }
        if (self.flags & 0x00000080) != 0 {
            if let Some(ref value) = self.scale {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000100) != 0 {
            if let Some(ref value) = self.friction {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000200) != 0 {
            if let Some(ref value) = self.elasticity {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags & 0x00040000) != 0 {
            if let Some(ref value) = self.translucency {
                write_f32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000004) != 0 {
            if let Some(ref value) = self.velocity {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.acceleration {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00000010) != 0 {
            if let Some(ref value) = self.omega {
                value.write(writer)?;
            }
        }
        if (self.flags & 0x00002000) != 0 {
            if let Some(ref value) = self.default_script {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00004000) != 0 {
            if let Some(ref value) = self.default_script_intensity {
                write_f32(writer, *value)?;
            }
        }
        write_u16(writer, self.object_position_sequence)?;
        write_u16(writer, self.object_movement_sequence)?;
        write_u16(writer, self.object_state_sequence)?;
        write_u16(writer, self.object_vector_sequence)?;
        write_u16(writer, self.object_teleport_sequence)?;
        write_u16(writer, self.object_server_control_sequence)?;
        write_u16(writer, self.object_force_position_sequence)?;
        write_u16(writer, self.object_visual_desc_sequence)?;
        write_u16(writer, self.object_instance_sequence)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for AdminAccountData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AdminAccountData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_account_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AccountName", position = pos).entered()
        };
        let account_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_account_name);
        #[cfg(feature = "tracing")]
        let _field_span_bookie_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BookieId", position = pos).entered()
        };
        let bookie_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_bookie_id);

        Ok(Self {
            account_name,
            bookie_id,
        })
    }
}

impl crate::writers::ACWritable for AdminAccountData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AdminAccountData").entered();

        write_string(writer, &self.account_name)?;
        write_u32(writer, self.bookie_id)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for AdminPlayerData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AdminPlayerData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);
        #[cfg(feature = "tracing")]
        let _field_span_bookie_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "bookieId", position = pos).entered()
        };
        let bookie_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_bookie_id);

        Ok(Self {
            name,
            bookie_id,
        })
    }
}

impl crate::writers::ACWritable for AdminPlayerData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AdminPlayerData").entered();

        write_string(writer, &self.name)?;
        write_u32(writer, self.bookie_id)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for VendorProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "VendorProfile").entered();

        #[cfg(feature = "tracing")]
        let _field_span_categories = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Categories", position = pos).entered()
        };
        let categories = Ok::<_, Box<dyn std::error::Error>>(ItemType::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_categories);
        #[cfg(feature = "tracing")]
        let _field_span_min_value = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MinValue", position = pos).entered()
        };
        let min_value = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_min_value);
        #[cfg(feature = "tracing")]
        let _field_span_max_value = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxValue", position = pos).entered()
        };
        let max_value = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_value);
        #[cfg(feature = "tracing")]
        let _field_span_deals_magic = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DealsMagic", position = pos).entered()
        };
        let deals_magic = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_deals_magic);
        #[cfg(feature = "tracing")]
        let _field_span_buy_price = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BuyPrice", position = pos).entered()
        };
        let buy_price = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_buy_price);
        #[cfg(feature = "tracing")]
        let _field_span_sell_price = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SellPrice", position = pos).entered()
        };
        let sell_price = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sell_price);
        #[cfg(feature = "tracing")]
        let _field_span_currency_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CurrencyId", position = pos).entered()
        };
        let currency_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_currency_id);
        #[cfg(feature = "tracing")]
        let _field_span_currency_amount = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CurrencyAmount", position = pos).entered()
        };
        let currency_amount = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_currency_amount);
        #[cfg(feature = "tracing")]
        let _field_span_currency_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CurrencyName", position = pos).entered()
        };
        let currency_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_currency_name);

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

impl crate::writers::ACWritable for VendorProfile {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "VendorProfile").entered();

        write_u32(writer, self.categories.bits())?;
        write_u32(writer, self.min_value)?;
        write_u32(writer, self.max_value)?;
        write_bool(writer, self.deals_magic)?;
        write_f32(writer, self.buy_price)?;
        write_f32(writer, self.sell_price)?;
        write_u32(writer, self.currency_id)?;
        write_u32(writer, self.currency_amount)?;
        write_string(writer, &self.currency_name)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for ArmorProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ArmorProfile").entered();

        #[cfg(feature = "tracing")]
        let _field_span_prot_slashing = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ProtSlashing", position = pos).entered()
        };
        let prot_slashing = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_prot_slashing);
        #[cfg(feature = "tracing")]
        let _field_span_prot_piercing = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ProtPiercing", position = pos).entered()
        };
        let prot_piercing = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_prot_piercing);
        #[cfg(feature = "tracing")]
        let _field_span_prot_bludgeoning = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ProtBludgeoning", position = pos).entered()
        };
        let prot_bludgeoning = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_prot_bludgeoning);
        #[cfg(feature = "tracing")]
        let _field_span_prot_cold = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ProtCold", position = pos).entered()
        };
        let prot_cold = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_prot_cold);
        #[cfg(feature = "tracing")]
        let _field_span_prot_fire = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ProtFire", position = pos).entered()
        };
        let prot_fire = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_prot_fire);
        #[cfg(feature = "tracing")]
        let _field_span_prot_acid = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ProtAcid", position = pos).entered()
        };
        let prot_acid = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_prot_acid);
        #[cfg(feature = "tracing")]
        let _field_span_prot_nether = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ProtNether", position = pos).entered()
        };
        let prot_nether = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_prot_nether);
        #[cfg(feature = "tracing")]
        let _field_span_prot_lightning = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ProtLightning", position = pos).entered()
        };
        let prot_lightning = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_prot_lightning);

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

impl crate::writers::ACWritable for ArmorProfile {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ArmorProfile").entered();

        write_f32(writer, self.prot_slashing)?;
        write_f32(writer, self.prot_piercing)?;
        write_f32(writer, self.prot_bludgeoning)?;
        write_f32(writer, self.prot_cold)?;
        write_f32(writer, self.prot_fire)?;
        write_f32(writer, self.prot_acid)?;
        write_f32(writer, self.prot_nether)?;
        write_f32(writer, self.prot_lightning)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for CreatureAppraisalProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CreatureAppraisalProfile").entered();

        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_health = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Health", position = pos).entered()
        };
        let health = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_health);
        #[cfg(feature = "tracing")]
        let _field_span_health_max = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HealthMax", position = pos).entered()
        };
        let health_max = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_health_max);
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
            #[cfg(feature = "tracing")]
            let _field_span_strength = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Strength", position = pos).entered()
            };
            strength = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_strength);
            #[cfg(feature = "tracing")]
            let _field_span_endurance = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Endurance", position = pos).entered()
            };
            endurance = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_endurance);
            #[cfg(feature = "tracing")]
            let _field_span_quickness = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Quickness", position = pos).entered()
            };
            quickness = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_quickness);
            #[cfg(feature = "tracing")]
            let _field_span_coordination = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Coordination", position = pos).entered()
            };
            coordination = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_coordination);
            #[cfg(feature = "tracing")]
            let _field_span_focus = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Focus", position = pos).entered()
            };
            focus = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_focus);
            #[cfg(feature = "tracing")]
            let _field_span_self_ = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Self", position = pos).entered()
            };
            self_ = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_self_);
            #[cfg(feature = "tracing")]
            let _field_span_stamina = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Stamina", position = pos).entered()
            };
            stamina = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_stamina);
            #[cfg(feature = "tracing")]
            let _field_span_mana = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Mana", position = pos).entered()
            };
            mana = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_mana);
            #[cfg(feature = "tracing")]
            let _field_span_stamina_max = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "StaminaMax", position = pos).entered()
            };
            stamina_max = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_stamina_max);
            #[cfg(feature = "tracing")]
            let _field_span_mana_max = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ManaMax", position = pos).entered()
            };
            mana_max = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_mana_max);
        }
        let mut attr_highlight = None;
        let mut attr_color = None;
        if (flags & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_attr_highlight = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "AttrHighlight", position = pos).entered()
            };
            attr_highlight = Some(Ok::<_, Box<dyn std::error::Error>>(AttributeMask::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_attr_highlight);
            #[cfg(feature = "tracing")]
            let _field_span_attr_color = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "AttrColor", position = pos).entered()
            };
            attr_color = Some(Ok::<_, Box<dyn std::error::Error>>(AttributeMask::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_attr_color);
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

impl crate::writers::ACWritable for CreatureAppraisalProfile {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CreatureAppraisalProfile").entered();

        write_u32(writer, self.flags)?;
        write_u32(writer, self.health)?;
        write_u32(writer, self.health_max)?;
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.strength {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.endurance {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.quickness {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.coordination {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.focus {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.self_ {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.stamina {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.mana {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.stamina_max {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000008) != 0 {
            if let Some(ref value) = self.mana_max {
                write_u32(writer, *value)?;
            }
        }
        if (self.flags & 0x00000001) != 0 {
            if let Some(ref value) = self.attr_highlight {
                write_u16(writer, value.bits())?;
            }
        }
        if (self.flags & 0x00000001) != 0 {
            if let Some(ref value) = self.attr_color {
                write_u16(writer, value.bits())?;
            }
        }
        Ok(())
    }
}

impl crate::readers::ACDataType for WeaponProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "WeaponProfile").entered();

        #[cfg(feature = "tracing")]
        let _field_span_damage_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DamageType", position = pos).entered()
        };
        let damage_type = Ok::<_, Box<dyn std::error::Error>>(DamageType::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_damage_type);
        #[cfg(feature = "tracing")]
        let _field_span_speed = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Speed", position = pos).entered()
        };
        let speed = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_speed);
        #[cfg(feature = "tracing")]
        let _field_span_skill = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Skill", position = pos).entered()
        };
        let skill = SkillId::try_from(read_i32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_skill);
        #[cfg(feature = "tracing")]
        let _field_span_damage = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Damage", position = pos).entered()
        };
        let damage = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_damage);
        #[cfg(feature = "tracing")]
        let _field_span_variance = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Variance", position = pos).entered()
        };
        let variance = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_variance);
        #[cfg(feature = "tracing")]
        let _field_span_modifier = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Modifier", position = pos).entered()
        };
        let modifier = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_modifier);
        #[cfg(feature = "tracing")]
        let _field_span_length = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Length", position = pos).entered()
        };
        let length = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_length);
        #[cfg(feature = "tracing")]
        let _field_span_max_velocity = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxVelocity", position = pos).entered()
        };
        let max_velocity = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_velocity);
        #[cfg(feature = "tracing")]
        let _field_span_offsense = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Offsense", position = pos).entered()
        };
        let offsense = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_offsense);
        #[cfg(feature = "tracing")]
        let _field_span_max_velocity_estimated = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxVelocityEstimated", position = pos).entered()
        };
        let max_velocity_estimated = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_velocity_estimated);

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

impl crate::writers::ACWritable for WeaponProfile {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "WeaponProfile").entered();

        write_u32(writer, self.damage_type.bits())?;
        write_u32(writer, self.speed)?;
        write_i32(writer, self.skill.clone() as i32)?;
        write_u32(writer, self.damage)?;
        write_f64(writer, self.variance)?;
        write_f64(writer, self.modifier)?;
        write_f64(writer, self.length)?;
        write_f64(writer, self.max_velocity)?;
        write_f64(writer, self.offsense)?;
        write_u32(writer, self.max_velocity_estimated)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for HookAppraisalProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HookAppraisalProfile").entered();

        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = Ok::<_, Box<dyn std::error::Error>>(HookAppraisalFlags::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_valid_locations = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ValidLocations", position = pos).entered()
        };
        let valid_locations = Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_valid_locations);
        #[cfg(feature = "tracing")]
        let _field_span_ammo_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AmmoType", position = pos).entered()
        };
        let ammo_type = Ok::<_, Box<dyn std::error::Error>>(AmmoType::from_bits_retain(read_u16(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_ammo_type);

        Ok(Self {
            flags,
            valid_locations,
            ammo_type,
        })
    }
}

impl crate::writers::ACWritable for HookAppraisalProfile {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HookAppraisalProfile").entered();

        write_u32(writer, self.flags.bits())?;
        write_u32(writer, self.valid_locations.bits())?;
        write_u16(writer, self.ammo_type.bits())?;
        Ok(())
    }
}

impl crate::readers::ACDataType for SquelchDB {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SquelchDB").entered();

        #[cfg(feature = "tracing")]
        let _field_span_account_hash = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AccountHash", position = pos).entered()
        };
        let account_hash = read_packable_hash_table::<String, u32>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_account_hash);
        #[cfg(feature = "tracing")]
        let _field_span_character_hash = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CharacterHash", position = pos).entered()
        };
        let character_hash = read_packable_hash_table::<ObjectId, SquelchInfo>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_character_hash);
        #[cfg(feature = "tracing")]
        let _field_span_global_info = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "GlobalInfo", position = pos).entered()
        };
        let global_info = SquelchInfo::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_global_info);

        Ok(Self {
            account_hash,
            character_hash,
            global_info,
        })
    }
}

impl crate::writers::ACWritable for SquelchDB {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "SquelchDB").entered();

        write_packable_hash_table::<String, u32>(writer, &self.account_hash)?;
        write_packable_hash_table::<ObjectId, SquelchInfo>(writer, &self.character_hash)?;
        self.global_info.write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for SquelchInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SquelchInfo").entered();

        #[cfg(feature = "tracing")]
        let _field_span_filters = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Filters", position = pos).entered()
        };
        let filters = read_packable_list::<LogTextType>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_filters);
        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);
        #[cfg(feature = "tracing")]
        let _field_span_account = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Account", position = pos).entered()
        };
        let account = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_account);

        Ok(Self {
            filters,
            name,
            account,
        })
    }
}

impl crate::writers::ACWritable for SquelchInfo {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "SquelchInfo").entered();

        write_packable_list::<LogTextType>(writer, &self.filters)?;
        write_string(writer, &self.name)?;
        write_bool(writer, self.account)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for HouseProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseProfile").entered();

        #[cfg(feature = "tracing")]
        let _field_span_dwelling_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DwellingId", position = pos).entered()
        };
        let dwelling_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_dwelling_id);
        #[cfg(feature = "tracing")]
        let _field_span_owner_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "OwnerId", position = pos).entered()
        };
        let owner_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_owner_id);
        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = HouseBitfield::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_min_level = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MinLevel", position = pos).entered()
        };
        let min_level = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_min_level);
        #[cfg(feature = "tracing")]
        let _field_span_max_level = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxLevel", position = pos).entered()
        };
        let max_level = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_level);
        #[cfg(feature = "tracing")]
        let _field_span_min_alleg_rank = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MinAllegRank", position = pos).entered()
        };
        let min_alleg_rank = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_min_alleg_rank);
        #[cfg(feature = "tracing")]
        let _field_span_max_alleg_rank = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxAllegRank", position = pos).entered()
        };
        let max_alleg_rank = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_alleg_rank);
        #[cfg(feature = "tracing")]
        let _field_span_maintenance_free = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaintenanceFree", position = pos).entered()
        };
        let maintenance_free = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_maintenance_free);
        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = HouseType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);
        #[cfg(feature = "tracing")]
        let _field_span_owner_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "OwnerName", position = pos).entered()
        };
        let owner_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_owner_name);
        #[cfg(feature = "tracing")]
        let _field_span_buy = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Buy", position = pos).entered()
        };
        let buy = read_packable_list::<HousePayment>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_buy);
        #[cfg(feature = "tracing")]
        let _field_span_rent = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Rent", position = pos).entered()
        };
        let rent = read_packable_list::<HousePayment>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_rent);

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

impl crate::writers::ACWritable for HouseProfile {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HouseProfile").entered();

        write_u32(writer, self.dwelling_id)?;
        self.owner_id.write(writer)?;
        write_u32(writer, self.flags.clone() as u32)?;
        write_i32(writer, self.min_level)?;
        write_i32(writer, self.max_level)?;
        write_i32(writer, self.min_alleg_rank)?;
        write_i32(writer, self.max_alleg_rank)?;
        write_bool(writer, self.maintenance_free)?;
        write_u32(writer, self.type_.clone() as u32)?;
        write_string(writer, &self.owner_name)?;
        write_packable_list::<HousePayment>(writer, &self.buy)?;
        write_packable_list::<HousePayment>(writer, &self.rent)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for HousePayment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HousePayment").entered();

        #[cfg(feature = "tracing")]
        let _field_span_required = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Required", position = pos).entered()
        };
        let required = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_required);
        #[cfg(feature = "tracing")]
        let _field_span_paid = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Paid", position = pos).entered()
        };
        let paid = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_paid);
        #[cfg(feature = "tracing")]
        let _field_span_weenie_class_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WeenieClassId", position = pos).entered()
        };
        let weenie_class_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_weenie_class_id);
        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);
        #[cfg(feature = "tracing")]
        let _field_span_plural_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PluralName", position = pos).entered()
        };
        let plural_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_plural_name);

        Ok(Self {
            required,
            paid,
            weenie_class_id,
            name,
            plural_name,
        })
    }
}

impl crate::writers::ACWritable for HousePayment {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HousePayment").entered();

        write_u32(writer, self.required)?;
        write_u32(writer, self.paid)?;
        write_u32(writer, self.weenie_class_id)?;
        write_string(writer, &self.name)?;
        write_string(writer, &self.plural_name)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for HouseData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_buy_time = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BuyTime", position = pos).entered()
        };
        let buy_time = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_buy_time);
        #[cfg(feature = "tracing")]
        let _field_span_rent_time = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "RentTime", position = pos).entered()
        };
        let rent_time = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_rent_time);
        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = HouseType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);
        #[cfg(feature = "tracing")]
        let _field_span_maintenance_free = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaintenanceFree", position = pos).entered()
        };
        let maintenance_free = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_maintenance_free);
        #[cfg(feature = "tracing")]
        let _field_span_buy = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Buy", position = pos).entered()
        };
        let buy = read_packable_list::<HousePayment>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_buy);
        #[cfg(feature = "tracing")]
        let _field_span_rent = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Rent", position = pos).entered()
        };
        let rent = read_packable_list::<HousePayment>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_rent);
        #[cfg(feature = "tracing")]
        let _field_span_position = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Position", position = pos).entered()
        };
        let position = Position::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_position);

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

impl crate::writers::ACWritable for HouseData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HouseData").entered();

        write_u32(writer, self.buy_time)?;
        write_u32(writer, self.rent_time)?;
        write_u32(writer, self.type_.clone() as u32)?;
        write_bool(writer, self.maintenance_free)?;
        write_packable_list::<HousePayment>(writer, &self.buy)?;
        write_packable_list::<HousePayment>(writer, &self.rent)?;
        self.position.write(writer)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for HAR {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HAR").entered();

        #[cfg(feature = "tracing")]
        let _field_span_version = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Version", position = pos).entered()
        };
        let version = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_version);
        #[cfg(feature = "tracing")]
        let _field_span_bitmask = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Bitmask", position = pos).entered()
        };
        let bitmask = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_bitmask);
        #[cfg(feature = "tracing")]
        let _field_span_monarch_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MonarchId", position = pos).entered()
        };
        let monarch_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_monarch_id);
        #[cfg(feature = "tracing")]
        let _field_span_guest_list = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "GuestList", position = pos).entered()
        };
        let guest_list = read_packable_hash_table::<ObjectId, GuestInfo>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_guest_list);
        #[cfg(feature = "tracing")]
        let _field_span_roommate_list = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "RoommateList", position = pos).entered()
        };
        let roommate_list = read_packable_list::<ObjectId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_roommate_list);

        Ok(Self {
            version,
            bitmask,
            monarch_id,
            guest_list,
            roommate_list,
        })
    }
}

impl crate::writers::ACWritable for HAR {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HAR").entered();

        write_u32(writer, self.version)?;
        write_u32(writer, self.bitmask)?;
        self.monarch_id.write(writer)?;
        write_packable_hash_table::<ObjectId, GuestInfo>(writer, &self.guest_list)?;
        write_packable_list::<ObjectId>(writer, &self.roommate_list)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for GuestInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GuestInfo").entered();

        #[cfg(feature = "tracing")]
        let _field_span_has_storage_permission = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HasStoragePermission", position = pos).entered()
        };
        let has_storage_permission = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_has_storage_permission);
        #[cfg(feature = "tracing")]
        let _field_span_guest_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "GuestName", position = pos).entered()
        };
        let guest_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_guest_name);

        Ok(Self {
            has_storage_permission,
            guest_name,
        })
    }
}

impl crate::writers::ACWritable for GuestInfo {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GuestInfo").entered();

        write_bool(writer, self.has_storage_permission)?;
        write_string(writer, &self.guest_name)?;
        Ok(())
    }
}

impl GameMoveDataType4 {
    #[allow(clippy::too_many_arguments)]
    pub fn read(reader: &mut dyn ACReader, player_id: ObjectId, team: int) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GameMoveDataType4").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GameMoveDataType5").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GameMoveDataType6").entered();

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "GameMoveData").entered();

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

impl GameMoveDataType4 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GameMoveDataType4").entered();

        write_i32(writer, self.id_piece_to_move)?;
        write_i32(writer, self.y_grid)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for GameMoveDataType4 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        GameMoveDataType4::write(self, writer)
    }
}

impl GameMoveDataType5 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GameMoveDataType5").entered();

        write_i32(writer, self.id_piece_to_move)?;
        write_i32(writer, self.y_grid)?;
        write_i32(writer, self.x_to)?;
        write_i32(writer, self.y_to)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for GameMoveDataType5 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        GameMoveDataType5::write(self, writer)
    }
}

impl GameMoveDataType6 {
    #[allow(clippy::too_many_arguments)]
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GameMoveDataType6").entered();

        write_i32(writer, self.id_piece_to_move)?;
        Ok(())
    }
}

impl crate::writers::ACWritable for GameMoveDataType6 {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        GameMoveDataType6::write(self, writer)
    }
}

impl GameMoveData {
    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "GameMoveData").entered();


        match self {
            Self::Type4(variant_struct) => {
                GameMoveDataType4::write(variant_struct, writer)?;
            },
            Self::Type5(variant_struct) => {
                GameMoveDataType5::write(variant_struct, writer)?;
            },
            Self::Type6(variant_struct) => {
                GameMoveDataType6::write(variant_struct, writer)?;
            },
        }
        Ok(())
    }
}

impl crate::writers::ACWritable for GameMoveData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        GameMoveData::write(self, writer)
    }
}

impl crate::readers::ACDataType for SalvageOperationsResultData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SalvageOperationsResultData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_skill_used = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SkillUsed", position = pos).entered()
        };
        let skill_used = SkillId::try_from(read_i32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_skill_used);
        #[cfg(feature = "tracing")]
        let _field_span_not_salvagable = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NotSalvagable", position = pos).entered()
        };
        let not_salvagable = read_packable_list::<ObjectId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_not_salvagable);
        #[cfg(feature = "tracing")]
        let _field_span_salvage_results = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SalvageResults", position = pos).entered()
        };
        let salvage_results = read_packable_list::<SalvageResult>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_salvage_results);
        #[cfg(feature = "tracing")]
        let _field_span_aug_bonus = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AugBonus", position = pos).entered()
        };
        let aug_bonus = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_aug_bonus);

        Ok(Self {
            skill_used,
            not_salvagable,
            salvage_results,
            aug_bonus,
        })
    }
}

impl crate::writers::ACWritable for SalvageOperationsResultData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "SalvageOperationsResultData").entered();

        write_i32(writer, self.skill_used.clone() as i32)?;
        write_packable_list::<ObjectId>(writer, &self.not_salvagable)?;
        write_packable_list::<SalvageResult>(writer, &self.salvage_results)?;
        write_i32(writer, self.aug_bonus)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for SalvageResult {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SalvageResult").entered();

        #[cfg(feature = "tracing")]
        let _field_span_material = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Material", position = pos).entered()
        };
        let material = MaterialType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_material);
        #[cfg(feature = "tracing")]
        let _field_span_workmanship = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Workmanship", position = pos).entered()
        };
        let workmanship = read_f64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_workmanship);
        #[cfg(feature = "tracing")]
        let _field_span_units = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Units", position = pos).entered()
        };
        let units = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_units);

        Ok(Self {
            material,
            workmanship,
            units,
        })
    }
}

impl crate::writers::ACWritable for SalvageResult {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "SalvageResult").entered();

        write_u32(writer, self.material.clone() as u32)?;
        write_f64(writer, self.workmanship)?;
        write_u32(writer, self.units)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for FellowshipLockData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "FellowshipLockData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_unknown1 = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Unknown1", position = pos).entered()
        };
        let unknown1 = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_unknown1);
        #[cfg(feature = "tracing")]
        let _field_span_unknown2 = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Unknown2", position = pos).entered()
        };
        let unknown2 = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_unknown2);
        #[cfg(feature = "tracing")]
        let _field_span_unknown3 = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Unknown3", position = pos).entered()
        };
        let unknown3 = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_unknown3);
        #[cfg(feature = "tracing")]
        let _field_span_timestamp = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Timestamp", position = pos).entered()
        };
        let timestamp = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_timestamp);
        #[cfg(feature = "tracing")]
        let _field_span_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Sequence", position = pos).entered()
        };
        let sequence = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sequence);

        Ok(Self {
            unknown1,
            unknown2,
            unknown3,
            timestamp,
            sequence,
        })
    }
}

impl crate::writers::ACWritable for FellowshipLockData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "FellowshipLockData").entered();

        write_u32(writer, self.unknown1)?;
        write_u32(writer, self.unknown2)?;
        write_u32(writer, self.unknown3)?;
        write_u32(writer, self.timestamp)?;
        write_u32(writer, self.sequence)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for Fellowship {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Fellowship").entered();

        #[cfg(feature = "tracing")]
        let _field_span_members = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Members", position = pos).entered()
        };
        let members = read_packable_hash_table::<ObjectId, Fellow>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_members);
        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);
        #[cfg(feature = "tracing")]
        let _field_span_leader_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "LeaderId", position = pos).entered()
        };
        let leader_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_leader_id);
        #[cfg(feature = "tracing")]
        let _field_span_share_xp = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ShareXP", position = pos).entered()
        };
        let share_xp = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_share_xp);
        #[cfg(feature = "tracing")]
        let _field_span_even_xp_split = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "EvenXPSplit", position = pos).entered()
        };
        let even_xp_split = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_even_xp_split);
        #[cfg(feature = "tracing")]
        let _field_span_open = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Open", position = pos).entered()
        };
        let open = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_open);
        #[cfg(feature = "tracing")]
        let _field_span_locked = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Locked", position = pos).entered()
        };
        let locked = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_locked);
        #[cfg(feature = "tracing")]
        let _field_span_recently_departed = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "RecentlyDeparted", position = pos).entered()
        };
        let recently_departed = read_packable_hash_table::<ObjectId, i32>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_recently_departed);
        #[cfg(feature = "tracing")]
        let _field_span_locks = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Locks", position = pos).entered()
        };
        let locks = read_packable_hash_table::<String, FellowshipLockData>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_locks);

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

impl crate::writers::ACWritable for Fellowship {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Fellowship").entered();

        write_packable_hash_table::<ObjectId, Fellow>(writer, &self.members)?;
        write_string(writer, &self.name)?;
        self.leader_id.write(writer)?;
        write_bool(writer, self.share_xp)?;
        write_bool(writer, self.even_xp_split)?;
        write_bool(writer, self.open)?;
        write_bool(writer, self.locked)?;
        write_packable_hash_table::<ObjectId, i32>(writer, &self.recently_departed)?;
        write_packable_hash_table::<String, FellowshipLockData>(writer, &self.locks)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for Fellow {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "Fellow").entered();

        #[cfg(feature = "tracing")]
        let _field_span_xp_cached = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "XPCached", position = pos).entered()
        };
        let xp_cached = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_xp_cached);
        #[cfg(feature = "tracing")]
        let _field_span_lum_cached = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "LumCached", position = pos).entered()
        };
        let lum_cached = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_lum_cached);
        #[cfg(feature = "tracing")]
        let _field_span_level = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Level", position = pos).entered()
        };
        let level = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_level);
        #[cfg(feature = "tracing")]
        let _field_span_max_health = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxHealth", position = pos).entered()
        };
        let max_health = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_health);
        #[cfg(feature = "tracing")]
        let _field_span_max_stamina = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxStamina", position = pos).entered()
        };
        let max_stamina = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_stamina);
        #[cfg(feature = "tracing")]
        let _field_span_max_mana = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MaxMana", position = pos).entered()
        };
        let max_mana = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_max_mana);
        #[cfg(feature = "tracing")]
        let _field_span_current_health = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CurrentHealth", position = pos).entered()
        };
        let current_health = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_current_health);
        #[cfg(feature = "tracing")]
        let _field_span_current_stamina = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CurrentStamina", position = pos).entered()
        };
        let current_stamina = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_current_stamina);
        #[cfg(feature = "tracing")]
        let _field_span_current_mana = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CurrentMana", position = pos).entered()
        };
        let current_mana = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_current_mana);
        #[cfg(feature = "tracing")]
        let _field_span_share_loot = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ShareLoot", position = pos).entered()
        };
        let share_loot = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_share_loot);
        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);

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

impl crate::writers::ACWritable for Fellow {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "Fellow").entered();

        write_u32(writer, self.xp_cached)?;
        write_u32(writer, self.lum_cached)?;
        write_u32(writer, self.level)?;
        write_u32(writer, self.max_health)?;
        write_u32(writer, self.max_stamina)?;
        write_u32(writer, self.max_mana)?;
        write_u32(writer, self.current_health)?;
        write_u32(writer, self.current_stamina)?;
        write_u32(writer, self.current_mana)?;
        write_bool(writer, self.share_loot)?;
        write_string(writer, &self.name)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for ContractTracker {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ContractTracker").entered();

        #[cfg(feature = "tracing")]
        let _field_span_version = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Version", position = pos).entered()
        };
        let version = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_version);
        #[cfg(feature = "tracing")]
        let _field_span_contract_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContractId", position = pos).entered()
        };
        let contract_id = ContractId::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_contract_id);
        #[cfg(feature = "tracing")]
        let _field_span_contract_stage = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContractStage", position = pos).entered()
        };
        let contract_stage = ContractStage::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_contract_stage);
        #[cfg(feature = "tracing")]
        let _field_span_time_when_done = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TimeWhenDone", position = pos).entered()
        };
        let time_when_done = read_i64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_time_when_done);
        #[cfg(feature = "tracing")]
        let _field_span_time_when_repeats = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TimeWhenRepeats", position = pos).entered()
        };
        let time_when_repeats = read_i64(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_time_when_repeats);

        Ok(Self {
            version,
            contract_id,
            contract_stage,
            time_when_done,
            time_when_repeats,
        })
    }
}

impl crate::writers::ACWritable for ContractTracker {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ContractTracker").entered();

        write_u32(writer, self.version)?;
        write_u32(writer, self.contract_id.clone() as u32)?;
        write_u32(writer, self.contract_stage.clone() as u32)?;
        write_i64(writer, self.time_when_done)?;
        write_i64(writer, self.time_when_repeats)?;
        Ok(())
    }
}

impl crate::readers::ACDataType for ContractTrackerTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ContractTrackerTable").entered();

        #[cfg(feature = "tracing")]
        let _field_span_contact_trackers = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContactTrackers", position = pos).entered()
        };
        let contact_trackers = read_packable_hash_table::<u32, ContractTracker>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_contact_trackers);

        Ok(Self {
            contact_trackers,
        })
    }
}

impl crate::writers::ACWritable for ContractTrackerTable {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ContractTrackerTable").entered();

        write_packable_hash_table::<u32, ContractTracker>(writer, &self.contact_trackers)?;
        Ok(())
    }
}

