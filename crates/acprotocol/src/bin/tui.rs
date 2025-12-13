use anyhow::Result;
use std::path::Path;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Stylize,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Terminal,
};
use serde_json::Value;
use std::io;

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
    headers: String,
    packet_type: String,
    size: usize,
    extra_info: String,
    opcode: u32,
    sequence: u32,
    queue: String,
    iteration: String,
    port: String,
    raw_json: String,
}

struct App {
    packets: Vec<PacketInfo>,
    selected: usize,
    scroll_offset: usize,
}

impl App {
    fn new(packets: Vec<PacketInfo>) -> Self {
        App {
            packets,
            selected: 0,
            scroll_offset: 0,
        }
    }

    fn next(&mut self, visible_rows: usize) {
        if !self.packets.is_empty() {
            self.selected = (self.selected + 1) % self.packets.len();
            self.update_scroll(visible_rows);
        }
    }

    fn prev(&mut self, visible_rows: usize) {
        if !self.packets.is_empty() {
            self.selected = if self.selected == 0 {
                self.packets.len() - 1
            } else {
                self.selected - 1
            };
            self.update_scroll(visible_rows);
        }
    }

    fn page_down(&mut self, visible_rows: usize) {
        if !self.packets.is_empty() {
            self.selected = (self.selected + visible_rows).min(self.packets.len() - 1);
            self.update_scroll(visible_rows);
        }
    }

    fn page_up(&mut self, visible_rows: usize) {
        if !self.packets.is_empty() {
            self.selected = self.selected.saturating_sub(visible_rows);
            self.update_scroll(visible_rows);
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
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                // Estimate visible rows (roughly 15-20 depending on screen)
                let visible_rows = 15;
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next(visible_rows),
                    KeyCode::Up | KeyCode::Char('k') => app.prev(visible_rows),
                    KeyCode::PageDown => app.page_down(visible_rows),
                    KeyCode::PageUp => app.page_up(visible_rows),
                    KeyCode::Home => {
                        app.selected = 0;
                        app.scroll_offset = 0;
                    }
                    KeyCode::End => {
                        if !app.packets.is_empty() {
                            app.selected = app.packets.len() - 1;
                            app.update_scroll(visible_rows);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(10),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Pcap Packet Viewer")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Table
    let header = Row::new(vec![
        "#", "Dir", "Timestamp", "Headers", "Type", "Size", "Info", "OpCode", "Seq", "Queue",
        "Iter", "Port",
    ])
    .style(Style::default().bg(Color::DarkGray).bold())
    .bottom_margin(0);

    let visible_rows = chunks[1].height.saturating_sub(2) as usize; // Account for borders

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
                packet.headers.clone(),
                packet.packet_type.clone(),
                packet.size.to_string(),
                packet.extra_info.clone(),
                packet.opcode.to_string(),
                packet.sequence.to_string(),
                packet.queue.clone(),
                packet.iteration.clone(),
                packet.port.clone(),
            ];

            let style = if i == app.selected {
                Style::default().bg(Color::Cyan).fg(Color::Black)
            } else {
                Style::default()
            };

            Row::new(cells).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Length(6),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Length(5),
            Constraint::Length(6),
            Constraint::Length(5),
            Constraint::Length(5),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Packets"));

    f.render_widget(table, chunks[1]);

    // Details panel
    let detail_text = if !app.packets.is_empty() {
        let packet = &app.packets[app.selected];
        format!(
            "Packet #{} - {}\nMessage Type: {}\nRaw JSON:\n{}",
            packet.id,
            packet.direction,
            packet.packet_type,
            truncate_json(&packet.raw_json, 500)
        )
    } else {
        "No packets loaded".to_string()
    };

    let detail_para = Paragraph::new(detail_text)
        .block(Block::default().title("Details").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(detail_para, chunks[2]);
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

        // Skip first 42 bytes (Ethernet + IP + UDP headers)
        if packet.data.len() <= 42 {
            continue;
        }
        let udp_payload = &packet.data[42..];

        // Try to parse messages from this packet
        match assembler.parse_packet_payload(udp_payload) {
            Ok(messages) => {
                for msg in messages {
                    if let Ok(json_str) = serde_json::to_string(&msg) {
                        if let Ok(json_val) = serde_json::from_str::<Value>(&json_str) {
                            packet_num += 1;
                            let info = extract_packet_info(packet_num, &json_val, &json_str);
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

fn extract_packet_info(id: u32, json_val: &Value, raw_json: &str) -> PacketInfo {
    let id_from_json = json_val.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
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

    // Calculate approximate size (raw JSON string length)
    let size = raw_json.len();

    // Extract headers info from direction
    let headers = match direction.as_str() {
        "Send" => "C2S".to_string(),
        "Recv" => "S2C".to_string(),
        _ => "Unknown".to_string(),
    };

    PacketInfo {
        id,
        direction,
        timestamp: "N/A".to_string(), // Timestamp not available in parsed data yet
        headers,
        packet_type: message_type,
        size,
        extra_info: format!("ID:{}", id_from_json),
        opcode,
        sequence,
        queue: "0".to_string(), // Placeholder
        iteration: "0".to_string(), // Placeholder
        port: "0".to_string(), // Placeholder
        raw_json: raw_json.to_string(),
    }
}
