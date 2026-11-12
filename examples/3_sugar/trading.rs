/*══════════════════════════════════════════════════════════════════════════════
 FILE: examples/3_sugar/trading.rs - HIGH-LEVEL (MT5Sugar) TRADING DEMO
 PURPOSE:
   A full one-liner order lifecycle with MT5Sugar: pip-based market entry,
   move the stop, then flatten the position.

 🎯 WHO SHOULD USE THIS:
   • Developers who want quick, readable order execution
   • Learning the open -> modify -> close flow with smart defaults
   • Prototyping trading logic without protobuf boilerplate

 📚 WHAT THIS DEMO COVERS (4 sections):

   1. CONNECT
      • Settings::load() + MT5Sugar::connect_by_server_name()

   2. ENTRY
      • buy_market_with_pips()   - BUY at market with pip SL/TP, returns ticket

   3. MANAGE
      • calculate_sltp()         - absolute SL price from a pip distance
      • modify_position_sl()     - tighten the stop

   4. EXIT
      • close_position()         - flatten the position

 🔄 API LEVELS:
   LOW-LEVEL  (MT5Account) - raw gRPC/protobuf
   MID-LEVEL  (MT5Service) - typed wrappers
   HIGH-LEVEL (MT5Sugar)   - THIS FILE: one-liner trading with smart defaults

 ⚠️  WARNING: sends real orders on a demo account! Opens and closes one
     0.01-lot position (self-cleaning). DEMO accounts only.

 USAGE:
   cargo run --example sugar_trading
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

    // region: ENTRY
    // ══════════════════════════════════════════════════════════════
    // 2. ENTRY
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // buy_market_with_pips()
    //   BUY at market with a 50-pip stop and 100-pip target, sized in
    //   pips. Returns the position ticket.  ⚠️ sends a real order.
    // ──────────────────────────────────────────────────────────────
    let ticket = sugar.buy_market_with_pips(Some("EURUSD"), 0.01, Some(50.0), Some(100.0), "sugar demo", 0).await?;
    println!("opened #{ticket}");
    tokio::time::sleep(Duration::from_millis(800)).await;
    // endregion: ENTRY

    // region: MANAGE
    // ══════════════════════════════════════════════════════════════
    // 3. MANAGE
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // calculate_sltp() + modify_position_sl()
    //   Tighten the stop to 40 pips: compute the absolute price, then
    //   move the position's stop-loss to it.
    // ──────────────────────────────────────────────────────────────
    if let (Some(sl), _) = sugar.calculate_sltp("EURUSD", true, Some(40.0), None).await? {
        let ok = sugar.modify_position_sl(ticket, sl).await?;
        println!("moved SL -> {sl:.5} ({})", if ok { "ok" } else { "rejected" });
    }
    // endregion: MANAGE

    // region: EXIT
    // ══════════════════════════════════════════════════════════════
    // 4. EXIT
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // close_position()
    //   Flatten the position.  ⚠️ closes the real order opened above.
    // ──────────────────────────────────────────────────────────────
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("closed: {}", sugar.close_position(ticket).await?);
    // endregion: EXIT

    sugar.disconnect(false).await?;
    Ok(())
}
