/*══════════════════════════════════════════════════════════════════════════════
 FILE: examples/3_sugar/basics.rs - HIGH-LEVEL (MT5Sugar) BASICS DEMO
 PURPOSE:
   Read-only one-liners off MT5Sugar: balances, prices, symbol info and risk
   math. No orders are sent - pure reads and calculations.

 🎯 WHO SHOULD USE THIS:
   • Developers starting with the high-level MT5Sugar API
   • Building dashboards / risk calculators without boilerplate
   • Learning the sugar helpers before placing any trade

 📚 WHAT THIS DEMO COVERS (4 sections):

   1. CONNECT
      • Settings::load() + MT5Sugar::connect_by_server_name()

   2. BALANCES & PRICES
      • balance/equity/margin/free_margin/margin_level/floating_profit()
      • bid/ask/spread()                 - default symbol when None

   3. SYMBOL INFO
      • get_symbol_info() / get_symbol_digits() / get_min_stop_level()
      • is_symbol_available() / get_all_symbols()

   4. RISK & SIZING
      • calculate_position_size()        - lots for a % risk and stop
      • calculate_required_margin() / get_max_lot_size()
      • calculate_sltp()                 - absolute SL/TP from pips

 🔄 API LEVELS:
   LOW-LEVEL  (MT5Account) - raw gRPC/protobuf
   MID-LEVEL  (MT5Service) - typed wrappers
   HIGH-LEVEL (MT5Sugar)   - THIS FILE: one-liner helpers with smart defaults

 ℹ️  SAFE: read-only - never sends an order.

 USAGE:
   cargo run --example sugar_basics
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

    // region: BALANCES & PRICES
    // ══════════════════════════════════════════════════════════════
    // 2. BALANCES & PRICES
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // balance/equity/margin/free_margin/margin_level/floating_profit()
    //   Each resolves one account money figure in a single call.
    // ──────────────────────────────────────────────────────────────
    println!(
        "balance {:.2}  equity {:.2}  margin {:.2}  free {:.2}  level {:.1}%  floating {:.2}",
        sugar.balance().await?,
        sugar.equity().await?,
        sugar.margin().await?,
        sugar.free_margin().await?,
        sugar.margin_level().await?,
        sugar.floating_profit().await?,
    );

    // ──────────────────────────────────────────────────────────────
    // bid() / ask() / spread()
    //   Current prices; the default symbol is used when None is passed.
    // ──────────────────────────────────────────────────────────────
    println!("bid/ask/spread {:.5}/{:.5}/{:.5}", sugar.bid(None).await?, sugar.ask(None).await?, sugar.spread(None).await?);
    // endregion: BALANCES & PRICES

    // region: SYMBOL INFO
    // ══════════════════════════════════════════════════════════════
    // 3. SYMBOL INFO
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // get_symbol_info() + per-field helpers
    //   One compact snapshot, then individual digits / min-stop /
    //   availability / full symbol list helpers.
    // ──────────────────────────────────────────────────────────────
    let si = sugar.get_symbol_info("EURUSD").await?;
    println!("{} point {} digits {} vol[min/max/step] {}/{}/{}", si.name, si.point, si.digits, si.volume_min, si.volume_max, si.volume_step);
    println!(
        "digits {}  min_stop {}  available {}  all_symbols {}",
        sugar.get_symbol_digits("EURUSD").await?,
        sugar.get_min_stop_level("EURUSD").await?,
        sugar.is_symbol_available("EURUSD").await?,
        sugar.get_all_symbols().await?.len(),
    );
    // endregion: SYMBOL INFO

    // region: RISK & SIZING
    // ══════════════════════════════════════════════════════════════
    // 4. RISK & SIZING
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // calculate_position_size() / calculate_required_margin()
    // get_max_lot_size() / calculate_sltp()
    //   Position sizing for a % risk + stop, margin for a lot, the max
    //   lot, and absolute SL/TP prices derived from pip distances.
    // ──────────────────────────────────────────────────────────────
    println!("position size (1% risk, 50-pip stop): {:.2} lot", sugar.calculate_position_size("EURUSD", 1.0, 50.0).await?);
    println!("required margin for 0.1 lot: {:.2}", sugar.calculate_required_margin("EURUSD", 0.1, None).await?);
    println!("max lot: {}", sugar.get_max_lot_size("EURUSD").await?);
    let (sl, tp) = sugar.calculate_sltp("EURUSD", true, Some(50.0), Some(100.0)).await?;
    println!("BUY 50/100 pips -> sl={sl:?} tp={tp:?}");
    // endregion: RISK & SIZING

    sugar.disconnect(false).await?;
    Ok(())
}
