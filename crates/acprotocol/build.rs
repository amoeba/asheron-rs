use std::{env, fs, path::PathBuf};

fn main() {
    env_logger::init();

    // Get the workspace root (two levels up from the gen crate)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
    let protocol_path = workspace_root.join("ACProtocol/protocol.xml");

    let generated_dir = manifest_dir.join("src/generated");
    let enums_dir = generated_dir.join("enums");
    let types_dir = generated_dir.join("types");
    let readers_dir = generated_dir.join("readers");
    fs::create_dir_all(&enums_dir).unwrap();
    fs::create_dir_all(&types_dir).unwrap();
    fs::create_dir_all(&readers_dir).unwrap();

    let enums_path = enums_dir.join("mod.rs");
    let types_common_path = types_dir.join("common.rs");
    let types_c2s_path = types_dir.join("c2s.rs");
    let types_s2c_path = types_dir.join("s2c.rs");
    let readers_common_path = readers_dir.join("common.rs");
    let readers_c2s_path = readers_dir.join("c2s.rs");
    let readers_s2c_path = readers_dir.join("s2c.rs");

    // Commented out for testing
    // println!("cargo:rerun-if-changed={}", protocol_path.display());

    // Read FILTER_TYPES env var - comma-separated list of types to generate readers for
    let filter_types = env::var("FILTER_TYPES")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    if !filter_types.is_empty() {
        println!("cargo:warning=Generating readers for types: {:?}", filter_types);
    }

    let xml = fs::read_to_string(&protocol_path).unwrap();
    let generated_code = genlib::generate(&xml, &filter_types);

    fs::write(enums_path, generated_code.enums).unwrap();
    fs::write(types_common_path, generated_code.common).unwrap();
    fs::write(types_c2s_path, generated_code.c2s).unwrap();
    fs::write(types_s2c_path, generated_code.s2c).unwrap();
    fs::write(readers_common_path, generated_code.readers_common).unwrap();
    fs::write(readers_c2s_path, generated_code.readers_c2s).unwrap();
    fs::write(readers_s2c_path, generated_code.readers_s2c).unwrap();
}
