//! Quickstart: connect by server name, print the account, quote a symbol, pull bars, stream
//! ticks for 5 s. No orders are sent.
//!
//!   MRPC_API_KEY=... MT5_LOGIN=... MT5_PASSWORD=... MT5_SERVER=FBS-Demo cargo run --example quickstart

use std::time::Duration;

use metarpc_mt5::{timeframe, MT5Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::args()
        .nth(1)
        .or_else(|| std::env::var("MRPC_API_KEY").ok())
        .unwrap_or_else(|| "TRIAL".to_string());
    let login: u64 = std::env::var("MT5_LOGIN")?.parse()?;
    let password = std::env::var("MT5_PASSWORD")?;
    let server = std::env::var("MT5_SERVER")?;
    let symbol = std::env::var("MT5_SYMBOL").unwrap_or_else(|_| "EURUSD".to_string());

    // Stable terminal id: the same label always maps to the same terminal on the gateway.
    let client = MT5Client::with_api_key("mt5.mrpc.pro", 443, api_key)
        .with_id(MT5Client::stable_terminal_id("rustmt5-quickstart"));

    println!("Connecting to {server} as {login} via mt5.mrpc.pro:443 ...");
    let data = client.connect_ex(login, &password, &server, Some("rustmt5-quickstart"), 120).await?;
    println!("Connected. terminal id = {}", data.terminal_instance_guid);

    let acc = client.get_account_info().await?;
    println!(
        "Account {} ({}) balance={} equity={} {} leverage=1:{} mode={:?}",
        acc.login, acc.name, acc.balance, acc.equity, acc.currency, acc.leverage, acc.trade_mode
    );

    let q = client.ensure_symbol(&symbol, Duration::from_secs(15)).await?;
    println!("{symbol}: bid={} ask={} spread={:.6}", q.bid, q.ask, q.spread());

    let bars = client.price_history(&symbol, timeframe::M1, 5).await?;
    for b in &bars {
        println!("bar t={} o={} h={} l={} c={} v={}", b.time, b.open, b.high, b.low, b.close, b.tick_volume);
    }

    let mut rx = client.subscribe_quotes(vec![symbol.clone()]).await?;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    let mut n = 0;
    while let Ok(Some(t)) = tokio::time::timeout_at(deadline, rx.recv()).await {
        n += 1;
        if n <= 3 {
            println!("tick {} bid={} ask={}", t.symbol, t.bid, t.ask);
        }
    }
    println!("{n} ticks in 5 s");
    drop(rx); // unsubscribe
    Ok(())
}
