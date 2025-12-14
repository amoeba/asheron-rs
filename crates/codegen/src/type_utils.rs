/// Check if a Rust type is a primitive (including String)
pub fn is_primitive_type(rust_type: &str) -> bool {
    matches!(
        rust_type,
        "u8" | "i8"
            | "u16"
            | "i16"
            | "u32"
            | "i32"
            | "u64"
            | "i64"
            | "f32"
            | "f64"
            | "bool"
            | "String"
    )
}

/// Check if a type should be generated as a newtype struct (vs type alias)
/// Returns true if it's a semantic type with a primitive parent (not a C-style alias)
pub fn should_be_newtype_struct(type_name: &str, rust_parent_type: &str) -> bool {
    // WString is always a newtype struct (has custom UTF-16 wire format)
    if type_name == "WString" {
        return true;
    }

    // Only consider newtype for types with primitive parents
    if !is_primitive_type(rust_parent_type) {
        return false;
    }

    // Hybrid approach: Use newtype structs for semantic types (type safety)
    // but keep type aliases for C-style primitive names (WORD, DWORD, byte, etc.)
    // Heuristic: C-style aliases are all-uppercase or all-lowercase
    let is_all_upper = type_name
        .chars()
        .all(|c| c.is_uppercase() || !c.is_alphabetic());
    let is_all_lower = type_name
        .chars()
        .all(|c| c.is_lowercase() || !c.is_alphabetic());
    let is_c_style_alias = is_all_upper || is_all_lower;

    !is_c_style_alias
}

/// Map an XML type name to a Rust type name.
pub fn get_rust_type(xml_type: &str) -> &str {
    match xml_type {
        "int" => "i32",
        "uint" => "u32",
        "short" => "i16",
        "ushort" => "u16",
        "long" => "i64",
        "ulong" => "u64",
        "byte" => "u8",
        "sbyte" => "i8",
        "float" => "f32",
        "double" => "f64",
        "bool" => "bool",
        "string" => "String",
        // WString is kept as-is so it becomes a newtype struct
        // Keep other types as-is (custom types like ObjectId, Vector3, etc.)
        other => other,
    }
}

/// Split a comma-separated list of types, handling nested generics
/// For example: "string, PackableList<byte>" -> ["string", "PackableList<byte>"]
fn split_generic_params(params: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut current_start = 0;
    let mut bracket_depth = 0;

    for (byte_pos, ch) in params.char_indices() {
        match ch {
            '<' => bracket_depth += 1,
            '>' => bracket_depth -= 1,
            ',' if bracket_depth == 0 => {
                result.push(&params[current_start..byte_pos]);
                current_start = byte_pos + 1;
            }
            _ => {}
        }
    }

    // Add the last parameter
    result.push(&params[current_start..]);
    result
}

/// Convert an XML type string (including generics) to Rust type string
/// Handles types like "Vec<byte>" -> "Vec<u8>"
pub fn convert_xml_type_to_rust(xml_type: &str) -> String {
    // Handle generic types recursively
    if xml_type.contains('<') && xml_type.contains('>') {
        // Extract the outer type and inner types
        if let Some(start) = xml_type.find('<') {
            let (outer, rest) = xml_type.split_at(start);
            let inner_end = rest.rfind('>').unwrap_or(rest.len());
            let inner = &rest[1..inner_end];
            let trailer = &rest[inner_end + 1..];

            // Recursively convert inner types, handling nested generics
            let converted_inner = split_generic_params(inner)
                .iter()
                .map(|t| convert_xml_type_to_rust(t.trim()))
                .collect::<Vec<_>>()
                .join(", ");

            // Preserve outer type as-is (Vec, PackableList, etc.)
            return format!("{}<{}>{}", outer, converted_inner, trailer);
        }
    }

    // Base case: convert primitive type
    get_rust_type(xml_type).to_string()
}

/// Get the reader function name for a primitive Rust type
///
/// Returns the unqualified function name (e.g., "read_u8") that reads this type from a reader.
/// Returns None for non-primitive types (enums, collections, custom types).
#[allow(dead_code)]
pub fn get_primitive_read_call(rust_type: &str) -> Option<&'static str> {
    match rust_type {
        "u8" => Some("read_u8"),
        "i8" => Some("read_i8"),
        "u16" => Some("read_u16"),
        "i16" => Some("read_i16"),
        "u32" => Some("read_u32"),
        "i32" => Some("read_i32"),
        "u64" => Some("read_u64"),
        "i64" => Some("read_i64"),
        "f32" => Some("read_f32"),
        "f64" => Some("read_f64"),
        "bool" => Some("read_bool"),
        "String" => Some("read_string"),
        "WString" => Some("read_wstring"),
        _ => None,
    }
}

/// Get the size in bits of a primitive type
pub fn get_type_size(xml_type: &str) -> usize {
    match xml_type {
        "byte" | "sbyte" => 8,
        "short" | "ushort" => 16,
        "int" | "uint" | "float" => 32,
        "long" | "ulong" | "double" => 64,
        _ => 0, // Unknown or non-numeric type
    }
}

/// Returns the larger of two types (for underfull reads in <if> blocks)
/// If types differ in signedness but same size, prefer the first type
pub fn get_larger_type(type1: &str, type2: &str) -> String {
    let size1 = get_type_size(type1);
    let size2 = get_type_size(type2);

    if size1 >= size2 {
        type1.to_string()
    } else {
        type2.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_simple_types() {
        // Primitive types should be converted
        assert_eq!(convert_xml_type_to_rust("byte"), "u8");
        assert_eq!(convert_xml_type_to_rust("sbyte"), "i8");
        assert_eq!(convert_xml_type_to_rust("short"), "i16");
        assert_eq!(convert_xml_type_to_rust("ushort"), "u16");
        assert_eq!(convert_xml_type_to_rust("int"), "i32");
        assert_eq!(convert_xml_type_to_rust("uint"), "u32");
        assert_eq!(convert_xml_type_to_rust("long"), "i64");
        assert_eq!(convert_xml_type_to_rust("ulong"), "u64");
        assert_eq!(convert_xml_type_to_rust("float"), "f32");
        assert_eq!(convert_xml_type_to_rust("double"), "f64");
        assert_eq!(convert_xml_type_to_rust("bool"), "bool");
        assert_eq!(convert_xml_type_to_rust("string"), "String");
    }

    #[test]
    fn test_convert_custom_types() {
        // Custom types should remain unchanged
        assert_eq!(convert_xml_type_to_rust("ObjectId"), "ObjectId");
        assert_eq!(convert_xml_type_to_rust("Vector3"), "Vector3");
        assert_eq!(convert_xml_type_to_rust("WString"), "WString");
    }

    #[test]
    fn test_convert_vec_single_primitive() {
        // Vec with primitive inner type
        assert_eq!(convert_xml_type_to_rust("Vec<byte>"), "Vec<u8>");
        assert_eq!(convert_xml_type_to_rust("Vec<int>"), "Vec<i32>");
        assert_eq!(convert_xml_type_to_rust("Vec<string>"), "Vec<String>");
    }

    #[test]
    fn test_convert_vec_custom_type() {
        // Vec with custom inner type
        assert_eq!(convert_xml_type_to_rust("Vec<ObjectId>"), "Vec<ObjectId>");
        assert_eq!(convert_xml_type_to_rust("Vec<Vector3>"), "Vec<Vector3>");
    }

    #[test]
    fn test_convert_packable_list_single_param() {
        // PackableList with single type parameter
        assert_eq!(
            convert_xml_type_to_rust("PackableList<byte>"),
            "PackableList<u8>"
        );
        assert_eq!(
            convert_xml_type_to_rust("PackableList<uint>"),
            "PackableList<u32>"
        );
        assert_eq!(
            convert_xml_type_to_rust("PackableList<string>"),
            "PackableList<String>"
        );
    }

    #[test]
    fn test_convert_multi_param_generics() {
        // Types with multiple parameters
        assert_eq!(
            convert_xml_type_to_rust("PackableHashTable<string, int>"),
            "PackableHashTable<String, i32>"
        );
        assert_eq!(
            convert_xml_type_to_rust("PackableHashTable<uint, byte>"),
            "PackableHashTable<u32, u8>"
        );
        assert_eq!(
            convert_xml_type_to_rust("PHashTable<int, string>"),
            "PHashTable<i32, String>"
        );
    }

    #[test]
    fn test_convert_nested_generics() {
        // Nested generic types
        assert_eq!(
            convert_xml_type_to_rust("Vec<PackableList<byte>>"),
            "Vec<PackableList<u8>>"
        );
        assert_eq!(
            convert_xml_type_to_rust("Vec<PackableList<string>>"),
            "Vec<PackableList<String>>"
        );
        assert_eq!(
            convert_xml_type_to_rust("PackableList<Vec<byte>>"),
            "PackableList<Vec<u8>>"
        );
    }

    #[test]
    fn test_convert_deeply_nested_generics() {
        // Deeply nested generic types
        assert_eq!(
            convert_xml_type_to_rust("Vec<PackableHashTable<string, PackableList<byte>>>"),
            "Vec<PackableHashTable<String, PackableList<u8>>>"
        );
        assert_eq!(
            convert_xml_type_to_rust("PackableHashTable<uint, Vec<int>>"),
            "PackableHashTable<u32, Vec<i32>>"
        );
    }

    #[test]
    fn test_convert_preserves_whitespace_in_outer() {
        // Outer type name should be preserved as-is
        assert_eq!(convert_xml_type_to_rust("Vec<byte>"), "Vec<u8>");
        assert_eq!(
            convert_xml_type_to_rust("PackableList<int>"),
            "PackableList<i32>"
        );
    }

    #[test]
    fn test_convert_strips_and_preserves_spaces() {
        // Spaces in parameters should be normalized
        assert_eq!(
            convert_xml_type_to_rust("PackableHashTable<string, int>"),
            "PackableHashTable<String, i32>"
        );
        assert_eq!(
            convert_xml_type_to_rust("PackableHashTable<string,int>"),
            "PackableHashTable<String, i32>"
        );
    }

    #[test]
    fn test_convert_mixed_custom_and_primitive() {
        // Mix of custom and primitive types
        assert_eq!(
            convert_xml_type_to_rust("PackableHashTable<ObjectId, byte>"),
            "PackableHashTable<ObjectId, u8>"
        );
        assert_eq!(convert_xml_type_to_rust("Vec<ObjectId>"), "Vec<ObjectId>");
        assert_eq!(
            convert_xml_type_to_rust("PackableList<Vector3>"),
            "PackableList<Vector3>"
        );
    }

    #[test]
    fn test_convert_complex_real_world_example() {
        // Example from network.xml: Fragment data field
        assert_eq!(convert_xml_type_to_rust("Vec<byte>"), "Vec<u8>");

        // Example from protocol.xml: typical message structure
        assert_eq!(
            convert_xml_type_to_rust("PackableList<string>"),
            "PackableList<String>"
        );
    }

    #[test]
    fn test_convert_already_correct_types() {
        // Types that don't need conversion
        assert_eq!(convert_xml_type_to_rust("Vec<u8>"), "Vec<u8>");
        assert_eq!(convert_xml_type_to_rust("Vec<i32>"), "Vec<i32>");
        assert_eq!(convert_xml_type_to_rust("Vec<String>"), "Vec<String>");
        assert_eq!(
            convert_xml_type_to_rust("PackableHashTable<String, i32>"),
            "PackableHashTable<String, i32>"
        );
    }

    #[test]
    fn test_convert_single_letter_custom_types() {
        // Edge case: single letter custom type names
        assert_eq!(convert_xml_type_to_rust("Vec<T>"), "Vec<T>");
        assert_eq!(
            convert_xml_type_to_rust("PackableList<K>"),
            "PackableList<K>"
        );
    }

    #[test]
    fn test_convert_numeric_in_type_name() {
        // Type names with numbers
        assert_eq!(convert_xml_type_to_rust("Vector3"), "Vector3");
        assert_eq!(convert_xml_type_to_rust("Vec<Vector3>"), "Vec<Vector3>");
        assert_eq!(
            convert_xml_type_to_rust("PackableList<Uint32>"),
            "PackableList<Uint32>"
        );
    }
}
