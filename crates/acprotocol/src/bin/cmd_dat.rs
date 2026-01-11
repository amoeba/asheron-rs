use std::{error::Error, io::Cursor};

use acprotocol::dat::{find_file_by_id, DatDatabase, DatFile, DatFileType, Texture, IconExportOptions};
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
        #[arg(short, long, default_value = "1", help = "Scale factor for exported textures (1-10)")]
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
        #[arg(long = "type", help = "Filter files by type (Texture, Unknown)")]
        file_type: Option<String>,
    },
    Icon {
        #[arg(help = "Path to DAT file", short('f'), long("file"))]
        dat_file: String,
        #[arg(help = "Icon texture ID (required)", long)]
        icon_id: String,
        #[arg(help = "Underlay texture ID (optional)", long)]
        underlay: Option<String>,
        #[arg(help = "Overlay texture ID (optional)", long)]
        overlay: Option<String>,
        #[arg(help = "Second overlay texture ID (optional)", long)]
        overlay2: Option<String>,
        #[arg(help = "UI effect texture ID (optional)", long)]
        ui_effect: Option<String>,
        #[arg(short, long, default_value = "icon.png", help = "Output PNG file")]
        output: String,
        #[arg(short, long, default_value = "1", help = "Scale factor (1-10)")]
        scale: u32,
    },
}

#[cfg(feature = "dat-tokio")]
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    use acprotocol::dat::reader::file_reader::FileRangeReader;

    let cli = Cli::parse();

    match cli.command {
        Commands::Extract {
            dat_file,
            object_id,
            output_dir,
            scale,
        } => {
            use acprotocol::dat::reader::dat_file_reader::DatFileReader;

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
                    println!("Texture format: {:?}, size: {}x{}", texture.format, texture.width, texture.height);
                    println!("Unknown field: {}", texture.unknown);
                    println!("Data length: {} bytes (expected: {} for {}x{})",
                        texture.data.len(),
                        texture.width * texture.height * 4,
                        texture.width,
                        texture.height);
                    println!("Bytes read from buffer: {}, remaining: {}", bytes_read, buf.len() - bytes_read as usize);

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
            // Use async database reading with RangeReader
            let file = tokio::fs::File::open(&dat_file).await?;
            let compat_file = tokio_util::compat::TokioAsyncReadCompatExt::compat(file);
            let mut range_reader = FileRangeReader::new(compat_file);

            let dat = DatDatabase::read_async(&mut range_reader).await?;
            let mut files = dat.list_files(true)?;

            // Filter by type if specified
            if let Some(type_str) = file_type {
                let filter_type = match type_str.to_lowercase().as_str() {
                    "texture" => DatFileType::Texture,
                    "unknown" => DatFileType::Unknown,
                    _ => {
                        eprintln!(
                            "Invalid file type: {}. Valid types are: Texture, Unknown",
                            type_str
                        );
                        return Ok(());
                    }
                };
                files.retain(|file| file.file_type() == filter_type);
            }

            if count {
                println!("{}", files.len());
            } else {
                println!(
                    "{:<10} {:<10} {:<10} {:<10}",
                    "ID", "OFFSET", "SIZE", "TYPE"
                );
                for file in files {
                    println!(
                        "{:08X} {:<10} {:<10} {:<10}",
                        file.object_id,
                        file.file_offset,
                        file.file_size,
                        file.file_type()
                    );
                }
            }
        }
        Commands::Icon {
            dat_file,
            icon_id,
            underlay,
            overlay,
            overlay2,
            ui_effect,
            output,
            scale,
        } => {
            use acprotocol::dat::Icon;

            // Helper function to load a texture by ID
            async fn load_texture(
                dat_file: &str,
                dat: &DatDatabase,
                texture_id: &str,
            ) -> Result<Texture, Box<dyn std::error::Error>> {
                use acprotocol::dat::reader::file_reader::FileRangeReader;
                use acprotocol::dat::reader::dat_file_reader::DatFileReader;

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
            let width = icon_texture.width as u32;
            let height = icon_texture.height as u32;

            // Build icon with optional layers
            let mut icon = Icon {
                width,
                height,
                scale,
                base: icon_texture,
                underlay: None,
                overlay: None,
                overlay2: None,
                effect: None,
            };

            if let Some(ref underlay_id) = underlay {
                println!("Loading underlay: {}", underlay_id);
                icon.underlay = Some(load_texture(&dat_file, &dat, underlay_id).await?);
            }

            if let Some(ref overlay_id) = overlay {
                println!("Loading overlay: {}", overlay_id);
                icon.overlay = Some(load_texture(&dat_file, &dat, overlay_id).await?);
            }

            if let Some(ref overlay2_id) = overlay2 {
                println!("Loading overlay2: {}", overlay2_id);
                icon.overlay2 = Some(load_texture(&dat_file, &dat, overlay2_id).await?);
            }

            if let Some(ref effect_id) = ui_effect {
                println!("Loading ui_effect: {}", effect_id);
                icon.effect = Some(load_texture(&dat_file, &dat, effect_id).await?);
            }

            println!("Compositing icon with white-to-black conversion");
            let options = IconExportOptions {
                convert_white_to_black: true,
            };
            let buf = icon.export_with_options(&options)?;
            std::fs::write(&output, buf)?;
            println!("Saved composited icon to {} ({}x{} @ {}x scale)", output, width, height, scale);
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
            if let Some(type_str) = file_type {
                let filter_type = match type_str.to_lowercase().as_str() {
                    "texture" => DatFileType::Texture,
                    "unknown" => DatFileType::Unknown,
                    _ => {
                        eprintln!(
                            "Invalid file type: {}. Valid types are: Texture, Unknown",
                            type_str
                        );
                        return Ok(());
                    }
                };
                files.retain(|file| file.file_type() == filter_type);
            }

            if count {
                println!("{}", files.len());
            } else {
                println!(
                    "{:<10} {:<10} {:<10} {:<10}",
                    "ID", "OFFSET", "SIZE", "TYPE"
                );
                for file in files {
                    println!(
                        "{:08X} {:<10} {:<10} {:<10}",
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
            underlay: _,
            overlay: _,
            overlay2: _,
            ui_effect: _,
            output: _,
            scale: _,
        } => {
            eprintln!("Icon compositing requires the 'dat-tokio' feature. Please rebuild with --features=\"dat-tokio dat-export\"");
            std::process::exit(1);
        }
    }

    Ok(())
}
