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
