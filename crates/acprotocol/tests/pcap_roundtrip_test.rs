use acprotocol::network::pcap::PcapIterator;
use acprotocol::network::{Fragment, FragmentHeader};
use acprotocol::readers::ACDataType;
use acprotocol::writers::ACWritable;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::Path;

/// Test that we can read fragments from a pcap file and write them back
/// with byte-for-byte identical output
#[test]
fn test_fragment_roundtrip_from_pcap() {
    // Use the smallest pcap file for faster testing
    // Path is relative to the workspace root
    let pcap_path = Path::new("../../data/pcaps/pkt_2025-3-7_1741309870_log.pcap");

    if !pcap_path.exists() {
        eprintln!(
            "Warning: Test pcap file not found at {:?}, skipping test",
            pcap_path
        );
        return;
    }

    let file = File::open(pcap_path).expect("Failed to open pcap file");
    let reader = BufReader::new(file);
    let pcap_iter = PcapIterator::new(reader).expect("Failed to create pcap iterator");

    let mut fragments_tested = 0;
    let mut packets_tested = 0;
    let mut total_bytes_matched = 0;

    for packet_result in pcap_iter {
        let packet = packet_result.expect("Failed to read packet");
        packets_tested += 1;

        // Skip ethernet (14 bytes), IP (20 bytes), and UDP (8 bytes) headers
        // AC protocol data starts at byte 42
        if packet.data.len() < 42 {
            continue;
        }

        let ac_data = &packet.data[42..];

        // Try to parse as a fragment
        let mut cursor = Cursor::new(ac_data);

        match Fragment::read(&mut cursor) {
            Ok(fragment) => {
                fragments_tested += 1;

                // Now write the fragment back
                let mut write_buffer = Vec::new();
                let mut write_cursor = Cursor::new(&mut write_buffer);

                fragment
                    .write(&mut write_cursor)
                    .expect("Failed to write fragment");

                // Compare byte-for-byte
                let bytes_written = write_buffer.len();
                let bytes_read = ac_data.len();

                // The written data should match the original AC data
                if write_buffer[..] != ac_data[..bytes_written] {
                    panic!(
                        "Fragment roundtrip mismatch!\n\
                         Packet #{}, Fragment #{}\n\
                         Original bytes: {} bytes\n\
                         Written bytes:  {} bytes\n\
                         First 32 bytes of original: {:02x?}\n\
                         First 32 bytes of written:  {:02x?}",
                        packets_tested,
                        fragments_tested,
                        bytes_read,
                        bytes_written,
                        &ac_data[..32.min(bytes_read)],
                        &write_buffer[..32.min(bytes_written)]
                    );
                }

                total_bytes_matched += bytes_written;

                // Additional validation: parse the header separately
                let mut header_cursor = Cursor::new(ac_data);
                let header = FragmentHeader::read(&mut header_cursor)
                    .expect("Failed to read fragment header");

                // Write header back
                let mut header_buffer = Vec::new();
                let mut header_write_cursor = Cursor::new(&mut header_buffer);
                header
                    .write(&mut header_write_cursor)
                    .expect("Failed to write fragment header");

                // Verify header bytes match
                let header_size = header_buffer.len();
                assert_eq!(
                    &header_buffer[..],
                    &ac_data[..header_size],
                    "Fragment header roundtrip failed for fragment #{}",
                    fragments_tested
                );
            }
            Err(e) => {
                // Some packets might not be valid fragments (could be connection handshake, etc.)
                eprintln!(
                    "Note: Couldn't parse packet #{} as fragment: {}",
                    packets_tested, e
                );
            }
        }

        // Test at least a few fragments
        if fragments_tested >= 10 {
            break;
        }
    }

    println!("\n✅ Fragment Roundtrip Test Results:");
    println!("   Packets tested:         {}", packets_tested);
    println!("   Fragments parsed:       {}", fragments_tested);
    println!("   Total bytes matched:    {}", total_bytes_matched);

    // Make sure we actually tested some fragments
    assert!(
        fragments_tested > 0,
        "Expected to parse at least one fragment from pcap file"
    );
}

/// Test roundtrip for various primitive types used in fragments
#[test]
fn test_primitive_roundtrip_comprehensive() {
    use acprotocol::readers::*;
    use acprotocol::writers::*;

    // Test u32 (used in fragment headers)
    {
        let mut buffer = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            write_u32(&mut cursor, 0x12345678).unwrap();
        }

        let mut read_cursor = Cursor::new(&buffer[..]);
        let value = read_u32(&mut read_cursor).unwrap();
        assert_eq!(value, 0x12345678);
        assert_eq!(buffer, vec![0x78, 0x56, 0x34, 0x12]); // Little-endian
    }

    // Test u16 (used in fragment headers)
    {
        let mut buffer = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            write_u16(&mut cursor, 0xABCD).unwrap();
        }

        let mut read_cursor = Cursor::new(&buffer[..]);
        let value = read_u16(&mut read_cursor).unwrap();
        assert_eq!(value, 0xABCD);
        assert_eq!(buffer, vec![0xCD, 0xAB]); // Little-endian
    }

    println!("✅ Primitive roundtrip tests passed");
}

/// Test that empty fragments work correctly
#[test]
fn test_empty_fragment_roundtrip() {
    let fragment = Fragment {
        header: FragmentHeader {
            sequence: 1,
            id: 0x80000001,
            count: 1,
            index: 0,
        },
        data: vec![],
    };

    let mut buffer = Vec::new();
    let mut write_cursor = Cursor::new(&mut buffer);
    fragment
        .write(&mut write_cursor)
        .expect("Failed to write empty fragment");

    let mut read_cursor = Cursor::new(&buffer[..]);
    let read_fragment = Fragment::read(&mut read_cursor).expect("Failed to read empty fragment");

    assert_eq!(fragment, read_fragment);
    println!("✅ Empty fragment roundtrip test passed");
}

/// Test fragment with various data sizes
#[test]
fn test_fragment_various_sizes() {
    for size in [0, 1, 16, 128, 256, 512, 1024, 1400] {
        let data = vec![0xAB; size];
        let fragment = Fragment {
            header: FragmentHeader {
                sequence: 1,
                id: 0x80000001,
                count: 1,
                index: 0,
            },
            data,
        };

        let mut buffer = Vec::new();
        let mut write_cursor = Cursor::new(&mut buffer);
        fragment
            .write(&mut write_cursor)
            .unwrap_or_else(|_| panic!("Failed to write fragment of size {}", size));

        let mut read_cursor = Cursor::new(&buffer[..]);
        let read_fragment = Fragment::read(&mut read_cursor)
            .unwrap_or_else(|_| panic!("Failed to read fragment of size {}", size));

        assert_eq!(fragment, read_fragment, "Mismatch for size {}", size);
    }

    println!("✅ Fragment size variation tests passed");
}
