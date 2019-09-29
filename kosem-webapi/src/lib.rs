pub use uuid::Uuid;

pub mod protocols;

pub mod handshake_messages;
pub mod pairing_messages;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KosemError {
    pub message: String,
    pub data_fields: std::collections::HashMap<String, serde_value::Value>,
}

impl KosemError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            data_fields: <_>::default(),
        }
    }

    pub fn with(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.data_fields.insert(key.into(), serde_value::to_value(value).unwrap());
        self
    }
}

pub type KosemResult<T> = Result<T, KosemError>;
