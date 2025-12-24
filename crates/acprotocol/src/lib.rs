mod generated {
    include!("generated/mod.rs");
}

pub use generated::enums;
pub use generated::gameactions;
pub use generated::gameevents;
pub use generated::message;
pub use generated::messages;
pub use generated::types;

pub mod cli_helper;
pub mod constants;
pub mod dat;
pub mod filter;
pub mod network;
pub mod readers;
pub mod writers;

#[cfg(feature = "cli")]
pub mod cli;
