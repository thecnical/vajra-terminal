use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackendStatus {
    pub status: String,
    pub creator: String,
    pub modules_active: u32,
    pub swarm_nodes: u32,
}

pub struct AppState {
    pub active_tab: usize,
    pub logs: Vec<String>,
    pub backend_connected: bool,
    pub is_running: bool,
    pub backend_status: Option<BackendStatus>,
    
    // Chat state
    pub chat_input: String,
    pub is_typing: bool,
    pub ai_response: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_tab: 0,
            logs: vec![
                "[*] Booting VAJRA Autonomous Core...".to_string(),
                "[*] Created by: Chandan Pandey".to_string(),
                "[*] Initializing Neural Networks...".to_string(),
                "[*] Loading Matrix Render Engine...".to_string(),
                "[+] Environment Ready. Drop into the Matrix.".to_string(),
            ],
            backend_connected: false,
            is_running: true,
            backend_status: None,
            chat_input: String::new(),
            is_typing: false,
            ai_response: "AI Assistant is on standby.".to_string(),
        }
    }

    pub fn next_tab(&mut self) {
        self.active_tab = (self.active_tab + 1) % 4;
    }

    pub fn previous_tab(&mut self) {
        if self.active_tab > 0 {
            self.active_tab -= 1;
        } else {
            self.active_tab = 3;
        }
    }

    pub fn add_log(&mut self, log: String) {
        self.logs.push(log);
        if self.logs.len() > 100 {
            self.logs.remove(0); // Keep buffer clean
        }
    }
}
