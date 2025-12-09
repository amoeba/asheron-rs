use std::{env, fs, path::PathBuf};

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("generate") => generate(),
        Some(cmd) => eprintln!("Unknown command: {}", cmd),
        None => eprintln!("Usage: cargo xtask generate"),
    }
}

fn generate() {
    // Get the workspace root
    let xtask_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = xtask_dir.parent().unwrap();
    let protocol_path = workspace_root.join("ACProtocol/protocol.xml");
    let network_path = workspace_root.join("network.xml");
    let acprotocol_dir = workspace_root.join("crates/acprotocol");
    let generated_dir = acprotocol_dir.join("src/generated");

    println!("Generating from: {}", protocol_path.display());

    // Clean the generated directory to remove old structure
    if generated_dir.exists() {
        fs::remove_dir_all(&generated_dir).unwrap();
        println!("Cleaned {}", generated_dir.display());
    }

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

    // Generate from protocol.xml
    let protocol_xml = fs::read_to_string(&protocol_path).unwrap();
    let mut generated_code = genlib::generate_with_source(&protocol_xml, &filter_types, genlib::GenerateSource::Protocol);

    // Generate from network.xml if it exists and merge results
    if network_path.exists() {
        println!("Processing additional types from: {}", network_path.display());
        let network_xml = fs::read_to_string(&network_path).unwrap();
        let network_code = genlib::generate_with_source(&network_xml, &filter_types, genlib::GenerateSource::Network);
        
        // Merge files from network.xml into generated_code
        for network_file in network_code.files {
            if network_file.path.ends_with("mod.rs") {
                // For mod.rs files, merge the pub mod and pub use declarations
                if let Some(existing) = generated_code.files.iter_mut().find(|f| f.path == network_file.path) {
                    // Merge module declarations from network.xml into existing mod.rs
                    let mut merged_content = existing.content.clone();
                    
                    // Extract pub mod and pub use lines from network file
                    for line in network_file.content.lines() {
                        let trimmed = line.trim();
                        if (trimmed.starts_with("pub mod ") || trimmed.starts_with("pub use ")) 
                            && !merged_content.contains(trimmed) {
                            // Add this line if it's not already present
                            merged_content.push('\n');
                            merged_content.push_str(line);
                        }
                    }
                    
                    existing.content = merged_content;
                } else {
                    // No existing mod.rs, add network's mod.rs
                    generated_code.files.push(network_file);
                }
            } else {
                // For non-mod.rs files, just add/replace them
                if let Some(pos) = generated_code.files.iter().position(|f| f.path == network_file.path) {
                    generated_code.files[pos] = network_file;
                } else {
                    generated_code.files.push(network_file);
                }
            }
        }
    }

    // Write all generated files
    for file in generated_code.files {
        let file_path = generated_dir.join(&file.path);

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        fs::write(&file_path, file.content).unwrap();
        println!("Generated: {}", file_path.display());
    }

    println!("Code generation complete!");
}
