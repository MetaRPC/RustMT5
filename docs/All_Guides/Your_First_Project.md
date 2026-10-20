# Your First Project in 10 Minutes (Rust)

> **Hands-on Quick Start** - Create a working trading project with MetaTrader 5 and RustMT5 from scratch.

---

## Step 0: Obtain Your API Key

To connect to MetaRPC endpoints (`mt5.mrpc.pro:443`), obtain your API key:
1. Register for free at [https://mrpc.pro/signup](https://mrpc.pro/signup).
2. Generate your API token in your dashboard at [https://mrpc.pro/my](https://mrpc.pro/my).
3. Set your token in your environment or connection config.

---

## Step 1: Create Your Project

Create a new directory for your trading bot:

```bash
mkdir my_rustmt5_bot
cd my_rustmt5_bot
```

Get the SDK (it is not on a package registry):

```bash
git clone https://github.com/MetaRPC/RustMT5.git
```

---

## Step 2: Write Your Trading Code

Create your main application file and paste the following snippet:

```rust
use metarpc_mt5::{MT5Client, OrderRequest, TradeAction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // API key from the MRPC_API_KEY environment variable; the terminal id is derived
    // from a label so every run reuses the same server-side terminal.
    let client = MT5Client::new("mt5.mrpc.pro", 443)
        .with_id(MT5Client::stable_terminal_id("my-first-bot"));

    // Login, password and the broker server name (as shown in the MT5 terminal).
    client.connect_ex(2005432, "account_pass", "FBS-Demo", Some("my-first-bot"), 120).await?;
    println!("Connected to MT5!");

    let acc = client.get_account_info().await?;
    println!("Account Balance: {} {}", acc.balance, acc.currency);

    // Send 0.01 lot Buy at market
    let req = OrderRequest::market("EURUSD", TradeAction::Buy, 0.01)
        .with_comment("Rust Quickstart");

    let result = client.order_send(req).await?;
    println!("Order sent! Deal #{} Ticket #{}", result.deal, result.order);

    Ok(())
}
```

---

## Step 3: Run the Program

Run your application:

```bash
cargo run --example quickstart
```

Run it from the cloned repository: its `examples/quickstart` does the same thing.

---

## 🚀 Next Steps

Congratulations! You have successfully established a direct gRPC connection to MetaTrader 5. Next:
- Explore **[gRPC Streaming](GRPC_STREAM_MANAGEMENT.md)** to listen to live ticks.
- Check the **[API Reference](../API_Reference/MT5Account.md)** for all 40+ available terminal methods.
- Learn about high-level risk management and auto-normalization in **[MT5Sugar](../API_Reference/MT5Sugar.md)**.
