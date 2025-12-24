use crate::type_utils::get_rust_type;

use crate::generation::context::ReaderContext;

/// Generate a Vec write
pub fn generate_vec_write(element_type: &str, field_name: &str) -> String {
    let rust_type = get_rust_type(element_type);

    // Use generic write_vec function
    format!("write_vec::<{}>(writer, &self.{})?", rust_type, field_name)
}

/// Generate a PackableList write (count + vector of items)
pub fn generate_packable_list_write(element_type: &str, field_name: &str) -> String {
    let rust_type = get_rust_type(element_type);
    format!(
        "write_packable_list::<{}>(writer, &self.{})?",
        rust_type, field_name
    )
}

/// Generate a PackableHashTable write (count + max_size + table of items)
pub fn generate_packable_hash_table_write(
    key_type: &str,
    value_type: &str,
    field_name: &str,
    _ctx: &ReaderContext,
) -> String {
    let key_rust_type = get_rust_type(key_type);
    let value_rust_type = get_rust_type(value_type);

    format!(
        "write_packable_hash_table::<{}, {}>(writer, &self.{})?",
        key_rust_type, value_rust_type, field_name
    )
}

/// Generate a PHashTable write (packed_size + table of items)
pub fn generate_phash_table_write(key_type: &str, value_type: &str, field_name: &str) -> String {
    let key_rust = get_rust_type(key_type);
    let value_rust = get_rust_type(value_type);
    format!(
        "write_phash_table::<{}, {}>(writer, &self.{})?",
        key_rust, value_rust, field_name
    )
}

/// Generate a HashMap write
pub fn generate_hashmap_write(
    key_type: &str,
    value_type: &str,
    field_name: &str,
    _ctx: &ReaderContext,
) -> String {
    let key_rust_type = get_rust_type(key_type);
    let value_rust_type = get_rust_type(value_type);

    // Write each key-value pair
    format!(
        "(|| -> Result<(), Box<dyn std::error::Error>> {{\n            for (key, value) in &self.{} {{\n                {}::write(key, writer)?;\n                {}::write(value, writer)?;\n            }}\n            Ok(())\n        }})()?",
        field_name, key_rust_type, value_rust_type
    )
}
