// Binary readers for s2c types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::types::*;
use crate::types::s2c::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl AllegianceAllegianceUpdateAborted {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let failure_type = WeenieError::try_from(read_u32(reader)?)?;

        Ok(Self {
            failure_type,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceUpdateAborted {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceUpdateAborted::read(reader)
    }
}

impl CommunicationPopUpString {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

impl crate::readers::ACDataType for CommunicationPopUpString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationPopUpString::read(reader)
    }
}

impl LoginPlayerDescription {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let base_qualities = ACBaseQualities::read(reader)?;
        let qualities = ACQualities::read(reader)?;
        let player_module = PlayerModule::read(reader)?;
        let content_profile = read_packable_list::<ContentProfile>(reader)?;
        let inventory_placement = read_packable_list::<InventoryPlacement>(reader)?;

        Ok(Self {
            base_qualities,
            qualities,
            player_module,
            content_profile,
            inventory_placement,
        })
    }
}

impl crate::readers::ACDataType for LoginPlayerDescription {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginPlayerDescription::read(reader)
    }
}

impl AllegianceAllegianceUpdate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let rank = read_u32(reader)?;
        let profile = AllegianceProfile::read(reader)?;

        Ok(Self {
            rank,
            profile,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceUpdate::read(reader)
    }
}

impl SocialFriendsUpdate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let friends = read_packable_list::<FriendData>(reader)?;
        let type_ = FriendsUpdateType::try_from(read_u32(reader)?)?;

        Ok(Self {
            friends,
            type_,
        })
    }
}

impl crate::readers::ACDataType for SocialFriendsUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialFriendsUpdate::read(reader)
    }
}

impl ItemServerSaysContainId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let container_id = ObjectId::read(reader)?;
        let slot_index = read_u32(reader)?;
        let container_type = ContainerProperties::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            container_id,
            slot_index,
            container_type,
        })
    }
}

impl crate::readers::ACDataType for ItemServerSaysContainId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemServerSaysContainId::read(reader)
    }
}

impl ItemWearItem {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let slot = EquipMask::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            slot,
        })
    }
}

impl crate::readers::ACDataType for ItemWearItem {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemWearItem::read(reader)
    }
}

impl SocialCharacterTitleTable {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let display_title = read_u32(reader)?;
        let titles = read_packable_list::<u32>(reader)?;

        Ok(Self {
            display_title,
            titles,
        })
    }
}

impl crate::readers::ACDataType for SocialCharacterTitleTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialCharacterTitleTable::read(reader)
    }
}

impl SocialAddOrSetCharacterTitle {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let new_title = read_u32(reader)?;
        let set_as_display_title = read_bool(reader)?;

        Ok(Self {
            new_title,
            set_as_display_title,
        })
    }
}

impl crate::readers::ACDataType for SocialAddOrSetCharacterTitle {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialAddOrSetCharacterTitle::read(reader)
    }
}

impl ItemStopViewingObjectContents {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for ItemStopViewingObjectContents {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemStopViewingObjectContents::read(reader)
    }
}

impl VendorVendorInfo {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let profile = VendorProfile::read(reader)?;
        let items = read_packable_list::<ItemProfile>(reader)?;

        Ok(Self {
            object_id,
            profile,
            items,
        })
    }
}

impl crate::readers::ACDataType for VendorVendorInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        VendorVendorInfo::read(reader)
    }
}

impl CharacterStartBarber {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let base_palette = DataId::read(reader)?;
        let head_object = DataId::read(reader)?;
        let head_texture = DataId::read(reader)?;
        let default_head_texture = DataId::read(reader)?;
        let eyes_texture = DataId::read(reader)?;
        let default_eyes_texture = DataId::read(reader)?;
        let nose_texture = DataId::read(reader)?;
        let default_nose_texture = DataId::read(reader)?;
        let mouth_texture = DataId::read(reader)?;
        let default_mouth_texture = DataId::read(reader)?;
        let skin_palette = DataId::read(reader)?;
        let hair_palette = DataId::read(reader)?;
        let eyes_palette = DataId::read(reader)?;
        let setup_id = DataId::read(reader)?;
        let option1 = read_i32(reader)?;
        let option2 = read_i32(reader)?;

        Ok(Self {
            base_palette,
            head_object,
            head_texture,
            default_head_texture,
            eyes_texture,
            default_eyes_texture,
            nose_texture,
            default_nose_texture,
            mouth_texture,
            default_mouth_texture,
            skin_palette,
            hair_palette,
            eyes_palette,
            setup_id,
            option1,
            option2,
        })
    }
}

impl crate::readers::ACDataType for CharacterStartBarber {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterStartBarber::read(reader)
    }
}

impl FellowshipQuit {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let disband = read_bool(reader)?;

        Ok(Self {
            disband,
        })
    }
}

impl crate::readers::ACDataType for FellowshipQuit {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipQuit::read(reader)
    }
}

impl FellowshipDismiss {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for FellowshipDismiss {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipDismiss::read(reader)
    }
}

impl WritingBookOpen {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let book_id = ObjectId::read(reader)?;
        let max_num_pages = read_u32(reader)?;
        let page_data = PageDataList::read(reader)?;
        let inscription = read_string(reader)?;
        let scribe_id = ObjectId::read(reader)?;
        let scribe_name = read_string(reader)?;

        Ok(Self {
            book_id,
            max_num_pages,
            page_data,
            inscription,
            scribe_id,
            scribe_name,
        })
    }
}

impl crate::readers::ACDataType for WritingBookOpen {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookOpen::read(reader)
    }
}

impl WritingBookAddPageResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let book_id = ObjectId::read(reader)?;
        let page_number = read_u32(reader)?;
        let success = read_bool(reader)?;

        Ok(Self {
            book_id,
            page_number,
            success,
        })
    }
}

impl crate::readers::ACDataType for WritingBookAddPageResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookAddPageResponse::read(reader)
    }
}

impl WritingBookDeletePageResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let book_id = ObjectId::read(reader)?;
        let page_number = read_u32(reader)?;
        let success = read_bool(reader)?;

        Ok(Self {
            book_id,
            page_number,
            success,
        })
    }
}

impl crate::readers::ACDataType for WritingBookDeletePageResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookDeletePageResponse::read(reader)
    }
}

impl WritingBookPageDataResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let page = read_u32(reader)?;
        let page_data = PageData::read(reader)?;

        Ok(Self {
            object_id,
            page,
            page_data,
        })
    }
}

impl crate::readers::ACDataType for WritingBookPageDataResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WritingBookPageDataResponse::read(reader)
    }
}

impl ItemGetInscriptionResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let inscription = read_string(reader)?;
        let scribe_name = read_string(reader)?;
        let scribe_account = read_string(reader)?;

        Ok(Self {
            inscription,
            scribe_name,
            scribe_account,
        })
    }
}

impl crate::readers::ACDataType for ItemGetInscriptionResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemGetInscriptionResponse::read(reader)
    }
}

impl ItemSetAppraiseInfo {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let flags = read_u32(reader)?;
        let success = read_bool(reader)?;
        let mut int_properties = None;
        if (flags & 0x00000001) != 0 {
            int_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyInt::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(read_i32(r)?)
        })?);
        }
        let mut int64_properties = None;
        if (flags & 0x00002000) != 0 {
            int64_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyInt64::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(read_i64(r)?)
        })?);
        }
        let mut bool_properties = None;
        if (flags & 0x00000002) != 0 {
            bool_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyBool::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(read_bool(r)?)
        })?);
        }
        let mut float_properties = None;
        if (flags & 0x00000004) != 0 {
            float_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyFloat::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(read_f64(r)?)
        })?);
        }
        let mut string_properties = None;
        if (flags & 0x00000008) != 0 {
            string_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyString::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(read_string(r)?)
        })?);
        }
        let mut data_id_properties = None;
        if (flags & 0x00001000) != 0 {
            data_id_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyDataId::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(DataId::read(r)?)
        })?);
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
            armor_highlight = Some(ArmorHighlightMask::try_from(read_u16(reader)?)?);
            armor_color = Some(ArmorHighlightMask::try_from(read_u16(reader)?)?);
        }
        let mut weapon_highlight = None;
        let mut weapon_color = None;
        if (flags & 0x00000800) != 0 {
            weapon_highlight = Some(WeaponHighlightMask::try_from(read_u16(reader)?)?);
            weapon_color = Some(WeaponHighlightMask::try_from(read_u16(reader)?)?);
        }
        let mut resist_highlight = None;
        let mut resist_color = None;
        if (flags & 0x00000400) != 0 {
            resist_highlight = Some(ResistHighlightMask::try_from(read_u16(reader)?)?);
            resist_color = Some(ResistHighlightMask::try_from(read_u16(reader)?)?);
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

impl crate::readers::ACDataType for ItemSetAppraiseInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemSetAppraiseInfo::read(reader)
    }
}

impl CommunicationChannelBroadcast {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::try_from(read_u32(reader)?)?;
        let message = read_string(reader)?;

        Ok(Self {
            channel,
            message,
        })
    }
}

impl crate::readers::ACDataType for CommunicationChannelBroadcast {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationChannelBroadcast::read(reader)
    }
}

impl CommunicationChannelList {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let characters = read_packable_list::<String>(reader)?;

        Ok(Self {
            characters,
        })
    }
}

impl crate::readers::ACDataType for CommunicationChannelList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationChannelList::read(reader)
    }
}

impl CommunicationChannelIndex {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channels = read_packable_list::<String>(reader)?;

        Ok(Self {
            channels,
        })
    }
}

impl crate::readers::ACDataType for CommunicationChannelIndex {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationChannelIndex::read(reader)
    }
}

impl ItemOnViewContents {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let container_id = ObjectId::read(reader)?;
        let items = read_packable_list::<ContentProfile>(reader)?;

        Ok(Self {
            container_id,
            items,
        })
    }
}

impl crate::readers::ACDataType for ItemOnViewContents {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemOnViewContents::read(reader)
    }
}

impl ItemServerSaysMoveItem {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for ItemServerSaysMoveItem {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemServerSaysMoveItem::read(reader)
    }
}

impl CombatHandleAttackDoneEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let number = read_u32(reader)?;

        Ok(Self {
            number,
        })
    }
}

impl crate::readers::ACDataType for CombatHandleAttackDoneEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleAttackDoneEvent::read(reader)
    }
}

impl MagicRemoveSpell {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for MagicRemoveSpell {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicRemoveSpell::read(reader)
    }
}

impl CombatHandleVictimNotificationEventSelf {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

impl crate::readers::ACDataType for CombatHandleVictimNotificationEventSelf {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleVictimNotificationEventSelf::read(reader)
    }
}

impl CombatHandleVictimNotificationEventOther {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

impl crate::readers::ACDataType for CombatHandleVictimNotificationEventOther {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleVictimNotificationEventOther::read(reader)
    }
}

impl CombatHandleAttackerNotificationEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let defender_name = read_string(reader)?;
        let type_ = DamageType::try_from(read_u32(reader)?)?;
        let damage_percent = read_f32(reader)?;
        let damage = read_u32(reader)?;
        let critical = read_bool(reader)?;
        let attack_conditions = AttackConditionsMask::try_from(read_u32(reader)?)?;

        Ok(Self {
            defender_name,
            type_,
            damage_percent,
            damage,
            critical,
            attack_conditions,
        })
    }
}

impl crate::readers::ACDataType for CombatHandleAttackerNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleAttackerNotificationEvent::read(reader)
    }
}

impl CombatHandleDefenderNotificationEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let attacker_name = read_string(reader)?;
        let type_ = DamageType::try_from(read_u32(reader)?)?;
        let damage_percent = read_f32(reader)?;
        let damage = read_u32(reader)?;
        let location = DamageLocation::try_from(read_u32(reader)?)?;
        let critical = read_bool(reader)?;
        let attack_conditions = AttackConditionsMask::try_from(read_u32(reader)?)?;

        Ok(Self {
            attacker_name,
            type_,
            damage_percent,
            damage,
            location,
            critical,
            attack_conditions,
        })
    }
}

impl crate::readers::ACDataType for CombatHandleDefenderNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleDefenderNotificationEvent::read(reader)
    }
}

impl CombatHandleEvasionAttackerNotificationEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let defender_name = read_string(reader)?;

        Ok(Self {
            defender_name,
        })
    }
}

impl crate::readers::ACDataType for CombatHandleEvasionAttackerNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleEvasionAttackerNotificationEvent::read(reader)
    }
}

impl CombatHandleEvasionDefenderNotificationEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let attacker_name = read_string(reader)?;

        Ok(Self {
            attacker_name,
        })
    }
}

impl crate::readers::ACDataType for CombatHandleEvasionDefenderNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleEvasionDefenderNotificationEvent::read(reader)
    }
}

impl CombatHandleCommenceAttackEvent {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CombatHandleCommenceAttackEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandleCommenceAttackEvent::read(reader)
    }
}

impl CombatQueryHealthResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let health = read_f32(reader)?;

        Ok(Self {
            object_id,
            health,
        })
    }
}

impl crate::readers::ACDataType for CombatQueryHealthResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatQueryHealthResponse::read(reader)
    }
}

impl CharacterQueryAgeResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_name = read_string(reader)?;
        let age = read_string(reader)?;

        Ok(Self {
            target_name,
            age,
        })
    }
}

impl crate::readers::ACDataType for CharacterQueryAgeResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterQueryAgeResponse::read(reader)
    }
}

impl ItemUseDone {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let failure_type = WeenieError::try_from(read_u32(reader)?)?;

        Ok(Self {
            failure_type,
        })
    }
}

impl crate::readers::ACDataType for ItemUseDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemUseDone::read(reader)
    }
}

impl AllegianceAllegianceUpdateDone {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let failure_type = WeenieError::try_from(read_u32(reader)?)?;

        Ok(Self {
            failure_type,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceUpdateDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceUpdateDone::read(reader)
    }
}

impl FellowshipFellowUpdateDone {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for FellowshipFellowUpdateDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipFellowUpdateDone::read(reader)
    }
}

impl FellowshipFellowStatsDone {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for FellowshipFellowStatsDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipFellowStatsDone::read(reader)
    }
}

impl ItemAppraiseDone {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown = read_u32(reader)?;

        Ok(Self {
            unknown,
        })
    }
}

impl crate::readers::ACDataType for ItemAppraiseDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemAppraiseDone::read(reader)
    }
}

impl CharacterReturnPing {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterReturnPing {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterReturnPing::read(reader)
    }
}

impl CommunicationSetSquelchDB {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let squelch_db = SquelchDB::read(reader)?;

        Ok(Self {
            squelch_db,
        })
    }
}

impl crate::readers::ACDataType for CommunicationSetSquelchDB {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationSetSquelchDB::read(reader)
    }
}

impl TradeRegisterTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let initiator_id = ObjectId::read(reader)?;
        let partner_id = ObjectId::read(reader)?;
        let stamp = read_i64(reader)?;

        Ok(Self {
            initiator_id,
            partner_id,
            stamp,
        })
    }
}

impl crate::readers::ACDataType for TradeRegisterTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeRegisterTrade::read(reader)
    }
}

impl TradeOpenTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for TradeOpenTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeOpenTrade::read(reader)
    }
}

impl TradeCloseTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let reason = EndTradeReason::try_from(read_u32(reader)?)?;

        Ok(Self {
            reason,
        })
    }
}

impl crate::readers::ACDataType for TradeCloseTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeCloseTrade::read(reader)
    }
}

impl TradeAddToTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let side = TradeSide::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            side,
        })
    }
}

impl crate::readers::ACDataType for TradeAddToTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeAddToTrade::read(reader)
    }
}

impl TradeRemoveFromTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let side = TradeSide::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            side,
        })
    }
}

impl crate::readers::ACDataType for TradeRemoveFromTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeRemoveFromTrade::read(reader)
    }
}

impl TradeAcceptTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for TradeAcceptTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeAcceptTrade::read(reader)
    }
}

impl TradeDeclineTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for TradeDeclineTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeDeclineTrade::read(reader)
    }
}

impl TradeResetTrade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for TradeResetTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeResetTrade::read(reader)
    }
}

impl TradeTradeFailure {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let reason = read_u32(reader)?;

        Ok(Self {
            object_id,
            reason,
        })
    }
}

impl crate::readers::ACDataType for TradeTradeFailure {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeTradeFailure::read(reader)
    }
}

impl TradeClearTradeAcceptance {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for TradeClearTradeAcceptance {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TradeClearTradeAcceptance::read(reader)
    }
}

impl HouseHouseProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let profile = HouseProfile::read(reader)?;

        Ok(Self {
            object_id,
            profile,
        })
    }
}

impl crate::readers::ACDataType for HouseHouseProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseHouseProfile::read(reader)
    }
}

impl HouseHouseData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let data = HouseData::read(reader)?;

        Ok(Self {
            data,
        })
    }
}

impl crate::readers::ACDataType for HouseHouseData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseHouseData::read(reader)
    }
}

impl HouseHouseStatus {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let notice_type = read_u32(reader)?;

        Ok(Self {
            notice_type,
        })
    }
}

impl crate::readers::ACDataType for HouseHouseStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseHouseStatus::read(reader)
    }
}

impl HouseUpdateRentTime {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let rent_time = read_u32(reader)?;

        Ok(Self {
            rent_time,
        })
    }
}

impl crate::readers::ACDataType for HouseUpdateRentTime {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseUpdateRentTime::read(reader)
    }
}

impl HouseUpdateRentPayment {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let rent = read_packable_list::<HousePayment>(reader)?;

        Ok(Self {
            rent,
        })
    }
}

impl crate::readers::ACDataType for HouseUpdateRentPayment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseUpdateRentPayment::read(reader)
    }
}

impl HouseUpdateRestrictions {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let sender_id = ObjectId::read(reader)?;
        let restrictions = RestrictionDB::read(reader)?;

        Ok(Self {
            sequence,
            sender_id,
            restrictions,
        })
    }
}

impl crate::readers::ACDataType for HouseUpdateRestrictions {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseUpdateRestrictions::read(reader)
    }
}

impl HouseUpdateHAR {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_list = HAR::read(reader)?;

        Ok(Self {
            guest_list,
        })
    }
}

impl crate::readers::ACDataType for HouseUpdateHAR {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseUpdateHAR::read(reader)
    }
}

impl HouseHouseTransaction {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let notice_type = read_u32(reader)?;

        Ok(Self {
            notice_type,
        })
    }
}

impl crate::readers::ACDataType for HouseHouseTransaction {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseHouseTransaction::read(reader)
    }
}

impl ItemQueryItemManaResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let mana = read_f32(reader)?;
        let success = read_bool(reader)?;

        Ok(Self {
            object_id,
            mana,
            success,
        })
    }
}

impl crate::readers::ACDataType for ItemQueryItemManaResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemQueryItemManaResponse::read(reader)
    }
}

impl HouseAvailableHouses {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = HouseType::try_from(read_u32(reader)?)?;
        let houses = read_packable_list::<u32>(reader)?;
        let num_houses = read_i32(reader)?;

        Ok(Self {
            type_,
            houses,
            num_houses,
        })
    }
}

impl crate::readers::ACDataType for HouseAvailableHouses {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseAvailableHouses::read(reader)
    }
}

impl CharacterConfirmationRequest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let confirmation_type = ConfirmationType::try_from(read_u32(reader)?)?;
        let context_id = read_u32(reader)?;
        let text = read_string(reader)?;

        Ok(Self {
            confirmation_type,
            context_id,
            text,
        })
    }
}

impl crate::readers::ACDataType for CharacterConfirmationRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterConfirmationRequest::read(reader)
    }
}

impl CharacterConfirmationDone {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let confirmation_type = ConfirmationType::try_from(read_u32(reader)?)?;
        let context_id = read_u32(reader)?;

        Ok(Self {
            confirmation_type,
            context_id,
        })
    }
}

impl crate::readers::ACDataType for CharacterConfirmationDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterConfirmationDone::read(reader)
    }
}

impl AllegianceAllegianceLoginNotificationEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;
        let is_logged_in = read_bool(reader)?;

        Ok(Self {
            character_id,
            is_logged_in,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceLoginNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceLoginNotificationEvent::read(reader)
    }
}

impl AllegianceAllegianceInfoResponseEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_id = ObjectId::read(reader)?;
        let profile = AllegianceProfile::read(reader)?;

        Ok(Self {
            target_id,
            profile,
        })
    }
}

impl crate::readers::ACDataType for AllegianceAllegianceInfoResponseEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceAllegianceInfoResponseEvent::read(reader)
    }
}

impl GameJoinGameResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let team = read_i32(reader)?;

        Ok(Self {
            game_id,
            team,
        })
    }
}

impl crate::readers::ACDataType for GameJoinGameResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameJoinGameResponse::read(reader)
    }
}

impl GameStartGame {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let team = read_i32(reader)?;

        Ok(Self {
            game_id,
            team,
        })
    }
}

impl crate::readers::ACDataType for GameStartGame {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameStartGame::read(reader)
    }
}

impl GameMoveResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let move_result = ChessMoveResult::try_from(read_i32(reader)?)?;

        Ok(Self {
            game_id,
            move_result,
        })
    }
}

impl crate::readers::ACDataType for GameMoveResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameMoveResponse::read(reader)
    }
}

impl GameOpponentTurn {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let team = read_i32(reader)?;
        let game_move = GameMoveData::read(reader)?;

        Ok(Self {
            game_id,
            team,
            game_move,
        })
    }
}

impl crate::readers::ACDataType for GameOpponentTurn {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameOpponentTurn::read(reader)
    }
}

impl GameOpponentStalemateState {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let team = read_i32(reader)?;
        let on = read_bool(reader)?;

        Ok(Self {
            game_id,
            team,
            on,
        })
    }
}

impl crate::readers::ACDataType for GameOpponentStalemateState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameOpponentStalemateState::read(reader)
    }
}

impl CommunicationWeenieError {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = WeenieError::try_from(read_u32(reader)?)?;

        Ok(Self {
            type_,
        })
    }
}

impl crate::readers::ACDataType for CommunicationWeenieError {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationWeenieError::read(reader)
    }
}

impl CommunicationWeenieErrorWithString {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = WeenieErrorWithString::try_from(read_u32(reader)?)?;
        let text = read_string(reader)?;

        Ok(Self {
            type_,
            text,
        })
    }
}

impl crate::readers::ACDataType for CommunicationWeenieErrorWithString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationWeenieErrorWithString::read(reader)
    }
}

impl GameGameOver {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let game_id = read_u32(reader)?;
        let team_winner = read_i32(reader)?;

        Ok(Self {
            game_id,
            team_winner,
        })
    }
}

impl crate::readers::ACDataType for GameGameOver {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameGameOver::read(reader)
    }
}

impl CommunicationChatRoomTracker {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let allegiance_room_id = read_u32(reader)?;
        let general_chat_room_id = read_u32(reader)?;
        let trade_chat_room_id = read_u32(reader)?;
        let lfg_chat_room_id = read_u32(reader)?;
        let roleplay_chat_room_id = read_u32(reader)?;
        let olthoi_chat_room_id = read_u32(reader)?;
        let society_chat_room_id = read_u32(reader)?;
        let society_celestial_hand_chat_room_id = read_u32(reader)?;
        let society_eldrich_web_chat_room_id = read_u32(reader)?;
        let society_radiant_blood_chat_room_id = read_u32(reader)?;

        Ok(Self {
            allegiance_room_id,
            general_chat_room_id,
            trade_chat_room_id,
            lfg_chat_room_id,
            roleplay_chat_room_id,
            olthoi_chat_room_id,
            society_chat_room_id,
            society_celestial_hand_chat_room_id,
            society_eldrich_web_chat_room_id,
            society_radiant_blood_chat_room_id,
        })
    }
}

impl crate::readers::ACDataType for CommunicationChatRoomTracker {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationChatRoomTracker::read(reader)
    }
}

impl AdminQueryPluginList {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AdminQueryPluginList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminQueryPluginList::read(reader)
    }
}

impl AdminQueryPlugin {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AdminQueryPlugin {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminQueryPlugin::read(reader)
    }
}

impl AdminQueryPluginResponse2 {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for AdminQueryPluginResponse2 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminQueryPluginResponse2::read(reader)
    }
}

impl InventorySalvageOperationsResultData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let result = SalvageOperationsResultData::read(reader)?;

        Ok(Self {
            result,
        })
    }
}

impl crate::readers::ACDataType for InventorySalvageOperationsResultData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventorySalvageOperationsResultData::read(reader)
    }
}

impl CommunicationHearDirectSpeech {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let sender_name = read_string(reader)?;
        let sender_id = ObjectId::read(reader)?;
        let target_id = ObjectId::read(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;
        let secret_flags = read_u32(reader)?;

        Ok(Self {
            message,
            sender_name,
            sender_id,
            target_id,
            type_,
            secret_flags,
        })
    }
}

impl crate::readers::ACDataType for CommunicationHearDirectSpeech {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationHearDirectSpeech::read(reader)
    }
}

impl FellowshipFullUpdate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let fellowship = Fellowship::read(reader)?;

        Ok(Self {
            fellowship,
        })
    }
}

impl crate::readers::ACDataType for FellowshipFullUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipFullUpdate::read(reader)
    }
}

impl FellowshipDisband {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for FellowshipDisband {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipDisband::read(reader)
    }
}

impl FellowshipUpdateFellow {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let fellow = Fellow::read(reader)?;
        let update_type = FellowUpdateType::try_from(read_u32(reader)?)?;

        Ok(Self {
            fellow,
            update_type,
        })
    }
}

impl crate::readers::ACDataType for FellowshipUpdateFellow {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipUpdateFellow::read(reader)
    }
}

impl MagicUpdateSpell {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for MagicUpdateSpell {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicUpdateSpell::read(reader)
    }
}

impl MagicUpdateEnchantment {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let enchantment = Enchantment::read(reader)?;

        Ok(Self {
            enchantment,
        })
    }
}

impl crate::readers::ACDataType for MagicUpdateEnchantment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicUpdateEnchantment::read(reader)
    }
}

impl MagicRemoveEnchantment {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for MagicRemoveEnchantment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicRemoveEnchantment::read(reader)
    }
}

impl MagicUpdateMultipleEnchantments {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let enchantments = read_packable_list::<Enchantment>(reader)?;

        Ok(Self {
            enchantments,
        })
    }
}

impl crate::readers::ACDataType for MagicUpdateMultipleEnchantments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicUpdateMultipleEnchantments::read(reader)
    }
}

impl MagicRemoveMultipleEnchantments {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let enchantments = read_packable_list::<LayeredSpellId>(reader)?;

        Ok(Self {
            enchantments,
        })
    }
}

impl crate::readers::ACDataType for MagicRemoveMultipleEnchantments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicRemoveMultipleEnchantments::read(reader)
    }
}

impl MagicPurgeEnchantments {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for MagicPurgeEnchantments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicPurgeEnchantments::read(reader)
    }
}

impl MagicDispelEnchantment {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for MagicDispelEnchantment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicDispelEnchantment::read(reader)
    }
}

impl MagicDispelMultipleEnchantments {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let enchantments = read_packable_list::<LayeredSpellId>(reader)?;

        Ok(Self {
            enchantments,
        })
    }
}

impl crate::readers::ACDataType for MagicDispelMultipleEnchantments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicDispelMultipleEnchantments::read(reader)
    }
}

impl MiscPortalStormBrewing {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let extent = read_f32(reader)?;

        Ok(Self {
            extent,
        })
    }
}

impl crate::readers::ACDataType for MiscPortalStormBrewing {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MiscPortalStormBrewing::read(reader)
    }
}

impl MiscPortalStormImminent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let extent = read_f32(reader)?;

        Ok(Self {
            extent,
        })
    }
}

impl crate::readers::ACDataType for MiscPortalStormImminent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MiscPortalStormImminent::read(reader)
    }
}

impl MiscPortalStorm {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for MiscPortalStorm {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MiscPortalStorm::read(reader)
    }
}

impl MiscPortalStormSubsided {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for MiscPortalStormSubsided {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MiscPortalStormSubsided::read(reader)
    }
}

impl CommunicationTransientString {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

impl crate::readers::ACDataType for CommunicationTransientString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationTransientString::read(reader)
    }
}

impl MagicPurgeBadEnchantments {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for MagicPurgeBadEnchantments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MagicPurgeBadEnchantments::read(reader)
    }
}

impl SocialSendClientContractTrackerTable {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contract_tracker = ContractTrackerTable::read(reader)?;

        Ok(Self {
            contract_tracker,
        })
    }
}

impl crate::readers::ACDataType for SocialSendClientContractTrackerTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialSendClientContractTrackerTable::read(reader)
    }
}

impl SocialSendClientContractTracker {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contract_tracker = ContractTracker::read(reader)?;
        let delete_contract = read_bool(reader)?;
        let set_as_display_contract = read_bool(reader)?;

        Ok(Self {
            contract_tracker,
            delete_contract,
            set_as_display_contract,
        })
    }
}

impl crate::readers::ACDataType for SocialSendClientContractTracker {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialSendClientContractTracker::read(reader)
    }
}

impl ItemServerSaysRemove {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for ItemServerSaysRemove {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemServerSaysRemove::read(reader)
    }
}

impl CharacterServerSaysAttemptFailed {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let reason = WeenieError::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            reason,
        })
    }
}

impl crate::readers::ACDataType for CharacterServerSaysAttemptFailed {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterServerSaysAttemptFailed::read(reader)
    }
}

impl ItemUpdateStackSize {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let amount = read_u32(reader)?;
        let new_value = read_u32(reader)?;

        Ok(Self {
            sequence,
            object_id,
            amount,
            new_value,
        })
    }
}

impl crate::readers::ACDataType for ItemUpdateStackSize {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemUpdateStackSize::read(reader)
    }
}

impl CombatHandlePlayerDeathEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let killed_id = ObjectId::read(reader)?;
        let killer_id = ObjectId::read(reader)?;

        Ok(Self {
            message,
            killed_id,
            killer_id,
        })
    }
}

impl crate::readers::ACDataType for CombatHandlePlayerDeathEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CombatHandlePlayerDeathEvent::read(reader)
    }
}

impl QualitiesPrivateRemoveIntEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let type_ = PropertyInt::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateRemoveIntEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateRemoveIntEvent::read(reader)
    }
}

impl QualitiesRemoveIntEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let type_ = PropertyInt::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            object_id,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesRemoveIntEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesRemoveIntEvent::read(reader)
    }
}

impl QualitiesPrivateRemoveBoolEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let type_ = PropertyBool::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateRemoveBoolEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateRemoveBoolEvent::read(reader)
    }
}

impl QualitiesRemoveBoolEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let type_ = PropertyBool::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            object_id,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesRemoveBoolEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesRemoveBoolEvent::read(reader)
    }
}

impl QualitiesPrivateRemoveFloatEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let type_ = PropertyFloat::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateRemoveFloatEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateRemoveFloatEvent::read(reader)
    }
}

impl QualitiesRemoveFloatEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let type_ = PropertyFloat::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            object_id,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesRemoveFloatEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesRemoveFloatEvent::read(reader)
    }
}

impl QualitiesPrivateRemoveStringEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let type_ = PropertyString::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateRemoveStringEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateRemoveStringEvent::read(reader)
    }
}

impl QualitiesRemoveStringEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let type_ = PropertyString::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            object_id,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesRemoveStringEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesRemoveStringEvent::read(reader)
    }
}

impl QualitiesPrivateRemoveDataIdEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let type_ = PropertyDataId::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateRemoveDataIdEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateRemoveDataIdEvent::read(reader)
    }
}

impl QualitiesRemoveDataIdEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let type_ = PropertyDataId::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            object_id,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesRemoveDataIdEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesRemoveDataIdEvent::read(reader)
    }
}

impl QualitiesPrivateRemoveInstanceIdEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let type_ = PropertyInstanceId::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateRemoveInstanceIdEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateRemoveInstanceIdEvent::read(reader)
    }
}

impl QualitiesRemoveInstanceIdEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let type_ = PropertyInstanceId::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            object_id,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesRemoveInstanceIdEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesRemoveInstanceIdEvent::read(reader)
    }
}

impl QualitiesPrivateRemovePositionEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let type_ = PropertyPosition::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateRemovePositionEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateRemovePositionEvent::read(reader)
    }
}

impl QualitiesRemovePositionEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let type_ = PropertyPosition::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            object_id,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesRemovePositionEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesRemovePositionEvent::read(reader)
    }
}

impl QualitiesPrivateRemoveInt64Event {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let type_ = PropertyInt64::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateRemoveInt64Event {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateRemoveInt64Event::read(reader)
    }
}

impl QualitiesRemoveInt64Event {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let type_ = PropertyInt64::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            object_id,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesRemoveInt64Event {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesRemoveInt64Event::read(reader)
    }
}

impl QualitiesPrivateUpdateInt {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = PropertyInt::try_from(read_u32(reader)?)?;
        let value = read_i32(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateInt {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateInt::read(reader)
    }
}

impl QualitiesUpdateInt {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = PropertyInt::try_from(read_u32(reader)?)?;
        let value = read_i32(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateInt {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateInt::read(reader)
    }
}

impl QualitiesPrivateUpdateInt64 {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = PropertyInt64::try_from(read_u32(reader)?)?;
        let value = read_i64(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateInt64 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateInt64::read(reader)
    }
}

impl QualitiesUpdateInt64 {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = PropertyInt64::try_from(read_u32(reader)?)?;
        let value = read_i64(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateInt64 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateInt64::read(reader)
    }
}

impl QualitiesPrivateUpdateBool {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = PropertyBool::try_from(read_u32(reader)?)?;
        let value = read_bool(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateBool {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateBool::read(reader)
    }
}

impl QualitiesUpdateBool {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = PropertyBool::try_from(read_u32(reader)?)?;
        let value = read_bool(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateBool {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateBool::read(reader)
    }
}

impl QualitiesPrivateUpdateFloat {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = PropertyFloat::try_from(read_u32(reader)?)?;
        let value = read_f32(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateFloat {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateFloat::read(reader)
    }
}

impl QualitiesUpdateFloat {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = PropertyFloat::try_from(read_u32(reader)?)?;
        let value = read_f32(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateFloat {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateFloat::read(reader)
    }
}

impl QualitiesPrivateUpdateString {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = PropertyString::try_from(read_u32(reader)?)?;
        let __alignment_marker___align_dword = align_dword(reader)?;
        let value = read_string(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateString::read(reader)
    }
}

impl QualitiesUpdateString {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = PropertyString::try_from(read_u32(reader)?)?;
        let __alignment_marker___align_dword = align_dword(reader)?;
        let value = read_string(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateString::read(reader)
    }
}

impl QualitiesPrivateUpdateDataId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = PropertyDataId::try_from(read_u32(reader)?)?;
        let value = read_u32(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateDataId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateDataId::read(reader)
    }
}

impl QualitiesUpdateDataId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = PropertyDataId::try_from(read_u32(reader)?)?;
        let value = read_u32(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateDataId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateDataId::read(reader)
    }
}

impl QualitiesPrivateUpdateInstanceId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = PropertyInstanceId::try_from(read_u32(reader)?)?;
        let value = ObjectId::read(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateInstanceId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateInstanceId::read(reader)
    }
}

impl QualitiesUpdateInstanceId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = PropertyInstanceId::try_from(read_u32(reader)?)?;
        let value = ObjectId::read(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateInstanceId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateInstanceId::read(reader)
    }
}

impl QualitiesPrivateUpdatePosition {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = PropertyPosition::try_from(read_u32(reader)?)?;
        let value = Position::read(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdatePosition {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdatePosition::read(reader)
    }
}

impl QualitiesUpdatePosition {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = PropertyPosition::try_from(read_u32(reader)?)?;
        let value = Position::read(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdatePosition {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdatePosition::read(reader)
    }
}

impl QualitiesPrivateUpdateSkill {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = SkillId::try_from(read_i32(reader)?)?;
        let value = Skill::read(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateSkill {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateSkill::read(reader)
    }
}

impl QualitiesUpdateSkill {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = SkillId::try_from(read_i32(reader)?)?;
        let value = Skill::read(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateSkill {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateSkill::read(reader)
    }
}

impl QualitiesPrivateUpdateSkillLevel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = SkillId::try_from(read_i32(reader)?)?;
        let value = read_u32(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateSkillLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateSkillLevel::read(reader)
    }
}

impl QualitiesUpdateSkillLevel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = SkillId::try_from(read_i32(reader)?)?;
        let value = read_u32(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateSkillLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateSkillLevel::read(reader)
    }
}

impl QualitiesPrivateUpdateSkillAC {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = SkillId::try_from(read_i32(reader)?)?;
        let value = SkillAdvancementClass::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateSkillAC {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateSkillAC::read(reader)
    }
}

impl QualitiesUpdateSkillAC {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = SkillId::try_from(read_i32(reader)?)?;
        let value = SkillAdvancementClass::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateSkillAC {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateSkillAC::read(reader)
    }
}

impl QualitiesPrivateUpdateAttribute {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = AttributeId::try_from(read_u32(reader)?)?;
        let value = AttributeInfo::read(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateAttribute {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateAttribute::read(reader)
    }
}

impl QualitiesUpdateAttribute {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = AttributeId::try_from(read_u32(reader)?)?;
        let value = AttributeInfo::read(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateAttribute {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateAttribute::read(reader)
    }
}

impl QualitiesPrivateUpdateAttributeLevel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = AttributeId::try_from(read_u32(reader)?)?;
        let value = read_u32(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateAttributeLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateAttributeLevel::read(reader)
    }
}

impl QualitiesUpdateAttributeLevel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = AttributeId::try_from(read_u32(reader)?)?;
        let value = read_u32(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateAttributeLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateAttributeLevel::read(reader)
    }
}

impl QualitiesPrivateUpdateAttribute2nd {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = VitalId::try_from(read_u32(reader)?)?;
        let value = SecondaryAttributeInfo::read(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateAttribute2nd {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateAttribute2nd::read(reader)
    }
}

impl QualitiesUpdateAttribute2nd {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = VitalId::try_from(read_u32(reader)?)?;
        let value = SecondaryAttributeInfo::read(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateAttribute2nd {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateAttribute2nd::read(reader)
    }
}

impl QualitiesPrivateUpdateAttribute2ndLevel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = CurVitalId::try_from(read_u32(reader)?)?;
        let value = read_u32(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateAttribute2ndLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateAttribute2ndLevel::read(reader)
    }
}

impl QualitiesUpdateAttribute2ndLevel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = CurVitalId::try_from(read_u32(reader)?)?;
        let value = read_u32(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateAttribute2ndLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateAttribute2ndLevel::read(reader)
    }
}

impl CommunicationHearEmote {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sender_id = ObjectId::read(reader)?;
        let sender_name = read_string(reader)?;
        let text = read_string(reader)?;

        Ok(Self {
            sender_id,
            sender_name,
            text,
        })
    }
}

impl crate::readers::ACDataType for CommunicationHearEmote {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationHearEmote::read(reader)
    }
}

impl CommunicationHearSoulEmote {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sender_id = ObjectId::read(reader)?;
        let sender_name = read_string(reader)?;
        let text = read_string(reader)?;

        Ok(Self {
            sender_id,
            sender_name,
            text,
        })
    }
}

impl crate::readers::ACDataType for CommunicationHearSoulEmote {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationHearSoulEmote::read(reader)
    }
}

impl CommunicationHearSpeech {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let sender_name = read_string(reader)?;
        let sender_id = ObjectId::read(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;

        Ok(Self {
            message,
            sender_name,
            sender_id,
            type_,
        })
    }
}

impl crate::readers::ACDataType for CommunicationHearSpeech {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationHearSpeech::read(reader)
    }
}

impl CommunicationHearRangedSpeech {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let sender_name = read_string(reader)?;
        let sender_id = ObjectId::read(reader)?;
        let range = read_f32(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;

        Ok(Self {
            message,
            sender_name,
            sender_id,
            range,
            type_,
        })
    }
}

impl crate::readers::ACDataType for CommunicationHearRangedSpeech {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationHearRangedSpeech::read(reader)
    }
}

impl AdminEnvirons {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let envrion_option = EnvrionChangeType::try_from(read_u32(reader)?)?;

        Ok(Self {
            envrion_option,
        })
    }
}

impl crate::readers::ACDataType for AdminEnvirons {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminEnvirons::read(reader)
    }
}

impl MovementPositionAndMovementEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let position = PositionPack::read(reader)?;
        let movement_data = MovementData::read(reader)?;

        Ok(Self {
            object_id,
            position,
            movement_data,
        })
    }
}

impl crate::readers::ACDataType for MovementPositionAndMovementEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementPositionAndMovementEvent::read(reader)
    }
}

impl ItemObjDescEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_description = ObjDesc::read(reader)?;
        let instance_sequence = read_u16(reader)?;
        let visual_desc_sequence = read_u16(reader)?;

        Ok(Self {
            object_id,
            object_description,
            instance_sequence,
            visual_desc_sequence,
        })
    }
}

impl crate::readers::ACDataType for ItemObjDescEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemObjDescEvent::read(reader)
    }
}

impl CharacterSetPlayerVisualDesc {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_description = ObjDesc::read(reader)?;

        Ok(Self {
            object_description,
        })
    }
}

impl crate::readers::ACDataType for CharacterSetPlayerVisualDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterSetPlayerVisualDesc::read(reader)
    }
}

impl CharacterCharGenVerificationResponseType1 {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;
        let name = read_string(reader)?;
        let seconds_until_deletion = read_u32(reader)?;
        let __alignment_marker___align_dword = align_dword(reader)?;

        Ok(Self {
            character_id,
            name,
            seconds_until_deletion,
        })
    }
}

impl CharacterCharGenVerificationResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let response_type = CharGenResponseType::try_from(read_u32(reader)?)?;

        match response_type {
            CharGenResponseType::OK => {
                let variant_struct = CharacterCharGenVerificationResponseType1::read(reader)?;
                Ok(Self::Type1(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "response_type", response_type).into()),
        }
    }
}

impl crate::readers::ACDataType for CharacterCharGenVerificationResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterCharGenVerificationResponse::read(reader)
    }
}

impl LoginAwaitingSubscriptionExpiration {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let seconds_remaining = read_u32(reader)?;

        Ok(Self {
            seconds_remaining,
        })
    }
}

impl crate::readers::ACDataType for LoginAwaitingSubscriptionExpiration {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginAwaitingSubscriptionExpiration::read(reader)
    }
}

impl LoginLogOffCharacter {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for LoginLogOffCharacter {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginLogOffCharacter::read(reader)
    }
}

impl CharacterCharacterDelete {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for CharacterCharacterDelete {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterCharacterDelete::read(reader)
    }
}

impl LoginLoginCharacterSet {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let status = read_u32(reader)?;
        let characters = read_packable_list::<CharacterIdentity>(reader)?;
        let deleted_characters = read_packable_list::<CharacterIdentity>(reader)?;
        let num_allowed_characters = read_u32(reader)?;
        let account = read_string(reader)?;
        let use_turbine_chat = read_bool(reader)?;
        let has_throneof_destiny = read_bool(reader)?;

        Ok(Self {
            status,
            characters,
            deleted_characters,
            num_allowed_characters,
            account,
            use_turbine_chat,
            has_throneof_destiny,
        })
    }
}

impl crate::readers::ACDataType for LoginLoginCharacterSet {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginLoginCharacterSet::read(reader)
    }
}

impl CharacterCharacterError {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let reason = CharacterErrorType::try_from(read_u32(reader)?)?;

        Ok(Self {
            reason,
        })
    }
}

impl crate::readers::ACDataType for CharacterCharacterError {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterCharacterError::read(reader)
    }
}

impl ItemCreateObject {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_description = ObjDesc::read(reader)?;
        let physics_description = PhysicsDesc::read(reader)?;
        let weenie_description = PublicWeenieDesc::read(reader)?;

        Ok(Self {
            object_id,
            object_description,
            physics_description,
            weenie_description,
        })
    }
}

impl crate::readers::ACDataType for ItemCreateObject {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemCreateObject::read(reader)
    }
}

impl LoginCreatePlayer {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;

        Ok(Self {
            character_id,
        })
    }
}

impl crate::readers::ACDataType for LoginCreatePlayer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginCreatePlayer::read(reader)
    }
}

impl ItemDeleteObject {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let __alignment_marker___align_dword = align_dword(reader)?;

        Ok(Self {
            object_id,
            object_instance_sequence,
        })
    }
}

impl crate::readers::ACDataType for ItemDeleteObject {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemDeleteObject::read(reader)
    }
}

impl MovementPositionEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let position = PositionPack::read(reader)?;

        Ok(Self {
            object_id,
            position,
        })
    }
}

impl crate::readers::ACDataType for MovementPositionEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementPositionEvent::read(reader)
    }
}

impl ItemParentEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let parent_id = ObjectId::read(reader)?;
        let child_id = ObjectId::read(reader)?;
        let location = ParentLocation::try_from(read_u32(reader)?)?;
        let placement = Placement::try_from(read_u32(reader)?)?;
        let object_instance_sequence = read_u16(reader)?;
        let child_position_sequence = read_u16(reader)?;

        Ok(Self {
            parent_id,
            child_id,
            location,
            placement,
            object_instance_sequence,
            child_position_sequence,
        })
    }
}

impl crate::readers::ACDataType for ItemParentEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemParentEvent::read(reader)
    }
}

impl InventoryPickupEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let object_position_sequence = read_u16(reader)?;

        Ok(Self {
            object_id,
            object_instance_sequence,
            object_position_sequence,
        })
    }
}

impl crate::readers::ACDataType for InventoryPickupEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryPickupEvent::read(reader)
    }
}

impl ItemSetState {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let new_state = PhysicsState::try_from(read_u32(reader)?)?;
        let object_instance_sequence = read_u16(reader)?;
        let object_state_sequence = read_u16(reader)?;

        Ok(Self {
            object_id,
            new_state,
            object_instance_sequence,
            object_state_sequence,
        })
    }
}

impl crate::readers::ACDataType for ItemSetState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemSetState::read(reader)
    }
}

impl MovementSetObjectMovement {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let movement_data = MovementData::read(reader)?;

        Ok(Self {
            object_id,
            object_instance_sequence,
            movement_data,
        })
    }
}

impl crate::readers::ACDataType for MovementSetObjectMovement {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementSetObjectMovement::read(reader)
    }
}

impl MovementVectorUpdate {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let velocity = Vector3::read(reader)?;
        let omega = Vector3::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let object_vector_sequence = read_u16(reader)?;

        Ok(Self {
            object_id,
            velocity,
            omega,
            object_instance_sequence,
            object_vector_sequence,
        })
    }
}

impl crate::readers::ACDataType for MovementVectorUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementVectorUpdate::read(reader)
    }
}

impl EffectsSoundEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let sound_type = Sound::try_from(read_i32(reader)?)?;
        let volume = read_f32(reader)?;

        Ok(Self {
            object_id,
            sound_type,
            volume,
        })
    }
}

impl crate::readers::ACDataType for EffectsSoundEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EffectsSoundEvent::read(reader)
    }
}

impl EffectsPlayerTeleport {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_teleport_sequence = read_u16(reader)?;
        let __alignment_marker___align_dword = align_dword(reader)?;

        Ok(Self {
            object_teleport_sequence,
        })
    }
}

impl crate::readers::ACDataType for EffectsPlayerTeleport {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EffectsPlayerTeleport::read(reader)
    }
}

impl EffectsPlayScriptId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let script_id = DataId::read(reader)?;

        Ok(Self {
            object_id,
            script_id,
        })
    }
}

impl crate::readers::ACDataType for EffectsPlayScriptId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EffectsPlayScriptId::read(reader)
    }
}

impl EffectsPlayScriptType {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let script_type = read_i32(reader)?;
        let speed = read_f32(reader)?;

        Ok(Self {
            object_id,
            script_type,
            speed,
        })
    }
}

impl crate::readers::ACDataType for EffectsPlayScriptType {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EffectsPlayScriptType::read(reader)
    }
}

impl LoginAccountBanned {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let banned_until = read_u32(reader)?;
        let text = read_string(reader)?;

        Ok(Self {
            banned_until,
            text,
        })
    }
}

impl crate::readers::ACDataType for LoginAccountBanned {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginAccountBanned::read(reader)
    }
}

impl AdminReceiveAccountData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown = read_u32(reader)?;
        let admin_account_data = read_packable_list::<AdminAccountData>(reader)?;

        Ok(Self {
            unknown,
            admin_account_data,
        })
    }
}

impl crate::readers::ACDataType for AdminReceiveAccountData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminReceiveAccountData::read(reader)
    }
}

impl AdminReceivePlayerData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown = read_i32(reader)?;
        let admin_player_data = read_packable_list::<AdminPlayerData>(reader)?;

        Ok(Self {
            unknown,
            admin_player_data,
        })
    }
}

impl crate::readers::ACDataType for AdminReceivePlayerData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminReceivePlayerData::read(reader)
    }
}

impl ItemUpdateObject {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_desc = ObjDesc::read(reader)?;
        let physics_desc = PhysicsDesc::read(reader)?;
        let weenie_desc = PublicWeenieDesc::read(reader)?;

        Ok(Self {
            object_id,
            object_desc,
            physics_desc,
            weenie_desc,
        })
    }
}

impl crate::readers::ACDataType for ItemUpdateObject {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemUpdateObject::read(reader)
    }
}

impl LoginAccountBooted {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let additional_reason_text = read_string(reader)?;
        let reason_text = read_string(reader)?;

        Ok(Self {
            additional_reason_text,
            reason_text,
        })
    }
}

impl crate::readers::ACDataType for LoginAccountBooted {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginAccountBooted::read(reader)
    }
}

impl CommunicationTurbineChat {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message_size = read_u32(reader)?;
        let type_ = TurbineChatType::try_from(read_u32(reader)?)?;
        let blob_dispatch_type = read_u32(reader)?;
        let target_type = read_i32(reader)?;
        let target_id = read_i32(reader)?;
        let transport_type = read_i32(reader)?;
        let transport_id = read_i32(reader)?;
        let cookie = read_i32(reader)?;
        let payload_size = read_u32(reader)?;

        match type_ {
            _ => Err(format!("Unknown {} value: {:?}", "type_", type_).into()),
        }
    }
}

impl crate::readers::ACDataType for CommunicationTurbineChat {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationTurbineChat::read(reader)
    }
}

impl LoginEnterGameServerReady {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for LoginEnterGameServerReady {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginEnterGameServerReady::read(reader)
    }
}

impl CommunicationTextboxString {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let text = read_string(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;

        Ok(Self {
            text,
            type_,
        })
    }
}

impl crate::readers::ACDataType for CommunicationTextboxString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationTextboxString::read(reader)
    }
}

impl LoginWorldInfo {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let connections = read_u32(reader)?;
        let max_connections = read_u32(reader)?;
        let world_name = read_string(reader)?;

        Ok(Self {
            connections,
            max_connections,
            world_name,
        })
    }
}

impl crate::readers::ACDataType for LoginWorldInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginWorldInfo::read(reader)
    }
}

impl DDDDataMessageType0 {
    pub fn read(reader: &mut dyn ACReader, dat_file: DatFileType, resource_type: uint, resource_id: DataId, iteration: uint, version: uint, data_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
        let data = read_vec::<u8>(reader, data_size as usize)?;

        Ok(Self {
            dat_file,
            resource_type,
            resource_id,
            iteration,
            version,
            data_size,
            data,
        })
    }
}

impl DDDDataMessageType1 {
    pub fn read(reader: &mut dyn ACReader, dat_file: DatFileType, resource_type: uint, resource_id: DataId, iteration: uint, version: uint, data_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
        let file_size = read_u32(reader)?;
        let data = read_vec::<u8>(reader, data_size as usize)?;

        Ok(Self {
            dat_file,
            resource_type,
            resource_id,
            iteration,
            version,
            data_size,
            file_size,
            data,
        })
    }
}

impl DDDDataMessage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let dat_file = DatFileType::try_from(read_i64(reader)?)?;
        let resource_type = read_u32(reader)?;
        let resource_id = DataId::read(reader)?;
        let iteration = read_u32(reader)?;
        let compression = CompressionType::try_from(read_u8(reader)?)?;
        let version = read_u32(reader)?;
        let data_size = read_u32(reader)?;

        match compression {
            CompressionType::None => {
                let variant_struct = DDDDataMessageType0::read(reader, dat_file, resource_type, resource_id, iteration, version, data_size)?;
                Ok(Self::Type0(variant_struct))
            },
            CompressionType::ZLib => {
                let variant_struct = DDDDataMessageType1::read(reader, dat_file, resource_type, resource_id, iteration, version, data_size)?;
                Ok(Self::Type1(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "compression", compression).into()),
        }
    }
}

impl crate::readers::ACDataType for DDDDataMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDDataMessage::read(reader)
    }
}

impl DDDErrorMessage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let resource_type = read_u32(reader)?;
        let resource_id = DataId::read(reader)?;
        let r_error = read_u32(reader)?;

        Ok(Self {
            resource_type,
            resource_id,
            r_error,
        })
    }
}

impl crate::readers::ACDataType for DDDErrorMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDErrorMessage::read(reader)
    }
}

impl DDDBeginDDDMessage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let data_expected = read_u32(reader)?;
        let revisions = read_packable_list::<DDDRevision>(reader)?;

        Ok(Self {
            data_expected,
            revisions,
        })
    }
}

impl crate::readers::ACDataType for DDDBeginDDDMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDBeginDDDMessage::read(reader)
    }
}

impl DDDInterrogationMessage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let servers_region = read_u32(reader)?;
        let name_rule_language = read_u32(reader)?;
        let product_id = read_u32(reader)?;
        let supported_languages = read_packable_list::<u32>(reader)?;

        Ok(Self {
            servers_region,
            name_rule_language,
            product_id,
            supported_languages,
        })
    }
}

impl crate::readers::ACDataType for DDDInterrogationMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDInterrogationMessage::read(reader)
    }
}

impl DDDOnEndDDD {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for DDDOnEndDDD {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDOnEndDDD::read(reader)
    }
}

