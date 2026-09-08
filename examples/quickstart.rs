use metarpc_mt5::{MT5Client, OrderRequest, TradeAction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MT5Client::new("mt5.broker.com", 443);

    println!("Connecting to MT5...");
    client.connect(2005432, "password").await?;
    println!("Connected!");

    let acc = client.get_account_info().await?;
    println!("Account Balance: {} {}", acc.balance, acc.currency);

    let req = OrderRequest {
        symbol: "EURUSD".to_string(),
        action: TradeAction::Buy,
        volume: 0.1,
        price: None,
        stop_loss: Some(1.0800),
        take_profit: Some(1.0950),
        slippage: Some(10),
        comment: Some("Rust MT5 Bot".to_string()),
    };

    let res = client.order_send(req).await?;
    println!("Order executed! Deal: #{} Ticket: #{}", res.deal, res.ticket);

    Ok(())
}
