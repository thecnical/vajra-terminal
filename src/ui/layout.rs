use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};
use crate::app::AppState;

pub fn draw_ui(f: &mut Frame, app: &AppState) {
    let size = f.size();
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(6)].as_ref())
        .split(size);

    // --- 1. HEADER (TABS) ---
    let titles = vec!["🔴 OFFENSIVE", "🔵 DEFENSIVE", "🟡 INTELLIGENCE", "🌐 WEB3/CRYPTO"]
        .into_iter()
        .map(|t| Line::from(Span::styled(t, Style::default().fg(Color::White).add_modifier(Modifier::BOLD))))
        .collect();
    
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" 👾 VAJRA GOD-TIER FRAMEWORK | CREATED BY CHANDAN PANDEY "))
        .select(app.active_tab)
        .style(Style::default().fg(Color::DarkGray))
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, chunks[0]);

    // --- 2. MAIN CONTENT (Logs + Status) ---
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(chunks[1]);

    // Matrix View / Logs
    let log_items: Vec<ListItem> = app.logs.iter()
        .map(|log| ListItem::new(Line::from(Span::styled(log.clone(), Style::default().fg(Color::Green)))))
        .collect();
    
    let logs_list = List::new(log_items)
        .block(Block::default().borders(Borders::ALL).title(" MATRIX DATA STREAM / TERMINAL ").border_style(Style::default().fg(Color::Green)));
    f.render_widget(logs_list, main_chunks[0]);

    // System / Backend Status
    let backend_text = match &app.backend_status {
        Some(status) => format!("\n[ STATUS ]\n{}\n\n[ CREATOR ]\n{}\n\n[ MODULES ]\n{}\n\n[ SWARM NODES ]\n{}", 
            status.status, status.creator, status.modules_active, status.swarm_nodes),
        None => "Connecting to VAJRA BRAIN...".to_string()
    };

    let status_panel = Paragraph::new(backend_text)
        .block(Block::default().borders(Borders::ALL).title(" 🧠 AI BRAIN ").border_style(Style::default().fg(Color::Yellow)));
    f.render_widget(status_panel, main_chunks[1]);

    // --- 3. FOOTER (Chat Box & Shortcuts) ---
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    // Chat Box Area
    let input_display = if app.is_typing {
        format!("> {}_", app.chat_input)
    } else {
        format!("Press 'i' to ask VAJRA AI Assistant...")
    };
    
    let chat_panel = Paragraph::new(format!("{}\n\nAI: {}", input_display, app.ai_response))
        .block(Block::default().borders(Borders::ALL).title(" 🤖 VAJRA AI ASSISTANT ").border_style(Style::default().fg(Color::Magenta)));
    f.render_widget(chat_panel, footer_chunks[0]);

    // Shortcuts Area
    let shortcuts_text = " [a] AI Attack Chain   [p] Polymorphic Payload\n [c] Cloud Slaughter   [z] Quantum Audit\n [t] Trace Crypto      [s] Broadcast Swarm\n [Tab] Switch Module   [q] Quit Matrix";
    let shortcuts = Paragraph::new(shortcuts_text)
        .block(Block::default().borders(Borders::ALL).title(" ⚡ QUICK COMMANDS ").border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(shortcuts, footer_chunks[1]);
}
