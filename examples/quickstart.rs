use metarpc_mt5::{MT5Client, OrderRequest, TradeAction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("MRPC_API_KEY").unwrap_or_else(|_| "YOUR_API_KEY_HERE".to_string());
    let mut client = MT5Client::with_api_key("mt5.mrpc.pro", 443, api_key);

    println!("Connecting to MT5 (mt5.mrpc.pro:443)...");
    client.connect(2005432, "demo_password").await?;
    println!("Connected successfully! Account ID: {}", client.id.as_deref().unwrap_or(""));

    println!("\nStep 1: Querying Account Balance...");
    let acc = client.get_account_info().await?;
    println!("Account: {} ({})", acc.login, acc.name);
    println!("Account Balance: {} {}", acc.balance, acc.currency);

    println!("\nStep 2: Executing Market Order...");
    let req = OrderRequest {
        symbol: "EURUSD".to_string(),
        action: TradeAction::Buy,
        volume: 0.1,
        price: Some(1.0850),
        stop_loss: Some(1.0800),
        take_profit: Some(1.0900),
        slippage: Some(5),
        comment: Some("Rust MT5 Bot".to_string()),
    };

    let res = client.order_send(req).await?;
    println!("Order executed! Deal: #{} Ticket: #{}", res.deal, res.ticket);

    client.disconnect().await;
    println!("\nDisconnected.");
    Ok(())
}
