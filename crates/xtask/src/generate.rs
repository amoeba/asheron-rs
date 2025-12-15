use std::path::PathBuf;

pub fn generate() {
    let xtask_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = xtask_dir.parent().unwrap().parent().unwrap();
    let generated_dir = workspace_root.join("./crates/acprotocol/src/generated");

    codegen::codegen::generate_and_write(workspace_root, &generated_dir)
        .expect("Code generation failed");
}
