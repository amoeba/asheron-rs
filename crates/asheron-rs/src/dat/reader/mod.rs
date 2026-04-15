pub mod dat_block_reader;
pub mod range_reader;
pub mod sync_file_reader;
pub mod types;

#[cfg(feature = "dat-tokio")]
pub mod dat_file_reader;
#[cfg(feature = "dat-tokio")]
pub mod file_reader;

#[cfg(feature = "dat-http")]
pub mod http_reader;

#[cfg(feature = "dat-cloudflare")]
pub mod worker_r2_reader;

pub mod sync_dat_file_reader;

// Re-export commonly used types
pub use types::{
    dat_block::DatBlock, dat_database::DatDatabase, dat_database_header::DatDatabaseHeader,
    dat_directory::DatDirectory, dat_directory_entry::DatDirectoryEntry,
    dat_directory_header::DatDirectoryHeader,
};

pub use sync_dat_file_reader::SyncDatFileReader;
pub use sync_file_reader::SyncFileRangeReader;

#[cfg(feature = "dat-tokio")]
pub use dat_file_reader::DatFileReader;
#[cfg(feature = "dat-tokio")]
pub use file_reader::FileRangeReader;

#[cfg(feature = "dat-http")]
pub use http_reader::{HttpRangeReader, HttpRangeReaderOptions};

#[cfg(feature = "dat-cloudflare")]
pub use worker_r2_reader::WorkerR2RangeReader;
