use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GPTMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct GPTRequest {
    pub model: String,
    pub messages: Vec<GPTMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct GPTResponseMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct GPTChoice {
    pub message: GPTResponseMessage,
    pub finish_reason: String,
    pub index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GPTUsage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GPTResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: GPTUsage,
    pub choices: Vec<GPTChoice>,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct GPTError {
    pub error: Error,
}