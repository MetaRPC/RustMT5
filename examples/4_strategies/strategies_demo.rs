//! STRATEGIES · orchestrators + presets.
//!
//! Runs the three ready-made orchestrators wired to presets. ⚠️ Opens and closes 0.01-lot
//! positions — DEMO accounts only. Run:
//!   MT5_LOGIN=… MT5_PASSWORD=… MT5_SERVER=MetaQuotes-Demo cargo run --example strategies_demo

use metarpc_mt5::strategies::{self, StrategyPreset};
use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = metarpc_mt5::Settings::load()?;
    let mut sugar = MT5Sugar::new(MT5Service::new(MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone()))).with_default_symbol("EURUSD");
    sugar.connect_by_server_name(&cfg.server_name, 30).await?;

    // ── presets catalog (WHAT × HOW MUCH) ──
    println!("risk presets:");
    println!("  conservative : 0.5% / SL25 / TP50");
    println!("  balanced     : 1.0% / SL20 / TP40 + trail 15 + BE 10/+2");
    println!("  aggressive   : 2.0% / SL15 / TP45 + trail 10 + BE 8/+1\n");

    // One strategy preset reused across all three orchestrators.
    let sp = StrategyPreset::new("EURUSD").lots(0.01).magic(101).comment("strategies-demo");

    // ── 1) Market One-Shot (balanced) ──
    strategies::run_market_one_shot(&sugar, &sp, &strategies::balanced()).await?;
    println!();

    // ── 2) Bracket + Trailing (aggressive), activate trailing after +10 pips ──
    strategies::run_bracket_trailing(&sugar, &sp, &strategies::aggressive(), 10.0).await?;
    println!();

    // ── 3) Pending Bracket (conservative), wait up to 15s for the fill ──
    strategies::run_pending_bracket(&sugar, &sp, &strategies::conservative(), 15).await?;

    sugar.disconnect(false).await?;
    Ok(())
}
