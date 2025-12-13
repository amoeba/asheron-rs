use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use acprotocol::constants::{PACKET_HEADER_SIZE, UDP_DEST_PORT_OFFSET};

// Border height in terminal UI (top and bottom borders)
const BORDER_HEIGHT: usize = 2;

mod read {
    use anyhow::Result;
    use std::path::Path;
    use crate::PACKET_HEADER_SIZE;

    pub fn run(path: &Path, message_index: Option<usize>) -> Result<()> {
        use acprotocol::network::{FragmentAssembler, pcap};

        let mut pcap_iter = pcap::open(path.to_str().unwrap())?;
        let mut assembler = FragmentAssembler::new();
        let mut message_count = 0;

        while let Some(packet_result) = pcap_iter.next() {
            let packet = packet_result?;

            // Skip network packet headers (Ethernet + IP + UDP)
            if packet.data.len() <= PACKET_HEADER_SIZE {
                continue;
            }
            let udp_payload = &packet.data[PACKET_HEADER_SIZE..];

            // Try to parse messages from this packet
            match assembler.parse_packet_payload(udp_payload) {
                Ok(messages) => {
                    for msg in messages {
                        if let Some(target_index) = message_index {
                            if message_count == target_index {
                                println!("{}", serde_json::to_string_pretty(&msg)?);
                                return Ok(());
                            }
                        } else {
                            println!("{}", serde_json::to_string(&msg)?);
                        }
                        message_count += 1;
                    }
                }
                Err(_) => {
                    // Silently skip packets that can't be parsed
                }
            }
        }

        if message_index.is_some() {
            eprintln!("Message index out of range. Total messages: {}", message_count);
        }

        Ok(())
    }
}

mod tui {
    use anyhow::Result;
    use std::path::Path;

    use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEvent, MouseEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout},
        prelude::Stylize,
        style::{Color, Modifier, Style},
        text::Span,
        widgets::{Block, Borders, Paragraph, Row, Table},
        Terminal,
    };
    use serde_json::Value;
    use std::io;
    use acprotocol::enums::PacketHeaderFlags;
    use acprotocol::packet_flags::format_packet_flags;
    use acprotocol::tree::{TreeNode, collect_all_expandable_paths};
    use crate::{PACKET_HEADER_SIZE, UDP_DEST_PORT_OFFSET, BORDER_HEIGHT};

    pub fn run(path: &Path) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Load pcap data
        let packets = load_packets(path)?;

        // Create app state
        let mut app = App::new(packets);
        
        // Apply initial sort by Id ascending
        app.apply_sort();

        // Run the TUI
        let res = run_app(&mut terminal, &mut app);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err);
        }

        Ok(())
    }

    struct PacketInfo {
        id: u32,
        direction: String,
        timestamp: String,
        flags: String,
        packet_type: String,
        size: usize,
        opcode: u32,
        sequence: u32,
        queue: String,
        iteration: u32,
        port: u16,
        raw_json: String,
    }



    struct App {
        packets: Vec<PacketInfo>,
        selected: usize,
        scroll_offset: usize,
        tree_expanded: std::collections::HashSet<String>,
        tree_scroll_offset: usize,
        tree_focused_line: usize,
        focused_pane: FocusedPane,
        sort_column: SortColumn,
        sort_ascending: bool,
        selected_column: usize,  // Index of the column header being selected (0-10)
        column_rects: Vec<(u16, u16, SortColumn)>,  // (start, end, column)
        list_pane_right: u16,  // Right boundary of list pane
    }

    enum FocusedPane {
        List,
        Details,
    }

    #[derive(Clone, Copy, PartialEq)]
    enum SortColumn {
        Id,
        Direction,
        Timestamp,
        Flags,
        MessageType,
        Size,
        OpCode,
        Sequence,
        Queue,
        Iteration,
        Port,
    }

    impl App {
        fn new(packets: Vec<PacketInfo>) -> Self {
            App {
                packets,
                selected: 0,
                scroll_offset: 0,
                tree_expanded: std::collections::HashSet::new(),
                tree_scroll_offset: 0,
                tree_focused_line: 0,
                focused_pane: FocusedPane::List,
                sort_column: SortColumn::Id,
                sort_ascending: true,
                selected_column: 0,
                column_rects: Vec::new(),
                list_pane_right: 0,
            }
        }

        fn reset_tree_state(&mut self) {
            self.tree_scroll_offset = 0;
            self.tree_focused_line = 0;
        }

        fn next(&mut self, visible_rows: usize) {
            if !self.packets.is_empty() && self.selected < self.packets.len() - 1 {
                self.selected += 1;
                self.update_scroll(visible_rows);
                self.reset_tree_state();
            }
        }

        fn prev(&mut self, visible_rows: usize) {
            if !self.packets.is_empty() && self.selected > 0 {
                self.selected -= 1;
                self.update_scroll(visible_rows);
                self.reset_tree_state();
            }
        }

        fn page_down(&mut self, visible_rows: usize) {
            if !self.packets.is_empty() {
                self.selected = (self.selected + visible_rows).min(self.packets.len() - 1);
                self.update_scroll(visible_rows);
                self.reset_tree_state();
            }
        }

        fn page_up(&mut self, visible_rows: usize) {
            if !self.packets.is_empty() {
                self.selected = self.selected.saturating_sub(visible_rows);
                self.update_scroll(visible_rows);
                self.reset_tree_state();
            }
        }

        fn update_scroll(&mut self, visible_rows: usize) {
            if visible_rows == 0 {
                return;
            }
            // Keep selected row visible
            if self.selected < self.scroll_offset {
                self.scroll_offset = self.selected;
            } else if self.selected >= self.scroll_offset + visible_rows {
                self.scroll_offset = self.selected.saturating_sub(visible_rows - 1);
            }
        }

        fn toggle_tree_node(&mut self, path: String) {
            if self.tree_expanded.contains(&path) {
                self.tree_expanded.remove(&path);
            } else {
                self.tree_expanded.insert(path);
            }
        }

        fn tree_line_down(&mut self, total_lines: usize, visible_rows: usize) {
            if self.tree_focused_line < total_lines.saturating_sub(1) {
                self.tree_focused_line += 1;
                // Auto-scroll to keep focused line visible
                if self.tree_focused_line >= self.tree_scroll_offset + visible_rows {
                    self.tree_scroll_offset = self.tree_focused_line.saturating_sub(visible_rows - 1);
                }
            }
        }

        fn tree_line_up(&mut self) {
            if self.tree_focused_line > 0 {
                self.tree_focused_line -= 1;
                // Auto-scroll to keep focused line visible
                if self.tree_focused_line < self.tree_scroll_offset {
                    self.tree_scroll_offset = self.tree_focused_line;
                }
            }
        }

        fn apply_sort(&mut self) {
            self.packets.sort_by(|a, b| {
                let cmp = match self.sort_column {
                    SortColumn::Id => a.id.cmp(&b.id),
                    SortColumn::Direction => a.direction.cmp(&b.direction),
                    SortColumn::Timestamp => a.timestamp.cmp(&b.timestamp),
                    SortColumn::Flags => a.flags.cmp(&b.flags),
                    SortColumn::MessageType => a.packet_type.cmp(&b.packet_type),
                    SortColumn::Size => a.size.cmp(&b.size),
                    SortColumn::OpCode => a.opcode.cmp(&b.opcode),
                    SortColumn::Sequence => a.sequence.cmp(&b.sequence),
                    SortColumn::Queue => a.queue.cmp(&b.queue),
                    SortColumn::Iteration => a.iteration.cmp(&b.iteration),
                    SortColumn::Port => a.port.cmp(&b.port),
                };
                if self.sort_ascending { cmp } else { cmp.reverse() }
            });
        }

        fn move_column_left(&mut self) {
            if self.selected_column > 0 {
                self.selected_column -= 1;
            }
        }

        fn move_column_right(&mut self) {
            if self.selected_column < 10 {  // 11 columns total (0-10)
                self.selected_column += 1;
            }
        }

        fn sort_by_selected_column(&mut self) {
            let columns = [
                SortColumn::Id,
                SortColumn::Direction,
                SortColumn::Timestamp,
                SortColumn::Flags,
                SortColumn::MessageType,
                SortColumn::Size,
                SortColumn::OpCode,
                SortColumn::Sequence,
                SortColumn::Queue,
                SortColumn::Iteration,
                SortColumn::Port,
            ];
            
            if self.selected_column < columns.len() {
                let col = columns[self.selected_column];
                if self.sort_column == col {
                    self.sort_ascending = !self.sort_ascending;
                } else {
                    self.sort_column = col;
                    self.sort_ascending = false;
                }
                self.apply_sort();
                self.scroll_offset = 0;
                self.selected = 0;
            }
        }
    }

    fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
        loop {
            // Get detail lines before drawing (for Enter key handling)
            let detail_lines = if !app.packets.is_empty() {
                let packet = &app.packets[app.selected];
                match serde_json::from_str::<Value>(&packet.raw_json) {
                    Ok(json_val) => {
                        let mut expanded = app.tree_expanded.clone();
                        expanded.insert("root".to_string()); // Root is always expanded
                        let root = TreeNode::from_json("root", &json_val);
                        root.get_display_lines(0, &expanded, String::new())
                    }
                    Err(_) => vec![],
                }
            } else {
                vec![]
            };

            terminal.draw(|f| ui(f, app))?;

            if crossterm::event::poll(std::time::Duration::from_millis(250))? {
                match event::read()? {
                    Event::Key(key) => {
                        // Estimate visible rows (roughly 15-20 depending on screen)
                        let visible_rows = 15;
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                            KeyCode::Tab => {
                                // Switch between panes
                                app.focused_pane = match app.focused_pane {
                                    FocusedPane::List => FocusedPane::Details,
                                    FocusedPane::Details => FocusedPane::List,
                                };
                            }
                            KeyCode::Char('j') | KeyCode::Down => {
                                match app.focused_pane {
                                    FocusedPane::List => app.next(visible_rows),
                                    FocusedPane::Details => {
                                        let total_lines = detail_lines.len();
                                        app.tree_line_down(total_lines, visible_rows);
                                    }
                                }
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                match app.focused_pane {
                                    FocusedPane::List => app.prev(visible_rows),
                                    FocusedPane::Details => app.tree_line_up(),
                                }
                            }
                            KeyCode::Left => app.move_column_left(),
                            KeyCode::Right => app.move_column_right(),
                            KeyCode::Char(' ') => app.sort_by_selected_column(),
                            KeyCode::PageDown => app.page_down(visible_rows),
                            KeyCode::PageUp => app.page_up(visible_rows),
                            KeyCode::Home => {
                                app.selected = 0;
                                app.scroll_offset = 0;
                                app.tree_scroll_offset = 0;
                                app.tree_focused_line = 0;
                            }
                            KeyCode::End => {
                                if !app.packets.is_empty() {
                                    app.selected = app.packets.len() - 1;
                                    app.update_scroll(visible_rows);
                                }
                            }
                            KeyCode::Enter => {
                                // Only expand/collapse in details pane
                                if matches!(app.focused_pane, FocusedPane::Details) {
                                    // Enter: toggle current node
                                    let focused_global_line = app.tree_scroll_offset + app.tree_focused_line;
                                    if focused_global_line < detail_lines.len() {
                                        let (line, path) = &detail_lines[focused_global_line];
                                        // Only toggle if line contains expand/collapse markers
                                        if (line.contains("▶") || line.contains("▼")) && !path.is_empty() {
                                            app.toggle_tree_node(path.clone());
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                // Ctrl+f: page down
                                match app.focused_pane {
                                    FocusedPane::List => app.page_down(visible_rows),
                                    FocusedPane::Details => {
                                        for _ in 0..visible_rows {
                                            let total_lines = detail_lines.len();
                                            app.tree_line_down(total_lines, visible_rows);
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                // Ctrl+b: page up
                                match app.focused_pane {
                                    FocusedPane::List => app.page_up(visible_rows),
                                    FocusedPane::Details => {
                                        for _ in 0..visible_rows {
                                            app.tree_line_up();
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                // Ctrl+a: expand all nodes (when Shift+Enter doesn't work)
                                if matches!(app.focused_pane, FocusedPane::Details) {
                                    if !app.packets.is_empty() {
                                        let packet = &app.packets[app.selected];
                                        if let Ok(json_val) = serde_json::from_str::<Value>(&packet.raw_json) {
                                            let root = TreeNode::from_json("root", &json_val);
                                            collect_all_expandable_paths(&root, String::new(), &mut app.tree_expanded);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Event::Mouse(mouse) => {
                        if mouse.kind == MouseEventKind::Up(crossterm::event::MouseButton::Left) {
                            handle_mouse_click(mouse, app);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_mouse_click(mouse: MouseEvent, app: &mut App) {
        // Only handle clicks in the left pane (list)
        if mouse.column >= app.list_pane_right {
            return;  // Click is in the details pane, ignore
        }
        
        // Check if click is in the header row (typically row 1 after title/border)
        if mouse.row == 1 {
            // Header row click - sort by column
            for (start, end, col) in &app.column_rects {
                if mouse.column >= *start && mouse.column < *end {
                    // Toggle sort direction if same column, otherwise switch column
                    if app.sort_column == *col {
                        app.sort_ascending = !app.sort_ascending;
                    } else {
                        app.sort_column = *col;
                        app.sort_ascending = false;
                    }
                    app.apply_sort();
                    app.scroll_offset = 0;
                    app.selected = 0;
                    break;
                }
            }
        } else if mouse.row > 1 {
            // Data row click - select packet
            // Row 2 onwards are data rows (row 0=top border, row 1=header)
            let row_index = (mouse.row - 2) as usize;
            if row_index < app.packets.len() {
                let visible_rows = 15; // Estimate from render
                let selected_in_view = row_index + app.scroll_offset;
                if selected_in_view < app.packets.len() {
                    app.selected = selected_in_view;
                    // Auto-scroll to keep selected visible
                    app.update_scroll(visible_rows);
                }
            }
        }
    }

    fn get_focused_style(is_focused: bool, pane_focused: bool) -> Style {
        if is_focused {
            Style::default().bg(Color::Black).fg(Color::White)
        } else if !pane_focused {
            Style::default().add_modifier(Modifier::DIM)
        } else {
            Style::default()
        }
    }

    fn ui(f: &mut ratatui::Frame, app: &mut App) {
        let outer_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Length(2),
            ])
            .split(f.area());

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(66),
                Constraint::Percentage(34),
            ])
            .split(outer_chunks[0]);
        
        // Store the right boundary of the list pane for mouse click detection
        app.list_pane_right = chunks[0].right();

        // Table header with sort indicators and column selection
        let arrow = if app.sort_ascending { "▲" } else { "▼" };
        let col_names = ["#", "Dir", "Timestamp", "Flags", "Message Type", "Size", "OpCode", "Seq", "Queue", "Iter", "Port"];
        
        let header_cells: Vec<Span> = col_names.iter().enumerate().map(|(idx, name)| {
            let has_sort = match idx {
                0 => app.sort_column == SortColumn::Id,
                1 => app.sort_column == SortColumn::Direction,
                2 => app.sort_column == SortColumn::Timestamp,
                3 => app.sort_column == SortColumn::Flags,
                4 => app.sort_column == SortColumn::MessageType,
                5 => app.sort_column == SortColumn::Size,
                6 => app.sort_column == SortColumn::OpCode,
                7 => app.sort_column == SortColumn::Sequence,
                8 => app.sort_column == SortColumn::Queue,
                9 => app.sort_column == SortColumn::Iteration,
                10 => app.sort_column == SortColumn::Port,
                _ => false,
            };
            
            let text = if has_sort { 
                format!("{}{}", name, arrow) 
            } else { 
                name.to_string() 
            };
            
            // Highlight selected column with black background and white text
            if idx == app.selected_column {
                Span::styled(text, Style::default().bg(Color::Black).fg(Color::White).add_modifier(Modifier::BOLD))
            } else {
                Span::raw(text)
            }
        }).collect();
        
        let header = Row::new(header_cells)
            .style(Style::default().bg(Color::DarkGray).bold())
            .bottom_margin(0);

        let visible_rows = chunks[1].height.saturating_sub(BORDER_HEIGHT as u16) as usize;

        let rows: Vec<Row> = app
            .packets
            .iter()
            .enumerate()
            .skip(app.scroll_offset)
            .take(visible_rows)
            .map(|(i, packet)| {
                let cells = vec![
                    packet.id.to_string(),
                    packet.direction.clone(),
                    packet.timestamp.clone(),
                    packet.flags.clone(),
                    packet.packet_type.clone(),
                    packet.size.to_string(),
                    format!("0x{:04x}", packet.opcode),
                    packet.sequence.to_string(),
                    packet.queue.clone(),
                    packet.iteration.to_string(),
                    packet.port.to_string(),
                ];

                let style = get_focused_style(
                    i == app.selected,
                    matches!(app.focused_pane, FocusedPane::List),
                );
                Row::new(cells).style(style)
            })
            .collect();

        // Calculate column positions for mouse click detection
        // These should match the Constraint values below
        let col_widths = [4, 4, 11, 8, 15, 5, 6, 4, 7, 3, 4];
        let col_headers = [
            SortColumn::Id,
            SortColumn::Direction,
            SortColumn::Timestamp,
            SortColumn::Flags,
            SortColumn::MessageType,
            SortColumn::Size,
            SortColumn::OpCode,
            SortColumn::Sequence,
            SortColumn::Queue,
            SortColumn::Iteration,
            SortColumn::Port,
        ];
        
        app.column_rects.clear();
        let mut col_start = 2; // Account for left border + padding
        for (width, col) in col_widths.iter().zip(col_headers.iter()) {
            let col_end = col_start + width;
            app.column_rects.push((col_start as u16, col_end as u16, *col));
            col_start = col_end + 1; // Add 1 for spacing between columns
        }

        let table = Table::new(
            rows,
            [
                Constraint::Length(4),           // #
                Constraint::Length(4),           // Dir
                Constraint::Length(11),          // Timestamp
                Constraint::Length(8),           // Flags
                Constraint::Min(15),             // Message Type (expandable)
                Constraint::Length(5),           // Size
                Constraint::Length(6),           // OpCode
                Constraint::Length(4),           // Seq
                Constraint::Length(7),           // Queue
                Constraint::Length(3),           // Iter
                Constraint::Length(4),           // Port
            ],
        )
        .header(header)
        .block({
            let style = if matches!(app.focused_pane, FocusedPane::List) {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            Block::default()
                .borders(Borders::ALL)
                .title("Packets")
                .style(style)
        });

        f.render_widget(table, chunks[0]);

        // Details panel
        let detail_title = "Details";

        if !app.packets.is_empty() {
            let packet = &app.packets[app.selected];
            
            // Parse JSON and build tree
            let detail_lines = match serde_json::from_str::<Value>(&packet.raw_json) {
                Ok(mut json_val) => {
                    // Format header_flags if present
                    if let Some(flags_val) = json_val.get("header_flags") {
                        if let Some(flags_num) = flags_val.as_u64() {
                            let formatted_flags = format_packet_flags(PacketHeaderFlags::from_bits_retain(flags_num as u32));
                            json_val["header_flags"] = Value::String(formatted_flags);
                        }
                    }
                    // Format opcode if present
                    if let Some(opcode_val) = json_val.get("opcode") {
                        if let Some(opcode_num) = opcode_val.as_u64() {
                            let formatted_opcode = format!("0x{:04x} ({})", opcode_num, opcode_num);
                            json_val["opcode"] = Value::String(formatted_opcode);
                        }
                    }
                    let mut expanded = app.tree_expanded.clone();
                    expanded.insert("root".to_string()); // Root is always expanded
                    let root = TreeNode::from_json("root", &json_val);
                    root.get_display_lines(0, &expanded, String::new())
                }
                Err(e) => {
                    vec![(format!("Failed to parse JSON: {}\n\nRaw:\n{}", e, &packet.raw_json), String::new())]
                }
            };
            
            let visible_rows = chunks[1].height.saturating_sub(BORDER_HEIGHT as u16) as usize;
            
            // Build styled lines with focused line highlighted
            let mut styled_lines = Vec::new();
            for (i, (line, _path)) in detail_lines.iter().enumerate() {
                if i >= app.tree_scroll_offset && i < app.tree_scroll_offset + visible_rows {
                    let display_idx = i - app.tree_scroll_offset;
                    let style = get_focused_style(
                        display_idx == app.tree_focused_line && matches!(app.focused_pane, FocusedPane::Details),
                        matches!(app.focused_pane, FocusedPane::Details),
                    );
                    styled_lines.push(ratatui::text::Line::from(ratatui::text::Span::styled(line.clone(), style)));
                }
            }
            
            let detail_para = Paragraph::new(styled_lines)
                .block({
                    let style = if matches!(app.focused_pane, FocusedPane::Details) {
                        Style::default().add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    Block::default()
                        .title(detail_title)
                        .borders(Borders::ALL)
                        .style(style)
                });
            f.render_widget(detail_para, chunks[1]);
        } else {
            let text_style = if matches!(app.focused_pane, FocusedPane::Details) {
                Style::default()
            } else {
                Style::default().add_modifier(Modifier::DIM)
            };
            
            let detail_para = Paragraph::new("No packets loaded")
                .block({
                    let style = if matches!(app.focused_pane, FocusedPane::Details) {
                        Style::default().add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    Block::default()
                        .title(detail_title)
                        .borders(Borders::ALL)
                        .style(style)
                })
                .style(text_style);
            f.render_widget(detail_para, chunks[1]);
        }

        // Footer with controls
        let controls_text = "q/Esc: Quit | Tab: Switch pane | ↑/↓: Navigate | Enter: Expand/collapse | Ctrl+a: Expand all";
        let controls = Paragraph::new(controls_text)
            .block(Block::default().borders(Borders::TOP));
        f.render_widget(controls, outer_chunks[1]);
    }

    fn truncate_json(json: &str, max_len: usize) -> String {
        if json.len() > max_len {
            format!("{}...", &json[..max_len])
        } else {
            json.to_string()
        }
    }

    fn load_packets(path: &Path) -> Result<Vec<PacketInfo>> {
        use acprotocol::network::{FragmentAssembler, pcap};

        let mut pcap_iter = pcap::open(path.to_str().unwrap())?;
        let mut assembler = FragmentAssembler::new();
        let mut packets = Vec::new();
        let mut packet_num = 0u32;

        while let Some(packet_result) = pcap_iter.next() {
            let packet = packet_result?;

            // Extract port info from UDP header (before stripping headers)
            let dest_port = if packet.data.len() > UDP_DEST_PORT_OFFSET + 1 {
                u16::from_be_bytes([packet.data[UDP_DEST_PORT_OFFSET], packet.data[UDP_DEST_PORT_OFFSET + 1]])
            } else {
                0
            };

            // Format timestamp
            let timestamp = format_timestamp(packet.ts_sec, packet.ts_usec);

            // Skip network packet headers (Ethernet + IP + UDP)
            if packet.data.len() <= PACKET_HEADER_SIZE {
                continue;
            }
            let udp_payload = &packet.data[PACKET_HEADER_SIZE..];

            // Try to parse messages from this packet
            match assembler.parse_packet_payload(udp_payload) {
                Ok(messages) => {
                    for msg in messages {
                        if let Ok(json_str) = serde_json::to_string(&msg) {
                            if let Ok(json_val) = serde_json::from_str::<Value>(&json_str) {
                                packet_num += 1;
                                let info = extract_packet_info(
                                    packet_num,
                                    &json_val,
                                    &json_str,
                                    &timestamp,
                                    dest_port,
                                );
                                packets.push(info);
                            }
                        }
                    }
                }
                Err(_) => {
                    // Silently skip packets that can't be parsed
                }
            }
        }

        Ok(packets)
    }

    fn format_timestamp(ts_sec: u32, ts_usec: u32) -> String {
        let millis = (ts_usec / 1000) % 1000;
        format!("{}.{:03}", ts_sec, millis)
    }



    fn extract_packet_info(
        id: u32,
        json_val: &Value,
        raw_json: &str,
        timestamp: &str,
        port: u16,
    ) -> PacketInfo {
        let direction = json_val
            .get("direction")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();
        let opcode = json_val
            .get("opcode")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        let sequence = json_val
            .get("sequence")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        let message_type = json_val
            .get("message_type")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();
        let iteration = json_val
            .get("iteration")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        // Calculate approximate size (raw JSON string length)
        let size = raw_json.len();

        // Extract flags from header_flags field if present
        let flags = json_val
            .get("header_flags")
            .and_then(|v| v.as_u64())
            .map(|f| format_packet_flags(PacketHeaderFlags::from_bits_retain(f as u32)))
            .unwrap_or_else(|| "Unknown".to_string());

        // Extract queue from JSON (already populated from codegen)
        let queue = json_val
            .get("queue")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        PacketInfo {
            id,
            direction,
            timestamp: timestamp.to_string(),
            flags,
            packet_type: message_type,
            size,
            opcode,
            sequence,
            queue,
            iteration,
            port,
            raw_json: raw_json.to_string(),
        }
    }
}

#[derive(Parser)]
#[command(name = "pcap")]
#[command(about = "Parse .pcap network capture files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read and print packet data from a pcap file
    Read {
        /// Path to .pcap file
        path: PathBuf,
        /// Optional message index to display (0-based)
        #[arg(short, long)]
        index: Option<usize>,
    },
    /// Interactive view for browsing pcap files
    View {
        /// Path to .pcap file
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Read { path, index } => read::run(&path, index)?,
        Commands::View { path } => tui::run(&path)?,
    }

    Ok(())
}
