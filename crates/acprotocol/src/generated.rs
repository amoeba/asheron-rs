// Full spell Id combining the spell id with the spell layer.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LayeredSpellId {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Layer")]
    layer: String
}

// List which is packable for network
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PackableList {
    #[serde(rename = "Count")]
    count: String
}

// HashTable which is packable for network
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PackableHashTable {
    #[serde(rename = "Count")]
    count: String,
    #[serde(rename = "MaxSize")]
    max_size: String
}

// HashTable which is packable for network
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PHashTable {
    #[serde(rename = "PackedSize")]
    packed_size: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vector3 {
    #[serde(rename = "X")]
    x: String,
    #[serde(rename = "Y")]
    y: String,
    #[serde(rename = "Z")]
    z: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Quaternion {
    #[serde(rename = "W")]
    w: String,
    #[serde(rename = "X")]
    x: String,
    #[serde(rename = "Y")]
    y: String,
    #[serde(rename = "Z")]
    z: String
}

// Landcell location, without orientation
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Origin {
    #[serde(rename = "Landcell")]
    landcell: String,
    #[serde(rename = "Location")]
    location: String
}

// Landcell location, including orientation
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Position {
    #[serde(rename = "Landcell")]
    landcell: String,
    #[serde(rename = "Frame")]
    frame: String
}

// A the location and orientation of an object within a landcell
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Frame {
    #[serde(rename = "Origin")]
    origin: String,
    #[serde(rename = "Orientation")]
    orientation: String
}

// Optional header data when PacketHeaderFlags includes ServerSwitch
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerSwitchHeader {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Type")]
    type_: String
}

// Optional header data when PacketHeaderFlags includes CICMDCommand
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CICMDCommandHeader {
    #[serde(rename = "Command")]
    command: String,
    #[serde(rename = "Parameter")]
    parameter: String
}

// Optional header data when PacketHeaderFlags includes Flow
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlowHeader {
    #[serde(rename = "Bytes")]
    bytes: String,
    #[serde(rename = "Interval")]
    interval: String
}

// Optional header data when PacketHeaderFlags includes LogonServerAddr
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SocketAddress {
    #[serde(rename = "Family")]
    family: String,
    #[serde(rename = "Port")]
    port: String,
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "Empty")]
    empty: String
}

// Optional header data when PacketHeaderFlags includes LoginRequest
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LoginRequestHeader {
    #[serde(rename = "ClientVersion")]
    client_version: String,
    #[serde(rename = "Length")]
    length: String,
    #[serde(rename = "AuthType")]
    auth_type: String,
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Account")]
    account: String,
    #[serde(rename = "AccountToLoginAs")]
    account_to_login_as: String
}

// Optional header data when PacketHeaderFlags includes Referral
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferralHeader {
    #[serde(rename = "Cookie")]
    cookie: String,
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "IdServer")]
    id_server: String,
    #[serde(rename = "Unknown")]
    unknown: String
}

// Optional header data when PacketHeaderFlags includes ConnectRequest
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConnectRequestHeader {
    #[serde(rename = "ServerTime")]
    server_time: String,
    #[serde(rename = "Cookie")]
    cookie: String,
    #[serde(rename = "NetID")]
    net_id: String,
    #[serde(rename = "OutgoingSeed")]
    outgoing_seed: String,
    #[serde(rename = "IncomingSeed")]
    incoming_seed: String,
    #[serde(rename = "Unknown")]
    unknown: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NetError {
    #[serde(rename = "StringId")]
    string_id: String,
    #[serde(rename = "TableId")]
    table_id: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EchoResponseHeader {
    #[serde(rename = "LocalTime")]
    local_time: String,
    #[serde(rename = "HoldingTime")]
    holding_time: String
}

// A collection of property tables.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ACBaseQualities {
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "WeenieType")]
    weenie_type: String,
    #[serde(rename = "IntProperties")]
    int_properties: String,
    #[serde(rename = "Int64Properties")]
    int64_properties: String,
    #[serde(rename = "BoolProperties")]
    bool_properties: String,
    #[serde(rename = "FloatProperties")]
    float_properties: String,
    #[serde(rename = "StringProperties")]
    string_properties: String,
    #[serde(rename = "DataProperties")]
    data_properties: String,
    #[serde(rename = "InstanceProperties")]
    instance_properties: String,
    #[serde(rename = "PositionProperties")]
    position_properties: String
}

// The ACQualities structure contains character property lists.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ACQualities {
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "HasHealth")]
    has_health: String,
    #[serde(rename = "Attributes")]
    attributes: String,
    #[serde(rename = "Skills")]
    skills: String,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "SpellBook")]
    spell_book: String,
    #[serde(rename = "Enchantments")]
    enchantments: String,
    #[serde(rename = "EventFilter")]
    event_filter: String,
    #[serde(rename = "Emotes")]
    emotes: String,
    #[serde(rename = "CreationProfile")]
    creation_profile: String,
    #[serde(rename = "PageData")]
    page_data: String,
    #[serde(rename = "Generators")]
    generators: String,
    #[serde(rename = "GeneratorRegistry")]
    generator_registry: String,
    #[serde(rename = "GeneratorQueue")]
    generator_queue: String
}

// The AttributeCache structure contains information about a character attributes.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AttributeCache {
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "Strength")]
    strength: String,
    #[serde(rename = "Endurance")]
    endurance: String,
    #[serde(rename = "Quickness")]
    quickness: String,
    #[serde(rename = "Coordination")]
    coordination: String,
    #[serde(rename = "Focus")]
    focus: String,
    #[serde(rename = "Self")]
    self: String,
    #[serde(rename = "Health")]
    health: String,
    #[serde(rename = "Stamina")]
    stamina: String,
    #[serde(rename = "Mana")]
    mana: String
}

// The Attribute structure contains information about a character attribute.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AttributeInfo {
    #[serde(rename = "PointsRaised")]
    points_raised: String,
    #[serde(rename = "InnatePoints")]
    innate_points: String,
    #[serde(rename = "ExperienceSpent")]
    experience_spent: String
}

// The SecondaryAttribute structure contains information about a character vital.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SecondaryAttributeInfo {
    #[serde(rename = "Attribute")]
    attribute: String,
    #[serde(rename = "Current")]
    current: String
}

// The Skill structure contains information about a character skill.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Skill {
    #[serde(rename = "PointsRaised")]
    points_raised: String,
    #[serde(rename = "AdjustPP")]
    adjust_pp: String,
    #[serde(rename = "TrainingLevel")]
    training_level: String,
    #[serde(rename = "ExperienceSpent")]
    experience_spent: String,
    #[serde(rename = "InnatePoints")]
    innate_points: String,
    #[serde(rename = "ResistanceOfLastCheck")]
    resistance_of_last_check: String,
    #[serde(rename = "LastUsedTime")]
    last_used_time: String
}

// Contains body part table
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Body {
    #[serde(rename = "BodyParts")]
    body_parts: String
}

// Information on individual body parts. (Needs to be confirmed if this was used in prod)
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BodyPart {
    #[serde(rename = "HasBPSD")]
    has_bpsd: String,
    #[serde(rename = "DamageType")]
    damage_type: String,
    #[serde(rename = "DamageVal")]
    damage_val: String,
    #[serde(rename = "DamageVar")]
    damage_var: String,
    #[serde(rename = "ArmorCache")]
    armor_cache: String,
    #[serde(rename = "BH")]
    bh: String,
    #[serde(rename = "BPSD")]
    bpsd: String
}

// Information on armor levels
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ArmorCache {
    #[serde(rename = "BaseArmor")]
    base_armor: String,
    #[serde(rename = "ArmorVsSlash")]
    armor_vs_slash: String,
    #[serde(rename = "ArmorVsPierce")]
    armor_vs_pierce: String,
    #[serde(rename = "ArmorVsBludgeon")]
    armor_vs_bludgeon: String,
    #[serde(rename = "ArmorVsCold")]
    armor_vs_cold: String,
    #[serde(rename = "ArmorVsFire")]
    armor_vs_fire: String,
    #[serde(rename = "ArmorVsAcid")]
    armor_vs_acid: String,
    #[serde(rename = "ArmorVsElectric")]
    armor_vs_electric: String,
    #[serde(rename = "ArmorVsNether")]
    armor_vs_nether: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BodyPartSelectionData {
    #[serde(rename = "HLF")]
    hlf: String,
    #[serde(rename = "MLF")]
    mlf: String,
    #[serde(rename = "LLF")]
    llf: String,
    #[serde(rename = "HRF")]
    hrf: String,
    #[serde(rename = "MRF")]
    mrf: String,
    #[serde(rename = "LRF")]
    lrf: String,
    #[serde(rename = "HLB")]
    hlb: String,
    #[serde(rename = "MLB")]
    mlb: String,
    #[serde(rename = "LLB")]
    llb: String,
    #[serde(rename = "HRB")]
    hrb: String,
    #[serde(rename = "MRB")]
    mrb: String,
    #[serde(rename = "LRB")]
    lrb: String
}

// Contains information related to the spell in your spellbook
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpellBookPage {
    #[serde(rename = "CastingLikelihood")]
    casting_likelihood: String,
    #[serde(rename = "Unknown")]
    unknown: String,
    #[serde(rename = "CastingLikelihood2")]
    casting_likelihood2: String
}

// Contains information related to the spells in effect on the character
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EnchantmentRegistry {
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "LifeSpells")]
    life_spells: String,
    #[serde(rename = "CreatureSpells")]
    creature_spells: String,
    #[serde(rename = "Vitae")]
    vitae: String,
    #[serde(rename = "Cooldowns")]
    cooldowns: String
}

// The Enchantment structure describes an active enchantment.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Enchantment {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "HasEquipmentSet")]
    has_equipment_set: String,
    #[serde(rename = "SpellCategory")]
    spell_category: String,
    #[serde(rename = "PowerLevel")]
    power_level: String,
    #[serde(rename = "StartTime")]
    start_time: String,
    #[serde(rename = "Duration")]
    duration: String,
    #[serde(rename = "CasterId")]
    caster_id: String,
    #[serde(rename = "DegradeModifier")]
    degrade_modifier: String,
    #[serde(rename = "DegradeLimit")]
    degrade_limit: String,
    #[serde(rename = "LastTimeDegraded")]
    last_time_degraded: String,
    #[serde(rename = "StatMod")]
    stat_mod: String,
    #[serde(rename = "EquipmentSet")]
    equipment_set: String
}

// Information on stat modification
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatMod {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Contains a list of events to filter? Unknown what this does currently.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EventFilter {
    #[serde(rename = "Events")]
    events: String
}

// Contains a list of emotes for NPCs? Unknown what this does currently.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EmoteTable {
    #[serde(rename = "Emotes")]
    emotes: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EmoteSetList {
    #[serde(rename = "Emotes")]
    emotes: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EmoteSet {
    #[serde(rename = "Category")]
    category: String,
    #[serde(rename = "Probability")]
    probability: String,
    #[serde(rename = "Emotes")]
    emotes: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Emote {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Delay")]
    delay: String,
    #[serde(rename = "Extent")]
    extent: String
}

// Set information about an item for creation
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreationProfile {
    #[serde(rename = "WeenieClassId")]
    weenie_class_id: String,
    #[serde(rename = "Palette")]
    palette: String,
    #[serde(rename = "Shade")]
    shade: String,
    #[serde(rename = "Destination")]
    destination: String,
    #[serde(rename = "StackSize")]
    stack_size: String,
    #[serde(rename = "TryToBond")]
    try_to_bond: String
}

// List of pages in a book
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PageDataList {
    #[serde(rename = "MaxNumPages")]
    max_num_pages: String,
    #[serde(rename = "MaxNumCharsPerPage")]
    max_num_chars_per_page: String,
    #[serde(rename = "Pages")]
    pages: String
}

// Data for an individual page
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PageData {
    #[serde(rename = "AuthorId")]
    author_id: String,
    #[serde(rename = "AuthorName")]
    author_name: String,
    #[serde(rename = "AuthorAccount")]
    author_account: String,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "TextIncluded")]
    text_included: String,
    #[serde(rename = "IgnoreAuthor")]
    ignore_author: String,
    #[serde(rename = "PageText")]
    page_text: String
}

// Blob fragment data used to contruct message data. These can be spread across multiple packets
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BlobFragments {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Count")]
    count: String,
    #[serde(rename = "Size")]
    size: String,
    #[serde(rename = "Index")]
    index: String,
    #[serde(rename = "Group")]
    group: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeneratorTable {
    #[serde(rename = "Generators")]
    generators: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeneratorProfile {
    #[serde(rename = "Probability")]
    probability: String,
    #[serde(rename = "TypeId")]
    type_id: String,
    #[serde(rename = "Delay")]
    delay: String,
    #[serde(rename = "InitCreate")]
    init_create: String,
    #[serde(rename = "MaxNum")]
    max_num: String,
    #[serde(rename = "WhenCreate")]
    when_create: String,
    #[serde(rename = "WhereCreate")]
    where_create: String,
    #[serde(rename = "StackSize")]
    stack_size: String,
    #[serde(rename = "Ptid")]
    ptid: String,
    #[serde(rename = "Shade")]
    shade: String,
    #[serde(rename = "PosVal")]
    pos_val: String,
    #[serde(rename = "Slot")]
    slot: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeneratorRegistry {
    #[serde(rename = "Registry")]
    registry: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeneratorRegistryNode {
    #[serde(rename = "WcidOrType")]
    wcid_or_type: String,
    #[serde(rename = "Ts")]
    ts: String,
    #[serde(rename = "TreasureType")]
    treasure_type: String,
    #[serde(rename = "Slot")]
    slot: String,
    #[serde(rename = "Checkpointed")]
    checkpointed: String,
    #[serde(rename = "Shop")]
    shop: String,
    #[serde(rename = "Amount")]
    amount: String
}

// Set of inventory items
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeneratorQueue {
    #[serde(rename = "Queue")]
    queue: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeneratorQueueNode {
    #[serde(rename = "Slot")]
    slot: String,
    #[serde(rename = "When")]
    when: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindowProperty {
    #[serde(rename = "Key_a")]
    key_a: String,
    #[serde(rename = "Unknown_1b")]
    unknown_1b: String,
    #[serde(rename = "Unknown_1c")]
    unknown_1c: String,
    #[serde(rename = "Unknown_d")]
    unknown_d: String,
    #[serde(rename = "Value_d")]
    value_d: String,
    #[serde(rename = "Unknown_e")]
    unknown_e: String,
    #[serde(rename = "Value_e")]
    value_e: String,
    #[serde(rename = "Unknown_f")]
    unknown_f: String,
    #[serde(rename = "Value_f")]
    value_f: String,
    #[serde(rename = "Unknown_h")]
    unknown_h: String,
    #[serde(rename = "Value_h")]
    value_h: String,
    #[serde(rename = "Unknown_i")]
    unknown_i: String,
    #[serde(rename = "Value_i")]
    value_i: String,
    #[serde(rename = "Unknown_j")]
    unknown_j: String,
    #[serde(rename = "Value_j")]
    value_j: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindowOption {
    #[serde(rename = "Type_a")]
    type_a: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OptionProperty {
    #[serde(rename = "Type")]
    type_: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GameplayOptions {
    #[serde(rename = "Size")]
    size: String,
    #[serde(rename = "Unknown200_2")]
    unknown200_2: String,
    #[serde(rename = "OptionPropertyCount")]
    option_property_count: String
}

// The PlayerModule structure contains character options.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlayerModule {
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "Options")]
    options: String,
    #[serde(rename = "Shortcuts")]
    shortcuts: String,
    #[serde(rename = "Tab1Spells")]
    tab1_spells: String,
    #[serde(rename = "Tab2Spells")]
    tab2_spells: String,
    #[serde(rename = "Tab3Spells")]
    tab3_spells: String,
    #[serde(rename = "Tab4Spells")]
    tab4_spells: String,
    #[serde(rename = "Tab5Spells")]
    tab5_spells: String,
    #[serde(rename = "Tab6Spells")]
    tab6_spells: String,
    #[serde(rename = "Tab7Spells")]
    tab7_spells: String,
    #[serde(rename = "Tab8Spells")]
    tab8_spells: String,
    #[serde(rename = "FillComps")]
    fill_comps: String,
    #[serde(rename = "SpellBookFilters")]
    spell_book_filters: String,
    #[serde(rename = "OptionFlags")]
    option_flags: String,
    #[serde(rename = "Unknown100_1")]
    unknown100_1: String,
    #[serde(rename = "OptionStrings")]
    option_strings: String,
    #[serde(rename = "GameplayOptions")]
    gameplay_options: String
}

// Set of shortcuts
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShortCutManager {
    #[serde(rename = "Shortcuts")]
    shortcuts: String
}

// Shortcut
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShortCutData {
    #[serde(rename = "Index")]
    index: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "SpellId")]
    spell_id: String
}

// List of spells in spell tab
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpellTab {
    #[serde(rename = "Spells")]
    spells: String
}

// Set of inventory items
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContentProfile {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ContainerType")]
    container_type: String
}

// Set of inventory items
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InventoryPlacement {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Location")]
    location: String,
    #[serde(rename = "Priority")]
    priority: String
}

// Allegience information
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllegianceProfile {
    #[serde(rename = "TotalMembers")]
    total_members: String,
    #[serde(rename = "TotalVassals")]
    total_vassals: String,
    #[serde(rename = "Hierarchy")]
    hierarchy: String
}

// Allegience record
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllegianceRecord {
    #[serde(rename = "TreeParent")]
    tree_parent: String,
    #[serde(rename = "AllegianceData")]
    allegiance_data: String
}

// Allegience hierarchy information
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllegianceHierarchy {
    #[serde(rename = "RecordCount")]
    record_count: String,
    #[serde(rename = "OldVersion")]
    old_version: String,
    #[serde(rename = "Officers")]
    officers: String,
    #[serde(rename = "OfficerTitles")]
    officer_titles: String,
    #[serde(rename = "MonarchBroadcastTime")]
    monarch_broadcast_time: String,
    #[serde(rename = "MonarchBroadcastsToday")]
    monarch_broadcasts_today: String,
    #[serde(rename = "SpokesBroadcastTime")]
    spokes_broadcast_time: String,
    #[serde(rename = "SpokesBroadcastsToday")]
    spokes_broadcasts_today: String,
    #[serde(rename = "Motd")]
    motd: String,
    #[serde(rename = "MotdSetBy")]
    motd_set_by: String,
    #[serde(rename = "ChatRoomId")]
    chat_room_id: String,
    #[serde(rename = "Bindpoint")]
    bindpoint: String,
    #[serde(rename = "AllegianceName")]
    allegiance_name: String,
    #[serde(rename = "NameLastSetTime")]
    name_last_set_time: String,
    #[serde(rename = "IsLocked")]
    is_locked: String,
    #[serde(rename = "ApprovedVassal")]
    approved_vassal: String,
    #[serde(rename = "MonarchData")]
    monarch_data: String
}

// Set of allegiance data for a specific player
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllegianceData {
    #[serde(rename = "CharacterId")]
    character_id: String,
    #[serde(rename = "XPCached")]
    xpcached: String,
    #[serde(rename = "XPTithed")]
    xptithed: String,
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "Gender")]
    gender: String,
    #[serde(rename = "Heritage")]
    heritage: String,
    #[serde(rename = "Rank")]
    rank: String,
    #[serde(rename = "Level")]
    level: String,
    #[serde(rename = "Loyalty")]
    loyalty: String,
    #[serde(rename = "Leadership")]
    leadership: String,
    #[serde(rename = "TimeOnline")]
    time_online: String,
    #[serde(rename = "TimeOnline")]
    time_online: String,
    #[serde(rename = "AllegianceAge")]
    allegiance_age: String,
    #[serde(rename = "Name")]
    name: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FriendData {
    #[serde(rename = "FriendId")]
    friend_id: String,
    #[serde(rename = "Online")]
    online: String,
    #[serde(rename = "AppearOffline")]
    appear_offline: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "OutFriends")]
    out_friends: String,
    #[serde(rename = "InFriends")]
    in_friends: String
}

// Data related to an item, namely the amount and description
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ItemProfile {
    #[serde(rename = "PackedAmount")]
    packed_amount: String,
    #[serde(rename = "ObjectId")]
    object_id: String
}

// The PublicWeenieDesc structure defines an object's game behavior.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PublicWeenieDesc {
    #[serde(rename = "Header")]
    header: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "WeenieClassId")]
    weenie_class_id: String,
    #[serde(rename = "Icon")]
    icon: String,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Behavior")]
    behavior: String,
    #[serde(rename = "Header2")]
    header2: String,
    #[serde(rename = "PluralName")]
    plural_name: String,
    #[serde(rename = "ItemsCapacity")]
    items_capacity: String,
    #[serde(rename = "ContainerCapacity")]
    container_capacity: String,
    #[serde(rename = "AmmunitionType")]
    ammunition_type: String,
    #[serde(rename = "Value")]
    value: String,
    #[serde(rename = "Useability")]
    useability: String,
    #[serde(rename = "UseRadius")]
    use_radius: String,
    #[serde(rename = "TargetType")]
    target_type: String,
    #[serde(rename = "Effects")]
    effects: String,
    #[serde(rename = "CombatUse")]
    combat_use: String,
    #[serde(rename = "Structure")]
    structure: String,
    #[serde(rename = "MaxStructure")]
    max_structure: String,
    #[serde(rename = "StackSize")]
    stack_size: String,
    #[serde(rename = "MaxStackSize")]
    max_stack_size: String,
    #[serde(rename = "ContainerId")]
    container_id: String,
    #[serde(rename = "WielderId")]
    wielder_id: String,
    #[serde(rename = "ValidSlots")]
    valid_slots: String,
    #[serde(rename = "Slot")]
    slot: String,
    #[serde(rename = "Priority")]
    priority: String,
    #[serde(rename = "BlipColor")]
    blip_color: String,
    #[serde(rename = "RadarEnum")]
    radar_enum: String,
    #[serde(rename = "PhysicsScript")]
    physics_script: String,
    #[serde(rename = "Workmanship")]
    workmanship: String,
    #[serde(rename = "Burden")]
    burden: String,
    #[serde(rename = "SpellId")]
    spell_id: String,
    #[serde(rename = "OwnerId")]
    owner_id: String,
    #[serde(rename = "Restrictions")]
    restrictions: String,
    #[serde(rename = "HookItemTypes")]
    hook_item_types: String,
    #[serde(rename = "MonarchId")]
    monarch_id: String,
    #[serde(rename = "HookType")]
    hook_type: String,
    #[serde(rename = "IconOverlay")]
    icon_overlay: String,
    #[serde(rename = "IconUnderlay")]
    icon_underlay: String,
    #[serde(rename = "Material")]
    material: String,
    #[serde(rename = "CooldownId")]
    cooldown_id: String,
    #[serde(rename = "CooldownDuration")]
    cooldown_duration: String,
    #[serde(rename = "PetOwnerId")]
    pet_owner_id: String
}

// The RestrictionDB contains the access control list for a dwelling object.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RestrictionDB {
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "MonarchId")]
    monarch_id: String,
    #[serde(rename = "Permissions")]
    permissions: String
}

// The OldPublicWeenieDesc structure defines an object's game behavior.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OldPublicWeenieDesc {
    #[serde(rename = "Header")]
    header: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "WeenieClassId")]
    weenie_class_id: String,
    #[serde(rename = "Icon")]
    icon: String,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Bitfield")]
    bitfield: String,
    #[serde(rename = "PluralName")]
    plural_name: String,
    #[serde(rename = "ItemsCapacity")]
    items_capacity: String,
    #[serde(rename = "ContainerCapacity")]
    container_capacity: String,
    #[serde(rename = "Value")]
    value: String,
    #[serde(rename = "Useability")]
    useability: String,
    #[serde(rename = "UseRadius")]
    use_radius: String,
    #[serde(rename = "tTargetType")]
    t_target_type: String,
    #[serde(rename = "Effects")]
    effects: String,
    #[serde(rename = "AmmunitionType")]
    ammunition_type: String,
    #[serde(rename = "CombatUse")]
    combat_use: String,
    #[serde(rename = "Structure")]
    structure: String,
    #[serde(rename = "MaxStructure")]
    max_structure: String,
    #[serde(rename = "StackSize")]
    stack_size: String,
    #[serde(rename = "MaxStackSize")]
    max_stack_size: String,
    #[serde(rename = "ContainerId")]
    container_id: String,
    #[serde(rename = "WielderId")]
    wielder_id: String,
    #[serde(rename = "ValidSlots")]
    valid_slots: String,
    #[serde(rename = "Slots")]
    slots: String,
    #[serde(rename = "Priority")]
    priority: String,
    #[serde(rename = "BlipColor")]
    blip_color: String,
    #[serde(rename = "RadarEnum")]
    radar_enum: String,
    #[serde(rename = "ObviousDistance")]
    obvious_distance: String,
    #[serde(rename = "Vndwcid")]
    vndwcid: String,
    #[serde(rename = "SpellId")]
    spell_id: String,
    #[serde(rename = "HouseOwnerId")]
    house_owner_id: String,
    #[serde(rename = "PhysicsScript")]
    physics_script: String,
    #[serde(rename = "Restrictions")]
    restrictions: String,
    #[serde(rename = "HookType")]
    hook_type: String,
    #[serde(rename = "HookItemTypes")]
    hook_item_types: String,
    #[serde(rename = "MonarchId")]
    monarch_id: String,
    #[serde(rename = "IconOverlay")]
    icon_overlay: String,
    #[serde(rename = "Material")]
    material: String
}

// Information related to a secure trade.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade {
    #[serde(rename = "PartnerId")]
    partner_id: String,
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "InitiatorId")]
    initiator_id: String,
    #[serde(rename = "Accepted")]
    accepted: String,
    #[serde(rename = "PartnerAccepted")]
    partner_accepted: String
}

// A jump with sequences
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JumpPack {
    #[serde(rename = "Extent")]
    extent: String,
    #[serde(rename = "Velocity")]
    velocity: String,
    #[serde(rename = "ObjectInstanceSequence")]
    object_instance_sequence: String,
    #[serde(rename = "ObjectServerControlSequence")]
    object_server_control_sequence: String,
    #[serde(rename = "ObjectTeleportSequence")]
    object_teleport_sequence: String,
    #[serde(rename = "ObjectForcePositionSequence")]
    object_force_position_sequence: String
}

// A set of data related to changing states with sequences
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MoveToStatePack {
    #[serde(rename = "RawMotionState")]
    raw_motion_state: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "ObjectInstanceSequence")]
    object_instance_sequence: String,
    #[serde(rename = "ObjectServerControlSequence")]
    object_server_control_sequence: String,
    #[serde(rename = "ObjectTeleportSequence")]
    object_teleport_sequence: String,
    #[serde(rename = "ObjectForcePositionSequence")]
    object_force_position_sequence: String,
    #[serde(rename = "Contact")]
    contact: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PackedMotionCommand {
    #[serde(rename = "CommandId")]
    command_id: String,
    #[serde(rename = "PackedSequence")]
    packed_sequence: String,
    #[serde(rename = "Speed")]
    speed: String
}

// Data related to the movement of the object sent from a client
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RawMotionState {
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "CurrentHoldkey")]
    current_holdkey: String,
    #[serde(rename = "CurrentStyle")]
    current_style: String,
    #[serde(rename = "ForwardCommand")]
    forward_command: String,
    #[serde(rename = "ForwardHoldkey")]
    forward_holdkey: String,
    #[serde(rename = "ForwardSpeed")]
    forward_speed: String,
    #[serde(rename = "SidestepCommand")]
    sidestep_command: String,
    #[serde(rename = "SidestepHoldkey")]
    sidestep_holdkey: String,
    #[serde(rename = "SidestepSpeed")]
    sidestep_speed: String,
    #[serde(rename = "TurnCommand")]
    turn_command: String,
    #[serde(rename = "TurnHoldkey")]
    turn_holdkey: String,
    #[serde(rename = "TurnSpeed")]
    turn_speed: String
}

// An autonomous position with sequences
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AutonomousPositionPack {
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "ObjectInstanceSequence")]
    object_instance_sequence: String,
    #[serde(rename = "ObjectServerControlSequence")]
    object_server_control_sequence: String,
    #[serde(rename = "ObjectTeleportSequence")]
    object_teleport_sequence: String,
    #[serde(rename = "ObjectForcePositionSequence")]
    object_force_position_sequence: String,
    #[serde(rename = "Contact")]
    contact: String
}

// A position with sequences
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PositionPack {
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "Origin")]
    origin: String,
    #[serde(rename = "WQuat")]
    wquat: String,
    #[serde(rename = "XQuat")]
    xquat: String,
    #[serde(rename = "YQuat")]
    yquat: String,
    #[serde(rename = "ZQuat")]
    zquat: String,
    #[serde(rename = "Velocity")]
    velocity: String,
    #[serde(rename = "PlacementId")]
    placement_id: String,
    #[serde(rename = "ObjectInstanceSequence")]
    object_instance_sequence: String,
    #[serde(rename = "ObjectPositionSequence")]
    object_position_sequence: String,
    #[serde(rename = "ObjectTeleportSequence")]
    object_teleport_sequence: String,
    #[serde(rename = "ObjectForcePositionSequence")]
    object_force_position_sequence: String
}

// Data related to the movement and animation of the object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MovementData {
    #[serde(rename = "ObjectMovementSequence")]
    object_movement_sequence: String,
    #[serde(rename = "ObjectServerControlSequence")]
    object_server_control_sequence: String,
    #[serde(rename = "Autonomous")]
    autonomous: String,
    #[serde(rename = "MovementType")]
    movement_type: String,
    #[serde(rename = "OptionFlags")]
    option_flags: String,
    #[serde(rename = "Stance")]
    stance: String
}

// Contains information for animations and general free motion
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InterpertedMotionState {
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "CurrentStyle")]
    current_style: String,
    #[serde(rename = "ForwardCommand")]
    forward_command: String,
    #[serde(rename = "SidestepCommand")]
    sidestep_command: String,
    #[serde(rename = "TurnCommand")]
    turn_command: String,
    #[serde(rename = "ForwardSpeed")]
    forward_speed: String,
    #[serde(rename = "SidestepSpeed")]
    sidestep_speed: String,
    #[serde(rename = "TurnSpeed")]
    turn_speed: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DDDRevision {
    #[serde(rename = "IdDatFile")]
    id_dat_file: String,
    #[serde(rename = "Iteration")]
    iteration: String,
    #[serde(rename = "IdsToDownload")]
    ids_to_download: String,
    #[serde(rename = "IdsToPurge")]
    ids_to_purge: String
}

// Set of movement parameters required for a MoveTo movement
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MoveToMovementParameters {
    #[serde(rename = "Bitmember")]
    bitmember: String,
    #[serde(rename = "DistanceToObject")]
    distance_to_object: String,
    #[serde(rename = "MinDistance")]
    min_distance: String,
    #[serde(rename = "FailDistance")]
    fail_distance: String,
    #[serde(rename = "AnimationSpeed")]
    animation_speed: String,
    #[serde(rename = "WalkRunThreshold")]
    walk_run_threshold: String,
    #[serde(rename = "DesiredHeading")]
    desired_heading: String
}

// Set of movement parameters required for a TurnTo motion
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TurnToMovementParameters {
    #[serde(rename = "Bitmember")]
    bitmember: String,
    #[serde(rename = "AnimationSpeed")]
    animation_speed: String,
    #[serde(rename = "DesiredHeading")]
    desired_heading: String
}

// The ObjDesc structure defines an object's visual appearance.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ObjDesc {
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "PaletteCount")]
    palette_count: String,
    #[serde(rename = "TextureCount")]
    texture_count: String,
    #[serde(rename = "ModelCount")]
    model_count: String,
    #[serde(rename = "Palette")]
    palette: String
}

// Contains data for a subpalette
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Subpalette {
    #[serde(rename = "Palette")]
    palette: String,
    #[serde(rename = "Offset")]
    offset: String,
    #[serde(rename = "NumColors")]
    num_colors: String
}

// Contains data for texture map changes
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextureMapChange {
    #[serde(rename = "PartIndex")]
    part_index: String,
    #[serde(rename = "OldTexId")]
    old_tex_id: String,
    #[serde(rename = "NewTexId")]
    new_tex_id: String
}

// Contains data for animation part changes
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnimPartChange {
    #[serde(rename = "PartIndex")]
    part_index: String,
    #[serde(rename = "PartId")]
    part_id: String
}

// Data for a character creation
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CharGenResult {
    #[serde(rename = "Account")]
    account: String,
    #[serde(rename = "One")]
    one: String,
    #[serde(rename = "HeritageGroup")]
    heritage_group: String,
    #[serde(rename = "Gender")]
    gender: String,
    #[serde(rename = "EyesStrip")]
    eyes_strip: String,
    #[serde(rename = "NoseStrip")]
    nose_strip: String,
    #[serde(rename = "MouthStrip")]
    mouth_strip: String,
    #[serde(rename = "HairColor")]
    hair_color: String,
    #[serde(rename = "EyeColor")]
    eye_color: String,
    #[serde(rename = "HairStyle")]
    hair_style: String,
    #[serde(rename = "HeadgearStyle")]
    headgear_style: String,
    #[serde(rename = "HeadgearColor")]
    headgear_color: String,
    #[serde(rename = "ShirtStyle")]
    shirt_style: String,
    #[serde(rename = "ShirtColor")]
    shirt_color: String,
    #[serde(rename = "TrousersStyle")]
    trousers_style: String,
    #[serde(rename = "TrousersColor")]
    trousers_color: String,
    #[serde(rename = "FootwearStyle")]
    footwear_style: String,
    #[serde(rename = "FootwearColor")]
    footwear_color: String,
    #[serde(rename = "SkinShade")]
    skin_shade: String,
    #[serde(rename = "HairShade")]
    hair_shade: String,
    #[serde(rename = "HeadgearShade")]
    headgear_shade: String,
    #[serde(rename = "ShirtShade")]
    shirt_shade: String,
    #[serde(rename = "TrousersShade")]
    trousers_shade: String,
    #[serde(rename = "TootwearShade")]
    tootwear_shade: String,
    #[serde(rename = "TemplateNum")]
    template_num: String,
    #[serde(rename = "Strength")]
    strength: String,
    #[serde(rename = "Endurance")]
    endurance: String,
    #[serde(rename = "Coordination")]
    coordination: String,
    #[serde(rename = "Quickness")]
    quickness: String,
    #[serde(rename = "Focus")]
    focus: String,
    #[serde(rename = "Self")]
    self: String,
    #[serde(rename = "Slot")]
    slot: String,
    #[serde(rename = "ClassId")]
    class_id: String,
    #[serde(rename = "Skills")]
    skills: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "StartArea")]
    start_area: String,
    #[serde(rename = "IsAdmin")]
    is_admin: String,
    #[serde(rename = "IsEnvoy")]
    is_envoy: String,
    #[serde(rename = "Validation")]
    validation: String
}

// Basic information for a character used at the Login screen
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CharacterIdentity {
    #[serde(rename = "CharacterId")]
    character_id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "SecondsGreyedOut")]
    seconds_greyed_out: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EquipLocation {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Slot")]
    slot: String
}

// The PhysicsDesc structure defines an object's physical behavior.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PhysicsDesc {
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "State")]
    state: String,
    #[serde(rename = "MovementBuffer")]
    movement_buffer: String,
    #[serde(rename = "Autonomous")]
    autonomous: String,
    #[serde(rename = "AnimationFrame")]
    animation_frame: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "MotionId")]
    motion_id: String,
    #[serde(rename = "SoundId")]
    sound_id: String,
    #[serde(rename = "PhysicsScriptId")]
    physics_script_id: String,
    #[serde(rename = "SetupId")]
    setup_id: String,
    #[serde(rename = "ParentId")]
    parent_id: String,
    #[serde(rename = "ParentLocation")]
    parent_location: String,
    #[serde(rename = "Children")]
    children: String,
    #[serde(rename = "Scale")]
    scale: String,
    #[serde(rename = "Friction")]
    friction: String,
    #[serde(rename = "Elasticity")]
    elasticity: String,
    #[serde(rename = "Translucency")]
    translucency: String,
    #[serde(rename = "Velocity")]
    velocity: String,
    #[serde(rename = "Acceleration")]
    acceleration: String,
    #[serde(rename = "Omega")]
    omega: String,
    #[serde(rename = "DefaultScript")]
    default_script: String,
    #[serde(rename = "DefaultScriptIntensity")]
    default_script_intensity: String,
    #[serde(rename = "ObjectPositionSequence")]
    object_position_sequence: String,
    #[serde(rename = "ObjectMovementSequence")]
    object_movement_sequence: String,
    #[serde(rename = "ObjectStateSequence")]
    object_state_sequence: String,
    #[serde(rename = "ObjectVectorSequence")]
    object_vector_sequence: String,
    #[serde(rename = "ObjectTeleportSequence")]
    object_teleport_sequence: String,
    #[serde(rename = "ObjectServerControlSequence")]
    object_server_control_sequence: String,
    #[serde(rename = "ObjectForcePositionSequence")]
    object_force_position_sequence: String,
    #[serde(rename = "ObjectVisualDescSequence")]
    object_visual_desc_sequence: String,
    #[serde(rename = "ObjectInstanceSequence")]
    object_instance_sequence: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminAccountData {
    #[serde(rename = "AccountName")]
    account_name: String,
    #[serde(rename = "BookieId")]
    bookie_id: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdminPlayerData {
    name: String,
    #[serde(rename = "bookieId")]
    bookie_id: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VendorProfile {
    #[serde(rename = "Categories")]
    categories: String,
    #[serde(rename = "MinValue")]
    min_value: String,
    #[serde(rename = "MaxValue")]
    max_value: String,
    #[serde(rename = "DealsMagic")]
    deals_magic: String,
    #[serde(rename = "BuyPrice")]
    buy_price: String,
    #[serde(rename = "SellPrice")]
    sell_price: String,
    #[serde(rename = "CurrencyId")]
    currency_id: String,
    #[serde(rename = "CurrencyAmount")]
    currency_amount: String,
    #[serde(rename = "CurrencyName")]
    currency_name: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ArmorProfile {
    #[serde(rename = "ProtSlashing")]
    prot_slashing: String,
    #[serde(rename = "ProtPiercing")]
    prot_piercing: String,
    #[serde(rename = "ProtBludgeoning")]
    prot_bludgeoning: String,
    #[serde(rename = "ProtCold")]
    prot_cold: String,
    #[serde(rename = "ProtFire")]
    prot_fire: String,
    #[serde(rename = "ProtAcid")]
    prot_acid: String,
    #[serde(rename = "ProtNether")]
    prot_nether: String,
    #[serde(rename = "ProtLightning")]
    prot_lightning: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatureAppraisalProfile {
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "Health")]
    health: String,
    #[serde(rename = "HealthMax")]
    health_max: String,
    #[serde(rename = "Strength")]
    strength: String,
    #[serde(rename = "Endurance")]
    endurance: String,
    #[serde(rename = "Quickness")]
    quickness: String,
    #[serde(rename = "Coordination")]
    coordination: String,
    #[serde(rename = "Focus")]
    focus: String,
    #[serde(rename = "Self")]
    self: String,
    #[serde(rename = "Stamina")]
    stamina: String,
    #[serde(rename = "Mana")]
    mana: String,
    #[serde(rename = "StaminaMax")]
    stamina_max: String,
    #[serde(rename = "ManaMax")]
    mana_max: String,
    #[serde(rename = "AttrHighlight")]
    attr_highlight: String,
    #[serde(rename = "AttrColor")]
    attr_color: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WeaponProfile {
    #[serde(rename = "DamageType")]
    damage_type: String,
    #[serde(rename = "Speed")]
    speed: String,
    #[serde(rename = "Skill")]
    skill: String,
    #[serde(rename = "Damage")]
    damage: String,
    #[serde(rename = "Variance")]
    variance: String,
    #[serde(rename = "Modifier")]
    modifier: String,
    #[serde(rename = "Length")]
    length: String,
    #[serde(rename = "MaxVelocity")]
    max_velocity: String,
    #[serde(rename = "Offsense")]
    offsense: String,
    #[serde(rename = "MaxVelocityEstimated")]
    max_velocity_estimated: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HookAppraisalProfile {
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "ValidLocations")]
    valid_locations: String,
    #[serde(rename = "AmmoType")]
    ammo_type: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SquelchDB {
    #[serde(rename = "AccountHash")]
    account_hash: String,
    #[serde(rename = "CharacterHash")]
    character_hash: String,
    #[serde(rename = "GlobalInfo")]
    global_info: String
}

// Set of information related to a squelch entry
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SquelchInfo {
    #[serde(rename = "Filters")]
    filters: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Account")]
    account: String
}

// Set of information related to purchasing a housing
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HouseProfile {
    #[serde(rename = "DwellingId")]
    dwelling_id: String,
    #[serde(rename = "OwnerId")]
    owner_id: String,
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "MinLevel")]
    min_level: String,
    #[serde(rename = "MaxLevel")]
    max_level: String,
    #[serde(rename = "MinAllegRank")]
    min_alleg_rank: String,
    #[serde(rename = "MaxAllegRank")]
    max_alleg_rank: String,
    #[serde(rename = "MaintenanceFree")]
    maintenance_free: String,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "OwnerName")]
    owner_name: String,
    #[serde(rename = "Buy")]
    buy: String,
    #[serde(rename = "Rent")]
    rent: String
}

// The HousePayment structure contains information about a house purchase or maintenance item.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HousePayment {
    #[serde(rename = "Required")]
    required: String,
    #[serde(rename = "Paid")]
    paid: String,
    #[serde(rename = "WeenieClassId")]
    weenie_class_id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "PluralName")]
    plural_name: String
}

// Set of information related to owning a housing
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HouseData {
    #[serde(rename = "BuyTime")]
    buy_time: String,
    #[serde(rename = "RentTime")]
    rent_time: String,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "MaintenanceFree")]
    maintenance_free: String,
    #[serde(rename = "Buy")]
    buy: String,
    #[serde(rename = "Rent")]
    rent: String,
    #[serde(rename = "Position")]
    position: String
}

// Set of information related to house access
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HAR {
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Bitmask")]
    bitmask: String,
    #[serde(rename = "MonarchId")]
    monarch_id: String,
    #[serde(rename = "GuestList")]
    guest_list: String,
    #[serde(rename = "RoommateList")]
    roommate_list: String
}

// Set of information related to a house guest
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuestInfo {
    #[serde(rename = "HasStoragePermission")]
    has_storage_permission: String,
    #[serde(rename = "GuestName")]
    guest_name: String
}

// Set of information related to a chess game move
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GameMoveData {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "PlayerId")]
    player_id: String,
    #[serde(rename = "Team")]
    team: String
}

// Set of information related to a salvage operation
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SalvageOperationsResultData {
    #[serde(rename = "SkillUsed")]
    skill_used: String,
    #[serde(rename = "NotSalvagable")]
    not_salvagable: String,
    #[serde(rename = "SalvageResults")]
    salvage_results: String,
    #[serde(rename = "AugBonus")]
    aug_bonus: String
}

// Set of information related to a salvage of an item
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SalvageResult {
    #[serde(rename = "Material")]
    material: String,
    #[serde(rename = "Workmanship")]
    workmanship: String,
    #[serde(rename = "Units")]
    units: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FellowshipLockData {
    #[serde(rename = "Unknown1")]
    unknown1: String,
    #[serde(rename = "Unknown2")]
    unknown2: String,
    #[serde(rename = "Unknown3")]
    unknown3: String,
    #[serde(rename = "Timestamp")]
    timestamp: String,
    #[serde(rename = "Sequence")]
    sequence: String
}

// Set of information for a fellowship
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship {
    #[serde(rename = "Members")]
    members: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "LeaderId")]
    leader_id: String,
    #[serde(rename = "ShareXP")]
    share_xp: String,
    #[serde(rename = "EvenXPSplit")]
    even_xpsplit: String,
    #[serde(rename = "Open")]
    open: String,
    #[serde(rename = "Locked")]
    locked: String,
    #[serde(rename = "RecentlyDeparted")]
    recently_departed: String,
    #[serde(rename = "Locks")]
    locks: String
}

// The FellowInfo structure contains information about a fellowship member.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellow {
    #[serde(rename = "XPCached")]
    xpcached: String,
    #[serde(rename = "LumCached")]
    lum_cached: String,
    #[serde(rename = "Level")]
    level: String,
    #[serde(rename = "MaxHealth")]
    max_health: String,
    #[serde(rename = "MaxStamina")]
    max_stamina: String,
    #[serde(rename = "MaxMana")]
    max_mana: String,
    #[serde(rename = "CurrentHealth")]
    current_health: String,
    #[serde(rename = "CurrentStamina")]
    current_stamina: String,
    #[serde(rename = "CurrentMana")]
    current_mana: String,
    #[serde(rename = "ShareLoot")]
    share_loot: String,
    #[serde(rename = "Name")]
    name: String
}

// Contains information about a contract.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContractTracker {
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "ContractId")]
    contract_id: String,
    #[serde(rename = "ContractStage")]
    contract_stage: String,
    #[serde(rename = "TimeWhenDone")]
    time_when_done: String,
    #[serde(rename = "TimeWhenRepeats")]
    time_when_repeats: String
}

// Contains table of ContractTrackers
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContractTrackerTable {
    #[serde(rename = "ContactTrackers")]
    contact_trackers: String
}

// Set a single character option.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_PlayerOptionChangedEvent {
    #[serde(rename = "Option")]
    option: String,
    #[serde(rename = "Value")]
    value: String
}

// Starts a melee attack against a target
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_TargetedMeleeAttack {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Height")]
    height: String,
    #[serde(rename = "Power")]
    power: String
}

// Starts a missle attack against a target
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_TargetedMissileAttack {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Height")]
    height: String,
    #[serde(rename = "Accuracy")]
    accuracy: String
}

// Set AFK mode.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_SetAFKMode {
    #[serde(rename = "AFK")]
    afk: String
}

// Set AFK message.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_SetAFKMessage {
    #[serde(rename = "Message")]
    message: String
}

// Talking
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_Talk {
    #[serde(rename = "Message")]
    message: String
}

// Removes a friend
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Social_RemoveFriend {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Adds a friend
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Social_AddFriend {
    #[serde(rename = "CharacterName")]
    character_name: String
}

// Store an item in a container.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_PutItemInContainer {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ContainerId")]
    container_id: String,
    #[serde(rename = "SlotIndex")]
    slot_index: String
}

// Gets and weilds an item.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_GetAndWieldItem {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Slot")]
    slot: String
}

// Drop an item.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_DropItem {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Swear allegiance
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_SwearAllegiance {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Break allegiance
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_BreakAllegiance {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Allegiance update request
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_UpdateRequest {
    #[serde(rename = "On")]
    on: String
}

// Sets a character's display title
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Social_SetDisplayCharacterTitle {
    #[serde(rename = "TitleId")]
    title_id: String
}

// Direct message by Id
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_TalkDirect {
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "TargetId")]
    target_id: String
}

// Sets the allegiance name
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_SetAllegianceName {
    #[serde(rename = "Name")]
    name: String
}

// Attempt to use an item with a target.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_UseWithTargetEvent {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "TargetId")]
    target_id: String
}

// Attempt to use an item.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_UseEvent {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Sets an allegiance officer
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_SetAllegianceOfficer {
    #[serde(rename = "CharacterName")]
    character_name: String,
    #[serde(rename = "Level")]
    level: String
}

// Sets an allegiance officer title for a given level
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_SetAllegianceOfficerTitle {
    #[serde(rename = "Level")]
    level: String,
    #[serde(rename = "Title")]
    title: String
}

// Perform the allegiance lock action
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_DoAllegianceLockAction {
    #[serde(rename = "Action")]
    action: String
}

// Sets a person as an approved vassal
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_SetAllegianceApprovedVassal {
    #[serde(rename = "CharacterName")]
    character_name: String
}

// Gags a person in allegiance chat
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_AllegianceChatGag {
    #[serde(rename = "CharacterName")]
    character_name: String,
    #[serde(rename = "On")]
    on: String
}

// Perform the allegiance house action
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_DoAllegianceHouseAction {
    #[serde(rename = "Action")]
    action: String
}

// Spend XP to raise a vital.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Train_TrainAttribute2nd {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Experience")]
    experience: String
}

// Spend XP to raise an attribute.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Train_TrainAttribute {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Experience")]
    experience: String
}

// Spend XP to raise a skill.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Train_TrainSkill {
    #[serde(rename = "Skill")]
    skill: String,
    #[serde(rename = "Experience")]
    experience: String
}

// Spend skill credits to train a skill.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Train_TrainSkillAdvancementClass {
    #[serde(rename = "Skill")]
    skill: String,
    #[serde(rename = "Credits")]
    credits: String
}

// Cast a spell with no target.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Magic_CastUntargetedSpell {
    #[serde(rename = "SpellId")]
    spell_id: String
}

// Cast a spell on a target
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Magic_CastTargetedSpell {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "SpellId")]
    spell_id: String
}

// Changes the combat mode
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_ChangeCombatMode {
    #[serde(rename = "Mode")]
    mode: String
}

// Merges one stack with another
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_StackableMerge {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "TargetId")]
    target_id: String,
    #[serde(rename = "Amount")]
    amount: String
}

// Split a stack and place it into a container
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_StackableSplitToContainer {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ContainerId")]
    container_id: String,
    #[serde(rename = "SlotIndex")]
    slot_index: String,
    #[serde(rename = "Amount")]
    amount: String
}

// Split a stack and place it into the world
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_StackableSplitTo3D {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Amount")]
    amount: String
}

// Changes an account squelch
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_ModifyCharacterSquelch {
    #[serde(rename = "Add")]
    add: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "CharacterName")]
    character_name: String,
    #[serde(rename = "Type")]
    type_: String
}

// Changes an account squelch
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_ModifyAccountSquelch {
    #[serde(rename = "Add")]
    add: String,
    #[serde(rename = "CharacterName")]
    character_name: String
}

// Changes the global filters, /filter -type as well as /chat and /notell
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_ModifyGlobalSquelch {
    #[serde(rename = "Add")]
    add: String,
    #[serde(rename = "Type")]
    type_: String
}

// Direct message by name
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_TalkDirectByName {
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "TargetName")]
    target_name: String
}

// Buy from a vendor
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vendor_Buy {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Items")]
    items: String,
    #[serde(rename = "AlternateCurrencyId")]
    alternate_currency_id: String
}

// Sell to a vendor
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vendor_Sell {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Items")]
    items: String
}

// Create a fellowship
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship_Create {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ShareXP")]
    share_xp: String
}

// Quit the fellowship
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship_Quit {
    #[serde(rename = "Disband")]
    disband: String
}

// Dismiss a player from the fellowship
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship_Dismiss {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Recruit a player to the fellowship
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship_Recruit {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Update request
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship_UpdateRequest {
    #[serde(rename = "On")]
    on: String
}

// Request update to book data (seems to be sent after failed add page)
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Writing_BookAddPage {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Updates a page in a book
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Writing_BookModifyPage {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "PageNum")]
    page_num: String,
    #[serde(rename = "PageText")]
    page_text: String
}

// Add a page to a book
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Writing_BookData {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Removes a page from a book
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Writing_BookDeletePage {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "PageNum")]
    page_num: String
}

// Requests data for a page in a book
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Writing_BookPageData {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "PageNum")]
    page_num: String
}

// Sets the inscription on an object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Writing_SetInscription {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Inscription")]
    inscription: String
}

// Appraise something
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_Appraise {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Give an item to someone.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_GiveObjectRequest {
    #[serde(rename = "TargetId")]
    target_id: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Amount")]
    amount: String
}

// Advocate Teleport
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Advocate_Teleport {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Destination")]
    destination: String
}

// Sends an abuse report.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_AbuseLogRequest {
    #[serde(rename = "Character")]
    character: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Complaint")]
    complaint: String
}

// Joins a chat channel
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_AddToChannel {
    #[serde(rename = "Channel")]
    channel: String
}

// Leaves a chat channel
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_RemoveFromChannel {
    #[serde(rename = "Channel")]
    channel: String
}

// Sends a message to a chat channel
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_ChannelBroadcast {
    #[serde(rename = "Channel")]
    channel: String,
    #[serde(rename = "SenderName")]
    sender_name: String,
    #[serde(rename = "Message")]
    message: String
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_ChannelList {
    #[serde(rename = "Channel")]
    channel: String
}

// Stop viewing the contents of a container
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_NoLongerViewingContents {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Splits an item to a wield location.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_StackableSplitToWield {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Slot")]
    slot: String,
    #[serde(rename = "Amount")]
    amount: String
}

// Add an item to the shortcut bar.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_AddShortCut {
    #[serde(rename = "Shortcut")]
    shortcut: String
}

// Remove an item from the shortcut bar.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_RemoveShortCut {
    #[serde(rename = "Index")]
    index: String
}

// Set multiple character options.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_CharacterOptionsEvent {
    #[serde(rename = "Options")]
    options: String
}

// Removes a spell from the spell book
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Magic_RemoveSpell {
    #[serde(rename = "SpellId")]
    spell_id: String
}

// Query's a creatures health
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_QueryHealth {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Query a character's age
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_QueryAge {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Query a character's birth day
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_QueryBirth {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Emote message
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_Emote {
    #[serde(rename = "Message")]
    message: String
}

// Soul emote message
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_SoulEmote {
    #[serde(rename = "Message")]
    message: String
}

// Add a spell to a spell bar.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_AddSpellFavorite {
    #[serde(rename = "SpellId")]
    spell_id: String,
    #[serde(rename = "Index")]
    index: String,
    #[serde(rename = "SpellBar")]
    spell_bar: String
}

// Remove a spell from a spell bar.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_RemoveSpellFavorite {
    #[serde(rename = "SpellId")]
    spell_id: String,
    #[serde(rename = "SpellBar")]
    spell_bar: String
}

// Starts trading with another player.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_OpenTradeNegotiations {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Adds an object to the trade.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_AddToTrade {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "SlotIndex")]
    slot_index: String
}

// Accepts a trade.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_AcceptTrade {
    #[serde(rename = "Contents")]
    contents: String
}

// Remove your corpse looting permission for the given player, /consent remove 
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_RemoveFromPlayerConsentList {
    #[serde(rename = "TargetName")]
    target_name: String
}

// Grants a player corpse looting permission, /permit add
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_AddPlayerPermission {
    #[serde(rename = "TargetName")]
    target_name: String
}

// Buy a house
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_BuyHouse {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Items")]
    items: String
}

// Revokes a player's corpse looting permission, /permit remove
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_RemovePlayerPermission {
    #[serde(rename = "TargetName")]
    target_name: String
}

// Pay rent for a house
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_RentHouse {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Items")]
    items: String
}

// Sets a new fill complevel for a component
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_SetDesiredComponentLevel {
    #[serde(rename = "Wcid")]
    wcid: String,
    #[serde(rename = "Amount")]
    amount: String
}

// Adds a guest to your house guest list
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_AddPermanentGuest {
    #[serde(rename = "GuestName")]
    guest_name: String
}

// Removes a specific player from your house guest list, /house guest remove
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_RemovePermanentGuest {
    #[serde(rename = "GuestName")]
    guest_name: String
}

// Sets your house open status
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_SetOpenHouseStatus {
    #[serde(rename = "OpenHouse")]
    open_house: String
}

// Changes a specific players storage permission, /house storage add/remove
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_ChangeStoragePermission {
    #[serde(rename = "GuestName")]
    guest_name: String,
    #[serde(rename = "HasPermission")]
    has_permission: String
}

// Boots a specific player from your house /house boot
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_BootSpecificHouseGuest {
    #[serde(rename = "GuestName")]
    guest_name: String
}

// Sets the allegiance message of the day, /allegiance motd set
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_SetMotd {
    #[serde(rename = "Message")]
    message: String
}

// Gets SlumLord info, sent after getting a failed house transaction
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_QueryLord {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Queries an item's mana
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_QueryItemMana {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Modify whether house hooks are visibile or not, /house hooks on/off
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_SetHooksVisibility {
    #[serde(rename = "Visible")]
    visible: String
}

// Modify whether allegiance members are guests, /house guest add_allegiance/remove_allegiance
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_ModifyAllegianceGuestPermission {
    #[serde(rename = "Add")]
    add: String
}

// Modify whether allegiance members can access storage, /house storage add_allegiance/remove_allegiance
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_ModifyAllegianceStoragePermission {
    #[serde(rename = "Add")]
    add: String
}

// Joins a chess game
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game_Join {
    #[serde(rename = "GameId")]
    game_id: String,
    #[serde(rename = "Team")]
    team: String
}

// Makes a chess move
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game_Move {
    #[serde(rename = "XFrom")]
    xfrom: String,
    #[serde(rename = "YFrom")]
    yfrom: String,
    #[serde(rename = "XTo")]
    xto: String,
    #[serde(rename = "YTo")]
    yto: String
}

// Offer or confirm stalemate
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game_Stalemate {
    #[serde(rename = "On")]
    on: String
}

// Lists available house /house available
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_ListAvailableHouses {
    #[serde(rename = "Type")]
    type_: String
}

// Confirms a dialog
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_ConfirmationResponse {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Context")]
    context: String,
    #[serde(rename = "Accepted")]
    accepted: String
}

// Boots a player from the allegiance, optionally all characters on their account
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_BreakAllegianceBoot {
    #[serde(rename = "BooteeName")]
    bootee_name: String,
    #[serde(rename = "AccountBoot")]
    account_boot: String
}

// Request allegiance info for a player
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_AllegianceInfoRequest {
    #[serde(rename = "TargetName")]
    target_name: String
}

// Salvages items
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_CreateTinkeringTool {
    #[serde(rename = "ToolId")]
    tool_id: String,
    #[serde(rename = "Items")]
    items: String
}

// Changes the spell book filter
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_SpellbookFilterEvent {
    #[serde(rename = "Options")]
    options: String
}

// Fellowship Assign a new leader
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship_AssignNewLeader {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Fellowship Change openness
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship_ChangeFellowOpeness {
    #[serde(rename = "Open")]
    open: String
}

// Boots a player from the allegiance chat
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_AllegianceChatBoot {
    #[serde(rename = "CharacterName")]
    character_name: String,
    #[serde(rename = "Reason")]
    reason: String
}

// Bans a player from the allegiance
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_AddAllegianceBan {
    #[serde(rename = "CharacterName")]
    character_name: String
}

// Removes a player ban from the allegiance
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_RemoveAllegianceBan {
    #[serde(rename = "CharacterName")]
    character_name: String
}

// Removes an allegiance officer
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_RemoveAllegianceOfficer {
    #[serde(rename = "CharacterName")]
    character_name: String
}

// Admin Returns a plugin list response
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Admin_QueryPluginListResponse {
    #[serde(rename = "Context")]
    context: String,
    #[serde(rename = "PluginList")]
    plugin_list: String
}

// Admin Returns plugin info
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Admin_QueryPluginResponse {
    #[serde(rename = "Context")]
    context: String,
    #[serde(rename = "Success")]
    success: String,
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
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_FinishBarber {
    #[serde(rename = "BasePalette")]
    base_palette: String,
    #[serde(rename = "HeadObject")]
    head_object: String,
    #[serde(rename = "HeadTexture")]
    head_texture: String,
    #[serde(rename = "DefaultHeadTexture")]
    default_head_texture: String,
    #[serde(rename = "EyesTexture")]
    eyes_texture: String,
    #[serde(rename = "DefaultEyesTexture")]
    default_eyes_texture: String,
    #[serde(rename = "NoseTexture")]
    nose_texture: String,
    #[serde(rename = "DefaultNoseTexture")]
    default_nose_texture: String,
    #[serde(rename = "MouthTexture")]
    mouth_texture: String,
    #[serde(rename = "DefaultMouthTexture")]
    default_mouth_texture: String,
    #[serde(rename = "SkinPalette")]
    skin_palette: String,
    #[serde(rename = "HairPalette")]
    hair_palette: String,
    #[serde(rename = "EyesPalette")]
    eyes_palette: String,
    #[serde(rename = "SetupId")]
    setup_id: String,
    #[serde(rename = "Option1")]
    option1: String,
    #[serde(rename = "Option2")]
    option2: String
}

// Abandons a contract
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Social_AbandonContract {
    #[serde(rename = "ContractId")]
    contract_id: String
}

// Performs a jump
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Movement_Jump {
    #[serde(rename = "Jump")]
    jump: String
}

// Move to state data
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Movement_MoveToState {
    #[serde(rename = "MoveToState")]
    move_to_state: String
}

// Performs a movement based on input
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Movement_DoMovementCommand {
    #[serde(rename = "Motion")]
    motion: String,
    #[serde(rename = "Speed")]
    speed: String,
    #[serde(rename = "HoldKey")]
    hold_key: String
}

// Stops a movement
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Movement_StopMovementCommand {
    #[serde(rename = "Motion")]
    motion: String,
    #[serde(rename = "HoldKey")]
    hold_key: String
}

// Sets an autonomy level
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Movement_AutonomyLevel {
    #[serde(rename = "AutonomyLevel")]
    autonomy_level: String
}

// Sends an autonomous position
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Movement_AutonomousPosition {
    #[serde(rename = "Position")]
    position: String
}

// Performs a non autonomous jump
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Movement_Jump_NonAutonomous {
    #[serde(rename = "Extent")]
    extent: String
}

// Allegiance update cancelled
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_AllegianceUpdateAborted {
    #[serde(rename = "FailureType")]
    failure_type: String
}

// Display a message in a popup message window.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_PopUpString {
    #[serde(rename = "Message")]
    message: String
}

// Information describing your character.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Login_PlayerDescription {
    #[serde(rename = "BaseQualities")]
    base_qualities: String,
    #[serde(rename = "Qualities")]
    qualities: String,
    #[serde(rename = "PlayerModule")]
    player_module: String,
    #[serde(rename = "ContentProfile")]
    content_profile: String,
    #[serde(rename = "InventoryPlacement")]
    inventory_placement: String
}

// Returns info related to your monarch, patron and vassals.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_AllegianceUpdate {
    #[serde(rename = "Rank")]
    rank: String,
    #[serde(rename = "Profile")]
    profile: String
}

// Friends list update
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Social_FriendsUpdate {
    #[serde(rename = "Friends")]
    friends: String,
    #[serde(rename = "Type")]
    type_: String
}

// Store an item in a container.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_ServerSaysContainId {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ContainerId")]
    container_id: String,
    #[serde(rename = "SlotIndex")]
    slot_index: String,
    #[serde(rename = "ContainerType")]
    container_type: String
}

// Equip an item.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_WearItem {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Slot")]
    slot: String
}

// Titles for the current character.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Social_CharacterTitleTable {
    #[serde(rename = "DisplayTitle")]
    display_title: String,
    #[serde(rename = "Titles")]
    titles: String
}

// Set a title for the current character.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Social_AddOrSetCharacterTitle {
    #[serde(rename = "NewTitle")]
    new_title: String,
    #[serde(rename = "SetAsDisplayTitle")]
    set_as_display_title: String
}

// Close Container - Only sent when explicitly closed
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_StopViewingObjectContents {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Open the buy/sell panel for a merchant.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vendor_VendorInfo {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Profile")]
    profile: String,
    #[serde(rename = "Items")]
    items: String
}

// Opens barber UI
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_StartBarber {
    #[serde(rename = "BasePalette")]
    base_palette: String,
    #[serde(rename = "HeadObject")]
    head_object: String,
    #[serde(rename = "HeadTexture")]
    head_texture: String,
    #[serde(rename = "DefaultHeadTexture")]
    default_head_texture: String,
    #[serde(rename = "EyesTexture")]
    eyes_texture: String,
    #[serde(rename = "DefaultEyesTexture")]
    default_eyes_texture: String,
    #[serde(rename = "NoseTexture")]
    nose_texture: String,
    #[serde(rename = "DefaultNoseTexture")]
    default_nose_texture: String,
    #[serde(rename = "MouthTexture")]
    mouth_texture: String,
    #[serde(rename = "DefaultMouthTexture")]
    default_mouth_texture: String,
    #[serde(rename = "SkinPalette")]
    skin_palette: String,
    #[serde(rename = "HairPalette")]
    hair_palette: String,
    #[serde(rename = "EyesPalette")]
    eyes_palette: String,
    #[serde(rename = "SetupId")]
    setup_id: String,
    #[serde(rename = "Option1")]
    option1: String,
    #[serde(rename = "Option2")]
    option2: String
}

// Member left fellowship
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship_Quit {
    #[serde(rename = "Disband")]
    disband: String
}

// Member dismissed from fellowship
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship_Dismiss {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Sent when you first open a book, contains the entire table of contents.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Writing_BookOpen {
    #[serde(rename = "BookId")]
    book_id: String,
    #[serde(rename = "MaxNumPages")]
    max_num_pages: String,
    #[serde(rename = "PageData")]
    page_data: String,
    #[serde(rename = "Inscription")]
    inscription: String,
    #[serde(rename = "ScribeId")]
    scribe_id: String,
    #[serde(rename = "ScribeName")]
    scribe_name: String
}

// Response to an attempt to add a page to a book.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Writing_BookAddPageResponse {
    #[serde(rename = "BookId")]
    book_id: String,
    #[serde(rename = "PageNumber")]
    page_number: String,
    #[serde(rename = "Success")]
    success: String
}

// Response to an attempt to delete a page from a book.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Writing_BookDeletePageResponse {
    #[serde(rename = "BookId")]
    book_id: String,
    #[serde(rename = "PageNumber")]
    page_number: String,
    #[serde(rename = "Success")]
    success: String
}

// Contains the text of a single page of a book, parchment or sign.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Writing_BookPageDataResponse {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Page")]
    page: String,
    #[serde(rename = "PageData")]
    page_data: String
}

// Get Inscription Response, doesn't seem to be really used by client
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_GetInscriptionResponse {
    #[serde(rename = "Inscription")]
    inscription: String,
    #[serde(rename = "ScribeName")]
    scribe_name: String,
    #[serde(rename = "ScribeAccount")]
    scribe_account: String
}

// The result of an attempt to assess an item or creature.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_SetAppraiseInfo {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "Success")]
    success: String,
    #[serde(rename = "IntProperties")]
    int_properties: String,
    #[serde(rename = "Int64Properties")]
    int64_properties: String,
    #[serde(rename = "BoolProperties")]
    bool_properties: String,
    #[serde(rename = "FloatProperties")]
    float_properties: String,
    #[serde(rename = "StringProperties")]
    string_properties: String,
    #[serde(rename = "DataIdProperties")]
    data_id_properties: String,
    #[serde(rename = "SpellBook")]
    spell_book: String,
    #[serde(rename = "ArmorProfile")]
    armor_profile: String,
    #[serde(rename = "CreatureProfile")]
    creature_profile: String,
    #[serde(rename = "WeaponProfile")]
    weapon_profile: String,
    #[serde(rename = "HookProfile")]
    hook_profile: String,
    #[serde(rename = "ArmorHighlight")]
    armor_highlight: String,
    #[serde(rename = "ArmorColor")]
    armor_color: String,
    #[serde(rename = "WeaponHighlight")]
    weapon_highlight: String,
    #[serde(rename = "WeaponColor")]
    weapon_color: String,
    #[serde(rename = "ResistHighlight")]
    resist_highlight: String,
    #[serde(rename = "ResistColor")]
    resist_color: String,
    #[serde(rename = "BaseArmorHead")]
    base_armor_head: String,
    #[serde(rename = "BaseArmorChest")]
    base_armor_chest: String,
    #[serde(rename = "BaseArmorGroin")]
    base_armor_groin: String,
    #[serde(rename = "BaseArmorBicep")]
    base_armor_bicep: String,
    #[serde(rename = "BaseArmorWrist")]
    base_armor_wrist: String,
    #[serde(rename = "BaseArmorHand")]
    base_armor_hand: String,
    #[serde(rename = "BaseArmorThigh")]
    base_armor_thigh: String,
    #[serde(rename = "BaseArmorShin")]
    base_armor_shin: String,
    #[serde(rename = "BaseArmorFoot")]
    base_armor_foot: String
}

// ChannelBroadcast: Group Chat
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_ChannelBroadcast {
    #[serde(rename = "Channel")]
    channel: String,
    #[serde(rename = "Message")]
    message: String
}

// ChannelList: Provides list of characters listening to a channel, I assume in response to a command
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_ChannelList {
    #[serde(rename = "Characters")]
    characters: String
}

// ChannelIndex: Provides list of channels available to the player, I assume in response to a command
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_ChannelIndex {
    #[serde(rename = "Channels")]
    channels: String
}

// Set Pack Contents
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_OnViewContents {
    #[serde(rename = "ContainerId")]
    container_id: String,
    #[serde(rename = "Items")]
    items: String
}

// ServerSaysMoveItem: Removes an item from inventory (when you place it on the ground or give it away)
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_ServerSaysMoveItem {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// HandleAttackDoneEvent: Melee attack completed
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_HandleAttackDoneEvent {
    #[serde(rename = "Number")]
    number: String
}

// RemoveSpell: Delete a spell from your spellbook.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Magic_RemoveSpell {
    #[serde(rename = "SpellId")]
    spell_id: String
}

// You just died.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_HandleVictimNotificationEventSelf {
    #[serde(rename = "Message")]
    message: String
}

// Message for a death, something you killed or your own death message.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_HandleVictimNotificationEventOther {
    #[serde(rename = "Message")]
    message: String
}

// HandleAttackerNotificationEvent: You have hit your target with a melee attack.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_HandleAttackerNotificationEvent {
    #[serde(rename = "DefenderName")]
    defender_name: String,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "DamagePercent")]
    damage_percent: String,
    #[serde(rename = "Damage")]
    damage: String,
    #[serde(rename = "Critical")]
    critical: String,
    #[serde(rename = "AttackConditions")]
    attack_conditions: String
}

// HandleDefenderNotificationEvent: You have been hit by a creature's melee attack.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_HandleDefenderNotificationEvent {
    #[serde(rename = "AttackerName")]
    attacker_name: String,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "DamagePercent")]
    damage_percent: String,
    #[serde(rename = "Damage")]
    damage: String,
    #[serde(rename = "Location")]
    location: String,
    #[serde(rename = "Critical")]
    critical: String,
    #[serde(rename = "AttackConditions")]
    attack_conditions: String
}

// HandleEvasionAttackerNotificationEvent: Your target has evaded your melee attack.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_HandleEvasionAttackerNotificationEvent {
    #[serde(rename = "DefenderName")]
    defender_name: String
}

// HandleEvasionDefenderNotificationEvent: You have evaded a creature's melee attack.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_HandleEvasionDefenderNotificationEvent {
    #[serde(rename = "AttackerName")]
    attacker_name: String
}

// QueryHealthResponse: Update a creature's health bar.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_QueryHealthResponse {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Health")]
    health: String
}

// QueryAgeResponse: happens when you do /age in the game
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_QueryAgeResponse {
    #[serde(rename = "TargetName")]
    target_name: String,
    #[serde(rename = "Age")]
    age: String
}

// UseDone: Ready. Previous action complete
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_UseDone {
    #[serde(rename = "FailureType")]
    failure_type: String
}

// Allegiance update finished
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_AllegianceUpdateDone {
    #[serde(rename = "FailureType")]
    failure_type: String
}

// Close Assess Panel
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_AppraiseDone {
    #[serde(rename = "Unknown")]
    unknown: String
}

// Squelch and Filter List
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_SetSquelchDB {
    #[serde(rename = "SquelchDB")]
    squelch_db: String
}

// RegisterTrade: Send to begin a trade and display the trade window
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_RegisterTrade {
    #[serde(rename = "InitiatorId")]
    initiator_id: String,
    #[serde(rename = "PartnerId")]
    partner_id: String,
    #[serde(rename = "Stamp")]
    stamp: String
}

// OpenTrade: Open trade window
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_OpenTrade {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// CloseTrade: End trading
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_CloseTrade {
    #[serde(rename = "Reason")]
    reason: String
}

// RemoveFromTrade: Item was removed from trade window
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_AddToTrade {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Side")]
    side: String
}

// Removes an item from the trade window, not sure if this is used still?
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_RemoveFromTrade {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Side")]
    side: String
}

// AcceptTrade: The trade was accepted
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_AcceptTrade {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// DeclineTrade: The trade was declined
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_DeclineTrade {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// ResetTrade: The trade window was reset
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_ResetTrade {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// TradeFailure: Failure to add a trade item
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade_TradeFailure {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Reason")]
    reason: String
}

// Buy a dwelling or pay maintenance
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_HouseProfile {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Profile")]
    profile: String
}

// House panel information for owners.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_HouseData {
    #[serde(rename = "Data")]
    data: String
}

// House Data
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_HouseStatus {
    #[serde(rename = "NoticeType")]
    notice_type: String
}

// Update Rent Time
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_UpdateRentTime {
    #[serde(rename = "RentTime")]
    rent_time: String
}

// Update Rent Payment
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_UpdateRentPayment {
    #[serde(rename = "Rent")]
    rent: String
}

// Update Restrictions
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_UpdateRestrictions {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "SenderId")]
    sender_id: String,
    #[serde(rename = "Restrictions")]
    restrictions: String
}

// House Guest List
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_UpdateHAR {
    #[serde(rename = "GuestList")]
    guest_list: String
}

// House Profile
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_HouseTransaction {
    #[serde(rename = "NoticeType")]
    notice_type: String
}

// Update an item's mana bar.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_QueryItemManaResponse {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Mana")]
    mana: String,
    #[serde(rename = "Success")]
    success: String
}

// Display a list of available dwellings in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct House_AvailableHouses {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Houses")]
    houses: String,
    #[serde(rename = "NumHouses")]
    num_houses: String
}

// Display a confirmation panel.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_ConfirmationRequest {
    #[serde(rename = "ConfirmationType")]
    confirmation_type: String,
    #[serde(rename = "ContextId")]
    context_id: String,
    #[serde(rename = "Text")]
    text: String
}

// Confirmation done
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_ConfirmationDone {
    #[serde(rename = "ConfirmationType")]
    confirmation_type: String,
    #[serde(rename = "ContextId")]
    context_id: String
}

// Display an allegiance login/logout message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_AllegianceLoginNotificationEvent {
    #[serde(rename = "CharacterId")]
    character_id: String,
    #[serde(rename = "IsLoggedIn")]
    is_logged_in: String
}

// Returns data for a player's allegiance information
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Allegiance_AllegianceInfoResponseEvent {
    #[serde(rename = "TargetId")]
    target_id: String,
    #[serde(rename = "Profile")]
    profile: String
}

// Joining game response
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game_JoinGameResponse {
    #[serde(rename = "GameId")]
    game_id: String,
    #[serde(rename = "Team")]
    team: String
}

// Start game
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game_StartGame {
    #[serde(rename = "GameId")]
    game_id: String,
    #[serde(rename = "Team")]
    team: String
}

// Move response
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game_MoveResponse {
    #[serde(rename = "GameId")]
    game_id: String,
    #[serde(rename = "MoveResult")]
    move_result: String
}

// Opponent Turn
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game_OpponentTurn {
    #[serde(rename = "GameId")]
    game_id: String,
    #[serde(rename = "Team")]
    team: String,
    #[serde(rename = "GameMove")]
    game_move: String
}

// Opponent Stalemate State
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game_OpponentStalemateState {
    #[serde(rename = "GameId")]
    game_id: String,
    #[serde(rename = "Team")]
    team: String,
    #[serde(rename = "On")]
    on: String
}

// Display a status message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_WeenieError {
    #[serde(rename = "Type")]
    type_: String
}

// Display a parameterized status message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_WeenieErrorWithString {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Text")]
    text: String
}

// End of Chess game
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game_GameOver {
    #[serde(rename = "GameId")]
    game_id: String,
    #[serde(rename = "TeamWinner")]
    team_winner: String
}

// Set Turbine Chat channel numbers.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_ChatRoomTracker {
    #[serde(rename = "AllegianceRoomId")]
    allegiance_room_id: String,
    #[serde(rename = "GeneralChatRoomId")]
    general_chat_room_id: String,
    #[serde(rename = "TradeChatRoomId")]
    trade_chat_room_id: String,
    #[serde(rename = "LFGChatRoomId")]
    lfgchat_room_id: String,
    #[serde(rename = "RoleplayChatRoomId")]
    roleplay_chat_room_id: String,
    #[serde(rename = "OlthoiChatRoomId")]
    olthoi_chat_room_id: String,
    #[serde(rename = "SocietyChatRoomId")]
    society_chat_room_id: String,
    #[serde(rename = "SocietyCelestialHandChatRoomId")]
    society_celestial_hand_chat_room_id: String,
    #[serde(rename = "SocietyEldrichWebChatRoomId")]
    society_eldrich_web_chat_room_id: String,
    #[serde(rename = "SocietyRadiantBloodChatRoomId")]
    society_radiant_blood_chat_room_id: String
}

// Salvage operation results
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_SalvageOperationsResultData {
    #[serde(rename = "Result")]
    result: String
}

// Someone has sent you a @tell.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_HearDirectSpeech {
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "SenderName")]
    sender_name: String,
    #[serde(rename = "SenderId")]
    sender_id: String,
    #[serde(rename = "TargetId")]
    target_id: String,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "SecretFlags")]
    secret_flags: String
}

// Create or join a fellowship
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship_FullUpdate {
    #[serde(rename = "Fellowship")]
    fellowship: String
}

// Add/Update a member to your fellowship.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fellowship_UpdateFellow {
    #[serde(rename = "Fellow")]
    fellow: String,
    #[serde(rename = "UpdateType")]
    update_type: String
}

// Add a spell to your spellbook.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Magic_UpdateSpell {
    #[serde(rename = "SpellId")]
    spell_id: String
}

// Apply an enchantment to your character.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Magic_UpdateEnchantment {
    #[serde(rename = "Enchantment")]
    enchantment: String
}

// Remove an enchantment from your character.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Magic_RemoveEnchantment {
    #[serde(rename = "SpellId")]
    spell_id: String
}

// Update multiple enchantments from your character.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Magic_UpdateMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    enchantments: String
}

// Remove multiple enchantments from your character.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Magic_RemoveMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    enchantments: String
}

// Silently remove An enchantment from your character.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Magic_DispelEnchantment {
    #[serde(rename = "SpellId")]
    spell_id: String
}

// Silently remove multiple enchantments from your character (no message in the chat window).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Magic_DispelMultipleEnchantments {
    #[serde(rename = "Enchantments")]
    enchantments: String
}

// A portal storm is brewing.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Misc_PortalStormBrewing {
    #[serde(rename = "Extent")]
    extent: String
}

// A portal storm is imminent.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Misc_PortalStormImminent {
    #[serde(rename = "Extent")]
    extent: String
}

// Display a status message on the Action Viewscreen (the red text overlaid on the 3D area).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_TransientString {
    #[serde(rename = "Message")]
    message: String
}

// Sends all contract data
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Social_SendClientContractTrackerTable {
    #[serde(rename = "ContractTracker")]
    contract_tracker: String
}

// Updates a contract data
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Social_SendClientContractTracker {
    #[serde(rename = "ContractTracker")]
    contract_tracker: String,
    #[serde(rename = "DeleteContract")]
    delete_contract: String,
    #[serde(rename = "SetAsDisplayContract")]
    set_as_display_contract: String
}

// Mark a character for deletetion.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_CharacterDelete {
    #[serde(rename = "Account")]
    account: String,
    #[serde(rename = "Slot")]
    slot: String
}

// Character creation result
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_SendCharGenResult {
    #[serde(rename = "Account")]
    account: String,
    #[serde(rename = "Result")]
    result: String
}

// The character to log in.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Login_SendEnterWorld {
    #[serde(rename = "CharacterId")]
    character_id: String,
    #[serde(rename = "Account")]
    account: String
}

// Asks server for a new object description
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Object_SendForceObjdesc {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Seems to be a legacy friends command, /friends old, for when Jan 2006 event changed the friends list
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Social_SendFriendsCommand {
    #[serde(rename = "Command")]
    command: String,
    #[serde(rename = "Player")]
    player: String
}

// Admin command to restore a character
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Admin_SendAdminRestoreCharacter {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "RestoredCharName")]
    restored_char_name: String,
    #[serde(rename = "AccountToRestoreTo")]
    account_to_restore_to: String
}

// Send or receive a message using Turbine Chat.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_TurbineChat {
    #[serde(rename = "MmessageSize")]
    mmessage_size: String,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "BlobDispatchType")]
    blob_dispatch_type: String,
    #[serde(rename = "TargetType")]
    target_type: String,
    #[serde(rename = "TargetId")]
    target_id: String,
    #[serde(rename = "TransportType")]
    transport_type: String,
    #[serde(rename = "TransportId")]
    transport_id: String,
    #[serde(rename = "Cookie")]
    cookie: String,
    #[serde(rename = "PayloadSize")]
    payload_size: String
}

// DDD request for data
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DDD_RequestDataMessage {
    #[serde(rename = "ResourceType")]
    resource_type: String,
    #[serde(rename = "ResourceId")]
    resource_id: String
}

// TODO
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DDD_InterrogationResponseMessage {
    #[serde(rename = "Language")]
    language: String,
    #[serde(rename = "Files")]
    files: String
}

// Sent every time an object you are aware of ceases to exist. Merely running out of range does not generate this message - in that case, the client just automatically stops tracking it after receiving no updates for a while (which I presume is a very short while).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_ServerSaysRemove {
    #[serde(rename = "ObjectId")]
    object_id: String
}

// Failure to give an item
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_ServerSaysAttemptFailed {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Reason")]
    reason: String
}

// For stackable items, this changes the number of items in the stack.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_UpdateStackSize {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Amount")]
    amount: String,
    #[serde(rename = "NewValue")]
    new_value: String
}

// A Player Kill occurred nearby (also sent for suicides).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Combat_HandlePlayerDeathEvent {
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "KilledId")]
    killed_id: String,
    #[serde(rename = "KillerId")]
    killer_id: String
}

// Remove an int property from the character
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateRemoveIntEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove an int property from an object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_RemoveIntEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove a bool property from the charactert
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateRemoveBoolEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove a bool property from an object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_RemoveBoolEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove a float property from the character
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateRemoveFloatEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove a float property from an object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_RemoveFloatEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove a string property from the character
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateRemoveStringEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove a string property from an object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_RemoveStringEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove an dataId property from the character
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateRemoveDataIdEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove an dataId property from an object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_RemoveDataIdEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove an instanceId property from the character
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateRemoveInstanceIdEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove an instanceId property from an object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_RemoveInstanceIdEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove a position property from the character
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateRemovePositionEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove a position property from an object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_RemovePositionEvent {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove an int64 property from the character
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateRemoveInt64Event {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Type")]
    type_: String
}

// Remove an int64 property from an object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_RemoveInt64Event {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Type")]
    type_: String
}

// Set or update a Character Int property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateInt {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update an Object Int property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateInt {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Int64 property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateInt64 {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Int64 property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateInt64 {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Boolean property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateBool {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update an Object Boolean property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateBool {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update an Object float property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateFloat {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update an Object float property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateFloat {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update an Object String property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateString {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update an Object String property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateString {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update an Object DId property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateDataId {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update an Object DId property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateDataId {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a IId property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateInstanceId {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update an Object IId property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateInstanceId {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Position property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdatePosition {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Position property value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdatePosition {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Skill value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateSkill {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Skill value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateSkill {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Skill Level
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateSkillLevel {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Skill Level
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateSkillLevel {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Skill state
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateSkillAC {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Skill state
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateSkillAC {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Attribute value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateAttribute {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Attribute value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateAttribute {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Attribute Level
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateAttributeLevel {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Attribute Level
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateAttributeLevel {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Vital value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateAttribute2nd {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Vital value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateAttribute2nd {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Vital value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_PrivateUpdateAttribute2ndLevel {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Set or update a Character Vital value
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualities_UpdateAttribute2ndLevel {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String
}

// Indirect '/e' text.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_HearEmote {
    #[serde(rename = "SenderId")]
    sender_id: String,
    #[serde(rename = "SenderName")]
    sender_name: String,
    #[serde(rename = "Text")]
    text: String
}

// Contains the text associated with an emote action.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_HearSoulEmote {
    #[serde(rename = "SenderId")]
    sender_id: String,
    #[serde(rename = "SenderName")]
    sender_name: String,
    #[serde(rename = "Text")]
    text: String
}

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_HearSpeech {
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "SenderName")]
    sender_name: String,
    #[serde(rename = "SenderId")]
    sender_id: String,
    #[serde(rename = "Type")]
    type_: String
}

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_HearRangedSpeech {
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "SenderName")]
    sender_name: String,
    #[serde(rename = "SenderId")]
    sender_id: String,
    #[serde(rename = "Range")]
    range: String,
    #[serde(rename = "Type")]
    type_: String
}

// This appears to be an admin command to change the environment (light, fog, sounds, colors)
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Admin_Environs {
    #[serde(rename = "EnvrionOption")]
    envrion_option: String
}

// Sets both the position and movement, such as when materializing at a lifestone
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Movement_PositionAndMovementEvent {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "MovementData")]
    movement_data: String
}

// Sent whenever a character changes their clothes. It contains the entire description of what their wearing (and possibly their facial features as well). This message is only sent for changes, when the character is first created, the body of this message is included inside the creation message.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_ObjDescEvent {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ObjectDescription")]
    object_description: String,
    #[serde(rename = "InstanceSequence")]
    instance_sequence: String,
    #[serde(rename = "VisualDescSequence")]
    visual_desc_sequence: String
}

// Sets the player visual desc, TODO confirm this
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_SetPlayerVisualDesc {
    #[serde(rename = "ObjectDescription")]
    object_description: String
}

// Character creation screen initilised.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_CharGenVerificationResponse {
    #[serde(rename = "ResponseType")]
    response_type: String
}

// Sent when your subsciption is about to expire
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Login_AwaitingSubscriptionExpiration {
    #[serde(rename = "SecondsRemaining")]
    seconds_remaining: String
}

// The list of characters on the current account.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Login_LoginCharacterSet {
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Characters")]
    characters: String,
    #[serde(rename = "DeletedCharacters")]
    deleted_characters: String,
    #[serde(rename = "NumAllowedCharacters")]
    num_allowed_characters: String,
    #[serde(rename = "Account")]
    account: String,
    #[serde(rename = "UseTurbineChat")]
    use_turbine_chat: String,
    #[serde(rename = "HasThroneofDestiny")]
    has_throneof_destiny: String
}

// Failure to log in
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character_CharacterError {
    #[serde(rename = "Reason")]
    reason: String
}

// Create an object somewhere in the world
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_CreateObject {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ObjectDescription")]
    object_description: String,
    #[serde(rename = "PhysicsDescription")]
    physics_description: String,
    #[serde(rename = "WeenieDescription")]
    weenie_description: String
}

// Login of player
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Login_CreatePlayer {
    #[serde(rename = "CharacterId")]
    character_id: String
}

// Sent whenever an object is being deleted from the scene.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_DeleteObject {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ObjectInstanceSequence")]
    object_instance_sequence: String
}

// Sets the position/motion of an object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Movement_PositionEvent {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Position")]
    position: String
}

// Sets the parent for an object, eg. equipting an object.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_ParentEvent {
    #[serde(rename = "ParentId")]
    parent_id: String,
    #[serde(rename = "ChildId")]
    child_id: String,
    #[serde(rename = "Location")]
    location: String,
    #[serde(rename = "Placement")]
    placement: String,
    #[serde(rename = "ObjectInstanceSequence")]
    object_instance_sequence: String,
    #[serde(rename = "ChildPositionSequence")]
    child_position_sequence: String
}

// Sent when picking up an object
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory_PickupEvent {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ObjectInstanceSequence")]
    object_instance_sequence: String,
    #[serde(rename = "ObjectPositionSequence")]
    object_position_sequence: String
}

// Set's the current state of the object. Client appears to only process the following state changes post creation: NoDraw, LightingOn, Hidden
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_SetState {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "NewState")]
    new_state: String,
    #[serde(rename = "ObjectInstanceSequence")]
    object_instance_sequence: String,
    #[serde(rename = "ObjectStateSequence")]
    object_state_sequence: String
}

// These are animations. Whenever a human, monster or object moves - one of these little messages is sent. Even idle emotes (like head scratching and nodding) are sent in this manner.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Movement_SetObjectMovement {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ObjectInstanceSequence")]
    object_instance_sequence: String,
    #[serde(rename = "MovementData")]
    movement_data: String
}

// Changes an objects vector, for things like jumping
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Movement_VectorUpdate {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "Velocity")]
    velocity: String,
    #[serde(rename = "Omega")]
    omega: String,
    #[serde(rename = "ObjectInstanceSequence")]
    object_instance_sequence: String,
    #[serde(rename = "ObjectVectorSequence")]
    object_vector_sequence: String
}

// Applies a sound effect.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Effects_SoundEvent {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "SoundType")]
    sound_type: String,
    #[serde(rename = "Volume")]
    volume: String
}

// Instructs the client to show the portal graphic.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Effects_PlayerTeleport {
    #[serde(rename = "ObjectTeleportSequence")]
    object_teleport_sequence: String
}

// Instructs the client to play a script. (Not seen so far, maybe prefered PlayScriptType)
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Effects_PlayScriptId {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ScriptId")]
    script_id: String
}

// Applies an effect with visual and sound by providing the type to be looked up in the Physics Script Table
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Effects_PlayScriptType {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ScriptType")]
    script_type: String,
    #[serde(rename = "Speed")]
    speed: String
}

// Account has been banned
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Login_AccountBanned {
    #[serde(rename = "BannedUntil")]
    banned_until: String,
    #[serde(rename = "Text")]
    text: String
}

// Admin Receive Account Data
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Admin_ReceiveAccountData {
    #[serde(rename = "Unknown")]
    unknown: String,
    #[serde(rename = "AdminAccountData")]
    admin_account_data: String
}

// Admin Receive Player Data
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Admin_ReceivePlayerData {
    #[serde(rename = "Unknown")]
    unknown: String,
    #[serde(rename = "AdminPlayerData")]
    admin_player_data: String
}

// Update an existing object's data.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item_UpdateObject {
    #[serde(rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "ObjectDesc")]
    object_desc: String,
    #[serde(rename = "PhysicsDesc")]
    physics_desc: String,
    #[serde(rename = "WeenieDesc")]
    weenie_desc: String
}

// Account has been booted
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Login_AccountBooted {
    #[serde(rename = "AdditionalReasonText")]
    additional_reason_text: String,
    #[serde(rename = "ReasonText")]
    reason_text: String
}

// Send or receive a message using Turbine Chat.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_TurbineChat {
    #[serde(rename = "MessageSize")]
    message_size: String,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "BlobDispatchType")]
    blob_dispatch_type: String,
    #[serde(rename = "TargetType")]
    target_type: String,
    #[serde(rename = "TargetId")]
    target_id: String,
    #[serde(rename = "TransportType")]
    transport_type: String,
    #[serde(rename = "TransportId")]
    transport_id: String,
    #[serde(rename = "Cookie")]
    cookie: String,
    #[serde(rename = "PayloadSize")]
    payload_size: String
}

// Display a message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Communication_TextboxString {
    #[serde(rename = "Text")]
    text: String,
    #[serde(rename = "Type")]
    type_: String
}

// The name of the current world.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Login_WorldInfo {
    #[serde(rename = "Connections")]
    connections: String,
    #[serde(rename = "MaxConnections")]
    max_connections: String,
    #[serde(rename = "WorldName")]
    world_name: String
}

// Add or update a dat file Resource.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DDD_DataMessage {
    #[serde(rename = "DatFile")]
    dat_file: String,
    #[serde(rename = "ResourceType")]
    resource_type: String,
    #[serde(rename = "ResourceId")]
    resource_id: String,
    #[serde(rename = "Iteration")]
    iteration: String,
    #[serde(rename = "Compression")]
    compression: String,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "DataSize")]
    data_size: String
}

// DDD error
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DDD_ErrorMessage {
    #[serde(rename = "ResourceType")]
    resource_type: String,
    #[serde(rename = "ResourceId")]
    resource_id: String,
    #[serde(rename = "RError")]
    rerror: String
}

// A list of dat files that need to be patched
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DDD_BeginDDDMessage {
    #[serde(rename = "DataExpected")]
    data_expected: String,
    #[serde(rename = "Revisions")]
    revisions: String
}

// Add or update a dat file Resource.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DDD_InterrogationMessage {
    #[serde(rename = "ServersRegion")]
    servers_region: String,
    #[serde(rename = "NameRuleLanguage")]
    name_rule_language: String,
    #[serde(rename = "ProductId")]
    product_id: String,
    #[serde(rename = "SupportedLanguages")]
    supported_languages: String
}

// Client to Server AC packet.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct C2SPacket {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "Checksum")]
    checksum: String,
    #[serde(rename = "RecipientId")]
    recipient_id: String,
    #[serde(rename = "TimeSinceLastPacket")]
    time_since_last_packet: String,
    #[serde(rename = "Size")]
    size: String,
    #[serde(rename = "Iteration")]
    iteration: String,
    #[serde(rename = "ServerSwitch")]
    server_switch: String,
    #[serde(rename = "RetransmitSequences")]
    retransmit_sequences: String,
    #[serde(rename = "RejectSequences")]
    reject_sequences: String,
    #[serde(rename = "AckSequence")]
    ack_sequence: String,
    #[serde(rename = "LoginRequest")]
    login_request: String,
    #[serde(rename = "WorldLoginRequest")]
    world_login_request: String,
    #[serde(rename = "ConnectResponse")]
    connect_response: String,
    #[serde(rename = "CICMDCommand")]
    cicmdcommand: String,
    #[serde(rename = "Time")]
    time: String,
    #[serde(rename = "EchoTime")]
    echo_time: String,
    #[serde(rename = "Flow")]
    flow: String,
    #[serde(rename = "Fragments")]
    fragments: String
}

// Server to Client AC packet.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct S2CPacket {
    #[serde(rename = "Sequence")]
    sequence: String,
    #[serde(rename = "Flags")]
    flags: String,
    #[serde(rename = "Checksum")]
    checksum: String,
    #[serde(rename = "RecipientId")]
    recipient_id: String,
    #[serde(rename = "TimeSinceLastPacket")]
    time_since_last_packet: String,
    #[serde(rename = "Size")]
    size: String,
    #[serde(rename = "Iteration")]
    iteration: String,
    #[serde(rename = "AckSequence")]
    ack_sequence: String,
    #[serde(rename = "LogonServerAddr")]
    logon_server_addr: String,
    #[serde(rename = "Referral")]
    referral: String,
    #[serde(rename = "ConnectRequest")]
    connect_request: String,
    #[serde(rename = "NetError")]
    net_error: String,
    #[serde(rename = "NetErrorDisconnect")]
    net_error_disconnect: String,
    #[serde(rename = "EchoResponse")]
    echo_response: String,
    #[serde(rename = "Fragments")]
    fragments: String
}

