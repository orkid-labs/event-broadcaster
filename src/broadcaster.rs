//! Event broadcasting system

use crate::events::Event;
use crate::BroadcastError;
use std::collections::HashMap;
use tokio::sync::broadcast;

/// Event broadcaster for publishing and subscribing to events
pub struct EventBroadcaster {
    senders: HashMap<String, broadcast::Sender<String>>,
}

impl EventBroadcaster {
    pub fn new() -> Self {
        Self {
            senders: HashMap::new(),
        }
    }

    /// Publish an event to all subscribers of its type
    pub fn publish<E>(&self, event: E) -> Result<(), BroadcastError>
    where
        E: Event + Send + Sync + Clone + 'static + serde::Serialize,
    {
        let event_type = event.event_type();
        if let Some(sender) = self.senders.get(&event_type) {
            let json = serde_json::to_string(&event)?;
            sender.send(json)?;
        }
        Ok(())
    }

    /// Subscribe to events of a specific type
    pub fn subscribe(&mut self, event_type: String) -> broadcast::Receiver<String> {
        self.senders
            .entry(event_type.clone())
            .or_insert_with(|| broadcast::channel(100).0)
            .subscribe()
    }
}

impl Default for EventBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broadcaster_new_is_empty() {
        let b = EventBroadcaster::new();
        assert!(b.senders.is_empty());
    }

    #[test]
    fn broadcaster_default_is_new() {
        let b = EventBroadcaster::default();
        assert!(b.senders.is_empty());
    }

    #[test]
    fn subscribe_creates_channel() {
        let mut b = EventBroadcaster::new();
        let _rx = b.subscribe("test_event".to_string());
        assert_eq!(b.senders.len(), 1);
    }

    #[test]
    fn subscribe_same_type_reuses_channel() {
        let mut b = EventBroadcaster::new();
        let _rx1 = b.subscribe("test_event".to_string());
        let _rx2 = b.subscribe("test_event".to_string());
        assert_eq!(b.senders.len(), 1);
    }

    #[test]
    fn subscribe_different_types() {
        let mut b = EventBroadcaster::new();
        let _rx1 = b.subscribe("test_event".to_string());
        let _rx2 = b.subscribe("other_event".to_string());
        assert_eq!(b.senders.len(), 2);
    }
}
