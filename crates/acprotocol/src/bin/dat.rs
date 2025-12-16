use std::{error::Error, io::Cursor};

use acprotocol::cli_helper::find_file_by_id;
use acprotocol::dat::{DatDatabase, DatFile, DatFileType, Texture};
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

            // Step 3: Convert the buffer into our file
            // This is the common part
            let mut buf_reader = Cursor::new(buf);
            match found_file.file_type() {
                DatFileType::Texture => {
                    let outer_file: DatFile<Texture> = DatFile::read(&mut buf_reader)?;
                    let texture = outer_file.inner;
                    let output_path = format!("{}.png", object_id);
                    texture.to_png(&output_path, 1)?;
                    println!("Texture saved to {:?}", output_path);
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
    }

    Ok(())
}
