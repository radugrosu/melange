use anyhow::Result;
use llm::{
    LLMProvider,
    builder::{LLMBackend, LLMBuilder},
    chat::{ChatMessage, ChatRole, MessageType},
    secret_store::SecretStore,
};
use serde::Deserialize;
use std::{fs, str::FromStr};

use crate::rules::generic::RuleWithCode;

#[derive(Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
}

pub struct LlmEngine {
    provider: Box<dyn LLMProvider>,
}

fn get_api_key(backend: &LLMBackend) -> Option<String> {
    let store = SecretStore::new().ok()?;
    match backend {
        LLMBackend::OpenAI => store
            .get("OPENAI_API_KEY")
            .cloned()
            .or_else(|| std::env::var("OPENAI_API_KEY").ok()),
        LLMBackend::Anthropic => store
            .get("ANTHROPIC_API_KEY")
            .cloned()
            .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok()),
        LLMBackend::DeepSeek => store
            .get("DEEPSEEK_API_KEY")
            .cloned()
            .or_else(|| std::env::var("DEEPSEEK_API_KEY").ok()),
        LLMBackend::XAI => store
            .get("XAI_API_KEY")
            .cloned()
            .or_else(|| std::env::var("XAI_API_KEY").ok()),
        LLMBackend::Google => store
            .get("GOOGLE_API_KEY")
            .cloned()
            .or_else(|| std::env::var("GOOGLE_API_KEY").ok()),
        LLMBackend::Groq => store
            .get("GROQ_API_KEY")
            .cloned()
            .or_else(|| std::env::var("GROQ_API_KEY").ok()),
        LLMBackend::Ollama => None,
        LLMBackend::Phind => None,
    }
}
struct SystemPrompt {
    prompt: String,
}
impl From<SystemPrompt> for String {
    fn from(val: SystemPrompt) -> Self {
        val.prompt
    }
}

impl Default for SystemPrompt {
    fn default() -> Self {
        Self {
            prompt: r#"
            Check the provided rule, surrounded by <rule> tags, against the subsequent piece of code,
            surrounded by <code> tags. 
            Make sure you are as fastidious as possible. 
            Quote the beginning of every potential violation.
            Include the specific way in which the code instance violates the rule.
            Be as brief as possible.
            The response should be one valid json object, containing a list of tuples where each tuple 
            is a pair of rule with its corresponding violation.
            "#
            .to_string(),
        }
    }
}

impl LlmEngine {
    pub fn from_config(config_path: &str) -> Result<Self> {
        let config_content = fs::read_to_string(config_path)?;
        let config: LlmConfig = toml::from_str(&config_content)?;
        let backend = LLMBackend::from_str(&config.provider)
            .map_err(|e| anyhow::anyhow!("Invalid provider: {}", e))?;
        let api_key = get_api_key(&backend);
        let system_prompt = SystemPrompt::default();
        let mut builder = LLMBuilder::new()
            .backend(backend)
            .system(system_prompt)
            .stream(false);

        if let Some(api_key) = api_key {
            builder = builder.api_key(api_key);
        }
        if let Some(model) = config.model {
            builder = builder.model(&model);
        }

        if let Some(temp) = config.temperature {
            builder = builder.temperature(temp);
        }

        let provider = builder
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build provider: {}", e))?;
        Ok(Self { provider })
    }

    pub async fn query_with_rule(&self, rule: RuleWithCode) -> Result<String> {
        self.query(rule.to_prompt().as_str()).await
    }

    pub async fn query(&self, prompt: &str) -> Result<String> {
        let messages = vec![ChatMessage {
            role: ChatRole::User,
            message_type: MessageType::Text,
            content: prompt.to_string(),
        }];
        let response = self.provider.chat(&messages).await?;
        response
            .text()
            .ok_or(anyhow::anyhow!("Failed to get response text"))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_with_config() {
        let engine = LlmEngine::from_config("melange-config.toml").unwrap();
        let response = engine
            .query("This is a test. Reply with \"I understand\"")
            .await
            .unwrap();
        assert!(!response.is_empty());
    }
}
