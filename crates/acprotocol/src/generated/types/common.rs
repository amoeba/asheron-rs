use serde::{Serialize, Deserialize};

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

#[allow(non_camel_case_types)]
pub type WString = String;

#[allow(non_camel_case_types)]
pub type WORD = u16;

#[allow(non_camel_case_types)]
pub type DWORD = u32;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackedWORD {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackedDWORD {}

#[allow(non_camel_case_types)]
pub type ObjectId = u32;

#[allow(non_camel_case_types)]
pub type LandcellId = u32;

#[allow(non_camel_case_types)]
pub type SpellId = u16;

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
#[serde(tag = "AuthType")]
pub enum LoginRequestHeader {
    #[serde(rename = "0x00000002")]
    Type00000002 {
    #[serde(rename = "ClientVersion")]
    client_version: String,
    #[serde(rename = "Length")]
    length: u32,
    #[serde(rename = "Flags")]
    flags: AuthFlags,
    #[serde(rename = "Sequence")]
    sequence: u32,
    #[serde(rename = "Account")]
    account: String,
    #[serde(rename = "AccountToLoginAs")]
    account_to_login_as: String,
    #[serde(rename = "Password")]
    password: String,
    },
    #[serde(rename = "0x40000002")]
    Type40000002 {
    #[serde(rename = "ClientVersion")]
    client_version: String,
    #[serde(rename = "Length")]
    length: u32,
    #[serde(rename = "Flags")]
    flags: AuthFlags,
    #[serde(rename = "Sequence")]
    sequence: u32,
    #[serde(rename = "Account")]
    account: String,
    #[serde(rename = "AccountToLoginAs")]
    account_to_login_as: String,
    #[serde(rename = "GlsTicket")]
    gls_ticket: String,
    },
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
    #[serde(rename = "DataProperties")]
    pub data_properties: Option<PackableHashTable<PropertyDataId, DataId>>,
    #[serde(rename = "InstanceProperties")]
    pub instance_properties: Option<PackableHashTable<PropertyInstanceId, ObjectId>>,
    #[serde(rename = "PositionProperties")]
    pub position_properties: Option<PackableHashTable<PropertyPosition, Position>>,
}

// The ACQualities structure contains character property lists.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ACQualities {
    #[serde(rename = "Flags")]
    pub flags: ACQualitiesFlags,
    #[serde(rename = "HasHealth")]
    pub has_health: bool,
    #[serde(rename = "Attributes")]
    pub attributes: Option<AttributeCache>,
    #[serde(rename = "Skills")]
    pub skills: Option<PackableHashTable<SkillId, Skill>>,
    #[serde(rename = "Body")]
    pub body: Option<Body>,
    #[serde(rename = "SpellBook")]
    pub spell_book: Option<PackableHashTable<LayeredSpellId, SpellBookPage>>,
    #[serde(rename = "Enchantments")]
    pub enchantments: Option<EnchantmentRegistry>,
    #[serde(rename = "EventFilter")]
    pub event_filter: Option<EventFilter>,
    #[serde(rename = "Emotes")]
    pub emotes: Option<EmoteTable>,
    #[serde(rename = "CreationProfile")]
    pub creation_profile: Option<PackableList<CreationProfile>>,
    #[serde(rename = "PageData")]
    pub page_data: Option<PageDataList>,
    #[serde(rename = "Generators")]
    pub generators: Option<GeneratorTable>,
    #[serde(rename = "GeneratorRegistry")]
    pub generator_registry: Option<GeneratorRegistry>,
    #[serde(rename = "GeneratorQueue")]
    pub generator_queue: Option<GeneratorQueue>,
}

// The AttributeCache structure contains information about a character attributes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeCache {
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "Strength")]
    pub strength: Option<AttributeInfo>,
    #[serde(rename = "Endurance")]
    pub endurance: Option<AttributeInfo>,
    #[serde(rename = "Quickness")]
    pub quickness: Option<AttributeInfo>,
    #[serde(rename = "Coordination")]
    pub coordination: Option<AttributeInfo>,
    #[serde(rename = "Focus")]
    pub focus: Option<AttributeInfo>,
    #[serde(rename = "Self")]
    pub self_: Option<AttributeInfo>,
    #[serde(rename = "Health")]
    pub health: Option<SecondaryAttributeInfo>,
    #[serde(rename = "Stamina")]
    pub stamina: Option<SecondaryAttributeInfo>,
    #[serde(rename = "Mana")]
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
    pub body_parts: PackableHashTable<uint, BodyPart>,
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
    #[serde(rename = "BPSD")]
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
    #[serde(rename = "CastingLikelihood2")]
    pub casting_likelihood2: Option<f32>,
    #[serde(rename = "Unknown")]
    pub unknown: Option<i32>,
}

// Contains information related to the spells in effect on the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnchantmentRegistry {
    #[serde(rename = "Flags")]
    pub flags: EnchantmentRegistryFlags,
    #[serde(rename = "LifeSpells")]
    pub life_spells: Option<PackableList<Enchantment>>,
    #[serde(rename = "CreatureSpells")]
    pub creature_spells: Option<PackableList<Enchantment>>,
    #[serde(rename = "Vitae")]
    pub vitae: Option<Enchantment>,
    #[serde(rename = "Cooldowns")]
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
    #[serde(rename = "EquipmentSet")]
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
    pub events: PackableList<uint>,
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
#[serde(tag = "Category")]
pub enum EmoteSet {
    #[serde(rename = "0x01")]
    #[serde(alias = "0x06")]
    Type01 {
    #[serde(rename = "Probability")]
    probability: f32,
    #[serde(rename = "Emotes")]
    emotes: PackableList<Emote>,
    #[serde(rename = "ClassId")]
    class_id: u32,
    },
    #[serde(rename = "0x02")]
    Type02 {
    #[serde(rename = "Probability")]
    probability: f32,
    #[serde(rename = "Emotes")]
    emotes: PackableList<Emote>,
    #[serde(rename = "VendorType")]
    vendor_type: u32,
    },
    #[serde(rename = "0x05")]
    Type05 {
    #[serde(rename = "Probability")]
    probability: f32,
    #[serde(rename = "Emotes")]
    emotes: PackableList<Emote>,
    #[serde(rename = "Style")]
    style: u32,
    #[serde(rename = "Substyle")]
    substyle: u32,
    },
    #[serde(rename = "0x0C")]
    #[serde(alias = "0x0D")]
    #[serde(alias = "0x16")]
    #[serde(alias = "0x17")]
    #[serde(alias = "0x1B")]
    #[serde(alias = "0x1C")]
    #[serde(alias = "0x1D")]
    #[serde(alias = "0x1E")]
    #[serde(alias = "0x1F")]
    #[serde(alias = "0x20")]
    #[serde(alias = "0x21")]
    #[serde(alias = "0x22")]
    #[serde(alias = "0x23")]
    #[serde(alias = "0x24")]
    #[serde(alias = "0x25")]
    #[serde(alias = "0x26")]
    Type0C {
    #[serde(rename = "Probability")]
    probability: f32,
    #[serde(rename = "Emotes")]
    emotes: PackableList<Emote>,
    #[serde(rename = "Quest")]
    quest: String,
    },
    #[serde(rename = "0x0F")]
    Type0F {
    #[serde(rename = "Probability")]
    probability: f32,
    #[serde(rename = "Emotes")]
    emotes: PackableList<Emote>,
    #[serde(rename = "MinHealth")]
    min_health: f32,
    #[serde(rename = "MaxHealth")]
    max_health: f32,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum Emote {
    #[serde(rename = "0x01")]
    #[serde(alias = "0x08")]
    #[serde(alias = "0x0A")]
    #[serde(alias = "0x0D")]
    #[serde(alias = "0x10")]
    #[serde(alias = "0x11")]
    #[serde(alias = "0x12")]
    #[serde(alias = "0x14")]
    #[serde(alias = "0x15")]
    #[serde(alias = "0x16")]
    #[serde(alias = "0x17")]
    #[serde(alias = "0x18")]
    #[serde(alias = "0x19")]
    #[serde(alias = "0x1A")]
    #[serde(alias = "0x1F")]
    #[serde(alias = "0x33")]
    #[serde(alias = "0x3A")]
    #[serde(alias = "0x3C")]
    #[serde(alias = "0x3D")]
    #[serde(alias = "0x40")]
    #[serde(alias = "0x41")]
    #[serde(alias = "0x43")]
    #[serde(alias = "0x44")]
    #[serde(alias = "0x4F")]
    #[serde(alias = "0x50")]
    #[serde(alias = "0x51")]
    #[serde(alias = "0x53")]
    #[serde(alias = "0x58")]
    #[serde(alias = "0x79")]
    Type01 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Message")]
    message: String,
    },
    #[serde(rename = "0x13")]
    #[serde(alias = "0x1B")]
    #[serde(alias = "0x49")]
    #[serde(alias = "0xE")]
    Type13 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "SpellId")]
    spell_id: u32,
    },
    #[serde(rename = "0x1C")]
    #[serde(alias = "0x1D")]
    Type1C {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Amount")]
    amount: u32,
    #[serde(rename = "Stat")]
    stat: u32,
    },
    #[serde(rename = "0x1E")]
    #[serde(alias = "0x3B")]
    #[serde(alias = "0x47")]
    #[serde(alias = "0x52")]
    Type1E {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Min")]
    min: u32,
    #[serde(rename = "Max")]
    max: u32,
    },
    #[serde(rename = "0x2")]
    #[serde(alias = "0x3E")]
    Type2 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Amount64")]
    amount64: u64,
    #[serde(rename = "HeroXP64")]
    hero_xp64: u64,
    },
    #[serde(rename = "0x20")]
    #[serde(alias = "0x21")]
    #[serde(alias = "0x46")]
    #[serde(alias = "0x54")]
    #[serde(alias = "0x55")]
    #[serde(alias = "0x56")]
    #[serde(alias = "0x59")]
    #[serde(alias = "0x66")]
    #[serde(alias = "0x67")]
    #[serde(alias = "0x68")]
    #[serde(alias = "0x69")]
    #[serde(alias = "0x6A")]
    #[serde(alias = "0x6B")]
    #[serde(alias = "0x6C")]
    #[serde(alias = "0x6D")]
    Type20 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Amount")]
    amount: u32,
    },
    #[serde(rename = "0x22")]
    #[serde(alias = "0x2F")]
    #[serde(alias = "0x30")]
    #[serde(alias = "0x5A")]
    #[serde(alias = "0x6F")]
    #[serde(alias = "0x77")]
    #[serde(alias = "0x78")]
    Type22 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Amount")]
    amount: u32,
    },
    #[serde(rename = "0x23")]
    #[serde(alias = "0x2D")]
    #[serde(alias = "0x2E")]
    Type23 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Stat")]
    stat: u32,
    },
    #[serde(rename = "0x24")]
    #[serde(alias = "0x27")]
    #[serde(alias = "0x28")]
    #[serde(alias = "0x29")]
    #[serde(alias = "0x2A")]
    #[serde(alias = "0x2B")]
    #[serde(alias = "0x2C")]
    Type24 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Min")]
    min: u32,
    #[serde(rename = "Max")]
    max: u32,
    #[serde(rename = "Stat")]
    stat: u32,
    },
    #[serde(rename = "0x25")]
    Type25 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "FMin")]
    f_min: f64,
    #[serde(rename = "FMax")]
    f_max: f64,
    #[serde(rename = "Stat")]
    stat: u32,
    },
    #[serde(rename = "0x26")]
    #[serde(alias = "0x4B")]
    Type26 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "TestString")]
    test_string: String,
    #[serde(rename = "Stat")]
    stat: u32,
    },
    #[serde(rename = "0x3")]
    #[serde(alias = "0x4A")]
    Type3 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "CProfile")]
    c_profile: CreationProfile,
    },
    #[serde(rename = "0x31")]
    Type31 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Percent")]
    percent: f64,
    #[serde(rename = "Min64")]
    min64: u64,
    #[serde(rename = "Max64")]
    max64: u64,
    },
    #[serde(rename = "0x32")]
    Type32 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Stat")]
    stat: u32,
    #[serde(rename = "Percent")]
    percent: f64,
    #[serde(rename = "Min")]
    min: u32,
    #[serde(rename = "Max")]
    max: u32,
    #[serde(rename = "Display")]
    display: bool,
    },
    #[serde(rename = "0x34")]
    #[serde(alias = "0x5")]
    Type34 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Motion")]
    motion: u32,
    },
    #[serde(rename = "0x35")]
    #[serde(alias = "0x36")]
    #[serde(alias = "0x37")]
    #[serde(alias = "0x45")]
    Type35 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Stat")]
    stat: u32,
    #[serde(rename = "Amount")]
    amount: u32,
    },
    #[serde(rename = "0x38")]
    Type38 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "WealthRating")]
    wealth_rating: i32,
    #[serde(rename = "TreasureClass")]
    treasure_class: i32,
    #[serde(rename = "TreasureType")]
    treasure_type: i32,
    },
    #[serde(rename = "0x3F")]
    #[serde(alias = "0x63")]
    #[serde(alias = "0x64")]
    Type3F {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Position")]
    position: Position,
    },
    #[serde(rename = "0x4")]
    #[serde(alias = "0x57")]
    #[serde(alias = "0x6")]
    #[serde(alias = "0xB")]
    Type4 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Frame")]
    frame: Frame,
    },
    #[serde(rename = "0x4C")]
    Type4C {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    msg: String,
    #[serde(rename = "CProfile")]
    c_profile: CreationProfile,
    },
    #[serde(rename = "0x6E")]
    #[serde(alias = "0x73")]
    Type6E {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Stat")]
    stat: u32,
    },
    #[serde(rename = "0x7")]
    Type7 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "PhysicsScript")]
    physics_script: u32,
    },
    #[serde(rename = "0x70")]
    #[serde(alias = "0x71")]
    Type70 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Amount64")]
    amount64: u64,
    },
    #[serde(rename = "0x72")]
    Type72 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Min64")]
    min64: u64,
    #[serde(rename = "Max64")]
    max64: u64,
    #[serde(rename = "Stat")]
    stat: u32,
    },
    #[serde(rename = "0x76")]
    Type76 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Stat")]
    stat: u32,
    #[serde(rename = "Percent")]
    percent: f64,
    },
    #[serde(rename = "0x9")]
    Type9 {
    #[serde(rename = "Delay")]
    delay: f32,
    #[serde(rename = "Extent")]
    extent: f32,
    #[serde(rename = "Sound")]
    sound: u32,
    },
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
    #[serde(rename = "PageText")]
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
    pub data: Vec<byte>,
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
    pub registry: PackableHashTable<uint, GeneratorRegistryNode>,
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
#[serde(tag = "TitleSource")]
pub enum WindowProperty {
    #[serde(rename = "0x00")]
    Type00 {
    #[serde(rename = "Key_a")]
    key_a: u32,
    #[serde(rename = "Unknown_1b")]
    unknown_1b: u32,
    #[serde(rename = "Unknown_1c")]
    unknown_1c: u16,
    #[serde(rename = "Unknown_d")]
    unknown_d: u32,
    #[serde(rename = "Value_d")]
    value_d: u8,
    #[serde(rename = "Unknown_e")]
    unknown_e: u32,
    #[serde(rename = "Value_e")]
    value_e: u32,
    #[serde(rename = "Unknown_f")]
    unknown_f: u32,
    #[serde(rename = "Value_f")]
    value_f: u32,
    #[serde(rename = "Unknown_h")]
    unknown_h: u32,
    #[serde(rename = "Value_h")]
    value_h: u32,
    #[serde(rename = "Unknown_i")]
    unknown_i: u32,
    #[serde(rename = "Value_i")]
    value_i: u32,
    #[serde(rename = "Unknown_j")]
    unknown_j: u32,
    #[serde(rename = "Value_j")]
    value_j: u64,
    #[serde(rename = "StringId")]
    string_id: u32,
    #[serde(rename = "FileId")]
    file_id: u32,
    },
    #[serde(rename = "0x01")]
    Type01 {
    #[serde(rename = "Key_a")]
    key_a: u32,
    #[serde(rename = "Unknown_1b")]
    unknown_1b: u32,
    #[serde(rename = "Unknown_1c")]
    unknown_1c: u16,
    #[serde(rename = "Unknown_d")]
    unknown_d: u32,
    #[serde(rename = "Value_d")]
    value_d: u8,
    #[serde(rename = "Unknown_e")]
    unknown_e: u32,
    #[serde(rename = "Value_e")]
    value_e: u32,
    #[serde(rename = "Unknown_f")]
    unknown_f: u32,
    #[serde(rename = "Value_f")]
    value_f: u32,
    #[serde(rename = "Unknown_h")]
    unknown_h: u32,
    #[serde(rename = "Value_h")]
    value_h: u32,
    #[serde(rename = "Unknown_i")]
    unknown_i: u32,
    #[serde(rename = "Value_i")]
    value_i: u32,
    #[serde(rename = "Unknown_j")]
    unknown_j: u32,
    #[serde(rename = "Value_j")]
    value_j: u64,
    #[serde(rename = "Value_a")]
    value_a: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type_a")]
pub enum WindowOption {
    #[serde(rename = "0x1000008b")]
    Type1000008B {
    #[serde(rename = "Unknown_b")]
    unknown_b: u8,
    #[serde(rename = "PropertyCount")]
    property_count: u8,
    #[serde(rename = "Properties")]
    properties: Vec<WindowProperty>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum OptionProperty {
    #[serde(rename = "0x10000080")]
    Type10000080 {
    unknown_l: u32,
    #[serde(rename = "inactiveOpacity")]
    inactive_opacity: f32,
    },
    #[serde(rename = "0x10000081")]
    Type10000081 {
    unknown_k: u32,
    #[serde(rename = "activeOpacity")]
    active_opacity: f32,
    },
    #[serde(rename = "0x1000008c")]
    Type1000008C {
    #[serde(rename = "Unknown_a")]
    unknown_a: u32,
    #[serde(rename = "WindowOptions")]
    window_options: PackableList<WindowOption>,
    },
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
    #[serde(rename = "Shortcuts")]
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
    #[serde(rename = "FillComps")]
    pub fill_comps: Option<PackableHashTable<uint, uint>>,
    #[serde(rename = "SpellBookFilters")]
    pub spell_book_filters: Option<u32>,
    #[serde(rename = "OptionFlags")]
    pub option_flags: Option<u32>,
    #[serde(rename = "Unknown100_1")]
    pub unknown100_1: Option<u32>,
    #[serde(rename = "OptionStrings")]
    pub option_strings: Option<PackableHashTable<uint, string>>,
    #[serde(rename = "GameplayOptions")]
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
    pub officer_titles: PackableList<string>,
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
    #[serde(rename = "MonarchData")]
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
    #[serde(rename = "Level")]
    pub level: Option<u32>,
    #[serde(rename = "Loyalty")]
    pub loyalty: u16,
    #[serde(rename = "Leadership")]
    pub leadership: u16,
    #[serde(rename = "AllegianceAge")]
    pub allegiance_age: Option<u32>,
    #[serde(rename = "TimeOnline")]
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
#[serde(tag = "PwdType")]
pub enum ItemProfile {
    #[serde(rename = "-1")]
    TypeNeg1 {
    #[serde(rename = "PackedAmount")]
    packed_amount: u32,
    #[serde(rename = "ObjectId")]
    object_id: ObjectId,
    #[serde(rename = "WeenieDescription")]
    weenie_description: PublicWeenieDesc,
    },
    #[serde(rename = "1")]
    Type1 {
    #[serde(rename = "PackedAmount")]
    packed_amount: u32,
    #[serde(rename = "ObjectId")]
    object_id: ObjectId,
    #[serde(rename = "OldWeenieDescription")]
    old_weenie_description: OldPublicWeenieDesc,
    },
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
    #[serde(rename = "Header2")]
    pub header2: Option<u32>,
    #[serde(rename = "PluralName")]
    pub plural_name: Option<String>,
    #[serde(rename = "ItemsCapacity")]
    pub items_capacity: Option<u8>,
    #[serde(rename = "ContainerCapacity")]
    pub container_capacity: Option<u8>,
    #[serde(rename = "AmmunitionType")]
    pub ammunition_type: Option<AmmoType>,
    #[serde(rename = "Value")]
    pub value: Option<u32>,
    #[serde(rename = "Useability")]
    pub useability: Option<Usable>,
    #[serde(rename = "UseRadius")]
    pub use_radius: Option<f32>,
    #[serde(rename = "TargetType")]
    pub target_type: Option<ItemType>,
    #[serde(rename = "Effects")]
    pub effects: Option<IconHighlight>,
    #[serde(rename = "CombatUse")]
    pub combat_use: Option<WieldType>,
    #[serde(rename = "Structure")]
    pub structure: Option<u16>,
    #[serde(rename = "MaxStructure")]
    pub max_structure: Option<u16>,
    #[serde(rename = "StackSize")]
    pub stack_size: Option<u16>,
    #[serde(rename = "MaxStackSize")]
    pub max_stack_size: Option<u16>,
    #[serde(rename = "ContainerId")]
    pub container_id: Option<ObjectId>,
    #[serde(rename = "WielderId")]
    pub wielder_id: Option<ObjectId>,
    #[serde(rename = "ValidSlots")]
    pub valid_slots: Option<EquipMask>,
    #[serde(rename = "Slot")]
    pub slot: Option<EquipMask>,
    #[serde(rename = "Priority")]
    pub priority: Option<CoverageMask>,
    #[serde(rename = "BlipColor")]
    pub blip_color: Option<RadarColor>,
    #[serde(rename = "RadarEnum")]
    pub radar_enum: Option<RadarBehavior>,
    #[serde(rename = "PhysicsScript")]
    pub physics_script: Option<u16>,
    #[serde(rename = "Workmanship")]
    pub workmanship: Option<f32>,
    #[serde(rename = "Burden")]
    pub burden: Option<u16>,
    #[serde(rename = "SpellId")]
    pub spell_id: Option<SpellId>,
    #[serde(rename = "OwnerId")]
    pub owner_id: Option<ObjectId>,
    #[serde(rename = "Restrictions")]
    pub restrictions: Option<RestrictionDB>,
    #[serde(rename = "HookItemTypes")]
    pub hook_item_types: Option<HookType>,
    #[serde(rename = "MonarchId")]
    pub monarch_id: Option<ObjectId>,
    #[serde(rename = "HookType")]
    pub hook_type: Option<HookType>,
    #[serde(rename = "IconOverlay")]
    pub icon_overlay: Option<PackedDWORD>,
    #[serde(rename = "IconUnderlay")]
    pub icon_underlay: Option<PackedDWORD>,
    #[serde(rename = "Material")]
    pub material: Option<MaterialType>,
    #[serde(rename = "CooldownId")]
    pub cooldown_id: Option<u32>,
    #[serde(rename = "CooldownDuration")]
    pub cooldown_duration: Option<u64>,
    #[serde(rename = "PetOwnerId")]
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
    pub permissions: PHashTable<ObjectId, uint>,
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
    #[serde(rename = "PluralName")]
    pub plural_name: Option<String>,
    #[serde(rename = "ItemsCapacity")]
    pub items_capacity: Option<u8>,
    #[serde(rename = "ContainerCapacity")]
    pub container_capacity: Option<u8>,
    #[serde(rename = "Value")]
    pub value: Option<u32>,
    #[serde(rename = "Useability")]
    pub useability: Option<Usable>,
    #[serde(rename = "UseRadius")]
    pub use_radius: Option<f32>,
    #[serde(rename = "tTargetType")]
    pub t_target_type: Option<ItemType>,
    #[serde(rename = "Effects")]
    pub effects: Option<IconHighlight>,
    #[serde(rename = "AmmunitionType")]
    pub ammunition_type: Option<AmmoType>,
    #[serde(rename = "CombatUse")]
    pub combat_use: Option<WieldType>,
    #[serde(rename = "Structure")]
    pub structure: Option<u16>,
    #[serde(rename = "MaxStructure")]
    pub max_structure: Option<u16>,
    #[serde(rename = "StackSize")]
    pub stack_size: Option<u16>,
    #[serde(rename = "MaxStackSize")]
    pub max_stack_size: Option<u16>,
    #[serde(rename = "ContainerId")]
    pub container_id: Option<ObjectId>,
    #[serde(rename = "WielderId")]
    pub wielder_id: Option<ObjectId>,
    #[serde(rename = "ValidSlots")]
    pub valid_slots: Option<EquipMask>,
    #[serde(rename = "Slots")]
    pub slots: Option<EquipMask>,
    #[serde(rename = "Priority")]
    pub priority: Option<CoverageMask>,
    #[serde(rename = "BlipColor")]
    pub blip_color: Option<RadarColor>,
    #[serde(rename = "RadarEnum")]
    pub radar_enum: Option<RadarBehavior>,
    #[serde(rename = "ObviousDistance")]
    pub obvious_distance: Option<f32>,
    #[serde(rename = "Vndwcid")]
    pub vndwcid: Option<u16>,
    #[serde(rename = "SpellId")]
    pub spell_id: Option<SpellId>,
    #[serde(rename = "HouseOwnerId")]
    pub house_owner_id: Option<ObjectId>,
    #[serde(rename = "PhysicsScript")]
    pub physics_script: Option<u16>,
    #[serde(rename = "Restrictions")]
    pub restrictions: Option<RestrictionDB>,
    #[serde(rename = "HookType")]
    pub hook_type: Option<HookType>,
    #[serde(rename = "HookItemTypes")]
    pub hook_item_types: Option<HookType>,
    #[serde(rename = "MonarchId")]
    pub monarch_id: Option<ObjectId>,
    #[serde(rename = "IconOverlay")]
    pub icon_overlay: Option<PackedDWORD>,
    #[serde(rename = "Material")]
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
    #[serde(rename = "CurrentHoldkey")]
    pub current_holdkey: Option<HoldKey>,
    #[serde(rename = "CurrentStyle")]
    pub current_style: Option<StanceMode>,
    #[serde(rename = "ForwardCommand")]
    pub forward_command: Option<MovementCommand>,
    #[serde(rename = "ForwardHoldkey")]
    pub forward_holdkey: Option<HoldKey>,
    #[serde(rename = "ForwardSpeed")]
    pub forward_speed: Option<f32>,
    #[serde(rename = "SidestepCommand")]
    pub sidestep_command: Option<MovementCommand>,
    #[serde(rename = "SidestepHoldkey")]
    pub sidestep_holdkey: Option<HoldKey>,
    #[serde(rename = "SidestepSpeed")]
    pub sidestep_speed: Option<f32>,
    #[serde(rename = "TurnCommand")]
    pub turn_command: Option<MovementCommand>,
    #[serde(rename = "TurnHoldkey")]
    pub turn_holdkey: Option<u32>,
    #[serde(rename = "TurnSpeed")]
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
    #[serde(rename = "WQuat")]
    pub w_quat: Option<f32>,
    #[serde(rename = "XQuat")]
    pub x_quat: Option<f32>,
    #[serde(rename = "YQuat")]
    pub y_quat: Option<f32>,
    #[serde(rename = "ZQuat")]
    pub z_quat: Option<f32>,
    #[serde(rename = "Velocity")]
    pub velocity: Option<Vector3>,
    #[serde(rename = "PlacementId")]
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
#[serde(tag = "MovementType")]
pub enum MovementData {
    #[serde(rename = "0x0000")]
    Type0000 {
    #[serde(rename = "ObjectMovementSequence")]
    object_movement_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    object_server_control_sequence: u16,
    #[serde(rename = "Autonomous")]
    autonomous: u16,
    #[serde(rename = "OptionFlags")]
    option_flags: MovementOption,
    #[serde(rename = "Stance")]
    stance: StanceMode,
    #[serde(rename = "State")]
    state: InterpertedMotionState,
    #[serde(rename = "StickyObject")]
    sticky_object: Option<ObjectId>,
    },
    #[serde(rename = "0x0006")]
    Type0006 {
    #[serde(rename = "ObjectMovementSequence")]
    object_movement_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    object_server_control_sequence: u16,
    #[serde(rename = "Autonomous")]
    autonomous: u16,
    #[serde(rename = "OptionFlags")]
    option_flags: MovementOption,
    #[serde(rename = "Stance")]
    stance: StanceMode,
    #[serde(rename = "Target")]
    target: ObjectId,
    #[serde(rename = "Origin")]
    origin: Origin,
    #[serde(rename = "MoveToParams")]
    move_to_params: MoveToMovementParameters,
    #[serde(rename = "MyRunRate")]
    my_run_rate: f32,
    },
    #[serde(rename = "0x0007")]
    Type0007 {
    #[serde(rename = "ObjectMovementSequence")]
    object_movement_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    object_server_control_sequence: u16,
    #[serde(rename = "Autonomous")]
    autonomous: u16,
    #[serde(rename = "OptionFlags")]
    option_flags: MovementOption,
    #[serde(rename = "Stance")]
    stance: StanceMode,
    #[serde(rename = "Origin")]
    origin: Origin,
    #[serde(rename = "MoveToParams")]
    move_to_params: MoveToMovementParameters,
    #[serde(rename = "MyRunRate")]
    my_run_rate: f32,
    },
    #[serde(rename = "0x0008")]
    Type0008 {
    #[serde(rename = "ObjectMovementSequence")]
    object_movement_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    object_server_control_sequence: u16,
    #[serde(rename = "Autonomous")]
    autonomous: u16,
    #[serde(rename = "OptionFlags")]
    option_flags: MovementOption,
    #[serde(rename = "Stance")]
    stance: StanceMode,
    #[serde(rename = "TargetId")]
    target_id: ObjectId,
    #[serde(rename = "DesiredHeading")]
    desired_heading: f32,
    #[serde(rename = "TurnToParams")]
    turn_to_params: TurnToMovementParameters,
    },
    #[serde(rename = "0x0009")]
    Type0009 {
    #[serde(rename = "ObjectMovementSequence")]
    object_movement_sequence: u16,
    #[serde(rename = "ObjectServerControlSequence")]
    object_server_control_sequence: u16,
    #[serde(rename = "Autonomous")]
    autonomous: u16,
    #[serde(rename = "OptionFlags")]
    option_flags: MovementOption,
    #[serde(rename = "Stance")]
    stance: StanceMode,
    #[serde(rename = "TurnToParams")]
    turn_to_params: TurnToMovementParameters,
    },
}

// Contains information for animations and general free motion
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterpertedMotionState {
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "CurrentStyle")]
    pub current_style: Option<StanceMode>,
    #[serde(rename = "ForwardCommand")]
    pub forward_command: Option<MovementCommand>,
    #[serde(rename = "SidestepCommand")]
    pub sidestep_command: Option<MovementCommand>,
    #[serde(rename = "TurnCommand")]
    pub turn_command: Option<MovementCommand>,
    #[serde(rename = "ForwardSpeed")]
    pub forward_speed: Option<f32>,
    #[serde(rename = "SidestepSpeed")]
    pub sidestep_speed: Option<f32>,
    #[serde(rename = "TurnSpeed")]
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
    #[serde(rename = "Palette")]
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
    #[serde(rename = "MovementBuffer")]
    pub movement_buffer: Option<PackableList<byte>>,
    #[serde(rename = "Autonomous")]
    pub autonomous: Option<bool>,
    #[serde(rename = "AnimationFrame")]
    pub animation_frame: Option<u32>,
    #[serde(rename = "Position")]
    pub position: Option<Position>,
    #[serde(rename = "MotionId")]
    pub motion_id: Option<DataId>,
    #[serde(rename = "SoundId")]
    pub sound_id: Option<DataId>,
    #[serde(rename = "PhysicsScriptId")]
    pub physics_script_id: Option<DataId>,
    #[serde(rename = "SetupId")]
    pub setup_id: Option<DataId>,
    #[serde(rename = "ParentId")]
    pub parent_id: Option<ObjectId>,
    #[serde(rename = "ParentLocation")]
    pub parent_location: Option<ParentLocation>,
    #[serde(rename = "Children")]
    pub children: Option<PackableList<EquipLocation>>,
    #[serde(rename = "Scale")]
    pub scale: Option<f32>,
    #[serde(rename = "Friction")]
    pub friction: Option<f32>,
    #[serde(rename = "Elasticity")]
    pub elasticity: Option<f32>,
    #[serde(rename = "Translucency")]
    pub translucency: Option<f32>,
    #[serde(rename = "Velocity")]
    pub velocity: Option<Vector3>,
    #[serde(rename = "Acceleration")]
    pub acceleration: Option<Vector3>,
    #[serde(rename = "Omega")]
    pub omega: Option<Vector3>,
    #[serde(rename = "DefaultScript")]
    pub default_script: Option<u32>,
    #[serde(rename = "DefaultScriptIntensity")]
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
    #[serde(rename = "Strength")]
    pub strength: Option<u32>,
    #[serde(rename = "Endurance")]
    pub endurance: Option<u32>,
    #[serde(rename = "Quickness")]
    pub quickness: Option<u32>,
    #[serde(rename = "Coordination")]
    pub coordination: Option<u32>,
    #[serde(rename = "Focus")]
    pub focus: Option<u32>,
    #[serde(rename = "Self")]
    pub self_: Option<u32>,
    #[serde(rename = "Stamina")]
    pub stamina: Option<u32>,
    #[serde(rename = "Mana")]
    pub mana: Option<u32>,
    #[serde(rename = "StaminaMax")]
    pub stamina_max: Option<u32>,
    #[serde(rename = "ManaMax")]
    pub mana_max: Option<u32>,
    #[serde(rename = "AttrHighlight")]
    pub attr_highlight: Option<AttributeMask>,
    #[serde(rename = "AttrColor")]
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
    pub account_hash: PackableHashTable<string, uint>,
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
#[serde(tag = "Type")]
pub enum GameMoveData {
    #[serde(rename = "0x4")]
    Type4 {
    #[serde(rename = "PlayerId")]
    player_id: ObjectId,
    #[serde(rename = "Team")]
    team: i32,
    #[serde(rename = "IdPieceToMove")]
    id_piece_to_move: i32,
    #[serde(rename = "YGrid")]
    y_grid: i32,
    },
    #[serde(rename = "0x5")]
    Type5 {
    #[serde(rename = "PlayerId")]
    player_id: ObjectId,
    #[serde(rename = "Team")]
    team: i32,
    #[serde(rename = "IdPieceToMove")]
    id_piece_to_move: i32,
    #[serde(rename = "YGrid")]
    y_grid: i32,
    #[serde(rename = "XTo")]
    x_to: i32,
    #[serde(rename = "YTo")]
    y_to: i32,
    },
    #[serde(rename = "0x6")]
    Type6 {
    #[serde(rename = "PlayerId")]
    player_id: ObjectId,
    #[serde(rename = "Team")]
    team: i32,
    #[serde(rename = "IdPieceToMove")]
    id_piece_to_move: i32,
    },
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
    pub recently_departed: PackableHashTable<ObjectId, int>,
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
    pub contact_trackers: PackableHashTable<uint, ContractTracker>,
}

// Client to Server AC packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct C2SPacket {
    #[serde(rename = "Sequence")]
    pub sequence: u32,
    #[serde(rename = "Flags")]
    pub flags: PacketHeaderFlags,
    #[serde(rename = "Checksum")]
    pub checksum: u32,
    #[serde(rename = "RecipientId")]
    pub recipient_id: u16,
    #[serde(rename = "TimeSinceLastPacket")]
    pub time_since_last_packet: u16,
    #[serde(rename = "Size")]
    pub size: u16,
    #[serde(rename = "Iteration")]
    pub iteration: u16,
    #[serde(rename = "ServerSwitch")]
    pub server_switch: Option<ServerSwitchHeader>,
    #[serde(rename = "RetransmitSequences")]
    pub retransmit_sequences: Option<PackableList<uint>>,
    #[serde(rename = "RejectSequences")]
    pub reject_sequences: Option<PackableList<uint>>,
    #[serde(rename = "AckSequence")]
    pub ack_sequence: Option<u32>,
    #[serde(rename = "LoginRequest")]
    pub login_request: Option<LoginRequestHeader>,
    #[serde(rename = "WorldLoginRequest")]
    pub world_login_request: Option<u64>,
    #[serde(rename = "ConnectResponse")]
    pub connect_response: Option<u64>,
    #[serde(rename = "CICMDCommand")]
    pub cicmd_command: Option<CICMDCommandHeader>,
    #[serde(rename = "Time")]
    pub time: Option<u64>,
    #[serde(rename = "EchoTime")]
    pub echo_time: Option<f32>,
    #[serde(rename = "Flow")]
    pub flow: Option<FlowHeader>,
    #[serde(rename = "Fragments")]
    pub fragments: Option<BlobFragments>,
}

// Server to Client AC packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct S2CPacket {
    #[serde(rename = "Sequence")]
    pub sequence: u32,
    #[serde(rename = "Flags")]
    pub flags: PacketHeaderFlags,
    #[serde(rename = "Checksum")]
    pub checksum: u32,
    #[serde(rename = "RecipientId")]
    pub recipient_id: u16,
    #[serde(rename = "TimeSinceLastPacket")]
    pub time_since_last_packet: u16,
    #[serde(rename = "Size")]
    pub size: u16,
    #[serde(rename = "Iteration")]
    pub iteration: u16,
    #[serde(rename = "AckSequence")]
    pub ack_sequence: Option<u32>,
    #[serde(rename = "LogonServerAddr")]
    pub logon_server_addr: Option<SocketAddress>,
    #[serde(rename = "Referral")]
    pub referral: Option<ReferralHeader>,
    #[serde(rename = "ConnectRequest")]
    pub connect_request: Option<ConnectRequestHeader>,
    #[serde(rename = "NetError")]
    pub net_error: Option<NetError>,
    #[serde(rename = "NetErrorDisconnect")]
    pub net_error_disconnect: Option<NetError>,
    #[serde(rename = "EchoResponse")]
    pub echo_response: Option<EchoResponseHeader>,
    #[serde(rename = "Fragments")]
    pub fragments: Option<BlobFragments>,
}

