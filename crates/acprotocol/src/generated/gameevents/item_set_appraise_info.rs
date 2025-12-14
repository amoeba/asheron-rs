use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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
        let object_id = ObjectId::read(reader)?;
        let flags = read_u32(reader)?;
        let success = read_bool(reader)?;
        let mut int_properties = None;
        if (flags & 0x00000001) != 0 {
            int_properties = Some(read_packable_hash_table::<PropertyInt, i32>(reader)?);
        }
        let mut int64_properties = None;
        if (flags & 0x00002000) != 0 {
            int64_properties = Some(read_packable_hash_table::<PropertyInt64, i64>(reader)?);
        }
        let mut bool_properties = None;
        if (flags & 0x00000002) != 0 {
            bool_properties = Some(read_packable_hash_table::<PropertyBool, bool>(reader)?);
        }
        let mut float_properties = None;
        if (flags & 0x00000004) != 0 {
            float_properties = Some(read_packable_hash_table::<PropertyFloat, f64>(reader)?);
        }
        let mut string_properties = None;
        if (flags & 0x00000008) != 0 {
            string_properties = Some(read_packable_hash_table::<PropertyString, String>(reader)?);
        }
        let mut data_id_properties = None;
        if (flags & 0x00001000) != 0 {
            data_id_properties = Some(read_packable_hash_table::<PropertyDataId, DataId>(reader)?);
        }
        let mut spell_book = None;
        if (flags & 0x00000010) != 0 {
            spell_book = Some(read_packable_list::<LayeredSpellId>(reader)?);
        }
        let mut armor_profile = None;
        if (flags & 0x00000080) != 0 {
            armor_profile = Some(ArmorProfile::read(reader)?);
        }
        let mut creature_profile = None;
        if (flags & 0x00000100) != 0 {
            creature_profile = Some(CreatureAppraisalProfile::read(reader)?);
        }
        let mut weapon_profile = None;
        if (flags & 0x00000020) != 0 {
            weapon_profile = Some(WeaponProfile::read(reader)?);
        }
        let mut hook_profile = None;
        if (flags & 0x00000040) != 0 {
            hook_profile = Some(HookAppraisalProfile::read(reader)?);
        }
        let mut armor_highlight = None;
        let mut armor_color = None;
        if (flags & 0x00000200) != 0 {
            armor_highlight = Some(Ok::<_, Box<dyn std::error::Error>>(ArmorHighlightMask::from_bits_retain(read_u16(reader)?))?);
            armor_color = Some(Ok::<_, Box<dyn std::error::Error>>(ArmorHighlightMask::from_bits_retain(read_u16(reader)?))?);
        }
        let mut weapon_highlight = None;
        let mut weapon_color = None;
        if (flags & 0x00000800) != 0 {
            weapon_highlight = Some(Ok::<_, Box<dyn std::error::Error>>(WeaponHighlightMask::from_bits_retain(read_u16(reader)?))?);
            weapon_color = Some(Ok::<_, Box<dyn std::error::Error>>(WeaponHighlightMask::from_bits_retain(read_u16(reader)?))?);
        }
        let mut resist_highlight = None;
        let mut resist_color = None;
        if (flags & 0x00000400) != 0 {
            resist_highlight = Some(Ok::<_, Box<dyn std::error::Error>>(ResistHighlightMask::from_bits_retain(read_u16(reader)?))?);
            resist_color = Some(Ok::<_, Box<dyn std::error::Error>>(ResistHighlightMask::from_bits_retain(read_u16(reader)?))?);
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
            base_armor_head = Some(read_u32(reader)?);
            base_armor_chest = Some(read_u32(reader)?);
            base_armor_groin = Some(read_u32(reader)?);
            base_armor_bicep = Some(read_u32(reader)?);
            base_armor_wrist = Some(read_u32(reader)?);
            base_armor_hand = Some(read_u32(reader)?);
            base_armor_thigh = Some(read_u32(reader)?);
            base_armor_shin = Some(read_u32(reader)?);
            base_armor_foot = Some(read_u32(reader)?);
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

