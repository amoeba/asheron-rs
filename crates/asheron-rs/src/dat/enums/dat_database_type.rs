use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, Display, PartialEq, EnumIter)]
#[repr(u32)]
pub enum DatDatabaseType {
    Portal,
    Cell,
}

impl DatDatabaseType {
    pub fn as_u32(&self) -> u32 {
        self.clone() as u32
    }

    pub fn from_u32(value: u32) -> Option<Self> {
        Self::iter().find(|variant| variant.as_u32() == value)
    }
}
