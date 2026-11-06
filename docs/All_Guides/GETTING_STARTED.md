# Getting Started with RustMT5

> Start automating MetaTrader 5 from Rust in minutes — connect, read the account, place a trade.

---

## 🎯 What is RustMT5?

**RustMT5** is a type-safe, async Rust client for **MetaTrader 5** terminals over **gRPC**. No C++ DLL wrappers — direct, typed, reliable programmatic trading.

### Three-layer architecture

| Layer | Type | What it gives you |
|-------|------|-------------------|
| Low | [`MT5Account`](../MT5Account/MT5Account.Master.Overview.md) | Raw gRPC, auto-metadata, typed `…Data` payloads |
| Mid | [`MT5Service`](../MT5Service/MT5Service.Overview.md) | Friendly args (typed enums, `&str`), clean scalar returns |
| High | [`MT5Sugar`](../MT5Sugar/MT5Sugar.Master.Overview.md) | One-liners: balances, prices, order construction, risk sizing |

Build on top with ready [Strategies](../Strategies/Strategies.Master.Overview.md).

---

## 📦 Installation

Add the crate and an async runtime:

```toml
# Cargo.toml
[dependencies]
metarpc-mt5 = "*"           # this SDK
tokio = { version = "1", features = ["full"] }
tokio-stream = "0.1"        # only if you consume streams
```

The SDK is async-first and built on [tokio](https://tokio.rs); every API call is `.await`ed.

> **Windows without MSVC?** Build against the GNU toolchain (`x86_64-pc-windows-gnu`) with a MinGW-w64 install on `PATH`. See the project README for the exact toolchain setup.

---

## 🔑 API key & authentication

Connecting to MetaRPC production (`mt5.mrpc.pro:443`) needs an API key:

1. **Sign up** — free at [https://mrpc.pro/signup](https://mrpc.pro/signup).
2. **Generate a key** — in the portal at [https://mrpc.pro/my](https://mrpc.pro/my), under **API Keys**.
3. **Provide it** — the SDK reads `MRPC_API_KEY` from the environment (falling back to `"TRIAL"`), and sends it as the `apikey` metadata header on every call.

### Session id is automatic

Two things authenticate each request: your **`apikey`** and a terminal **session `id`**. You don't fetch the id yourself — `connect_*` obtains it from the gateway (`ConnectEx` → `terminalInstanceGuid`), the SDK stores it, and attaches it to every later unary call and stream. Read it any time with `account.id()`.

---

## 🔌 Minimal connection example

```rust
use metarpc_mt5::MT5Account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // user, password, grpc_server (None → mt5.mrpc.pro:443)
    let mut account = MT5Account::new(12_345_678, "password", None);

    // connect by broker cluster name (timeout in seconds)
    account.connect_by_server_name("MetaQuotes-Demo", 30).await?;

    let s = account.account_summary().await?;
    println!("balance {:.2} {}  equity {:.2}", s.account_balance, s.account_currency, s.account_equity);

    account.disconnect().await?;
    Ok(())
}
```

Prefer the high-level API? Wrap it:

```rust
use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};

let mut sugar = MT5Sugar::new(MT5Service::new(MT5Account::new(12_345_678, "password", None)))
    .with_default_symbol("EURUSD");
sugar.connect_by_server_name("MetaQuotes-Demo", 30).await?;

println!("balance {:.2}, bid {:.5}", sugar.balance().await?, sugar.bid(None).await?);
let ticket = sugar.buy_market_with_pips(None, 0.10, Some(20.0), Some(40.0), "first trade", 1001).await?;
println!("opened #{ticket}");
sugar.disconnect(false).await?;
```

---

## 🧩 Error handling

Every call returns `Result<_, MT5Error>`. A returned `Ok` means the API worked — for **trading** calls, also check the `returned_code` (`10009` = success). See [Return Codes](RETURN_CODES_REFERENCE.md).

```rust
let res = sugar.service().place_order(req).await?;  // transport/gateway errors → MT5Error
if res.returned_code != 10009 {
    eprintln!("trade rejected: {} ({})", res.returned_code, res.returned_code_description);
}
```

---

## 🗺️ Where to go next

1. **[Return Codes](RETURN_CODES_REFERENCE.md)** — interpret trade results.
2. **[ENUMs Usage](ENUMS_USAGE_REFERENCE.md)** — pass property/type enums correctly.
3. **[gRPC Streaming](GRPC_STREAM_MANAGEMENT.md)** — live ticks, trades, position P/L.
4. **[MT5Account](../MT5Account/MT5Account.Master.Overview.md)** · **[MT5Service](../MT5Service/MT5Service.Overview.md)** · **[MT5Sugar](../MT5Sugar/MT5Sugar.Master.Overview.md)** — the API by layer.
5. **[Strategies](../Strategies/Strategies.Master.Overview.md)** — ready orchestrators and presets.
