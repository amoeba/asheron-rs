use std::{env, path::PathBuf};

fn main() {
    env_logger::init();

    // Get the workspace root (two levels up from acprotocol)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
    let protocol_path = workspace_root.join("ACProtocol/protocol.xml");
    let network_path = workspace_root.join("network.xml");
    let generated_dir = manifest_dir.join("src/generated");

    println!("cargo:rerun-if-changed={}", protocol_path.display());
    if network_path.exists() {
        println!("cargo:rerun-if-changed={}", network_path.display());
    }

    // Read FILTER_TYPES env var - comma-separated list of types to generate readers for
    let filter_types = env::var("FILTER_TYPES")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    if !filter_types.is_empty() {
        println!(
            "cargo:warning=Generating readers for types: {:?}",
            filter_types
        );
    }

    // Use shared code generation workflow
    codegen::codegen::generate_and_write(workspace_root, &generated_dir, &filter_types)
        .expect("Code generation failed");
}
