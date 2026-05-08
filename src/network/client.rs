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

fn format_json(val: serde_json::Value) -> String {
    match val {
        serde_json::Value::Object(map) => {
            let mut out = Vec::new();
            for (k, v) in map {
                let v_str = match v {
                    serde_json::Value::String(s) => s.to_string(),
                    _ => v.to_string()
                };
                out.push(format!("{}: {}", k, v_str));
            }
            out.join(" | ")
        },
        _ => val.to_string()
    }
}

impl VajraClient {
    pub fn new() -> Self {
        Self {
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
        let val: serde_json::Value = res.json().await?;
        Ok(format_json(val))
    }

    pub async fn scan_cloud(&self, bucket: &str) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "bucket_name": bucket });
        let res = self.client.post(&format!("{}/api/offensive/cloud-slaughter", self.base_url)).json(&payload).send().await?;
        let val: serde_json::Value = res.json().await?;
        Ok(format_json(val))
    }

    pub async fn check_quantum(&self, rsa_size: u32, qubits: u32) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "rsa_size": rsa_size, "qubits": qubits });
        let res = self.client.post(&format!("{}/api/defense/quantum-audit", self.base_url)).json(&payload).send().await?;
        let val: serde_json::Value = res.json().await?;
        Ok(format_json(val))
    }

    pub async fn trace_crypto(&self, wallet: &str) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "wallet": wallet });
        let res = self.client.post(&format!("{}/api/osint/trace-crypto", self.base_url)).json(&payload).send().await?;
        let val: serde_json::Value = res.json().await?;
        Ok(format_json(val))
    }

    pub async fn broadcast_swarm(&self, indicator: &str) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "indicator": indicator, "severity": "CRITICAL" });
        let res = self.client.post(&format!("{}/api/osint/swarm-broadcast", self.base_url)).json(&payload).send().await?;
        let val: serde_json::Value = res.json().await?;
        Ok(format_json(val))
    }

    pub async fn generate_payload(&self, lhost: &str, lport: u32) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "ip": lhost, "port": lport, "lang": "python" });
        let res = self.client.post(&format!("{}/api/offensive/polymorphic", self.base_url)).json(&payload).send().await?;
        let val: serde_json::Value = res.json().await?;
        Ok(format_json(val))
    }

    pub async fn chat_with_ai(&self, message: &str, context: &str) -> Result<String, Box<dyn Error>> {
        let payload = serde_json::json!({ "message": message, "context": context });
        let res = self.client.post(&format!("{}/api/chat", self.base_url)).json(&payload).send().await?;
        let val: serde_json::Value = res.json().await?;
        if let Some(r) = val.get("response").and_then(|v| v.as_str()) {
            return Ok(r.to_string());
        }
        Ok(format_json(val))
    }
}
