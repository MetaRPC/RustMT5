# Async Model — How RustMT5 Executes

> The SDK is **async-only**, built on tokio. There is no blocking/sync API — this guide shows how to run it and when to use streams vs unary calls.

---

## 🎯 The short version

Every API method is `async fn` returning a `Result` (or, for subscriptions, a `Stream`). You run them inside a tokio runtime and `.await` them. There are **no synchronous wrappers** — if you need a blocking call, drive the future yourself (e.g. `tokio::runtime::Runtime::block_on`).

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut account = metarpc_mt5::MT5Account::new(12_345_678, "password", None);
    account.connect_by_server_name("MetaQuotes-Demo", 30).await?;
    let s = account.account_summary().await?;
    println!("balance {:.2}", s.account_balance);
    account.disconnect().await?;
    Ok(())
}
```

---

## Unary vs streaming

| | Unary call | Stream |
|---|-----------|--------|
| **Shape** | `async fn … -> Result<Data, MT5Error>` | `async fn … -> Result<impl Stream<Item = Result<Data, MT5Error>>, MT5Error>` |
| **Returns** | one value, once | many frames over time |
| **Use for** | snapshots: balance, a quote, place/modify/close | live feeds: ticks, trade events, position P/L |
| **Examples** | `account_summary`, `symbol_info_tick`, `order_send` | `on_symbol_tick`, `on_position_profit` |

Consume a stream with `tokio_stream::StreamExt` — see [gRPC Streaming](GRPC_STREAM_MANAGEMENT.md).

---

## Running concurrently

Because calls are futures, you can run independent ones together instead of sequentially:

```rust
// fetch three quotes concurrently
let (a, b, c) = tokio::join!(
    sugar.bid(Some("EURUSD")),
    sugar.bid(Some("GBPUSD")),
    sugar.bid(Some("USDJPY")),
);
println!("{:.5} {:.5} {:.5}", a?, b?, c?);
```

Run background work (e.g. a stream consumer) with `tokio::spawn`; cancel it with the task handle's `abort()`.

> **One shared connection.** `MT5Account` holds one channel/session. Prefer concurrency via `join!`/`spawn` over opening many accounts; the gateway tracks one terminal session per connection.

---

## Driving async from a sync context

If you must call the SDK from non-async code:

```rust
let rt = tokio::runtime::Runtime::new()?;
let summary = rt.block_on(async {
    let mut acc = metarpc_mt5::MT5Account::new(12_345_678, "password", None);
    acc.connect_by_server_name("MetaQuotes-Demo", 30).await?;
    acc.account_summary().await
})?;
```

---

## 💡 Guidance

- **Production bots, multi-symbol, GUIs/services** → embrace async: `#[tokio::main]`, `join!`, `spawn`, streams.
- **Quick scripts / one-offs** → still async, just a small `#[tokio::main]` wrapper (or `block_on`).
- **Never block the runtime** inside an async fn (no `std::thread::sleep`; use `tokio::time::sleep`).

---

## 📚 See also

- [Getting Started](GETTING_STARTED.md) · [gRPC Streaming](GRPC_STREAM_MANAGEMENT.md)
- [Project Map](PROJECT_MAP.md)
