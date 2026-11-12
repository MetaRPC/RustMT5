/*══════════════════════════════════════════════════════════════════════════════
 FILE: examples/4_strategies/strategies_demo.rs - MT5 STRATEGY ORCHESTRATORS DEMO
 PURPOSE:
   Run the three ready-made strategy orchestrators wired to risk presets, all on
   top of MT5Sugar. Each one opens and manages a small 0.01-lot position.

 🎯 WHO SHOULD USE THIS:
   • Developers exploring the built-in strategy orchestrators
   • Seeing how a StrategyPreset + a risk preset drive an orchestrator
   • A starting point for your own automated strategies

 📚 WHAT THIS DEMO COVERS:

   PRESETS
      • conservative() / balanced() / aggressive() - risk presets (WHAT x HOW MUCH)
      • StrategyPreset::new(...) - symbol / lots / magic / comment

   ORCHESTRATORS
      • run_market_one_shot()  - instant market entry (balanced)
      • run_bracket_trailing() - entry + trailing after +10 pips (aggressive)
      • run_pending_bracket()  - pending entry, wait up to 15s for a fill (conservative)

 🔄 API LEVELS:
   This demo drives the HIGH-LEVEL MT5Sugar via the strategies module, which sits
   above Sugar -> Service -> Account.

 ⚠️  WARNING: sends real orders - opens/closes 0.01-lot positions on your account.
   DEMO accounts only.

 USAGE:
   cargo run --example strategies_demo
   (reads creds from examples/0_common/settings.json or the MT5_* env vars)
══════════════════════════════════════════════════════════════════════════════*/

use metarpc_mt5::strategies::{self, StrategyPreset};
use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // region: CONNECT
    // ══════════════════════════════════════════════════════════════
    // 1. CONNECT
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // Settings::load() + Sugar.connect_by_server_name()
    //   Build Account -> Service -> Sugar and connect to the broker.
    // ──────────────────────────────────────────────────────────────
    let cfg = metarpc_mt5::Settings::load()?;
    let mut sugar = MT5Sugar::new(MT5Service::new(MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone()))).with_default_symbol("EURUSD");
    sugar.connect_by_server_name(&cfg.server_name, 30).await?;
    // endregion: CONNECT

    // region: PRESETS
    // ══════════════════════════════════════════════════════════════
    // 2. PRESETS (WHAT x HOW MUCH)
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // risk presets catalog
    //   conservative / balanced / aggressive - risk %, SL/TP, trailing, BE.
    // ──────────────────────────────────────────────────────────────
    println!("risk presets:");
    println!("  conservative : 0.5% / SL25 / TP50");
    println!("  balanced     : 1.0% / SL20 / TP40 + trail 15 + BE 10/+2");
    println!("  aggressive   : 2.0% / SL15 / TP45 + trail 10 + BE 8/+1\n");

    // ──────────────────────────────────────────────────────────────
    // StrategyPreset::new()
    //   One strategy preset (symbol/lots/magic/comment) reused below.
    // ──────────────────────────────────────────────────────────────
    // One strategy preset reused across all three orchestrators.
    let sp = StrategyPreset::new("EURUSD").lots(0.01).magic(101).comment("strategies-demo");
    // endregion: PRESETS

    // region: ORCHESTRATORS
    // ══════════════════════════════════════════════════════════════
    // 3. ORCHESTRATORS (each opens/manages a 0.01-lot position)
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // run_market_one_shot() (balanced)
    //   Instant market entry with the balanced risk preset.
    // ──────────────────────────────────────────────────────────────
    strategies::run_market_one_shot(&sugar, &sp, &strategies::balanced()).await?;
    println!();

    // ──────────────────────────────────────────────────────────────
    // run_bracket_trailing() (aggressive)
    //   Market entry; activate the trailing stop after +10 pips.
    // ──────────────────────────────────────────────────────────────
    strategies::run_bracket_trailing(&sugar, &sp, &strategies::aggressive(), 10.0).await?;
    println!();

    // ──────────────────────────────────────────────────────────────
    // run_pending_bracket() (conservative)
    //   Pending entry with a bracket; wait up to 15s for the fill.
    // ──────────────────────────────────────────────────────────────
    strategies::run_pending_bracket(&sugar, &sp, &strategies::conservative(), 15).await?;
    // endregion: ORCHESTRATORS

    sugar.disconnect(false).await?;
    Ok(())
}
