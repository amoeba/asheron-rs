use std::error::Error;

use acprotocol::dat::reader::types::dat_database::DatDatabase;
use acprotocol::dat::reader::types::dat_directory_entry::DatDirectoryEntry;

pub async fn find_file_by_id(
    db: &DatDatabase,
    object_id: &str,
) -> Result<DatDirectoryEntry, Box<dyn Error>> {
    // Convert hex string to u32
    let parsed_id = if object_id.starts_with("0x") {
        u32::from_str_radix(&object_id[2..], 16)?
    } else {
        u32::from_str_radix(object_id, 16)?
    };

    println!("parsed_id: {}", parsed_id);
    let files = db.list_files(true)?;
    let target_file = files.iter().find(|file| file.object_id == parsed_id);

    match target_file {
        Some(file) => Ok(*file),
        None => {
            return Err(format!("Object ID {} not found in DAT file", object_id).into());
        }
    }
}
