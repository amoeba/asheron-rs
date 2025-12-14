use crate::type_utils::get_rust_type;

use crate::generation::context::ReaderContext;

/// Generate a Vec read with a known length
pub fn generate_vec_read_with_length(element_type: &str, length_code: &str) -> String {
    // All types that can be read (primitives, enums, or custom structs) implement ACDataType
    let rust_type = get_rust_type(element_type);

    // Special case: "*" means read all remaining bytes
    if length_code == "*" {
        // For primitive types like u8, read directly as bytes
        if rust_type == "u8" {
            // Use read_to_end which requires special handling - wrap in Ok
            return "{\n                let mut data = Vec::new();\n                let _ = reader.read_to_end(&mut data);\n                Ok::<_, Box<dyn std::error::Error>>(data)\n            }"
                .to_string();
        } else {
            // For other types, read items until EOF - wrap in Ok
            return format!(
                "{{\n                let mut vec = Vec::new();\n                loop {{\n                    match read_item::<{}>(reader) {{\n                        Ok(item) => vec.push(item),\n                        Err(_) => break,\n                    }}\n                }}\n                Ok::<_, Box<dyn std::error::Error>>(vec)\n            }}",
                rust_type
            );
        }
    }

    format!("read_vec::<{}>(reader, {})", rust_type, length_code)
}

/// Generate a PackableList read (count + vector of items)
pub fn generate_packable_list_read(element_type: &str) -> String {
    // All types that can be read (primitives, enums, or custom structs) implement ACDataType
    let rust_type = get_rust_type(element_type);
    format!("read_packable_list::<{}>(reader)", rust_type)
}

/// Generate a PackableHashTable read (count + max_size + table of items)
pub fn generate_packable_hash_table_read(
    key_type: &str,
    value_type: &str,
    _ctx: &ReaderContext,
) -> String {
    // Use the simpler generic function if available
    let key_rust_type = get_rust_type(key_type);
    let value_rust_type = get_rust_type(value_type);

    format!(
        "read_packable_hash_table::<{}, {}>(reader)",
        key_rust_type, value_rust_type
    )
}

/// Generate a PHashTable read (packed_size + table of items)
pub fn generate_phash_table_read(key_type: &str, value_type: &str) -> String {
    // All types that can be read (primitives, enums, or custom structs) implement ACDataType
    let key_rust = get_rust_type(key_type);
    let value_rust = get_rust_type(value_type);
    format!("read_phash_table::<{}, {}>(reader)", key_rust, value_rust)
}

/// Generate a HashMap read with a known length
pub fn generate_hashmap_read_with_length(
    key_type: &str,
    value_type: &str,
    length_code: &str,
    _ctx: &ReaderContext,
) -> String {
    let key_rust_type = get_rust_type(key_type);
    let value_rust_type = get_rust_type(value_type);

    // The block is an expression that evaluates to Result<HashMap<K, V>, Error>
    // We use ? inside the block and wrap in Ok() at the end
    format!(
        "(|| -> Result<_, Box<dyn std::error::Error>> {{\n            let length = {};\n            let mut map = std::collections::HashMap::with_capacity(length);\n            for _ in 0..length {{\n                let key = {}::read(reader)?;\n                let value = {}::read(reader)?;\n                map.insert(key, value);\n            }}\n            Ok(map)\n        }})()",
        length_code, key_rust_type, value_rust_type
    )
}
