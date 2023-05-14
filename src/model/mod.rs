pub mod chat;

pub use chat::*;

pub mod completions;

pub use completions::*;



use async_openai::types::{ChatChoice, Choice};

use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct CompletionMessage {
    pub msg: String,
    pub model: String,
    pub max_token: u16,
    pub temperature: f32,
}

#[derive(Deserialize)]
pub struct ChatMessage {
    pub msg: String
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
