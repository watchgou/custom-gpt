pub mod chat;

pub use chat::*;

pub mod completions;

pub use completions::*;

use async_openai::types::{ChatChoice, Choice};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Message {
    pub msg: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_max_token")]
    pub max_token: u16,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResult {
    pub code: u8,
    pub data: Vec<ChatChoice>,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResult {
    pub code: u8,
    pub data: Vec<Choice>,
    pub msg: String,
}

fn default_model() -> String {
    "text-davinci-003".to_string()
}

fn default_max_token() -> u16 {
    7
}

fn default_temperature() -> f32 {
    0.0
}
