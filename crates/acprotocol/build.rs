use std::{env, fs, path::PathBuf};

fn main() {
    // Get the workspace root (two levels up from the gen crate)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
    let protocol_path = workspace_root.join("ACProtocol/protocol.xml");
    let dest_path = manifest_dir.join("src/generated.rs");

    // Commented out for testing
    // println!("cargo:rerun-if-changed={}", protocol_path.display());

    let xml = fs::read_to_string(&protocol_path).unwrap();
    let generated_code = genlib::generate(&xml);
    fs::write(dest_path, generated_code).unwrap();
}
