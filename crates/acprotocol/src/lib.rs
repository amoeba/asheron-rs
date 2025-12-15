mod generated {
    include!("generated/mod.rs");
}

pub use generated::enums;
pub use generated::gameactions;
pub use generated::gameevents;
pub use generated::messages;
pub use generated::types;
pub use generated::unified;

pub mod constants;
pub mod network;
pub mod readers;
