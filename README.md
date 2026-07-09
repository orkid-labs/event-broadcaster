# Event Broadcaster

A generic event broadcasting system for Rust applications using tokio broadcast channels.

## Features

- Type-safe event broadcasting
- Subscribe to specific event types
- Async/await support with tokio
- JSON serialization for events
- Zero-allocation event routing

## Usage

```rust
use event_broadcaster::{EventBroadcaster, Event, now};

#[derive(serde::Serialize, serde::Deserialize)]
struct MyEvent {
    data: String,
    timestamp: u64,
}

impl Event for MyEvent {
    fn event_type(&self) -> String {
        "MyEvent".to_string()
    }
    
    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

#[tokio::main]
async fn main() {
    let mut broadcaster = EventBroadcaster::new();
    let mut rx = broadcaster.subscribe("MyEvent".to_string());
    
    let event = MyEvent {
        data: "test".to_string(),
        timestamp: now(),
    };
    
    broadcaster.publish(event).unwrap();
    
    let msg = rx.recv().await.unwrap();
    println!("Received: {}", msg);
}
```

## License

MIT
