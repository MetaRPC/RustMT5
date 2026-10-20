# RustMT5 SDK

> Asynchronous Rust SDK for MetaTrader 5 via the MetaRPC gRPC gateway (`mt5.mrpc.pro:443`, TLS).

[![Docs](https://img.shields.io/badge/docs-RustMT5-blue.svg)](https://metarpc.github.io/RustMT5/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

📄 **Full Documentation**: [https://metarpc.github.io/RustMT5/](https://metarpc.github.io/RustMT5/)

---

## 📦 Installation

Crate name: `metarpc-mt5`. It is not on crates.io — clone the repository and use a path dependency:

```bash
git clone https://github.com/MetaRPC/RustMT5.git
```

```toml
[dependencies]
metarpc-mt5 = { path = "../RustMT5" }
tokio = { version = "1", features = ["full"] }
```

The crate ships the prost/tonic generated types in `package/mt5_term_api.rs` (no `protoc` needed
to build) and a high-level `MT5Client` in `src/client.rs`. Transport is tonic 0.12 / prost 0.13.

See **`FLEET_AUDIT.md`** in this folder for the complete, verified call reference (connect, account,
quotes, bars, tick stream, OrderSend/Modify/Close, history), the enums, the gateway quirks and the
Windows / Linux build commands.

---

## 🏃 Quick start

```bash
git clone https://github.com/MetaRPC/RustMT5.git
cd RustMT5

# Windows PowerShell:  $env:MRPC_API_KEY="..."; $env:MT5_LOGIN="..."; $env:MT5_PASSWORD="..."; $env:MT5_SERVER="FBS-Demo"
MRPC_API_KEY=... MT5_LOGIN=... MT5_PASSWORD=... MT5_SERVER=FBS-Demo cargo run --example quickstart
```

```rust
use metarpc_mt5::{timeframe, MT5Client, OrderRequest, TradeAction};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MT5Client::new("mt5.mrpc.pro", 443)                     // api key from MRPC_API_KEY
        .with_id(MT5Client::stable_terminal_id("my-strategy-id"));      // stable terminal id!
    client.connect_ex(106805114, "password", "FBS-Demo", Some("my-strategy-id"), 120).await?;

    let acc = client.get_account_info().await?;
    assert!(acc.trade_mode.is_demo());

    let q = client.ensure_symbol("EURUSD", Duration::from_secs(15)).await?;   // select + wait for a 2-sided quote
    let bars = client.price_history("EURUSD", timeframe::M1, 50).await?;
    let mut ticks = client.subscribe_quotes(vec!["EURUSD".into()]).await?;    // drop `ticks` to unsubscribe

    let res = client.order_send(OrderRequest::market("EURUSD", TradeAction::Buy, 0.01).with_magic(600001).with_comment("S001")).await?;
    client.order_modify(res.order, Some(q.bid * 0.995), Some(q.ask * 1.005)).await?;   // position ticket == order ticket
    client.order_close(res.order, 0.01, 50).await?;
    Ok(())
}
```

## 🧪 Tests

`cargo test` runs offline unit tests. The live test is `#[ignore]`d:

```bash
MRPC_API_KEY=... MT5_LOGIN=... MT5_PASSWORD=... MT5_SERVER=FBS-Demo cargo test -- --ignored
```

---

## 🔑 API Key & Authentication

Connecting to MetaRPC production endpoints (`mt5.mrpc.pro:443`) requires an API key:

1. **Sign Up**: Create an account for free at [https://mrpc.pro/signup](https://mrpc.pro/signup).
2. **Generate API Key**: In your MetaRPC Portal dashboard at [https://mrpc.pro/my](https://mrpc.pro/my), go to **API Keys** to generate and copy your personal API token.
3. **Configure Connection**: Pass it as `MRPC_API_KEY` (read by `MT5Client::new(host, port)`) or explicitly via `MT5Client::with_api_key(host, port, key)`.

---

## 🆔 Automatic Account ID & Authentication

MetaRPC endpoints route calls using a terminal session identifier (`id`):
- `MT5Client::stable_terminal_id(label)` derives a deterministic id (uuid5 of the label) so every run reuses the same terminal; `connect_ex` / `connect` send it, and the server returns the session GUID (`terminalInstanceGuid`).
- The SDK attaches both the `id` and `apikey` metadata headers on all subsequent calls (`AccountSummary`, `OrderSend`, streaming, etc.) — no manual `GetId` or `curl` calls required.
- Pass your API key via `MT5Client::with_api_key(..)` or the `MRPC_API_KEY` environment variable.

## 🌐 Production Endpoints

| Environment | Host | Port | Protocol |
| :--- | :--- | :--- | :--- |
| **Production** | `mt5.mrpc.pro` | `443` | TLS / gRPC |
| **Direct API UI (Swagger)** | `https://mt5.mrpc.pro/apiui` | `443` | HTTPS |
| **Portal Dashboard** | `https://mrpc.pro/my` | `443` | HTTPS |
| **Registration / API Key** | `https://mrpc.pro/signup` | `443` | HTTPS |

---

## 📄 Documentation & Guides

Explore comprehensive documentation at [https://metarpc.github.io/RustMT5/](https://metarpc.github.io/RustMT5/):
- 🚀 **Quick Start & First Project**
- 🔑 **Authentication & API Keys**
- 📡 **Live Market Data & gRPC Streaming**
- 💼 **Account Management & Order Execution**
- 📊 **Return Codes & Error Handling Reference**
