# RustMT5 SDK

> Asynchronous Rust SDK for MetaTrader 5 via the MetaRPC gRPC gateway (`mt5.mrpc.pro:443`, TLS).

Crate name: `metarpc-mt5`. Not on crates.io — use a path dependency on this folder:

```toml
[dependencies]
metarpc-mt5 = { path = "../lib/RustMT5" }
tokio = { version = "1", features = ["full"] }
```

The crate ships the prost/tonic generated types in `package/mt5_term_api.rs` (no `protoc` needed
to build) and a high-level `MT5Client` in `src/client.rs`. Transport is tonic 0.12 / prost 0.13.

See **`FLEET_AUDIT.md`** in this folder for the complete, verified call reference (connect, account,
quotes, bars, tick stream, OrderSend/Modify/Close, history), the enums, the gateway quirks and the
Windows / Linux build commands.

## Quick start

```bash
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

## Tests

`cargo test` runs offline unit tests. The live test is `#[ignore]`d:

```bash
MRPC_API_KEY=... MT5_LOGIN=... MT5_PASSWORD=... MT5_SERVER=FBS-Demo cargo test -- --ignored
```

## API key

Production (`mt5.mrpc.pro:443`) needs an API key: register at <https://mrpc.pro/signup>, create a
key under <https://mrpc.pro/my>, pass it as `MRPC_API_KEY` or `MT5Client::with_api_key(..)`. Every
call carries the `apikey` and `id` (terminal instance) gRPC metadata headers automatically.
