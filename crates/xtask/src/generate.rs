use std::path::PathBuf;

pub fn generate() {
    // Get the workspace root
    let xtask_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = xtask_dir.parent().unwrap().parent().unwrap();

    let generated_dir = workspace_root.join("./crates/acprotocol/src/generated");

    // Read FILTER_TYPES env var - comma-separated list of types to generate readers for
    let filter_types = env::var("FILTER_TYPES")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    if !filter_types.is_empty() {
        println!("Generating readers for types: {:?}", filter_types);
    }

    // Use shared code generation workflow
    codegen::codegen::generate_and_write(workspace_root, &generated_dir, &filter_types)
        .expect("Code generation failed");

    println!("Code generation complete!");
}
