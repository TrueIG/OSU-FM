use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SessionResponse {
    Success { session: Session },
    Error { error: u32, message: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub key: String,
    pub name: String,
    pub subscriber: u8,
}
