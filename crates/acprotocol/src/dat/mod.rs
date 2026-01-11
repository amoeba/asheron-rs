pub mod enums;
pub mod file_types;
pub mod reader;

#[cfg(feature = "dat-core")]
pub mod utils;

#[cfg(feature = "dat-export")]
pub mod icon;

// Re-export commonly used types
pub use enums::*;
pub use file_types::{DatFile, DatFileRead, Texture};
pub use reader::{
    DatBlock, DatDatabase, DatDatabaseHeader, DatDirectory, DatDirectoryEntry, DatDirectoryHeader,
};

#[cfg(feature = "dat-core")]
pub use utils::find_file_by_id;

#[cfg(feature = "dat-export")]
pub use icon::{Icon, IconExportOptions};
