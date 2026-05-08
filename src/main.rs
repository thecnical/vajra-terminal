mod app;
mod ui;
mod network;

use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};

use crate::app::AppState;
use crate::network::client::VajraClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::new();
    let client = VajraClient::new();

    loop {
        terminal.draw(|f| ui::layout::draw_ui(f, &app_state))?;

        // Background Check Backend Health
        if !app_state.backend_connected {
            if let Ok(status) = client.check_health().await {
                app_state.backend_status = Some(status);
                app_state.backend_connected = true;
                app_state.add_log("[+] SECURE UPLINK ESTABLISHED WITH AI BRAIN.".to_string());
            }
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // If user is currently typing in the AI Chat
                if app_state.is_typing {
                    match key.code {
                        KeyCode::Enter => {
                            app_state.is_typing = false;
                            app_state.ai_response = "Thinking...".to_string();
                            
                            // Send to AI
                            let input_copy = app_state.chat_input.clone();
                            if let Ok(res) = client.chat_with_ai(&input_copy, "User is in the main dashboard.").await {
                                app_state.ai_response = res;
                            }
                            app_state.chat_input.clear();
                        }
                        KeyCode::Char(c) => {
                            app_state.chat_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app_state.chat_input.pop();
                        }
                        KeyCode::Esc => {
                            app_state.is_typing = false;
                            app_state.chat_input.clear();
                        }
                        _ => {}
                    }
                    continue; // Skip the shortcut keys if typing
                }

                // Global Shortcuts
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Tab => app_state.next_tab(),
                    KeyCode::BackTab => app_state.previous_tab(),
                    KeyCode::Char('i') => {
                        app_state.is_typing = true; // Enter Chat mode
                    }
                    KeyCode::Char('a') => {
                        app_state.add_log("[*] Initializing Autonomous RL Attack Chaining...".to_string());
                        if let Ok(res) = client.trigger_auto_chain("192.168.1.1").await {
                            app_state.add_log(format!("[+] AI Response: {}", res));
                        }
                    }
                    KeyCode::Char('p') => {
                        app_state.add_log("[*] Generating Polymorphic Payload...".to_string());
                        if let Ok(res) = client.generate_payload("192.168.1.100", 4444).await {
                            app_state.add_log(format!("[+] Payload Gen: {}", res));
                        }
                    }
                    KeyCode::Char('c') => {
                        app_state.add_log("[*] Scanning Cloud S3 for Takeover...".to_string());
                        if let Ok(res) = client.scan_cloud("target-corp-backup").await {
                            app_state.add_log(format!("[+] Cloud Analysis: {}", res));
                        }
                    }
                    KeyCode::Char('z') => {
                        app_state.add_log("[*] Running Y2Q Quantum Encryption Audit...".to_string());
                        if let Ok(res) = client.check_quantum(2048, 4096).await {
                            app_state.add_log(format!("[+] Quantum Report: {}", res));
                        }
                    }
                    KeyCode::Char('t') => {
                        app_state.add_log("[*] Tracing Crypto Wallet via Blockchain heuristics...".to_string());
                        if let Ok(res) = client.trace_crypto("0xEvilWallet").await {
                            app_state.add_log(format!("[+] Trace Complete: {}", res));
                        }
                    }
                    KeyCode::Char('s') => {
                        app_state.add_log("[*] Broadcasting Zero-Day to Global Swarm...".to_string());
                        if let Ok(res) = client.broadcast_swarm("hash_of_new_malware").await {
                            app_state.add_log(format!("[+] Swarm Sync: {}", res));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
