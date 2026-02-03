use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, Display, PartialEq, EnumIter)]
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
}

impl DatFileType {
    pub fn as_u32(&self) -> u32 {
        self.clone() as u32
    }

    pub fn from_u32(value: u32) -> Option<Self> {
        Self::iter().find(|variant| variant.as_u32() == value)
    }
}

#[derive(Clone, Debug, Display, PartialEq, EnumIter)]
#[repr(u32)]
pub enum DatFileSubtype {
    Icon,
    Unknown,
}

impl DatFileSubtype {
    pub fn as_u32(&self) -> u32 {
        self.clone() as u32
    }

    pub fn from_u32(value: u32) -> Option<Self> {
        Self::iter().find(|variant| variant.as_u32() == value)
    }
}
