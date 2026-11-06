//! HIGH-LEVEL · MT5Sugar positions.
//!
//! Open a position, inspect the position queries, then flatten. ⚠️ Opens/closes a 0.01-lot
//! position — DEMO accounts only. Run:
//!   MT5_LOGIN=… MT5_PASSWORD=… MT5_SERVER=MetaQuotes-Demo cargo run --example sugar_positions

use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = metarpc_mt5::Settings::load()?;
    let mut sugar = MT5Sugar::new(MT5Service::new(MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone()))).with_default_symbol("EURUSD");
    sugar.connect_by_server_name(&cfg.server_name, 30).await?;

    // ── open one position so the queries have something to show ──
    let ticket = sugar.buy_market(Some("EURUSD"), 0.01, "pos demo", 0).await?;
    println!("opened #{ticket}");
    tokio::time::sleep(Duration::from_millis(800)).await;

    // ── queries ──
    println!("open positions: {}", sugar.count_open_positions(None).await?);
    println!("has EURUSD position: {}", sugar.has_open_position(Some("EURUSD")).await?);
    println!("total floating profit: {:.2}", sugar.get_total_profit().await?);
    println!("EURUSD profit: {:.2}", sugar.get_profit_by_symbol("EURUSD").await?);
    for p in sugar.get_open_positions().await? {
        println!("  #{} {} {:.2} lot  P/L {:.2}", p.ticket, p.symbol, p.volume, p.profit);
    }

    // ── flatten everything on the symbol ──
    let closed = sugar.close_all_positions(Some("EURUSD")).await?;
    println!("closed {closed} position(s)");

    sugar.disconnect(false).await?;
    Ok(())
}
