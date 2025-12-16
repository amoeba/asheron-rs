use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

// Border height in terminal UI (top and bottom borders)
const BORDER_HEIGHT: usize = 2;

mod tui {
    use anyhow::Result;
    use std::path::Path;

    use crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers,
            MouseEvent, MouseEventKind,
        },
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    };
    use ratatui::{
        Terminal,
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout},
        prelude::Stylize,
        style::{Color, Modifier, Style},
        text::Span,
        widgets::{Block, Borders, Paragraph, Row, Table},
    };
    use serde_json::Value;
    use std::io;

    use crate::BORDER_HEIGHT;
    use acprotocol::network::{FragmentAssembler, ParsedMessage};

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
        opcode: String,
        sequence: u32,
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
        selected_column: usize, // Index of the column header being selected (0-7)
        column_rects: Vec<(u16, u16, SortColumn)>, // (start, end, column)
        list_pane_right: u16,   // Right boundary of list pane
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
                    self.tree_scroll_offset =
                        self.tree_focused_line.saturating_sub(visible_rows - 1);
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
                };
                if self.sort_ascending {
                    cmp
                } else {
                    cmp.reverse()
                }
            });
        }

        fn move_column_left(&mut self) {
            if self.selected_column > 0 {
                self.selected_column -= 1;
            }
        }

        fn move_column_right(&mut self) {
            if self.selected_column < 7 {
                // 8 columns total (0-7)
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

    fn run_app(
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        app: &mut App,
    ) -> io::Result<()> {
        loop {
            // Get detail lines before drawing (for Enter key handling)
            let detail_lines = if !app.packets.is_empty() {
                let packet = &app.packets[app.selected];
                match serde_json::from_str::<Value>(&packet.raw_json) {
                    Ok(json_val) => {
                        tree_display_lines(&json_val, &app.tree_expanded)
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
                            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                return Ok(());
                            }
                            KeyCode::Tab => {
                                // Switch between panes
                                app.focused_pane = match app.focused_pane {
                                    FocusedPane::List => FocusedPane::Details,
                                    FocusedPane::Details => FocusedPane::List,
                                };
                            }
                            KeyCode::Char('j') | KeyCode::Down => match app.focused_pane {
                                FocusedPane::List => app.next(visible_rows),
                                FocusedPane::Details => {
                                    let total_lines = detail_lines.len();
                                    app.tree_line_down(total_lines, visible_rows);
                                }
                            },
                            KeyCode::Char('k') | KeyCode::Up => match app.focused_pane {
                                FocusedPane::List => app.prev(visible_rows),
                                FocusedPane::Details => app.tree_line_up(),
                            },
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
                                    let focused_global_line =
                                        app.tree_scroll_offset + app.tree_focused_line;
                                    if focused_global_line < detail_lines.len() {
                                        let (line, path) = &detail_lines[focused_global_line];
                                        // Only toggle if line contains expand/collapse markers
                                        if (line.contains("▶") || line.contains("▼"))
                                            && !path.is_empty()
                                        {
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
                                // Ctrl+a: expand all nodes
                                if matches!(app.focused_pane, FocusedPane::Details)
                                    && !app.packets.is_empty()
                                {
                                    let packet = &app.packets[app.selected];
                                    if let Ok(json_val) =
                                        serde_json::from_str::<Value>(&packet.raw_json)
                                    {
                                        collect_all_paths(&json_val, String::new(), &mut app.tree_expanded);
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
            return; // Click is in the details pane, ignore
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

    fn tree_display_lines(value: &Value, expanded: &std::collections::HashSet<String>) -> Vec<(String, String)> {
        let mut lines = Vec::new();
        format_json_value(value, 0, &mut lines, expanded, String::new());
        lines
    }

    fn format_json_value(
        value: &Value,
        indent: usize,
        lines: &mut Vec<(String, String)>,
        expanded: &std::collections::HashSet<String>,
        path: String,
    ) {
        let indent_str = "  ".repeat(indent);

        match value {
            Value::Object(obj) => {
                for (key, val) in obj {
                    let item_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };

                    match val {
                        Value::Object(_) | Value::Array(_) => {
                            let is_expanded = expanded.contains(&item_path);
                            let indicator = if is_expanded { "▼" } else { "▶" };
                            lines.push((
                                format!("{}{} {} {}", indent_str, indicator, key, ":"),
                                item_path.clone(),
                            ));
                            if is_expanded {
                                format_json_value(val, indent + 1, lines, expanded, item_path);
                            }
                        }
                        _ => {
                            lines.push((
                                format!("{}{}: {}", indent_str, key, val),
                                item_path,
                            ));
                        }
                    }
                }
            }
            Value::Array(arr) => {
                for (idx, val) in arr.iter().enumerate() {
                    let item_path = format!("{}[{}]", path, idx);

                    match val {
                        Value::Object(_) | Value::Array(_) => {
                            let is_expanded = expanded.contains(&item_path);
                            let indicator = if is_expanded { "▼" } else { "▶" };
                            lines.push((
                                format!("{}{}[{}]", indent_str, indicator, idx),
                                item_path.clone(),
                            ));
                            if is_expanded {
                                format_json_value(val, indent + 1, lines, expanded, item_path);
                            }
                        }
                        _ => {
                            lines.push((
                                format!("{}[{}]: {}", indent_str, idx, val),
                                item_path,
                            ));
                        }
                    }
                }
            }
            _ => {
                lines.push((format!("{}{}", indent_str, value), path));
            }
        }
    }

    fn collect_all_paths(value: &Value, path: String, expanded: &mut std::collections::HashSet<String>) {
        match value {
            Value::Object(obj) => {
                for (key, val) in obj {
                    let item_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };

                    if matches!(val, Value::Object(_) | Value::Array(_)) {
                        expanded.insert(item_path.clone());
                        collect_all_paths(val, item_path, expanded);
                    }
                }
            }
            Value::Array(arr) => {
                for (idx, val) in arr.iter().enumerate() {
                    let item_path = format!("{}[{}]", path, idx);

                    if matches!(val, Value::Object(_) | Value::Array(_)) {
                        expanded.insert(item_path.clone());
                        collect_all_paths(val, item_path, expanded);
                    }
                }
            }
            _ => {}
        }
    }

    fn ui(f: &mut ratatui::Frame, app: &mut App) {
        let outer_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(2)])
            .split(f.area());

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(66), Constraint::Percentage(34)])
            .split(outer_chunks[0]);

        // Store the right boundary of the list pane for mouse click detection
        app.list_pane_right = chunks[0].right();

        // Table header with sort indicators and column selection
        let arrow = if app.sort_ascending { "▲" } else { "▼" };
        let col_names = [
            "#",
            "Dir",
            "Timestamp",
            "Flags",
            "Message Type",
            "Size",
            "OpCode",
            "Seq",
        ];

        let header_cells: Vec<Span> = col_names
            .iter()
            .enumerate()
            .map(|(idx, name)| {
                let has_sort = match idx {
                    0 => app.sort_column == SortColumn::Id,
                    1 => app.sort_column == SortColumn::Direction,
                    2 => app.sort_column == SortColumn::Timestamp,
                    3 => app.sort_column == SortColumn::Flags,
                    4 => app.sort_column == SortColumn::MessageType,
                    5 => app.sort_column == SortColumn::Size,
                    6 => app.sort_column == SortColumn::OpCode,
                    7 => app.sort_column == SortColumn::Sequence,
                    _ => false,
                };

                let text = if has_sort {
                    format!("{}{}", name, arrow)
                } else {
                    name.to_string()
                };

                // Highlight selected column with black background and white text
                if idx == app.selected_column {
                    Span::styled(
                        text,
                        Style::default()
                            .bg(Color::Black)
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw(text)
                }
            })
            .collect();

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
                    packet.opcode.clone(),
                    packet.sequence.to_string(),
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
        let col_widths = [4, 4, 11, 15, 20, 5, 10, 4];
        let col_headers = [
            SortColumn::Id,
            SortColumn::Direction,
            SortColumn::Timestamp,
            SortColumn::Flags,
            SortColumn::MessageType,
            SortColumn::Size,
            SortColumn::OpCode,
            SortColumn::Sequence,
        ];

        app.column_rects.clear();
        let mut col_start = 2; // Account for left border + padding
        for (width, col) in col_widths.iter().zip(col_headers.iter()) {
            let col_end = col_start + width;
            app.column_rects
                .push((col_start as u16, col_end as u16, *col));
            col_start = col_end + 1; // Add 1 for spacing between columns
        }

        let table = Table::new(
            rows,
            [
                Constraint::Length(4),  // #
                Constraint::Length(4),  // Dir
                Constraint::Length(11), // Timestamp
                Constraint::Length(15), // Flags
                Constraint::Min(20),    // Message Type (expandable)
                Constraint::Length(5),  // Size
                Constraint::Length(10), // OpCode
                Constraint::Length(4),  // Seq
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
                .title("Messages")
                .style(style)
        });

        f.render_widget(table, chunks[0]);

        // Details panel
        let detail_title = "Details";

        if !app.packets.is_empty() {
            let packet = &app.packets[app.selected];

            // Parse JSON and build display
            let detail_lines = match serde_json::from_str::<Value>(&packet.raw_json) {
                Ok(json_val) => {
                    tree_display_lines(&json_val, &app.tree_expanded)
                }
                Err(e) => {
                    vec![(
                        format!("Failed to parse JSON: {}\n\nRaw:\n{}", e, &packet.raw_json),
                        String::new(),
                    )]
                }
            };

            let visible_rows = chunks[1].height.saturating_sub(BORDER_HEIGHT as u16) as usize;

            // Build styled lines with focused line highlighted
            let mut styled_lines = Vec::new();
            for (i, (line, _path)) in detail_lines.iter().enumerate() {
                if i >= app.tree_scroll_offset && i < app.tree_scroll_offset + visible_rows {
                    let display_idx = i - app.tree_scroll_offset;
                    let style = get_focused_style(
                        display_idx == app.tree_focused_line
                            && matches!(app.focused_pane, FocusedPane::Details),
                        matches!(app.focused_pane, FocusedPane::Details),
                    );
                    styled_lines.push(ratatui::text::Line::from(ratatui::text::Span::styled(
                        line.clone(),
                        style,
                    )));
                }
            }

            let detail_para = Paragraph::new(styled_lines).block({
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

            let detail_para = Paragraph::new("No messages loaded")
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
        let controls = Paragraph::new(controls_text).block(Block::default().borders(Borders::TOP));
        f.render_widget(controls, outer_chunks[1]);
    }

    fn load_packets(path: &Path) -> Result<Vec<PacketInfo>> {
        use acprotocol::network::pcap;

        let mut assembler = FragmentAssembler::new();
        let mut messages: Vec<ParsedMessage> = vec![];

        let pcap_iter = pcap::open(path)?;
        for packet_result in pcap_iter {
            let packet = packet_result?;
            let parsed_messages = assembler.parse_packet_payload(&packet.data)?;
            messages.extend(parsed_messages);
        }

        let mut packet_infos = Vec::new();

        for msg in messages {
            let info = PacketInfo {
                id: msg.id,
                direction: msg.direction.clone(),
                timestamp: "".to_string(),
                flags: "".to_string(),
                packet_type: msg.message_type.clone(),
                size: msg.data.len(),
                opcode: format!("{:#06x}", msg.opcode),
                sequence: msg.sequence,
                raw_json: serde_json::to_string(&msg).unwrap_or_default(),
            };
            packet_infos.push(info);
        }

        Ok(packet_infos)
    }
}

#[derive(Parser)]
#[command(name = "tui")]
#[command(about = "Enhanced TUI for browsing .pcap network capture files", long_about = None)]
struct Cli {
    /// Path to .pcap file
    path: PathBuf,
    /// Print all messages to stdout instead of launching interactive TUI
    #[arg(short, long)]
    print: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.print {
        print_messages(&cli.path)?;
    } else {
        tui::run(&cli.path)?;
    }
    Ok(())
}

fn print_messages(path: &std::path::Path) -> anyhow::Result<()> {
    use acprotocol::network::pcap;
    use acprotocol::network::FragmentAssembler;

    let mut assembler = FragmentAssembler::new();
    let pcap_iter = pcap::open(path)?;
    
    for packet_result in pcap_iter {
        let packet = packet_result?;
        let parsed_messages = assembler.parse_packet_payload(&packet.data)?;
        for msg in parsed_messages {
            println!("{}", serde_json::to_string_pretty(&msg)?);
        }
    }

    Ok(())
}
