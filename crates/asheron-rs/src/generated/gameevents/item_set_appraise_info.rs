use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// The result of an attempt to assess an item or creature.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_SetAppraiseInfo")]
pub struct ItemSetAppraiseInfo {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "IntProperties", skip_serializing_if = "Option::is_none")]
    pub int_properties: Option<PackableHashTable<PropertyInt, i32>>,
    #[serde(rename = "Int64Properties", skip_serializing_if = "Option::is_none")]
    pub int64_properties: Option<PackableHashTable<PropertyInt64, i64>>,
    #[serde(rename = "BoolProperties", skip_serializing_if = "Option::is_none")]
    pub bool_properties: Option<PackableHashTable<PropertyBool, bool>>,
    #[serde(rename = "FloatProperties", skip_serializing_if = "Option::is_none")]
    pub float_properties: Option<PackableHashTable<PropertyFloat, f64>>,
    #[serde(rename = "StringProperties", skip_serializing_if = "Option::is_none")]
    pub string_properties: Option<PackableHashTable<PropertyString, String>>,
    #[serde(rename = "DataIdProperties", skip_serializing_if = "Option::is_none")]
    pub data_id_properties: Option<PackableHashTable<PropertyDataId, DataId>>,
    #[serde(rename = "SpellBook", skip_serializing_if = "Option::is_none")]
    pub spell_book: Option<PackableList<LayeredSpellId>>,
    #[serde(rename = "ArmorProfile", skip_serializing_if = "Option::is_none")]
    pub armor_profile: Option<ArmorProfile>,
    #[serde(rename = "CreatureProfile", skip_serializing_if = "Option::is_none")]
    pub creature_profile: Option<CreatureAppraisalProfile>,
    #[serde(rename = "WeaponProfile", skip_serializing_if = "Option::is_none")]
    pub weapon_profile: Option<WeaponProfile>,
    #[serde(rename = "HookProfile", skip_serializing_if = "Option::is_none")]
    pub hook_profile: Option<HookAppraisalProfile>,
    #[serde(rename = "ArmorHighlight", skip_serializing_if = "Option::is_none")]
    pub armor_highlight: Option<ArmorHighlightMask>,
    #[serde(rename = "ArmorColor", skip_serializing_if = "Option::is_none")]
    pub armor_color: Option<ArmorHighlightMask>,
    #[serde(rename = "WeaponHighlight", skip_serializing_if = "Option::is_none")]
    pub weapon_highlight: Option<WeaponHighlightMask>,
    #[serde(rename = "WeaponColor", skip_serializing_if = "Option::is_none")]
    pub weapon_color: Option<WeaponHighlightMask>,
    #[serde(rename = "ResistHighlight", skip_serializing_if = "Option::is_none")]
    pub resist_highlight: Option<ResistHighlightMask>,
    #[serde(rename = "ResistColor", skip_serializing_if = "Option::is_none")]
    pub resist_color: Option<ResistHighlightMask>,
    #[serde(rename = "BaseArmorHead", skip_serializing_if = "Option::is_none")]
    pub base_armor_head: Option<u32>,
    #[serde(rename = "BaseArmorChest", skip_serializing_if = "Option::is_none")]
    pub base_armor_chest: Option<u32>,
    #[serde(rename = "BaseArmorGroin", skip_serializing_if = "Option::is_none")]
    pub base_armor_groin: Option<u32>,
    #[serde(rename = "BaseArmorBicep", skip_serializing_if = "Option::is_none")]
    pub base_armor_bicep: Option<u32>,
    #[serde(rename = "BaseArmorWrist", skip_serializing_if = "Option::is_none")]
    pub base_armor_wrist: Option<u32>,
    #[serde(rename = "BaseArmorHand", skip_serializing_if = "Option::is_none")]
    pub base_armor_hand: Option<u32>,
    #[serde(rename = "BaseArmorThigh", skip_serializing_if = "Option::is_none")]
    pub base_armor_thigh: Option<u32>,
    #[serde(rename = "BaseArmorShin", skip_serializing_if = "Option::is_none")]
    pub base_armor_shin: Option<u32>,
    #[serde(rename = "BaseArmorFoot", skip_serializing_if = "Option::is_none")]
    pub base_armor_foot: Option<u32>,
}

impl crate::readers::ACDataType for ItemSetAppraiseInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemSetAppraiseInfo").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Flags", position = pos).entered()
        };
        let flags = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_flags);
        #[cfg(feature = "tracing")]
        let _field_span_success = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Success", position = pos).entered()
        };
        let success = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_success);
        let mut int_properties = None;
        if (flags & 0x00000001) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_int_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "IntProperties", position = pos).entered()
            };
            int_properties = Some(read_packable_hash_table::<PropertyInt, i32>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_int_properties);
        }
        let mut int64_properties = None;
        if (flags & 0x00002000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_int64_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "Int64Properties", position = pos).entered()
            };
            int64_properties = Some(read_packable_hash_table::<PropertyInt64, i64>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_int64_properties);
        }
        let mut bool_properties = None;
        if (flags & 0x00000002) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_bool_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BoolProperties", position = pos).entered()
            };
            bool_properties = Some(read_packable_hash_table::<PropertyBool, bool>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_bool_properties);
        }
        let mut float_properties = None;
        if (flags & 0x00000004) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_float_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "FloatProperties", position = pos).entered()
            };
            float_properties = Some(read_packable_hash_table::<PropertyFloat, f64>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_float_properties);
        }
        let mut string_properties = None;
        if (flags & 0x00000008) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_string_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "StringProperties", position = pos).entered()
            };
            string_properties = Some(read_packable_hash_table::<PropertyString, String>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_string_properties);
        }
        let mut data_id_properties = None;
        if (flags & 0x00001000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_data_id_properties = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "DataIdProperties", position = pos).entered()
            };
            data_id_properties = Some(read_packable_hash_table::<PropertyDataId, DataId>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_data_id_properties);
        }
        let mut spell_book = None;
        if (flags & 0x00000010) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_spell_book = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "SpellBook", position = pos).entered()
            };
            spell_book = Some(read_packable_list::<LayeredSpellId>(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_spell_book);
        }
        let mut armor_profile = None;
        if (flags & 0x00000080) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_armor_profile = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ArmorProfile", position = pos).entered()
            };
            armor_profile = Some(ArmorProfile::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_armor_profile);
        }
        let mut creature_profile = None;
        if (flags & 0x00000100) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_creature_profile = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "CreatureProfile", position = pos).entered()
            };
            creature_profile = Some(CreatureAppraisalProfile::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_creature_profile);
        }
        let mut weapon_profile = None;
        if (flags & 0x00000020) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_weapon_profile = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "WeaponProfile", position = pos).entered()
            };
            weapon_profile = Some(WeaponProfile::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_weapon_profile);
        }
        let mut hook_profile = None;
        if (flags & 0x00000040) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_hook_profile = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "HookProfile", position = pos).entered()
            };
            hook_profile = Some(HookAppraisalProfile::read(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_hook_profile);
        }
        let mut armor_highlight = None;
        let mut armor_color = None;
        if (flags & 0x00000200) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_armor_highlight = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ArmorHighlight", position = pos).entered()
            };
            armor_highlight = Some(Ok::<_, Box<dyn std::error::Error>>(ArmorHighlightMask::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_armor_highlight);
            #[cfg(feature = "tracing")]
            let _field_span_armor_color = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ArmorColor", position = pos).entered()
            };
            armor_color = Some(Ok::<_, Box<dyn std::error::Error>>(ArmorHighlightMask::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_armor_color);
        }
        let mut weapon_highlight = None;
        let mut weapon_color = None;
        if (flags & 0x00000800) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_weapon_highlight = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "WeaponHighlight", position = pos).entered()
            };
            weapon_highlight = Some(Ok::<_, Box<dyn std::error::Error>>(WeaponHighlightMask::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_weapon_highlight);
            #[cfg(feature = "tracing")]
            let _field_span_weapon_color = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "WeaponColor", position = pos).entered()
            };
            weapon_color = Some(Ok::<_, Box<dyn std::error::Error>>(WeaponHighlightMask::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_weapon_color);
        }
        let mut resist_highlight = None;
        let mut resist_color = None;
        if (flags & 0x00000400) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_resist_highlight = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ResistHighlight", position = pos).entered()
            };
            resist_highlight = Some(Ok::<_, Box<dyn std::error::Error>>(ResistHighlightMask::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_resist_highlight);
            #[cfg(feature = "tracing")]
            let _field_span_resist_color = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "ResistColor", position = pos).entered()
            };
            resist_color = Some(Ok::<_, Box<dyn std::error::Error>>(ResistHighlightMask::from_bits_retain(read_u16(reader)?))?);
            #[cfg(feature = "tracing")]
            drop(_field_span_resist_color);
        }
        let mut base_armor_head = None;
        let mut base_armor_chest = None;
        let mut base_armor_groin = None;
        let mut base_armor_bicep = None;
        let mut base_armor_wrist = None;
        let mut base_armor_hand = None;
        let mut base_armor_thigh = None;
        let mut base_armor_shin = None;
        let mut base_armor_foot = None;
        if (flags & 0x00004000) != 0 {
            #[cfg(feature = "tracing")]
            let _field_span_base_armor_head = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BaseArmorHead", position = pos).entered()
            };
            base_armor_head = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_base_armor_head);
            #[cfg(feature = "tracing")]
            let _field_span_base_armor_chest = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BaseArmorChest", position = pos).entered()
            };
            base_armor_chest = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_base_armor_chest);
            #[cfg(feature = "tracing")]
            let _field_span_base_armor_groin = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BaseArmorGroin", position = pos).entered()
            };
            base_armor_groin = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_base_armor_groin);
            #[cfg(feature = "tracing")]
            let _field_span_base_armor_bicep = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BaseArmorBicep", position = pos).entered()
            };
            base_armor_bicep = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_base_armor_bicep);
            #[cfg(feature = "tracing")]
            let _field_span_base_armor_wrist = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BaseArmorWrist", position = pos).entered()
            };
            base_armor_wrist = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_base_armor_wrist);
            #[cfg(feature = "tracing")]
            let _field_span_base_armor_hand = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BaseArmorHand", position = pos).entered()
            };
            base_armor_hand = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_base_armor_hand);
            #[cfg(feature = "tracing")]
            let _field_span_base_armor_thigh = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BaseArmorThigh", position = pos).entered()
            };
            base_armor_thigh = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_base_armor_thigh);
            #[cfg(feature = "tracing")]
            let _field_span_base_armor_shin = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BaseArmorShin", position = pos).entered()
            };
            base_armor_shin = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_base_armor_shin);
            #[cfg(feature = "tracing")]
            let _field_span_base_armor_foot = {
                let pos = reader.stream_position().unwrap_or(0);
                tracing::span!(tracing::Level::TRACE, "field", name = "BaseArmorFoot", position = pos).entered()
            };
            base_armor_foot = Some(read_u32(reader)?);
            #[cfg(feature = "tracing")]
            drop(_field_span_base_armor_foot);
        }

        Ok(Self {
            object_id,
            flags,
            success,
            int_properties,
            int64_properties,
            bool_properties,
            float_properties,
            string_properties,
            data_id_properties,
            spell_book,
            armor_profile,
            creature_profile,
            weapon_profile,
            hook_profile,
            armor_highlight,
            armor_color,
            weapon_highlight,
            weapon_color,
            resist_highlight,
            resist_color,
            base_armor_head,
            base_armor_chest,
            base_armor_groin,
            base_armor_bicep,
            base_armor_wrist,
            base_armor_hand,
            base_armor_thigh,
            base_armor_shin,
            base_armor_foot,
        })
    }
}

impl crate::writers::ACWritable for ItemSetAppraiseInfo {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ItemSetAppraiseInfo").entered();

        self.object_id.write(writer)?;
        write_u32(writer, self.flags)?;
        write_bool(writer, self.success)?;
        if (self.flags & 0x00000001) != 0
            && let Some(ref value) = self.int_properties {
                write_packable_hash_table::<PropertyInt, i32>(writer, value)?;
            }
        if (self.flags & 0x00002000) != 0
            && let Some(ref value) = self.int64_properties {
                write_packable_hash_table::<PropertyInt64, i64>(writer, value)?;
            }
        if (self.flags & 0x00000002) != 0
            && let Some(ref value) = self.bool_properties {
                write_packable_hash_table::<PropertyBool, bool>(writer, value)?;
            }
        if (self.flags & 0x00000004) != 0
            && let Some(ref value) = self.float_properties {
                write_packable_hash_table::<PropertyFloat, f64>(writer, value)?;
            }
        if (self.flags & 0x00000008) != 0
            && let Some(ref value) = self.string_properties {
                write_packable_hash_table::<PropertyString, String>(writer, value)?;
            }
        if (self.flags & 0x00001000) != 0
            && let Some(ref value) = self.data_id_properties {
                write_packable_hash_table::<PropertyDataId, DataId>(writer, value)?;
            }
        if (self.flags & 0x00000010) != 0
            && let Some(ref value) = self.spell_book {
                write_packable_list::<LayeredSpellId>(writer, value)?;
            }
        if (self.flags & 0x00000080) != 0
            && let Some(ref value) = self.armor_profile {
                value.write(writer)?;
            }
        if (self.flags & 0x00000100) != 0
            && let Some(ref value) = self.creature_profile {
                value.write(writer)?;
            }
        if (self.flags & 0x00000020) != 0
            && let Some(ref value) = self.weapon_profile {
                value.write(writer)?;
            }
        if (self.flags & 0x00000040) != 0
            && let Some(ref value) = self.hook_profile {
                value.write(writer)?;
            }
        if (self.flags & 0x00000200) != 0
            && let Some(ref value) = self.armor_highlight {
                write_u16(writer, value.bits())?;
            }
        if (self.flags & 0x00000200) != 0
            && let Some(ref value) = self.armor_color {
                write_u16(writer, value.bits())?;
            }
        if (self.flags & 0x00000800) != 0
            && let Some(ref value) = self.weapon_highlight {
                write_u16(writer, value.bits())?;
            }
        if (self.flags & 0x00000800) != 0
            && let Some(ref value) = self.weapon_color {
                write_u16(writer, value.bits())?;
            }
        if (self.flags & 0x00000400) != 0
            && let Some(ref value) = self.resist_highlight {
                write_u16(writer, value.bits())?;
            }
        if (self.flags & 0x00000400) != 0
            && let Some(ref value) = self.resist_color {
                write_u16(writer, value.bits())?;
            }
        if (self.flags & 0x00004000) != 0
            && let Some(ref value) = self.base_armor_head {
                write_u32(writer, *value)?;
            }
        if (self.flags & 0x00004000) != 0
            && let Some(ref value) = self.base_armor_chest {
                write_u32(writer, *value)?;
            }
        if (self.flags & 0x00004000) != 0
            && let Some(ref value) = self.base_armor_groin {
                write_u32(writer, *value)?;
            }
        if (self.flags & 0x00004000) != 0
            && let Some(ref value) = self.base_armor_bicep {
                write_u32(writer, *value)?;
            }
        if (self.flags & 0x00004000) != 0
            && let Some(ref value) = self.base_armor_wrist {
                write_u32(writer, *value)?;
            }
        if (self.flags & 0x00004000) != 0
            && let Some(ref value) = self.base_armor_hand {
                write_u32(writer, *value)?;
            }
        if (self.flags & 0x00004000) != 0
            && let Some(ref value) = self.base_armor_thigh {
                write_u32(writer, *value)?;
            }
        if (self.flags & 0x00004000) != 0
            && let Some(ref value) = self.base_armor_shin {
                write_u32(writer, *value)?;
            }
        if (self.flags & 0x00004000) != 0
            && let Some(ref value) = self.base_armor_foot {
                write_u32(writer, *value)?;
            }
        Ok(())
    }
}

