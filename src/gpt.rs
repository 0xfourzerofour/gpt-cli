use anyhow::{Error, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::Config;

pub struct GPT {
    pub previous: Vec<Message>,
    client: Client,
    config: Config,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletion {
    pub id: String,
    object: String,
    created: i64,
    pub choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Choice {
    index: i32,
    pub message: Message,
    finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

impl Message {
    pub fn new(role: String, content: String) -> Self {
        Self { role, content }
    }
}

const GPT_URL: &str = "https://api.openai.com/v1/chat/completions";

impl GPT {
    pub fn new(config: Config) -> Self {
        Self {
            previous: vec![],
            client: Client::new(),
            config,
        }
    }

    pub fn update_api_key(&mut self, key: &String) {
        self.config.set_api_key(key.to_owned());
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.config.save()
    }

    pub fn append_to_log(&mut self, m: Message) {
        self.config.previous_chat_log.push(m)
    }

    pub async fn ask_question(&mut self, question: &str) -> Result<ChatCompletion, Error> {
        let mut headers = HeaderMap::new();

        let api_key = &self.config.get_api_key();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key.as_str())).unwrap(),
        );

        let message_body = Message::new("user".to_string(), question.to_string());

        self.append_to_log(message_body.clone());

        let request_body = json!({
            "model": self.config.get_model(),
            "messages": self.config.previous_chat_log,
        });

        // Send the request to the OpenAI API
        let response = self
            .client
            .post(GPT_URL)
            .headers(headers)
            .json(&request_body)
            .send()
            .await?;

        let response_text = response.text().await?;

        let chat_completion: ChatCompletion = serde_json::from_str(&response_text)?;

        self.append_to_log(chat_completion.choices[0].message.clone());

        Ok(chat_completion)
    }

    pub fn change_model(&mut self, model: &str) {
        self.config.change_model(model)
    }

    pub fn reset_chat_log(&mut self) {
        self.config.reset()
    }
}
