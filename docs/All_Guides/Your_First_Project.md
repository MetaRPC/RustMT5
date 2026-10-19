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
    let mut client = MT5Client::new("mt5.mrpc.pro", 443);

    client.connect(2005432, "account_pass").await?;
    println!("Connected to MT5!");

    let acc = client.get_account_info().await?;
    println!("Account Balance: {} {}", acc.balance, acc.currency);

    // Send 0.01 lot Buy
    let req = OrderRequest {
        symbol: "EURUSD".to_string(),
        action: TradeAction::Buy,
        volume: 0.01,
        price: None,
        stop_loss: None,
        take_profit: None,
        slippage: Some(10),
        comment: Some("Rust Quickstart".to_string()),
    };

    let result = client.order_send(req).await?;
    println!("Order sent! Deal #{} Ticket #{}", result.deal, result.ticket);

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
