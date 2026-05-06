// discord/dtos.rs

use serde::Serialize;

// --- discordに送信するためのdto ---
#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub content: String,
}
impl Message {
    pub fn new(message: &str) -> Self {
        Self {
            content: message.to_string(),
        }
    }
}