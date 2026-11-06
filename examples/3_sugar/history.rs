//! HIGH-LEVEL · MT5Sugar history & stats.
//!
//! Account snapshot, realized profit by period, deals, and a daily summary (read-only). Run:
//!   MT5_LOGIN=… MT5_PASSWORD=… MT5_SERVER=MetaQuotes-Demo cargo run --example sugar_history

use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = metarpc_mt5::Settings::load()?;
    let mut sugar = MT5Sugar::new(MT5Service::new(MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone()))).with_default_symbol("EURUSD");
    sugar.connect_by_server_name(&cfg.server_name, 30).await?;

    // ── account snapshot ──
    let info = sugar.get_account_info().await?;
    println!("account #{}  {} {}  balance {:.2}  equity {:.2}", info.login, info.company, info.currency, info.balance, info.equity);

    // ── realized profit by period (closed positions) ──
    println!(
        "profit  today {:.2}  this week {:.2}  this month {:.2}",
        sugar.get_profit_today().await?,
        sugar.get_profit_this_week().await?,
        sugar.get_profit_this_month().await?,
    );

    // ── deals (closed positions) today ──
    let deals = sugar.get_deals_today().await?;
    println!("deals today: {}", deals.len());
    for d in deals.iter().take(5) {
        println!("  {} {:.2} lot  profit {:.2}  swap {:.2}", d.symbol, d.volume, d.profit, d.swap);
    }

    // ── a compact daily summary ──
    let st = sugar.get_daily_stats().await?;
    println!("daily: {} deals, profit {:.2}, {} wins / {} losses", st.deals, st.profit, st.wins, st.losses);

    sugar.disconnect(false).await?;
    Ok(())
}
