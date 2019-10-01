pub use uuid::Uuid;

pub mod protocols;

pub mod handshake_messages;
pub mod pairing_messages;
pub mod phase_control_messages;

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
        let value = serde_value::to_value(value).expect("Error values must be serializable");
        self.data_fields.insert(key.into(), value);
        self
    }
}

pub type KosemResult<T> = Result<T, KosemError>;
