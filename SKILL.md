---
name: event-broadcaster
description: Generic async event broadcasting over tokio broadcast channels.
---

# event-broadcaster

event-broadcaster provides a typed publish/subscribe bus on tokio's broadcast channel: named topics, lagged-receiver handling, and graceful shutdown. Use it to decouple producers and consumers inside an async service.

## When to use

Building async Rust services that need internal pub/sub without an external broker.

## Install

```
cargo add event-broadcaster
```

## Links

- Repository: https://github.com/orkid-labs/event-broadcaster
- Package: https://crates.io/crates/event-broadcaster
- Documentation: https://docs.rs/event-broadcaster
- Orkid Labs: https://orkidlabs.xyz
