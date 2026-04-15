use std::collections::HashMap;

use crate::network::RawMessage;

use super::types::{OutputFormat, RawMessageOutput};

/// Truncate a string to a maximum length, adding "..." if truncated
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Print summary statistics for a collection of messages
pub fn print_summary(messages: &[RawMessage]) {
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

/// Helper function to format and output messages in raw format (with hex data)
pub fn format_raw_messages<'a, I>(messages: I, output: OutputFormat)
where
    I: IntoIterator<Item = &'a RawMessage>,
{
    let messages: Vec<_> = messages.into_iter().collect();

    match output {
        OutputFormat::Jsonl => {
            for msg in messages {
                let raw_output = RawMessageOutput {
                    id: msg.id,
                    opcode: msg.opcode,
                    message_type: msg.message_type.clone(),
                    direction: msg.direction.clone(),
                    queue: msg.queue.as_ref().map(|q| format!("{:?}", q)),
                    data_len: msg.data.len(),
                    raw: hex::encode(&msg.data),
                    sequence: msg.sequence,
                    iteration: msg.iteration,
                    header_flags: msg.header_flags,
                };
                println!("{}", serde_json::to_string(&raw_output).unwrap());
            }
        }
        OutputFormat::Json => {
            let raw_outputs: Vec<_> = messages
                .iter()
                .map(|msg| RawMessageOutput {
                    id: msg.id,
                    opcode: msg.opcode,
                    message_type: msg.message_type.clone(),
                    direction: msg.direction.clone(),
                    queue: msg.queue.as_ref().map(|q| format!("{:?}", q)),
                    data_len: msg.data.len(),
                    raw: hex::encode(&msg.data),
                    sequence: msg.sequence,
                    iteration: msg.iteration,
                    header_flags: msg.header_flags,
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&raw_outputs).unwrap());
        }
        OutputFormat::Table => {
            println!(
                "{:>6}  {:40}  {:>6}  {:>10}  {:>6}  Raw Data",
                "ID", "Type", "Dir", "OpCode", "Len"
            );
            println!("{}", "-".repeat(140));
            for msg in messages {
                let hex_data = hex::encode(&msg.data);
                let truncated_hex = if hex_data.len() > 50 {
                    format!("{}...", &hex_data[..50])
                } else {
                    hex_data
                };
                println!(
                    "{:>6}  {:40}  {:>6}  {:#06x}  {:>6}  {}",
                    msg.id,
                    truncate(&msg.message_type, 40),
                    msg.direction,
                    msg.opcode,
                    msg.data.len(),
                    truncated_hex
                );
            }
        }
    }
}

/// Helper function to format and output messages in parsed format (JSON serialization)
pub fn format_parsed_messages<'a, I>(messages: I, output: OutputFormat)
where
    I: IntoIterator<Item = &'a RawMessage>,
{
    let messages: Vec<_> = messages.into_iter().collect();

    match output {
        OutputFormat::Jsonl => {
            for msg in messages {
                println!("{}", serde_json::to_string(&msg).unwrap());
            }
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&messages).unwrap());
        }
        OutputFormat::Table => {
            println!("{:>6}  {:40}  {:>6}  {:>10}", "ID", "Type", "Dir", "OpCode");
            println!("{}", "-".repeat(70));
            for msg in messages {
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
