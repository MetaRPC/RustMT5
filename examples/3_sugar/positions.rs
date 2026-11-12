/*══════════════════════════════════════════════════════════════════════════════
 FILE: examples/3_sugar/positions.rs - HIGH-LEVEL (MT5Sugar) POSITIONS DEMO
 PURPOSE:
   Open one position, exercise every MT5Sugar position query, then flatten
   everything on the symbol.

 🎯 WHO SHOULD USE THIS:
   • Developers building position monitors / risk views
   • Learning the sugar position-query helpers against a live position
   • Prototyping close-all / exposure logic

 📚 WHAT THIS DEMO COVERS (4 sections):

   1. CONNECT
      • Settings::load() + MT5Sugar::connect_by_server_name()

   2. OPEN
      • buy_market()             - open a 0.01 position so queries have data

   3. QUERIES
      • count_open_positions() / has_open_position()
      • get_total_profit() / get_profit_by_symbol()
      • get_open_positions()     - iterate open positions

   4. FLATTEN
      • close_all_positions()    - close everything on the symbol

 🔄 API LEVELS:
   LOW-LEVEL  (MT5Account) - raw gRPC/protobuf
   MID-LEVEL  (MT5Service) - typed wrappers
   HIGH-LEVEL (MT5Sugar)   - THIS FILE: one-liner position helpers

 ⚠️  WARNING: sends real orders on a demo account! Opens and closes one
     0.01-lot position (self-cleaning). DEMO accounts only.

 USAGE:
   cargo run --example sugar_positions
   (reads creds from examples/0_common/settings.json or the MT5_* env vars)
══════════════════════════════════════════════════════════════════════════════*/

use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};
use std::time::Duration;

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

    // region: OPEN
    // ══════════════════════════════════════════════════════════════
    // 2. OPEN
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // buy_market()
    //   Open one position so the queries below have something to show.
    //   ⚠️ sends a real order (closed at the end).
    // ──────────────────────────────────────────────────────────────
    let ticket = sugar.buy_market(Some("EURUSD"), 0.01, "pos demo", 0).await?;
    println!("opened #{ticket}");
    tokio::time::sleep(Duration::from_millis(800)).await;
    // endregion: OPEN

    // region: QUERIES
    // ══════════════════════════════════════════════════════════════
    // 3. QUERIES
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // count_open_positions() / has_open_position() / get_total_profit()
    // get_profit_by_symbol() / get_open_positions()
    //   Count, existence, total & per-symbol floating P/L, and a full
    //   listing of the open positions.
    // ──────────────────────────────────────────────────────────────
    println!("open positions: {}", sugar.count_open_positions(None).await?);
    println!("has EURUSD position: {}", sugar.has_open_position(Some("EURUSD")).await?);
    println!("total floating profit: {:.2}", sugar.get_total_profit().await?);
    println!("EURUSD profit: {:.2}", sugar.get_profit_by_symbol("EURUSD").await?);
    for p in sugar.get_open_positions().await? {
        println!("  #{} {} {:.2} lot  P/L {:.2}", p.ticket, p.symbol, p.volume, p.profit);
    }
    // endregion: QUERIES

    // region: FLATTEN
    // ══════════════════════════════════════════════════════════════
    // 4. FLATTEN
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // close_all_positions()
    //   Flatten everything on the symbol.  ⚠️ closes real positions.
    // ──────────────────────────────────────────────────────────────
    let closed = sugar.close_all_positions(Some("EURUSD")).await?;
    println!("closed {closed} position(s)");
    // endregion: FLATTEN

    sugar.disconnect(false).await?;
    Ok(())
}
