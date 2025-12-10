use crate::types::{ProtocolType, ProtocolEnum};
use crate::identifiers::to_snake_case;

pub fn generate_message_parser(
    c2s_types: &[ProtocolType],
    s2c_types: &[ProtocolType],
    enums: &[ProtocolEnum],
) -> String {
    let mut out = String::new();

    // Add imports
    out.push_str(&format!("// Generated message parser with {} C2S and {} S2C types\n", c2s_types.len(), s2c_types.len()));
    out.push_str("use serde_json::json;\n");
    out.push_str("use std::io::Cursor;\n");
    out.push_str("use crate::generated::enums::{C2SMessage, S2CMessage, GameEvent, GameAction};\n");
    out.push_str("use crate::readers::ACReader;\n\n");

    // Generate parse_message_to_json function
    out.push_str("/// Try to parse message data based on opcode and return as JSON\n");
    out.push_str("pub fn parse_message_to_json(\n");
    out.push_str("    opcode: u32,\n");
    out.push_str("    data: &[u8],\n");
    out.push_str(") -> Result<serde_json::Value, Box<dyn std::error::Error>> {\n");
    out.push_str("    // Skip opcode (first 4 bytes)\n");
    out.push_str("    if data.len() < 4 {\n");
    out.push_str("        return Err(\"Data too short\".into());\n");
    out.push_str("    }\n\n");
    out.push_str("    let payload = &data[4..];\n");
    out.push_str("    let mut cursor = Cursor::new(payload);\n\n");
    out.push_str("    // Try C2S messages first\n");
    out.push_str("    if let Ok(msg_type) = C2SMessage::try_from(opcode) {\n");
    out.push_str("        return parse_c2s_message(msg_type, &mut cursor);\n");
    out.push_str("    }\n\n");
    out.push_str("    // Try S2C messages\n");
    out.push_str("    if let Ok(msg_type) = S2CMessage::try_from(opcode) {\n");
    out.push_str("        return parse_s2c_message(msg_type, &mut cursor);\n");
    out.push_str("    }\n\n");
    out.push_str("    // Unknown opcode, return as hex\n");
    out.push_str("    let hex_string: String = data.iter().map(|b| format!(\"{b:02x}\")).collect();\n");
    out.push_str("    Ok(json!({ \"hex\": hex_string }))\n");
    out.push_str("}\n\n");

    // Generate parse_c2s_message function
    out.push_str("fn parse_c2s_message(\n");
    out.push_str("    msg_type: C2SMessage,\n");
    out.push_str("    cursor: &mut Cursor<&[u8]>,\n");
    out.push_str(") -> Result<serde_json::Value, Box<dyn std::error::Error>> {\n");
    out.push_str("    // Special handling for OrderedGameAction\n");
    out.push_str("    if msg_type == C2SMessage::OrderedGameAction {\n");
    out.push_str("        return parse_game_action(cursor);\n");
    out.push_str("    }\n\n");
    out.push_str("    match msg_type {\n");

    // Generate match arms for each C2S message type
    for protocol_type in c2s_types {
         if !protocol_type.is_primitive {
             let type_name = &protocol_type.name;
             // Skip OrderedGameAction as we handle it above
             if type_name != "Ordered_GameAction" {
                 // Remove underscores for enum variant and struct names
                 let type_name_no_underscore = type_name.replace('_', "");
                 out.push_str(&format!("        C2SMessage::{} => {{\n", type_name_no_underscore));
                 out.push_str(&format!("            use crate::messages::c2s::{};\n", type_name_no_underscore));
                 out.push_str(&format!("            serde_json::to_value({}::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)\n", type_name_no_underscore));
                 out.push_str("        }\n");
             }
         }
     }

    out.push_str("        _ => {\n");
    out.push_str("            // Fallback to basic info for unimplemented message types\n");
    out.push_str("            Ok(json!({\"opcode\": msg_type as u32, \"type\": \"C2S\"}))\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Generate parse_s2c_message function
    out.push_str("fn parse_s2c_message(\n");
    out.push_str("    msg_type: S2CMessage,\n");
    out.push_str("    cursor: &mut Cursor<&[u8]>,\n");
    out.push_str(") -> Result<serde_json::Value, Box<dyn std::error::Error>> {\n");
    out.push_str("    // Special handling for OrderedGameEvent\n");
    out.push_str("    if msg_type == S2CMessage::OrderedGameEvent {\n");
    out.push_str("        return parse_game_event(cursor);\n");
    out.push_str("    }\n\n");
    out.push_str("    match msg_type {\n");

    // Generate match arms for each S2C message type
    for protocol_type in s2c_types {
        if !protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            // Skip OrderedGameEvent as we handle it above
            if type_name != "Ordered_GameEvent" {
                // Remove underscores for enum variant and struct names
                let type_name_no_underscore = type_name.replace('_', "");
                out.push_str(&format!("        S2CMessage::{} => {{\n", type_name_no_underscore));
                out.push_str(&format!("            use crate::messages::s2c::{};\n", type_name_no_underscore));
                out.push_str(&format!("            serde_json::to_value({}::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)\n", type_name_no_underscore));
                out.push_str("        }\n");
            }
        }
    }

    out.push_str("        _ => {\n");
    out.push_str("            // Fallback to basic info for unimplemented message types\n");
    out.push_str("            Ok(json!({\"opcode\": msg_type as u32, \"type\": \"S2C\"}))\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Generate parse_game_event function
    out.push_str("fn parse_game_event(\n");
    out.push_str("    cursor: &mut Cursor<&[u8]>,\n");
    out.push_str(") -> Result<serde_json::Value, Box<dyn std::error::Error>> {\n");
    out.push_str("    let mut reader: Box<dyn ACReader> = Box::new(cursor.clone());\n");
    out.push_str("    let object_id = crate::readers::read_u32(&mut reader)?;\n");
    out.push_str("    let sequence = crate::readers::read_u32(&mut reader)?;\n");
    out.push_str("    let event_type_value = crate::readers::read_u32(&mut reader)?;\n");
    out.push_str("    let event_type = match GameEvent::try_from(event_type_value) {\n");
    out.push_str("        Ok(et) => et,\n");
    out.push_str("        Err(_) => return Ok(json!({\n");
    out.push_str("            \"objectId\": object_id,\n");
    out.push_str("            \"sequence\": sequence,\n");
    out.push_str("            \"eventType\": event_type_value,\n");
    out.push_str("            \"error\": \"Unknown event type\"\n");
    out.push_str("        })),\n");
    out.push_str("    };\n\n");
    out.push_str("    // Reset cursor position and parse the event\n");
    out.push_str("    cursor.set_position(0);\n");
    out.push_str("    let mut reader = cursor.clone();\n");
    out.push_str("    \n");
    out.push_str("    // Skip the 12-byte header (objectId, sequence, eventType)\n");
    out.push_str("    // The event struct reads only its payload, not the header\n");
    out.push_str("    crate::readers::read_u32(&mut reader)?;  // skip objectId\n");
    out.push_str("    crate::readers::read_u32(&mut reader)?;  // skip sequence\n");
    out.push_str("    crate::readers::read_u32(&mut reader)?;  // skip eventType\n\n");
    out.push_str("    let event_data: Result<serde_json::Value, Box<dyn std::error::Error>> = match event_type {\n");
    generate_game_event_cases(&mut out, enums);
    out.push_str("    };\n\n");
    out.push_str("    let mut result = match event_data {\n");
    out.push_str("        Ok(data) => data,\n");
    out.push_str("        Err(e) => return Ok(json!({\n");
    out.push_str("            \"objectId\": object_id,\n");
    out.push_str("            \"sequence\": sequence,\n");
    out.push_str("            \"eventType\": event_type_value,\n");
    out.push_str("            \"error\": format!(\"Failed to parse: {}\", e)\n");
    out.push_str("        })),\n");
    out.push_str("    };\n\n");
    out.push_str("    // Add header fields to the result\n");
    out.push_str("    if let serde_json::Value::Object(ref mut map) = result {\n");
    out.push_str("        map.insert(\"objectId\".to_string(), serde_json::json!(object_id));\n");
    out.push_str("        map.insert(\"sequence\".to_string(), serde_json::json!(sequence));\n");
    out.push_str("        map.insert(\"eventType\".to_string(), serde_json::json!(event_type_value));\n");
    out.push_str("    }\n\n");
    out.push_str("    Ok(result)\n");
    out.push_str("}\n\n");

    // Generate parse_game_action function
    out.push_str("fn parse_game_action(\n");
    out.push_str("    cursor: &mut Cursor<&[u8]>,\n");
    out.push_str(") -> Result<serde_json::Value, Box<dyn std::error::Error>> {\n");
    out.push_str("    let mut reader: Box<dyn ACReader> = Box::new(cursor.clone());\n");
    out.push_str("    let sequence = crate::readers::read_u32(&mut reader)?;\n");
    out.push_str("    let action_type_value = crate::readers::read_u32(&mut reader)?;\n");
    out.push_str("    let action_type = match GameAction::try_from(action_type_value) {\n");
    out.push_str("        Ok(at) => at,\n");
    out.push_str("        Err(_) => return Ok(json!({\n");
    out.push_str("            \"sequence\": sequence,\n");
    out.push_str("            \"actionType\": action_type_value,\n");
    out.push_str("            \"error\": \"Unknown action type\"\n");
    out.push_str("        })),\n");
    out.push_str("    };\n\n");
    out.push_str("    // Reset cursor position and parse the action\n");
    out.push_str("    cursor.set_position(0);\n");
    out.push_str("    let mut reader = cursor.clone();\n");
    out.push_str("    \n");
    out.push_str("    // Skip the 8-byte header (sequence, actionType)\n");
    out.push_str("    // The action struct reads only its payload, not the header\n");
    out.push_str("    crate::readers::read_u32(&mut reader)?;  // skip sequence\n");
    out.push_str("    crate::readers::read_u32(&mut reader)?;  // skip actionType\n\n");
    out.push_str("    let action_data: Result<serde_json::Value, Box<dyn std::error::Error>> = match action_type {\n");
    generate_game_action_cases(&mut out, enums);
    out.push_str("    };\n\n");
    out.push_str("    let mut result = match action_data {\n");
    out.push_str("        Ok(data) => data,\n");
    out.push_str("        Err(e) => return Ok(json!({\n");
    out.push_str("            \"sequence\": sequence,\n");
    out.push_str("            \"actionType\": action_type_value,\n");
    out.push_str("            \"error\": format!(\"Failed to parse: {}\", e)\n");
    out.push_str("        })),\n");
    out.push_str("    };\n\n");
    out.push_str("    // Add header fields to the result\n");
    out.push_str("    if let serde_json::Value::Object(ref mut map) = result {\n");
    out.push_str("        map.insert(\"sequence\".to_string(), serde_json::json!(sequence));\n");
    out.push_str("        map.insert(\"actionType\".to_string(), serde_json::json!(action_type_value));\n");
    out.push_str("    }\n\n");
    out.push_str("    Ok(result)\n");
    out.push_str("}\n");

    out
}

fn generate_game_event_cases(out: &mut String, enums: &[ProtocolEnum]) {
    // Find the GameEvent enum
    let game_event_enum = enums.iter().find(|e| e.name == "GameEvent");
    
    if let Some(game_event) = game_event_enum {
        // Generate a case for each GameEvent variant
        for variant in &game_event.values {
            let variant_name = variant.name.replace('_', "");
            // Convert to module name (remove underscores, then snake_case)
            let module_name = to_snake_case(&variant_name);
            out.push_str(&format!("        GameEvent::{} => {{\n", variant_name));
            out.push_str(&format!("            use crate::generated::gameevents::{}::{};\n", module_name, variant_name));
            out.push_str(&format!("            serde_json::to_value({}::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)\n", variant_name));
            out.push_str("        }\n");
        }
    }
    
    // Fallback for unknown variants
    out.push_str("        _ => Ok(json!({})),\n");
}

fn generate_game_action_cases(out: &mut String, enums: &[ProtocolEnum]) {
     // Find the GameAction enum
     let game_action_enum = enums.iter().find(|e| e.name == "GameAction");
     
     if let Some(game_action) = game_action_enum {
         // Generate a case for each GameAction variant
         for variant in &game_action.values {
             let variant_name = variant.name.replace('_', "");
             // Convert to module name (remove underscores, then snake_case)
             let module_name = to_snake_case(&variant_name);
             out.push_str(&format!("        GameAction::{} => {{\n", variant_name));
             out.push_str(&format!("            use crate::generated::gameactions::{}::{};\n", module_name, variant_name));
             out.push_str(&format!("            serde_json::to_value({}::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)\n", variant_name));
             out.push_str("        }\n");
         }
     }
     
     // Fallback for unknown variants
     out.push_str("        _ => Ok(json!({})),\n");
}
