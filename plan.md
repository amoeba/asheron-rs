# PCAP Viewer TUI Code Review & Improvement Plan

## Issues & Fixes

| Issue | Priority | Location | Fix | Status |
|-------|----------|----------|-----|--------|
| Hardcoded `PacketHeaderFlags` | High | lines 693-714 in pcap.rs | Codegen from protocol.xml | ✅ DONE |
| Duplicate tree state reset | Medium | lines 219, 228, 238, 247 | Extract `reset_tree_state()` method | ✅ DONE |
| Detail lines computed twice | Medium | lines 321-334 vs 534-558 | Compute once, store in `App` | |
| Magic numbers (42, 2, 15, etc.) | Medium | throughout | Named constants or calculate from layout | ✅ DONE |
| Duplicate styling logic | Medium | lines 484-490, 564-576, 592-596 | Extract `get_style()` helper | ✅ DONE |
| Unused `extra_info` field | Low | line 101 in `PacketInfo` | Remove or use | ✅ DONE |
| `TreeNode` generic tree | Low | lines 110-178 | Consider own module or dependency | ✅ DONE |
| # column too narrow | Low | line 499 | Constraint::Length(4) | ✅ DONE |
| Click-to-sort columns | Feature | pcap.rs | Implement mouse click sorting | ✅ DONE |

## Code Duplication Details

### 1. Flag Formatting Constants ✅ COMPLETED
The flag constants were hardcoded in `format_packet_flags()` and duplicated `PacketHeaderFlags` enum from protocol.xml.
- **Source**: `ACProtocol/protocol.xml` lines 4-28
- **Solution**: 
  - `PacketHeaderFlags` already generated as bitflags in `crates/acprotocol/src/generated/enums/mod.rs`
  - Created `crates/acprotocol/src/packet_flags.rs` with `format_packet_flags()` helper that uses the generated enum
  - Updated TUI to import and use the generated `PacketHeaderFlags` enum instead of hardcoded constants
  - Removed 95 lines of duplicate flag definitions from pcap.rs

### 2. Scroll Reset Logic ✅ COMPLETED
Every navigation function was resetting tree state identically.
- **Locations**: `next()`, `prev()`, `page_down()`, `page_up()` methods (4 occurrences)
- **Solution**: 
  - Added `App::reset_tree_state()` method that encapsulates the reset logic
  - Replaced 8 lines (2 per location × 4 locations) with single method calls
  - Improves maintainability: changing tree reset behavior now requires one edit

### 3. Magic Numbers ✅ COMPLETED
Protocol constants were hardcoded throughout the codebase.
- **Solution**:
  - Created `acprotocol/src/constants.rs` for protocol-level constants
  - Moved `PACKET_HEADER_SIZE` (42) and `UDP_DEST_PORT_OFFSET` (36) to constants module
  - Kept `BORDER_HEIGHT` (2) in pcap.rs as it's UI-specific
  - Updated all 6 usages to reference named constants
  - Improved code self-documentation and maintainability

### 4. Duplicate Styling Logic ✅ COMPLETED
The focus/dim style pattern was repeated 2 times in the UI rendering.
- **Locations**: lines 493-499, 576-582 (table rows and detail lines)
- **Solution**:
   - Created `get_focused_style(is_focused: bool, pane_focused: bool) -> Style` helper function
   - Replaced 12 lines of duplicate conditional logic with 2 calls to helper
   - Cleaner, more maintainable style application logic

### 5. Click-to-Sort Columns ✅ COMPLETED
- **Added**: Mouse event handling for clickable column headers
- **Features**:
   - Click any column header to sort by that column
   - Click same column again to toggle sort direction (ascending/descending)
   - Sort indicators (▲ for ascending, ▼ for descending) shown in header
   - Default: sorted by Id ascending on load
   - Packets list is cleared and reset to top after sort

### 3. Detail Line Computation
Computed twice in the same render loop:
- **First**: lines 321-334 for key handling
- **Second**: lines 534-558 for rendering
- **Action**: Compute once per frame, cache in `App` state

### 4. Focus Styling Pattern
Repeated 3+ times with identical logic:
```rust
if focused {
    Style::default().bg(Color::Black).fg(Color::White)
} else if !has_focus {
    Style::default().add_modifier(Modifier::DIM)
} else {
    Style::default()
}
```
- **Locations**: lines 484-490, 564-576, 592-596
- **Action**: Extract `get_focused_style(is_focused: bool, is_pane_focused: bool) -> Style`

## Magic Numbers to Extract ✅ COMPLETED

| Value | Context | Name | Location | Status |
|-------|---------|------|----------|--------|
| `42` | Ethernet + IP + UDP header size | `PACKET_HEADER_SIZE` | `acprotocol/src/constants.rs` | ✅ |
| `2` | Border height | `BORDER_HEIGHT` | `pcap.rs` (UI-specific) | ✅ |
| `36`, `37` | UDP port offset in packet | `UDP_DEST_PORT_OFFSET` | `acprotocol/src/constants.rs` | ✅ |
| `15` | Visible rows | Calculated from layout | Already done | ✅ |

## Codegen Candidates

### 1. PacketHeaderFlags Enum
- **Source**: `ACProtocol/protocol.xml` (lines 4-28)
- **Type**: Enum mask
- **Generated Output**: 
  - Rust enum with all flag variants
  - `format_packet_flags(flags: u32) -> String` function
  - Mapping table of values to display names

### 2. GameMessageGroup (future)
- **Source**: `ACProtocol/protocol.xml` (lines 50-63)
- **Type**: Enum
- **Status**: Currently hardcoded in JSON extraction, consider for codegen

## Implementation Order

1. **High Priority**: Generate `PacketHeaderFlags` from codegen
2. **Medium Priority**: Extract helper methods (reset_tree_state, get_style)
3. **Medium Priority**: Cache detail_lines in App state
4. **Medium Priority**: Define named constants for magic numbers
5. **Low Priority**: Remove unused `PacketInfo.extra_info` field
6. **Low Priority**: Consider `TreeNode` module extraction
