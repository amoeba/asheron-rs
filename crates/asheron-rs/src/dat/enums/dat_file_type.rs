use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash, EnumIter)]
#[repr(u32)]
pub enum DatFileType {
    Unknown = 0,
    LandBlock = 1,
    LandBlockInfo = 2,
    EnvCell = 3,
    LandBlockObjects = 4,
    Instantiation = 5,
    GraphicsObject = 6,
    Setup = 7,
    Animation = 8,
    AnimationHook = 9,
    Palette = 10,
    SurfaceTexture = 11,
    Texture = 12,
    Surface = 13,
    MotionTable = 14,
    Wave = 15,
    Environment = 16,
    ChatPoseTable = 17,
    ObjectHierarchy = 18,
    BadData = 19,
    TabooTable = 20,
    FileToId = 21,
    NameFilterTable = 22,
    MonitoredProperties = 23,
    PaletteSet = 24,
    Clothing = 25,
    DegradeInfo = 26,
    Scene = 27,
    Region = 28,
    KeyMap = 29,
    RenderTexture = 30,
    RenderMaterial = 31,
    MaterialModifier = 32,
    MaterialInstance = 33,
    SoundTable = 34,
    UiLayout = 35,
    EnumMapper = 36,
    StringTable = 37,
    DidMapper = 38,
    ActionMap = 39,
    DualDidMapper = 40,
    String = 41,
    ParticleEmitter = 42,
    PhysicsScript = 43,
    PhysicsScriptTable = 44,
    MasterProperty = 45,
    Font = 46,
    FontLocal = 47,
    StringState = 48,
    DbProperties = 49,
    RenderMesh = 67,
    CharGen = 68,
    WeenieDefaults = 0x10000001,
    CharacterGenerator = 0x10000002,
    SecondaryAttributeTable = 0x10000003,
    SkillTable = 0x10000004,
    SpellTable = 0x10000005,
    SpellComponentTable = 0x10000006,
    TreasureTable = 0x10000007,
    CraftTable = 0x10000008,
    XpTable = 0x10000009,
    Quests = 0x1000000A,
    GameEventTable = 0x1000000B,
    QualityFilter = 0x1000000C,
    CombatTable = 0x1000000D,
    ItemMutation = 0x1000000E,
    ContractTable = 0x10000010,
}

impl DatFileType {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn from_u32(value: u32) -> Option<Self> {
        Self::iter().find(|variant| variant.as_u32() == value)
    }

    pub fn from_object_id(object_id: u32) -> Self {
        match object_id {
            0x01000000..=0x0100FFFF => DatFileType::GraphicsObject,
            0x02000000..=0x0200FFFF => DatFileType::Setup,
            0x03000000..=0x0300FFFF => DatFileType::Animation,
            0x04000000..=0x0400FFFF => DatFileType::Palette,
            0x05000000..=0x05FFFFFF => DatFileType::SurfaceTexture,
            0x06000000..=0x07FFFFFF => DatFileType::Texture,
            0x08000000..=0x0800FFFF => DatFileType::Surface,
            0x09000000..=0x0900FFFF => DatFileType::MotionTable,
            0x0A000000..=0x0A00FFFF => DatFileType::Wave,
            0x0D000000..=0x0D00FFFF => DatFileType::Environment,
            0x0E000002 => DatFileType::CharacterGenerator,
            0x0E000003 => DatFileType::SecondaryAttributeTable,
            0x0E000004 => DatFileType::SkillTable,
            0x0E000007 => DatFileType::ChatPoseTable,
            0x0E00000D => DatFileType::ObjectHierarchy,
            0x0E00000E => DatFileType::SpellTable,
            0x0E00000F => DatFileType::SpellComponentTable,
            0x0E000011 => DatFileType::TreasureTable,
            0x0E000018 => DatFileType::XpTable,
            0x0E000019 => DatFileType::CraftTable,
            0x0E00001A => DatFileType::BadData,
            0x0E00001B => DatFileType::Quests,
            0x0E00001C => DatFileType::GameEventTable,
            0x0E00001D => DatFileType::ContractTable,
            0x0E00001E => DatFileType::TabooTable,
            0x0E00001F => DatFileType::FileToId,
            0x0E000020 => DatFileType::NameFilterTable,
            0x0E010000..=0x0E01FFFF => DatFileType::QualityFilter,
            0x0E020000..=0x0E02FFFF => DatFileType::MonitoredProperties,
            0x0E000000..=0x0EFFFFFF => DatFileType::CharGen,
            0x0F000000..=0x0F00FFFF => DatFileType::PaletteSet,
            0x10000000..=0x1000FFFF => DatFileType::Clothing,
            0x11000000..=0x1100FFFF => DatFileType::DegradeInfo,
            0x12000000..=0x1200FFFF => DatFileType::Scene,
            0x13000000..=0x1300FFFF => DatFileType::Region,
            0x14000000..=0x1400FFFF => DatFileType::KeyMap,
            0x15000000..=0x15FFFFFF => DatFileType::RenderTexture,
            0x16000000..=0x16FFFFFF => DatFileType::RenderMaterial,
            0x17000000..=0x17FFFFFF => DatFileType::MaterialModifier,
            0x18000000..=0x18FFFFFF => DatFileType::MaterialInstance,
            0x19000000..=0x19FFFFFF => DatFileType::RenderMesh,
            0x20000000..=0x2000FFFF => DatFileType::SoundTable,
            0x21000000..=0x21FFFFFF => DatFileType::UiLayout,
            0x22000000..=0x22FFFFFF => DatFileType::EnumMapper,
            0x23000000..=0x24FFFFFF => DatFileType::StringTable,
            0x25000000..=0x25FFFFFF => DatFileType::DidMapper,
            0x26000000..=0x2600FFFF => DatFileType::ActionMap,
            0x27000000..=0x27FFFFFF => DatFileType::DualDidMapper,
            0x30000000..=0x3000FFFF => DatFileType::CombatTable,
            0x31000000..=0x3100FFFF => DatFileType::String,
            0x32000000..=0x3200FFFF => DatFileType::ParticleEmitter,
            0x33000000..=0x3300FFFF => DatFileType::PhysicsScript,
            0x34000000..=0x3400FFFF => DatFileType::PhysicsScriptTable,
            0x38000000..=0x3800FFFF => DatFileType::ItemMutation,
            0x39000000..=0x39FFFFFF => DatFileType::MasterProperty,
            0x40000000..=0x40000FFF => DatFileType::Font,
            0x40001000..=0x400FFFFF => DatFileType::FontLocal,
            0x41000000..=0x41FFFFFF => DatFileType::StringState,
            0x78000000..=0x7FFFFFFF => DatFileType::DbProperties,
            0x0000FFFE => DatFileType::LandBlockInfo,
            0x0000FFFF => DatFileType::LandBlock,
            0x00000001..=0x0000FFFD => DatFileType::WeenieDefaults,
            _ => DatFileType::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash, EnumIter)]
#[repr(u32)]
pub enum DatFileSubtype {
    Icon,
    None,
}

impl DatFileSubtype {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn from_u32(value: u32) -> Option<Self> {
        Self::iter().find(|variant| variant.as_u32() == value)
    }
}

#[cfg(test)]
mod tests {
    use super::DatFileType;

    #[test]
    fn object_id_mapping_covers_known_dat_file_types() {
        let cases = [
            (0xDEADBEEF, DatFileType::Unknown),
            (0x0000FFFF, DatFileType::LandBlock),
            (0x0000FFFE, DatFileType::LandBlockInfo),
            (0x00001234, DatFileType::WeenieDefaults),
            (0x01000001, DatFileType::GraphicsObject),
            (0x02000001, DatFileType::Setup),
            (0x03000001, DatFileType::Animation),
            (0x04000001, DatFileType::Palette),
            (0x05000001, DatFileType::SurfaceTexture),
            (0x06000001, DatFileType::Texture),
            (0x08000001, DatFileType::Surface),
            (0x09000001, DatFileType::MotionTable),
            (0x0A000001, DatFileType::Wave),
            (0x0D000001, DatFileType::Environment),
            (0x0E000001, DatFileType::CharGen),
            (0x0E000002, DatFileType::CharacterGenerator),
            (0x0E000003, DatFileType::SecondaryAttributeTable),
            (0x0E000004, DatFileType::SkillTable),
            (0x0E000007, DatFileType::ChatPoseTable),
            (0x0E00000D, DatFileType::ObjectHierarchy),
            (0x0E00000E, DatFileType::SpellTable),
            (0x0E00000F, DatFileType::SpellComponentTable),
            (0x0E000011, DatFileType::TreasureTable),
            (0x0E000018, DatFileType::XpTable),
            (0x0E000019, DatFileType::CraftTable),
            (0x0E00001A, DatFileType::BadData),
            (0x0E00001B, DatFileType::Quests),
            (0x0E00001C, DatFileType::GameEventTable),
            (0x0E00001D, DatFileType::ContractTable),
            (0x0E00001E, DatFileType::TabooTable),
            (0x0E00001F, DatFileType::FileToId),
            (0x0E000020, DatFileType::NameFilterTable),
            (0x0E010001, DatFileType::QualityFilter),
            (0x0E020001, DatFileType::MonitoredProperties),
            (0x0F000001, DatFileType::PaletteSet),
            (0x10000001, DatFileType::Clothing),
            (0x11000001, DatFileType::DegradeInfo),
            (0x12000001, DatFileType::Scene),
            (0x13000001, DatFileType::Region),
            (0x14000001, DatFileType::KeyMap),
            (0x15000001, DatFileType::RenderTexture),
            (0x16000001, DatFileType::RenderMaterial),
            (0x17000001, DatFileType::MaterialModifier),
            (0x18000001, DatFileType::MaterialInstance),
            (0x19000001, DatFileType::RenderMesh),
            (0x20000001, DatFileType::SoundTable),
            (0x21000001, DatFileType::UiLayout),
            (0x22000001, DatFileType::EnumMapper),
            (0x23000001, DatFileType::StringTable),
            (0x25000001, DatFileType::DidMapper),
            (0x26000001, DatFileType::ActionMap),
            (0x27000001, DatFileType::DualDidMapper),
            (0x30000001, DatFileType::CombatTable),
            (0x31000001, DatFileType::String),
            (0x32000001, DatFileType::ParticleEmitter),
            (0x33000001, DatFileType::PhysicsScript),
            (0x34000001, DatFileType::PhysicsScriptTable),
            (0x38000001, DatFileType::ItemMutation),
            (0x39000001, DatFileType::MasterProperty),
            (0x40000001, DatFileType::Font),
            (0x40001000, DatFileType::FontLocal),
            (0x41000001, DatFileType::StringState),
            (0x78000001, DatFileType::DbProperties),
        ];

        for (object_id, expected) in cases {
            assert_eq!(DatFileType::from_object_id(object_id), expected);
        }
    }
}
