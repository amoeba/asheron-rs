// Binary readers for common types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl WString {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_wstring(reader)?))
    }
}

impl crate::readers::ACDataType for WString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WString::read(reader)
    }
}

impl PackedWORD {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for PackedWORD {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PackedWORD::read(reader)
    }
}

impl PackedDWORD {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for PackedDWORD {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PackedDWORD::read(reader)
    }
}

impl ObjectId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u32(reader)?))
    }
}

impl crate::readers::ACDataType for ObjectId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ObjectId::read(reader)
    }
}

impl LandcellId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u32(reader)?))
    }
}

impl crate::readers::ACDataType for LandcellId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LandcellId::read(reader)
    }
}

impl SpellId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u16(reader)?))
    }
}

impl crate::readers::ACDataType for SpellId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SpellId::read(reader)
    }
}

impl LayeredSpellId {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let id = SpellId::read(reader)?;
        let layer = read_u16(reader)?;

        Ok(Self {
            id,
            layer,
        })
    }
}

impl crate::readers::ACDataType for LayeredSpellId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LayeredSpellId::read(reader)
    }
}

impl Vector3 {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let x = read_f32(reader)?;
        let y = read_f32(reader)?;
        let z = read_f32(reader)?;

        Ok(Self {
            x,
            y,
            z,
        })
    }
}

impl crate::readers::ACDataType for Vector3 {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Vector3::read(reader)
    }
}

impl Quaternion {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let w = read_f32(reader)?;
        let x = read_f32(reader)?;
        let y = read_f32(reader)?;
        let z = read_f32(reader)?;

        Ok(Self {
            w,
            x,
            y,
            z,
        })
    }
}

impl crate::readers::ACDataType for Quaternion {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Quaternion::read(reader)
    }
}

impl Origin {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let landcell = LandcellId::read(reader)?;
        let location = Vector3::read(reader)?;

        Ok(Self {
            landcell,
            location,
        })
    }
}

impl crate::readers::ACDataType for Origin {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Origin::read(reader)
    }
}

impl Position {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let landcell = LandcellId::read(reader)?;
        let frame = Frame::read(reader)?;

        Ok(Self {
            landcell,
            frame,
        })
    }
}

impl crate::readers::ACDataType for Position {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Position::read(reader)
    }
}

impl Frame {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let origin = Vector3::read(reader)?;
        let orientation = Quaternion::read(reader)?;

        Ok(Self {
            origin,
            orientation,
        })
    }
}

impl crate::readers::ACDataType for Frame {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Frame::read(reader)
    }
}

impl ServerSwitchHeader {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u32(reader)?;
        let type_ = ServerSwitchType::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for ServerSwitchHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ServerSwitchHeader::read(reader)
    }
}

impl CICMDCommandHeader {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let command = read_u32(reader)?;
        let parameter = read_u32(reader)?;

        Ok(Self {
            command,
            parameter,
        })
    }
}

impl crate::readers::ACDataType for CICMDCommandHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CICMDCommandHeader::read(reader)
    }
}

impl FlowHeader {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = read_u32(reader)?;
        let interval = read_u16(reader)?;

        Ok(Self {
            bytes,
            interval,
        })
    }
}

impl crate::readers::ACDataType for FlowHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FlowHeader::read(reader)
    }
}

impl SocketAddress {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let family = read_i16(reader)?;
        let port = read_u16(reader)?;
        let address = read_u32(reader)?;
        let empty = read_u64(reader)?;

        Ok(Self {
            family,
            port,
            address,
            empty,
        })
    }
}

impl crate::readers::ACDataType for SocketAddress {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocketAddress::read(reader)
    }
}

impl LoginRequestHeaderType2 {
    pub fn read(reader: &mut dyn ACReader, client_version: string, length: uint, flags: AuthFlags, sequence: uint, account: string, account_to_login_as: string) -> Result<Self, Box<dyn std::error::Error>> {
        let password = read_wstring(reader).map(WString)?;

        Ok(Self {
            client_version,
            length,
            flags,
            sequence,
            account,
            account_to_login_as,
            password,
        })
    }
}

impl LoginRequestHeaderType40000002 {
    pub fn read(reader: &mut dyn ACReader, client_version: string, length: uint, flags: AuthFlags, sequence: uint, account: string, account_to_login_as: string) -> Result<Self, Box<dyn std::error::Error>> {
        let gls_ticket = read_string(reader)?;

        Ok(Self {
            client_version,
            length,
            flags,
            sequence,
            account,
            account_to_login_as,
            gls_ticket,
        })
    }
}

impl LoginRequestHeader {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let client_version = read_string(reader)?;
        let length = read_u32(reader)?;
        let auth_type = NetAuthType::try_from(read_u32(reader)?)?;
        let flags = AuthFlags::try_from(read_u32(reader)?)?;
        let sequence = read_u32(reader)?;
        let account = read_string(reader)?;
        let account_to_login_as = read_string(reader)?;

        match auth_type {
            NetAuthType::AccountPassword => {
                let variant_struct = LoginRequestHeaderType2::read(reader, client_version, length, flags, sequence, account, account_to_login_as)?;
                Ok(Self::Type2(variant_struct))
            },
            NetAuthType::GlsTicket => {
                let variant_struct = LoginRequestHeaderType40000002::read(reader, client_version, length, flags, sequence, account, account_to_login_as)?;
                Ok(Self::Type40000002(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "auth_type", auth_type).into()),
        }
    }
}

impl crate::readers::ACDataType for LoginRequestHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginRequestHeader::read(reader)
    }
}

impl ReferralHeader {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let cookie = read_u64(reader)?;
        let address = SocketAddress::read(reader)?;
        let id_server = read_u16(reader)?;
        let __alignment_marker_align_dword = align_dword(reader)?;
        let unknown = DWORD::read(reader)?;

        Ok(Self {
            cookie,
            address,
            id_server,
            unknown,
        })
    }
}

impl crate::readers::ACDataType for ReferralHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ReferralHeader::read(reader)
    }
}

impl ConnectRequestHeader {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let server_time = read_f64(reader)?;
        let cookie = read_u64(reader)?;
        let net_id = read_i32(reader)?;
        let outgoing_seed = read_u32(reader)?;
        let incoming_seed = read_u32(reader)?;
        let unknown = DWORD::read(reader)?;

        Ok(Self {
            server_time,
            cookie,
            net_id,
            outgoing_seed,
            incoming_seed,
            unknown,
        })
    }
}

impl crate::readers::ACDataType for ConnectRequestHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ConnectRequestHeader::read(reader)
    }
}

impl NetError {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let string_id = DataId::read(reader)?;
        let table_id = DataId::read(reader)?;

        Ok(Self {
            string_id,
            table_id,
        })
    }
}

impl crate::readers::ACDataType for NetError {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        NetError::read(reader)
    }
}

impl EchoResponseHeader {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let local_time = read_f32(reader)?;
        let holding_time = read_f32(reader)?;

        Ok(Self {
            local_time,
            holding_time,
        })
    }
}

impl crate::readers::ACDataType for EchoResponseHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EchoResponseHeader::read(reader)
    }
}

impl ACBaseQualities {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = ACBaseQualitiesFlags::try_from(read_u32(reader)?)?;
        let weenie_type = WeenieType::try_from(read_u32(reader)?)?;
        let mut int_properties = None;
        if (flags.clone() as u32 & ACBaseQualitiesFlags::PropertyInt as u32) != 0 {
            int_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyInt::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(read_i32(r)?)
        })?);
        }
        let mut int64_properties = None;
        if (flags.clone() as u32 & ACBaseQualitiesFlags::PropertyInt64 as u32) != 0 {
            int64_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyInt64::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(read_i64(r)?)
        })?);
        }
        let mut bool_properties = None;
        if (flags.clone() as u32 & ACBaseQualitiesFlags::PropertyBool as u32) != 0 {
            bool_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyBool::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(read_bool(r)?)
        })?);
        }
        let mut float_properties = None;
        if (flags.clone() as u32 & ACBaseQualitiesFlags::PropertyFloat as u32) != 0 {
            float_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyFloat::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(read_f64(r)?)
        })?);
        }
        let mut string_properties = None;
        if (flags.clone() as u32 & ACBaseQualitiesFlags::PropertyString as u32) != 0 {
            string_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyString::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(read_string(r)?)
        })?);
        }
        let mut data_properties = None;
        if (flags.clone() as u32 & ACBaseQualitiesFlags::PropertyDataId as u32) != 0 {
            data_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyDataId::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(DataId::read(r)?)
        })?);
        }
        let mut instance_properties = None;
        if (flags.clone() as u32 & ACBaseQualitiesFlags::PropertyInstanceId as u32) != 0 {
            instance_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyInstanceId::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(ObjectId::read(r)?)
        })?);
        }
        let mut position_properties = None;
        if (flags.clone() as u32 & ACBaseQualitiesFlags::PropertyPosition as u32) != 0 {
            position_properties = Some(read_packable_hash_table_with(reader, |r| {
            Ok(PropertyPosition::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(Position::read(r)?)
        })?);
        }

        Ok(Self {
            flags,
            weenie_type,
            int_properties,
            int64_properties,
            bool_properties,
            float_properties,
            string_properties,
            data_properties,
            instance_properties,
            position_properties,
        })
    }
}

impl crate::readers::ACDataType for ACBaseQualities {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ACBaseQualities::read(reader)
    }
}

impl ACQualities {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = ACQualitiesFlags::try_from(read_u32(reader)?)?;
        let has_health = read_bool(reader)?;
        let mut attributes = None;
        if (flags.clone() as u32 & ACQualitiesFlags::Attributes as u32) != 0 {
            attributes = Some(AttributeCache::read(reader)?);
        }
        let mut skills = None;
        if (flags.clone() as u32 & ACQualitiesFlags::Skills as u32) != 0 {
            skills = Some(read_packable_hash_table_with(reader, |r| {
            Ok(SkillId::try_from(read_i32(r)?)?)
        }, |r| {
            Ok(Skill::read(r)?)
        })?);
        }
        let mut body = None;
        if (flags.clone() as u32 & ACQualitiesFlags::Body as u32) != 0 {
            body = Some(Body::read(reader)?);
        }
        let mut spell_book = None;
        if (flags.clone() as u32 & ACQualitiesFlags::SpellBook as u32) != 0 {
            spell_book = Some(read_packable_hash_table_with(reader, |r| {
            Ok(LayeredSpellId::read(r)?)
        }, |r| {
            Ok(SpellBookPage::read(r)?)
        })?);
        }
        let mut enchantments = None;
        if (flags.clone() as u32 & ACQualitiesFlags::Enchantments as u32) != 0 {
            enchantments = Some(EnchantmentRegistry::read(reader)?);
        }
        let mut event_filter = None;
        if (flags.clone() as u32 & ACQualitiesFlags::EventFilter as u32) != 0 {
            event_filter = Some(EventFilter::read(reader)?);
        }
        let mut emotes = None;
        if (flags.clone() as u32 & ACQualitiesFlags::Emotes as u32) != 0 {
            emotes = Some(EmoteTable::read(reader)?);
        }
        let mut creation_profile = None;
        if (flags.clone() as u32 & ACQualitiesFlags::CreationProfile as u32) != 0 {
            creation_profile = Some(read_packable_list::<CreationProfile>(reader)?);
        }
        let mut page_data = None;
        if (flags.clone() as u32 & ACQualitiesFlags::PageData as u32) != 0 {
            page_data = Some(PageDataList::read(reader)?);
        }
        let mut generators = None;
        if (flags.clone() as u32 & ACQualitiesFlags::Generators as u32) != 0 {
            generators = Some(GeneratorTable::read(reader)?);
        }
        let mut generator_registry = None;
        if (flags.clone() as u32 & ACQualitiesFlags::GeneratorRegistry as u32) != 0 {
            generator_registry = Some(GeneratorRegistry::read(reader)?);
        }
        let mut generator_queue = None;
        if (flags.clone() as u32 & ACQualitiesFlags::GeneratorQueue as u32) != 0 {
            generator_queue = Some(GeneratorQueue::read(reader)?);
        }

        Ok(Self {
            flags,
            has_health,
            attributes,
            skills,
            body,
            spell_book,
            enchantments,
            event_filter,
            emotes,
            creation_profile,
            page_data,
            generators,
            generator_registry,
            generator_queue,
        })
    }
}

impl crate::readers::ACDataType for ACQualities {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ACQualities::read(reader)
    }
}

impl AttributeCache {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let mut strength = None;
        if (flags & 0x00000001) != 0 {
            strength = Some(AttributeInfo::read(reader)?);
        }
        let mut endurance = None;
        if (flags & 0x00000002) != 0 {
            endurance = Some(AttributeInfo::read(reader)?);
        }
        let mut quickness = None;
        if (flags & 0x00000004) != 0 {
            quickness = Some(AttributeInfo::read(reader)?);
        }
        let mut coordination = None;
        if (flags & 0x00000008) != 0 {
            coordination = Some(AttributeInfo::read(reader)?);
        }
        let mut focus = None;
        if (flags & 0x00000010) != 0 {
            focus = Some(AttributeInfo::read(reader)?);
        }
        let mut self_ = None;
        if (flags & 0x00000020) != 0 {
            self_ = Some(AttributeInfo::read(reader)?);
        }
        let mut health = None;
        if (flags & 0x00000040) != 0 {
            health = Some(SecondaryAttributeInfo::read(reader)?);
        }
        let mut stamina = None;
        if (flags & 0x00000080) != 0 {
            stamina = Some(SecondaryAttributeInfo::read(reader)?);
        }
        let mut mana = None;
        if (flags & 0x00000100) != 0 {
            mana = Some(SecondaryAttributeInfo::read(reader)?);
        }

        Ok(Self {
            flags,
            strength,
            endurance,
            quickness,
            coordination,
            focus,
            self_,
            health,
            stamina,
            mana,
        })
    }
}

impl crate::readers::ACDataType for AttributeCache {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AttributeCache::read(reader)
    }
}

impl AttributeInfo {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let points_raised = read_u32(reader)?;
        let innate_points = read_u32(reader)?;
        let experience_spent = read_u32(reader)?;

        Ok(Self {
            points_raised,
            innate_points,
            experience_spent,
        })
    }
}

impl crate::readers::ACDataType for AttributeInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AttributeInfo::read(reader)
    }
}

impl SecondaryAttributeInfo {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let attribute = AttributeInfo::read(reader)?;
        let current = read_u32(reader)?;

        Ok(Self {
            attribute,
            current,
        })
    }
}

impl crate::readers::ACDataType for SecondaryAttributeInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SecondaryAttributeInfo::read(reader)
    }
}

impl Skill {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let points_raised = read_u16(reader)?;
        let adjust_pp = read_u16(reader)?;
        let training_level = SkillAdvancementClass::try_from(read_u32(reader)?)?;
        let experience_spent = read_u32(reader)?;
        let innate_points = read_u32(reader)?;
        let resistance_of_last_check = read_u32(reader)?;
        let last_used_time = read_f64(reader)?;

        Ok(Self {
            points_raised,
            adjust_pp,
            training_level,
            experience_spent,
            innate_points,
            resistance_of_last_check,
            last_used_time,
        })
    }
}

impl crate::readers::ACDataType for Skill {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Skill::read(reader)
    }
}

impl Body {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let body_parts = read_packable_hash_table_with(reader, |r| {
            Ok(read_u32(r)?)
        }, |r| {
            Ok(BodyPart::read(r)?)
        })?;

        Ok(Self {
            body_parts,
        })
    }
}

impl crate::readers::ACDataType for Body {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Body::read(reader)
    }
}

impl BodyPart {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let has_bpsd = read_i32(reader)?;
        let damage_type = DamageType::try_from(read_u32(reader)?)?;
        let damage_val = read_i32(reader)?;
        let damage_var = read_i32(reader)?;
        let armor_cache = ArmorCache::read(reader)?;
        let bh = read_i32(reader)?;
        let mut bpsd = None;
        if (has_bpsd & 0x00000001) != 0 {
            bpsd = Some(BodyPartSelectionData::read(reader)?);
        }

        Ok(Self {
            has_bpsd,
            damage_type,
            damage_val,
            damage_var,
            armor_cache,
            bh,
            bpsd,
        })
    }
}

impl crate::readers::ACDataType for BodyPart {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        BodyPart::read(reader)
    }
}

impl ArmorCache {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let base_armor = read_i32(reader)?;
        let armor_vs_slash = read_i32(reader)?;
        let armor_vs_pierce = read_i32(reader)?;
        let armor_vs_bludgeon = read_i32(reader)?;
        let armor_vs_cold = read_i32(reader)?;
        let armor_vs_fire = read_i32(reader)?;
        let armor_vs_acid = read_i32(reader)?;
        let armor_vs_electric = read_i32(reader)?;
        let armor_vs_nether = read_i32(reader)?;

        Ok(Self {
            base_armor,
            armor_vs_slash,
            armor_vs_pierce,
            armor_vs_bludgeon,
            armor_vs_cold,
            armor_vs_fire,
            armor_vs_acid,
            armor_vs_electric,
            armor_vs_nether,
        })
    }
}

impl crate::readers::ACDataType for ArmorCache {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ArmorCache::read(reader)
    }
}

impl BodyPartSelectionData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let hlf = read_i32(reader)?;
        let mlf = read_i32(reader)?;
        let llf = read_i32(reader)?;
        let hrf = read_i32(reader)?;
        let mrf = read_i32(reader)?;
        let lrf = read_i32(reader)?;
        let hlb = read_i32(reader)?;
        let mlb = read_i32(reader)?;
        let llb = read_i32(reader)?;
        let hrb = read_i32(reader)?;
        let mrb = read_i32(reader)?;
        let lrb = read_i32(reader)?;

        Ok(Self {
            hlf,
            mlf,
            llf,
            hrf,
            mrf,
            lrf,
            hlb,
            mlb,
            llb,
            hrb,
            mrb,
            lrb,
        })
    }
}

impl crate::readers::ACDataType for BodyPartSelectionData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        BodyPartSelectionData::read(reader)
    }
}

impl SpellBookPage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let casting_likelihood = read_f32(reader)?;
        let mut casting_likelihood2 = None;
        let mut unknown = None;
        if casting_likelihood < 2.0 {
            casting_likelihood2 = Some(read_f32(reader)?);
            unknown = Some(read_i32(reader)?);
        }

        Ok(Self {
            casting_likelihood,
            casting_likelihood2,
            unknown,
        })
    }
}

impl crate::readers::ACDataType for SpellBookPage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SpellBookPage::read(reader)
    }
}

impl EnchantmentRegistry {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = EnchantmentRegistryFlags::try_from(read_u32(reader)?)?;
        let mut life_spells = None;
        if (flags.clone() as u32 & EnchantmentRegistryFlags::LifeSpells as u32) != 0 {
            life_spells = Some(read_packable_list::<Enchantment>(reader)?);
        }
        let mut creature_spells = None;
        if (flags.clone() as u32 & EnchantmentRegistryFlags::CreatureSpells as u32) != 0 {
            creature_spells = Some(read_packable_list::<Enchantment>(reader)?);
        }
        let mut vitae = None;
        if (flags.clone() as u32 & EnchantmentRegistryFlags::Vitae as u32) != 0 {
            vitae = Some(Enchantment::read(reader)?);
        }
        let mut cooldowns = None;
        if (flags.clone() as u32 & EnchantmentRegistryFlags::Cooldowns as u32) != 0 {
            cooldowns = Some(read_packable_list::<Enchantment>(reader)?);
        }

        Ok(Self {
            flags,
            life_spells,
            creature_spells,
            vitae,
            cooldowns,
        })
    }
}

impl crate::readers::ACDataType for EnchantmentRegistry {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EnchantmentRegistry::read(reader)
    }
}

impl Enchantment {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let id = LayeredSpellId::read(reader)?;
        let has_equipment_set = read_u16(reader)?;
        let spell_category = SpellCategory::try_from(read_u16(reader)?)?;
        let power_level = read_u32(reader)?;
        let start_time = read_f64(reader)?;
        let duration = read_f64(reader)?;
        let caster_id = ObjectId::read(reader)?;
        let degrade_modifier = read_f32(reader)?;
        let degrade_limit = read_f32(reader)?;
        let last_time_degraded = read_f64(reader)?;
        let stat_mod = StatMod::read(reader)?;
        let mut equipment_set = None;
        if has_equipment_set > 0 {
            equipment_set = Some(EquipmentSet::try_from(read_u32(reader)?)?);
        }

        Ok(Self {
            id,
            has_equipment_set,
            spell_category,
            power_level,
            start_time,
            duration,
            caster_id,
            degrade_modifier,
            degrade_limit,
            last_time_degraded,
            stat_mod,
            equipment_set,
        })
    }
}

impl crate::readers::ACDataType for Enchantment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Enchantment::read(reader)
    }
}

impl StatMod {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = EnchantmentTypeFlags::try_from(read_u32(reader)?)?;
        let key = read_u32(reader)?;
        let value = read_f32(reader)?;

        Ok(Self {
            type_,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for StatMod {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        StatMod::read(reader)
    }
}

impl EventFilter {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let events = read_packable_list::<u32>(reader)?;

        Ok(Self {
            events,
        })
    }
}

impl crate::readers::ACDataType for EventFilter {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EventFilter::read(reader)
    }
}

impl EmoteTable {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let emotes = read_packable_hash_table_with(reader, |r| {
            Ok(EmoteCategory::try_from(read_u32(r)?)?)
        }, |r| {
            Ok(EmoteSetList::read(r)?)
        })?;

        Ok(Self {
            emotes,
        })
    }
}

impl crate::readers::ACDataType for EmoteTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EmoteTable::read(reader)
    }
}

impl EmoteSetList {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let emotes = read_packable_list::<EmoteSet>(reader)?;

        Ok(Self {
            emotes,
        })
    }
}

impl crate::readers::ACDataType for EmoteSetList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EmoteSetList::read(reader)
    }
}

impl EmoteSetType1 {
    pub fn read(reader: &mut dyn ACReader, probability: float, emotes: PackableList<Emote>) -> Result<Self, Box<dyn std::error::Error>> {
        let class_id = read_u32(reader)?;

        Ok(Self {
            probability,
            emotes,
            class_id,
        })
    }
}

impl EmoteSetType2 {
    pub fn read(reader: &mut dyn ACReader, probability: float, emotes: PackableList<Emote>) -> Result<Self, Box<dyn std::error::Error>> {
        let vendor_type = read_u32(reader)?;

        Ok(Self {
            probability,
            emotes,
            vendor_type,
        })
    }
}

impl EmoteSetType5 {
    pub fn read(reader: &mut dyn ACReader, probability: float, emotes: PackableList<Emote>) -> Result<Self, Box<dyn std::error::Error>> {
        let style = read_u32(reader)?;
        let substyle = read_u32(reader)?;

        Ok(Self {
            probability,
            emotes,
            style,
            substyle,
        })
    }
}

impl EmoteSetTypeC {
    pub fn read(reader: &mut dyn ACReader, probability: float, emotes: PackableList<Emote>) -> Result<Self, Box<dyn std::error::Error>> {
        let quest = read_string(reader)?;

        Ok(Self {
            probability,
            emotes,
            quest,
        })
    }
}

impl EmoteSetTypeF {
    pub fn read(reader: &mut dyn ACReader, probability: float, emotes: PackableList<Emote>) -> Result<Self, Box<dyn std::error::Error>> {
        let min_health = read_f32(reader)?;
        let max_health = read_f32(reader)?;

        Ok(Self {
            probability,
            emotes,
            min_health,
            max_health,
        })
    }
}

impl EmoteSet {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let category = EmoteCategory::try_from(read_u32(reader)?)?;
        let probability = read_f32(reader)?;
        let emotes = read_packable_list::<Emote>(reader)?;

        match category {
            EmoteCategory::RefuseEmoteCategory | EmoteCategory::GiveEmoteCategory => {
                let variant_struct = EmoteSetType1::read(reader, probability, emotes)?;
                Ok(Self::Type1(variant_struct))
            },
            EmoteCategory::VendorEmoteCategory => {
                let variant_struct = EmoteSetType2::read(reader, probability, emotes)?;
                Ok(Self::Type2(variant_struct))
            },
            EmoteCategory::HeartBeatEmoteCategory => {
                let variant_struct = EmoteSetType5::read(reader, probability, emotes)?;
                Ok(Self::Type5(variant_struct))
            },
            EmoteCategory::QuestSuccessEmoteCategory | EmoteCategory::QuestFailureEmoteCategory | EmoteCategory::TestSuccessEmoteCategory | EmoteCategory::TestFailureEmoteCategory | EmoteCategory::EventSuccessEmoteCategory | EmoteCategory::EventFailureEmoteCategory | EmoteCategory::TestNoQualityEmoteCategory | EmoteCategory::QuestNoFellowEmoteCategory | EmoteCategory::TestNoFellowEmoteCategory | EmoteCategory::GotoSetEmoteCategory | EmoteCategory::NumFellowsSuccessEmoteCategory | EmoteCategory::NumFellowsFailureEmoteCategory | EmoteCategory::NumCharacterTitlesSuccessEmoteCategory | EmoteCategory::NumCharacterTitlesFailureEmoteCategory | EmoteCategory::ReceiveLocalSignalEmoteCategory | EmoteCategory::ReceiveTalkDirectEmoteCategory => {
                let variant_struct = EmoteSetTypeC::read(reader, probability, emotes)?;
                Ok(Self::TypeC(variant_struct))
            },
            EmoteCategory::WoundedTauntEmoteCategory => {
                let variant_struct = EmoteSetTypeF::read(reader, probability, emotes)?;
                Ok(Self::TypeF(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "category", category).into()),
        }
    }
}

impl crate::readers::ACDataType for EmoteSet {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EmoteSet::read(reader)
    }
}

impl EmoteType1 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
        })
    }
}

impl EmoteType2 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let amount64 = read_u64(reader)?;
        let hero_xp64 = read_u64(reader)?;

        Ok(Self {
            delay,
            extent,
            amount64,
            hero_xp64,
        })
    }
}

impl EmoteType3 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let c_profile = CreationProfile::read(reader)?;

        Ok(Self {
            delay,
            extent,
            c_profile,
        })
    }
}

impl EmoteType4 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let frame = Frame::read(reader)?;

        Ok(Self {
            delay,
            extent,
            frame,
        })
    }
}

impl EmoteType5 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let motion = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            motion,
        })
    }
}

impl EmoteType7 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let physics_script = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            physics_script,
        })
    }
}

impl EmoteType9 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let sound = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            sound,
        })
    }
}

impl EmoteTypeE {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let spell_id = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            spell_id,
        })
    }
}

impl EmoteType1C {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let amount = read_u32(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            amount,
            stat,
        })
    }
}

impl EmoteType1E {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let min = read_u32(reader)?;
        let max = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            min,
            max,
        })
    }
}

impl EmoteType20 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            amount,
        })
    }
}

impl EmoteType22 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let amount = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            amount,
        })
    }
}

impl EmoteType23 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            stat,
        })
    }
}

impl EmoteType24 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let min = read_u32(reader)?;
        let max = read_u32(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            min,
            max,
            stat,
        })
    }
}

impl EmoteType25 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let f_min = read_f64(reader)?;
        let f_max = read_f64(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            f_min,
            f_max,
            stat,
        })
    }
}

impl EmoteType26 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let test_string = read_string(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            test_string,
            stat,
        })
    }
}

impl EmoteType31 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let percent = read_f64(reader)?;
        let min64 = read_u64(reader)?;
        let max64 = read_u64(reader)?;

        Ok(Self {
            delay,
            extent,
            percent,
            min64,
            max64,
        })
    }
}

impl EmoteType32 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let stat = read_u32(reader)?;
        let percent = read_f64(reader)?;
        let min = read_u32(reader)?;
        let max = read_u32(reader)?;
        let display = read_bool(reader)?;

        Ok(Self {
            delay,
            extent,
            stat,
            percent,
            min,
            max,
            display,
        })
    }
}

impl EmoteType35 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let stat = read_u32(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            stat,
            amount,
        })
    }
}

impl EmoteType38 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let wealth_rating = read_i32(reader)?;
        let treasure_class = read_i32(reader)?;
        let treasure_type = read_i32(reader)?;

        Ok(Self {
            delay,
            extent,
            wealth_rating,
            treasure_class,
            treasure_type,
        })
    }
}

impl EmoteType3F {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let position = Position::read(reader)?;

        Ok(Self {
            delay,
            extent,
            position,
        })
    }
}

impl EmoteType4C {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let msg = read_string(reader)?;
        let c_profile = CreationProfile::read(reader)?;

        Ok(Self {
            delay,
            extent,
            msg,
            c_profile,
        })
    }
}

impl EmoteType6E {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            stat,
        })
    }
}

impl EmoteType70 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let amount64 = read_u64(reader)?;

        Ok(Self {
            delay,
            extent,
            amount64,
        })
    }
}

impl EmoteType72 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let min64 = read_u64(reader)?;
        let max64 = read_u64(reader)?;
        let stat = read_u32(reader)?;

        Ok(Self {
            delay,
            extent,
            message,
            min64,
            max64,
            stat,
        })
    }
}

impl EmoteType76 {
    pub fn read(reader: &mut dyn ACReader, delay: float, extent: float) -> Result<Self, Box<dyn std::error::Error>> {
        let stat = read_u32(reader)?;
        let percent = read_f64(reader)?;

        Ok(Self {
            delay,
            extent,
            stat,
            percent,
        })
    }
}

impl Emote {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = EmoteType::try_from(read_u32(reader)?)?;
        let delay = read_f32(reader)?;
        let extent = read_f32(reader)?;

        match type_ {
            EmoteType::ActEmoteType | EmoteType::SayEmoteType | EmoteType::TellEmoteType | EmoteType::TextDirectEmoteType | EmoteType::WorldBroadcastEmoteType | EmoteType::LocalBroadcastEmoteType | EmoteType::DirectBroadcastEmoteType | EmoteType::UpdateQuestEmoteType | EmoteType::InqQuestEmoteType | EmoteType::StampQuestEmoteType | EmoteType::StartEventEmoteType | EmoteType::StopEventEmoteType | EmoteType::BLogEmoteType | EmoteType::AdminSpamEmoteType | EmoteType::EraseQuestEmoteType | EmoteType::InqEventEmoteType | EmoteType::InqFellowQuestEmoteType | EmoteType::UpdateFellowQuestEmoteType | EmoteType::StampFellowQuestEmoteType | EmoteType::TellFellowEmoteType | EmoteType::FellowBroadcastEmoteType | EmoteType::GotoEmoteType | EmoteType::PopUpEmoteType | EmoteType::UpdateMyQuestEmoteType | EmoteType::InqMyQuestEmoteType | EmoteType::StampMyQuestEmoteType | EmoteType::EraseMyQuestEmoteType | EmoteType::LocalSignalEmoteType | EmoteType::InqContractsFullEmoteType => {
                let variant_struct = EmoteType1::read(reader, delay, extent)?;
                Ok(Self::Type1(variant_struct))
            },
            EmoteType::AwardXPEmoteType | EmoteType::AwardNoShareXPEmoteType => {
                let variant_struct = EmoteType2::read(reader, delay, extent)?;
                Ok(Self::Type2(variant_struct))
            },
            EmoteType::GiveEmoteType | EmoteType::TakeItemsEmoteType => {
                let variant_struct = EmoteType3::read(reader, delay, extent)?;
                Ok(Self::Type3(variant_struct))
            },
            EmoteType::MoveHomeEmoteType | EmoteType::MoveEmoteType | EmoteType::TurnEmoteType | EmoteType::MoveToPosEmoteType => {
                let variant_struct = EmoteType4::read(reader, delay, extent)?;
                Ok(Self::Type4(variant_struct))
            },
            EmoteType::MotionEmoteType | EmoteType::ForceMotionEmoteType => {
                let variant_struct = EmoteType5::read(reader, delay, extent)?;
                Ok(Self::Type5(variant_struct))
            },
            EmoteType::PhysScriptEmoteType => {
                let variant_struct = EmoteType7::read(reader, delay, extent)?;
                Ok(Self::Type7(variant_struct))
            },
            EmoteType::SoundEmoteType => {
                let variant_struct = EmoteType9::read(reader, delay, extent)?;
                Ok(Self::Type9(variant_struct))
            },
            EmoteType::CastSpellEmoteType | EmoteType::CastSpellInstantEmoteType | EmoteType::TeachSpellEmoteType | EmoteType::PetCastSpellOnOwnerEmoteType => {
                let variant_struct = EmoteTypeE::read(reader, delay, extent)?;
                Ok(Self::TypeE(variant_struct))
            },
            EmoteType::AwardSkillXPEmoteType | EmoteType::AwardSkillPointsEmoteType => {
                let variant_struct = EmoteType1C::read(reader, delay, extent)?;
                Ok(Self::Type1C(variant_struct))
            },
            EmoteType::InqQuestSolvesEmoteType | EmoteType::InqFellowNumEmoteType | EmoteType::InqNumCharacterTitlesEmoteType | EmoteType::InqMyQuestSolvesEmoteType => {
                let variant_struct = EmoteType1E::read(reader, delay, extent)?;
                Ok(Self::Type1E(variant_struct))
            },
            EmoteType::DecrementQuestEmoteType | EmoteType::IncrementQuestEmoteType | EmoteType::SetQuestCompletionsEmoteType | EmoteType::DecrementMyQuestEmoteType | EmoteType::IncrementMyQuestEmoteType | EmoteType::SetMyQuestCompletionsEmoteType | EmoteType::InqPackSpaceEmoteType | EmoteType::InqQuestBitsOnEmoteType | EmoteType::InqQuestBitsOffEmoteType | EmoteType::InqMyQuestBitsOnEmoteType | EmoteType::InqMyQuestBitsOffEmoteType | EmoteType::SetQuestBitsOnEmoteType | EmoteType::SetQuestBitsOffEmoteType | EmoteType::SetMyQuestBitsOnEmoteType | EmoteType::SetMyQuestBitsOffEmoteType => {
                let variant_struct = EmoteType20::read(reader, delay, extent)?;
                Ok(Self::Type20(variant_struct))
            },
            EmoteType::AddCharacterTitleEmoteType | EmoteType::AwardTrainingCreditsEmoteType | EmoteType::InflictVitaePenaltyEmoteType | EmoteType::RemoveVitaePenaltyEmoteType | EmoteType::SetAltRacialSkillsEmoteType | EmoteType::AddContractEmoteType | EmoteType::RemoveContractEmoteType => {
                let variant_struct = EmoteType22::read(reader, delay, extent)?;
                Ok(Self::Type22(variant_struct))
            },
            EmoteType::InqBoolStatEmoteType | EmoteType::InqSkillTrainedEmoteType | EmoteType::InqSkillSpecializedEmoteType => {
                let variant_struct = EmoteType23::read(reader, delay, extent)?;
                Ok(Self::Type23(variant_struct))
            },
            EmoteType::InqIntStatEmoteType | EmoteType::InqAttributeStatEmoteType | EmoteType::InqRawAttributeStatEmoteType | EmoteType::InqSecondaryAttributeStatEmoteType | EmoteType::InqRawSecondaryAttributeStatEmoteType | EmoteType::InqSkillStatEmoteType | EmoteType::InqRawSkillStatEmoteType => {
                let variant_struct = EmoteType24::read(reader, delay, extent)?;
                Ok(Self::Type24(variant_struct))
            },
            EmoteType::InqFloatStatEmoteType => {
                let variant_struct = EmoteType25::read(reader, delay, extent)?;
                Ok(Self::Type25(variant_struct))
            },
            EmoteType::InqStringStatEmoteType | EmoteType::InqYesNoEmoteType => {
                let variant_struct = EmoteType26::read(reader, delay, extent)?;
                Ok(Self::Type26(variant_struct))
            },
            EmoteType::AwardLevelProportionalXPEmoteType => {
                let variant_struct = EmoteType31::read(reader, delay, extent)?;
                Ok(Self::Type31(variant_struct))
            },
            EmoteType::AwardLevelProportionalSkillXPEmoteType => {
                let variant_struct = EmoteType32::read(reader, delay, extent)?;
                Ok(Self::Type32(variant_struct))
            },
            EmoteType::SetIntStatEmoteType | EmoteType::IncrementIntStatEmoteType | EmoteType::DecrementIntStatEmoteType | EmoteType::SetBoolStatEmoteType => {
                let variant_struct = EmoteType35::read(reader, delay, extent)?;
                Ok(Self::Type35(variant_struct))
            },
            EmoteType::CreateTreasureEmoteType => {
                let variant_struct = EmoteType38::read(reader, delay, extent)?;
                Ok(Self::Type38(variant_struct))
            },
            EmoteType::SetSanctuaryPositionEmoteType | EmoteType::TeleportTargetEmoteType | EmoteType::TeleportSelfEmoteType => {
                let variant_struct = EmoteType3F::read(reader, delay, extent)?;
                Ok(Self::Type3F(variant_struct))
            },
            EmoteType::InqOwnsItemsEmoteType => {
                let variant_struct = EmoteType4C::read(reader, delay, extent)?;
                Ok(Self::Type4C(variant_struct))
            },
            EmoteType::UntrainSkillEmoteType | EmoteType::SetInt64StatEmoteType => {
                let variant_struct = EmoteType6E::read(reader, delay, extent)?;
                Ok(Self::Type6E(variant_struct))
            },
            EmoteType::SpendLuminanceEmoteType | EmoteType::AwardLuminanceEmoteType => {
                let variant_struct = EmoteType70::read(reader, delay, extent)?;
                Ok(Self::Type70(variant_struct))
            },
            EmoteType::InqInt64StatEmoteType => {
                let variant_struct = EmoteType72::read(reader, delay, extent)?;
                Ok(Self::Type72(variant_struct))
            },
            EmoteType::SetFloatStatEmoteType => {
                let variant_struct = EmoteType76::read(reader, delay, extent)?;
                Ok(Self::Type76(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "type_", type_).into()),
        }
    }
}

impl crate::readers::ACDataType for Emote {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Emote::read(reader)
    }
}

impl CreationProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let weenie_class_id = read_u32(reader)?;
        let palette = read_u32(reader)?;
        let shade = read_f32(reader)?;
        let destination = read_u32(reader)?;
        let stack_size = read_i32(reader)?;
        let try_to_bond = read_bool(reader)?;

        Ok(Self {
            weenie_class_id,
            palette,
            shade,
            destination,
            stack_size,
            try_to_bond,
        })
    }
}

impl crate::readers::ACDataType for CreationProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CreationProfile::read(reader)
    }
}

impl PageDataList {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let max_num_pages = read_u32(reader)?;
        let max_num_chars_per_page = read_u32(reader)?;
        let pages = read_packable_list::<PageData>(reader)?;

        Ok(Self {
            max_num_pages,
            max_num_chars_per_page,
            pages,
        })
    }
}

impl crate::readers::ACDataType for PageDataList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PageDataList::read(reader)
    }
}

impl PageData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let author_id = ObjectId::read(reader)?;
        let author_name = read_string(reader)?;
        let author_account = read_string(reader)?;
        let version = read_u32(reader)?;
        let text_included = read_bool(reader)?;
        let ignore_author = read_bool(reader)?;
        let mut page_text = None;
        if text_included {
            page_text = Some(read_string(reader)?);
        }

        Ok(Self {
            author_id,
            author_name,
            author_account,
            version,
            text_included,
            ignore_author,
            page_text,
        })
    }
}

impl crate::readers::ACDataType for PageData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PageData::read(reader)
    }
}

impl BlobFragments {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u32(reader)?;
        let id = read_u32(reader)?;
        let count = read_u16(reader)?;
        let size = read_u16(reader)?;
        let body_size = (size - 16) as u16;
        let index = read_u16(reader)?;
        let group = FragmentGroup::try_from(read_u16(reader)?)?;
        let data = read_vec::<u8>(reader, body_size as usize)?;

        Ok(Self {
            sequence,
            id,
            count,
            size,
            index,
            group,
            data,
        })
    }
}

impl crate::readers::ACDataType for BlobFragments {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        BlobFragments::read(reader)
    }
}

impl GeneratorTable {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let generators = read_packable_list::<GeneratorProfile>(reader)?;

        Ok(Self {
            generators,
        })
    }
}

impl crate::readers::ACDataType for GeneratorTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GeneratorTable::read(reader)
    }
}

impl GeneratorProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let probability = read_f32(reader)?;
        let type_id = read_u32(reader)?;
        let delay = read_f64(reader)?;
        let init_create = read_u32(reader)?;
        let max_num = read_u32(reader)?;
        let when_create = read_u32(reader)?;
        let where_create = read_u32(reader)?;
        let stack_size = read_u32(reader)?;
        let ptid = read_u32(reader)?;
        let shade = read_f32(reader)?;
        let pos_val = Position::read(reader)?;
        let slot = read_u32(reader)?;

        Ok(Self {
            probability,
            type_id,
            delay,
            init_create,
            max_num,
            when_create,
            where_create,
            stack_size,
            ptid,
            shade,
            pos_val,
            slot,
        })
    }
}

impl crate::readers::ACDataType for GeneratorProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GeneratorProfile::read(reader)
    }
}

impl GeneratorRegistry {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = read_packable_hash_table_with(reader, |r| {
            Ok(read_u32(r)?)
        }, |r| {
            Ok(GeneratorRegistryNode::read(r)?)
        })?;

        Ok(Self {
            registry,
        })
    }
}

impl crate::readers::ACDataType for GeneratorRegistry {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GeneratorRegistry::read(reader)
    }
}

impl GeneratorRegistryNode {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let wcid_or_type = read_u32(reader)?;
        let ts = read_f64(reader)?;
        let treasure_type = read_u32(reader)?;
        let slot = read_u32(reader)?;
        let checkpointed = read_u32(reader)?;
        let shop = read_u32(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            wcid_or_type,
            ts,
            treasure_type,
            slot,
            checkpointed,
            shop,
            amount,
        })
    }
}

impl crate::readers::ACDataType for GeneratorRegistryNode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GeneratorRegistryNode::read(reader)
    }
}

impl GeneratorQueue {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let queue = read_packable_list::<GeneratorQueueNode>(reader)?;

        Ok(Self {
            queue,
        })
    }
}

impl crate::readers::ACDataType for GeneratorQueue {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GeneratorQueue::read(reader)
    }
}

impl GeneratorQueueNode {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let slot = read_u32(reader)?;
        let when = read_f64(reader)?;

        Ok(Self {
            slot,
            when,
        })
    }
}

impl crate::readers::ACDataType for GeneratorQueueNode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GeneratorQueueNode::read(reader)
    }
}

impl WindowPropertyType1000007F {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_j = read_u32(reader)?;
        let value_j = read_u64(reader)?;

        Ok(Self {
            unknown_j,
            value_j,
        })
    }
}

impl WindowPropertyType10000086 {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_i = read_u32(reader)?;
        let value_i = read_u32(reader)?;

        Ok(Self {
            unknown_i,
            value_i,
        })
    }
}

impl WindowPropertyType10000087 {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_h = read_u32(reader)?;
        let value_h = read_u32(reader)?;

        Ok(Self {
            unknown_h,
            value_h,
        })
    }
}

impl WindowPropertyType10000088 {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_f = read_u32(reader)?;
        let value_f = read_u32(reader)?;

        Ok(Self {
            unknown_f,
            value_f,
        })
    }
}

impl WindowPropertyType10000089 {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_e = read_u32(reader)?;
        let value_e = read_u32(reader)?;

        Ok(Self {
            unknown_e,
            value_e,
        })
    }
}

impl WindowPropertyType1000008A {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_d = read_u32(reader)?;
        let value_d = read_u8(reader)?;

        Ok(Self {
            unknown_d,
            value_d,
        })
    }
}

impl WindowPropertyType1000008D {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_c = read_u32(reader)?;
        let title_source = WindowPropertyType1000008DTitleSourceVariant::read(reader)?;

impl WindowPropertyType1000008DTitleSourceVariant {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let title_source = read_u8(reader)?;

        match title_source {
            0x00 => {
                let string_id = read_u32(reader)?;
                let file_id = read_u32(reader)?;
                return Ok(Self::Type0 {
                    string_id,
                    file_id,
                });
            },
            0x01 => {
                let value_a = read_wstring(reader).map(WString)?;
                return Ok(Self::Type1 {
                    value_a,
                });
            },
            _ => Err("Unknown nested switch value".into()),
        }
    }
}



        Ok(Self {
            unknown_c,
            title_source,
        })
    }
}

impl WindowProperty {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let key_a = read_u32(reader)?;

        match key_a {
            0x1000007F => {
                let variant_struct = WindowPropertyType1000007F::read(reader)?;
                Ok(Self::Type1000007F(variant_struct))
            },
            0x10000086 => {
                let variant_struct = WindowPropertyType10000086::read(reader)?;
                Ok(Self::Type10000086(variant_struct))
            },
            0x10000087 => {
                let variant_struct = WindowPropertyType10000087::read(reader)?;
                Ok(Self::Type10000087(variant_struct))
            },
            0x10000088 => {
                let variant_struct = WindowPropertyType10000088::read(reader)?;
                Ok(Self::Type10000088(variant_struct))
            },
            0x10000089 => {
                let variant_struct = WindowPropertyType10000089::read(reader)?;
                Ok(Self::Type10000089(variant_struct))
            },
            0x1000008A => {
                let variant_struct = WindowPropertyType1000008A::read(reader)?;
                Ok(Self::Type1000008A(variant_struct))
            },
            0x1000008D => {
                let variant_struct = WindowPropertyType1000008D::read(reader)?;
                Ok(Self::Type1000008D(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "key_a", key_a).into()),
        }
    }
}

impl crate::readers::ACDataType for WindowProperty {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WindowProperty::read(reader)
    }
}

impl WindowOptionType1000008B {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_b = read_u8(reader)?;
        let property_count = read_u8(reader)?;
        let properties = read_vec::<WindowProperty>(reader, property_count as usize)?;

        Ok(Self {
            unknown_b,
            property_count,
            properties,
        })
    }
}

impl WindowOption {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_a = read_u32(reader)?;

        match type_a {
            0x1000008B => {
                let variant_struct = WindowOptionType1000008B::read(reader)?;
                Ok(Self::Type1000008B(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "type_a", type_a).into()),
        }
    }
}

impl crate::readers::ACDataType for WindowOption {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WindowOption::read(reader)
    }
}

impl OptionPropertyType10000080 {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_l = read_u32(reader)?;
        let inactive_opacity = read_f32(reader)?;

        Ok(Self {
            unknown_l,
            inactive_opacity,
        })
    }
}

impl OptionPropertyType10000081 {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_k = read_u32(reader)?;
        let active_opacity = read_f32(reader)?;

        Ok(Self {
            unknown_k,
            active_opacity,
        })
    }
}

impl OptionPropertyType1000008C {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown_a = read_u32(reader)?;
        let window_options = read_packable_list::<WindowOption>(reader)?;

        Ok(Self {
            unknown_a,
            window_options,
        })
    }
}

impl OptionProperty {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = read_u32(reader)?;

        match type_ {
            0x10000080 => {
                let variant_struct = OptionPropertyType10000080::read(reader)?;
                Ok(Self::Type10000080(variant_struct))
            },
            0x10000081 => {
                let variant_struct = OptionPropertyType10000081::read(reader)?;
                Ok(Self::Type10000081(variant_struct))
            },
            0x1000008C => {
                let variant_struct = OptionPropertyType1000008C::read(reader)?;
                Ok(Self::Type1000008C(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "type_", type_).into()),
        }
    }
}

impl crate::readers::ACDataType for OptionProperty {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        OptionProperty::read(reader)
    }
}

impl GameplayOptions {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let size = read_u32(reader)?;
        let unknown200_2 = read_u8(reader)?;
        let option_property_count = read_u8(reader)?;
        let option_properties = read_vec::<OptionProperty>(reader, option_property_count as usize)?;
        let __alignment_marker_align_dword = align_dword(reader)?;

        Ok(Self {
            size,
            unknown200_2,
            option_property_count,
            option_properties,
        })
    }
}

impl crate::readers::ACDataType for GameplayOptions {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameplayOptions::read(reader)
    }
}

impl PlayerModule {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let options = CharacterOptions1::try_from(read_u32(reader)?)?;
        let mut shortcuts = None;
        if (flags & 0x00000001) != 0 {
            shortcuts = Some(read_packable_list::<ShortCutData>(reader)?);
        }
        let tab1_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab2_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab3_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab4_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab5_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab6_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab7_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let tab8_spells = read_packable_list::<LayeredSpellId>(reader)?;
        let mut fill_comps = None;
        if (flags & 0x00000008) != 0 {
            fill_comps = Some(read_packable_hash_table_with(reader, |r| {
            Ok(read_u32(r)?)
        }, |r| {
            Ok(read_u32(r)?)
        })?);
        }
        let mut spell_book_filters = None;
        if (flags & 0x00000020) != 0 {
            spell_book_filters = Some(read_u32(reader)?);
        }
        let mut option_flags = None;
        if (flags & 0x00000040) != 0 {
            option_flags = Some(read_u32(reader)?);
        }
        let mut unknown100_1 = None;
        let mut option_strings = None;
        if (flags & 0x00000100) != 0 {
            unknown100_1 = Some(read_u32(reader)?);
            option_strings = Some(read_packable_hash_table_with(reader, |r| {
            Ok(read_u32(r)?)
        }, |r| {
            Ok(read_string(r)?)
        })?);
        }
        let mut gameplay_options = None;
        if (flags & 0x00000200) != 0 {
            gameplay_options = Some(GameplayOptions::read(reader)?);
        }

        Ok(Self {
            flags,
            options,
            shortcuts,
            tab1_spells,
            tab2_spells,
            tab3_spells,
            tab4_spells,
            tab5_spells,
            tab6_spells,
            tab7_spells,
            tab8_spells,
            fill_comps,
            spell_book_filters,
            option_flags,
            unknown100_1,
            option_strings,
            gameplay_options,
        })
    }
}

impl crate::readers::ACDataType for PlayerModule {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PlayerModule::read(reader)
    }
}

impl ShortCutManager {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let shortcuts = read_packable_list::<ShortCutData>(reader)?;

        Ok(Self {
            shortcuts,
        })
    }
}

impl crate::readers::ACDataType for ShortCutManager {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ShortCutManager::read(reader)
    }
}

impl ShortCutData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let index = read_u32(reader)?;
        let object_id = ObjectId::read(reader)?;
        let spell_id = LayeredSpellId::read(reader)?;

        Ok(Self {
            index,
            object_id,
            spell_id,
        })
    }
}

impl crate::readers::ACDataType for ShortCutData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ShortCutData::read(reader)
    }
}

impl SpellTab {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let spells = read_packable_list::<LayeredSpellId>(reader)?;

        Ok(Self {
            spells,
        })
    }
}

impl crate::readers::ACDataType for SpellTab {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SpellTab::read(reader)
    }
}

impl ContentProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let container_type = ContainerProperties::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            container_type,
        })
    }
}

impl crate::readers::ACDataType for ContentProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ContentProfile::read(reader)
    }
}

impl InventoryPlacement {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let location = EquipMask::try_from(read_u32(reader)?)?;
        let priority = CoverageMask::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            location,
            priority,
        })
    }
}

impl crate::readers::ACDataType for InventoryPlacement {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryPlacement::read(reader)
    }
}

impl AllegianceProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let total_members = read_u32(reader)?;
        let total_vassals = read_u32(reader)?;
        let hierarchy = AllegianceHierarchy::read(reader)?;

        Ok(Self {
            total_members,
            total_vassals,
            hierarchy,
        })
    }
}

impl crate::readers::ACDataType for AllegianceProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceProfile::read(reader)
    }
}

impl AllegianceRecord {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let tree_parent = ObjectId::read(reader)?;
        let allegiance_data = AllegianceData::read(reader)?;

        Ok(Self {
            tree_parent,
            allegiance_data,
        })
    }
}

impl crate::readers::ACDataType for AllegianceRecord {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceRecord::read(reader)
    }
}

impl AllegianceHierarchy {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let record_count = read_u16(reader)?;
        let old_version = read_u16(reader)?;
        let officers = read_phash_table::<ObjectId, AllegianceOfficerLevel>(reader)?;
        let officer_titles = read_packable_list::<String>(reader)?;
        let monarch_broadcast_time = read_u32(reader)?;
        let monarch_broadcasts_today = read_u32(reader)?;
        let spokes_broadcast_time = read_u32(reader)?;
        let spokes_broadcasts_today = read_u32(reader)?;
        let motd = read_string(reader)?;
        let motd_set_by = read_string(reader)?;
        let chat_room_id = read_u32(reader)?;
        let bindpoint = Position::read(reader)?;
        let allegiance_name = read_string(reader)?;
        let name_last_set_time = read_u32(reader)?;
        let is_locked = read_bool(reader)?;
        let approved_vassal = read_i32(reader)?;
        let mut monarch_data = None;
        if record_count > 0 {
            monarch_data = Some(AllegianceData::read(reader)?);
        }
        let records = read_vec::<AllegianceRecord>(reader, record_count as usize - 1)?;

        Ok(Self {
            record_count,
            old_version,
            officers,
            officer_titles,
            monarch_broadcast_time,
            monarch_broadcasts_today,
            spokes_broadcast_time,
            spokes_broadcasts_today,
            motd,
            motd_set_by,
            chat_room_id,
            bindpoint,
            allegiance_name,
            name_last_set_time,
            is_locked,
            approved_vassal,
            monarch_data,
            records,
        })
    }
}

impl crate::readers::ACDataType for AllegianceHierarchy {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceHierarchy::read(reader)
    }
}

impl AllegianceData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;
        let xp_cached = read_u32(reader)?;
        let xp_tithed = read_u32(reader)?;
        let flags = read_u32(reader)?;
        let gender = Gender::try_from(read_u8(reader)?)?;
        let heritage = HeritageGroup::try_from(read_u8(reader)?)?;
        let rank = read_u16(reader)?;
        let mut level = None;
        if (flags & 0x8) != 0 {
            level = Some(read_u32(reader)?);
        }
        let loyalty = read_u16(reader)?;
        let leadership = read_u16(reader)?;
        let mut allegiance_age = None;
        let time_online;
        if flags == 0x4 {
            time_online = Some(read_u64(reader)?);
        } else {
            allegiance_age = Some(read_u32(reader)?);
            time_online = Some((read_u32(reader))? as u64);
        }
        let name = read_string(reader)?;

        Ok(Self {
            character_id,
            xp_cached,
            xp_tithed,
            flags,
            gender,
            heritage,
            rank,
            level,
            loyalty,
            leadership,
            allegiance_age,
            time_online,
            name,
        })
    }
}

impl crate::readers::ACDataType for AllegianceData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceData::read(reader)
    }
}

impl FriendData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let friend_id = ObjectId::read(reader)?;
        let online = read_bool(reader)?;
        let appear_offline = read_bool(reader)?;
        let name = read_string(reader)?;
        let out_friends = read_packable_list::<ObjectId>(reader)?;
        let in_friends = read_packable_list::<ObjectId>(reader)?;

        Ok(Self {
            friend_id,
            online,
            appear_offline,
            name,
            out_friends,
            in_friends,
        })
    }
}

impl crate::readers::ACDataType for FriendData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FriendData::read(reader)
    }
}

impl ItemProfileTypeNeg1 {
    pub fn read(reader: &mut dyn ACReader, packed_amount: uint, object_id: ObjectId) -> Result<Self, Box<dyn std::error::Error>> {
        let weenie_description = PublicWeenieDesc::read(reader)?;

        Ok(Self {
            packed_amount,
            object_id,
            weenie_description,
        })
    }
}

impl ItemProfileType1 {
    pub fn read(reader: &mut dyn ACReader, packed_amount: uint, object_id: ObjectId) -> Result<Self, Box<dyn std::error::Error>> {
        let old_weenie_description = OldPublicWeenieDesc::read(reader)?;

        Ok(Self {
            packed_amount,
            object_id,
            old_weenie_description,
        })
    }
}

impl ItemProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let packed_amount = read_u32(reader)?;
        let amount = (packed_amount & 0x_ffffff) as i32;
        let pwd_type = (packed_amount >> 24) as i32;
        let object_id = ObjectId::read(reader)?;

        match pwd_type {
            -1 => {
                let variant_struct = ItemProfileTypeNeg1::read(reader, packed_amount, object_id)?;
                Ok(Self::TypeNeg1(variant_struct))
            },
            0x01 => {
                let variant_struct = ItemProfileType1::read(reader, packed_amount, object_id)?;
                Ok(Self::Type1(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "pwd_type", pwd_type).into()),
        }
    }
}

impl crate::readers::ACDataType for ItemProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemProfile::read(reader)
    }
}

impl PublicWeenieDesc {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let header = read_u32(reader)?;
        let name = read_string(reader)?;
        let weenie_class_id = PackedDWORD::read(reader)?;
        let icon = PackedDWORD::read(reader)?;
        let type_ = ItemType::try_from(read_u32(reader)?)?;
        let behavior = ObjectDescriptionFlag::try_from(read_u32(reader)?)?;
        let __alignment_marker_align_dword = align_dword(reader)?;
        let mut header2 = None;
        if (behavior.clone() as u32 & 0x04000000) != 0 {
            header2 = Some(read_u32(reader)?);
        }
        let mut plural_name = None;
        if (header & 0x00000001) != 0 {
            plural_name = Some(read_string(reader)?);
        }
        let mut items_capacity = None;
        if (header & 0x00000002) != 0 {
            items_capacity = Some(read_u8(reader)?);
        }
        let mut container_capacity = None;
        if (header & 0x00000004) != 0 {
            container_capacity = Some(read_u8(reader)?);
        }
        let mut ammunition_type = None;
        if (header & 0x00000100) != 0 {
            ammunition_type = Some(AmmoType::try_from(read_u16(reader)?)?);
        }
        let mut value = None;
        if (header & 0x00000008) != 0 {
            value = Some(read_u32(reader)?);
        }
        let mut useability = None;
        if (header & 0x00000010) != 0 {
            useability = Some(Usable::try_from(read_u32(reader)?)?);
        }
        let mut use_radius = None;
        if (header & 0x00000020) != 0 {
            use_radius = Some(read_f32(reader)?);
        }
        let mut target_type = None;
        if (header & 0x00080000) != 0 {
            target_type = Some(ItemType::try_from(read_u32(reader)?)?);
        }
        let mut effects = None;
        if (header & 0x00000080) != 0 {
            effects = Some(IconHighlight::try_from(read_u32(reader)?)?);
        }
        let mut combat_use = None;
        if (header & 0x00000200) != 0 {
            combat_use = Some(WieldType::try_from(read_u8(reader)?)?);
        }
        let mut structure = None;
        if (header & 0x00000400) != 0 {
            structure = Some(read_u16(reader)?);
        }
        let mut max_structure = None;
        if (header & 0x00000800) != 0 {
            max_structure = Some(read_u16(reader)?);
        }
        let mut stack_size = None;
        if (header & 0x00001000) != 0 {
            stack_size = Some(read_u16(reader)?);
        }
        let mut max_stack_size = None;
        if (header & 0x00002000) != 0 {
            max_stack_size = Some(read_u16(reader)?);
        }
        let mut container_id = None;
        if (header & 0x00004000) != 0 {
            container_id = Some(ObjectId::read(reader)?);
        }
        let mut wielder_id = None;
        if (header & 0x00008000) != 0 {
            wielder_id = Some(ObjectId::read(reader)?);
        }
        let mut valid_slots = None;
        if (header & 0x00010000) != 0 {
            valid_slots = Some(EquipMask::try_from(read_u32(reader)?)?);
        }
        let mut slot = None;
        if (header & 0x00020000) != 0 {
            slot = Some(EquipMask::try_from(read_u32(reader)?)?);
        }
        let mut priority = None;
        if (header & 0x00040000) != 0 {
            priority = Some(CoverageMask::try_from(read_u32(reader)?)?);
        }
        let mut blip_color = None;
        if (header & 0x00100000) != 0 {
            blip_color = Some(RadarColor::try_from(read_u8(reader)?)?);
        }
        let mut radar_enum = None;
        if (header & 0x00800000) != 0 {
            radar_enum = Some(RadarBehavior::try_from(read_u8(reader)?)?);
        }
        let mut physics_script = None;
        if (header & 0x08000000) != 0 {
            physics_script = Some(read_u16(reader)?);
        }
        let mut workmanship = None;
        if (header & 0x01000000) != 0 {
            workmanship = Some(read_f32(reader)?);
        }
        let mut burden = None;
        if (header & 0x00200000) != 0 {
            burden = Some(read_u16(reader)?);
        }
        let mut spell_id = None;
        if (header & 0x00400000) != 0 {
            spell_id = Some(SpellId::read(reader)?);
        }
        let mut owner_id = None;
        if (header & 0x02000000) != 0 {
            owner_id = Some(ObjectId::read(reader)?);
        }
        let mut restrictions = None;
        if (header & 0x04000000) != 0 {
            restrictions = Some(RestrictionDB::read(reader)?);
        }
        let mut hook_item_types = None;
        if (header & 0x20000000) != 0 {
            hook_item_types = Some(HookType::try_from(read_u16(reader)?)?);
        }
        let mut monarch_id = None;
        if (header & 0x00000040) != 0 {
            monarch_id = Some(ObjectId::read(reader)?);
        }
        let mut hook_type = None;
        if (header & 0x10000000) != 0 {
            hook_type = Some(HookType::try_from(read_u16(reader)?)?);
        }
        let mut icon_overlay = None;
        if (header & 0x40000000) != 0 {
            icon_overlay = Some(PackedDWORD::read(reader)?);
        }
        let mut icon_underlay = None;
        if (header2.unwrap_or(0) & 0x00000001) != 0 {
            icon_underlay = Some(PackedDWORD::read(reader)?);
        }
        let mut material = None;
        if (header & 0x80000000) != 0 {
            material = Some(MaterialType::try_from(read_u32(reader)?)?);
        }
        let mut cooldown_id = None;
        if (header2.unwrap_or(0) & 0x00000002) != 0 {
            cooldown_id = Some(read_u32(reader)?);
        }
        let mut cooldown_duration = None;
        if (header2.unwrap_or(0) & 0x00000004) != 0 {
            cooldown_duration = Some(read_u64(reader)?);
        }
        let mut pet_owner_id = None;
        if (header2.unwrap_or(0) & 0x00000008) != 0 {
            pet_owner_id = Some(ObjectId::read(reader)?);
        }
        let __alignment_marker_align_dword = align_dword(reader)?;

        Ok(Self {
            header,
            name,
            weenie_class_id,
            icon,
            type_,
            behavior,
            header2,
            plural_name,
            items_capacity,
            container_capacity,
            ammunition_type,
            value,
            useability,
            use_radius,
            target_type,
            effects,
            combat_use,
            structure,
            max_structure,
            stack_size,
            max_stack_size,
            container_id,
            wielder_id,
            valid_slots,
            slot,
            priority,
            blip_color,
            radar_enum,
            physics_script,
            workmanship,
            burden,
            spell_id,
            owner_id,
            restrictions,
            hook_item_types,
            monarch_id,
            hook_type,
            icon_overlay,
            icon_underlay,
            material,
            cooldown_id,
            cooldown_duration,
            pet_owner_id,
        })
    }
}

impl crate::readers::ACDataType for PublicWeenieDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PublicWeenieDesc::read(reader)
    }
}

impl RestrictionDB {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let version = read_u32(reader)?;
        let flags = read_u32(reader)?;
        let monarch_id = ObjectId::read(reader)?;
        let permissions = read_phash_table::<ObjectId, u32>(reader)?;

        Ok(Self {
            version,
            flags,
            monarch_id,
            permissions,
        })
    }
}

impl crate::readers::ACDataType for RestrictionDB {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        RestrictionDB::read(reader)
    }
}

impl OldPublicWeenieDesc {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let header = read_u32(reader)?;
        let name = read_string(reader)?;
        let weenie_class_id = PackedDWORD::read(reader)?;
        let icon = PackedDWORD::read(reader)?;
        let type_ = ItemType::try_from(read_u32(reader)?)?;
        let bitfield = ObjectDescriptionFlag::try_from(read_u32(reader)?)?;
        let mut plural_name = None;
        if (header & 0x00000001) != 0 {
            plural_name = Some(read_string(reader)?);
        }
        let mut items_capacity = None;
        if (header & 0x00000002) != 0 {
            items_capacity = Some(read_u8(reader)?);
        }
        let mut container_capacity = None;
        if (header & 0x00000004) != 0 {
            container_capacity = Some(read_u8(reader)?);
        }
        let mut value = None;
        if (header & 0x00000008) != 0 {
            value = Some(read_u32(reader)?);
        }
        let mut useability = None;
        if (header & 0x00000010) != 0 {
            useability = Some(Usable::try_from(read_u32(reader)?)?);
        }
        let mut use_radius = None;
        if (header & 0x00000020) != 0 {
            use_radius = Some(read_f32(reader)?);
        }
        let mut t_target_type = None;
        if (header & 0x00080000) != 0 {
            t_target_type = Some(ItemType::try_from(read_u32(reader)?)?);
        }
        let mut effects = None;
        if (header & 0x00000080) != 0 {
            effects = Some(IconHighlight::try_from(read_u32(reader)?)?);
        }
        let mut ammunition_type = None;
        if (header & 0x00000100) != 0 {
            ammunition_type = Some(AmmoType::try_from(read_u16(reader)?)?);
        }
        let mut combat_use = None;
        if (header & 0x00000200) != 0 {
            combat_use = Some(WieldType::try_from(read_u8(reader)?)?);
        }
        let mut structure = None;
        if (header & 0x00000400) != 0 {
            structure = Some(read_u16(reader)?);
        }
        let mut max_structure = None;
        if (header & 0x00000800) != 0 {
            max_structure = Some(read_u16(reader)?);
        }
        let mut stack_size = None;
        if (header & 0x00001000) != 0 {
            stack_size = Some(read_u16(reader)?);
        }
        let mut max_stack_size = None;
        if (header & 0x00002000) != 0 {
            max_stack_size = Some(read_u16(reader)?);
        }
        let mut container_id = None;
        if (header & 0x00004000) != 0 {
            container_id = Some(ObjectId::read(reader)?);
        }
        let mut wielder_id = None;
        if (header & 0x00008000) != 0 {
            wielder_id = Some(ObjectId::read(reader)?);
        }
        let mut valid_slots = None;
        if (header & 0x00010000) != 0 {
            valid_slots = Some(EquipMask::try_from(read_u32(reader)?)?);
        }
        let mut slots = None;
        if (header & 0x00020000) != 0 {
            slots = Some(EquipMask::try_from(read_u32(reader)?)?);
        }
        let mut priority = None;
        if (header & 0x00040000) != 0 {
            priority = Some(CoverageMask::try_from(read_u32(reader)?)?);
        }
        let mut blip_color = None;
        if (header & 0x00100000) != 0 {
            blip_color = Some(RadarColor::try_from(read_u8(reader)?)?);
        }
        let mut radar_enum = None;
        if (header & 0x00800000) != 0 {
            radar_enum = Some(RadarBehavior::try_from(read_u8(reader)?)?);
        }
        let mut obvious_distance = None;
        if (header & 0x01000000) != 0 {
            obvious_distance = Some(read_f32(reader)?);
        }
        let mut vndwcid = None;
        if (header & 0x00200000) != 0 {
            vndwcid = Some(read_u16(reader)?);
        }
        let mut spell_id = None;
        if (header & 0x00400000) != 0 {
            spell_id = Some(SpellId::read(reader)?);
        }
        let mut house_owner_id = None;
        if (header & 0x02000000) != 0 {
            house_owner_id = Some(ObjectId::read(reader)?);
        }
        let mut physics_script = None;
        if (header & 0x08000000) != 0 {
            physics_script = Some(read_u16(reader)?);
        }
        let mut restrictions = None;
        if (header & 0x04000000) != 0 {
            restrictions = Some(RestrictionDB::read(reader)?);
        }
        let mut hook_type = None;
        if (header & 0x10000000) != 0 {
            hook_type = Some(HookType::try_from(read_u16(reader)?)?);
        }
        let mut hook_item_types = None;
        if (header & 0x20000000) != 0 {
            hook_item_types = Some(HookType::try_from(read_u16(reader)?)?);
        }
        let mut monarch_id = None;
        if (header & 0x00000040) != 0 {
            monarch_id = Some(ObjectId::read(reader)?);
        }
        let mut icon_overlay = None;
        if (header & 0x40000000) != 0 {
            icon_overlay = Some(PackedDWORD::read(reader)?);
        }
        let mut material = None;
        if (header & 0x80000000) != 0 {
            material = Some(MaterialType::try_from(read_u32(reader)?)?);
        }
        let __alignment_marker_align_dword = align_dword(reader)?;

        Ok(Self {
            header,
            name,
            weenie_class_id,
            icon,
            type_,
            bitfield,
            plural_name,
            items_capacity,
            container_capacity,
            value,
            useability,
            use_radius,
            t_target_type,
            effects,
            ammunition_type,
            combat_use,
            structure,
            max_structure,
            stack_size,
            max_stack_size,
            container_id,
            wielder_id,
            valid_slots,
            slots,
            priority,
            blip_color,
            radar_enum,
            obvious_distance,
            vndwcid,
            spell_id,
            house_owner_id,
            physics_script,
            restrictions,
            hook_type,
            hook_item_types,
            monarch_id,
            icon_overlay,
            material,
        })
    }
}

impl crate::readers::ACDataType for OldPublicWeenieDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        OldPublicWeenieDesc::read(reader)
    }
}

impl Trade {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let partner_id = ObjectId::read(reader)?;
        let sequence = read_u64(reader)?;
        let status = read_u32(reader)?;
        let initiator_id = ObjectId::read(reader)?;
        let accepted = read_bool(reader)?;
        let partner_accepted = read_bool(reader)?;

        Ok(Self {
            partner_id,
            sequence,
            status,
            initiator_id,
            accepted,
            partner_accepted,
        })
    }
}

impl crate::readers::ACDataType for Trade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Trade::read(reader)
    }
}

impl JumpPack {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let extent = read_f32(reader)?;
        let velocity = Vector3::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let object_server_control_sequence = read_u16(reader)?;
        let object_teleport_sequence = read_u16(reader)?;
        let object_force_position_sequence = read_u16(reader)?;
        let __alignment_marker_align_dword = align_dword(reader)?;

        Ok(Self {
            extent,
            velocity,
            object_instance_sequence,
            object_server_control_sequence,
            object_teleport_sequence,
            object_force_position_sequence,
        })
    }
}

impl crate::readers::ACDataType for JumpPack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        JumpPack::read(reader)
    }
}

impl MoveToStatePack {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let raw_motion_state = RawMotionState::read(reader)?;
        let position = Position::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let object_server_control_sequence = read_u16(reader)?;
        let object_teleport_sequence = read_u16(reader)?;
        let object_force_position_sequence = read_u16(reader)?;
        let contact = read_u8(reader)?;
        let __alignment_marker_align_dword = align_dword(reader)?;

        Ok(Self {
            raw_motion_state,
            position,
            object_instance_sequence,
            object_server_control_sequence,
            object_teleport_sequence,
            object_force_position_sequence,
            contact,
        })
    }
}

impl crate::readers::ACDataType for MoveToStatePack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MoveToStatePack::read(reader)
    }
}

impl PackedMotionCommand {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let command_id = Command::try_from(read_u16(reader)?)?;
        let packed_sequence = read_u16(reader)?;
        let server_action_sequence = (packed_sequence & 0x7fff) as u16;
        let autonomous = ((packed_sequence >> 15) & 0x1) as u16;
        let speed = read_f32(reader)?;

        Ok(Self {
            command_id,
            packed_sequence,
            speed,
        })
    }
}

impl crate::readers::ACDataType for PackedMotionCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PackedMotionCommand::read(reader)
    }
}

impl RawMotionState {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let command_list_length = ((flags >> 11) & 0x_f8) as u16;
        let mut current_holdkey = None;
        if (flags & 0x00000001) != 0 {
            current_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
        }
        let mut current_style = None;
        if (flags & 0x00000002) != 0 {
            current_style = Some(StanceMode::try_from(read_u16(reader)?)?);
        }
        let mut forward_command = None;
        if (flags & 0x00000004) != 0 {
            forward_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut forward_holdkey = None;
        if (flags & 0x0000008) != 0 {
            forward_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
        }
        let mut forward_speed = None;
        if (flags & 0x00000010) != 0 {
            forward_speed = Some(read_f32(reader)?);
        }
        let mut sidestep_command = None;
        if (flags & 0x00000020) != 0 {
            sidestep_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut sidestep_holdkey = None;
        if (flags & 0x00000040) != 0 {
            sidestep_holdkey = Some(HoldKey::try_from(read_u32(reader)?)?);
        }
        let mut sidestep_speed = None;
        if (flags & 0x00000080) != 0 {
            sidestep_speed = Some(read_f32(reader)?);
        }
        let mut turn_command = None;
        if (flags & 0x00000100) != 0 {
            turn_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut turn_holdkey = None;
        if (flags & 0x00000200) != 0 {
            turn_holdkey = Some(read_u32(reader)?);
        }
        let mut turn_speed = None;
        if (flags & 0x00000400) != 0 {
            turn_speed = Some(read_f32(reader)?);
        }
        let commands = read_vec::<PackedMotionCommand>(reader, command_list_length as usize)?;

        Ok(Self {
            flags,
            current_holdkey,
            current_style,
            forward_command,
            forward_holdkey,
            forward_speed,
            sidestep_command,
            sidestep_holdkey,
            sidestep_speed,
            turn_command,
            turn_holdkey,
            turn_speed,
            commands,
        })
    }
}

impl crate::readers::ACDataType for RawMotionState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        RawMotionState::read(reader)
    }
}

impl AutonomousPositionPack {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let position = Position::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let object_server_control_sequence = read_u16(reader)?;
        let object_teleport_sequence = read_u16(reader)?;
        let object_force_position_sequence = read_u16(reader)?;
        let contact = read_u8(reader)?;
        let __alignment_marker_align_dword = align_dword(reader)?;

        Ok(Self {
            position,
            object_instance_sequence,
            object_server_control_sequence,
            object_teleport_sequence,
            object_force_position_sequence,
            contact,
        })
    }
}

impl crate::readers::ACDataType for AutonomousPositionPack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AutonomousPositionPack::read(reader)
    }
}

impl PositionPack {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = PositionFlags::try_from(read_u32(reader)?)?;
        let origin = Origin::read(reader)?;
        let mut w_quat = None;
        if (flags.clone() as u32 & 0x00000008) != 0 {
            w_quat = Some(read_f32(reader)?);
        }
        let mut x_quat = None;
        if (flags.clone() as u32 & 0x00000010) != 0 {
            x_quat = Some(read_f32(reader)?);
        }
        let mut y_quat = None;
        if (flags.clone() as u32 & 0x00000020) != 0 {
            y_quat = Some(read_f32(reader)?);
        }
        let mut z_quat = None;
        if (flags.clone() as u32 & 0x00000040) != 0 {
            z_quat = Some(read_f32(reader)?);
        }
        let mut velocity = None;
        if (flags.clone() as u32 & 0x00000001) != 0 {
            velocity = Some(Vector3::read(reader)?);
        }
        let mut placement_id = None;
        if (flags.clone() as u32 & 0x00000002) != 0 {
            placement_id = Some(read_u32(reader)?);
        }
        let object_instance_sequence = read_u16(reader)?;
        let object_position_sequence = read_u16(reader)?;
        let object_teleport_sequence = read_u16(reader)?;
        let object_force_position_sequence = read_u16(reader)?;

        Ok(Self {
            flags,
            origin,
            w_quat,
            x_quat,
            y_quat,
            z_quat,
            velocity,
            placement_id,
            object_instance_sequence,
            object_position_sequence,
            object_teleport_sequence,
            object_force_position_sequence,
        })
    }
}

impl crate::readers::ACDataType for PositionPack {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PositionPack::read(reader)
    }
}

impl MovementDataType0 {
    pub fn read(reader: &mut dyn ACReader, object_movement_sequence: ushort, object_server_control_sequence: ushort, autonomous: ushort, option_flags: MovementOption, stance: StanceMode) -> Result<Self, Box<dyn std::error::Error>> {
        let state = InterpertedMotionState::read(reader)?;
        let sticky_object = if (option_flags.clone() as u32 & 0x01) != 0 { ObjectId::read(reader).map(Some) } else { Ok(None) }?;

        Ok(Self {
            object_movement_sequence,
            object_server_control_sequence,
            autonomous,
            option_flags,
            stance,
            state,
            sticky_object,
        })
    }
}

impl MovementDataType6 {
    pub fn read(reader: &mut dyn ACReader, object_movement_sequence: ushort, object_server_control_sequence: ushort, autonomous: ushort, option_flags: MovementOption, stance: StanceMode) -> Result<Self, Box<dyn std::error::Error>> {
        let target = ObjectId::read(reader)?;
        let origin = Origin::read(reader)?;
        let move_to_params = MoveToMovementParameters::read(reader)?;
        let my_run_rate = read_f32(reader)?;

        Ok(Self {
            object_movement_sequence,
            object_server_control_sequence,
            autonomous,
            option_flags,
            stance,
            target,
            origin,
            move_to_params,
            my_run_rate,
        })
    }
}

impl MovementDataType7 {
    pub fn read(reader: &mut dyn ACReader, object_movement_sequence: ushort, object_server_control_sequence: ushort, autonomous: ushort, option_flags: MovementOption, stance: StanceMode) -> Result<Self, Box<dyn std::error::Error>> {
        let origin = Origin::read(reader)?;
        let move_to_params = MoveToMovementParameters::read(reader)?;
        let my_run_rate = read_f32(reader)?;

        Ok(Self {
            object_movement_sequence,
            object_server_control_sequence,
            autonomous,
            option_flags,
            stance,
            origin,
            move_to_params,
            my_run_rate,
        })
    }
}

impl MovementDataType8 {
    pub fn read(reader: &mut dyn ACReader, object_movement_sequence: ushort, object_server_control_sequence: ushort, autonomous: ushort, option_flags: MovementOption, stance: StanceMode) -> Result<Self, Box<dyn std::error::Error>> {
        let target_id = ObjectId::read(reader)?;
        let desired_heading = read_f32(reader)?;
        let turn_to_params = TurnToMovementParameters::read(reader)?;

        Ok(Self {
            object_movement_sequence,
            object_server_control_sequence,
            autonomous,
            option_flags,
            stance,
            target_id,
            desired_heading,
            turn_to_params,
        })
    }
}

impl MovementDataType9 {
    pub fn read(reader: &mut dyn ACReader, object_movement_sequence: ushort, object_server_control_sequence: ushort, autonomous: ushort, option_flags: MovementOption, stance: StanceMode) -> Result<Self, Box<dyn std::error::Error>> {
        let turn_to_params = TurnToMovementParameters::read(reader)?;

        Ok(Self {
            object_movement_sequence,
            object_server_control_sequence,
            autonomous,
            option_flags,
            stance,
            turn_to_params,
        })
    }
}

impl MovementData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_movement_sequence = read_u16(reader)?;
        let object_server_control_sequence = read_u16(reader)?;
        let autonomous = read_u16(reader)?;
        let movement_type = MovementType::try_from(read_u8(reader)?)?;
        let option_flags = MovementOption::try_from(read_u8(reader)?)?;
        let stance = StanceMode::try_from(read_u16(reader)?)?;

        match movement_type {
            MovementType::InterpertedMotionState => {
                let variant_struct = MovementDataType0::read(reader, object_movement_sequence, object_server_control_sequence, autonomous, option_flags, stance)?;
                Ok(Self::Type0(variant_struct))
            },
            MovementType::MoveToObject => {
                let variant_struct = MovementDataType6::read(reader, object_movement_sequence, object_server_control_sequence, autonomous, option_flags, stance)?;
                Ok(Self::Type6(variant_struct))
            },
            MovementType::MoveToPosition => {
                let variant_struct = MovementDataType7::read(reader, object_movement_sequence, object_server_control_sequence, autonomous, option_flags, stance)?;
                Ok(Self::Type7(variant_struct))
            },
            MovementType::TurnToObject => {
                let variant_struct = MovementDataType8::read(reader, object_movement_sequence, object_server_control_sequence, autonomous, option_flags, stance)?;
                Ok(Self::Type8(variant_struct))
            },
            MovementType::TurnToPosition => {
                let variant_struct = MovementDataType9::read(reader, object_movement_sequence, object_server_control_sequence, autonomous, option_flags, stance)?;
                Ok(Self::Type9(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "movement_type", movement_type).into()),
        }
    }
}

impl crate::readers::ACDataType for MovementData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementData::read(reader)
    }
}

impl InterpertedMotionState {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let command_list_length = ((flags >> 7) & 0x7f) as u32;
        let mut current_style = None;
        if (flags & 0x00000001) != 0 {
            current_style = Some(StanceMode::try_from(read_u16(reader)?)?);
        }
        let mut forward_command = None;
        if (flags & 0x00000002) != 0 {
            forward_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut sidestep_command = None;
        if (flags & 0x00000008) != 0 {
            sidestep_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut turn_command = None;
        if (flags & 0x00000020) != 0 {
            turn_command = Some(MovementCommand::try_from(read_u16(reader)?)?);
        }
        let mut forward_speed = None;
        if (flags & 0x00000004) != 0 {
            forward_speed = Some(read_f32(reader)?);
        }
        let mut sidestep_speed = None;
        if (flags & 0x00000010) != 0 {
            sidestep_speed = Some(read_f32(reader)?);
        }
        let mut turn_speed = None;
        if (flags & 0x00000040) != 0 {
            turn_speed = Some(read_f32(reader)?);
        }
        let commands = read_vec::<PackedMotionCommand>(reader, command_list_length as usize)?;
        let __alignment_marker_align_dword = align_dword(reader)?;

        Ok(Self {
            flags,
            current_style,
            forward_command,
            sidestep_command,
            turn_command,
            forward_speed,
            sidestep_speed,
            turn_speed,
            commands,
        })
    }
}

impl crate::readers::ACDataType for InterpertedMotionState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InterpertedMotionState::read(reader)
    }
}

impl DDDRevision {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let id_dat_file = read_u64(reader)?;
        let dat_file_type = (id_dat_file >> 32) as u32;
        let iteration = read_u32(reader)?;
        let ids_to_download = read_packable_list::<DataId>(reader)?;
        let ids_to_purge = read_packable_list::<DataId>(reader)?;

        Ok(Self {
            id_dat_file,
            iteration,
            ids_to_download,
            ids_to_purge,
        })
    }
}

impl crate::readers::ACDataType for DDDRevision {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDRevision::read(reader)
    }
}

impl MoveToMovementParameters {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let bitmember = read_u32(reader)?;
        let distance_to_object = read_f32(reader)?;
        let min_distance = read_f32(reader)?;
        let fail_distance = read_f32(reader)?;
        let animation_speed = read_f32(reader)?;
        let walk_run_threshold = read_f32(reader)?;
        let desired_heading = read_f32(reader)?;

        Ok(Self {
            bitmember,
            distance_to_object,
            min_distance,
            fail_distance,
            animation_speed,
            walk_run_threshold,
            desired_heading,
        })
    }
}

impl crate::readers::ACDataType for MoveToMovementParameters {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MoveToMovementParameters::read(reader)
    }
}

impl TurnToMovementParameters {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let bitmember = read_u32(reader)?;
        let animation_speed = read_f32(reader)?;
        let desired_heading = read_f32(reader)?;

        Ok(Self {
            bitmember,
            animation_speed,
            desired_heading,
        })
    }
}

impl crate::readers::ACDataType for TurnToMovementParameters {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TurnToMovementParameters::read(reader)
    }
}

impl ObjDesc {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let version = read_u8(reader)?;
        let palette_count = read_u8(reader)?;
        let texture_count = read_u8(reader)?;
        let model_count = read_u8(reader)?;
        let mut palette = None;
        if palette_count > 0 {
            palette = Some(DataId::read(reader)?);
        }
        let subpalettes = read_vec::<Subpalette>(reader, palette_count as usize)?;
        let tm_changes = read_vec::<TextureMapChange>(reader, texture_count as usize)?;
        let ap_changes = read_vec::<AnimPartChange>(reader, model_count as usize)?;
        let __alignment_marker_align_dword = align_dword(reader)?;

        Ok(Self {
            version,
            palette_count,
            texture_count,
            model_count,
            palette,
            subpalettes,
            tm_changes,
            ap_changes,
        })
    }
}

impl crate::readers::ACDataType for ObjDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ObjDesc::read(reader)
    }
}

impl Subpalette {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let palette = DataId::read(reader)?;
        let offset = read_u8(reader)?;
        let num_colors = read_u8(reader)?;

        Ok(Self {
            palette,
            offset,
            num_colors,
        })
    }
}

impl crate::readers::ACDataType for Subpalette {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Subpalette::read(reader)
    }
}

impl TextureMapChange {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let part_index = read_u8(reader)?;
        let old_tex_id = DataId::read(reader)?;
        let new_tex_id = DataId::read(reader)?;

        Ok(Self {
            part_index,
            old_tex_id,
            new_tex_id,
        })
    }
}

impl crate::readers::ACDataType for TextureMapChange {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TextureMapChange::read(reader)
    }
}

impl AnimPartChange {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let part_index = read_u8(reader)?;
        let part_id = DataId::read(reader)?;

        Ok(Self {
            part_index,
            part_id,
        })
    }
}

impl crate::readers::ACDataType for AnimPartChange {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AnimPartChange::read(reader)
    }
}

impl CharGenResult {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let account = read_string(reader)?;
        let one = read_u32(reader)?;
        let heritage_group = HeritageGroup::try_from(read_u8(reader)?)?;
        let gender = Gender::try_from(read_u8(reader)?)?;
        let eyes_strip = read_u32(reader)?;
        let nose_strip = read_u32(reader)?;
        let mouth_strip = read_u32(reader)?;
        let hair_color = read_u32(reader)?;
        let eye_color = read_u32(reader)?;
        let hair_style = read_u32(reader)?;
        let headgear_style = read_u32(reader)?;
        let headgear_color = read_u32(reader)?;
        let shirt_style = read_u32(reader)?;
        let shirt_color = read_u32(reader)?;
        let trousers_style = read_u32(reader)?;
        let trousers_color = read_u32(reader)?;
        let footwear_style = read_u32(reader)?;
        let footwear_color = read_u32(reader)?;
        let skin_shade = read_u64(reader)?;
        let hair_shade = read_u64(reader)?;
        let headgear_shade = read_u64(reader)?;
        let shirt_shade = read_u64(reader)?;
        let trousers_shade = read_u64(reader)?;
        let tootwear_shade = read_u64(reader)?;
        let template_num = read_u32(reader)?;
        let strength = read_u32(reader)?;
        let endurance = read_u32(reader)?;
        let coordination = read_u32(reader)?;
        let quickness = read_u32(reader)?;
        let focus = read_u32(reader)?;
        let self_ = read_u32(reader)?;
        let slot = read_u32(reader)?;
        let class_id = read_u32(reader)?;
        let skills = read_packable_list::<SkillAdvancementClass>(reader)?;
        let name = read_string(reader)?;
        let start_area = read_u32(reader)?;
        let is_admin = read_u32(reader)?;
        let is_envoy = read_u32(reader)?;
        let validation = read_u32(reader)?;

        Ok(Self {
            account,
            one,
            heritage_group,
            gender,
            eyes_strip,
            nose_strip,
            mouth_strip,
            hair_color,
            eye_color,
            hair_style,
            headgear_style,
            headgear_color,
            shirt_style,
            shirt_color,
            trousers_style,
            trousers_color,
            footwear_style,
            footwear_color,
            skin_shade,
            hair_shade,
            headgear_shade,
            shirt_shade,
            trousers_shade,
            tootwear_shade,
            template_num,
            strength,
            endurance,
            coordination,
            quickness,
            focus,
            self_,
            slot,
            class_id,
            skills,
            name,
            start_area,
            is_admin,
            is_envoy,
            validation,
        })
    }
}

impl crate::readers::ACDataType for CharGenResult {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharGenResult::read(reader)
    }
}

impl CharacterIdentity {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;
        let name = read_string(reader)?;
        let seconds_greyed_out = read_u32(reader)?;
        let __alignment_marker_align_dword = align_dword(reader)?;

        Ok(Self {
            character_id,
            name,
            seconds_greyed_out,
        })
    }
}

impl crate::readers::ACDataType for CharacterIdentity {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterIdentity::read(reader)
    }
}

impl EquipLocation {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let slot = EquipMask::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            slot,
        })
    }
}

impl crate::readers::ACDataType for EquipLocation {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        EquipLocation::read(reader)
    }
}

impl PhysicsDesc {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let state = PhysicsState::try_from(read_u32(reader)?)?;
        let mut movement_buffer = None;
        let mut autonomous = None;
        if (flags & 0x00010000) != 0 {
            movement_buffer = Some(read_packable_list::<u8>(reader)?);
            autonomous = Some(read_bool(reader)?);
        }
        let mut animation_frame = None;
        if (flags & 0x00020000) != 0 {
            animation_frame = Some(read_u32(reader)?);
        }
        let mut position = None;
        if (flags & 0x00008000) != 0 {
            position = Some(Position::read(reader)?);
        }
        let mut motion_id = None;
        if (flags & 0x00000002) != 0 {
            motion_id = Some(DataId::read(reader)?);
        }
        let mut sound_id = None;
        if (flags & 0x00000800) != 0 {
            sound_id = Some(DataId::read(reader)?);
        }
        let mut physics_script_id = None;
        if (flags & 0x00001000) != 0 {
            physics_script_id = Some(DataId::read(reader)?);
        }
        let mut setup_id = None;
        if (flags & 0x00000001) != 0 {
            setup_id = Some(DataId::read(reader)?);
        }
        let mut parent_id = None;
        let mut parent_location = None;
        if (flags & 0x00000020) != 0 {
            parent_id = Some(ObjectId::read(reader)?);
            parent_location = Some(ParentLocation::try_from(read_u32(reader)?)?);
        }
        let mut children = None;
        if (flags & 0x00000040) != 0 {
            children = Some(read_packable_list::<EquipLocation>(reader)?);
        }
        let mut scale = None;
        if (flags & 0x00000080) != 0 {
            scale = Some(read_f32(reader)?);
        }
        let mut friction = None;
        if (flags & 0x00000100) != 0 {
            friction = Some(read_f32(reader)?);
        }
        let mut elasticity = None;
        if (flags & 0x00000200) != 0 {
            elasticity = Some(read_f32(reader)?);
        }
        let mut translucency = None;
        if (flags & 0x00040000) != 0 {
            translucency = Some(read_f32(reader)?);
        }
        let mut velocity = None;
        if (flags & 0x00000004) != 0 {
            velocity = Some(Vector3::read(reader)?);
        }
        let mut acceleration = None;
        if (flags & 0x00000008) != 0 {
            acceleration = Some(Vector3::read(reader)?);
        }
        let mut omega = None;
        if (flags & 0x00000010) != 0 {
            omega = Some(Vector3::read(reader)?);
        }
        let mut default_script = None;
        if (flags & 0x00002000) != 0 {
            default_script = Some(read_u32(reader)?);
        }
        let mut default_script_intensity = None;
        if (flags & 0x00004000) != 0 {
            default_script_intensity = Some(read_f32(reader)?);
        }
        let object_position_sequence = read_u16(reader)?;
        let object_movement_sequence = read_u16(reader)?;
        let object_state_sequence = read_u16(reader)?;
        let object_vector_sequence = read_u16(reader)?;
        let object_teleport_sequence = read_u16(reader)?;
        let object_server_control_sequence = read_u16(reader)?;
        let object_force_position_sequence = read_u16(reader)?;
        let object_visual_desc_sequence = read_u16(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let __alignment_marker_align_dword = align_dword(reader)?;

        Ok(Self {
            flags,
            state,
            movement_buffer,
            autonomous,
            animation_frame,
            position,
            motion_id,
            sound_id,
            physics_script_id,
            setup_id,
            parent_id,
            parent_location,
            children,
            scale,
            friction,
            elasticity,
            translucency,
            velocity,
            acceleration,
            omega,
            default_script,
            default_script_intensity,
            object_position_sequence,
            object_movement_sequence,
            object_state_sequence,
            object_vector_sequence,
            object_teleport_sequence,
            object_server_control_sequence,
            object_force_position_sequence,
            object_visual_desc_sequence,
            object_instance_sequence,
        })
    }
}

impl crate::readers::ACDataType for PhysicsDesc {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        PhysicsDesc::read(reader)
    }
}

impl AdminAccountData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let account_name = read_string(reader)?;
        let bookie_id = read_u32(reader)?;

        Ok(Self {
            account_name,
            bookie_id,
        })
    }
}

impl crate::readers::ACDataType for AdminAccountData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminAccountData::read(reader)
    }
}

impl AdminPlayerData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let name = read_string(reader)?;
        let bookie_id = read_u32(reader)?;

        Ok(Self {
            name,
            bookie_id,
        })
    }
}

impl crate::readers::ACDataType for AdminPlayerData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AdminPlayerData::read(reader)
    }
}

impl VendorProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let categories = ItemType::try_from(read_u32(reader)?)?;
        let min_value = read_u32(reader)?;
        let max_value = read_u32(reader)?;
        let deals_magic = read_bool(reader)?;
        let buy_price = read_f32(reader)?;
        let sell_price = read_f32(reader)?;
        let currency_id = read_u32(reader)?;
        let currency_amount = read_u32(reader)?;
        let currency_name = read_string(reader)?;

        Ok(Self {
            categories,
            min_value,
            max_value,
            deals_magic,
            buy_price,
            sell_price,
            currency_id,
            currency_amount,
            currency_name,
        })
    }
}

impl crate::readers::ACDataType for VendorProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        VendorProfile::read(reader)
    }
}

impl ArmorProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let prot_slashing = read_f32(reader)?;
        let prot_piercing = read_f32(reader)?;
        let prot_bludgeoning = read_f32(reader)?;
        let prot_cold = read_f32(reader)?;
        let prot_fire = read_f32(reader)?;
        let prot_acid = read_f32(reader)?;
        let prot_nether = read_f32(reader)?;
        let prot_lightning = read_f32(reader)?;

        Ok(Self {
            prot_slashing,
            prot_piercing,
            prot_bludgeoning,
            prot_cold,
            prot_fire,
            prot_acid,
            prot_nether,
            prot_lightning,
        })
    }
}

impl crate::readers::ACDataType for ArmorProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ArmorProfile::read(reader)
    }
}

impl CreatureAppraisalProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = read_u32(reader)?;
        let health = read_u32(reader)?;
        let health_max = read_u32(reader)?;
        let mut strength = None;
        let mut endurance = None;
        let mut quickness = None;
        let mut coordination = None;
        let mut focus = None;
        let mut self_ = None;
        let mut stamina = None;
        let mut mana = None;
        let mut stamina_max = None;
        let mut mana_max = None;
        if (flags & 0x00000008) != 0 {
            strength = Some(read_u32(reader)?);
            endurance = Some(read_u32(reader)?);
            quickness = Some(read_u32(reader)?);
            coordination = Some(read_u32(reader)?);
            focus = Some(read_u32(reader)?);
            self_ = Some(read_u32(reader)?);
            stamina = Some(read_u32(reader)?);
            mana = Some(read_u32(reader)?);
            stamina_max = Some(read_u32(reader)?);
            mana_max = Some(read_u32(reader)?);
        }
        let mut attr_highlight = None;
        let mut attr_color = None;
        if (flags & 0x00000001) != 0 {
            attr_highlight = Some(AttributeMask::try_from(read_u16(reader)?)?);
            attr_color = Some(AttributeMask::try_from(read_u16(reader)?)?);
        }

        Ok(Self {
            flags,
            health,
            health_max,
            strength,
            endurance,
            quickness,
            coordination,
            focus,
            self_,
            stamina,
            mana,
            stamina_max,
            mana_max,
            attr_highlight,
            attr_color,
        })
    }
}

impl crate::readers::ACDataType for CreatureAppraisalProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CreatureAppraisalProfile::read(reader)
    }
}

impl WeaponProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let damage_type = DamageType::try_from(read_u32(reader)?)?;
        let speed = read_u32(reader)?;
        let skill = SkillId::try_from(read_i32(reader)?)?;
        let damage = read_u32(reader)?;
        let variance = read_f64(reader)?;
        let modifier = read_f64(reader)?;
        let length = read_f64(reader)?;
        let max_velocity = read_f64(reader)?;
        let offsense = read_f64(reader)?;
        let max_velocity_estimated = read_u32(reader)?;

        Ok(Self {
            damage_type,
            speed,
            skill,
            damage,
            variance,
            modifier,
            length,
            max_velocity,
            offsense,
            max_velocity_estimated,
        })
    }
}

impl crate::readers::ACDataType for WeaponProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        WeaponProfile::read(reader)
    }
}

impl HookAppraisalProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let flags = HookAppraisalFlags::try_from(read_u32(reader)?)?;
        let valid_locations = EquipMask::try_from(read_u32(reader)?)?;
        let ammo_type = AmmoType::try_from(read_u16(reader)?)?;

        Ok(Self {
            flags,
            valid_locations,
            ammo_type,
        })
    }
}

impl crate::readers::ACDataType for HookAppraisalProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HookAppraisalProfile::read(reader)
    }
}

impl SquelchDB {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let account_hash = read_packable_hash_table_with(reader, |r| {
            Ok(read_string(r)?)
        }, |r| {
            Ok(read_u32(r)?)
        })?;
        let character_hash = read_packable_hash_table_with(reader, |r| {
            Ok(ObjectId::read(r)?)
        }, |r| {
            Ok(SquelchInfo::read(r)?)
        })?;
        let global_info = SquelchInfo::read(reader)?;

        Ok(Self {
            account_hash,
            character_hash,
            global_info,
        })
    }
}

impl crate::readers::ACDataType for SquelchDB {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SquelchDB::read(reader)
    }
}

impl SquelchInfo {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let filters = read_packable_list::<LogTextType>(reader)?;
        let name = read_string(reader)?;
        let account = read_bool(reader)?;

        Ok(Self {
            filters,
            name,
            account,
        })
    }
}

impl crate::readers::ACDataType for SquelchInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SquelchInfo::read(reader)
    }
}

impl HouseProfile {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let dwelling_id = read_u32(reader)?;
        let owner_id = ObjectId::read(reader)?;
        let flags = HouseBitfield::try_from(read_u32(reader)?)?;
        let min_level = read_i32(reader)?;
        let max_level = read_i32(reader)?;
        let min_alleg_rank = read_i32(reader)?;
        let max_alleg_rank = read_i32(reader)?;
        let maintenance_free = read_bool(reader)?;
        let type_ = HouseType::try_from(read_u32(reader)?)?;
        let owner_name = read_string(reader)?;
        let buy = read_packable_list::<HousePayment>(reader)?;
        let rent = read_packable_list::<HousePayment>(reader)?;

        Ok(Self {
            dwelling_id,
            owner_id,
            flags,
            min_level,
            max_level,
            min_alleg_rank,
            max_alleg_rank,
            maintenance_free,
            type_,
            owner_name,
            buy,
            rent,
        })
    }
}

impl crate::readers::ACDataType for HouseProfile {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseProfile::read(reader)
    }
}

impl HousePayment {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let required = read_u32(reader)?;
        let paid = read_u32(reader)?;
        let weenie_class_id = read_u32(reader)?;
        let name = read_string(reader)?;
        let plural_name = read_string(reader)?;

        Ok(Self {
            required,
            paid,
            weenie_class_id,
            name,
            plural_name,
        })
    }
}

impl crate::readers::ACDataType for HousePayment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HousePayment::read(reader)
    }
}

impl HouseData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let buy_time = read_u32(reader)?;
        let rent_time = read_u32(reader)?;
        let type_ = HouseType::try_from(read_u32(reader)?)?;
        let maintenance_free = read_bool(reader)?;
        let buy = read_packable_list::<HousePayment>(reader)?;
        let rent = read_packable_list::<HousePayment>(reader)?;
        let position = Position::read(reader)?;

        Ok(Self {
            buy_time,
            rent_time,
            type_,
            maintenance_free,
            buy,
            rent,
            position,
        })
    }
}

impl crate::readers::ACDataType for HouseData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseData::read(reader)
    }
}

impl HAR {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let version = read_u32(reader)?;
        let bitmask = read_u32(reader)?;
        let monarch_id = ObjectId::read(reader)?;
        let guest_list = read_packable_hash_table_with(reader, |r| {
            Ok(ObjectId::read(r)?)
        }, |r| {
            Ok(GuestInfo::read(r)?)
        })?;
        let roommate_list = read_packable_list::<ObjectId>(reader)?;

        Ok(Self {
            version,
            bitmask,
            monarch_id,
            guest_list,
            roommate_list,
        })
    }
}

impl crate::readers::ACDataType for HAR {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HAR::read(reader)
    }
}

impl GuestInfo {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let has_storage_permission = read_bool(reader)?;
        let guest_name = read_string(reader)?;

        Ok(Self {
            has_storage_permission,
            guest_name,
        })
    }
}

impl crate::readers::ACDataType for GuestInfo {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GuestInfo::read(reader)
    }
}

impl GameMoveDataType4 {
    pub fn read(reader: &mut dyn ACReader, player_id: ObjectId, team: int) -> Result<Self, Box<dyn std::error::Error>> {
        let id_piece_to_move = read_i32(reader)?;
        let y_grid = read_i32(reader)?;

        Ok(Self {
            player_id,
            team,
            id_piece_to_move,
            y_grid,
        })
    }
}

impl GameMoveDataType5 {
    pub fn read(reader: &mut dyn ACReader, player_id: ObjectId, team: int) -> Result<Self, Box<dyn std::error::Error>> {
        let id_piece_to_move = read_i32(reader)?;
        let y_grid = read_i32(reader)?;
        let x_to = read_i32(reader)?;
        let y_to = read_i32(reader)?;

        Ok(Self {
            player_id,
            team,
            id_piece_to_move,
            y_grid,
            x_to,
            y_to,
        })
    }
}

impl GameMoveDataType6 {
    pub fn read(reader: &mut dyn ACReader, player_id: ObjectId, team: int) -> Result<Self, Box<dyn std::error::Error>> {
        let id_piece_to_move = read_i32(reader)?;

        Ok(Self {
            player_id,
            team,
            id_piece_to_move,
        })
    }
}

impl GameMoveData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = read_i32(reader)?;
        let player_id = ObjectId::read(reader)?;
        let team = read_i32(reader)?;

        match type_ {
            0x04 => {
                let variant_struct = GameMoveDataType4::read(reader, player_id, team)?;
                Ok(Self::Type4(variant_struct))
            },
            0x05 => {
                let variant_struct = GameMoveDataType5::read(reader, player_id, team)?;
                Ok(Self::Type5(variant_struct))
            },
            0x06 => {
                let variant_struct = GameMoveDataType6::read(reader, player_id, team)?;
                Ok(Self::Type6(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "type_", type_).into()),
        }
    }
}

impl crate::readers::ACDataType for GameMoveData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        GameMoveData::read(reader)
    }
}

impl SalvageOperationsResultData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let skill_used = SkillId::try_from(read_i32(reader)?)?;
        let not_salvagable = read_packable_list::<ObjectId>(reader)?;
        let salvage_results = read_packable_list::<SalvageResult>(reader)?;
        let aug_bonus = read_i32(reader)?;

        Ok(Self {
            skill_used,
            not_salvagable,
            salvage_results,
            aug_bonus,
        })
    }
}

impl crate::readers::ACDataType for SalvageOperationsResultData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SalvageOperationsResultData::read(reader)
    }
}

impl SalvageResult {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let material = MaterialType::try_from(read_u32(reader)?)?;
        let workmanship = read_f64(reader)?;
        let units = read_u32(reader)?;

        Ok(Self {
            material,
            workmanship,
            units,
        })
    }
}

impl crate::readers::ACDataType for SalvageResult {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SalvageResult::read(reader)
    }
}

impl FellowshipLockData {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let unknown1 = read_u32(reader)?;
        let unknown2 = read_u32(reader)?;
        let unknown3 = read_u32(reader)?;
        let timestamp = read_u32(reader)?;
        let sequence = read_u32(reader)?;

        Ok(Self {
            unknown1,
            unknown2,
            unknown3,
            timestamp,
            sequence,
        })
    }
}

impl crate::readers::ACDataType for FellowshipLockData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FellowshipLockData::read(reader)
    }
}

impl Fellowship {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let members = read_packable_hash_table_with(reader, |r| {
            Ok(ObjectId::read(r)?)
        }, |r| {
            Ok(Fellow::read(r)?)
        })?;
        let name = read_string(reader)?;
        let leader_id = ObjectId::read(reader)?;
        let share_xp = read_bool(reader)?;
        let even_xp_split = read_bool(reader)?;
        let open = read_bool(reader)?;
        let locked = read_bool(reader)?;
        let recently_departed = read_packable_hash_table_with(reader, |r| {
            Ok(ObjectId::read(r)?)
        }, |r| {
            Ok(read_i32(r)?)
        })?;
        let locks = read_packable_hash_table_with(reader, |r| {
            Ok(read_string(r)?)
        }, |r| {
            Ok(FellowshipLockData::read(r)?)
        })?;

        Ok(Self {
            members,
            name,
            leader_id,
            share_xp,
            even_xp_split,
            open,
            locked,
            recently_departed,
            locks,
        })
    }
}

impl crate::readers::ACDataType for Fellowship {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Fellowship::read(reader)
    }
}

impl Fellow {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let xp_cached = read_u32(reader)?;
        let lum_cached = read_u32(reader)?;
        let level = read_u32(reader)?;
        let max_health = read_u32(reader)?;
        let max_stamina = read_u32(reader)?;
        let max_mana = read_u32(reader)?;
        let current_health = read_u32(reader)?;
        let current_stamina = read_u32(reader)?;
        let current_mana = read_u32(reader)?;
        let share_loot = read_bool(reader)?;
        let name = read_string(reader)?;

        Ok(Self {
            xp_cached,
            lum_cached,
            level,
            max_health,
            max_stamina,
            max_mana,
            current_health,
            current_stamina,
            current_mana,
            share_loot,
            name,
        })
    }
}

impl crate::readers::ACDataType for Fellow {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Fellow::read(reader)
    }
}

impl ContractTracker {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let version = read_u32(reader)?;
        let contract_id = ContractId::try_from(read_u32(reader)?)?;
        let contract_stage = ContractStage::try_from(read_u32(reader)?)?;
        let time_when_done = read_i64(reader)?;
        let time_when_repeats = read_i64(reader)?;

        Ok(Self {
            version,
            contract_id,
            contract_stage,
            time_when_done,
            time_when_repeats,
        })
    }
}

impl crate::readers::ACDataType for ContractTracker {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ContractTracker::read(reader)
    }
}

impl ContractTrackerTable {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contact_trackers = read_packable_hash_table_with(reader, |r| {
            Ok(read_u32(r)?)
        }, |r| {
            Ok(ContractTracker::read(r)?)
        })?;

        Ok(Self {
            contact_trackers,
        })
    }
}

impl crate::readers::ACDataType for ContractTrackerTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ContractTrackerTable::read(reader)
    }
}

impl C2SPacket {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u32(reader)?;
        let flags = PacketHeaderFlags::try_from(read_u32(reader)?)?;
        let checksum = read_u32(reader)?;
        let recipient_id = read_u16(reader)?;
        let time_since_last_packet = read_u16(reader)?;
        let size = read_u16(reader)?;
        let iteration = read_u16(reader)?;
        let mut server_switch = None;
        if (flags.clone() as u32 & PacketHeaderFlags::ServerSwitch as u32) != 0 {
            server_switch = Some(ServerSwitchHeader::read(reader)?);
        }
        let mut retransmit_sequences = None;
        if (flags.clone() as u32 & PacketHeaderFlags::RequestRetransmit as u32) != 0 {
            retransmit_sequences = Some(read_packable_list::<u32>(reader)?);
        }
        let mut reject_sequences = None;
        if (flags.clone() as u32 & PacketHeaderFlags::RejectRetransmit as u32) != 0 {
            reject_sequences = Some(read_packable_list::<u32>(reader)?);
        }
        let mut ack_sequence = None;
        if (flags.clone() as u32 & PacketHeaderFlags::AckSequence as u32) != 0 {
            ack_sequence = Some(read_u32(reader)?);
        }
        let mut login_request = None;
        if (flags.clone() as u32 & PacketHeaderFlags::LoginRequest as u32) != 0 {
            login_request = Some(LoginRequestHeader::read(reader)?);
        }
        let mut world_login_request = None;
        if (flags.clone() as u32 & PacketHeaderFlags::WorldLoginRequest as u32) != 0 {
            world_login_request = Some(read_u64(reader)?);
        }
        let mut connect_response = None;
        if (flags.clone() as u32 & PacketHeaderFlags::ConnectResponse as u32) != 0 {
            connect_response = Some(read_u64(reader)?);
        }
        let mut cicmd_command = None;
        if (flags.clone() as u32 & PacketHeaderFlags::CICMDCommand as u32) != 0 {
            cicmd_command = Some(CICMDCommandHeader::read(reader)?);
        }
        let mut time = None;
        if (flags.clone() as u32 & PacketHeaderFlags::TimeSync as u32) != 0 {
            time = Some(read_u64(reader)?);
        }
        let mut echo_time = None;
        if (flags.clone() as u32 & PacketHeaderFlags::EchoRequest as u32) != 0 {
            echo_time = Some(read_f32(reader)?);
        }
        let mut flow = None;
        if (flags.clone() as u32 & PacketHeaderFlags::Flow as u32) != 0 {
            flow = Some(FlowHeader::read(reader)?);
        }
        let mut fragments = None;
        if (flags.clone() as u32 & PacketHeaderFlags::BlobFragments as u32) != 0 {
            fragments = Some(BlobFragments::read(reader)?);
        }

        Ok(Self {
            sequence,
            flags,
            checksum,
            recipient_id,
            time_since_last_packet,
            size,
            iteration,
            server_switch,
            retransmit_sequences,
            reject_sequences,
            ack_sequence,
            login_request,
            world_login_request,
            connect_response,
            cicmd_command,
            time,
            echo_time,
            flow,
            fragments,
        })
    }
}

impl crate::readers::ACDataType for C2SPacket {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        C2SPacket::read(reader)
    }
}

impl S2CPacket {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u32(reader)?;
        let flags = PacketHeaderFlags::try_from(read_u32(reader)?)?;
        let checksum = read_u32(reader)?;
        let recipient_id = read_u16(reader)?;
        let time_since_last_packet = read_u16(reader)?;
        let size = read_u16(reader)?;
        let iteration = read_u16(reader)?;
        let mut ack_sequence = None;
        if (flags.clone() as u32 & PacketHeaderFlags::AckSequence as u32) != 0 {
            ack_sequence = Some(read_u32(reader)?);
        }
        let mut logon_server_addr = None;
        if (flags.clone() as u32 & PacketHeaderFlags::LogonServerAddr as u32) != 0 {
            logon_server_addr = Some(SocketAddress::read(reader)?);
        }
        let mut referral = None;
        if (flags.clone() as u32 & PacketHeaderFlags::Referral as u32) != 0 {
            referral = Some(ReferralHeader::read(reader)?);
        }
        let mut connect_request = None;
        if (flags.clone() as u32 & PacketHeaderFlags::ConnectRequest as u32) != 0 {
            connect_request = Some(ConnectRequestHeader::read(reader)?);
        }
        let mut net_error = None;
        if (flags.clone() as u32 & PacketHeaderFlags::NetError as u32) != 0 {
            net_error = Some(NetError::read(reader)?);
        }
        let mut net_error_disconnect = None;
        if (flags.clone() as u32 & PacketHeaderFlags::NetErrorDisconnect as u32) != 0 {
            net_error_disconnect = Some(NetError::read(reader)?);
        }
        let mut echo_response = None;
        if (flags.clone() as u32 & PacketHeaderFlags::EchoResponse as u32) != 0 {
            echo_response = Some(EchoResponseHeader::read(reader)?);
        }
        let mut fragments = None;
        if (flags.clone() as u32 & PacketHeaderFlags::BlobFragments as u32) != 0 {
            fragments = Some(BlobFragments::read(reader)?);
        }

        Ok(Self {
            sequence,
            flags,
            checksum,
            recipient_id,
            time_since_last_packet,
            size,
            iteration,
            ack_sequence,
            logon_server_addr,
            referral,
            connect_request,
            net_error,
            net_error_disconnect,
            echo_response,
            fragments,
        })
    }
}

impl crate::readers::ACDataType for S2CPacket {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        S2CPacket::read(reader)
    }
}

