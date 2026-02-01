//! ACE-specific enums for loot generation and game mechanics.
//! Based on ACEmulator's enums.
//! NOTE: This file is generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use num_enum::TryFromPrimitive;
use crate::readers::ACReader;
use crate::writers::ACWriter;

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum TreasureItemType {
    Undef = 0x0,
    Pyreal = 0x1,
    Gem = 0x2,
    Jewelry = 0x3,
    ArtObject = 0x4,
    Weapon = 0x5,
    Armor = 0x6,
    Clothing = 0x7,
    Scroll = 0x8,
    Caster = 0x9,
    ManaStone = 0xA,
    Consumable = 0xB,
    HealKit = 0xC,
    Lockpick = 0xD,
    SpellComponent = 0xE,
    SocietyArmor = 0xF,
    SocietyBreastplate = 0x10,
    SocietyGauntlets = 0x11,
    SocietyGirth = 0x12,
    SocietyGreaves = 0x13,
    SocietyHelm = 0x14,
    SocietyPauldrons = 0x15,
    SocietyTassets = 0x16,
    SocietyVambraces = 0x17,
    SocietySollerets = 0x18,
    Cloak = 0x19,
    PetDevice = 0x1A,
    EncapsulatedSpirit = 0x1B,
    Aetheria = 0x1C,
}

impl crate::readers::ACDataType for TreasureItemType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(TreasureItemType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for TreasureItemType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for TreasureItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TreasureItemType::Undef => "Undef",
            TreasureItemType::Pyreal => "Pyreal",
            TreasureItemType::Gem => "Gem",
            TreasureItemType::Jewelry => "Jewelry",
            TreasureItemType::ArtObject => "ArtObject",
            TreasureItemType::Weapon => "Weapon",
            TreasureItemType::Armor => "Armor",
            TreasureItemType::Clothing => "Clothing",
            TreasureItemType::Scroll => "Scroll",
            TreasureItemType::Caster => "Caster",
            TreasureItemType::ManaStone => "ManaStone",
            TreasureItemType::Consumable => "Consumable",
            TreasureItemType::HealKit => "HealKit",
            TreasureItemType::Lockpick => "Lockpick",
            TreasureItemType::SpellComponent => "SpellComponent",
            TreasureItemType::SocietyArmor => "SocietyArmor",
            TreasureItemType::SocietyBreastplate => "SocietyBreastplate",
            TreasureItemType::SocietyGauntlets => "SocietyGauntlets",
            TreasureItemType::SocietyGirth => "SocietyGirth",
            TreasureItemType::SocietyGreaves => "SocietyGreaves",
            TreasureItemType::SocietyHelm => "SocietyHelm",
            TreasureItemType::SocietyPauldrons => "SocietyPauldrons",
            TreasureItemType::SocietyTassets => "SocietyTassets",
            TreasureItemType::SocietyVambraces => "SocietyVambraces",
            TreasureItemType::SocietySollerets => "SocietySollerets",
            TreasureItemType::Cloak => "Cloak",
            TreasureItemType::PetDevice => "PetDevice",
            TreasureItemType::EncapsulatedSpirit => "EncapsulatedSpirit",
            TreasureItemType::Aetheria => "Aetheria",
        };
        write!(f, "{}", s)
    }
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TryFromPrimitive)]
pub enum TreasureWeaponType {
    Undef = 0x0,
    MeleeWeapon = 0x1,
    Axe = 0x2,
    Dagger = 0x3,
    DaggerMS = 0x4,
    Mace = 0x5,
    MaceJitte = 0x6,
    Spear = 0x7,
    Staff = 0x8,
    Sword = 0x9,
    SwordMS = 0xA,
    Unarmed = 0xB,
    MissileWeapon = 0xC,
    Bow = 0xD,
    Crossbow = 0xE,
    Atlatl = 0xF,
    Caster = 0x10,
    TwoHandedWeapon = 0x11,
    TwoHandedAxe = 0x12,
    TwoHandedMace = 0x13,
    TwoHandedSpear = 0x14,
    TwoHandedSword = 0x15,
}

impl crate::readers::ACDataType for TreasureWeaponType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let value = crate::readers::read_u32(reader)?;
        Ok(TreasureWeaponType::try_from(value)?)
    }
}

impl crate::writers::ACWritable for TreasureWeaponType {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        crate::writers::write_u32(writer, self.clone() as u32)?;
        Ok(())
    }
}

impl std::fmt::Display for TreasureWeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TreasureWeaponType::Undef => "Undef",
            TreasureWeaponType::MeleeWeapon => "MeleeWeapon",
            TreasureWeaponType::Axe => "Axe",
            TreasureWeaponType::Dagger => "Dagger",
            TreasureWeaponType::DaggerMS => "DaggerMS",
            TreasureWeaponType::Mace => "Mace",
            TreasureWeaponType::MaceJitte => "MaceJitte",
            TreasureWeaponType::Spear => "Spear",
            TreasureWeaponType::Staff => "Staff",
            TreasureWeaponType::Sword => "Sword",
            TreasureWeaponType::SwordMS => "SwordMS",
            TreasureWeaponType::Unarmed => "Unarmed",
            TreasureWeaponType::MissileWeapon => "MissileWeapon",
            TreasureWeaponType::Bow => "Bow",
            TreasureWeaponType::Crossbow => "Crossbow",
            TreasureWeaponType::Atlatl => "Atlatl",
            TreasureWeaponType::Caster => "Caster",
            TreasureWeaponType::TwoHandedWeapon => "TwoHandedWeapon",
            TreasureWeaponType::TwoHandedAxe => "TwoHandedAxe",
            TreasureWeaponType::TwoHandedMace => "TwoHandedMace",
            TreasureWeaponType::TwoHandedSpear => "TwoHandedSpear",
            TreasureWeaponType::TwoHandedSword => "TwoHandedSword",
        };
        write!(f, "{}", s)
    }
}

