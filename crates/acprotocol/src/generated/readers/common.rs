// Binary readers for common types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl WString {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_wstring(reader)?))
    }
}

impl crate::readers::ACDataType for WString {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        WString::read(reader)
    }
}

impl PackedDWORD {
    pub fn read(_reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for PackedDWORD {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        PackedDWORD::read(reader)
    }
}

impl ObjectId {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u32(reader)?))
    }
}

impl crate::readers::ACDataType for ObjectId {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        ObjectId::read(reader)
    }
}

impl LandcellId {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u32(reader)?))
    }
}

impl crate::readers::ACDataType for LandcellId {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        LandcellId::read(reader)
    }
}

impl SpellId {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u16(reader)?))
    }
}

impl crate::readers::ACDataType for SpellId {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        SpellId::read(reader)
    }
}

impl LayeredSpellId {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let id = SpellId::read(reader)?;
        let layer = read_u16(reader)?;

        Ok(Self {
            id,
            layer,
        })
    }
}

impl crate::readers::ACDataType for LayeredSpellId {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        LayeredSpellId::read(reader)
    }
}

impl Vector3 {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Vector3::read(reader)
    }
}

impl Quaternion {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Quaternion::read(reader)
    }
}

impl Origin {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let landcell = LandcellId::read(reader)?;
        let location = Vector3::read(reader)?;

        Ok(Self {
            landcell,
            location,
        })
    }
}

impl crate::readers::ACDataType for Origin {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Origin::read(reader)
    }
}

impl Position {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let landcell = LandcellId::read(reader)?;
        let frame = Frame::read(reader)?;

        Ok(Self {
            landcell,
            frame,
        })
    }
}

impl crate::readers::ACDataType for Position {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Position::read(reader)
    }
}

impl Frame {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let origin = Vector3::read(reader)?;
        let orientation = Quaternion::read(reader)?;

        Ok(Self {
            origin,
            orientation,
        })
    }
}

impl crate::readers::ACDataType for Frame {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Frame::read(reader)
    }
}

impl ServerSwitchHeader {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u32(reader)?;
        let type_ = ServerSwitchType::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            type_,
        })
    }
}

impl crate::readers::ACDataType for ServerSwitchHeader {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        ServerSwitchHeader::read(reader)
    }
}

impl CICMDCommandHeader {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let command = read_u32(reader)?;
        let parameter = read_u32(reader)?;

        Ok(Self {
            command,
            parameter,
        })
    }
}

impl crate::readers::ACDataType for CICMDCommandHeader {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        CICMDCommandHeader::read(reader)
    }
}

impl FlowHeader {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = read_u32(reader)?;
        let interval = read_u16(reader)?;

        Ok(Self {
            bytes,
            interval,
        })
    }
}

impl crate::readers::ACDataType for FlowHeader {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        FlowHeader::read(reader)
    }
}

impl SocketAddress {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        SocketAddress::read(reader)
    }
}

impl ReferralHeader {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let cookie = read_u64(reader)?;
        let address = SocketAddress::read(reader)?;
        let id_server = read_u16(reader)?;
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        ReferralHeader::read(reader)
    }
}

impl ConnectRequestHeader {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        ConnectRequestHeader::read(reader)
    }
}

impl NetError {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let string_id = DataId::read(reader)?;
        let table_id = DataId::read(reader)?;

        Ok(Self {
            string_id,
            table_id,
        })
    }
}

impl crate::readers::ACDataType for NetError {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        NetError::read(reader)
    }
}

impl EchoResponseHeader {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let local_time = read_f32(reader)?;
        let holding_time = read_f32(reader)?;

        Ok(Self {
            local_time,
            holding_time,
        })
    }
}

impl crate::readers::ACDataType for EchoResponseHeader {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        EchoResponseHeader::read(reader)
    }
}

impl AttributeInfo {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        AttributeInfo::read(reader)
    }
}

impl SecondaryAttributeInfo {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let attribute = AttributeInfo::read(reader)?;
        let current = read_u32(reader)?;

        Ok(Self {
            attribute,
            current,
        })
    }
}

impl crate::readers::ACDataType for SecondaryAttributeInfo {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        SecondaryAttributeInfo::read(reader)
    }
}

impl Skill {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Skill::read(reader)
    }
}

impl BodyPart {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        BodyPart::read(reader)
    }
}

impl ArmorCache {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        ArmorCache::read(reader)
    }
}

impl BodyPartSelectionData {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        BodyPartSelectionData::read(reader)
    }
}

impl StatMod {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        StatMod::read(reader)
    }
}

impl EventFilter {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let events = read_packable_list::<u32>(reader)?;

        Ok(Self {
            events,
        })
    }
}

impl crate::readers::ACDataType for EventFilter {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        EventFilter::read(reader)
    }
}

impl AllegianceRecord {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let tree_parent = ObjectId::read(reader)?;
        let allegiance_data = AllegianceData::read(reader)?;

        Ok(Self {
            tree_parent,
            allegiance_data,
        })
    }
}

impl crate::readers::ACDataType for AllegianceRecord {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceRecord::read(reader)
    }
}

impl AllegianceHierarchy {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
        let records = read_vec::<AllegianceRecord>(reader, (record_count as usize) - 1)?;

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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceHierarchy::read(reader)
    }
}

impl AllegianceData {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceData::read(reader)
    }
}

impl ObjDesc {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        ObjDesc::read(reader)
    }
}

impl Subpalette {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        Subpalette::read(reader)
    }
}

impl TextureMapChange {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        TextureMapChange::read(reader)
    }
}

impl AnimPartChange {
    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let part_index = read_u8(reader)?;
        let part_id = DataId::read(reader)?;

        Ok(Self {
            part_index,
            part_id,
        })
    }
}

impl crate::readers::ACDataType for AnimPartChange {
    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
        AnimPartChange::read(reader)
    }
}

