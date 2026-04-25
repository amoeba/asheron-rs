use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum MagicSchool {
    None = 0,
    WarMagic = 1,
    LifeMagic = 2,
    ItemEnchantment = 3,
    CreatureEnchantment = 4,
    VoidMagic = 5,
}
