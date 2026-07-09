//! Event types and traits

use serde::{Deserialize, Serialize};

/// Base event trait
pub trait Event {
    fn event_type(&self) -> String;
    fn timestamp(&self) -> u64;
}

/// Example event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventType {
    PoolUpdate,
    PriceUpdate,
    TradeExecuted,
    TradeFailed,
    SystemStatus,
}

/// Example pool update event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolUpdateEvent {
    pub pool_id: String,
    pub token0: String,
    pub token1: String,
    pub price: f64,
    pub tvl_usd: f64,
    pub timestamp: u64,
}

impl Event for PoolUpdateEvent {
    fn event_type(&self) -> String {
        "PoolUpdate".to_string()
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

/// Example trade executed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeExecutedEvent {
    pub trade_id: String,
    pub tx_hash: String,
    pub amount: String,
    pub timestamp: u64,
}

impl Event for TradeExecutedEvent {
    fn event_type(&self) -> String {
        "TradeExecuted".to_string()
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

/// Example system status event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusEvent {
    pub component: String,
    pub status: String,
    pub message: Option<String>,
    pub timestamp: u64,
}

impl Event for SystemStatusEvent {
    fn event_type(&self) -> String {
        "SystemStatus".to_string()
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_type_equality() {
        assert_eq!(EventType::PoolUpdate, EventType::PoolUpdate);
        assert_ne!(EventType::PoolUpdate, EventType::PriceUpdate);
    }

    #[test]
    fn event_type_serde_roundtrip() {
        let event_type = EventType::TradeExecuted;
        let json = serde_json::to_string(&event_type).unwrap();
        let deserialized: EventType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, event_type);
    }

    #[test]
    fn pool_update_event_trait() {
        let event = PoolUpdateEvent {
            pool_id: "pool-1".to_string(),
            token0: "WETH".to_string(),
            token1: "USDC".to_string(),
            price: 2000.0,
            tvl_usd: 4_000_000.0,
            timestamp: 1700000000,
        };
        assert_eq!(event.event_type(), "PoolUpdate");
        assert_eq!(event.timestamp(), 1700000000);
    }

    #[test]
    fn pool_update_event_serde() {
        let event = PoolUpdateEvent {
            pool_id: "test-pool".to_string(),
            token0: "WETH".to_string(),
            token1: "DAI".to_string(),
            price: 1.5,
            tvl_usd: 1500.0,
            timestamp: 1700000000,
        };
        let json = serde_json::to_string(&event).unwrap();
        let deserialized: PoolUpdateEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.pool_id, event.pool_id);
        assert_eq!(deserialized.price, event.price);
    }

    #[test]
    fn trade_executed_event_trait() {
        let event = TradeExecutedEvent {
            trade_id: "trade-1".to_string(),
            tx_hash: "0xabc".to_string(),
            amount: "1000".to_string(),
            timestamp: 1700000000,
        };
        assert_eq!(event.event_type(), "TradeExecuted");
        assert_eq!(event.timestamp(), 1700000000);
    }

    #[test]
    fn system_status_event_trait() {
        let event = SystemStatusEvent {
            component: "executor".to_string(),
            status: "running".to_string(),
            message: Some("all systems go".to_string()),
            timestamp: 1700000000,
        };
        assert_eq!(event.event_type(), "SystemStatus");
        assert_eq!(event.timestamp(), 1700000000);
    }
}
