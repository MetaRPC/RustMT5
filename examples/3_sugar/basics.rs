//! HIGH-LEVEL · MT5Sugar basics.
//!
//! Read-only one-liners: balances, prices, symbol info and risk math (no orders). Run:
//!   MT5_LOGIN=… MT5_PASSWORD=… MT5_SERVER=MetaQuotes-Demo cargo run --example sugar_basics

use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = metarpc_mt5::Settings::load()?;
    let mut sugar = MT5Sugar::new(MT5Service::new(MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone()))).with_default_symbol("EURUSD");
    sugar.connect_by_server_name(&cfg.server_name, 30).await?;

    // ── balances: each via the account property enums ──
    println!(
        "balance {:.2}  equity {:.2}  margin {:.2}  free {:.2}  level {:.1}%  floating {:.2}",
        sugar.balance().await?,
        sugar.equity().await?,
        sugar.margin().await?,
        sugar.free_margin().await?,
        sugar.margin_level().await?,
        sugar.floating_profit().await?,
    );

    // ── prices: default symbol is used when None is passed ──
    println!("bid/ask/spread {:.5}/{:.5}/{:.5}", sugar.bid(None).await?, sugar.ask(None).await?, sugar.spread(None).await?);

    // ── symbol info: one compact snapshot + individual helpers ──
    let si = sugar.get_symbol_info("EURUSD").await?;
    println!("{} point {} digits {} vol[min/max/step] {}/{}/{}", si.name, si.point, si.digits, si.volume_min, si.volume_max, si.volume_step);
    println!(
        "digits {}  min_stop {}  available {}  all_symbols {}",
        sugar.get_symbol_digits("EURUSD").await?,
        sugar.get_min_stop_level("EURUSD").await?,
        sugar.is_symbol_available("EURUSD").await?,
        sugar.get_all_symbols().await?.len(),
    );

    // ── risk & sizing ──
    println!("position size (1% risk, 50-pip stop): {:.2} lot", sugar.calculate_position_size("EURUSD", 1.0, 50.0).await?);
    println!("required margin for 0.1 lot: {:.2}", sugar.calculate_required_margin("EURUSD", 0.1, None).await?);
    println!("max lot: {}", sugar.get_max_lot_size("EURUSD").await?);
    let (sl, tp) = sugar.calculate_sltp("EURUSD", true, Some(50.0), Some(100.0)).await?;
    println!("BUY 50/100 pips -> sl={sl:?} tp={tp:?}");

    sugar.disconnect(false).await?;
    Ok(())
}
