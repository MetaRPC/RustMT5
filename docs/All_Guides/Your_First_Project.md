# Your First Project in 10 Minutes

> Hands-on quick start — a working MetaTrader 5 bot in Rust, from empty folder to a live balance print and a demo trade.

---

## Step 0 — Get an API key

To reach MetaRPC (`mt5.mrpc.pro:443`):

1. Register free at [https://mrpc.pro/signup](https://mrpc.pro/signup).
2. Generate a token in the portal at [https://mrpc.pro/my](https://mrpc.pro/my).
3. Expose it to your program: `export MRPC_API_KEY=your_token` (the SDK reads `MRPC_API_KEY`, falling back to `"TRIAL"`).

You also need an MT5 **demo account** (login, password, server name) — see [MT5 for Beginners](MT5_For_Beginners.md).

---

## Step 1 — Create the project

```bash
cargo new my_rustmt5_bot
cd my_rustmt5_bot
cargo add metarpc-mt5
cargo add tokio --features full
```

---

## Step 2 — Write `src/main.rs`

```rust
use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) construct: user, password, grpc_server (None → mt5.mrpc.pro:443)
    let account = MT5Account::new(12_345_678, "your-mt5-password", None);

    // 2) wrap in the high-level API, with a default symbol
    let mut sugar = MT5Sugar::new(MT5Service::new(account)).with_default_symbol("EURUSD");

    // 3) connect by broker cluster name (timeout seconds)
    sugar.connect_by_server_name("MetaQuotes-Demo", 30).await?;

    // 4) read the account
    let info = sugar.get_account_info().await?;
    println!("#{}  balance {:.2} {}  equity {:.2}",
        info.login, info.balance, info.currency, info.equity);

    // 5) a quote
    println!("EURUSD bid/ask {:.5}/{:.5}", sugar.bid(None).await?, sugar.ask(None).await?);

    // 6) (optional, DEMO ONLY) a tiny risk-sized trade with a 20/40 pip bracket
    // let lots = sugar.calculate_position_size("EURUSD", 1.0, 20.0).await?;
    // let ticket = sugar.buy_market_with_pips(None, lots, Some(20.0), Some(40.0), "first", 1001).await?;
    // println!("opened #{ticket}");

    sugar.disconnect(false).await?;
    Ok(())
}
```

> Replace the login/password/server with your demo credentials. Keep the trade block commented until you're on a demo account you're happy to trade.

---

## Step 3 — Run

```bash
cargo run
```

Expected output (values vary):

```text
#12345678  balance 10000.00 USD  equity 10000.00
EURUSD bid/ask 1.08512/1.08531
```

If you see a connection error, re-check the server name, credentials and `MRPC_API_KEY`.

---

## 🚀 Next steps

- **[gRPC Streaming](GRPC_STREAM_MANAGEMENT.md)** — subscribe to live ticks and position P/L.
- **[Return Codes](RETURN_CODES_REFERENCE.md)** — interpret trade results (`10009` = done).
- **[MT5Sugar — Master Overview](../MT5Sugar/MT5Sugar.Master.Overview.md)** — all 58 high-level methods.
- **[Strategies](../Strategies/Strategies.Master.Overview.md)** — ready orchestrators (`cargo run --example strategies_demo`).
