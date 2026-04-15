use std::error::Error;

use crate::dat::{DatDatabase, DatDirectoryEntry};

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
