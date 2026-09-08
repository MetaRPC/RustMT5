# RustMT5 SDK

Welcome to the **RustMT5 SDK** documentation. This high-performance asynchronous Rust crate connects directly to MetaTrader 5 servers without requiring a local desktop terminal or Wine.

## Key Features

- **Memory Safety & Performance**: Built purely in safe, modern Rust.
- **Asynchronous Tokio Engine**: Non-blocking channel streams for ticks and order responses.
- **Complete Order Lifecycle**: Market execution, pending limit/stop/stop-limit orders, SL/TP modify, and position closing.
- **Type-Safe Models**: Strongly typed structs and enums with Serde serialization.

## Architecture

```mermaid
graph TD
    A[Rust Application] -->|Tokio Async Socket| B[MT5Client]
    B -->|Encrypted Protocol| C[MetaTrader 5 Server]
    C -->|Price Feeds| B
    C -->|Order Confirmations| B
    B -->|mpsc::Receiver<Quote>| A
```

## Quick Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
metarpc-mt5 = { git = "https://github.com/MetaRPC/RustMT5.git" }
tokio = { version = "1.38", features = ["full"] }
```

See [Getting Started](getting-started.md) to build your first strategy.
