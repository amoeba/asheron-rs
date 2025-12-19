use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::collections::HashMap;

use acprotocol::cli::tui;
use acprotocol::cli_helper::parse_opcode_filter;
use acprotocol::network::pcap;
use acprotocol::network::{FragmentAssembler, ParsedMessage};

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
    },

    /// Launch interactive TUI
    Tui {
        /// PCAP file to parse
        #[arg(value_name = "FILE", required = true)]
        file: String,
    },
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

fn print_summary(messages: &[ParsedMessage]) {
    println!("=== PCAP Summary ===\n");

    println!("Messages: {}", messages.len());

    let send_msgs = messages.iter().filter(|m| m.direction == "Send").count();
    let recv_msgs = messages.iter().filter(|m| m.direction == "Recv").count();
    println!("\nMessages by Direction:");
    println!("  Send (C→S): {send_msgs}");
    println!("  Recv (S→C): {recv_msgs}");

    let mut type_counts: HashMap<&str, usize> = HashMap::new();
    for msg in messages {
        *type_counts.entry(&msg.message_type).or_insert(0) += 1;
    }

    let mut sorted_types: Vec<_> = type_counts.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));

    println!("\nMessage Types (top 20):");
    for (t, count) in sorted_types.iter().take(20) {
        println!("  {t:40} {count:>5}");
    }

    if sorted_types.len() > 20 {
        println!("  ... and {} more types", sorted_types.len() - 20);
    }
}

#[allow(clippy::too_many_arguments)]
fn output_messages(
    messages: &[ParsedMessage],
    filter_type: Option<&str>,
    filter_opcode: Option<&str>,
    direction: Option<DirectionFilter>,
    sort: SortField,
    reverse: bool,
    limit: Option<usize>,
    output: OutputFormat,
) {
    // Parse opcode filter if provided
    let opcode_filter: Option<u32> = filter_opcode.and_then(|s| parse_opcode_filter(s).ok());

    let mut filtered: Vec<&ParsedMessage> = messages
        .iter()
        .filter(|m| {
            if let Some(ft) = filter_type
                && !m.message_type.to_lowercase().contains(&ft.to_lowercase())
            {
                return false;
            }
            if let Some(oc) = opcode_filter
                && m.opcode != oc
            {
                return false;
            }
            if let Some(d) = direction {
                match d {
                    DirectionFilter::Send => {
                        if m.direction != "Send" {
                            return false;
                        }
                    }
                    DirectionFilter::Recv => {
                        if m.direction != "Recv" {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .collect();

    filtered.sort_by(|a, b| {
        let cmp = match sort {
            SortField::Id => a.id.cmp(&b.id),
            SortField::Type => a.message_type.cmp(&b.message_type),
            SortField::Direction => a.direction.cmp(&b.direction),
        };
        if reverse { cmp.reverse() } else { cmp }
    });

    if let Some(lim) = limit {
        filtered.truncate(lim);
    }

    match output {
        OutputFormat::Jsonl => {
            for msg in filtered {
                println!("{}", serde_json::to_string(&msg).unwrap());
            }
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&filtered).unwrap());
        }
        OutputFormat::Table => {
            println!("{:>6}  {:40}  {:>6}  {:>10}", "ID", "Type", "Dir", "OpCode");
            println!("{}", "-".repeat(70));
            for msg in filtered {
                println!(
                    "{:>6}  {:40}  {:>6}  {:#06x}",
                    msg.id,
                    truncate(&msg.message_type, 40),
                    msg.direction,
                    msg.opcode
                );
            }
        }
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Print {
            file,
            filter_type,
            filter_opcode,
            direction,
            sort,
            reverse,
            limit,
            output,
            summary,
        }) => {
            let file_path = file;
            eprintln!("Parsing PCAP file: {}", file_path);

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
            } else if filter_type.is_none()
                && filter_opcode.is_none()
                && direction.is_none()
                && limit.is_none()
            {
                // If no filters are applied, print all messages (like the original cat command)
                match output {
                    OutputFormat::Jsonl => {
                        for msg in &messages {
                            println!("{}", serde_json::to_string(&msg).unwrap());
                        }
                    }
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&messages).unwrap());
                    }
                    OutputFormat::Table => {
                        println!("{:>6}  {:40}  {:>6}  {:>10}", "ID", "Type", "Dir", "OpCode");
                        println!("{}", "-".repeat(70));
                        for msg in &messages {
                            println!(
                                "{:>6}  {:40}  {:>6}  {:#06x}",
                                msg.id,
                                truncate(&msg.message_type, 40),
                                msg.direction,
                                msg.opcode
                            );
                        }
                    }
                }
            } else {
                // If any filters are applied, use the filtering logic
                output_messages(
                    &messages,
                    filter_type.as_deref(),
                    filter_opcode.as_deref(),
                    direction,
                    sort,
                    reverse,
                    limit,
                    output,
                );
            }
        }
        Some(Commands::Tui { file }) => {
            // Launch the TUI
            let file_path = file;
            eprintln!("Parsing PCAP file: {}", file_path);

            let path = std::path::Path::new(&file_path);
            tui::run(path)?;
        }
        None => {
            eprintln!("Use --help for available commands");
        }
    }

    Ok(())
}
