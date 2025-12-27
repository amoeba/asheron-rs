use acprotocol::gameactions;
use acprotocol::message::{C2SMessage, GameActionMessage, S2CMessage};
use acprotocol::messages::s2c;
use acprotocol::types;
use acprotocol::writers::ACWritable;
use std::io::Cursor;

#[test]
fn test_s2c_message_ddd_begin_write_roundtrip() {
    // Test 1: Simple S2CMessage with DDDBeginDDDMessage (no inner opcodes)
    // This demonstrates writing a basic S2C message with the new ACWritable impl

    let ddd_message = s2c::DDDBeginDDDMessage {
        data_expected: 42u32,
        revisions: types::PackableList {
            count: 0,
            list: vec![],
        },
    };

    let msg = S2CMessage::DDDBeginDDDMessage(ddd_message);

    // Write to buffer
    let mut buffer = Vec::new();
    {
        let mut cursor = Cursor::new(&mut buffer);
        msg.write(&mut cursor).expect("Failed to write S2CMessage");
    }

    // Verify the structure:
    // The ACWritable impl should have written:
    // - Bytes 0-3: S2CMessage opcode (DDDBeginDDDMessage = 0xF7E7)
    // - Bytes 4+: The DDDBeginDDDMessage data (data_expected + revisions)

    assert!(
        buffer.len() >= 8,
        "Buffer should have at least 8 bytes (opcode + data_expected)"
    );

    let opcode = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
    assert_eq!(
        opcode, 0xF7E7,
        "First 4 bytes should be DDDBeginDDDMessage opcode"
    );

    let data_expected = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
    assert_eq!(
        data_expected, 42,
        "data_expected should be 42 in the message body"
    );

    // Verify round-trip: read it back and verify
    let mut cursor = Cursor::new(&buffer[..]);
    let read_msg = S2CMessage::read(&mut cursor).expect("Failed to read S2CMessage");

    match read_msg {
        S2CMessage::DDDBeginDDDMessage(ddd) => {
            assert_eq!(ddd.data_expected, 42);
            assert_eq!(ddd.revisions.count, 0);
        }
        _ => panic!("Expected DDDBeginDDDMessage"),
    }
}

#[test]
fn test_c2s_message_ordered_game_action_write_roundtrip() {
    // Test 2: More complex C2SMessage with OrderedGameAction containing a GameAction
    // This demonstrates writing nested messages with multiple opcode levels:
    // - C2SMessage opcode (OrderedGameAction)
    // - sequence number
    // - GameAction opcode (CharacterLoginCompleteNotification)
    // - GameAction payload (empty in this case)

    let action = gameactions::CharacterLoginCompleteNotification {};
    let game_action = GameActionMessage::CharacterLoginCompleteNotification(action);

    let msg = C2SMessage::OrderedGameAction {
        sequence: 123u32,
        action: game_action,
    };

    // Write to buffer
    let mut buffer = Vec::new();
    {
        let mut cursor = Cursor::new(&mut buffer);
        msg.write(&mut cursor).expect("Failed to write C2SMessage");
    }

    // Verify the binary structure:
    // The ACWritable impl should have written:
    // - Bytes 0-3: C2SMessage opcode (OrderedGameAction)
    // - Bytes 4-7: sequence (123)
    // - Bytes 8-11: GameAction opcode (CharacterLoginCompleteNotification = 0xA1)
    // - Bytes 12+: GameAction payload (empty)

    assert!(buffer.len() >= 12, "Buffer should have at least 12 bytes");

    let c2s_opcode = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
    let sequence = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
    let game_action_opcode = u32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);

    // OrderedGameAction = 0xF7B1 in the C2SMessage enum
    assert_eq!(
        c2s_opcode, 0xF7B1,
        "Bytes 0-3 should be C2SMessage::OrderedGameAction opcode (0xF7B1)"
    );
    assert_eq!(
        sequence, 123,
        "Bytes 4-7 should be the sequence number (123)"
    );
    assert_eq!(
        game_action_opcode, 0xA1,
        "Bytes 8-11 should be GameAction::CharacterLoginCompleteNotification (0xA1)"
    );

    // Verify round-trip: read it back and verify structure
    let mut cursor = Cursor::new(&buffer[..]);
    let read_msg = C2SMessage::read(&mut cursor).expect("Failed to read C2SMessage");

    match read_msg {
        C2SMessage::OrderedGameAction {
            sequence: seq,
            action: act,
        } => {
            assert_eq!(seq, 123, "Sequence should match");
            match act {
                GameActionMessage::CharacterLoginCompleteNotification(_) => {
                    // Success! The nested structure was correctly written and read
                }
                _ => panic!("Expected CharacterLoginCompleteNotification action"),
            }
        }
        _ => panic!("Expected OrderedGameAction"),
    }
}
