use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum SpellType {
    Undef = 0,
    Enchantment = 1,
    Projectile = 2,
    Boost = 3,
    Transfer = 4,
    PortalLink = 5,
    PortalRecall = 6,
    PortalSummon = 7,
    PortalSending = 8,
    Dispel = 9,
    LifeProjectile = 10,
    FellowBoost = 11,
    FellowEnchantment = 12,
    FellowPortalSending = 13,
    FellowDispel = 14,
    EnchantmentProjectile = 15,
}
