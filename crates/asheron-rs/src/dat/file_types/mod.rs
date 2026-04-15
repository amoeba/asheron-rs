pub mod dat_file;
pub mod texture;

// Re-export commonly used types
pub use dat_file::{DatFile, DatFileRead};
pub use texture::Texture;

#[cfg(feature = "dat-export")]
use std::io::Error;

/// Trait for DAT file types that can be exported to disk
#[cfg(feature = "dat-export")]
pub trait Exportable {
    /// Export the file to the given path
    fn export_to_path(&self, path: &str) -> Result<(), Error>;

    /// Get the file extension for this exportable type (e.g., "png", "bin")
    fn file_extension(&self) -> &str;
}
