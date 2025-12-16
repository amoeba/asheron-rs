use std::error::Error;
use std::io::Seek;
use std::{
    fs::{self, File, create_dir},
    io::{Cursor, SeekFrom},
};

use crate::dat::reader::dat_block_reader::DatBlockReader;
use crate::dat::{DatDatabase, DatDirectoryEntry, DatFile, DatFileType, Texture};

pub async fn find_file_by_id(
    db: &DatDatabase,
    object_id: &str,
) -> Result<DatDirectoryEntry, Box<dyn Error>> {
    // TODO: Factor out into testable helper
    // Convert hex string to u32
    let parsed_id = if let Some(stripped) = object_id.strip_prefix("0x") {
        u32::from_str_radix(stripped, 16)?
    } else {
        u32::from_str_radix(object_id, 16)?
    };

    println!("parsed_id: {}", parsed_id);
    let files = db.list_files(true)?;
    let target_file = files.iter().find(|file| file.object_id == parsed_id);

    match target_file {
        Some(file) => Ok(*file),
        None => Err(format!("Object ID {} not found in DAT file", object_id).into()),
    }
}

pub async fn extract_texture_by_id(
    dat_file_path: &str,
    object_id: &str,
    output_dir: &str,
) -> Result<(), Box<dyn Error>> {
    // Convert hex string to u32
    let parsed_id = if let Some(stripped) = object_id.strip_prefix("0x") {
        u32::from_str_radix(stripped, 16)?
    } else {
        u32::from_str_radix(object_id, 16)?
    };

    // Read the database to find the file entry
    let mut db_file = File::open(dat_file_path)?;
    db_file.seek(SeekFrom::Start(0))?;
    let db = DatDatabase::read(&mut db_file)?;
    let files = db.list_files(true)?;

    // Find the file with matching object ID
    let target_file = files.iter().find(|file| file.object_id == parsed_id);

    let target_file = match target_file {
        Some(file) => file,
        None => {
            eprintln!("Object ID {} not found in DAT file", object_id);
            return Ok(());
        }
    };

    // Check if it's a texture
    if target_file.file_type() != DatFileType::Texture {
        eprintln!(
            "Object ID {} is not a texture (type: {:?})",
            object_id,
            target_file.file_type()
        );
        return Ok(());
    }

    // Set up export directory
    if !fs::exists(output_dir)? {
        create_dir(output_dir)?;
    }

    // Read the texture data
    let dat_file_buffer = DatBlockReader::read(
        &mut db_file,
        target_file.file_offset,
        target_file.file_size,
        db.header.block_size,
    )?;
    let mut reader = Cursor::new(dat_file_buffer);

    let dat_file: DatFile<Texture> = DatFile::read(&mut reader)?;
    let texture = dat_file.inner;

    // Export to PNG
    let output_path = format!("{}/{}.png", output_dir, object_id);
    texture.to_png(&output_path, 1)?;

    println!(
        "Extracted texture {} ({}x{}) to {}",
        object_id, texture.width, texture.height, output_path
    );
    println!(
        "File info: offset={}, size={}",
        target_file.file_offset, target_file.file_size
    );

    Ok(())
}

use anyhow::Result;

/// Parse an opcode filter string to u32
///
/// Supports:
/// - Hex with prefix: "0xF7B1" or "0xf7b1"
/// - Decimal: "63409"
pub fn parse_opcode_filter(s: &str) -> Result<u32> {
    let s = s.trim();

    if let Some(stripped) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        // Parse as hex
        u32::from_str_radix(stripped, 16)
            .map_err(|e| anyhow::anyhow!("Invalid hex opcode '{s}': {e}"))
    } else {
        // Parse as decimal
        s.parse::<u32>()
            .map_err(|e| anyhow::anyhow!("Invalid decimal opcode '{s}': {e}"))
    }
}

/// Convert opcode string (hex format like "F7B1") to u32
pub fn opcode_str_to_u32(opcode_str: &str) -> Option<u32> {
    u32::from_str_radix(opcode_str, 16).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode_hex_with_prefix_uppercase() {
        assert_eq!(parse_opcode_filter("0xF7B1").unwrap(), 0xF7B1);
    }

    #[test]
    fn test_parse_opcode_hex_with_prefix_lowercase() {
        assert_eq!(parse_opcode_filter("0xf7b1").unwrap(), 0xf7b1);
    }

    #[test]
    fn test_parse_opcode_hex_with_prefix_mixed() {
        assert_eq!(parse_opcode_filter("0xF7b1").unwrap(), 0xF7B1);
    }

    #[test]
    fn test_parse_opcode_decimal() {
        assert_eq!(parse_opcode_filter("63409").unwrap(), 0xF7B1);
    }

    #[test]
    fn test_parse_opcode_invalid_hex() {
        assert!(parse_opcode_filter("0xGHIJ").is_err());
    }

    #[test]
    fn test_parse_opcode_invalid_decimal() {
        assert!(parse_opcode_filter("not_a_number").is_err());
    }

    #[test]
    fn test_opcode_str_to_u32() {
        assert_eq!(opcode_str_to_u32("F7B1"), Some(0xF7B1));
        assert_eq!(opcode_str_to_u32("f7b1"), Some(0xf7b1));
        assert_eq!(opcode_str_to_u32("GHIJ"), None);
    }

    // Comprehensive equivalence tests: hex and decimal representations should be identical
    #[test]
    fn test_hex_and_decimal_equivalence_f7b1() {
        // 0xF7B1 = 63409 in decimal
        let hex_result = parse_opcode_filter("0xF7B1").unwrap();
        let hex_lowercase = parse_opcode_filter("0xf7b1").unwrap();
        let decimal_result = parse_opcode_filter("63409").unwrap();

        assert_eq!(hex_result, decimal_result);
        assert_eq!(hex_lowercase, decimal_result);
        assert_eq!(hex_result, 0xF7B1);
    }

    #[test]
    fn test_hex_and_decimal_equivalence_02cd() {
        // 0x02CD = 717 in decimal
        let hex_result = parse_opcode_filter("0x02CD").unwrap();
        let hex_lowercase = parse_opcode_filter("0x02cd").unwrap();
        let decimal_result = parse_opcode_filter("717").unwrap();

        assert_eq!(hex_result, decimal_result);
        assert_eq!(hex_lowercase, decimal_result);
        assert_eq!(hex_result, 0x02CD);
    }

    #[test]
    fn test_hex_and_decimal_equivalence_edge_cases() {
        // Test 0x0000 (0 decimal)
        assert_eq!(
            parse_opcode_filter("0x0000").unwrap(),
            parse_opcode_filter("0").unwrap()
        );

        // Test 0x0001 (1 decimal)
        assert_eq!(
            parse_opcode_filter("0x0001").unwrap(),
            parse_opcode_filter("1").unwrap()
        );

        // Test 0xFFFF (65535 decimal)
        assert_eq!(
            parse_opcode_filter("0xFFFF").unwrap(),
            parse_opcode_filter("65535").unwrap()
        );
    }

    #[test]
    fn test_opcode_str_matches_parsed_filter() {
        // When we parse "0xF7B1" as a filter and convert "F7B1" as opcode string,
        // they should produce the same value
        let parsed = parse_opcode_filter("0xF7B1").unwrap();
        let converted = opcode_str_to_u32("F7B1").unwrap();
        assert_eq!(parsed, converted);

        let parsed_decimal = parse_opcode_filter("63409").unwrap();
        assert_eq!(parsed_decimal, converted);
    }

    #[test]
    fn test_decimal_and_hex_produce_identical_filtering_value() {
        // This test verifies that both input formats would match the same message
        // by ensuring they produce the same u32 value
        let test_opcode_str = "F7B1"; // As it appears in message
        let message_opcode_value = opcode_str_to_u32(test_opcode_str).unwrap();

        // Both ways of filtering should match
        let hex_filter = parse_opcode_filter("0xF7B1").unwrap();
        let decimal_filter = parse_opcode_filter("63409").unwrap();

        assert_eq!(message_opcode_value, hex_filter);
        assert_eq!(message_opcode_value, decimal_filter);
    }
}
