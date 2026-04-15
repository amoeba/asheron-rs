//! Treasure system enums and helper methods.
//! Enums are imported from the ace module. Helper methods are defined here.

pub use crate::ace::*;

impl TreasureWeaponType {
    /// Returns true if this is a melee weapon type.
    pub fn is_melee_weapon(self) -> bool {
        matches!(
            self,
            TreasureWeaponType::MeleeWeapon
                | TreasureWeaponType::Axe
                | TreasureWeaponType::Dagger
                | TreasureWeaponType::DaggerMS
                | TreasureWeaponType::Mace
                | TreasureWeaponType::MaceJitte
                | TreasureWeaponType::Spear
                | TreasureWeaponType::Staff
                | TreasureWeaponType::Sword
                | TreasureWeaponType::SwordMS
                | TreasureWeaponType::Unarmed
                | TreasureWeaponType::TwoHandedWeapon
                | TreasureWeaponType::TwoHandedAxe
                | TreasureWeaponType::TwoHandedMace
                | TreasureWeaponType::TwoHandedSpear
                | TreasureWeaponType::TwoHandedSword
        )
    }

    /// Returns true if this is a missile weapon type.
    pub fn is_missile_weapon(self) -> bool {
        matches!(
            self,
            TreasureWeaponType::MissileWeapon
                | TreasureWeaponType::Bow
                | TreasureWeaponType::Crossbow
                | TreasureWeaponType::Atlatl
        )
    }

    /// Returns true if this is a caster weapon type.
    pub fn is_caster(self) -> bool {
        self == TreasureWeaponType::Caster
    }
}
