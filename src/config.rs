use crate::gpt::{ChatCompletion, Message};
use anyhow::{Error, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use sled::open;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    api_key: String,
    model: String,
    pub chat_completions: Vec<ChatCompletion>,
    pub previous_chat_log: Vec<Message>,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let project_dirs = ProjectDirs::from("org", "gptcli", "chat").unwrap();
        let config_dir = project_dirs.config_dir();

        let db = open(config_dir.join("config"))?;
        if let Some(data) = db.get("config".as_bytes())? {
            Ok(serde_json::from_slice(&data).unwrap())
        } else {
            Ok(Self {
                api_key: "".into(),
                model: "gpt-3.5-turbo".into(),
                chat_completions: vec![],
                previous_chat_log: vec![],
            })
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        let project_dirs = ProjectDirs::from("org", "gptcli", "chat").unwrap();
        let config_dir = project_dirs.config_dir();
        let db = open(config_dir.join("config"))?;
        db.insert("config".as_bytes(), serde_json::to_vec(self).unwrap())?;
        db.flush()?;
        Ok(())
    }

    pub fn has_api_key(&self) -> bool {
        !self.api_key.is_empty()
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn get_model(&self) -> String {
        self.model.clone()
    }

    pub fn reset(&mut self) {
        self.previous_chat_log = vec![];
    }

    pub fn change_model(&mut self, model: &str) {
        self.model = model.to_string();
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }

    pub fn add_chat_log(&mut self, chat: ChatCompletion) {
        self.chat_completions.push(chat);
    }
}
