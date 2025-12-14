Cargo check passes successfully. The warnings fall into these categories:

Unused imports in generated files (~1,300 warnings) - These are in the generated code that shouldn't be manually edited. They come from the codegen tool.
Unreachable pattern matches (4 warnings) - In crates/acprotocol/src/generated/unified.rs, the catch-all other => patterns are unreachable because all cases are explicitly matched.
Dead code warnings - Structs/methods that exist but aren't currently used (C2SPacket, S2CPacket, parse_fragment, truncate_json, etc.)
Unused variable - mut reader in unified_message.rs:35 doesn't need to be mutable

Plan to Fix

Fix the codegen template to not generate unused imports (most impactful - removes 1,300 warnings)
Remove unreachable patterns in unified.rs - replace the catch-all with a proper error or remove it
Remove unused variable mut in unified_message.rs
Dead code warnings - Can be cleaned up but are less critical
