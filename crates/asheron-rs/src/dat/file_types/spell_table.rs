use byteorder::{LittleEndian, ReadBytesExt};
use num_traits::FromPrimitive;
use std::collections::BTreeMap;
use std::io::{Read, Result, Seek};

#[cfg(feature = "dat-export")]
use serde::Serialize;

use super::common::{
    align_boundary, read_obfuscated_string, read_packed_hash_table, read_plain_array_u32,
};
use crate::dat::SpellType;

// ── SpellRef ──────────────────────────────────────────────────────────────────

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct SpellRef {
    pub id: u32,
    pub label: String,
}

// ── SpellSetTiers ─────────────────────────────────────────────────────────────

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct SpellSetTiers {
    #[cfg_attr(feature = "dat-export", serde(rename = "Spells"))]
    pub spells: Vec<SpellRef>,
}

fn read_spell_set_tiers<R: Read>(
    r: &mut R,
    spell_lookup: &BTreeMap<u32, SpellBase>,
) -> Result<SpellSetTiers> {
    let ids = read_plain_array_u32(r)?;
    let spells = ids
        .into_iter()
        .map(|id| {
            let label = spell_lookup
                .get(&id)
                .map(|s| s.name.clone())
                .unwrap_or_default();
            SpellRef { id, label }
        })
        .collect();
    Ok(SpellSetTiers { spells })
}

// ── SpellSet ──────────────────────────────────────────────────────────────────

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct SpellSet {
    /// Keyed by piece-count (total combined item level of equipped set pieces).
    #[cfg_attr(feature = "dat-export", serde(rename = "SpellSetTiers"))]
    pub tiers: BTreeMap<u32, SpellSetTiers>,
}

fn read_spell_set<R: Read>(r: &mut R, spell_lookup: &BTreeMap<u32, SpellBase>) -> Result<SpellSet> {
    Ok(SpellSet {
        tiers: read_packed_hash_table(r, |r| read_spell_set_tiers(r, spell_lookup))?,
    })
}

// ── SpellBase ─────────────────────────────────────────────────────────────────

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct SpellBase {
    #[cfg_attr(feature = "dat-export", serde(rename = "Name"))]
    pub name: String,
    #[cfg_attr(feature = "dat-export", serde(rename = "Desc"))]
    pub desc: String,
    #[cfg_attr(feature = "dat-export", serde(rename = "School"))]
    pub school: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Icon"))]
    pub icon: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Category"))]
    pub category: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Bitfield"))]
    pub bitfield: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "BaseMana"))]
    pub base_mana: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "BaseRangeConstant"))]
    pub base_range_constant: f32,
    #[cfg_attr(feature = "dat-export", serde(rename = "BaseRangeMod"))]
    pub base_range_mod: f32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Power"))]
    pub power: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "SpellEconomyMod"))]
    pub spell_economy_mod: f32,
    #[cfg_attr(feature = "dat-export", serde(rename = "FormulaVersion"))]
    pub formula_version: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "ComponentLoss"))]
    pub component_loss: f32,
    #[cfg_attr(feature = "dat-export", serde(rename = "MetaSpellType"))]
    pub meta_spell_type: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "MetaSpellId"))]
    pub meta_spell_id: u32,
    // Present only for Enchantment / FellowEnchantment
    #[cfg_attr(
        feature = "dat-export",
        serde(rename = "Duration", skip_serializing_if = "Option::is_none")
    )]
    pub duration: Option<f64>,
    #[cfg_attr(
        feature = "dat-export",
        serde(rename = "DegradeModifier", skip_serializing_if = "Option::is_none")
    )]
    pub degrade_modifier: Option<f32>,
    #[cfg_attr(
        feature = "dat-export",
        serde(rename = "DegradeLimit", skip_serializing_if = "Option::is_none")
    )]
    pub degrade_limit: Option<f32>,
    // Present only for PortalSummon
    #[cfg_attr(
        feature = "dat-export",
        serde(rename = "PortalLifetime", skip_serializing_if = "Option::is_none")
    )]
    pub portal_lifetime: Option<f64>,
    /// Decrypted spell component IDs (up to 8; zero entries are omitted).
    #[cfg_attr(feature = "dat-export", serde(rename = "Formula"))]
    pub formula: Vec<u32>,
    #[cfg_attr(feature = "dat-export", serde(rename = "CasterEffect"))]
    pub caster_effect: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "TargetEffect"))]
    pub target_effect: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "FizzleEffect"))]
    pub fizzle_effect: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "RecoveryInterval"))]
    pub recovery_interval: f64,
    #[cfg_attr(feature = "dat-export", serde(rename = "RecoveryAmount"))]
    pub recovery_amount: f32,
    #[cfg_attr(feature = "dat-export", serde(rename = "DisplayOrder"))]
    pub display_order: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "NonComponentTargetType"))]
    pub non_component_target_type: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "ManaMod"))]
    pub mana_mod: u32,
}

/// Hash used to decrypt spell component formula (matching C# `SpellTable.ComputeHash`).
fn compute_hash(s: &str) -> u32 {
    let mut result: u32 = 0;
    for b in s.bytes() {
        let c = b as i8 as u32;
        result = c.wrapping_add(result << 4);
        if result & 0xF000_0000 != 0 {
            result = (result ^ ((result & 0xF000_0000) >> 24)) & 0x0FFF_FFFF;
        }
    }
    result
}

fn decrypt_formula(raw_comps: &[u32], name: &str, desc: &str) -> Vec<u32> {
    const HIGHEST_COMP_ID: u32 = 198;
    let name_hash = compute_hash(name);
    let desc_hash = compute_hash(desc);
    let key = (name_hash % 0x1210_7680).wrapping_add(desc_hash % 0xBEAD_CF45);
    raw_comps
        .iter()
        .map(|&comp| {
            let mut c = comp.wrapping_sub(key);
            if c > HIGHEST_COMP_ID {
                c &= 0xFF;
            }
            c
        })
        .collect()
}

fn read_spell_base<R: Read + Seek>(r: &mut R) -> Result<SpellBase> {
    let name = read_obfuscated_string(r)?;
    align_boundary(r)?;
    let desc = read_obfuscated_string(r)?;
    align_boundary(r)?;

    let school = r.read_u32::<LittleEndian>()?;
    let icon = r.read_u32::<LittleEndian>()?;
    let category = r.read_u32::<LittleEndian>()?;
    let bitfield = r.read_u32::<LittleEndian>()?;
    let base_mana = r.read_u32::<LittleEndian>()?;
    let base_range_constant = r.read_f32::<LittleEndian>()?;
    let base_range_mod = r.read_f32::<LittleEndian>()?;
    let power = r.read_u32::<LittleEndian>()?;
    let spell_economy_mod = r.read_f32::<LittleEndian>()?;
    let formula_version = r.read_u32::<LittleEndian>()?;
    let component_loss = r.read_f32::<LittleEndian>()?;
    let meta_spell_type = r.read_u32::<LittleEndian>()?;
    let meta_spell_id = r.read_u32::<LittleEndian>()?;

    let spell_type = SpellType::from_u32(meta_spell_type);
    let (duration, degrade_modifier, degrade_limit, portal_lifetime) = match spell_type {
        Some(SpellType::Enchantment) | Some(SpellType::FellowEnchantment) => {
            let d = r.read_f64::<LittleEndian>()?;
            let dm = r.read_f32::<LittleEndian>()?;
            let dl = r.read_f32::<LittleEndian>()?;
            (Some(d), Some(dm), Some(dl), None)
        }
        Some(SpellType::PortalSummon) => {
            let pl = r.read_f64::<LittleEndian>()?;
            (None, None, None, Some(pl))
        }
        _ => (None, None, None, None),
    };

    let mut raw_comps: Vec<u32> = Vec::with_capacity(8);
    for _ in 0..8 {
        let comp = r.read_u32::<LittleEndian>()?;
        if comp > 0 {
            raw_comps.push(comp);
        }
    }
    let formula = decrypt_formula(&raw_comps, &name, &desc);

    let caster_effect = r.read_u32::<LittleEndian>()?;
    let target_effect = r.read_u32::<LittleEndian>()?;
    let fizzle_effect = r.read_u32::<LittleEndian>()?;
    let recovery_interval = r.read_f64::<LittleEndian>()?;
    let recovery_amount = r.read_f32::<LittleEndian>()?;
    let display_order = r.read_u32::<LittleEndian>()?;
    let non_component_target_type = r.read_u32::<LittleEndian>()?;
    let mana_mod = r.read_u32::<LittleEndian>()?;

    Ok(SpellBase {
        name,
        desc,
        school,
        icon,
        category,
        bitfield,
        base_mana,
        base_range_constant,
        base_range_mod,
        power,
        spell_economy_mod,
        formula_version,
        component_loss,
        meta_spell_type,
        meta_spell_id,
        duration,
        degrade_modifier,
        degrade_limit,
        portal_lifetime,
        formula,
        caster_effect,
        target_effect,
        fizzle_effect,
        recovery_interval,
        recovery_amount,
        display_order,
        non_component_target_type,
        mana_mod,
    })
}

// ── SpellTable ────────────────────────────────────────────────────────────────

#[derive(Debug)]
#[cfg_attr(feature = "dat-export", derive(Serialize))]
pub struct SpellTable {
    #[cfg_attr(feature = "dat-export", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "dat-export", serde(rename = "Spells"))]
    pub spells: BTreeMap<u32, SpellBase>,
    #[cfg_attr(feature = "dat-export", serde(rename = "SpellSet"))]
    pub spell_set: BTreeMap<u32, SpellSet>,
}

impl SpellTable {
    pub fn read<R: Read + Seek>(r: &mut R) -> Result<Self> {
        let id = r.read_u32::<LittleEndian>()?;
        let spells = read_packed_hash_table(r, read_spell_base)?;
        let spell_set = read_packed_hash_table(r, |r| read_spell_set(r, &spells))?;
        Ok(SpellTable {
            id,
            spells,
            spell_set,
        })
    }
}

#[cfg(feature = "dat-export")]
impl super::Exportable for SpellTable {
    fn export_to_path(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(path, json)
    }

    fn file_extension(&self) -> &str {
        "json"
    }
}
