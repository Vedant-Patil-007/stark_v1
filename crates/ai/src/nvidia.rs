use std::time::Instant;
use serde::{Deserialize, Serialize};

use crate::action::AiAction;
use crate::error::{AiError, Result};
use crate::provider::{extract_json, system_prompt, AiProvider, CommandContext, ProviderResponse};

const ENDPOINT: &str = "https://integrate.api.nvidia.com/v1/chat/completions";

pub struct NvidiaProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl NvidiaProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    temperature: f32,
    top_p: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    #[serde(default)]
    #[allow(dead_code)]
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Usage {
    #[serde(default)]
    pub prompt_tokens: u32,
    #[serde(default)]
    pub completion_tokens: u32,
    #[serde(default)]
    pub total_tokens: u32,
}

#[async_trait::async_trait]
impl AiProvider for NvidiaProvider {
    async fn interpret(&self, ctx: &CommandContext) -> Result<ProviderResponse> {
        let started = Instant::now();
        let prompt = system_prompt(ctx);

        let body = ChatRequest {
            model: &self.model,
            messages: vec![
                Message { role: "system", content: &prompt },
                Message { role: "user", content: &ctx.instruction },
            ],
            // Low temperature: the same input should produce the same
            // structured action, not creative variation.
            temperature: 0.1,
            top_p: 0.7,
            max_tokens: 1024,
        };

        let resp = self
            .client
            .post(ENDPOINT)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() || e.is_timeout() {
                    AiError::Unavailable(e.to_string())
                } else {
                    AiError::Provider(e.to_string())
                }
            })?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(AiError::Provider(format!("HTTP {status}: {text}")));
        }

        let parsed: ChatResponse = resp
            .json()
            .await
            .map_err(|e| AiError::Provider(format!("malformed response: {e}")))?;

        let raw = parsed
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| AiError::Provider("no choices in response".into()))?;

        let json = extract_json(&raw)?;
        let action: AiAction = serde_json::from_str(json)
            .map_err(|e| AiError::Parse(format!("{e}; raw was: {raw}")))?;

        Ok(ProviderResponse {
            action,
            raw,
            model: self.model.clone(),
            latency_ms: started.elapsed().as_millis() as u64,
        })
    }

    fn name(&self) -> &'static str {
        "nvidia"
    }
}