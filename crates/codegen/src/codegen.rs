use std::{fs, path::Path};

/// Execute the full code generation workflow: read files, generate, and write output
pub fn generate_and_write(workspace_root: &Path, generated_dir: &Path) -> std::io::Result<()> {
    let protocol_path = workspace_root.join("ACProtocol/protocol.xml");
    let network_path = workspace_root.join("network.xml");

    // Clean the generated directory
    if generated_dir.exists() {
        fs::remove_dir_all(generated_dir)?;
    }

    // Read XML files
    let protocol_xml = fs::read_to_string(&protocol_path)?;
    let network_xml = if network_path.exists() {
        Some(fs::read_to_string(&network_path)?)
    } else {
        None
    };

    // Generate code
    let generated_code = crate::generate_and_merge(&protocol_xml, network_xml.as_deref());

    // Print summary before writing
    print_generation_summary(&generated_code);

    // Write all generated files
    for file in generated_code.files {
        let file_path = generated_dir.join(&file.path);

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, file.content)?;
    }

    Ok(())
}

/// Print a summary of what was generated
fn print_generation_summary(generated_code: &crate::generation::GeneratedCode) {
    let mut enum_count = 0;
    let mut struct_count = 0;

    for file in &generated_code.files {
        enum_count += count_items(&file.content, "pub enum ");
        struct_count += count_items(&file.content, "pub struct ");
    }

    println!(
        "codegen complete:\n----------------\nEnums:\t\t{}\nStructs:\t{}",
        enum_count, struct_count
    );
}

/// Count occurrences of a pattern in a string
fn count_items(content: &str, pattern: &str) -> usize {
    content.matches(pattern).count()
}
