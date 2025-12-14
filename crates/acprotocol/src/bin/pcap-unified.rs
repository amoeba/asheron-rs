use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "pcap-unified")]
#[command(about = "Parse .pcap network capture files using unified message types", long_about = None)]
struct Cli {
    /// Path to .pcap file
    path: String,
}

fn process_pcap(path: &str) -> Result<()> {
    use acprotocol::network::{FragmentAssembler, UnifiedMessage, pcap};
    
    

    let mut pcap_iter = pcap::open(path)?;
    let mut assembler = FragmentAssembler::new();

    while let Some(packet_result) = pcap_iter.next() {
        let packet = packet_result?;

        // Skip first 42 bytes (Ethernet + IP + UDP headers)
        if packet.data.len() <= 42 {
            continue;
        }
        let udp_payload = &packet.data[42..];

        // Try to parse messages from this packet
        match assembler.parse_packet_payload(udp_payload) {
            Ok(messages) => {
                for msg in messages {
                    // Convert ParsedMessage to UnifiedMessage
                    match UnifiedMessage::from_fragment(msg.data.clone(), msg.sequence, msg.id) {
                        Ok(unified_msg) => {
                            println!("{}", serde_json::to_string(&unified_msg)?);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse message {}: {}", msg.id, e);
                        }
                    }
                }
            }
            Err(_) => {
                // Silently skip packets that can't be parsed
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    process_pcap(&cli.path)
}
