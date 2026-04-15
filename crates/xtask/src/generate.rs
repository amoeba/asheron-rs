use std::{env, fs, path::PathBuf};

pub fn generate() {
    let xtask_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = xtask_dir.parent().unwrap().parent().unwrap();
    let generated_dir = workspace_root.join("crates/asheron-rs/src/generated");
    let ace_path = workspace_root.join("ace.xml");

    codegen::codegen::generate_and_write(workspace_root, &generated_dir)
        .expect("Code generation failed");

    if ace_path.exists() {
        let ace_xml = fs::read_to_string(&ace_path).expect("Failed to read ace.xml");
        let generated_ace = codegen::generate_ace(&ace_xml).expect("ACE code generation failed");

        let ace_rs = generated_dir.join("ace.rs");
        fs::write(&ace_rs, &generated_ace).expect("Failed to write generated/ace.rs");

        let mod_rs = generated_dir.join("mod.rs");
        let mut mod_content = fs::read_to_string(&mod_rs).unwrap_or_default();
        if !mod_content.contains("pub mod ace;") {
            mod_content.push_str("pub mod ace;\n");
            fs::write(&mod_rs, &mod_content).expect("Failed to write generated/mod.rs");
        }
    }
}
