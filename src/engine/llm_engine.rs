use anyhow::Result;
use reqwest::blocking::Client;
use serde_json::{self, json};

pub struct LlmEngine {
    client: Client,
    endpoint: String,
}

impl LlmEngine {
    pub fn new(endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.to_string(),
        }
    }

    pub fn query(&self, model: &str, prompt: &str) -> Result<String> {
        let response = self
            .client
            .post(format!("{}/api/generate", self.endpoint))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": model,
                "prompt": prompt,
            }))
            .send()?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "HTTP request failed with status: {}",
                response.status()
            ));
        }
        let raw_response = response.text()?;
        let mut output = String::new();

        for line in raw_response.lines() {
            if let Ok(response_json) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(text) = response_json["response"].as_str() {
                    output.push_str(text);
                }
            } else {
                return Err(anyhow::anyhow!("Failed to parse JSON line: {}", line));
            }
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_engine_ollama() {
        let engine = LlmEngine::new("http://localhost:11434");
        let response = engine.query("qwen2.5-coder:0.5b", "hi there").unwrap();
        println!("{}", response);
        assert!(!response.is_empty());
    }

    #[test]
    fn test_llm_engine_openai() {
        let engine = LlmEngine::new("http://localhost:11434");
        let response = engine.query("qwen2.5-coder:0.5b", "hi there").unwrap();
        println!("{}", response);
        assert!(!response.is_empty());
    }
}
