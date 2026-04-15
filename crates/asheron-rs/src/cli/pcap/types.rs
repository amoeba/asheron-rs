use clap::ValueEnum;
use serde::Serialize;

/// A simplified message representation showing only metadata and raw hex data
#[derive(Serialize)]
pub struct RawMessageOutput {
    pub id: u32,
    pub opcode: u32,
    pub message_type: String,
    pub direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    pub data_len: usize,
    pub raw: String,
    pub sequence: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iteration: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_flags: Option<u32>,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum DirectionFilter {
    Send,
    Recv,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum SortField {
    Id,
    Type,
    Direction,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Jsonl,
    Json,
    Table,
}
