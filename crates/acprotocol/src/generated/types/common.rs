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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayeredSpellId {
        #[serde(rename = "Id")]
        id: SpellId,
        #[serde(rename = "Layer")]
        layer: u16
}

// List which is packable for network
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackableList<T> {
        #[serde(rename = "Count")]
        count: u32,
        #[serde(rename = "List")]
        list: Vec<T>
}

// HashTable which is packable for network
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackableHashTable<T,U>
where
    T: std::hash::Hash + Eq + serde::Serialize + serde::de::DeserializeOwned,
    U: serde::Serialize + serde::de::DeserializeOwned {
        #[serde(rename = "Count")]
        count: u16,
        #[serde(rename = "MaxSize")]
        max_size: u16,
        #[serde(rename = "Table")]
        table: std::collections::HashMap<T, U>
}

// HashTable which is packable for network
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PHashTable<T,U>
where
    T: std::hash::Hash + Eq + serde::Serialize + serde::de::DeserializeOwned,
    U: serde::Serialize + serde::de::DeserializeOwned {
        #[serde(rename = "PackedSize")]
        packed_size: u32,
        #[serde(rename = "Table")]
        table: std::collections::HashMap<T, U>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
        #[serde(rename = "X")]
        x: f32,
        #[serde(rename = "Y")]
        y: f32,
        #[serde(rename = "Z")]
        z: f32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
        #[serde(rename = "W")]
        w: f32,
        #[serde(rename = "X")]
        x: f32,
        #[serde(rename = "Y")]
        y: f32,
        #[serde(rename = "Z")]
        z: f32
}

// Landcell location, without orientation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Origin {
        #[serde(rename = "Landcell")]
        landcell: LandcellId,
        #[serde(rename = "Location")]
        location: Vector3
}

// Landcell location, including orientation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
        #[serde(rename = "Landcell")]
        landcell: LandcellId,
        #[serde(rename = "Frame")]
        frame: Frame
}

// A the location and orientation of an object within a landcell
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Frame {
        #[serde(rename = "Origin")]
        origin: Vector3,
        #[serde(rename = "Orientation")]
        orientation: Quaternion
}

// Optional header data when PacketHeaderFlags includes ServerSwitch
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerSwitchHeader {
        #[serde(rename = "Sequence")]
        sequence: u32,
        #[serde(rename = "Type")]
        type_: ServerSwitchType
}

// Optional header data when PacketHeaderFlags includes CICMDCommand
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CICMDCommandHeader {
        #[serde(rename = "Command")]
        command: u32,
        #[serde(rename = "Parameter")]
        parameter: u32
}

// Optional header data when PacketHeaderFlags includes Flow
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlowHeader {
        #[serde(rename = "Bytes")]
        bytes: u32,
        #[serde(rename = "Interval")]
        interval: u16
}

// Optional header data when PacketHeaderFlags includes LogonServerAddr
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocketAddress {
        #[serde(rename = "Family")]
        family: i16,
        #[serde(rename = "Port")]
        port: u16,
        #[serde(rename = "Address")]
        address: u32,
        #[serde(rename = "Empty")]
        empty: u64
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
        cookie: u64,
        #[serde(rename = "Address")]
        address: SocketAddress,
        #[serde(rename = "IdServer")]
        id_server: u16,
        #[serde(rename = "Unknown")]
        unknown: DWORD
}

// Optional header data when PacketHeaderFlags includes ConnectRequest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectRequestHeader {
        #[serde(rename = "ServerTime")]
        server_time: f64,
        #[serde(rename = "Cookie")]
        cookie: u64,
        #[serde(rename = "NetID")]
        net_id: i32,
        #[serde(rename = "OutgoingSeed")]
        outgoing_seed: u32,
        #[serde(rename = "IncomingSeed")]
        incoming_seed: u32,
        #[serde(rename = "Unknown")]
        unknown: DWORD
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetError {
        #[serde(rename = "StringId")]
        string_id: DataId,
        #[serde(rename = "TableId")]
        table_id: DataId
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EchoResponseHeader {
        #[serde(rename = "LocalTime")]
        local_time: f32,
        #[serde(rename = "HoldingTime")]
        holding_time: f32
}

// A collection of property tables.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ACBaseQualities {
        #[serde(rename = "Flags")]
        flags: ACBaseQualitiesFlags,
        #[serde(rename = "WeenieType")]
        weenie_type: WeenieType,
        #[serde(rename = "IntProperties")]
        int_properties: Option<PackableHashTable<PropertyInt, int>>,
        #[serde(rename = "Int64Properties")]
        int64_properties: Option<PackableHashTable<PropertyInt64, long>>,
        #[serde(rename = "BoolProperties")]
        bool_properties: Option<PackableHashTable<PropertyBool, bool>>,
        #[serde(rename = "FloatProperties")]
        float_properties: Option<PackableHashTable<PropertyFloat, double>>,
        #[serde(rename = "StringProperties")]
        string_properties: Option<PackableHashTable<PropertyString, string>>,
        #[serde(rename = "DataProperties")]
        data_properties: Option<PackableHashTable<PropertyDataId, DataId>>,
        #[serde(rename = "InstanceProperties")]
        instance_properties: Option<PackableHashTable<PropertyInstanceId, ObjectId>>,
        #[serde(rename = "PositionProperties")]
        position_properties: Option<PackableHashTable<PropertyPosition, Position>>
}

// The ACQualities structure contains character property lists.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ACQualities {
        #[serde(rename = "Flags")]
        flags: ACQualitiesFlags,
        #[serde(rename = "HasHealth")]
        has_health: bool,
        #[serde(rename = "Attributes")]
        attributes: Option<AttributeCache>,
        #[serde(rename = "Skills")]
        skills: Option<PackableHashTable<Skill, Skill>>,
        #[serde(rename = "Body")]
        body: Option<Body>,
        #[serde(rename = "SpellBook")]
        spell_book: Option<PackableHashTable<LayeredSpellId, SpellBookPage>>,
        #[serde(rename = "Enchantments")]
        enchantments: Option<EnchantmentRegistry>,
        #[serde(rename = "EventFilter")]
        event_filter: Option<EventFilter>,
        #[serde(rename = "Emotes")]
        emotes: Option<EmoteTable>,
        #[serde(rename = "CreationProfile")]
        creation_profile: Option<PackableList<CreationProfile>>,
        #[serde(rename = "PageData")]
        page_data: Option<PageDataList>,
        #[serde(rename = "Generators")]
        generators: Option<GeneratorTable>,
        #[serde(rename = "GeneratorRegistry")]
        generator_registry: Option<GeneratorRegistry>,
        #[serde(rename = "GeneratorQueue")]
        generator_queue: Option<GeneratorQueue>
}

// The AttributeCache structure contains information about a character attributes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeCache {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Strength")]
        strength: Option<AttributeInfo>,
        #[serde(rename = "Endurance")]
        endurance: Option<AttributeInfo>,
        #[serde(rename = "Quickness")]
        quickness: Option<AttributeInfo>,
        #[serde(rename = "Coordination")]
        coordination: Option<AttributeInfo>,
        #[serde(rename = "Focus")]
        focus: Option<AttributeInfo>,
        #[serde(rename = "Self")]
        self_: Option<AttributeInfo>,
        #[serde(rename = "Health")]
        health: Option<SecondaryAttributeInfo>,
        #[serde(rename = "Stamina")]
        stamina: Option<SecondaryAttributeInfo>,
        #[serde(rename = "Mana")]
        mana: Option<SecondaryAttributeInfo>
}

// The Attribute structure contains information about a character attribute.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeInfo {
        #[serde(rename = "PointsRaised")]
        points_raised: u32,
        #[serde(rename = "InnatePoints")]
        innate_points: u32,
        #[serde(rename = "ExperienceSpent")]
        experience_spent: u32
}

// The SecondaryAttribute structure contains information about a character vital.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecondaryAttributeInfo {
        #[serde(rename = "Attribute")]
        attribute: AttributeInfo,
        #[serde(rename = "Current")]
        current: u32
}

// The Skill structure contains information about a character skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Skill {
        #[serde(rename = "PointsRaised")]
        points_raised: u16,
        #[serde(rename = "AdjustPP")]
        adjust_pp: u16,
        #[serde(rename = "TrainingLevel")]
        training_level: SkillAdvancementClass,
        #[serde(rename = "ExperienceSpent")]
        experience_spent: u32,
        #[serde(rename = "InnatePoints")]
        innate_points: u32,
        #[serde(rename = "ResistanceOfLastCheck")]
        resistance_of_last_check: u32,
        #[serde(rename = "LastUsedTime")]
        last_used_time: f64
}

// Contains body part table
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Body {
        #[serde(rename = "BodyParts")]
        body_parts: PackableHashTable<uint, BodyPart>
}

// Information on individual body parts. (Needs to be confirmed if this was used in prod)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BodyPart {
        #[serde(rename = "HasBPSD")]
        has_bpsd: i32,
        #[serde(rename = "DamageType")]
        damage_type: DamageType,
        #[serde(rename = "DamageVal")]
        damage_val: i32,
        #[serde(rename = "DamageVar")]
        damage_var: i32,
        #[serde(rename = "ArmorCache")]
        armor_cache: ArmorCache,
        #[serde(rename = "BH")]
        bh: i32,
        #[serde(rename = "BPSD")]
        bpsd: Option<BodyPartSelectionData>
}

// Information on armor levels
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmorCache {
        #[serde(rename = "BaseArmor")]
        base_armor: i32,
        #[serde(rename = "ArmorVsSlash")]
        armor_vs_slash: i32,
        #[serde(rename = "ArmorVsPierce")]
        armor_vs_pierce: i32,
        #[serde(rename = "ArmorVsBludgeon")]
        armor_vs_bludgeon: i32,
        #[serde(rename = "ArmorVsCold")]
        armor_vs_cold: i32,
        #[serde(rename = "ArmorVsFire")]
        armor_vs_fire: i32,
        #[serde(rename = "ArmorVsAcid")]
        armor_vs_acid: i32,
        #[serde(rename = "ArmorVsElectric")]
        armor_vs_electric: i32,
        #[serde(rename = "ArmorVsNether")]
        armor_vs_nether: i32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BodyPartSelectionData {
        #[serde(rename = "HLF")]
        hlf: i32,
        #[serde(rename = "MLF")]
        mlf: i32,
        #[serde(rename = "LLF")]
        llf: i32,
        #[serde(rename = "HRF")]
        hrf: i32,
        #[serde(rename = "MRF")]
        mrf: i32,
        #[serde(rename = "LRF")]
        lrf: i32,
        #[serde(rename = "HLB")]
        hlb: i32,
        #[serde(rename = "MLB")]
        mlb: i32,
        #[serde(rename = "LLB")]
        llb: i32,
        #[serde(rename = "HRB")]
        hrb: i32,
        #[serde(rename = "MRB")]
        mrb: i32,
        #[serde(rename = "LRB")]
        lrb: i32
}

// Contains information related to the spell in your spellbook
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpellBookPage {
        #[serde(rename = "CastingLikelihood")]
        casting_likelihood: f32,
        #[serde(rename = "CastingLikelihood2")]
        casting_likelihood2: Option<f32>,
        #[serde(rename = "Unknown")]
        unknown: Option<i32>
}

// Contains information related to the spells in effect on the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnchantmentRegistry {
        #[serde(rename = "Flags")]
        flags: EnchantmentRegistryFlags,
        #[serde(rename = "LifeSpells")]
        life_spells: Option<PackableList<Enchantment>>,
        #[serde(rename = "CreatureSpells")]
        creature_spells: Option<PackableList<Enchantment>>,
        #[serde(rename = "Vitae")]
        vitae: Option<Enchantment>,
        #[serde(rename = "Cooldowns")]
        cooldowns: Option<PackableList<Enchantment>>
}

// The Enchantment structure describes an active enchantment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Enchantment {
        #[serde(rename = "Id")]
        id: LayeredSpellId,
        #[serde(rename = "HasEquipmentSet")]
        has_equipment_set: u16,
        #[serde(rename = "SpellCategory")]
        spell_category: SpellCategory,
        #[serde(rename = "PowerLevel")]
        power_level: u32,
        #[serde(rename = "StartTime")]
        start_time: f64,
        #[serde(rename = "Duration")]
        duration: f64,
        #[serde(rename = "CasterId")]
        caster_id: ObjectId,
        #[serde(rename = "DegradeModifier")]
        degrade_modifier: f32,
        #[serde(rename = "DegradeLimit")]
        degrade_limit: f32,
        #[serde(rename = "LastTimeDegraded")]
        last_time_degraded: f64,
        #[serde(rename = "StatMod")]
        stat_mod: StatMod,
        #[serde(rename = "EquipmentSet")]
        equipment_set: Option<EquipmentSet>
}

// Information on stat modification
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatMod {
        #[serde(rename = "Type")]
        type_: EnchantmentTypeFlags,
        #[serde(rename = "Key")]
        key: u32,
        #[serde(rename = "Value")]
        value: f32
}

// Contains a list of events to filter? Unknown what this does currently.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventFilter {
        #[serde(rename = "Events")]
        events: PackableList<uint>
}

// Contains a list of emotes for NPCs? Unknown what this does currently.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteTable {
        #[serde(rename = "Emotes")]
        emotes: PackableHashTable<EmoteCategory, EmoteSetList>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmoteSetList {
        #[serde(rename = "Emotes")]
        emotes: PackableList<EmoteSet>
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
        weenie_class_id: u32,
        #[serde(rename = "Palette")]
        palette: u32,
        #[serde(rename = "Shade")]
        shade: f32,
        #[serde(rename = "Destination")]
        destination: u32,
        #[serde(rename = "StackSize")]
        stack_size: i32,
        #[serde(rename = "TryToBond")]
        try_to_bond: bool
}

// List of pages in a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageDataList {
        #[serde(rename = "MaxNumPages")]
        max_num_pages: u32,
        #[serde(rename = "MaxNumCharsPerPage")]
        max_num_chars_per_page: u32,
        #[serde(rename = "Pages")]
        pages: PackableList<PageData>
}

// Data for an individual page
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageData {
        #[serde(rename = "AuthorId")]
        author_id: ObjectId,
        #[serde(rename = "AuthorName")]
        author_name: String,
        #[serde(rename = "AuthorAccount")]
        author_account: String,
        #[serde(rename = "Version")]
        version: u32,
        #[serde(rename = "TextIncluded")]
        text_included: bool,
        #[serde(rename = "IgnoreAuthor")]
        ignore_author: bool,
        #[serde(rename = "PageText")]
        page_text: Option<String>
}

// Blob fragment data used to contruct message data. These can be spread across multiple packets
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobFragments {
        #[serde(rename = "Sequence")]
        sequence: u32,
        #[serde(rename = "Id")]
        id: u32,
        #[serde(rename = "Count")]
        count: u16,
        #[serde(rename = "Size")]
        size: u16,
        #[serde(rename = "Index")]
        index: u16,
        #[serde(rename = "Group")]
        group: FragmentGroup,
        #[serde(rename = "Data")]
        data: Vec<byte>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorTable {
        #[serde(rename = "Generators")]
        generators: PackableList<GeneratorProfile>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorProfile {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "TypeId")]
        type_id: u32,
        #[serde(rename = "Delay")]
        delay: f64,
        #[serde(rename = "InitCreate")]
        init_create: u32,
        #[serde(rename = "MaxNum")]
        max_num: u32,
        #[serde(rename = "WhenCreate")]
        when_create: u32,
        #[serde(rename = "WhereCreate")]
        where_create: u32,
        #[serde(rename = "StackSize")]
        stack_size: u32,
        #[serde(rename = "Ptid")]
        ptid: u32,
        #[serde(rename = "Shade")]
        shade: f32,
        #[serde(rename = "PosVal")]
        pos_val: Position,
        #[serde(rename = "Slot")]
        slot: u32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorRegistry {
        #[serde(rename = "Registry")]
        registry: PackableHashTable<uint, GeneratorRegistryNode>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorRegistryNode {
        #[serde(rename = "WcidOrType")]
        wcid_or_type: u32,
        #[serde(rename = "Ts")]
        ts: f64,
        #[serde(rename = "TreasureType")]
        treasure_type: u32,
        #[serde(rename = "Slot")]
        slot: u32,
        #[serde(rename = "Checkpointed")]
        checkpointed: u32,
        #[serde(rename = "Shop")]
        shop: u32,
        #[serde(rename = "Amount")]
        amount: u32
}

// Set of inventory items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorQueue {
        #[serde(rename = "Queue")]
        queue: PackableList<GeneratorQueueNode>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorQueueNode {
        #[serde(rename = "Slot")]
        slot: u32,
        #[serde(rename = "When")]
        when: f64
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
        size: u32,
        #[serde(rename = "Unknown200_2")]
        unknown200_2: u8,
        #[serde(rename = "OptionPropertyCount")]
        option_property_count: u8,
        #[serde(rename = "OptionProperties")]
        option_properties: Vec<OptionProperty>
}

// The PlayerModule structure contains character options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerModule {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Options")]
        options: CharacterOptions1,
        #[serde(rename = "Shortcuts")]
        shortcuts: Option<PackableList<ShortCutData>>,
        #[serde(rename = "Tab1Spells")]
        tab1_spells: PackableList<LayeredSpellId>,
        #[serde(rename = "Tab2Spells")]
        tab2_spells: PackableList<LayeredSpellId>,
        #[serde(rename = "Tab3Spells")]
        tab3_spells: PackableList<LayeredSpellId>,
        #[serde(rename = "Tab4Spells")]
        tab4_spells: PackableList<LayeredSpellId>,
        #[serde(rename = "Tab5Spells")]
        tab5_spells: PackableList<LayeredSpellId>,
        #[serde(rename = "Tab6Spells")]
        tab6_spells: PackableList<LayeredSpellId>,
        #[serde(rename = "Tab7Spells")]
        tab7_spells: PackableList<LayeredSpellId>,
        #[serde(rename = "Tab8Spells")]
        tab8_spells: PackableList<LayeredSpellId>,
        #[serde(rename = "FillComps")]
        fill_comps: Option<PackableHashTable<uint, uint>>,
        #[serde(rename = "SpellBookFilters")]
        spell_book_filters: Option<u32>,
        #[serde(rename = "OptionFlags")]
        option_flags: Option<u32>,
        #[serde(rename = "Unknown100_1")]
        unknown100_1: Option<u32>,
        #[serde(rename = "OptionStrings")]
        option_strings: Option<PackableHashTable<uint, string>>,
        #[serde(rename = "GameplayOptions")]
        gameplay_options: Option<GameplayOptions>
}

// Set of shortcuts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortCutManager {
        #[serde(rename = "Shortcuts")]
        shortcuts: PackableList<ShortCutData>
}

// Shortcut
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortCutData {
        #[serde(rename = "Index")]
        index: u32,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// List of spells in spell tab
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpellTab {
        #[serde(rename = "Spells")]
        spells: PackableList<LayeredSpellId>
}

// Set of inventory items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentProfile {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ContainerType")]
        container_type: ContainerProperties
}

// Set of inventory items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryPlacement {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Location")]
        location: EquipMask,
        #[serde(rename = "Priority")]
        priority: CoverageMask
}

// Allegience information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllegianceProfile {
        #[serde(rename = "TotalMembers")]
        total_members: u32,
        #[serde(rename = "TotalVassals")]
        total_vassals: u32,
        #[serde(rename = "Hierarchy")]
        hierarchy: AllegianceHierarchy
}

// Allegience record
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllegianceRecord {
        #[serde(rename = "TreeParent")]
        tree_parent: ObjectId,
        #[serde(rename = "AllegianceData")]
        allegiance_data: AllegianceData
}

// Allegience hierarchy information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllegianceHierarchy {
        #[serde(rename = "RecordCount")]
        record_count: u16,
        #[serde(rename = "OldVersion")]
        old_version: u16,
        #[serde(rename = "Officers")]
        officers: PHashTable<ObjectId, AllegianceOfficerLevel>,
        #[serde(rename = "OfficerTitles")]
        officer_titles: PackableList<string>,
        #[serde(rename = "MonarchBroadcastTime")]
        monarch_broadcast_time: u32,
        #[serde(rename = "MonarchBroadcastsToday")]
        monarch_broadcasts_today: u32,
        #[serde(rename = "SpokesBroadcastTime")]
        spokes_broadcast_time: u32,
        #[serde(rename = "SpokesBroadcastsToday")]
        spokes_broadcasts_today: u32,
        #[serde(rename = "Motd")]
        motd: String,
        #[serde(rename = "MotdSetBy")]
        motd_set_by: String,
        #[serde(rename = "ChatRoomId")]
        chat_room_id: u32,
        #[serde(rename = "Bindpoint")]
        bindpoint: Position,
        #[serde(rename = "AllegianceName")]
        allegiance_name: String,
        #[serde(rename = "NameLastSetTime")]
        name_last_set_time: u32,
        #[serde(rename = "IsLocked")]
        is_locked: bool,
        #[serde(rename = "ApprovedVassal")]
        approved_vassal: i32,
        #[serde(rename = "MonarchData")]
        monarch_data: Option<AllegianceData>,
        #[serde(rename = "Records")]
        records: Vec<AllegianceRecord>
}

// Set of allegiance data for a specific player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllegianceData {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId,
        #[serde(rename = "XPCached")]
        xp_cached: u32,
        #[serde(rename = "XPTithed")]
        xp_tithed: u32,
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Gender")]
        gender: Gender,
        #[serde(rename = "Heritage")]
        heritage: HeritageGroup,
        #[serde(rename = "Rank")]
        rank: u16,
        #[serde(rename = "Level")]
        level: Option<u32>,
        #[serde(rename = "Loyalty")]
        loyalty: u16,
        #[serde(rename = "Leadership")]
        leadership: u16,
        #[serde(rename = "AllegianceAge")]
        allegiance_age: Option<u32>,
        #[serde(rename = "TimeOnline")]
        time_online: Option<u64>,
        #[serde(rename = "Name")]
        name: String
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendData {
        #[serde(rename = "FriendId")]
        friend_id: ObjectId,
        #[serde(rename = "Online")]
        online: bool,
        #[serde(rename = "AppearOffline")]
        appear_offline: bool,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "OutFriends")]
        out_friends: PackableList<ObjectId>,
        #[serde(rename = "InFriends")]
        in_friends: PackableList<ObjectId>
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
        header: u32,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "WeenieClassId")]
        weenie_class_id: PackedDWORD,
        #[serde(rename = "Icon")]
        icon: PackedDWORD,
        #[serde(rename = "Type")]
        type_: ItemType,
        #[serde(rename = "Behavior")]
        behavior: ObjectDescriptionFlag,
        #[serde(rename = "Header2")]
        header2: Option<u32>,
        #[serde(rename = "PluralName")]
        plural_name: Option<String>,
        #[serde(rename = "ItemsCapacity")]
        items_capacity: Option<u8>,
        #[serde(rename = "ContainerCapacity")]
        container_capacity: Option<u8>,
        #[serde(rename = "AmmunitionType")]
        ammunition_type: Option<AmmoType>,
        #[serde(rename = "Value")]
        value: Option<u32>,
        #[serde(rename = "Useability")]
        useability: Option<Usable>,
        #[serde(rename = "UseRadius")]
        use_radius: Option<f32>,
        #[serde(rename = "TargetType")]
        target_type: Option<ItemType>,
        #[serde(rename = "Effects")]
        effects: Option<IconHighlight>,
        #[serde(rename = "CombatUse")]
        combat_use: Option<WieldType>,
        #[serde(rename = "Structure")]
        structure: Option<u16>,
        #[serde(rename = "MaxStructure")]
        max_structure: Option<u16>,
        #[serde(rename = "StackSize")]
        stack_size: Option<u16>,
        #[serde(rename = "MaxStackSize")]
        max_stack_size: Option<u16>,
        #[serde(rename = "ContainerId")]
        container_id: Option<ObjectId>,
        #[serde(rename = "WielderId")]
        wielder_id: Option<ObjectId>,
        #[serde(rename = "ValidSlots")]
        valid_slots: Option<EquipMask>,
        #[serde(rename = "Slot")]
        slot: Option<EquipMask>,
        #[serde(rename = "Priority")]
        priority: Option<CoverageMask>,
        #[serde(rename = "BlipColor")]
        blip_color: Option<RadarColor>,
        #[serde(rename = "RadarEnum")]
        radar_enum: Option<RadarBehavior>,
        #[serde(rename = "PhysicsScript")]
        physics_script: Option<u16>,
        #[serde(rename = "Workmanship")]
        workmanship: Option<f32>,
        #[serde(rename = "Burden")]
        burden: Option<u16>,
        #[serde(rename = "SpellId")]
        spell_id: Option<SpellId>,
        #[serde(rename = "OwnerId")]
        owner_id: Option<ObjectId>,
        #[serde(rename = "Restrictions")]
        restrictions: Option<RestrictionDB>,
        #[serde(rename = "HookItemTypes")]
        hook_item_types: Option<HookType>,
        #[serde(rename = "MonarchId")]
        monarch_id: Option<ObjectId>,
        #[serde(rename = "HookType")]
        hook_type: Option<HookType>,
        #[serde(rename = "IconOverlay")]
        icon_overlay: Option<PackedDWORD>,
        #[serde(rename = "IconUnderlay")]
        icon_underlay: Option<PackedDWORD>,
        #[serde(rename = "Material")]
        material: Option<MaterialType>,
        #[serde(rename = "CooldownId")]
        cooldown_id: Option<u32>,
        #[serde(rename = "CooldownDuration")]
        cooldown_duration: Option<u64>,
        #[serde(rename = "PetOwnerId")]
        pet_owner_id: Option<ObjectId>
}

// The RestrictionDB contains the access control list for a dwelling object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestrictionDB {
        #[serde(rename = "Version")]
        version: u32,
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "MonarchId")]
        monarch_id: ObjectId,
        #[serde(rename = "Permissions")]
        permissions: PHashTable<ObjectId, uint>
}

// The OldPublicWeenieDesc structure defines an object's game behavior.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OldPublicWeenieDesc {
        #[serde(rename = "Header")]
        header: u32,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "WeenieClassId")]
        weenie_class_id: PackedDWORD,
        #[serde(rename = "Icon")]
        icon: PackedDWORD,
        #[serde(rename = "Type")]
        type_: ItemType,
        #[serde(rename = "Bitfield")]
        bitfield: ObjectDescriptionFlag,
        #[serde(rename = "PluralName")]
        plural_name: Option<String>,
        #[serde(rename = "ItemsCapacity")]
        items_capacity: Option<u8>,
        #[serde(rename = "ContainerCapacity")]
        container_capacity: Option<u8>,
        #[serde(rename = "Value")]
        value: Option<u32>,
        #[serde(rename = "Useability")]
        useability: Option<Usable>,
        #[serde(rename = "UseRadius")]
        use_radius: Option<f32>,
        #[serde(rename = "tTargetType")]
        t_target_type: Option<ItemType>,
        #[serde(rename = "Effects")]
        effects: Option<IconHighlight>,
        #[serde(rename = "AmmunitionType")]
        ammunition_type: Option<AmmoType>,
        #[serde(rename = "CombatUse")]
        combat_use: Option<WieldType>,
        #[serde(rename = "Structure")]
        structure: Option<u16>,
        #[serde(rename = "MaxStructure")]
        max_structure: Option<u16>,
        #[serde(rename = "StackSize")]
        stack_size: Option<u16>,
        #[serde(rename = "MaxStackSize")]
        max_stack_size: Option<u16>,
        #[serde(rename = "ContainerId")]
        container_id: Option<ObjectId>,
        #[serde(rename = "WielderId")]
        wielder_id: Option<ObjectId>,
        #[serde(rename = "ValidSlots")]
        valid_slots: Option<EquipMask>,
        #[serde(rename = "Slots")]
        slots: Option<EquipMask>,
        #[serde(rename = "Priority")]
        priority: Option<CoverageMask>,
        #[serde(rename = "BlipColor")]
        blip_color: Option<RadarColor>,
        #[serde(rename = "RadarEnum")]
        radar_enum: Option<RadarBehavior>,
        #[serde(rename = "ObviousDistance")]
        obvious_distance: Option<f32>,
        #[serde(rename = "Vndwcid")]
        vndwcid: Option<u16>,
        #[serde(rename = "SpellId")]
        spell_id: Option<SpellId>,
        #[serde(rename = "HouseOwnerId")]
        house_owner_id: Option<ObjectId>,
        #[serde(rename = "PhysicsScript")]
        physics_script: Option<u16>,
        #[serde(rename = "Restrictions")]
        restrictions: Option<RestrictionDB>,
        #[serde(rename = "HookType")]
        hook_type: Option<HookType>,
        #[serde(rename = "HookItemTypes")]
        hook_item_types: Option<HookType>,
        #[serde(rename = "MonarchId")]
        monarch_id: Option<ObjectId>,
        #[serde(rename = "IconOverlay")]
        icon_overlay: Option<PackedDWORD>,
        #[serde(rename = "Material")]
        material: Option<MaterialType>
}

// Information related to a secure trade.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trade {
        #[serde(rename = "PartnerId")]
        partner_id: ObjectId,
        #[serde(rename = "Sequence")]
        sequence: u64,
        #[serde(rename = "Status")]
        status: u32,
        #[serde(rename = "InitiatorId")]
        initiator_id: ObjectId,
        #[serde(rename = "Accepted")]
        accepted: bool,
        #[serde(rename = "PartnerAccepted")]
        partner_accepted: bool
}

// A jump with sequences
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JumpPack {
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Velocity")]
        velocity: Vector3,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16,
        #[serde(rename = "ObjectForcePositionSequence")]
        object_force_position_sequence: u16
}

// A set of data related to changing states with sequences
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveToStatePack {
        #[serde(rename = "RawMotionState")]
        raw_motion_state: RawMotionState,
        #[serde(rename = "Position")]
        position: Position,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16,
        #[serde(rename = "ObjectForcePositionSequence")]
        object_force_position_sequence: u16,
        #[serde(rename = "Contact")]
        contact: u8
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackedMotionCommand {
        #[serde(rename = "CommandId")]
        command_id: Command,
        #[serde(rename = "PackedSequence")]
        packed_sequence: u16,
        #[serde(rename = "Speed")]
        speed: f32
}

// Data related to the movement of the object sent from a client
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawMotionState {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "CurrentHoldkey")]
        current_holdkey: Option<HoldKey>,
        #[serde(rename = "CurrentStyle")]
        current_style: Option<StanceMode>,
        #[serde(rename = "ForwardCommand")]
        forward_command: Option<MovementCommand>,
        #[serde(rename = "ForwardHoldkey")]
        forward_holdkey: Option<HoldKey>,
        #[serde(rename = "ForwardSpeed")]
        forward_speed: Option<f32>,
        #[serde(rename = "SidestepCommand")]
        sidestep_command: Option<MovementCommand>,
        #[serde(rename = "SidestepHoldkey")]
        sidestep_holdkey: Option<HoldKey>,
        #[serde(rename = "SidestepSpeed")]
        sidestep_speed: Option<f32>,
        #[serde(rename = "TurnCommand")]
        turn_command: Option<MovementCommand>,
        #[serde(rename = "TurnHoldkey")]
        turn_holdkey: Option<u32>,
        #[serde(rename = "TurnSpeed")]
        turn_speed: Option<f32>,
        #[serde(rename = "Commands")]
        commands: Vec<PackedMotionCommand>
}

// An autonomous position with sequences
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutonomousPositionPack {
        #[serde(rename = "Position")]
        position: Position,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16,
        #[serde(rename = "ObjectForcePositionSequence")]
        object_force_position_sequence: u16,
        #[serde(rename = "Contact")]
        contact: u8
}

// A position with sequences
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PositionPack {
        #[serde(rename = "Flags")]
        flags: PositionFlags,
        #[serde(rename = "Origin")]
        origin: Origin,
        #[serde(rename = "WQuat")]
        w_quat: Option<f32>,
        #[serde(rename = "XQuat")]
        x_quat: Option<f32>,
        #[serde(rename = "YQuat")]
        y_quat: Option<f32>,
        #[serde(rename = "ZQuat")]
        z_quat: Option<f32>,
        #[serde(rename = "Velocity")]
        velocity: Option<Vector3>,
        #[serde(rename = "PlacementId")]
        placement_id: Option<u32>,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectPositionSequence")]
        object_position_sequence: u16,
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16,
        #[serde(rename = "ObjectForcePositionSequence")]
        object_force_position_sequence: u16
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
        flags: u32,
        #[serde(rename = "CurrentStyle")]
        current_style: Option<StanceMode>,
        #[serde(rename = "ForwardCommand")]
        forward_command: Option<MovementCommand>,
        #[serde(rename = "SidestepCommand")]
        sidestep_command: Option<MovementCommand>,
        #[serde(rename = "TurnCommand")]
        turn_command: Option<MovementCommand>,
        #[serde(rename = "ForwardSpeed")]
        forward_speed: Option<f32>,
        #[serde(rename = "SidestepSpeed")]
        sidestep_speed: Option<f32>,
        #[serde(rename = "TurnSpeed")]
        turn_speed: Option<f32>,
        #[serde(rename = "Commands")]
        commands: Vec<PackedMotionCommand>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DDDRevision {
        #[serde(rename = "IdDatFile")]
        id_dat_file: u64,
        #[serde(rename = "Iteration")]
        iteration: u32,
        #[serde(rename = "IdsToDownload")]
        ids_to_download: PackableList<DataId>,
        #[serde(rename = "IdsToPurge")]
        ids_to_purge: PackableList<DataId>
}

// Set of movement parameters required for a MoveTo movement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveToMovementParameters {
        #[serde(rename = "Bitmember")]
        bitmember: u32,
        #[serde(rename = "DistanceToObject")]
        distance_to_object: f32,
        #[serde(rename = "MinDistance")]
        min_distance: f32,
        #[serde(rename = "FailDistance")]
        fail_distance: f32,
        #[serde(rename = "AnimationSpeed")]
        animation_speed: f32,
        #[serde(rename = "WalkRunThreshold")]
        walk_run_threshold: f32,
        #[serde(rename = "DesiredHeading")]
        desired_heading: f32
}

// Set of movement parameters required for a TurnTo motion
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TurnToMovementParameters {
        #[serde(rename = "Bitmember")]
        bitmember: u32,
        #[serde(rename = "AnimationSpeed")]
        animation_speed: f32,
        #[serde(rename = "DesiredHeading")]
        desired_heading: f32
}

// The ObjDesc structure defines an object's visual appearance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjDesc {
        #[serde(rename = "Version")]
        version: u8,
        #[serde(rename = "PaletteCount")]
        palette_count: u8,
        #[serde(rename = "TextureCount")]
        texture_count: u8,
        #[serde(rename = "ModelCount")]
        model_count: u8,
        #[serde(rename = "Palette")]
        palette: Option<DataId>,
        #[serde(rename = "Subpalettes")]
        subpalettes: Vec<Subpalette>,
        #[serde(rename = "TMChanges")]
        tm_changes: Vec<TextureMapChange>,
        #[serde(rename = "APChanges")]
        ap_changes: Vec<AnimPartChange>
}

// Contains data for a subpalette
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subpalette {
        #[serde(rename = "Palette")]
        palette: DataId,
        #[serde(rename = "Offset")]
        offset: u8,
        #[serde(rename = "NumColors")]
        num_colors: u8
}

// Contains data for texture map changes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextureMapChange {
        #[serde(rename = "PartIndex")]
        part_index: u8,
        #[serde(rename = "OldTexId")]
        old_tex_id: DataId,
        #[serde(rename = "NewTexId")]
        new_tex_id: DataId
}

// Contains data for animation part changes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnimPartChange {
        #[serde(rename = "PartIndex")]
        part_index: u8,
        #[serde(rename = "PartId")]
        part_id: DataId
}

// Data for a character creation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharGenResult {
        #[serde(rename = "Account")]
        account: String,
        #[serde(rename = "One")]
        one: u32,
        #[serde(rename = "HeritageGroup")]
        heritage_group: HeritageGroup,
        #[serde(rename = "Gender")]
        gender: Gender,
        #[serde(rename = "EyesStrip")]
        eyes_strip: u32,
        #[serde(rename = "NoseStrip")]
        nose_strip: u32,
        #[serde(rename = "MouthStrip")]
        mouth_strip: u32,
        #[serde(rename = "HairColor")]
        hair_color: u32,
        #[serde(rename = "EyeColor")]
        eye_color: u32,
        #[serde(rename = "HairStyle")]
        hair_style: u32,
        #[serde(rename = "HeadgearStyle")]
        headgear_style: u32,
        #[serde(rename = "HeadgearColor")]
        headgear_color: u32,
        #[serde(rename = "ShirtStyle")]
        shirt_style: u32,
        #[serde(rename = "ShirtColor")]
        shirt_color: u32,
        #[serde(rename = "TrousersStyle")]
        trousers_style: u32,
        #[serde(rename = "TrousersColor")]
        trousers_color: u32,
        #[serde(rename = "FootwearStyle")]
        footwear_style: u32,
        #[serde(rename = "FootwearColor")]
        footwear_color: u32,
        #[serde(rename = "SkinShade")]
        skin_shade: u64,
        #[serde(rename = "HairShade")]
        hair_shade: u64,
        #[serde(rename = "HeadgearShade")]
        headgear_shade: u64,
        #[serde(rename = "ShirtShade")]
        shirt_shade: u64,
        #[serde(rename = "TrousersShade")]
        trousers_shade: u64,
        #[serde(rename = "TootwearShade")]
        tootwear_shade: u64,
        #[serde(rename = "TemplateNum")]
        template_num: u32,
        #[serde(rename = "Strength")]
        strength: u32,
        #[serde(rename = "Endurance")]
        endurance: u32,
        #[serde(rename = "Coordination")]
        coordination: u32,
        #[serde(rename = "Quickness")]
        quickness: u32,
        #[serde(rename = "Focus")]
        focus: u32,
        #[serde(rename = "Self")]
        self_: u32,
        #[serde(rename = "Slot")]
        slot: u32,
        #[serde(rename = "ClassId")]
        class_id: u32,
        #[serde(rename = "Skills")]
        skills: PackableList<SkillAdvancementClass>,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "StartArea")]
        start_area: u32,
        #[serde(rename = "IsAdmin")]
        is_admin: u32,
        #[serde(rename = "IsEnvoy")]
        is_envoy: u32,
        #[serde(rename = "Validation")]
        validation: u32
}

// Basic information for a character used at the Login screen
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterIdentity {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "SecondsGreyedOut")]
        seconds_greyed_out: u32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquipLocation {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Slot")]
        slot: EquipMask
}

// The PhysicsDesc structure defines an object's physical behavior.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicsDesc {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "State")]
        state: PhysicsState,
        #[serde(rename = "MovementBuffer")]
        movement_buffer: Option<PackableList<byte>>,
        #[serde(rename = "Autonomous")]
        autonomous: Option<bool>,
        #[serde(rename = "AnimationFrame")]
        animation_frame: Option<u32>,
        #[serde(rename = "Position")]
        position: Option<Position>,
        #[serde(rename = "MotionId")]
        motion_id: Option<DataId>,
        #[serde(rename = "SoundId")]
        sound_id: Option<DataId>,
        #[serde(rename = "PhysicsScriptId")]
        physics_script_id: Option<DataId>,
        #[serde(rename = "SetupId")]
        setup_id: Option<DataId>,
        #[serde(rename = "ParentId")]
        parent_id: Option<ObjectId>,
        #[serde(rename = "ParentLocation")]
        parent_location: Option<ParentLocation>,
        #[serde(rename = "Children")]
        children: Option<PackableList<EquipLocation>>,
        #[serde(rename = "Scale")]
        scale: Option<f32>,
        #[serde(rename = "Friction")]
        friction: Option<f32>,
        #[serde(rename = "Elasticity")]
        elasticity: Option<f32>,
        #[serde(rename = "Translucency")]
        translucency: Option<f32>,
        #[serde(rename = "Velocity")]
        velocity: Option<Vector3>,
        #[serde(rename = "Acceleration")]
        acceleration: Option<Vector3>,
        #[serde(rename = "Omega")]
        omega: Option<Vector3>,
        #[serde(rename = "DefaultScript")]
        default_script: Option<u32>,
        #[serde(rename = "DefaultScriptIntensity")]
        default_script_intensity: Option<f32>,
        #[serde(rename = "ObjectPositionSequence")]
        object_position_sequence: u16,
        #[serde(rename = "ObjectMovementSequence")]
        object_movement_sequence: u16,
        #[serde(rename = "ObjectStateSequence")]
        object_state_sequence: u16,
        #[serde(rename = "ObjectVectorSequence")]
        object_vector_sequence: u16,
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "ObjectForcePositionSequence")]
        object_force_position_sequence: u16,
        #[serde(rename = "ObjectVisualDescSequence")]
        object_visual_desc_sequence: u16,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminAccountData {
        #[serde(rename = "AccountName")]
        account_name: String,
        #[serde(rename = "BookieId")]
        bookie_id: u32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminPlayerData {
        name: String,
        #[serde(rename = "bookieId")]
        bookie_id: u32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VendorProfile {
        #[serde(rename = "Categories")]
        categories: ItemType,
        #[serde(rename = "MinValue")]
        min_value: u32,
        #[serde(rename = "MaxValue")]
        max_value: u32,
        #[serde(rename = "DealsMagic")]
        deals_magic: bool,
        #[serde(rename = "BuyPrice")]
        buy_price: f32,
        #[serde(rename = "SellPrice")]
        sell_price: f32,
        #[serde(rename = "CurrencyId")]
        currency_id: u32,
        #[serde(rename = "CurrencyAmount")]
        currency_amount: u32,
        #[serde(rename = "CurrencyName")]
        currency_name: String
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmorProfile {
        #[serde(rename = "ProtSlashing")]
        prot_slashing: f32,
        #[serde(rename = "ProtPiercing")]
        prot_piercing: f32,
        #[serde(rename = "ProtBludgeoning")]
        prot_bludgeoning: f32,
        #[serde(rename = "ProtCold")]
        prot_cold: f32,
        #[serde(rename = "ProtFire")]
        prot_fire: f32,
        #[serde(rename = "ProtAcid")]
        prot_acid: f32,
        #[serde(rename = "ProtNether")]
        prot_nether: f32,
        #[serde(rename = "ProtLightning")]
        prot_lightning: f32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatureAppraisalProfile {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Health")]
        health: u32,
        #[serde(rename = "HealthMax")]
        health_max: u32,
        #[serde(rename = "Strength")]
        strength: Option<u32>,
        #[serde(rename = "Endurance")]
        endurance: Option<u32>,
        #[serde(rename = "Quickness")]
        quickness: Option<u32>,
        #[serde(rename = "Coordination")]
        coordination: Option<u32>,
        #[serde(rename = "Focus")]
        focus: Option<u32>,
        #[serde(rename = "Self")]
        self_: Option<u32>,
        #[serde(rename = "Stamina")]
        stamina: Option<u32>,
        #[serde(rename = "Mana")]
        mana: Option<u32>,
        #[serde(rename = "StaminaMax")]
        stamina_max: Option<u32>,
        #[serde(rename = "ManaMax")]
        mana_max: Option<u32>,
        #[serde(rename = "AttrHighlight")]
        attr_highlight: Option<AttributeMask>,
        #[serde(rename = "AttrColor")]
        attr_color: Option<AttributeMask>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeaponProfile {
        #[serde(rename = "DamageType")]
        damage_type: DamageType,
        #[serde(rename = "Speed")]
        speed: u32,
        #[serde(rename = "Skill")]
        skill: SkillId,
        #[serde(rename = "Damage")]
        damage: u32,
        #[serde(rename = "Variance")]
        variance: f64,
        #[serde(rename = "Modifier")]
        modifier: f64,
        #[serde(rename = "Length")]
        length: f64,
        #[serde(rename = "MaxVelocity")]
        max_velocity: f64,
        #[serde(rename = "Offsense")]
        offsense: f64,
        #[serde(rename = "MaxVelocityEstimated")]
        max_velocity_estimated: u32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HookAppraisalProfile {
        #[serde(rename = "Flags")]
        flags: HookAppraisalFlags,
        #[serde(rename = "ValidLocations")]
        valid_locations: EquipMask,
        #[serde(rename = "AmmoType")]
        ammo_type: AmmoType
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SquelchDB {
        #[serde(rename = "AccountHash")]
        account_hash: PackableHashTable<string, uint>,
        #[serde(rename = "CharacterHash")]
        character_hash: PackableHashTable<ObjectId, SquelchInfo>,
        #[serde(rename = "GlobalInfo")]
        global_info: SquelchInfo
}

// Set of information related to a squelch entry
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SquelchInfo {
        #[serde(rename = "Filters")]
        filters: PackableList<LogTextType>,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "Account")]
        account: bool
}

// Set of information related to purchasing a housing
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HouseProfile {
        #[serde(rename = "DwellingId")]
        dwelling_id: u32,
        #[serde(rename = "OwnerId")]
        owner_id: ObjectId,
        #[serde(rename = "Flags")]
        flags: HouseBitfield,
        #[serde(rename = "MinLevel")]
        min_level: i32,
        #[serde(rename = "MaxLevel")]
        max_level: i32,
        #[serde(rename = "MinAllegRank")]
        min_alleg_rank: i32,
        #[serde(rename = "MaxAllegRank")]
        max_alleg_rank: i32,
        #[serde(rename = "MaintenanceFree")]
        maintenance_free: bool,
        #[serde(rename = "Type")]
        type_: HouseType,
        #[serde(rename = "OwnerName")]
        owner_name: String,
        #[serde(rename = "Buy")]
        buy: PackableList<HousePayment>,
        #[serde(rename = "Rent")]
        rent: PackableList<HousePayment>
}

// The HousePayment structure contains information about a house purchase or maintenance item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HousePayment {
        #[serde(rename = "Required")]
        required: u32,
        #[serde(rename = "Paid")]
        paid: u32,
        #[serde(rename = "WeenieClassId")]
        weenie_class_id: u32,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "PluralName")]
        plural_name: String
}

// Set of information related to owning a housing
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HouseData {
        #[serde(rename = "BuyTime")]
        buy_time: u32,
        #[serde(rename = "RentTime")]
        rent_time: u32,
        #[serde(rename = "Type")]
        type_: HouseType,
        #[serde(rename = "MaintenanceFree")]
        maintenance_free: bool,
        #[serde(rename = "Buy")]
        buy: PackableList<HousePayment>,
        #[serde(rename = "Rent")]
        rent: PackableList<HousePayment>,
        #[serde(rename = "Position")]
        position: Position
}

// Set of information related to house access
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HAR {
        #[serde(rename = "Version")]
        version: u32,
        #[serde(rename = "Bitmask")]
        bitmask: u32,
        #[serde(rename = "MonarchId")]
        monarch_id: ObjectId,
        #[serde(rename = "GuestList")]
        guest_list: PackableHashTable<ObjectId, GuestInfo>,
        #[serde(rename = "RoommateList")]
        roommate_list: PackableList<ObjectId>
}

// Set of information related to a house guest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestInfo {
        #[serde(rename = "HasStoragePermission")]
        has_storage_permission: bool,
        #[serde(rename = "GuestName")]
        guest_name: String
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
        skill_used: SkillId,
        #[serde(rename = "NotSalvagable")]
        not_salvagable: PackableList<ObjectId>,
        #[serde(rename = "SalvageResults")]
        salvage_results: PackableList<SalvageResult>,
        #[serde(rename = "AugBonus")]
        aug_bonus: i32
}

// Set of information related to a salvage of an item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SalvageResult {
        #[serde(rename = "Material")]
        material: MaterialType,
        #[serde(rename = "Workmanship")]
        workmanship: f64,
        #[serde(rename = "Units")]
        units: u32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FellowshipLockData {
        #[serde(rename = "Unknown1")]
        unknown1: u32,
        #[serde(rename = "Unknown2")]
        unknown2: u32,
        #[serde(rename = "Unknown3")]
        unknown3: u32,
        #[serde(rename = "Timestamp")]
        timestamp: u32,
        #[serde(rename = "Sequence")]
        sequence: u32
}

// Set of information for a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship {
        #[serde(rename = "Members")]
        members: PackableHashTable<ObjectId, Fellow>,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "LeaderId")]
        leader_id: ObjectId,
        #[serde(rename = "ShareXP")]
        share_xp: bool,
        #[serde(rename = "EvenXPSplit")]
        even_xp_split: bool,
        #[serde(rename = "Open")]
        open: bool,
        #[serde(rename = "Locked")]
        locked: bool,
        #[serde(rename = "RecentlyDeparted")]
        recently_departed: PackableHashTable<ObjectId, int>,
        #[serde(rename = "Locks")]
        locks: PackableHashTable<String, FellowshipLockData>
}

// The FellowInfo structure contains information about a fellowship member.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellow {
        #[serde(rename = "XPCached")]
        xp_cached: u32,
        #[serde(rename = "LumCached")]
        lum_cached: u32,
        #[serde(rename = "Level")]
        level: u32,
        #[serde(rename = "MaxHealth")]
        max_health: u32,
        #[serde(rename = "MaxStamina")]
        max_stamina: u32,
        #[serde(rename = "MaxMana")]
        max_mana: u32,
        #[serde(rename = "CurrentHealth")]
        current_health: u32,
        #[serde(rename = "CurrentStamina")]
        current_stamina: u32,
        #[serde(rename = "CurrentMana")]
        current_mana: u32,
        #[serde(rename = "ShareLoot")]
        share_loot: bool,
        #[serde(rename = "Name")]
        name: String
}

// Contains information about a contract.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractTracker {
        #[serde(rename = "Version")]
        version: u32,
        #[serde(rename = "ContractId")]
        contract_id: ContractId,
        #[serde(rename = "ContractStage")]
        contract_stage: ContractStage,
        #[serde(rename = "TimeWhenDone")]
        time_when_done: i64,
        #[serde(rename = "TimeWhenRepeats")]
        time_when_repeats: i64
}

// Contains table of ContractTrackers
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractTrackerTable {
        #[serde(rename = "ContactTrackers")]
        contact_trackers: PackableHashTable<uint, ContractTracker>
}

// Client to Server AC packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct C2SPacket {
        #[serde(rename = "Sequence")]
        sequence: u32,
        #[serde(rename = "Flags")]
        flags: PacketHeaderFlags,
        #[serde(rename = "Checksum")]
        checksum: u32,
        #[serde(rename = "RecipientId")]
        recipient_id: u16,
        #[serde(rename = "TimeSinceLastPacket")]
        time_since_last_packet: u16,
        #[serde(rename = "Size")]
        size: u16,
        #[serde(rename = "Iteration")]
        iteration: u16,
        #[serde(rename = "ServerSwitch")]
        server_switch: Option<ServerSwitchHeader>,
        #[serde(rename = "RetransmitSequences")]
        retransmit_sequences: Option<PackableList<uint>>,
        #[serde(rename = "RejectSequences")]
        reject_sequences: Option<PackableList<uint>>,
        #[serde(rename = "AckSequence")]
        ack_sequence: Option<u32>,
        #[serde(rename = "LoginRequest")]
        login_request: Option<LoginRequestHeader>,
        #[serde(rename = "WorldLoginRequest")]
        world_login_request: Option<u64>,
        #[serde(rename = "ConnectResponse")]
        connect_response: Option<u64>,
        #[serde(rename = "CICMDCommand")]
        cicmd_command: Option<CICMDCommandHeader>,
        #[serde(rename = "Time")]
        time: Option<u64>,
        #[serde(rename = "EchoTime")]
        echo_time: Option<f32>,
        #[serde(rename = "Flow")]
        flow: Option<FlowHeader>,
        #[serde(rename = "Fragments")]
        fragments: Option<BlobFragments>
}

// Server to Client AC packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct S2CPacket {
        #[serde(rename = "Sequence")]
        sequence: u32,
        #[serde(rename = "Flags")]
        flags: PacketHeaderFlags,
        #[serde(rename = "Checksum")]
        checksum: u32,
        #[serde(rename = "RecipientId")]
        recipient_id: u16,
        #[serde(rename = "TimeSinceLastPacket")]
        time_since_last_packet: u16,
        #[serde(rename = "Size")]
        size: u16,
        #[serde(rename = "Iteration")]
        iteration: u16,
        #[serde(rename = "AckSequence")]
        ack_sequence: Option<u32>,
        #[serde(rename = "LogonServerAddr")]
        logon_server_addr: Option<SocketAddress>,
        #[serde(rename = "Referral")]
        referral: Option<ReferralHeader>,
        #[serde(rename = "ConnectRequest")]
        connect_request: Option<ConnectRequestHeader>,
        #[serde(rename = "NetError")]
        net_error: Option<NetError>,
        #[serde(rename = "NetErrorDisconnect")]
        net_error_disconnect: Option<NetError>,
        #[serde(rename = "EchoResponse")]
        echo_response: Option<EchoResponseHeader>,
        #[serde(rename = "Fragments")]
        fragments: Option<BlobFragments>
}

