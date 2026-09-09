# Getting Started with RustMT5

## Prerequisites
- **Rust 1.70+**
- Cargo package manager

## Minimal Example

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


> **Authentication Note**: Connecting to `mt5.mrpc.pro:443` requires a valid MetaRPC API key. Register for free at [https://mrpc.pro/signup](https://mrpc.pro/signup) and generate your token in [https://mrpc.pro/my](https://mrpc.pro/my).



---

## 🆔 Automatic Account ID & Authentication

MetaRPC endpoints require two credentials for all terminal operations:
1. **`APIKey`**: Your personal authentication token from [https://mrpc.pro/my](https://mrpc.pro/my) (obtained by registering at [https://mrpc.pro/signup](https://mrpc.pro/signup)). Sent in the `APIKey` header.
2. **`id`**: A deterministic account GUID derived from your MetaTrader `user` (login number) and `password`.

> 💡 **Seamless Automation**: You do not need to call `GetId` manually. The SDK automatically derives your deterministic account ID from your credentials upon initialization and attaches both the `id` and `APIKey` headers to all requests and streaming subscriptions.

