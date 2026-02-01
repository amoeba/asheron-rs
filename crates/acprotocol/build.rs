use std::{env, path::PathBuf, fs};

fn main() {
    env_logger::init();

    // Get the workspace root (two levels up from acprotocol)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
    let protocol_path = workspace_root.join("ACProtocol/protocol.xml");
    let network_path = workspace_root.join("network.xml");
    let ace_path = workspace_root.join("ace.xml");
    let generated_dir = manifest_dir.join("src/generated");

    println!("cargo:rerun-if-changed={}", protocol_path.display());
    if network_path.exists() {
        println!("cargo:rerun-if-changed={}", network_path.display());
    }
    println!("cargo:rerun-if-changed={}", ace_path.display());

    // Use shared code generation workflow
    codegen::codegen::generate_and_write(workspace_root, &generated_dir)
        .expect("Code generation failed");

    // Generate ace enums from ace.xml
    if ace_path.exists() {
        let ace_xml = fs::read_to_string(&ace_path)
            .expect("Failed to read ace.xml");
        let generated_ace = codegen::generate_ace(&ace_xml)
            .expect("ACE code generation failed");
        
        let ace_rs = manifest_dir.join("src/generated/ace.rs");
        fs::write(&ace_rs, &generated_ace)
            .expect("Failed to write generated/ace.rs");
        
        // Ensure ace module is in generated/mod.rs
        let mod_rs = manifest_dir.join("src/generated/mod.rs");
        let mut mod_content = fs::read_to_string(&mod_rs)
            .unwrap_or_default();
        if !mod_content.contains("pub mod ace;") {
            mod_content.push_str("pub mod ace;\n");
            fs::write(&mod_rs, &mod_content)
                .expect("Failed to write generated/mod.rs");
        }
    }
}
