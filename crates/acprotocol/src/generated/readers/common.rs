// Binary readers for common types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl PackedDWORD {
    pub fn read(_reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl ObjectId {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u32(reader)?))
    }
}

impl LandcellId {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(read_u32(reader)?))
    }
}

impl Vector3 {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
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

impl Quaternion {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
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

impl Position {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        let landcell = LandcellId::read(reader)?;
        let frame = Frame::read(reader)?;

        Ok(Self {
            landcell,
            frame,
        })
    }
}

impl Frame {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        let origin = Vector3::read(reader)?;
        let orientation = Quaternion::read(reader)?;

        Ok(Self {
            origin,
            orientation,
        })
    }
}

impl AllegianceRecord {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        let tree_parent = ObjectId::read(reader)?;
        let allegiance_data = AllegianceData::read(reader)?;

        Ok(Self {
            tree_parent,
            allegiance_data,
        })
    }
}

impl AllegianceHierarchy {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        let record_count = read_u16(reader)?;
        let old_version = read_u16(reader)?;
        let officers = PHashTable::<ObjectId, AllegianceOfficerLevel>::read(reader)?;
        let officer_titles = unimplemented!("PackableList reading not yet implemented")?;
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
        let records = (|| -> Result<_, Box<dyn std::error::Error>> {
            let length = (record_count as usize) - 1;
            let mut vec = Vec::with_capacity(length);
            for _ in 0..length {
                vec.push(AllegianceRecord::read(reader)?);
            }
            Ok(vec)
        })()?;

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

impl AllegianceData {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
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

impl ObjDesc {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        let version = read_u8(reader)?;
        let palette_count = read_u8(reader)?;
        let texture_count = read_u8(reader)?;
        let model_count = read_u8(reader)?;
        let mut palette = None;
        if palette_count > 0 {
            palette = Some(DataId::read(reader)?);
        }
        let subpalettes = (|| -> Result<_, Box<dyn std::error::Error>> {
            let length = palette_count as usize;
            let mut vec = Vec::with_capacity(length);
            for _ in 0..length {
                vec.push(Subpalette::read(reader)?);
            }
            Ok(vec)
        })()?;
        let tm_changes = (|| -> Result<_, Box<dyn std::error::Error>> {
            let length = texture_count as usize;
            let mut vec = Vec::with_capacity(length);
            for _ in 0..length {
                vec.push(TextureMapChange::read(reader)?);
            }
            Ok(vec)
        })()?;
        let ap_changes = (|| -> Result<_, Box<dyn std::error::Error>> {
            let length = model_count as usize;
            let mut vec = Vec::with_capacity(length);
            for _ in 0..length {
                vec.push(AnimPartChange::read(reader)?);
            }
            Ok(vec)
        })()?;

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

impl Subpalette {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
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

impl TextureMapChange {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
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

impl AnimPartChange {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        let part_index = read_u8(reader)?;
        let part_id = DataId::read(reader)?;

        Ok(Self {
            part_index,
            part_id,
        })
    }
}

