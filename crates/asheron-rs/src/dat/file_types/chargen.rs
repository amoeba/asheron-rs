use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::BTreeMap;
use std::io::{Read, Result, Seek, SeekFrom};

#[cfg(feature = "dat-export")]
use serde::Serialize;

// ── low-level reader helpers ──────────────────────────────────────────────────

/// C# `ReadCompressedUInt32`: 1, 2, or 4 bytes depending on high bits.
fn read_compressed_u32<R: Read>(r: &mut R) -> Result<u32> {
    let b0 = r.read_u8()?;
    if (b0 & 0x80) == 0 {
        return Ok(b0 as u32);
    }
    let b1 = r.read_u8()?;
    if (b0 & 0x40) == 0 {
        return Ok((((b0 & 0x7F) as u32) << 8) | b1 as u32);
    }
    let hi = r.read_u16::<LittleEndian>()? as u32;
    Ok(((((b0 & 0x3F) as u32) << 8 | b1 as u32) << 16) | hi)
}

/// C# `BinaryReader.ReadString()`: 7-bit encoded length then UTF-8 bytes.
fn read_ac_string<R: Read>(r: &mut R) -> Result<String> {
    let mut len: u32 = 0;
    let mut shift = 0u32;
    loop {
        let b = r.read_u8()?;
        len |= ((b & 0x7F) as u32) << shift;
        if (b & 0x80) == 0 {
            break;
        }
        shift += 7;
    }
    let mut buf = vec![0u8; len as usize];
    r.read_exact(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

/// C# `ReadAsDataIDOfKnownType`: 2-byte or 4-byte partial ID added to base.
fn read_data_id_of_known_type<R: Read>(r: &mut R, known_type: u32) -> Result<u32> {
    let value = r.read_u16::<LittleEndian>()? as u32;
    if (value & 0x8000) != 0 {
        let lower = r.read_u16::<LittleEndian>()? as u32;
        let higher = (value & 0x3FFF) << 16;
        return Ok(known_type + (higher | lower));
    }
    Ok(known_type + value)
}

/// Align the stream to the next 4-byte boundary (C# `AlignBoundary`).
fn align_boundary<R: Read + Seek>(r: &mut R) -> Result<()> {
    let pos = r.stream_position()?;
    let delta = pos % 4;
    if delta != 0 {
        r.seek(SeekFrom::Current((4 - delta) as i64))?;
    }
    Ok(())
}

fn read_smart_array_u32<R: Read>(r: &mut R) -> Result<Vec<u32>> {
    let count = read_compressed_u32(r)? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(r.read_u32::<LittleEndian>()?);
    }
    Ok(out)
}

fn read_smart_array_i32<R: Read>(r: &mut R) -> Result<Vec<i32>> {
    let count = read_compressed_u32(r)? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(r.read_i32::<LittleEndian>()?);
    }
    Ok(out)
}

// ── ObjDesc sub-structures (parse-only, no serialization) ────────────────────

struct SubPalette;
impl SubPalette {
    fn read<R: Read>(r: &mut R) -> Result<()> {
        read_data_id_of_known_type(r, 0x04000000)?;
        r.read_u8()?; // offset * 8
        r.read_u8()?; // num_colors
        Ok(())
    }
}

struct TextureMapChange;
impl TextureMapChange {
    fn read<R: Read>(r: &mut R) -> Result<()> {
        r.read_u8()?; // part_index
        read_data_id_of_known_type(r, 0x05000000)?;
        read_data_id_of_known_type(r, 0x05000000)?;
        Ok(())
    }
}

struct AnimationPartChange;
impl AnimationPartChange {
    fn read<R: Read>(r: &mut R) -> Result<()> {
        r.read_u8()?; // part_index
        read_data_id_of_known_type(r, 0x01000000)?;
        Ok(())
    }
}

fn read_obj_desc<R: Read + Seek>(r: &mut R) -> Result<()> {
    align_boundary(r)?;
    r.read_u8()?; // always 0x11
    let num_palettes = r.read_u8()? as usize;
    let num_textures = r.read_u8()? as usize;
    let num_anim_parts = r.read_u8()? as usize;
    if num_palettes > 0 {
        read_data_id_of_known_type(r, 0x04000000)?; // palette_id
    }
    for _ in 0..num_palettes {
        SubPalette::read(r)?;
    }
    for _ in 0..num_textures {
        TextureMapChange::read(r)?;
    }
    for _ in 0..num_anim_parts {
        AnimationPartChange::read(r)?;
    }
    align_boundary(r)?;
    Ok(())
}

fn read_hair_style_cg<R: Read + Seek>(r: &mut R) -> Result<()> {
    r.read_u32::<LittleEndian>()?; // icon_image
    r.read_u8()?; // bald
    r.read_u32::<LittleEndian>()?; // alternate_setup
    read_obj_desc(r)?;
    Ok(())
}

fn read_eye_strip_cg<R: Read + Seek>(r: &mut R) -> Result<()> {
    r.read_u32::<LittleEndian>()?; // icon_image
    r.read_u32::<LittleEndian>()?; // icon_image_bald
    read_obj_desc(r)?;
    read_obj_desc(r)?;
    Ok(())
}

fn read_face_strip_cg<R: Read + Seek>(r: &mut R) -> Result<()> {
    r.read_u32::<LittleEndian>()?; // icon_image
    read_obj_desc(r)?;
    Ok(())
}

fn read_gear_cg<R: Read>(r: &mut R) -> Result<()> {
    read_ac_string(r)?;
    r.read_u32::<LittleEndian>()?; // clothing_table
    r.read_u32::<LittleEndian>()?; // weenie_default
    Ok(())
}

/// Parse a SexCG entry, discarding all data.
fn read_sex_cg<R: Read + Seek>(r: &mut R) -> Result<()> {
    read_ac_string(r)?; // name
    for _ in 0..9 {
        r.read_u32::<LittleEndian>()?; // scale, setup_id, sound_table, icon, base_palette, skin_pal_set, physics, motion, combat
    }
    read_obj_desc(r)?;

    // hair_color_list (SmartArray<u32>)
    let n = read_compressed_u32(r)? as usize;
    for _ in 0..n {
        r.read_u32::<LittleEndian>()?;
    }
    // hair_style_list (SmartArray<HairStyleCG>)
    let n = read_compressed_u32(r)? as usize;
    for _ in 0..n {
        read_hair_style_cg(r)?;
    }
    // eye_color_list (SmartArray<u32>)
    let n = read_compressed_u32(r)? as usize;
    for _ in 0..n {
        r.read_u32::<LittleEndian>()?;
    }
    // eye_strip_list (SmartArray<EyeStripCG>)
    let n = read_compressed_u32(r)? as usize;
    for _ in 0..n {
        read_eye_strip_cg(r)?;
    }
    // nose_strip_list (SmartArray<FaceStripCG>)
    let n = read_compressed_u32(r)? as usize;
    for _ in 0..n {
        read_face_strip_cg(r)?;
    }
    // mouth_strip_list (SmartArray<FaceStripCG>)
    let n = read_compressed_u32(r)? as usize;
    for _ in 0..n {
        read_face_strip_cg(r)?;
    }
    // headgear_list, shirt_list, pants_list, footwear_list (SmartArray<GearCG> each)
    for _ in 0..4 {
        let n = read_compressed_u32(r)? as usize;
        for _ in 0..n {
            read_gear_cg(r)?;
        }
    }
    // clothing_colors_list (SmartArray<u32>)
    let n = read_compressed_u32(r)? as usize;
    for _ in 0..n {
        r.read_u32::<LittleEndian>()?;
    }
    Ok(())
}

// ── Public serializable types ─────────────────────────────────────────────────

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct Quaternion {
    #[cfg_attr(feature = "dat-export", serde(rename = "W"))]
    pub w: f32,
    #[cfg_attr(feature = "dat-export", serde(rename = "X"))]
    pub x: f32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Y"))]
    pub y: f32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Z"))]
    pub z: f32,
}

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct Vector3 {
    #[cfg_attr(feature = "dat-export", serde(rename = "X"))]
    pub x: f32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Y"))]
    pub y: f32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Z"))]
    pub z: f32,
}

/// Orientation first (matches expected JSON field order), then origin.
#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct Frame {
    #[cfg_attr(feature = "dat-export", serde(rename = "Orientation"))]
    pub orientation: Quaternion,
    #[cfg_attr(feature = "dat-export", serde(rename = "Origin"))]
    pub origin: Vector3,
}

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct Location {
    #[cfg_attr(feature = "dat-export", serde(rename = "CellId"))]
    pub cell_id: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Frame"))]
    pub frame: Frame,
}

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct StarterArea {
    #[cfg_attr(feature = "dat-export", serde(rename = "Name"))]
    pub name: String,
    #[cfg_attr(feature = "dat-export", serde(rename = "Locations"))]
    pub locations: Vec<Location>,
}

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct SkillCost {
    #[cfg_attr(feature = "dat-export", serde(rename = "Id"))]
    pub id: String,
    #[cfg_attr(feature = "dat-export", serde(rename = "NormalCost"))]
    pub normal_cost: i32,
    #[cfg_attr(feature = "dat-export", serde(rename = "PrimaryCost"))]
    pub primary_cost: i32,
}

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct TemplateCG {
    #[cfg_attr(feature = "dat-export", serde(rename = "Name"))]
    pub name: String,
    #[cfg_attr(feature = "dat-export", serde(rename = "IconId"))]
    pub icon_id: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Title"))]
    pub title: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Strength"))]
    pub strength: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Endurance"))]
    pub endurance: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Coordination"))]
    pub coordination: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Quickness"))]
    pub quickness: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Focus"))]
    pub focus: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Self"))]
    pub self_stat: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "NormalSkills"))]
    pub normal_skills: Vec<u32>,
    #[cfg_attr(feature = "dat-export", serde(rename = "PrimarySkills"))]
    pub primary_skills: Vec<u32>,
}

/// Heritage group data as read from the file.
#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct HeritageGroupCG {
    #[cfg_attr(feature = "dat-export", serde(rename = "Name"))]
    pub name: String,
    #[cfg_attr(feature = "dat-export", serde(rename = "IconId"))]
    pub icon_id: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "AttributeCredits"))]
    pub attribute_credits: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "SkillCredits"))]
    pub skill_credits: u32,
    /// Keyed by skill name string; genders omitted from output (always empty).
    #[cfg_attr(feature = "dat-export", serde(rename = "Skills"))]
    pub skills: BTreeMap<String, SkillCost>,
    #[cfg_attr(feature = "dat-export", serde(rename = "Genders"))]
    pub genders: BTreeMap<String, serde_json::Value>,
    #[cfg_attr(feature = "dat-export", serde(rename = "Templates"))]
    pub templates: Vec<TemplateCG>,
    #[cfg_attr(feature = "dat-export", serde(rename = "SetupId"))]
    pub setup_id: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "EnvironmentSetupId"))]
    pub env_setup_id: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "PrimaryStartAreas"))]
    pub primary_start_areas: Vec<i32>,
    #[cfg_attr(feature = "dat-export", serde(rename = "SecondaryStartAreas"))]
    pub secondary_start_areas: Vec<i32>,
}

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct CharGen {
    #[cfg_attr(feature = "dat-export", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "HeritageGroups"))]
    pub heritage_groups: Vec<HeritageGroupCG>,
    #[cfg_attr(feature = "dat-export", serde(rename = "StartingAreas"))]
    pub starting_areas: Vec<StarterArea>,
}

// ── Parsing ───────────────────────────────────────────────────────────────────

fn read_location<R: Read>(r: &mut R) -> Result<Location> {
    let cell_id = r.read_u32::<LittleEndian>()?;
    let ox = r.read_f32::<LittleEndian>()?;
    let oy = r.read_f32::<LittleEndian>()?;
    let oz = r.read_f32::<LittleEndian>()?;
    let qw = r.read_f32::<LittleEndian>()?;
    let qx = r.read_f32::<LittleEndian>()?;
    let qy = r.read_f32::<LittleEndian>()?;
    let qz = r.read_f32::<LittleEndian>()?;
    Ok(Location {
        cell_id,
        frame: Frame {
            orientation: Quaternion {
                w: qw,
                x: qx,
                y: qy,
                z: qz,
            },
            origin: Vector3 {
                x: ox,
                y: oy,
                z: oz,
            },
        },
    })
}

fn read_starter_area<R: Read>(r: &mut R) -> Result<StarterArea> {
    let name = read_ac_string(r)?;
    let n_locs = read_compressed_u32(r)? as usize;
    let mut locations = Vec::with_capacity(n_locs);
    for _ in 0..n_locs {
        locations.push(read_location(r)?);
    }
    Ok(StarterArea { name, locations })
}

fn skill_id_to_name(id: u32) -> String {
    use crate::enums::SkillId;
    SkillId::try_from(id as i32)
        .map(|s| format!("{}", s))
        .unwrap_or_else(|_| format!("Skill_{}", id))
}

fn read_skill_cg<R: Read>(r: &mut R) -> Result<(String, SkillCost)> {
    let skill_num = r.read_u32::<LittleEndian>()?;
    let normal_cost = r.read_i32::<LittleEndian>()?;
    let primary_cost = r.read_i32::<LittleEndian>()?;
    let name = skill_id_to_name(skill_num);
    Ok((
        name.clone(),
        SkillCost {
            id: name,
            normal_cost,
            primary_cost,
        },
    ))
}

fn read_template_cg<R: Read>(r: &mut R) -> Result<TemplateCG> {
    let name = read_ac_string(r)?;
    let icon_id = r.read_u32::<LittleEndian>()?;
    let title = r.read_u32::<LittleEndian>()?;
    let strength = r.read_u32::<LittleEndian>()?;
    let endurance = r.read_u32::<LittleEndian>()?;
    let coordination = r.read_u32::<LittleEndian>()?;
    let quickness = r.read_u32::<LittleEndian>()?;
    let focus = r.read_u32::<LittleEndian>()?;
    let self_stat = r.read_u32::<LittleEndian>()?;
    let normal_skills = read_smart_array_u32(r)?;
    let primary_skills = read_smart_array_u32(r)?;
    Ok(TemplateCG {
        name,
        icon_id,
        title,
        strength,
        endurance,
        coordination,
        quickness,
        focus,
        self_stat,
        normal_skills,
        primary_skills,
    })
}

fn read_heritage_group<R: Read + Seek>(r: &mut R) -> Result<HeritageGroupCG> {
    let name = read_ac_string(r)?;
    let icon_id = r.read_u32::<LittleEndian>()?;
    let setup_id = r.read_u32::<LittleEndian>()?;
    let env_setup_id = r.read_u32::<LittleEndian>()?;
    let attribute_credits = r.read_u32::<LittleEndian>()?;
    let skill_credits = r.read_u32::<LittleEndian>()?;

    let primary_start_areas = read_smart_array_i32(r)?;
    let secondary_start_areas = read_smart_array_i32(r)?;

    let n_skills = read_compressed_u32(r)? as usize;
    let mut skills = BTreeMap::new();
    for _ in 0..n_skills {
        let (name, cost) = read_skill_cg(r)?;
        skills.insert(name, cost);
    }

    let n_templates = read_compressed_u32(r)? as usize;
    let mut templates = Vec::with_capacity(n_templates);
    for _ in 0..n_templates {
        templates.push(read_template_cg(r)?);
    }

    // Skip 1 unknown byte (ACE comment: "0x01 byte here, not sure what/why")
    r.read_u8()?;

    // Genders (SmartArray<i32 key, SexCG value>) — parsed but not output
    let n_genders = read_compressed_u32(r)? as usize;
    for _ in 0..n_genders {
        r.read_i32::<LittleEndian>()?; // key (gender id)
        read_sex_cg(r)?;
    }

    Ok(HeritageGroupCG {
        name,
        icon_id,
        attribute_credits,
        skill_credits,
        skills,
        genders: BTreeMap::new(), // always empty in output
        templates,
        setup_id,
        env_setup_id,
        primary_start_areas,
        secondary_start_areas,
    })
}

impl CharGen {
    pub fn read<R: Read + Seek>(r: &mut R) -> Result<Self> {
        // The outer DatFile wrapper already consumed the first 4 bytes (file ID).
        // The chargen file then stores the same ID again as the first field.
        let id = r.read_u32::<LittleEndian>()?;

        // Starting areas
        let n_areas = read_compressed_u32(r)? as usize;
        let mut starting_areas = Vec::with_capacity(n_areas);
        for _ in 0..n_areas {
            starting_areas.push(read_starter_area(r)?);
        }

        // Skip 1 byte before heritage groups (ACE: "Not sure what this byte is")
        r.read_u8()?;

        // Heritage groups (SmartArray<u32 key, HeritageGroupCG value>)
        let n_groups = read_compressed_u32(r)? as usize;
        let mut heritage_groups = Vec::with_capacity(n_groups);
        for _ in 0..n_groups {
            r.read_u32::<LittleEndian>()?; // key (HeritageGroup enum value)
            heritage_groups.push(read_heritage_group(r)?);
        }

        Ok(CharGen {
            id,
            heritage_groups,
            starting_areas,
        })
    }
}

#[cfg(feature = "dat-export")]
impl super::Exportable for CharGen {
    fn export_to_path(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(path, json)
    }

    fn file_extension(&self) -> &str {
        "json"
    }
}
