pub mod codegen;
pub mod identifiers;
pub mod types;
pub mod util;

mod field_gen;
pub mod generation;
mod type_utils;
mod xml;
mod xml_parser;

pub use generation::context::ReaderContext;
pub use generation::{
    GenerateSource, GeneratedCode, GeneratedFile, generate, generate_and_merge,
    generate_with_source,
};
pub use type_utils::get_rust_type;
pub use types::ProtocolCategory;

/// Generate treasure enums from ace.xml
/// Returns the Rust source code as a string
pub fn generate_ace(ace_xml: &str) -> std::io::Result<String> {
    let parse_result = xml_parser::parse_xml_content(ace_xml, GenerateSource::Protocol);

    let mut output = String::new();
    output.push_str("//! ACE-specific enums for loot generation and game mechanics.\n");
    output.push_str("//! Based on ACEmulator's enums.\n");
    output.push_str("//! NOTE: This file is generated. Do not edit manually.\n\n");

    output.push_str("use serde::{Deserialize, Serialize};\n");
    output.push_str("use num_enum::TryFromPrimitive;\n");
    output.push_str("use crate::readers::ACReader;\n");
    output.push_str("use crate::writers::ACWriter;\n\n");

    for protocol_enum in &parse_result.enums {
        if protocol_enum.is_mask {
            output.push_str(&generation::enum_generation::generate_bitflags(
                protocol_enum,
            ));
        } else {
            output.push_str(&generation::enum_generation::generate_enum(protocol_enum));
        }
    }

    Ok(output)
}
