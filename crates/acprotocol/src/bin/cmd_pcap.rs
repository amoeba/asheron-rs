use anyhow::Result;
use clap::{Parser, Subcommand};

use acprotocol::cli::pcap::{
    format_parsed_messages, format_raw_messages, output_messages, print_summary, DirectionFilter,
    OutputFormat, SortField,
};
use acprotocol::cli::tui;
use acprotocol::network::pcap;
use acprotocol::network::FragmentAssembler;

#[derive(Parser)]
#[command(name = "pcap")]
#[command(about = "Parse Asheron's Call PCAP files", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show messages in JSON format (or summary if --summary is used)
    Print {
        /// PCAP file to parse
        #[arg(value_name = "FILE", required = true)]
        file: String,

        /// Filter by message ID
        #[arg(short = 'i', long)]
        id: Option<u32>,

        /// Filter by message type (substring match)
        #[arg(short = 't', long)]
        filter_type: Option<String>,

        /// Filter by opcode (hex like 0xF7B1 or decimal like 63409)
        #[arg(short = 'c', long)]
        filter_opcode: Option<String>,

        /// Filter by direction (Send/Recv)
        #[arg(short = 'd', long)]
        direction: Option<DirectionFilter>,

        /// Sort by field
        #[arg(short, long, default_value = "id")]
        sort: SortField,

        /// Reverse sort order
        #[arg(short, long)]
        reverse: bool,

        /// Limit number of results
        #[arg(short, long)]
        limit: Option<usize>,

        /// Output format (json or jsonl)
        #[arg(short, long, default_value = "jsonl")]
        output: OutputFormat,

        /// Show summary statistics
        #[arg(long)]
        summary: bool,

        /// Print raw message data as hex instead of parsed content
        #[arg(long)]
        raw: bool,
    },

    /// Launch interactive TUI
    Tui {
        /// PCAP file to parse
        #[arg(value_name = "FILE", required = true)]
        file: String,
    },
}

#[cfg(feature = "tracing")]
fn setup_tracing() {
    use tracing_subscriber::{EnvFilter, fmt, prelude::*};

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_span_events(fmt::format::FmtSpan::ENTER | fmt::format::FmtSpan::CLOSE),
        )
        .with(EnvFilter::from_default_env().add_directive("acprotocol=trace".parse().unwrap()))
        .init();
}

fn main() -> Result<()> {
    #[cfg(feature = "tracing")]
    setup_tracing();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Print {
            file,
            id,
            filter_type,
            filter_opcode,
            direction,
            sort,
            reverse,
            limit,
            output,
            summary,
            raw,
        }) => {
            let file_path = file;

            // Load PCAP file and parse packets
            let mut assembler = FragmentAssembler::new();
            let mut messages = Vec::new();

            let pcap_iter = pcap::open(&file_path)?;
            for packet_result in pcap_iter {
                let packet = packet_result?;
                let parsed_messages = assembler.parse_packet_payload(&packet.data)?;
                messages.extend(parsed_messages);
            }

            if summary {
                print_summary(&messages);
            } else if id.is_none()
                && filter_type.is_none()
                && filter_opcode.is_none()
                && direction.is_none()
                && limit.is_none()
            {
                // If no filters are applied, print all messages (like the original cat command)
                if raw {
                    format_raw_messages(&messages, output);
                } else {
                    format_parsed_messages(&messages, output);
                }
            } else {
                // If any filters are applied, use the filtering logic
                output_messages(
                    &messages,
                    id,
                    filter_type.as_deref(),
                    filter_opcode.as_deref(),
                    direction,
                    sort,
                    reverse,
                    limit,
                    output,
                    raw,
                );
            }
        }
        Some(Commands::Tui { file }) => {
            // Launch the TUI
            let file_path = file;

            let path = std::path::Path::new(&file_path);
            tui::run(path)?;
        }
        None => {}
    }

    Ok(())
}
