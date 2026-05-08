// Updating the client to support the Chat endpoint
use reqwest::Client;
use std::error::Error;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BackendStatus {
    pub status: String,
    pub creator: String,
    pub modules_active: u32,
    pub swarm_nodes: u32,
}

pub struct VajraClient {
    base_url: String,
    client: Client,
}

impl VajraClient {
    pub fn new() -> Self {
        Self {
            // Pointing to Production Render AI Cloud instead of localhost
            base_url: "https://vajra-brain.onrender.com".to_string(),
            client: Client::new(),
        }
    }

    pub async fn check_health(&self) -> Result<BackendStatus, Box<dyn Error>> {
        let res = self.client.get(&format!("{}/", self.base_url)).send().await?;
        let status: BackendStatus = res.json().await?;
        Ok(status)
    }

    pub async fn trigger_auto_chain(&self, target: &str) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({
            "target": target,
            "data": {"open_ports": [22, 6379], "services": ["ssh", "redis"]}
        });
        let res = self.client.post(&format!("{}/api/offensive/auto-chain", self.base_url)).json(&payload).send().await?;
        Ok(res.text().await?)
    }

    pub async fn scan_cloud(&self, bucket: &str) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "bucket_name": bucket });
        let res = self.client.post(&format!("{}/api/offensive/cloud-slaughter", self.base_url)).json(&payload).send().await?;
        Ok(res.text().await?)
    }

    pub async fn check_quantum(&self, rsa_size: u32, qubits: u32) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "rsa_size": rsa_size, "qubits": qubits });
        let res = self.client.post(&format!("{}/api/defense/quantum-audit", self.base_url)).json(&payload).send().await?;
        Ok(res.text().await?)
    }

    pub async fn trace_crypto(&self, wallet: &str) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "wallet": wallet });
        let res = self.client.post(&format!("{}/api/osint/trace-crypto", self.base_url)).json(&payload).send().await?;
        Ok(res.text().await?)
    }

    pub async fn broadcast_swarm(&self, indicator: &str) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "indicator": indicator, "severity": "CRITICAL" });
        let res = self.client.post(&format!("{}/api/osint/swarm-broadcast", self.base_url)).json(&payload).send().await?;
        Ok(res.text().await?)
    }

    pub async fn generate_payload(&self, lhost: &str, lport: u32) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "target_os": "linux", "format": "elf", "lhost": lhost, "lport": lport });
        let res = self.client.post(&format!("{}/api/offensive/polymorphic-payload", self.base_url)).json(&payload).send().await?;
        Ok(res.text().await?)
    }

    pub async fn chat_with_ai(&self, message: &str, context: &str) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "message": message, "context": context });
        let res = self.client.post(&format!("{}/api/chat", self.base_url)).json(&payload).send().await?;
        Ok(res.text().await?)
    }
}
