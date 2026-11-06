# RustMT5 SDK

> Asynchronous, memory-safe Rust SDK for MetaTrader 5 algorithmic trading automation over gRPC.

[![Docs](https://img.shields.io/badge/docs-RustMT5-blue.svg)](https://metarpc.github.io/RustMT5/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

📄 **Full documentation**: [https://metarpc.github.io/RustMT5/](https://metarpc.github.io/RustMT5/)

---

## 🧱 Three-layer architecture

| Layer | Type | What it gives you |
|-------|------|-------------------|
| Low | `MT5Account` | Raw gRPC, auto-metadata, typed `…Data` payloads |
| Mid | `MT5Service` | Friendly args (typed enums, `&str`), clean scalar returns |
| High | `MT5Sugar` | One-liners: balances, prices, order construction, risk sizing |

Each layer wraps the one below; ready [Strategies](https://metarpc.github.io/RustMT5/Strategies/Strategies.Master.Overview/) build on top.

---

## 📦 Installation

```toml
# Cargo.toml
[dependencies]
metarpc-mt5 = "*"
tokio = { version = "1", features = ["full"] }
tokio-stream = "0.1"   # only if you consume streams
```

or:

```bash
cargo add metarpc-mt5
cargo add tokio --features full
```

> **Windows without MSVC?** Build against the GNU toolchain (`x86_64-pc-windows-gnu`) with a MinGW-w64 install on `PATH`.

---

## 🚀 Minimal example

```rust
use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // user, password, grpc_server (None -> mt5.mrpc.pro:443); API key read from MRPC_API_KEY
    let account = MT5Account::new(12_345_678, "password", None);
    let mut sugar = MT5Sugar::new(MT5Service::new(account)).with_default_symbol("EURUSD");

    sugar.connect_by_server_name("MetaQuotes-Demo", 30).await?;
    println!("balance {:.2}, bid {:.5}", sugar.balance().await?, sugar.bid(None).await?);
    sugar.disconnect(false).await?;
    Ok(())
}
```

---

## 🏃 Running the examples

Clone the repo and configure the connection. Settings come from `examples/0_common/settings.json` **and/or** environment variables — **environment variables override the file**:

```bash
git clone https://github.com/MetaRPC/RustMT5.git
cd RustMT5

# option A — edit the JSON template:
#   examples/0_common/settings.json  (user, password, server_name, api_key, …)

# option B — environment variables (override the file):
export MT5_LOGIN=12345678            # Windows PowerShell: $env:MT5_LOGIN="12345678"
export MT5_PASSWORD="your-password"
export MT5_SERVER="MetaQuotes-Demo"  # defaults to MetaQuotes-Demo if unset
export MRPC_API_KEY="your_api_key"   # defaults to TRIAL if unset
```

Then launch the **interactive menu** and pick a demo by number:

```bash
cargo run --example menu
```

…or run any example directly:

```bash
cargo run --example quickstart
cargo run --example lowlevel_market_data
cargo run --example service_demo
cargo run --example sugar_basics
cargo run --example strategies_demo
```

---

## 🔑 API key & authentication

Connecting to MetaRPC production (`mt5.mrpc.pro:443`) requires an API key:

1. **Sign up** — free at [https://mrpc.pro/signup](https://mrpc.pro/signup).
2. **Generate a key** — in the portal at [https://mrpc.pro/my](https://mrpc.pro/my), under **API Keys**.
3. **Provide it** — the SDK reads the `MRPC_API_KEY` environment variable (falling back to `TRIAL`) and sends it as the `apikey` metadata header on every call.

### Automatic session id

Two things authenticate each request: your **`apikey`** and a terminal **session `id`**. You don't fetch the id — `connect_by_server_name` / `connect_by_host_port` obtain it from the gateway (`ConnectEx` → `terminalInstanceGuid`), and the SDK attaches it to every later call and stream automatically (read it via `account.id()`).

---

## 🌐 Production endpoints

| Environment | Host | Port | Protocol |
| :--- | :--- | :--- | :--- |
| **Production** | `mt5.mrpc.pro` | `443` | TLS / gRPC |
| **Direct API UI (Swagger)** | `https://mt5.mrpc.pro/apiui` | `443` | HTTPS |
| **Portal Dashboard** | `https://mrpc.pro/my` | `443` | HTTPS |
| **Registration / API Key** | `https://mrpc.pro/signup` | `443` | HTTPS |

---

## 📄 Documentation & guides

Full docs at [https://metarpc.github.io/RustMT5/](https://metarpc.github.io/RustMT5/):

- 🚀 **[Getting Started](https://metarpc.github.io/RustMT5/All_Guides/GETTING_STARTED/)** & **[Your First Project](https://metarpc.github.io/RustMT5/All_Guides/Your_First_Project/)**
- 🧱 **[Project Map](https://metarpc.github.io/RustMT5/All_Guides/PROJECT_MAP/)** — the three layers and source layout
- 📡 **[gRPC Streaming](https://metarpc.github.io/RustMT5/All_Guides/GRPC_STREAM_MANAGEMENT/)** — live ticks, trades, position P/L
- 📊 **[Return Codes](https://metarpc.github.io/RustMT5/All_Guides/RETURN_CODES_REFERENCE/)** & **[ENUMs](https://metarpc.github.io/RustMT5/All_Guides/ENUMS_USAGE_REFERENCE/)**
- 💼 **API reference** — [MT5Account](https://metarpc.github.io/RustMT5/MT5Account/MT5Account.Master.Overview/) · [MT5Service](https://metarpc.github.io/RustMT5/MT5Service/MT5Service.Overview/) · [MT5Sugar](https://metarpc.github.io/RustMT5/MT5Sugar/MT5Sugar.Master.Overview/)

---

## 📜 License

MIT
