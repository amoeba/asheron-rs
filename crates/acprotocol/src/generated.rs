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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackedWORD {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LayeredSpellId {
        #[serde(rename = "Id")]
        id: SpellId,
        #[serde(rename = "Layer")]
        layer: u16
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
        #[serde(rename = "X")]
        x: f32,
        #[serde(rename = "Y")]
        y: f32,
        #[serde(rename = "Z")]
        z: f32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Origin {
        #[serde(rename = "Landcell")]
        landcell: LandcellId,
        #[serde(rename = "Location")]
        location: Vector3
}

// Landcell location, including orientation
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Position {
        #[serde(rename = "Landcell")]
        landcell: LandcellId,
        #[serde(rename = "Frame")]
        frame: Frame
}

// A the location and orientation of an object within a landcell
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
        #[serde(rename = "Origin")]
        origin: Vector3,
        #[serde(rename = "Orientation")]
        orientation: Quaternion
}

// Optional header data when PacketHeaderFlags includes ServerSwitch
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServerSwitchHeader {
        #[serde(rename = "Sequence")]
        sequence: u32,
        #[serde(rename = "Type")]
        type_: ServerSwitchType
}

// Optional header data when PacketHeaderFlags includes CICMDCommand
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CICMDCommandHeader {
        #[serde(rename = "Command")]
        command: u32,
        #[serde(rename = "Parameter")]
        parameter: u32
}

// Optional header data when PacketHeaderFlags includes Flow
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FlowHeader {
        #[serde(rename = "Bytes")]
        bytes: u32,
        #[serde(rename = "Interval")]
        interval: u16
}

// Optional header data when PacketHeaderFlags includes LogonServerAddr
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "AuthType")]
pub enum LoginRequestHeader {
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
}

// Optional header data when PacketHeaderFlags includes Referral
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NetError {
        #[serde(rename = "StringId")]
        string_id: DataId,
        #[serde(rename = "TableId")]
        table_id: DataId
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EchoResponseHeader {
        #[serde(rename = "LocalTime")]
        local_time: f32,
        #[serde(rename = "HoldingTime")]
        holding_time: f32
}

// A collection of property tables.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ACBaseQualities {
        #[serde(rename = "Flags")]
        flags: ACBaseQualitiesFlags,
        #[serde(rename = "WeenieType")]
        weenie_type: WeenieType,
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
        #[serde(rename = "DataProperties")]
        data_properties: PackableHashTable,
        #[serde(rename = "InstanceProperties")]
        instance_properties: PackableHashTable,
        #[serde(rename = "PositionProperties")]
        position_properties: PackableHashTable
}

// The ACQualities structure contains character property lists.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ACQualities {
        #[serde(rename = "Flags")]
        flags: ACQualitiesFlags,
        #[serde(rename = "HasHealth")]
        has_health: bool,
        #[serde(rename = "Attributes")]
        attributes: AttributeCache,
        #[serde(rename = "Skills")]
        skills: PackableHashTable,
        #[serde(rename = "Body")]
        body: Body,
        #[serde(rename = "SpellBook")]
        spell_book: PackableHashTable,
        #[serde(rename = "Enchantments")]
        enchantments: EnchantmentRegistry,
        #[serde(rename = "EventFilter")]
        event_filter: EventFilter,
        #[serde(rename = "Emotes")]
        emotes: EmoteTable,
        #[serde(rename = "CreationProfile")]
        creation_profile: PackableList,
        #[serde(rename = "PageData")]
        page_data: PageDataList,
        #[serde(rename = "Generators")]
        generators: GeneratorTable,
        #[serde(rename = "GeneratorRegistry")]
        generator_registry: GeneratorRegistry,
        #[serde(rename = "GeneratorQueue")]
        generator_queue: GeneratorQueue
}

// The AttributeCache structure contains information about a character attributes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AttributeCache {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Strength")]
        strength: AttributeInfo,
        #[serde(rename = "Endurance")]
        endurance: AttributeInfo,
        #[serde(rename = "Quickness")]
        quickness: AttributeInfo,
        #[serde(rename = "Coordination")]
        coordination: AttributeInfo,
        #[serde(rename = "Focus")]
        focus: AttributeInfo,
        #[serde(rename = "Self")]
        self_: AttributeInfo,
        #[serde(rename = "Health")]
        health: SecondaryAttributeInfo,
        #[serde(rename = "Stamina")]
        stamina: SecondaryAttributeInfo,
        #[serde(rename = "Mana")]
        mana: SecondaryAttributeInfo
}

// The Attribute structure contains information about a character attribute.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AttributeInfo {
        #[serde(rename = "PointsRaised")]
        points_raised: u32,
        #[serde(rename = "InnatePoints")]
        innate_points: u32,
        #[serde(rename = "ExperienceSpent")]
        experience_spent: u32
}

// The SecondaryAttribute structure contains information about a character vital.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SecondaryAttributeInfo {
        #[serde(rename = "Attribute")]
        attribute: AttributeInfo,
        #[serde(rename = "Current")]
        current: u32
}

// The Skill structure contains information about a character skill.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Body {
        #[serde(rename = "BodyParts")]
        body_parts: PackableHashTable
}

// Information on individual body parts. (Needs to be confirmed if this was used in prod)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        bpsd: BodyPartSelectionData
}

// Information on armor levels
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SpellBookPage {
        #[serde(rename = "CastingLikelihood")]
        casting_likelihood: f32,
        #[serde(rename = "Unknown")]
        unknown: i32,
        #[serde(rename = "CastingLikelihood2")]
        casting_likelihood2: f32
}

// Contains information related to the spells in effect on the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EnchantmentRegistry {
        #[serde(rename = "Flags")]
        flags: EnchantmentRegistryFlags,
        #[serde(rename = "LifeSpells")]
        life_spells: PackableList,
        #[serde(rename = "CreatureSpells")]
        creature_spells: PackableList,
        #[serde(rename = "Vitae")]
        vitae: Enchantment,
        #[serde(rename = "Cooldowns")]
        cooldowns: PackableList
}

// The Enchantment structure describes an active enchantment.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        equipment_set: EquipmentSet
}

// Information on stat modification
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StatMod {
        #[serde(rename = "Type")]
        type_: EnchantmentTypeFlags,
        #[serde(rename = "Key")]
        key: u32,
        #[serde(rename = "Value")]
        value: f32
}

// Contains a list of events to filter? Unknown what this does currently.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventFilter {
        #[serde(rename = "Events")]
        events: PackableList
}

// Contains a list of emotes for NPCs? Unknown what this does currently.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EmoteTable {
        #[serde(rename = "Emotes")]
        emotes: PackableHashTable
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EmoteSetList {
        #[serde(rename = "Emotes")]
        emotes: PackableList
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Category")]
pub enum EmoteSet {
    #[serde(rename = "0x01 | 0x06")]
    Type01 | 0x06 {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "Emotes")]
        emotes: PackableList,
        #[serde(rename = "ClassId")]
        class_id: u32,
    },
    #[serde(rename = "0x02")]
    Type02 {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "Emotes")]
        emotes: PackableList,
        #[serde(rename = "VendorType")]
        vendor_type: u32,
    },
    #[serde(rename = "0x05")]
    Type05 {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "Emotes")]
        emotes: PackableList,
        #[serde(rename = "Style")]
        style: u32,
        #[serde(rename = "Substyle")]
        substyle: u32,
    },
    #[serde(rename = "0x0F")]
    Type0F {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "Emotes")]
        emotes: PackableList,
        #[serde(rename = "MinHealth")]
        min_health: f32,
        #[serde(rename = "MaxHealth")]
        max_health: f32,
    },
    #[serde(rename = "0x0C | 0x0D | 0x16 | 0x17 | 0x1B | 0x1C | 0x1D | 0x1E | 0x1F | 0x20 | 0x21 | 0x22 | 0x23 | 0x24 | 0x25 | 0x26")]
    Type0C | 0x0D | 0x16 | 0x17 | 0x1B | 0x1C | 0x1D | 0x1E | 0x1F | 0x20 | 0x21 | 0x22 | 0x23 | 0x24 | 0x25 | 0x26 {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "Emotes")]
        emotes: PackableList,
        #[serde(rename = "Quest")]
        quest: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum Emote {
    #[serde(rename = "0x35 | 0x36 | 0x37 | 0x45")]
    Type35 | 0x36 | 0x37 | 0x45 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Stat")]
        stat: u32,
        #[serde(rename = "Amount")]
        amount: u32,
    },
    #[serde(rename = "0x2 | 0x3E")]
    Type2 | 0x3E {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Amount64")]
        amount64: u64,
        #[serde(rename = "HeroXP64")]
        hero_xp64: u64,
    },
    #[serde(rename = "0x70 | 0x71")]
    Type70 | 0x71 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Amount64")]
        amount64: u64,
    },
    #[serde(rename = "0x22 | 0x2F | 0x30 | 0x5A | 0x77 | 0x78")]
    Type22 | 0x2F | 0x30 | 0x5A | 0x77 | 0x78 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Amount")]
        amount: u32,
    },
    #[serde(rename = "0x01 | 0x08 | 0x0A | 0x0D | 0x10 | 0x11 | 0x12 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1A | 0x1F | 0x33 | 0x3A | 0x3C | 0x3D | 0x40 | 0x41 | 0x43 | 0x44 | 0x4F | 0x50 | 0x51 | 0x53 | 0x58 | 0x79")]
    Type01 | 0x08 | 0x0A | 0x0D | 0x10 | 0x11 | 0x12 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1A | 0x1F | 0x33 | 0x3A | 0x3C | 0x3D | 0x40 | 0x41 | 0x43 | 0x44 | 0x4F | 0x50 | 0x51 | 0x53 | 0x58 | 0x79 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Message")]
        message: String,
    },
    #[serde(rename = "0x3 | 0x4A")]
    Type3 | 0x4A {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "CProfile")]
        c_profile: CreationProfile,
    },
    #[serde(rename = "0x73")]
    Type73 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Stat")]
        stat: u32,
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
    #[serde(rename = "0x7")]
    Type7 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "PhysicsScript")]
        physics_script: u32,
    },
    #[serde(rename = "0x1C | 0x1D")]
    Type1C | 0x1D {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Amount")]
        amount: u32,
        #[serde(rename = "Stat")]
        stat: u32,
    },
    #[serde(rename = "0x6F")]
    Type6F {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Amount")]
        amount: u32,
    },
    #[serde(rename = "0x23 | 0x2D | 0x2E")]
    Type23 | 0x2D | 0x2E {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "Stat")]
        stat: u32,
    },
    #[serde(rename = "0xE | 0x13 | 0x1B | 0x49")]
    TypeE | 0x13 | 0x1B | 0x49 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "SpellId")]
        spell_id: u32,
    },
    #[serde(rename = "0x26 | 0x4B")]
    Type26 | 0x4B {
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
    #[serde(rename = "0x9")]
    Type9 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Sound")]
        sound: u32,
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
    #[serde(rename = "0x3F | 0x63 | 0x64")]
    Type3F | 0x63 | 0x64 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Position")]
        position: Position,
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
    #[serde(rename = "0x1E | 0x3B | 0x47 | 0x52")]
    Type1E | 0x3B | 0x47 | 0x52 {
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
    #[serde(rename = "0x5 | 0x34")]
    Type5 | 0x34 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Motion")]
        motion: u32,
    },
    #[serde(rename = "0x6E")]
    Type6E {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Stat")]
        stat: u32,
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
    #[serde(rename = "0x20 | 0x21 | 0x46 | 0x54 | 0x55 | 0x56 | 0x59 | 0x66 | 0x67 | 0x68 | 0x69 | 0x6A | 0x6B | 0x6C | 0x6D")]
    Type20 | 0x21 | 0x46 | 0x54 | 0x55 | 0x56 | 0x59 | 0x66 | 0x67 | 0x68 | 0x69 | 0x6A | 0x6B | 0x6C | 0x6D {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "Amount")]
        amount: u32,
    },
    #[serde(rename = "0x24 | 0x27 | 0x28 | 0x29 | 0x2A | 0x2B | 0x2C")]
    Type24 | 0x27 | 0x28 | 0x29 | 0x2A | 0x2B | 0x2C {
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
    #[serde(rename = "0x4 | 0x6 | 0xB | 0x57")]
    Type4 | 0x6 | 0xB | 0x57 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Frame")]
        frame: Frame,
    },
}

// Set information about an item for creation
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PageDataList {
        #[serde(rename = "MaxNumPages")]
        max_num_pages: u32,
        #[serde(rename = "MaxNumCharsPerPage")]
        max_num_chars_per_page: u32,
        #[serde(rename = "Pages")]
        pages: PackableList
}

// Data for an individual page
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        page_text: String
}

// Blob fragment data used to contruct message data. These can be spread across multiple packets
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        group: FragmentGroup
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratorTable {
        #[serde(rename = "Generators")]
        generators: PackableList
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratorRegistry {
        #[serde(rename = "Registry")]
        registry: PackableHashTable
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratorQueue {
        #[serde(rename = "Queue")]
        queue: PackableList
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratorQueueNode {
        #[serde(rename = "Slot")]
        slot: u32,
        #[serde(rename = "When")]
        when: f64
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type_a")]
pub enum WindowOption {
    #[serde(rename = "0x1000008b")]
    Type1000008b {
        #[serde(rename = "Unknown_b")]
        unknown_b: u8,
        #[serde(rename = "PropertyCount")]
        property_count: u8,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum OptionProperty {
    #[serde(rename = "0x1000008c")]
    Type1000008c {
        #[serde(rename = "Unknown_a")]
        unknown_a: u32,
        #[serde(rename = "WindowOptions")]
        window_options: PackableList,
    },
    #[serde(rename = "0x10000081")]
    Type10000081 {
        unknown_k: u32,
        #[serde(rename = "activeOpacity")]
        active_opacity: f32,
    },
    #[serde(rename = "0x10000080")]
    Type10000080 {
        unknown_l: u32,
        #[serde(rename = "inactiveOpacity")]
        inactive_opacity: f32,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GameplayOptions {
        #[serde(rename = "Size")]
        size: u32,
        #[serde(rename = "Unknown200_2")]
        unknown200_2: u8,
        #[serde(rename = "OptionPropertyCount")]
        option_property_count: u8
}

// The PlayerModule structure contains character options.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlayerModule {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Options")]
        options: CharacterOptions1,
        #[serde(rename = "Shortcuts")]
        shortcuts: PackableList,
        #[serde(rename = "Tab1Spells")]
        tab1_spells: PackableList,
        #[serde(rename = "Tab2Spells")]
        tab2_spells: PackableList,
        #[serde(rename = "Tab3Spells")]
        tab3_spells: PackableList,
        #[serde(rename = "Tab4Spells")]
        tab4_spells: PackableList,
        #[serde(rename = "Tab5Spells")]
        tab5_spells: PackableList,
        #[serde(rename = "Tab6Spells")]
        tab6_spells: PackableList,
        #[serde(rename = "Tab7Spells")]
        tab7_spells: PackableList,
        #[serde(rename = "Tab8Spells")]
        tab8_spells: PackableList,
        #[serde(rename = "FillComps")]
        fill_comps: PackableHashTable,
        #[serde(rename = "SpellBookFilters")]
        spell_book_filters: u32,
        #[serde(rename = "OptionFlags")]
        option_flags: u32,
        #[serde(rename = "Unknown100_1")]
        unknown100_1: u32,
        #[serde(rename = "OptionStrings")]
        option_strings: PackableHashTable,
        #[serde(rename = "GameplayOptions")]
        gameplay_options: GameplayOptions
}

// Set of shortcuts
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShortCutManager {
        #[serde(rename = "Shortcuts")]
        shortcuts: PackableList
}

// Shortcut
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShortCutData {
        #[serde(rename = "Index")]
        index: u32,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// List of spells in spell tab
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SpellTab {
        #[serde(rename = "Spells")]
        spells: PackableList
}

// Set of inventory items
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContentProfile {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ContainerType")]
        container_type: ContainerProperties
}

// Set of inventory items
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InventoryPlacement {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Location")]
        location: EquipMask,
        #[serde(rename = "Priority")]
        priority: CoverageMask
}

// Allegience information
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AllegianceProfile {
        #[serde(rename = "TotalMembers")]
        total_members: u32,
        #[serde(rename = "TotalVassals")]
        total_vassals: u32,
        #[serde(rename = "Hierarchy")]
        hierarchy: AllegianceHierarchy
}

// Allegience record
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AllegianceRecord {
        #[serde(rename = "TreeParent")]
        tree_parent: ObjectId,
        #[serde(rename = "AllegianceData")]
        allegiance_data: AllegianceData
}

// Allegience hierarchy information
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AllegianceHierarchy {
        #[serde(rename = "RecordCount")]
        record_count: u16,
        #[serde(rename = "OldVersion")]
        old_version: u16,
        #[serde(rename = "Officers")]
        officers: PHashTable,
        #[serde(rename = "OfficerTitles")]
        officer_titles: PackableList,
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
        monarch_data: AllegianceData
}

// Set of allegiance data for a specific player
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        level: u32,
        #[serde(rename = "Loyalty")]
        loyalty: u16,
        #[serde(rename = "Leadership")]
        leadership: u16,
        #[serde(rename = "TimeOnline")]
        time_online: u64,
        #[serde(rename = "TimeOnline")]
        time_online: u32,
        #[serde(rename = "AllegianceAge")]
        allegiance_age: u32,
        #[serde(rename = "Name")]
        name: String
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        out_friends: PackableList,
        #[serde(rename = "InFriends")]
        in_friends: PackableList
}

// Data related to an item, namely the amount and description
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "PwdType")]
pub enum ItemProfile {
    #[serde(rename = "-1")]
    Type-1 {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        header2: u32,
        #[serde(rename = "PluralName")]
        plural_name: String,
        #[serde(rename = "ItemsCapacity")]
        items_capacity: u8,
        #[serde(rename = "ContainerCapacity")]
        container_capacity: u8,
        #[serde(rename = "AmmunitionType")]
        ammunition_type: AmmoType,
        #[serde(rename = "Value")]
        value: u32,
        #[serde(rename = "Useability")]
        useability: Usable,
        #[serde(rename = "UseRadius")]
        use_radius: f32,
        #[serde(rename = "TargetType")]
        target_type: ItemType,
        #[serde(rename = "Effects")]
        effects: IconHighlight,
        #[serde(rename = "CombatUse")]
        combat_use: WieldType,
        #[serde(rename = "Structure")]
        structure: u16,
        #[serde(rename = "MaxStructure")]
        max_structure: u16,
        #[serde(rename = "StackSize")]
        stack_size: u16,
        #[serde(rename = "MaxStackSize")]
        max_stack_size: u16,
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "WielderId")]
        wielder_id: ObjectId,
        #[serde(rename = "ValidSlots")]
        valid_slots: EquipMask,
        #[serde(rename = "Slot")]
        slot: EquipMask,
        #[serde(rename = "Priority")]
        priority: CoverageMask,
        #[serde(rename = "BlipColor")]
        blip_color: RadarColor,
        #[serde(rename = "RadarEnum")]
        radar_enum: RadarBehavior,
        #[serde(rename = "PhysicsScript")]
        physics_script: u16,
        #[serde(rename = "Workmanship")]
        workmanship: f32,
        #[serde(rename = "Burden")]
        burden: u16,
        #[serde(rename = "SpellId")]
        spell_id: SpellId,
        #[serde(rename = "OwnerId")]
        owner_id: ObjectId,
        #[serde(rename = "Restrictions")]
        restrictions: RestrictionDB,
        #[serde(rename = "HookItemTypes")]
        hook_item_types: HookType,
        #[serde(rename = "MonarchId")]
        monarch_id: ObjectId,
        #[serde(rename = "HookType")]
        hook_type: HookType,
        #[serde(rename = "IconOverlay")]
        icon_overlay: PackedDWORD,
        #[serde(rename = "IconUnderlay")]
        icon_underlay: PackedDWORD,
        #[serde(rename = "Material")]
        material: MaterialType,
        #[serde(rename = "CooldownId")]
        cooldown_id: u32,
        #[serde(rename = "CooldownDuration")]
        cooldown_duration: u64,
        #[serde(rename = "PetOwnerId")]
        pet_owner_id: ObjectId
}

// The RestrictionDB contains the access control list for a dwelling object.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RestrictionDB {
        #[serde(rename = "Version")]
        version: u32,
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "MonarchId")]
        monarch_id: ObjectId,
        #[serde(rename = "Permissions")]
        permissions: PHashTable
}

// The OldPublicWeenieDesc structure defines an object's game behavior.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        plural_name: String,
        #[serde(rename = "ItemsCapacity")]
        items_capacity: u8,
        #[serde(rename = "ContainerCapacity")]
        container_capacity: u8,
        #[serde(rename = "Value")]
        value: u32,
        #[serde(rename = "Useability")]
        useability: Usable,
        #[serde(rename = "UseRadius")]
        use_radius: f32,
        #[serde(rename = "tTargetType")]
        t_target_type: ItemType,
        #[serde(rename = "Effects")]
        effects: IconHighlight,
        #[serde(rename = "AmmunitionType")]
        ammunition_type: AmmoType,
        #[serde(rename = "CombatUse")]
        combat_use: WieldType,
        #[serde(rename = "Structure")]
        structure: u16,
        #[serde(rename = "MaxStructure")]
        max_structure: u16,
        #[serde(rename = "StackSize")]
        stack_size: u16,
        #[serde(rename = "MaxStackSize")]
        max_stack_size: u16,
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "WielderId")]
        wielder_id: ObjectId,
        #[serde(rename = "ValidSlots")]
        valid_slots: EquipMask,
        #[serde(rename = "Slots")]
        slots: EquipMask,
        #[serde(rename = "Priority")]
        priority: CoverageMask,
        #[serde(rename = "BlipColor")]
        blip_color: RadarColor,
        #[serde(rename = "RadarEnum")]
        radar_enum: RadarBehavior,
        #[serde(rename = "ObviousDistance")]
        obvious_distance: f32,
        #[serde(rename = "Vndwcid")]
        vndwcid: u16,
        #[serde(rename = "SpellId")]
        spell_id: SpellId,
        #[serde(rename = "HouseOwnerId")]
        house_owner_id: ObjectId,
        #[serde(rename = "PhysicsScript")]
        physics_script: u16,
        #[serde(rename = "Restrictions")]
        restrictions: RestrictionDB,
        #[serde(rename = "HookType")]
        hook_type: HookType,
        #[serde(rename = "HookItemTypes")]
        hook_item_types: HookType,
        #[serde(rename = "MonarchId")]
        monarch_id: ObjectId,
        #[serde(rename = "IconOverlay")]
        icon_overlay: PackedDWORD,
        #[serde(rename = "Material")]
        material: MaterialType
}

// Information related to a secure trade.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackedMotionCommand {
        #[serde(rename = "CommandId")]
        command_id: Command,
        #[serde(rename = "PackedSequence")]
        packed_sequence: u16,
        #[serde(rename = "Speed")]
        speed: f32
}

// Data related to the movement of the object sent from a client
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RawMotionState {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "CurrentHoldkey")]
        current_holdkey: HoldKey,
        #[serde(rename = "CurrentStyle")]
        current_style: StanceMode,
        #[serde(rename = "ForwardCommand")]
        forward_command: MovementCommand,
        #[serde(rename = "ForwardHoldkey")]
        forward_holdkey: HoldKey,
        #[serde(rename = "ForwardSpeed")]
        forward_speed: f32,
        #[serde(rename = "SidestepCommand")]
        sidestep_command: MovementCommand,
        #[serde(rename = "SidestepHoldkey")]
        sidestep_holdkey: HoldKey,
        #[serde(rename = "SidestepSpeed")]
        sidestep_speed: f32,
        #[serde(rename = "TurnCommand")]
        turn_command: MovementCommand,
        #[serde(rename = "TurnHoldkey")]
        turn_holdkey: u32,
        #[serde(rename = "TurnSpeed")]
        turn_speed: f32
}

// An autonomous position with sequences
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PositionPack {
        #[serde(rename = "Flags")]
        flags: PositionFlags,
        #[serde(rename = "Origin")]
        origin: Origin,
        #[serde(rename = "WQuat")]
        w_quat: f32,
        #[serde(rename = "XQuat")]
        x_quat: f32,
        #[serde(rename = "YQuat")]
        y_quat: f32,
        #[serde(rename = "ZQuat")]
        z_quat: f32,
        #[serde(rename = "Velocity")]
        velocity: Vector3,
        #[serde(rename = "PlacementId")]
        placement_id: u32,
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "MovementType")]
pub enum MovementData {
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
        sticky_object: ObjectId,
    },
}

// Contains information for animations and general free motion
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InterpertedMotionState {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "CurrentStyle")]
        current_style: StanceMode,
        #[serde(rename = "ForwardCommand")]
        forward_command: MovementCommand,
        #[serde(rename = "SidestepCommand")]
        sidestep_command: MovementCommand,
        #[serde(rename = "TurnCommand")]
        turn_command: MovementCommand,
        #[serde(rename = "ForwardSpeed")]
        forward_speed: f32,
        #[serde(rename = "SidestepSpeed")]
        sidestep_speed: f32,
        #[serde(rename = "TurnSpeed")]
        turn_speed: f32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDDRevision {
        #[serde(rename = "IdDatFile")]
        id_dat_file: u64,
        #[serde(rename = "Iteration")]
        iteration: u32,
        #[serde(rename = "IdsToDownload")]
        ids_to_download: PackableList,
        #[serde(rename = "IdsToPurge")]
        ids_to_purge: PackableList
}

// Set of movement parameters required for a MoveTo movement
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TurnToMovementParameters {
        #[serde(rename = "Bitmember")]
        bitmember: u32,
        #[serde(rename = "AnimationSpeed")]
        animation_speed: f32,
        #[serde(rename = "DesiredHeading")]
        desired_heading: f32
}

// The ObjDesc structure defines an object's visual appearance.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        palette: DataId
}

// Contains data for a subpalette
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Subpalette {
        #[serde(rename = "Palette")]
        palette: DataId,
        #[serde(rename = "Offset")]
        offset: u8,
        #[serde(rename = "NumColors")]
        num_colors: u8
}

// Contains data for texture map changes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TextureMapChange {
        #[serde(rename = "PartIndex")]
        part_index: u8,
        #[serde(rename = "OldTexId")]
        old_tex_id: DataId,
        #[serde(rename = "NewTexId")]
        new_tex_id: DataId
}

// Contains data for animation part changes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnimPartChange {
        #[serde(rename = "PartIndex")]
        part_index: u8,
        #[serde(rename = "PartId")]
        part_id: DataId
}

// Data for a character creation
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        skills: PackableList,
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CharacterIdentity {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "SecondsGreyedOut")]
        seconds_greyed_out: u32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EquipLocation {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Slot")]
        slot: EquipMask
}

// The PhysicsDesc structure defines an object's physical behavior.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PhysicsDesc {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "State")]
        state: PhysicsState,
        #[serde(rename = "MovementBuffer")]
        movement_buffer: PackableList,
        #[serde(rename = "Autonomous")]
        autonomous: bool,
        #[serde(rename = "AnimationFrame")]
        animation_frame: u32,
        #[serde(rename = "Position")]
        position: Position,
        #[serde(rename = "MotionId")]
        motion_id: DataId,
        #[serde(rename = "SoundId")]
        sound_id: DataId,
        #[serde(rename = "PhysicsScriptId")]
        physics_script_id: DataId,
        #[serde(rename = "SetupId")]
        setup_id: DataId,
        #[serde(rename = "ParentId")]
        parent_id: ObjectId,
        #[serde(rename = "ParentLocation")]
        parent_location: ParentLocation,
        #[serde(rename = "Children")]
        children: PackableList,
        #[serde(rename = "Scale")]
        scale: f32,
        #[serde(rename = "Friction")]
        friction: f32,
        #[serde(rename = "Elasticity")]
        elasticity: f32,
        #[serde(rename = "Translucency")]
        translucency: f32,
        #[serde(rename = "Velocity")]
        velocity: Vector3,
        #[serde(rename = "Acceleration")]
        acceleration: Vector3,
        #[serde(rename = "Omega")]
        omega: Vector3,
        #[serde(rename = "DefaultScript")]
        default_script: u32,
        #[serde(rename = "DefaultScriptIntensity")]
        default_script_intensity: f32,
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdminAccountData {
        #[serde(rename = "AccountName")]
        account_name: String,
        #[serde(rename = "BookieId")]
        bookie_id: u32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdminPlayerData {
        name: String,
        #[serde(rename = "bookieId")]
        bookie_id: u32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreatureAppraisalProfile {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Health")]
        health: u32,
        #[serde(rename = "HealthMax")]
        health_max: u32,
        #[serde(rename = "Strength")]
        strength: u32,
        #[serde(rename = "Endurance")]
        endurance: u32,
        #[serde(rename = "Quickness")]
        quickness: u32,
        #[serde(rename = "Coordination")]
        coordination: u32,
        #[serde(rename = "Focus")]
        focus: u32,
        #[serde(rename = "Self")]
        self_: u32,
        #[serde(rename = "Stamina")]
        stamina: u32,
        #[serde(rename = "Mana")]
        mana: u32,
        #[serde(rename = "StaminaMax")]
        stamina_max: u32,
        #[serde(rename = "ManaMax")]
        mana_max: u32,
        #[serde(rename = "AttrHighlight")]
        attr_highlight: AttributeMask,
        #[serde(rename = "AttrColor")]
        attr_color: AttributeMask
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HookAppraisalProfile {
        #[serde(rename = "Flags")]
        flags: HookAppraisalFlags,
        #[serde(rename = "ValidLocations")]
        valid_locations: EquipMask,
        #[serde(rename = "AmmoType")]
        ammo_type: AmmoType
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SquelchDB {
        #[serde(rename = "AccountHash")]
        account_hash: PackableHashTable,
        #[serde(rename = "CharacterHash")]
        character_hash: PackableHashTable,
        #[serde(rename = "GlobalInfo")]
        global_info: SquelchInfo
}

// Set of information related to a squelch entry
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SquelchInfo {
        #[serde(rename = "Filters")]
        filters: PackableList,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "Account")]
        account: bool
}

// Set of information related to purchasing a housing
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        buy: PackableList,
        #[serde(rename = "Rent")]
        rent: PackableList
}

// The HousePayment structure contains information about a house purchase or maintenance item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        buy: PackableList,
        #[serde(rename = "Rent")]
        rent: PackableList,
        #[serde(rename = "Position")]
        position: Position
}

// Set of information related to house access
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HAR {
        #[serde(rename = "Version")]
        version: u32,
        #[serde(rename = "Bitmask")]
        bitmask: u32,
        #[serde(rename = "MonarchId")]
        monarch_id: ObjectId,
        #[serde(rename = "GuestList")]
        guest_list: PackableHashTable,
        #[serde(rename = "RoommateList")]
        roommate_list: PackableList
}

// Set of information related to a house guest
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GuestInfo {
        #[serde(rename = "HasStoragePermission")]
        has_storage_permission: bool,
        #[serde(rename = "GuestName")]
        guest_name: String
}

// Set of information related to a chess game move
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SalvageOperationsResultData {
        #[serde(rename = "SkillUsed")]
        skill_used: SkillId,
        #[serde(rename = "NotSalvagable")]
        not_salvagable: PackableList,
        #[serde(rename = "SalvageResults")]
        salvage_results: PackableList,
        #[serde(rename = "AugBonus")]
        aug_bonus: i32
}

// Set of information related to a salvage of an item
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SalvageResult {
        #[serde(rename = "Material")]
        material: MaterialType,
        #[serde(rename = "Workmanship")]
        workmanship: f64,
        #[serde(rename = "Units")]
        units: u32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship {
        #[serde(rename = "Members")]
        members: PackableHashTable,
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
        recently_departed: PackableHashTable,
        #[serde(rename = "Locks")]
        locks: PackableHashTable
}

// The FellowInfo structure contains information about a fellowship member.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContractTrackerTable {
        #[serde(rename = "ContactTrackers")]
        contact_trackers: PackableHashTable
}

// Set a single character option.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_PlayerOptionChangedEvent {
        #[serde(rename = "Option")]
        option: CharacterOptions1,
        #[serde(rename = "Value")]
        value: bool
}

// Starts a melee attack against a target
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_TargetedMeleeAttack {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Height")]
        height: AttackHeight,
        #[serde(rename = "Power")]
        power: f32
}

// Starts a missle attack against a target
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_TargetedMissileAttack {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Height")]
        height: AttackHeight,
        #[serde(rename = "Accuracy")]
        accuracy: f32
}

// Set AFK mode.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_SetAFKMode {
        #[serde(rename = "AFK")]
        afk: bool
}

// Set AFK message.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_SetAFKMessage {
        #[serde(rename = "Message")]
        message: String
}

// Talking
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_Talk {
        #[serde(rename = "Message")]
        message: String
}

// Removes a friend
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_RemoveFriend {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Adds a friend
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_AddFriend {
        #[serde(rename = "CharacterName")]
        character_name: String
}

// Store an item in a container.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_PutItemInContainer {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "SlotIndex")]
        slot_index: u32
}

// Gets and weilds an item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_GetAndWieldItem {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Slot")]
        slot: EquipMask
}

// Drop an item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_DropItem {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Swear allegiance
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SwearAllegiance {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Break allegiance
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_BreakAllegiance {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Allegiance update request
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_UpdateRequest {
        #[serde(rename = "On")]
        on: bool
}

// Clears friend list
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_ClearFriends {}

// Teleport to the PKLite Arena
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_TeleToPKLArena {}

// Teleport to the PK Arena
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_TeleToPKArena {}

// Sets a character's display title
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_SetDisplayCharacterTitle {
        #[serde(rename = "TitleId")]
        title_id: u32
}

// Query the allegiance name
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_QueryAllegianceName {}

// Clears the allegiance name
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ClearAllegianceName {}

// Direct message by Id
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_TalkDirect {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "TargetId")]
        target_id: ObjectId
}

// Sets the allegiance name
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SetAllegianceName {
        #[serde(rename = "Name")]
        name: String
}

// Attempt to use an item with a target.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_UseWithTargetEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "TargetId")]
        target_id: ObjectId
}

// Attempt to use an item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_UseEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Sets an allegiance officer
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SetAllegianceOfficer {
        #[serde(rename = "CharacterName")]
        character_name: String,
        #[serde(rename = "Level")]
        level: AllegianceOfficerLevel
}

// Sets an allegiance officer title for a given level
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SetAllegianceOfficerTitle {
        #[serde(rename = "Level")]
        level: AllegianceOfficerLevel,
        #[serde(rename = "Title")]
        title: String
}

// List the allegiance officer titles
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ListAllegianceOfficerTitles {}

// Clears the allegiance officer titles
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ClearAllegianceOfficerTitles {}

// Perform the allegiance lock action
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_DoAllegianceLockAction {
        #[serde(rename = "Action")]
        action: AllegianceLockAction
}

// Sets a person as an approved vassal
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SetAllegianceApprovedVassal {
        #[serde(rename = "CharacterName")]
        character_name: String
}

// Gags a person in allegiance chat
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceChatGag {
        #[serde(rename = "CharacterName")]
        character_name: String,
        #[serde(rename = "On")]
        on: bool
}

// Perform the allegiance house action
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_DoAllegianceHouseAction {
        #[serde(rename = "Action")]
        action: AllegianceHouseAction
}

// Spend XP to raise a vital.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Train_TrainAttribute2nd {
        #[serde(rename = "Type")]
        type_: VitalId,
        #[serde(rename = "Experience")]
        experience: u32
}

// Spend XP to raise an attribute.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Train_TrainAttribute {
        #[serde(rename = "Type")]
        type_: AttributeId,
        #[serde(rename = "Experience")]
        experience: u32
}

// Spend XP to raise a skill.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Train_TrainSkill {
        #[serde(rename = "Skill")]
        skill: SkillId,
        #[serde(rename = "Experience")]
        experience: u32
}

// Spend skill credits to train a skill.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Train_TrainSkillAdvancementClass {
        #[serde(rename = "Skill")]
        skill: SkillId,
        #[serde(rename = "Credits")]
        credits: u32
}

// Cast a spell with no target.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_CastUntargetedSpell {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Cast a spell on a target
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_CastTargetedSpell {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Changes the combat mode
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_ChangeCombatMode {
        #[serde(rename = "Mode")]
        mode: CombatMode
}

// Merges one stack with another
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_StackableMerge {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "TargetId")]
        target_id: ObjectId,
        #[serde(rename = "Amount")]
        amount: u32
}

// Split a stack and place it into a container
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_StackableSplitToContainer {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "SlotIndex")]
        slot_index: u32,
        #[serde(rename = "Amount")]
        amount: u32
}

// Split a stack and place it into the world
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_StackableSplitTo3D {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Amount")]
        amount: u32
}

// Changes an account squelch
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ModifyCharacterSquelch {
        #[serde(rename = "Add")]
        add: bool,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "CharacterName")]
        character_name: String,
        #[serde(rename = "Type")]
        type_: ChatFragmentType
}

// Changes an account squelch
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ModifyAccountSquelch {
        #[serde(rename = "Add")]
        add: bool,
        #[serde(rename = "CharacterName")]
        character_name: String
}

// Changes the global filters, /filter -type as well as /chat and /notell
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ModifyGlobalSquelch {
        #[serde(rename = "Add")]
        add: bool,
        #[serde(rename = "Type")]
        type_: ChatFragmentType
}

// Direct message by name
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_TalkDirectByName {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "TargetName")]
        target_name: String
}

// Buy from a vendor
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vendor_Buy {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList,
        #[serde(rename = "AlternateCurrencyId")]
        alternate_currency_id: u32
}

// Sell to a vendor
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vendor_Sell {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList
}

// Teleport to your lifestone. (/lifestone)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_TeleToLifestone {}

// The client is ready for the character to materialize after portalling or logging on.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_LoginCompleteNotification {}

// Create a fellowship
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Create {
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "ShareXP")]
        share_xp: bool
}

// Quit the fellowship
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Quit {
        #[serde(rename = "Disband")]
        disband: bool
}

// Dismiss a player from the fellowship
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Dismiss {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Recruit a player to the fellowship
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Recruit {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Update request
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_UpdateRequest {
        #[serde(rename = "On")]
        on: bool
}

// Request update to book data (seems to be sent after failed add page)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookAddPage {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Updates a page in a book
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookModifyPage {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "PageNum")]
        page_num: i32,
        #[serde(rename = "PageText")]
        page_text: String
}

// Add a page to a book
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookData {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Removes a page from a book
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookDeletePage {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "PageNum")]
        page_num: i32
}

// Requests data for a page in a book
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookPageData {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "PageNum")]
        page_num: i32
}

// Sets the inscription on an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_SetInscription {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Inscription")]
        inscription: String
}

// Appraise something
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_Appraise {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Give an item to someone.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_GiveObjectRequest {
        #[serde(rename = "TargetId")]
        target_id: ObjectId,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Amount")]
        amount: u32
}

// Advocate Teleport
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Advocate_Teleport {
        #[serde(rename = "ObjectId")]
        object_id: String,
        #[serde(rename = "Destination")]
        destination: Position
}

// Sends an abuse report.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_AbuseLogRequest {
        #[serde(rename = "Character")]
        character: String,
        #[serde(rename = "Status")]
        status: u32,
        #[serde(rename = "Complaint")]
        complaint: String
}

// Joins a chat channel
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_AddToChannel {
        #[serde(rename = "Channel")]
        channel: Channel
}

// Leaves a chat channel
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_RemoveFromChannel {
        #[serde(rename = "Channel")]
        channel: Channel
}

// Sends a message to a chat channel
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelBroadcast {
        #[serde(rename = "Channel")]
        channel: Channel,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "Message")]
        message: String
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelList {
        #[serde(rename = "Channel")]
        channel: Channel
}

// Requests a channel index
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelIndex {}

// Stop viewing the contents of a container
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_NoLongerViewingContents {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Splits an item to a wield location.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_StackableSplitToWield {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Slot")]
        slot: EquipMask,
        #[serde(rename = "Amount")]
        amount: i32
}

// Add an item to the shortcut bar.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_AddShortCut {
        #[serde(rename = "Shortcut")]
        shortcut: ShortCutData
}

// Remove an item from the shortcut bar.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_RemoveShortCut {
        #[serde(rename = "Index")]
        index: u32
}

// Set multiple character options.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_CharacterOptionsEvent {
        #[serde(rename = "Options")]
        options: PlayerModule
}

// Removes a spell from the spell book
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_RemoveSpell {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Cancels an attack
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_CancelAttack {}

// Query's a creatures health
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_QueryHealth {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Query a character's age
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_QueryAge {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Query a character's birth day
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_QueryBirth {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Emote message
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_Emote {
        #[serde(rename = "Message")]
        message: String
}

// Soul emote message
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_SoulEmote {
        #[serde(rename = "Message")]
        message: String
}

// Add a spell to a spell bar.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_AddSpellFavorite {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId,
        #[serde(rename = "Index")]
        index: u32,
        #[serde(rename = "SpellBar")]
        spell_bar: u32
}

// Remove a spell from a spell bar.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_RemoveSpellFavorite {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId,
        #[serde(rename = "SpellBar")]
        spell_bar: u32
}

// Request a ping
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_RequestPing {}

// Starts trading with another player.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_OpenTradeNegotiations {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Ends trading, when trade window is closed?
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_CloseTradeNegotiations {}

// Adds an object to the trade.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_AddToTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "SlotIndex")]
        slot_index: u32
}

// Accepts a trade.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_AcceptTrade {
        #[serde(rename = "Contents")]
        contents: Trade
}

// Declines a trade, when cancel is clicked?
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_DeclineTrade {}

// Resets the trade, when clear all is clicked?
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_ResetTrade {}

// Clears the player's corpse looting consent list, /consent clear
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_ClearPlayerConsentList {}

// Display the player's corpse looting consent list, /consent who 
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_DisplayPlayerConsentList {}

// Remove your corpse looting permission for the given player, /consent remove 
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_RemoveFromPlayerConsentList {
        #[serde(rename = "TargetName")]
        target_name: String
}

// Grants a player corpse looting permission, /permit add
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_AddPlayerPermission {
        #[serde(rename = "TargetName")]
        target_name: String
}

// Buy a house
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_BuyHouse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList
}

// Query your house info, during signin
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_QueryHouse {}

// Abandons your house
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_AbandonHouse {}

// Revokes a player's corpse looting permission, /permit remove
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_RemovePlayerPermission {
        #[serde(rename = "TargetName")]
        target_name: String
}

// Pay rent for a house
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_RentHouse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList
}

// Sets a new fill complevel for a component
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_SetDesiredComponentLevel {
        #[serde(rename = "Wcid")]
        wcid: u32,
        #[serde(rename = "Amount")]
        amount: u32
}

// Adds a guest to your house guest list
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_AddPermanentGuest {
        #[serde(rename = "GuestName")]
        guest_name: String
}

// Removes a specific player from your house guest list, /house guest remove
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_RemovePermanentGuest {
        #[serde(rename = "GuestName")]
        guest_name: String
}

// Sets your house open status
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_SetOpenHouseStatus {
        #[serde(rename = "OpenHouse")]
        open_house: bool
}

// Changes a specific players storage permission, /house storage add/remove
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_ChangeStoragePermission {
        #[serde(rename = "GuestName")]
        guest_name: String,
        #[serde(rename = "HasPermission")]
        has_permission: bool
}

// Boots a specific player from your house /house boot
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_BootSpecificHouseGuest {
        #[serde(rename = "GuestName")]
        guest_name: String
}

// Removes all storage permissions, /house storage remove_all
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_RemoveAllStoragePermission {}

// Requests your full guest list, /house guest list
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_RequestFullGuestList {}

// Sets the allegiance message of the day, /allegiance motd set
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SetMotd {
        #[serde(rename = "Message")]
        message: String
}

// Query the motd, /allegiance motd
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_QueryMotd {}

// Clear the motd, /allegiance motd clear
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ClearMotd {}

// Gets SlumLord info, sent after getting a failed house transaction
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_QueryLord {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Adds all to your storage permissions, /house storage add -all
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_AddAllStoragePermission {}

// Removes all guests, /house guest remove_all
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_RemoveAllPermanentGuests {}

// Boot everyone from your house, /house boot -all
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_BootEveryone {}

// Teleports you to your house, /house recall
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_TeleToHouse {}

// Queries an item's mana
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_QueryItemMana {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Modify whether house hooks are visibile or not, /house hooks on/off
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_SetHooksVisibility {
        #[serde(rename = "Visible")]
        visible: bool
}

// Modify whether allegiance members are guests, /house guest add_allegiance/remove_allegiance
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_ModifyAllegianceGuestPermission {
        #[serde(rename = "Add")]
        add: bool
}

// Modify whether allegiance members can access storage, /house storage add_allegiance/remove_allegiance
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_ModifyAllegianceStoragePermission {
        #[serde(rename = "Add")]
        add: bool
}

// Joins a chess game
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_Join {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: u32
}

// Quits a chess game
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_Quit {}

// Makes a chess move
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_Move {
        #[serde(rename = "XFrom")]
        x_from: i32,
        #[serde(rename = "YFrom")]
        y_from: i32,
        #[serde(rename = "XTo")]
        x_to: i32,
        #[serde(rename = "YTo")]
        y_to: i32
}

// Skip a move?
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_MovePass {}

// Offer or confirm stalemate
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_Stalemate {
        #[serde(rename = "On")]
        on: bool
}

// Lists available house /house available
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_ListAvailableHouses {
        #[serde(rename = "Type")]
        type_: HouseType
}

// Confirms a dialog
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_ConfirmationResponse {
        #[serde(rename = "Type")]
        type_: ConfirmationType,
        #[serde(rename = "Context")]
        context: u32,
        #[serde(rename = "Accepted")]
        accepted: bool
}

// Boots a player from the allegiance, optionally all characters on their account
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_BreakAllegianceBoot {
        #[serde(rename = "BooteeName")]
        bootee_name: String,
        #[serde(rename = "AccountBoot")]
        account_boot: bool
}

// Teleports player to their allegiance housing, /house mansion_recall
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_TeleToMansion {}

// Player is commiting suicide
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_Suicide {}

// Request allegiance info for a player
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceInfoRequest {
        #[serde(rename = "TargetName")]
        target_name: String
}

// Salvages items
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_CreateTinkeringTool {
        #[serde(rename = "ToolId")]
        tool_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList
}

// Changes the spell book filter
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_SpellbookFilterEvent {
        #[serde(rename = "Options")]
        options: SpellBookFilterOptions
}

// Teleport to the marketplace
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_TeleToMarketplace {}

// Enter PKLite mode
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_EnterPKLite {}

// Fellowship Assign a new leader
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_AssignNewLeader {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Fellowship Change openness
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_ChangeFellowOpeness {
        #[serde(rename = "Open")]
        open: bool
}

// Boots a player from the allegiance chat
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceChatBoot {
        #[serde(rename = "CharacterName")]
        character_name: String,
        #[serde(rename = "Reason")]
        reason: String
}

// Bans a player from the allegiance
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AddAllegianceBan {
        #[serde(rename = "CharacterName")]
        character_name: String
}

// Removes a player ban from the allegiance
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_RemoveAllegianceBan {
        #[serde(rename = "CharacterName")]
        character_name: String
}

// Display allegiance bans
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ListAllegianceBans {}

// Removes an allegiance officer
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_RemoveAllegianceOfficer {
        #[serde(rename = "CharacterName")]
        character_name: String
}

// List allegiance officers
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ListAllegianceOfficers {}

// Clear allegiance officers
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ClearAllegianceOfficers {}

// Recalls to players allegiance hometown
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_RecallAllegianceHometown {}

// Admin Returns a plugin list response
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_QueryPluginListResponse {
        #[serde(rename = "Context")]
        context: u32,
        #[serde(rename = "PluginList")]
        plugin_list: String
}

// Admin Returns plugin info
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_QueryPluginResponse {
        #[serde(rename = "Context")]
        context: u32,
        #[serde(rename = "Success")]
        success: bool,
        #[serde(rename = "PluginName")]
        plugin_name: String,
        #[serde(rename = "PluginAuthor")]
        plugin_author: String,
        #[serde(rename = "PluginEmail")]
        plugin_email: String,
        #[serde(rename = "PluginWebpage")]
        plugin_webpage: String
}

// Completes the barber interaction
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_FinishBarber {
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

// Abandons a contract
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_AbandonContract {
        #[serde(rename = "ContractId")]
        contract_id: ContractId
}

// Performs a jump
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_Jump {
        #[serde(rename = "Jump")]
        jump: JumpPack
}

// Move to state data
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_MoveToState {
        #[serde(rename = "MoveToState")]
        move_to_state: MoveToStatePack
}

// Performs a movement based on input
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_DoMovementCommand {
        #[serde(rename = "Motion")]
        motion: u32,
        #[serde(rename = "Speed")]
        speed: f32,
        #[serde(rename = "HoldKey")]
        hold_key: HoldKey
}

// Stops a movement
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_StopMovementCommand {
        #[serde(rename = "Motion")]
        motion: u32,
        #[serde(rename = "HoldKey")]
        hold_key: HoldKey
}

// Sets an autonomy level
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_AutonomyLevel {
        #[serde(rename = "AutonomyLevel")]
        autonomy_level: u32
}

// Sends an autonomous position
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_AutonomousPosition {
        #[serde(rename = "Position")]
        position: AutonomousPositionPack
}

// Performs a non autonomous jump
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_Jump_NonAutonomous {
        #[serde(rename = "Extent")]
        extent: f32
}

// Allegiance update cancelled
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceUpdateAborted {
        #[serde(rename = "FailureType")]
        failure_type: WeenieError
}

// Display a message in a popup message window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_PopUpString {
        #[serde(rename = "Message")]
        message: String
}

// Information describing your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_PlayerDescription {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceUpdate {
        #[serde(rename = "Rank")]
        rank: u32,
        #[serde(rename = "Profile")]
        profile: AllegianceProfile
}

// Friends list update
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_FriendsUpdate {
        #[serde(rename = "Friends")]
        friends: PackableList,
        #[serde(rename = "Type")]
        type_: FriendsUpdateType
}

// Store an item in a container.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_ServerSaysContainId {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_WearItem {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Slot")]
        slot: EquipMask
}

// Titles for the current character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_CharacterTitleTable {
        #[serde(rename = "DisplayTitle")]
        display_title: u32,
        #[serde(rename = "Titles")]
        titles: PackableList
}

// Set a title for the current character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_AddOrSetCharacterTitle {
        #[serde(rename = "NewTitle")]
        new_title: u32,
        #[serde(rename = "SetAsDisplayTitle")]
        set_as_display_title: bool
}

// Close Container - Only sent when explicitly closed
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_StopViewingObjectContents {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Open the buy/sell panel for a merchant.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vendor_VendorInfo {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Profile")]
        profile: VendorProfile,
        #[serde(rename = "Items")]
        items: PackableList
}

// Opens barber UI
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_StartBarber {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Quit {
        #[serde(rename = "Disband")]
        disband: bool
}

// Member dismissed from fellowship
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Dismiss {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Sent when you first open a book, contains the entire table of contents.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookOpen {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookAddPageResponse {
        #[serde(rename = "BookId")]
        book_id: ObjectId,
        #[serde(rename = "PageNumber")]
        page_number: u32,
        #[serde(rename = "Success")]
        success: bool
}

// Response to an attempt to delete a page from a book.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookDeletePageResponse {
        #[serde(rename = "BookId")]
        book_id: ObjectId,
        #[serde(rename = "PageNumber")]
        page_number: u32,
        #[serde(rename = "Success")]
        success: bool
}

// Contains the text of a single page of a book, parchment or sign.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookPageDataResponse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Page")]
        page: u32,
        #[serde(rename = "PageData")]
        page_data: PageData
}

// Get Inscription Response, doesn't seem to be really used by client
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_GetInscriptionResponse {
        #[serde(rename = "Inscription")]
        inscription: String,
        #[serde(rename = "ScribeName")]
        scribe_name: String,
        #[serde(rename = "ScribeAccount")]
        scribe_account: String
}

// The result of an attempt to assess an item or creature.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_SetAppraiseInfo {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelBroadcast {
        #[serde(rename = "Channel")]
        channel: Channel,
        #[serde(rename = "Message")]
        message: String
}

// ChannelList: Provides list of characters listening to a channel, I assume in response to a command
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelList {
        #[serde(rename = "Characters")]
        characters: PackableList
}

// ChannelIndex: Provides list of channels available to the player, I assume in response to a command
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelIndex {
        #[serde(rename = "Channels")]
        channels: PackableList
}

// Set Pack Contents
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_OnViewContents {
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList
}

// ServerSaysMoveItem: Removes an item from inventory (when you place it on the ground or give it away)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_ServerSaysMoveItem {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// HandleAttackDoneEvent: Melee attack completed
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleAttackDoneEvent {
        #[serde(rename = "Number")]
        number: u32
}

// RemoveSpell: Delete a spell from your spellbook.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_RemoveSpell {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// You just died.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleVictimNotificationEventSelf {
        #[serde(rename = "Message")]
        message: String
}

// Message for a death, something you killed or your own death message.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleVictimNotificationEventOther {
        #[serde(rename = "Message")]
        message: String
}

// HandleAttackerNotificationEvent: You have hit your target with a melee attack.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleAttackerNotificationEvent {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleDefenderNotificationEvent {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleEvasionAttackerNotificationEvent {
        #[serde(rename = "DefenderName")]
        defender_name: String
}

// HandleEvasionDefenderNotificationEvent: You have evaded a creature's melee attack.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleEvasionDefenderNotificationEvent {
        #[serde(rename = "AttackerName")]
        attacker_name: String
}

// HandleCommenceAttackEvent: Start melee attack
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleCommenceAttackEvent {}

// QueryHealthResponse: Update a creature's health bar.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_QueryHealthResponse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Health")]
        health: f32
}

// QueryAgeResponse: happens when you do /age in the game
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_QueryAgeResponse {
        #[serde(rename = "TargetName")]
        target_name: String,
        #[serde(rename = "Age")]
        age: String
}

// UseDone: Ready. Previous action complete
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_UseDone {
        #[serde(rename = "FailureType")]
        failure_type: WeenieError
}

// Allegiance update finished
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceUpdateDone {
        #[serde(rename = "FailureType")]
        failure_type: WeenieError
}

// Fellow update is done
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_FellowUpdateDone {}

// Fellow stats are done
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_FellowStatsDone {}

// Close Assess Panel
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_AppraiseDone {
        #[serde(rename = "Unknown")]
        unknown: u32
}

// Ping Reply
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_ReturnPing {}

// Squelch and Filter List
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_SetSquelchDB {
        #[serde(rename = "SquelchDB")]
        squelch_db: SquelchDB
}

// RegisterTrade: Send to begin a trade and display the trade window
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_RegisterTrade {
        #[serde(rename = "InitiatorId")]
        initiator_id: ObjectId,
        #[serde(rename = "PartnerId")]
        partner_id: ObjectId,
        #[serde(rename = "Stamp")]
        stamp: i64
}

// OpenTrade: Open trade window
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_OpenTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// CloseTrade: End trading
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_CloseTrade {
        #[serde(rename = "Reason")]
        reason: EndTradeReason
}

// RemoveFromTrade: Item was removed from trade window
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_AddToTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Side")]
        side: TradeSide
}

// Removes an item from the trade window, not sure if this is used still?
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_RemoveFromTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Side")]
        side: TradeSide
}

// AcceptTrade: The trade was accepted
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_AcceptTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// DeclineTrade: The trade was declined
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_DeclineTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// ResetTrade: The trade window was reset
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_ResetTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// TradeFailure: Failure to add a trade item
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_TradeFailure {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Reason")]
        reason: u32
}

// ClearTradeAcceptance: Failure to complete a trade
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_ClearTradeAcceptance {}

// Buy a dwelling or pay maintenance
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_HouseProfile {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Profile")]
        profile: HouseProfile
}

// House panel information for owners.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_HouseData {
        #[serde(rename = "Data")]
        data: HouseData
}

// House Data
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_HouseStatus {
        #[serde(rename = "NoticeType")]
        notice_type: u32
}

// Update Rent Time
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_UpdateRentTime {
        #[serde(rename = "RentTime")]
        rent_time: u32
}

// Update Rent Payment
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_UpdateRentPayment {
        #[serde(rename = "Rent")]
        rent: PackableList
}

// Update Restrictions
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_UpdateRestrictions {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "Restrictions")]
        restrictions: RestrictionDB
}

// House Guest List
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_UpdateHAR {
        #[serde(rename = "GuestList")]
        guest_list: HAR
}

// House Profile
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_HouseTransaction {
        #[serde(rename = "NoticeType")]
        notice_type: u32
}

// Update an item's mana bar.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_QueryItemManaResponse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Mana")]
        mana: f32,
        #[serde(rename = "Success")]
        success: bool
}

// Display a list of available dwellings in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_AvailableHouses {
        #[serde(rename = "Type")]
        type_: HouseType,
        #[serde(rename = "Houses")]
        houses: PackableList,
        #[serde(rename = "NumHouses")]
        num_houses: i32
}

// Display a confirmation panel.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_ConfirmationRequest {
        #[serde(rename = "ConfirmationType")]
        confirmation_type: ConfirmationType,
        #[serde(rename = "ContextId")]
        context_id: u32,
        #[serde(rename = "Text")]
        text: String
}

// Confirmation done
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_ConfirmationDone {
        #[serde(rename = "ConfirmationType")]
        confirmation_type: ConfirmationType,
        #[serde(rename = "ContextId")]
        context_id: u32
}

// Display an allegiance login/logout message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceLoginNotificationEvent {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId,
        #[serde(rename = "IsLoggedIn")]
        is_logged_in: bool
}

// Returns data for a player's allegiance information
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceInfoResponseEvent {
        #[serde(rename = "TargetId")]
        target_id: ObjectId,
        #[serde(rename = "Profile")]
        profile: AllegianceProfile
}

// Joining game response
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_JoinGameResponse {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32
}

// Start game
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_StartGame {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32
}

// Move response
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_MoveResponse {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "MoveResult")]
        move_result: ChessMoveResult
}

// Opponent Turn
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_OpponentTurn {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32,
        #[serde(rename = "GameMove")]
        game_move: GameMoveData
}

// Opponent Stalemate State
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_OpponentStalemateState {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32,
        #[serde(rename = "On")]
        on: bool
}

// Display a status message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_WeenieError {
        #[serde(rename = "Type")]
        type_: WeenieError
}

// Display a parameterized status message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_WeenieErrorWithString {
        #[serde(rename = "Type")]
        type_: WeenieErrorWithString,
        #[serde(rename = "Text")]
        text: String
}

// End of Chess game
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_GameOver {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "TeamWinner")]
        team_winner: i32
}

// Set Turbine Chat channel numbers.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChatRoomTracker {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_QueryPluginList {}

// TODO: QueryPlugin
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_QueryPlugin {}

// TODO: QueryPluginResponse
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_QueryPluginResponse2 {}

// Salvage operation results
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_SalvageOperationsResultData {
        #[serde(rename = "Result")]
        result: SalvageOperationsResultData
}

// Someone has sent you a @tell.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_HearDirectSpeech {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_FullUpdate {
        #[serde(rename = "Fellowship")]
        fellowship: Fellowship
}

// Disband your fellowship.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Disband {}

// Add/Update a member to your fellowship.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_UpdateFellow {
        #[serde(rename = "Fellow")]
        fellow: Fellow,
        #[serde(rename = "UpdateType")]
        update_type: FellowUpdateType
}

// Add a spell to your spellbook.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_UpdateSpell {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Apply an enchantment to your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_UpdateEnchantment {
        #[serde(rename = "Enchantment")]
        enchantment: Enchantment
}

// Remove an enchantment from your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_RemoveEnchantment {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Update multiple enchantments from your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_UpdateMultipleEnchantments {
        #[serde(rename = "Enchantments")]
        enchantments: PackableList
}

// Remove multiple enchantments from your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_RemoveMultipleEnchantments {
        #[serde(rename = "Enchantments")]
        enchantments: PackableList
}

// Silently remove all enchantments from your character, e.g. when you die (no message in the chat window).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_PurgeEnchantments {}

// Silently remove An enchantment from your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_DispelEnchantment {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Silently remove multiple enchantments from your character (no message in the chat window).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_DispelMultipleEnchantments {
        #[serde(rename = "Enchantments")]
        enchantments: PackableList
}

// A portal storm is brewing.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Misc_PortalStormBrewing {
        #[serde(rename = "Extent")]
        extent: f32
}

// A portal storm is imminent.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Misc_PortalStormImminent {
        #[serde(rename = "Extent")]
        extent: f32
}

// You have been portal stormed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Misc_PortalStorm {}

// The portal storm has subsided.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Misc_PortalStormSubsided {}

// Display a status message on the Action Viewscreen (the red text overlaid on the 3D area).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_TransientString {
        #[serde(rename = "Message")]
        message: String
}

// Remove all bad enchantments from your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_PurgeBadEnchantments {}

// Sends all contract data
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_SendClientContractTrackerTable {
        #[serde(rename = "ContractTracker")]
        contract_tracker: ContractTrackerTable
}

// Updates a contract data
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_SendClientContractTracker {
        #[serde(rename = "ContractTracker")]
        contract_tracker: ContractTracker,
        #[serde(rename = "DeleteContract")]
        delete_contract: bool,
        #[serde(rename = "SetAsDisplayContract")]
        set_as_display_contract: bool
}

// Instructs the client to return to 2D mode - the character list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_LogOffCharacter {}

// Mark a character for deletetion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_CharacterDelete {
        #[serde(rename = "Account")]
        account: String,
        #[serde(rename = "Slot")]
        slot: i32
}

// Character creation result
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_SendCharGenResult {
        #[serde(rename = "Account")]
        account: String,
        #[serde(rename = "Result")]
        result: CharGenResult
}

// The character to log in.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_SendEnterWorld {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId,
        #[serde(rename = "Account")]
        account: String
}

// Asks server for a new object description
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Object_SendForceObjdesc {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// The user has clicked 'Enter'. This message does not contain the Id of the character logging on; that comes later.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_SendEnterWorldRequest {}

// Sent if player is an PSR, I assume it displays the server version number
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_SendAdminGetServerVersion {}

// Seems to be a legacy friends command, /friends old, for when Jan 2006 event changed the friends list
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_SendFriendsCommand {
        #[serde(rename = "Command")]
        command: u32,
        #[serde(rename = "Player")]
        player: String
}

// Admin command to restore a character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_SendAdminRestoreCharacter {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "RestoredCharName")]
        restored_char_name: String,
        #[serde(rename = "AccountToRestoreTo")]
        account_to_restore_to: String
}

// Send or receive a message using Turbine Chat.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "BlobDispatchType")]
pub enum Communication_TurbineChat {
    #[serde(rename = "0x02")]
    Type02 {
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
    #[serde(rename = "0x01")]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDD_RequestDataMessage {
        #[serde(rename = "ResourceType")]
        resource_type: u32,
        #[serde(rename = "ResourceId")]
        resource_id: DataId
}

// TODO
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDD_InterrogationResponseMessage {
        #[serde(rename = "Language")]
        language: u32,
        #[serde(rename = "Files")]
        files: PackableList
}

// Ends DDD message update
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDD_EndDDDMessage {}

// Ends DDD update
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDD_OnEndDDD {}

// Sent every time an object you are aware of ceases to exist. Merely running out of range does not generate this message - in that case, the client just automatically stops tracking it after receiving no updates for a while (which I presume is a very short while).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_ServerSaysRemove {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Failure to give an item
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_ServerSaysAttemptFailed {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Reason")]
        reason: WeenieError
}

// For stackable items, this changes the number of items in the stack.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_UpdateStackSize {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandlePlayerDeathEvent {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "KilledId")]
        killed_id: ObjectId,
        #[serde(rename = "KillerId")]
        killer_id: ObjectId
}

// Remove an int property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveIntEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyInt
}

// Remove an int property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveIntEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyInt
}

// Remove a bool property from the charactert
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveBoolEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyBool
}

// Remove a bool property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveBoolEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyBool
}

// Remove a float property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveFloatEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyFloat
}

// Remove a float property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveFloatEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyFloat
}

// Remove a string property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveStringEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyString
}

// Remove a string property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveStringEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyString
}

// Remove an dataId property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveDataIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyDataId
}

// Remove an dataId property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveDataIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyDataId
}

// Remove an instanceId property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveInstanceIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyInstanceId
}

// Remove an instanceId property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveInstanceIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyInstanceId
}

// Remove a position property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemovePositionEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyPosition
}

// Remove a position property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemovePositionEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyPosition
}

// Remove an int64 property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveInt64Event {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyInt64
}

// Remove an int64 property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveInt64Event {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyInt64
}

// Set or update a Character Int property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateInt {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyInt,
        #[serde(rename = "Value")]
        value: i32
}

// Set or update an Object Int property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateInt {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateInt64 {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyInt64,
        #[serde(rename = "Value")]
        value: i64
}

// Set or update a Character Int64 property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateInt64 {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateBool {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyBool,
        #[serde(rename = "Value")]
        value: bool
}

// Set or update an Object Boolean property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateBool {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateFloat {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyFloat,
        #[serde(rename = "Value")]
        value: f32
}

// Set or update an Object float property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateFloat {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateString {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyString,
        #[serde(rename = "Value")]
        value: String
}

// Set or update an Object String property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateString {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateDataId {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyDataId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update an Object DId property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateDataId {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateInstanceId {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyInstanceId,
        #[serde(rename = "Value")]
        value: ObjectId
}

// Set or update an Object IId property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateInstanceId {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdatePosition {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyPosition,
        #[serde(rename = "Value")]
        value: Position
}

// Set or update a Character Position property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdatePosition {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateSkill {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: Skill
}

// Set or update a Character Skill value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateSkill {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateSkillLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Skill Level
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateSkillLevel {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateSkillAC {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: SkillAdvancementClass
}

// Set or update a Character Skill state
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateSkillAC {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateAttribute {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: AttributeId,
        #[serde(rename = "Value")]
        value: AttributeInfo
}

// Set or update a Character Attribute value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateAttribute {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateAttributeLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: AttributeId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Attribute Level
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateAttributeLevel {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateAttribute2nd {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: VitalId,
        #[serde(rename = "Value")]
        value: SecondaryAttributeInfo
}

// Set or update a Character Vital value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateAttribute2nd {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateAttribute2ndLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: CurVitalId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Vital value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateAttribute2ndLevel {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_HearEmote {
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "Text")]
        text: String
}

// Contains the text associated with an emote action.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_HearSoulEmote {
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "Text")]
        text: String
}

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_HearSpeech {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_HearRangedSpeech {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_Environs {
        #[serde(rename = "EnvrionOption")]
        envrion_option: EnvrionChangeType
}

// Sets both the position and movement, such as when materializing at a lifestone
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_PositionAndMovementEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Position")]
        position: PositionPack,
        #[serde(rename = "MovementData")]
        movement_data: MovementData
}

// Sent whenever a character changes their clothes. It contains the entire description of what their wearing (and possibly their facial features as well). This message is only sent for changes, when the character is first created, the body of this message is included inside the creation message.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_ObjDescEvent {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_SetPlayerVisualDesc {
        #[serde(rename = "ObjectDescription")]
        object_description: ObjDesc
}

// Character creation screen initilised.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "ResponseType")]
pub enum Character_CharGenVerificationResponse {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_AwaitingSubscriptionExpiration {
        #[serde(rename = "SecondsRemaining")]
        seconds_remaining: u32
}

// Instructs the client to return to 2D mode - the character list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_LogOffCharacter {}

// A character was marked for deletetion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_CharacterDelete {}

// The list of characters on the current account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_LoginCharacterSet {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_CharacterError {
        #[serde(rename = "Reason")]
        reason: CharacterErrorType
}

// Create an object somewhere in the world
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_CreateObject {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_CreatePlayer {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId
}

// Sent whenever an object is being deleted from the scene.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_DeleteObject {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16
}

// Sets the position/motion of an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_PositionEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Position")]
        position: PositionPack
}

// Sets the parent for an object, eg. equipting an object.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_ParentEvent {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_PickupEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectPositionSequence")]
        object_position_sequence: u16
}

// Set's the current state of the object. Client appears to only process the following state changes post creation: NoDraw, LightingOn, Hidden
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_SetState {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_SetObjectMovement {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "MovementData")]
        movement_data: MovementData
}

// Changes an objects vector, for things like jumping
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_VectorUpdate {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Effects_SoundEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "SoundType")]
        sound_type: Sound,
        #[serde(rename = "Volume")]
        volume: f32
}

// Instructs the client to show the portal graphic.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Effects_PlayerTeleport {
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16
}

// Instructs the client to play a script. (Not seen so far, maybe prefered PlayScriptType)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Effects_PlayScriptId {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ScriptId")]
        script_id: DataId
}

// Applies an effect with visual and sound by providing the type to be looked up in the Physics Script Table
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Effects_PlayScriptType {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ScriptType")]
        script_type: i32,
        #[serde(rename = "Speed")]
        speed: f32
}

// Account has been banned
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_AccountBanned {
        #[serde(rename = "BannedUntil")]
        banned_until: u32,
        #[serde(rename = "Text")]
        text: String
}

// Admin Receive Account Data
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_ReceiveAccountData {
        #[serde(rename = "Unknown")]
        unknown: u32,
        #[serde(rename = "AdminAccountData")]
        admin_account_data: PackableList
}

// Admin Receive Player Data
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_ReceivePlayerData {
        #[serde(rename = "Unknown")]
        unknown: i32,
        #[serde(rename = "AdminPlayerData")]
        admin_player_data: PackableList
}

// Update an existing object's data.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_UpdateObject {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_AccountBooted {
        #[serde(rename = "AdditionalReasonText")]
        additional_reason_text: String,
        #[serde(rename = "ReasonText")]
        reason_text: String
}

// Send or receive a message using Turbine Chat.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "BlobDispatchType")]
pub enum Communication_TurbineChat {
    #[serde(rename = "0x02")]
    Type02 {
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
    #[serde(rename = "0x01")]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_EnterGame_ServerReady {}

// Display a message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_TextboxString {
        #[serde(rename = "Text")]
        text: String,
        #[serde(rename = "Type")]
        type_: ChatFragmentType
}

// The name of the current world.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_WorldInfo {
        #[serde(rename = "Connections")]
        connections: u32,
        #[serde(rename = "MaxConnections")]
        max_connections: u32,
        #[serde(rename = "WorldName")]
        world_name: String
}

// Add or update a dat file Resource.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Compression")]
pub enum DDD_DataMessage {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDD_ErrorMessage {
        #[serde(rename = "ResourceType")]
        resource_type: u32,
        #[serde(rename = "ResourceId")]
        resource_id: DataId,
        #[serde(rename = "RError")]
        r_error: u32
}

// A list of dat files that need to be patched
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDD_BeginDDDMessage {
        #[serde(rename = "DataExpected")]
        data_expected: u32,
        #[serde(rename = "Revisions")]
        revisions: PackableList
}

// Add or update a dat file Resource.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDD_InterrogationMessage {
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDD_OnEndDDD {}

// Client to Server AC packet.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        server_switch: ServerSwitchHeader,
        #[serde(rename = "RetransmitSequences")]
        retransmit_sequences: PackableList,
        #[serde(rename = "RejectSequences")]
        reject_sequences: PackableList,
        #[serde(rename = "AckSequence")]
        ack_sequence: u32,
        #[serde(rename = "LoginRequest")]
        login_request: LoginRequestHeader,
        #[serde(rename = "WorldLoginRequest")]
        world_login_request: u64,
        #[serde(rename = "ConnectResponse")]
        connect_response: u64,
        #[serde(rename = "CICMDCommand")]
        cicmd_command: CICMDCommandHeader,
        #[serde(rename = "Time")]
        time: u64,
        #[serde(rename = "EchoTime")]
        echo_time: f32,
        #[serde(rename = "Flow")]
        flow: FlowHeader,
        #[serde(rename = "Fragments")]
        fragments: BlobFragments
}

// Server to Client AC packet.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        ack_sequence: u32,
        #[serde(rename = "LogonServerAddr")]
        logon_server_addr: SocketAddress,
        #[serde(rename = "Referral")]
        referral: ReferralHeader,
        #[serde(rename = "ConnectRequest")]
        connect_request: ConnectRequestHeader,
        #[serde(rename = "NetError")]
        net_error: NetError,
        #[serde(rename = "NetErrorDisconnect")]
        net_error_disconnect: NetError,
        #[serde(rename = "EchoResponse")]
        echo_response: EchoResponseHeader,
        #[serde(rename = "Fragments")]
        fragments: BlobFragments
}

