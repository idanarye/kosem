pub use uuid::Uuid;

pub mod protocols;

pub mod handshake_messages;
pub mod pairing_messages;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KosemError {
    pub message: String,
}

impl KosemError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub type KosemResult<T> = Result<T, KosemError>;
