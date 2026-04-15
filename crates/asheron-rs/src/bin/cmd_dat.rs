use std::{error::Error, io::Cursor};

use asheron_rs::dat::{DatDatabase, DatFile, DatFileType, Texture, find_file_by_id};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dat")]
#[command(about = "A CLI tool for extracting data from DAT files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Extract {
        #[arg(
            help = "Path to DAT file (e.g., ./client_portal.dat)",
            short('f'),
            long("file")
        )]
        dat_file: String,
        #[arg(help = "Object ID to extract (e.g., 0321)")]
        object_id: String,
        #[arg(short, long, default_value = "./")]
        output_dir: String,
        #[arg(
            short,
            long,
            default_value = "1",
            help = "Scale factor for exported textures (1-10)"
        )]
        scale: u32,
    },
    Read {
        #[arg(
            help = "Path or URI to DAT file (e.g., ./client_portal.dat)",
            short('f'),
            long("file")
        )]
        uri: String,
        #[arg(short('o'), long("offset"), help = "Object ID to extract (e.g., 0321)")]
        offset: String,
        #[arg(short('s'), long("size"))]
        file_size: String,
    },
    List {
        #[arg(help = "Path to DAT file")]
        dat_file: String,
        #[arg(long, help = "Print only the total count of files")]
        count: bool,
        #[arg(long = "type", help = "Filter files by type (Texture, Unknown, Icon)")]
        file_type: Option<String>,
    },
    Icon {
        #[arg(help = "Path to DAT file", short('f'), long("file"))]
        dat_file: String,

        // Required
        #[arg(help = "Icon texture ID (required)", long)]
        icon_id: String,

        // Literal mode - direct texture IDs
        #[arg(help = "Background texture ID (literal mode)", long)]
        background_id: Option<String>,
        #[arg(help = "Underlay texture ID", long)]
        underlay_id: Option<String>,
        #[arg(help = "Overlay texture ID", long)]
        overlay_id: Option<String>,
        #[arg(help = "UI effect texture ID (literal mode)", long)]
        ui_effect_id: Option<String>,

        // Builder mode - semantic helpers
        #[arg(
            help = "Item type for automatic background (e.g., 'food', 'armor', 'random')",
            long
        )]
        item_type: Option<String>,
        #[arg(help = "UI effect name (e.g., 'fire', 'magical', 'random')", long)]
        ui_effect: Option<String>,

        // Dimensions (optional, defaults to icon texture size)
        #[arg(help = "Output width (defaults to icon texture width)", long)]
        width: Option<u32>,
        #[arg(help = "Output height (defaults to icon texture height)", long)]
        height: Option<u32>,

        // Output settings
        #[arg(short, long, default_value = "icon.png", help = "Output PNG file")]
        output: String,
        #[arg(short, long, default_value = "1", help = "Scale factor (1-10)")]
        scale: u32,
    },
    Export {
        #[arg(help = "Path to DAT file", short('f'), long("file"))]
        dat_file: String,
        #[arg(short, long, default_value = "export", help = "Output directory")]
        output: String,
    },
}

#[cfg(feature = "dat-tokio")]
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    use asheron_rs::dat::reader::file_reader::FileRangeReader;

    let cli = Cli::parse();

    match cli.command {
        Commands::Extract {
            dat_file,
            object_id,
            output_dir,
            scale,
        } => {
            use asheron_rs::dat::reader::dat_file_reader::DatFileReader;

            println!(
                "cli::extract: {:?}, {:?}, {:?}!",
                dat_file, object_id, output_dir
            );

            // Use async file operations with FileRangeReader for database reading
            let file = tokio::fs::File::open(&dat_file).await?;
            let compat_file = tokio_util::compat::TokioAsyncReadCompatExt::compat(file);
            let mut range_reader = FileRangeReader::new(compat_file);

            let dat = DatDatabase::read_async(&mut range_reader).await?;
            let found_file = find_file_by_id(&dat, &object_id).await?;
            println!("Found file: {:?}", found_file);

            // Read the file into a buffer
            // TODO: This is messy

            // TODO: Can this setup be simplified?
            let file = tokio::fs::File::open(&dat_file).await?;
            let compat_file = tokio_util::compat::TokioAsyncReadCompatExt::compat(file);

            // My actual code
            let mut file_reader: FileRangeReader<tokio_util::compat::Compat<tokio::fs::File>> =
                FileRangeReader::new(compat_file);
            let mut reader = DatFileReader::new(
                found_file.file_size as usize,
                dat.header.block_size as usize,
            )?;
            let buf = reader
                .read_file(&mut file_reader, found_file.file_offset)
                .await
                .unwrap();

            println!("Total file buffer size: {} bytes", buf.len());

            // Step 3: Convert the buffer into our file
            // This is the common part
            let mut buf_reader = Cursor::new(buf.clone());
            match found_file.file_type() {
                DatFileType::Texture => {
                    let outer_file: DatFile<Texture> = DatFile::read(&mut buf_reader)?;
                    let bytes_read = buf_reader.position();
                    let texture = outer_file.inner;
                    println!(
                        "Texture format: {:?}, size: {}x{}",
                        texture.format, texture.width, texture.height
                    );
                    println!("Unknown field: {}", texture.unknown);
                    println!(
                        "Data length: {} bytes (expected: {} for {}x{})",
                        texture.data.len(),
                        texture.width * texture.height * 4,
                        texture.width,
                        texture.height
                    );
                    println!(
                        "Bytes read from buffer: {}, remaining: {}",
                        bytes_read,
                        buf.len() - bytes_read as usize
                    );

                    let output_path = format!("{}.png", object_id);
                    texture.to_png(&output_path, scale)?;
                    println!("Texture saved to {:?} (scale: {}x)", output_path, scale);
                }
                _ => {
                    println!(
                        "Unsupported file type for extraction: {:?}",
                        found_file.file_type()
                    );
                }
            }
        }
        Commands::Read {
            uri,
            offset,
            file_size,
        } => {
            println!("uri: {}, offset: {}, file_size: {}", uri, offset, file_size);
        }
        Commands::List {
            dat_file,
            count,
            file_type,
        } => {
            use asheron_rs::dat::reader::dat_file_reader::DatFileReader;

            // Use async database reading with RangeReader
            let file = tokio::fs::File::open(&dat_file).await?;
            let compat_file = tokio_util::compat::TokioAsyncReadCompatExt::compat(file);
            let mut range_reader = FileRangeReader::new(compat_file);

            let dat = DatDatabase::read_async(&mut range_reader).await?;
            let mut files = dat.list_files(true)?;

            // Filter by type if specified
            let filter_to_icons_only = file_type
                .as_ref()
                .map(|s| s.to_lowercase() == "icon")
                .unwrap_or(false);

            if let Some(type_str) = &file_type {
                match type_str.to_lowercase().as_str() {
                    "texture" => {
                        files.retain(|file| file.file_type() == DatFileType::Texture);
                    }
                    "unknown" => {
                        files.retain(|file| file.file_type() == DatFileType::Unknown);
                    }
                    "icon" => {
                        // Filter to textures first
                        files.retain(|file| file.file_type() == DatFileType::Texture);
                    }
                    _ => {
                        eprintln!(
                            "Invalid file type: {}. Valid types are: Texture, Unknown, Icon",
                            type_str
                        );
                        return Ok(());
                    }
                }
            }

            // Build a map of file IDs to their subtype (Icon or None)
            // This requires reading texture headers for all texture files
            use std::collections::HashMap;
            let mut icon_map: HashMap<u32, bool> = HashMap::new();

            let texture_files: Vec<_> = files
                .iter()
                .filter(|f| f.file_type() == DatFileType::Texture)
                .collect();

            for file_entry in &texture_files {
                // Read texture data to check dimensions
                let read_result = async {
                    let file = tokio::fs::File::open(&dat_file).await?;
                    let compat_file = tokio_util::compat::TokioAsyncReadCompatExt::compat(file);
                    let mut file_reader = FileRangeReader::new(compat_file);
                    let mut reader = DatFileReader::new(
                        file_entry.file_size as usize,
                        dat.header.block_size as usize,
                    )?;
                    reader
                        .read_file(&mut file_reader, file_entry.file_offset)
                        .await
                }
                .await;

                if let Ok(buf) = read_result {
                    let mut buf_reader = Cursor::new(buf);
                    if let Ok(outer_file) = DatFile::<Texture>::read(&mut buf_reader) {
                        let texture = outer_file.inner;
                        let is_icon = texture.width == 32 && texture.height == 32;
                        icon_map.insert(file_entry.object_id, is_icon);
                    }
                }
            }

            // If filtering to icons only, keep only those with 32x32 dimensions
            if filter_to_icons_only {
                files.retain(|f| icon_map.get(&f.object_id).copied().unwrap_or(false));
            }

            if count {
                println!("{}", files.len());
            } else {
                println!(
                    "{:<12} {:<10} {:<10} {:<20}",
                    "ID", "OFFSET", "SIZE", "TYPE"
                );
                for file in files {
                    let is_icon = icon_map.get(&file.object_id).copied().unwrap_or(false);
                    let type_display = if is_icon {
                        format!("{} (Icon)", file.file_type())
                    } else {
                        format!("{}", file.file_type())
                    };
                    println!(
                        "0x{:08X} {:<10} {:<10} {:<20}",
                        file.object_id, file.file_offset, file.file_size, type_display
                    );
                }
            }
        }
        Commands::Icon {
            dat_file,
            icon_id,
            background_id,
            underlay_id,
            overlay_id,
            ui_effect_id,
            item_type,
            ui_effect,
            width: width_override,
            height: height_override,
            output,
            scale,
        } => {
            use asheron_rs::dat::icon::{
                Icon, get_background_from_item_type, get_ui_effect_texture_id, parse_item_type,
                parse_ui_effect,
            };

            // Helper function to load a texture by ID
            async fn load_texture(
                dat_file: &str,
                dat: &DatDatabase,
                texture_id: &str,
            ) -> Result<Texture, Box<dyn std::error::Error>> {
                use asheron_rs::dat::reader::dat_file_reader::DatFileReader;
                use asheron_rs::dat::reader::file_reader::FileRangeReader;

                let found_file = find_file_by_id(dat, texture_id).await?;

                let file = tokio::fs::File::open(dat_file).await?;
                let compat_file = tokio_util::compat::TokioAsyncReadCompatExt::compat(file);
                let mut file_reader = FileRangeReader::new(compat_file);
                let mut reader = DatFileReader::new(
                    found_file.file_size as usize,
                    dat.header.block_size as usize,
                )?;
                let buf = reader
                    .read_file(&mut file_reader, found_file.file_offset)
                    .await?;

                let mut buf_reader = Cursor::new(buf);
                let outer_file: DatFile<Texture> = DatFile::read(&mut buf_reader)?;
                Ok(outer_file.inner)
            }

            let file = tokio::fs::File::open(&dat_file).await?;
            let compat_file = tokio_util::compat::TokioAsyncReadCompatExt::compat(file);
            let mut range_reader = FileRangeReader::new(compat_file);
            let dat = DatDatabase::read_async(&mut range_reader).await?;

            // Load the base icon texture
            println!("Loading icon: {}", icon_id);
            let icon_texture = load_texture(&dat_file, &dat, &icon_id).await?;

            // Use overrides if provided, otherwise use icon texture dimensions
            let width = width_override.unwrap_or(icon_texture.width as u32);
            let height = height_override.unwrap_or(icon_texture.height as u32);

            // Build icon with optional layers
            let mut icon = Icon {
                width,
                height,
                scale,
                background: None,
                underlay: None,
                icon: icon_texture,
                overlay: None,
                effect: None,
            };

            // BACKGROUND: Literal mode (background_id) takes precedence over builder mode (item_type)
            if let Some(ref bg_id) = background_id {
                println!("Loading background (literal): {}", bg_id);
                icon.background = Some(load_texture(&dat_file, &dat, bg_id).await?);
            } else if let Some(ref item_type_str) = item_type {
                // Builder mode: parse item type and get automatic background
                match parse_item_type(item_type_str) {
                    Ok(item_type_value) => {
                        let bg_texture_id = get_background_from_item_type(item_type_value);
                        let bg_id_str = format!("{:08X}", bg_texture_id);
                        println!(
                            "Loading background (builder, item_type={}): {}",
                            item_type_str, bg_id_str
                        );
                        icon.background = Some(load_texture(&dat_file, &dat, &bg_id_str).await?);
                    }
                    Err(e) => {
                        eprintln!("Error parsing item type '{}': {}", item_type_str, e);
                        return Err(e.into());
                    }
                }
            }

            // UNDERLAY: Always explicit ID
            if let Some(ref underlay_id_str) = underlay_id {
                println!("Loading underlay: {}", underlay_id_str);
                icon.underlay = Some(load_texture(&dat_file, &dat, underlay_id_str).await?);
            }

            // OVERLAY: Always explicit ID
            if let Some(ref overlay_id_str) = overlay_id {
                println!("Loading overlay: {}", overlay_id_str);
                icon.overlay = Some(load_texture(&dat_file, &dat, overlay_id_str).await?);
            }

            // UI EFFECT: Literal mode (ui_effect_id) takes precedence over builder mode (ui_effect)
            if let Some(ref effect_id) = ui_effect_id {
                println!("Loading ui_effect (literal): {}", effect_id);
                icon.effect = Some(load_texture(&dat_file, &dat, effect_id).await?);
            } else if let Some(ref effect_str) = ui_effect {
                // Builder mode: parse UI effect name and get automatic effect texture
                match parse_ui_effect(effect_str) {
                    Ok(ui_effect_flags) => {
                        let effect_texture_id = get_ui_effect_texture_id(ui_effect_flags);
                        let effect_id_str = format!("{:08X}", effect_texture_id);
                        println!(
                            "Loading ui_effect (builder, ui_effect={}): {}",
                            effect_str, effect_id_str
                        );
                        icon.effect = Some(load_texture(&dat_file, &dat, &effect_id_str).await?);
                    }
                    Err(e) => {
                        eprintln!("Error parsing UI effect '{}': {}", effect_str, e);
                        return Err(e.into());
                    }
                }
            } else if icon.background.is_some() || icon.underlay.is_some() || icon.overlay.is_some()
            {
                // Compositing with layers but no effect specified - use default UNDEF effect
                // This replaces white pixels with black (matching Lifestoned behavior)
                println!("Loading default effect (UNDEF): 060011C5");
                icon.effect = Some(load_texture(&dat_file, &dat, "060011C5").await?);
            }

            println!("Compositing icon layers");
            let buf = icon.export()?;
            std::fs::write(&output, buf)?;
            println!(
                "Saved composited icon to {} ({}x{} @ {}x scale)",
                output, width, height, scale
            );
        }
        Commands::Export { dat_file, output } => {
            use asheron_rs::dat::Exportable;
            use asheron_rs::dat::reader::dat_file_reader::DatFileReader;
            use std::path::Path;

            println!("Exporting all files from {} to {}", dat_file, output);

            // Check if output directory already exists
            if tokio::fs::metadata(&output).await.is_ok() {
                eprintln!(
                    "Error: Output directory '{}' already exists. Please delete it before running export.",
                    output
                );
                std::process::exit(1);
            }

            // Read the DAT database
            let file = tokio::fs::File::open(&dat_file).await?;
            let compat_file = tokio_util::compat::TokioAsyncReadCompatExt::compat(file);
            let mut range_reader = FileRangeReader::new(compat_file);
            let dat = DatDatabase::read_async(&mut range_reader).await?;

            // Get all files and filter by exportable types
            let all_files = dat.list_files(true)?;
            let files: Vec<_> = all_files
                .iter()
                .filter(|f| f.file_type() == DatFileType::Texture)
                .collect();

            println!(
                "Found {} exportable files (out of {} total)",
                files.len(),
                all_files.len()
            );

            // Create output directory
            tokio::fs::create_dir_all(&output).await?;

            // Create subdirectories for each file type
            let texture_dir = Path::new(&output).join("Texture");
            tokio::fs::create_dir_all(&texture_dir).await?;

            // Export each file
            let mut texture_count = 0;
            let mut error_count = 0;

            for (index, file_entry) in files.iter().enumerate() {
                if index % 100 == 0 {
                    println!("Processing file {}/{}", index + 1, files.len());
                }

                let file_type = file_entry.file_type();
                let object_id = format!("{:08X}", file_entry.object_id);

                // Read file data
                let read_result = async {
                    let file = tokio::fs::File::open(&dat_file).await?;
                    let compat_file = tokio_util::compat::TokioAsyncReadCompatExt::compat(file);
                    let mut file_reader = FileRangeReader::new(compat_file);
                    let mut reader = DatFileReader::new(
                        file_entry.file_size as usize,
                        dat.header.block_size as usize,
                    )?;
                    reader
                        .read_file(&mut file_reader, file_entry.file_offset)
                        .await
                }
                .await;

                let buf = match read_result {
                    Ok(buf) => buf,
                    Err(e) => {
                        eprintln!("Error reading file {}: {}", object_id, e);
                        error_count += 1;
                        continue;
                    }
                };

                // Parse and export based on file type
                match file_type {
                    DatFileType::Texture => {
                        let mut buf_reader = Cursor::new(buf);
                        match DatFile::<Texture>::read(&mut buf_reader) {
                            Ok(outer_file) => {
                                let texture = outer_file.inner;
                                let extension = texture.file_extension();
                                let output_path =
                                    texture_dir.join(format!("0x{}.{}", object_id, extension));
                                match texture.export_to_path(&output_path.to_string_lossy()) {
                                    Ok(_) => texture_count += 1,
                                    Err(e) => {
                                        eprintln!("Error exporting texture {}: {}", object_id, e);
                                        error_count += 1;
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error parsing texture {}: {}", object_id, e);
                                error_count += 1;
                            }
                        }
                    }
                    _ => {
                        // This shouldn't happen due to our filter, but handle it anyway
                        eprintln!("Unexpected file type: {:?}", file_type);
                    }
                }
            }

            println!("\nExport complete!");
            println!("  Textures exported: {}", texture_count);
            println!("  Errors: {}", error_count);
            println!("  Total: {}", texture_count + error_count);
        }
    }

    Ok(())
}

#[cfg(not(feature = "dat-tokio"))]
fn main() -> Result<(), Box<dyn Error>> {
    use std::fs::File;

    let cli = Cli::parse();

    match cli.command {
        Commands::Extract {
            dat_file,
            object_id,
            output_dir,
            scale,
        } => {
            println!(
                "Extract: {:?}, {:?}, {:?}!",
                dat_file, object_id, output_dir
            );

            // TODO
            // 1. Determine method to use from uri
            // 2. Do the read
        }
        Commands::Read {
            uri,
            offset,
            file_size,
        } => {
            println!("uri: {}, offset: {}, file_size: {}", uri, offset, file_size);
        }
        Commands::List {
            dat_file,
            count,
            file_type,
        } => {
            let mut db_file = File::open(&dat_file)?;
            let dat = DatDatabase::read(&mut db_file)?;
            let mut files = dat.list_files(true)?;

            // Filter by type if specified
            if let Some(type_str) = &file_type {
                match type_str.to_lowercase().as_str() {
                    "texture" => {
                        files.retain(|file| file.file_type() == DatFileType::Texture);
                    }
                    "unknown" => {
                        files.retain(|file| file.file_type() == DatFileType::Unknown);
                    }
                    "icon" => {
                        eprintln!(
                            "Icon subtype filtering requires the 'dat-tokio' feature. Please rebuild with --features=\"dat-tokio dat-export\""
                        );
                        std::process::exit(1);
                    }
                    _ => {
                        eprintln!(
                            "Invalid file type: {}. Valid types are: Texture, Unknown, Icon",
                            type_str
                        );
                        return Ok(());
                    }
                }
            }

            if count {
                println!("{}", files.len());
            } else {
                println!(
                    "{:<12} {:<10} {:<10} {:<10}",
                    "ID", "OFFSET", "SIZE", "TYPE"
                );
                for file in files {
                    println!(
                        "0x{:08X} {:<10} {:<10} {:<10}",
                        file.object_id,
                        file.file_offset,
                        file.file_size,
                        file.file_type()
                    );
                }
            }
        }
        Commands::Icon {
            dat_file: _,
            icon_id: _,
            background_id: _,
            underlay_id: _,
            overlay_id: _,
            ui_effect_id: _,
            item_type: _,
            ui_effect: _,
            width: _,
            height: _,
            output: _,
            scale: _,
        } => {
            eprintln!(
                "Icon compositing requires the 'dat-tokio' feature. Please rebuild with --features=\"dat-tokio dat-export\""
            );
            std::process::exit(1);
        }
        Commands::Export {
            dat_file: _,
            output: _,
        } => {
            eprintln!(
                "Export requires the 'dat-tokio' feature. Please rebuild with --features=\"dat-tokio dat-export\""
            );
            std::process::exit(1);
        }
    }

    Ok(())
}
