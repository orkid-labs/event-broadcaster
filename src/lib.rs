//! Event broadcasting system for Rust applications
//!
//! Provides a generic event broadcasting system using tokio broadcast channels.
//! Publishers can broadcast events to subscribers by event type.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::broadcast;

pub mod broadcaster;
pub mod events;

pub use broadcaster::*;
pub use events::*;

/// Error type for event broadcasting
#[derive(Debug, Error)]
pub enum BroadcastError {
    #[error("Serialization failed: {0}")]
    SerializationError(String),
    
    #[error("Send error: {0}")]
    SendError(String),
}

impl From<broadcast::error::SendError<String>> for BroadcastError {
    fn from(err: broadcast::error::SendError<String>) -> Self {
        BroadcastError::SendError(err.to_string())
    }
}

impl From<serde_json::Error> for BroadcastError {
    fn from(err: serde_json::Error) -> Self {
        BroadcastError::SerializationError(err.to_string())
    }
}

/// Helper function to get current timestamp in seconds
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
