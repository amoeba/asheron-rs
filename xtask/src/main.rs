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
    let mut generated_code = genlib::generate(&protocol_xml, &filter_types);

    // Generate from network.xml if it exists and merge results
    if network_path.exists() {
        println!("Processing additional types from: {}", network_path.display());
        let network_xml = fs::read_to_string(&network_path).unwrap();
        let network_code = genlib::generate(&network_xml, &filter_types);
        
        // Merge files from network.xml into generated_code
        // For mod.rs files, only merge if they have actual content (not just pub mod declarations)
        for network_file in network_code.files {
            if network_file.path.ends_with("mod.rs") {
                // Skip empty or minimal mod.rs files from network.xml
                let has_real_content = !network_file.content.trim().is_empty() 
                    && !network_file.content.lines().all(|line| line.trim().is_empty() || line.trim().starts_with("pub mod") || line.trim().starts_with("pub use"));
                
                if has_real_content {
                    // Find and merge with existing mod.rs file
                    if let Some(existing) = generated_code.files.iter_mut().find(|f| f.path == network_file.path) {
                        // Strip common imports/headers from network file before appending
                        let mut network_content = network_file.content.clone();
                        // Remove use statements and #[allow] attributes from the start
                        while let Some(line) = network_content.lines().next() {
                            let trimmed = line.trim();
                            if trimmed.starts_with("use ") 
                                || trimmed.starts_with("#[allow") 
                                || trimmed.is_empty() {
                                // Remove this line
                                if let Some(newline_pos) = network_content.find('\n') {
                                    network_content = network_content[newline_pos + 1..].to_string();
                                } else {
                                    network_content.clear();
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        
                        // Append network content to existing content
                        if !network_content.trim().is_empty() {
                            existing.content.push('\n');
                            existing.content.push_str(&network_content);
                        }
                    } else {
                        // No existing mod.rs, just add it
                        generated_code.files.push(network_file);
                    }
                }
                // If no real content, skip it entirely (don't add or merge)
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
