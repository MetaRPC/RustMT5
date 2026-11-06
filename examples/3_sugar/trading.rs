//! HIGH-LEVEL · MT5Sugar trading.
//!
//! A full one-liner order lifecycle: pip-based entry → move the stop → close. ⚠️ Opens and closes
//! a 0.01-lot position — DEMO accounts only. Run:
//!   MT5_LOGIN=… MT5_PASSWORD=… MT5_SERVER=MetaQuotes-Demo cargo run --example sugar_trading

use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = metarpc_mt5::Settings::load()?;
    let mut sugar = MT5Sugar::new(MT5Service::new(MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone()))).with_default_symbol("EURUSD");
    sugar.connect_by_server_name(&cfg.server_name, 30).await?;

    // ── entry: BUY at market with a 50-pip stop and 100-pip target, sized in pips ──
    let ticket = sugar.buy_market_with_pips(Some("EURUSD"), 0.01, Some(50.0), Some(100.0), "sugar demo", 0).await?;
    println!("opened #{ticket}");
    tokio::time::sleep(Duration::from_millis(800)).await;

    // ── manage: tighten the stop to 40 pips (compute the absolute price via calculate_sltp) ──
    if let (Some(sl), _) = sugar.calculate_sltp("EURUSD", true, Some(40.0), None).await? {
        let ok = sugar.modify_position_sl(ticket, sl).await?;
        println!("moved SL -> {sl:.5} ({})", if ok { "ok" } else { "rejected" });
    }

    // ── exit: flatten the position ──
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("closed: {}", sugar.close_position(ticket).await?);

    sugar.disconnect(false).await?;
    Ok(())
}
