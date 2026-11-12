/*══════════════════════════════════════════════════════════════════════════════
 FILE: examples/3_sugar/history.rs - HIGH-LEVEL (MT5Sugar) HISTORY & STATS DEMO
 PURPOSE:
   Read-only account history off MT5Sugar: account snapshot, realized profit by
   period, recent deals, and a compact daily summary. No orders are sent.

 🎯 WHO SHOULD USE THIS:
   • Developers building reporting / performance views
   • Learning the sugar history & stats helpers
   • Pulling realized P/L without protobuf boilerplate

 📚 WHAT THIS DEMO COVERS (4 sections):

   1. CONNECT
      • Settings::load() + MT5Sugar::connect_by_server_name()

   2. ACCOUNT SNAPSHOT
      • get_account_info()       - login / company / currency / balance / equity

   3. REALIZED PROFIT
      • get_profit_today() / get_profit_this_week() / get_profit_this_month()

   4. DEALS & DAILY SUMMARY
      • get_deals_today()        - closed positions today
      • get_daily_stats()        - deals, profit, wins / losses

 🔄 API LEVELS:
   LOW-LEVEL  (MT5Account) - raw gRPC/protobuf
   MID-LEVEL  (MT5Service) - typed wrappers
   HIGH-LEVEL (MT5Sugar)   - THIS FILE: one-liner history & stats helpers

 ℹ️  SAFE: read-only - never sends an order.

 USAGE:
   cargo run --example sugar_history
   (reads creds from examples/0_common/settings.json or the MT5_* env vars)
══════════════════════════════════════════════════════════════════════════════*/

use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // region: CONNECT
    // ══════════════════════════════════════════════════════════════
    // 1. CONNECT
    // ══════════════════════════════════════════════════════════════
    let cfg = metarpc_mt5::Settings::load()?;
    let mut sugar = MT5Sugar::new(MT5Service::new(MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone()))).with_default_symbol("EURUSD");
    sugar.connect_by_server_name(&cfg.server_name, 30).await?;
    // endregion: CONNECT

    // region: ACCOUNT SNAPSHOT
    // ══════════════════════════════════════════════════════════════
    // 2. ACCOUNT SNAPSHOT
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // get_account_info()
    //   One struct with login / company / currency / balance / equity.
    // ──────────────────────────────────────────────────────────────
    let info = sugar.get_account_info().await?;
    println!("account #{}  {} {}  balance {:.2}  equity {:.2}", info.login, info.company, info.currency, info.balance, info.equity);
    // endregion: ACCOUNT SNAPSHOT

    // region: REALIZED PROFIT
    // ══════════════════════════════════════════════════════════════
    // 3. REALIZED PROFIT (closed positions)
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // get_profit_today() / get_profit_this_week() / get_profit_this_month()
    //   Realized profit aggregated over each period.
    // ──────────────────────────────────────────────────────────────
    println!(
        "profit  today {:.2}  this week {:.2}  this month {:.2}",
        sugar.get_profit_today().await?,
        sugar.get_profit_this_week().await?,
        sugar.get_profit_this_month().await?,
    );
    // endregion: REALIZED PROFIT

    // region: DEALS & DAILY SUMMARY
    // ══════════════════════════════════════════════════════════════
    // 4. DEALS & DAILY SUMMARY
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // get_deals_today()
    //   Closed positions (deals) today; first few are printed.
    // ──────────────────────────────────────────────────────────────
    let deals = sugar.get_deals_today().await?;
    println!("deals today: {}", deals.len());
    for d in deals.iter().take(5) {
        println!("  {} {:.2} lot  profit {:.2}  swap {:.2}", d.symbol, d.volume, d.profit, d.swap);
    }

    // ──────────────────────────────────────────────────────────────
    // get_daily_stats()
    //   Compact daily summary: deal count, profit, wins / losses.
    // ──────────────────────────────────────────────────────────────
    let st = sugar.get_daily_stats().await?;
    println!("daily: {} deals, profit {:.2}, {} wins / {} losses", st.deals, st.profit, st.wins, st.losses);
    // endregion: DEALS & DAILY SUMMARY

    sugar.disconnect(false).await?;
    Ok(())
}
