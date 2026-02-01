// from trevis. not hooked up yet but these come from the dats.
use num_derive::FromPrimitive;

#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Debug, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum UIAttributeIcons {
    Invalid = 0x00000000,
    Strength = 0x060002C8,
    Endurance = 0x060002C4,
    Quickness = 0x060002C6,
    Coordination = 0x060002C9,
    Focus = 0x060002C5,
    SelfAttr = 0x060002C7,
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Debug, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum UIAttribute2ndIcons {
    Invalid = 0x00000000,
    MaxHealth = 0x06004C3B,
    MaxStamina = 0x06004C3C,
    MaxMana = 0x06004C3D,
}

impl UIAttribute2ndIcons {
    #[allow(dead_code)]
    pub const HEALTH: u32 = 0x06004C3B;
    #[allow(dead_code)]
    pub const STAMINA: u32 = 0x06004C3C;
    #[allow(dead_code)]
    pub const MANA: u32 = 0x06004C3D;
}

#[allow(non_camel_case_types, dead_code, clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum UIIconBackgrounds {
    UNDEF = 0x00000000,
    MeleeWeapon = 0x060011CB,
    Armor = 0x060011CF,
    Clothing = 0x060011F3,
    Jewelry = 0x060011D5,
    Creature = 0x060011D1,
    Food = 0x060011CC,
    Money = 0x060011F4,
    MissileWeapon = 0x060011D2,
    Container = 0x060011CE,
    Useless = 0x060011D0,
    Gem = 0x060011D3,
    SpellComponents = 0x060011CD,
    Default = 0x060011D4,
    Service = 0x06005E23,
}

impl UIIconBackgrounds {
    // Aliases with value 0x060011D4 (same as Default)
    #[allow(dead_code)]
    pub const CRAFT_COOKING_BASE: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const CRAFT_ALCHEMY_BASE: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const CRAFT_FLETCHING_BASE: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const NOT_USED: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const CRAFT_ALCHEMY_INTERMEDIATE: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const CRAFT_FLETCHING_INTERMEDIATE: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const LIFESTONE: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const TINKERING_TOOL: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const TINKERING_MATERIAL: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const MISC: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const GAMEBOARD: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const WRITABLE: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const KEY: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const CASTER: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const PORTAL: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const LOCKABLE: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const PROMISSORY_NOTE: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const MANASTONE: u32 = 0x060011D4;
    #[allow(dead_code)]
    pub const MAGIC_WIELDABLE: u32 = 0x060011D4;
}

#[allow(non_camel_case_types, dead_code, clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum UIEffectIcons {
    UNDEF = 0x00000000,
    Magical = 0x060011CA,
    Poisoned = 0x060011C6,
    BoostHealth = 0x06001B05,
    BoostStamina = 0x06001B06,
    Fire = 0x06001B2E,
    Lightning = 0x06001B2D,
    Frost = 0x06001B2F,
    Acid = 0x06001B2C,
    Default = 0x060011C5,
    Bludgeoning = 0x060033C3,
    Slashing = 0x060033C2,
    Piercing = 0x060033C4,
}

impl UIEffectIcons {
    #[allow(dead_code)]
    pub const BOOST_MANA: u32 = 0x060011CA;
}

#[allow(non_camel_case_types, dead_code, clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum UISpellBackgrounds {
    UNDEF = 0x00000000,
    LeadScarab = 0x060013F4,
    IronScarab = 0x060013F5,
    CopperScarab = 0x060013F6,
    SilverScarab = 0x060013F7,
    GoldScarab = 0x060013F8,
    PyrealScarab = 0x060013F9,
    PlatinumScarab = 0x06001F63,
    Default = 0x060011C5,
    ManaScarab = 0x060067A6,
}

impl UISpellBackgrounds {
    #[allow(dead_code)]
    pub const DIAMOND_SCARAB: u32 = 0x060013F6;
    #[allow(dead_code)]
    pub const DARK_SCARAB: u32 = 0x060013F6;
}

#[allow(non_camel_case_types, dead_code, clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum UISpellOverlays {
    UNDEF = 0x00000000,
    Reversed = 0x06004C3E,
    NonReversed = 0x06004C3F,
    TargetSelf = 0x060013F3,
    TargetFellowship = 0x060030D7,
    Default = 0x060011C5,
}
