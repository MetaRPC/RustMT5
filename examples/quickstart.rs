/*══════════════════════════════════════════════════════════════════════════════
 FILE: examples/quickstart.rs - MT5 QUICKSTART (CONNECT & READ)
 PURPOSE:
   The smallest end-to-end example: build the three API layers, connect to MT5
   via the gateway, read the account and a quote, then disconnect. No trades.

 🎯 WHO SHOULD USE THIS:
   • First-time users verifying their credentials and connection
   • A copy-paste starting point for a new program

 📚 WHAT THIS DEMO COVERS:

   1. BUILD & CONNECT
      • MT5Account -> MT5Service -> MT5Sugar, then connect_by_server_name()

   2. READ
      • get_account_info()        - login / company / currency / balance / equity
      • bid() / ask() / spread()  - current EURUSD quote
      • count_open_positions()    - number of open positions

 🔄 API LEVELS:
   LOW-LEVEL  (MT5Account) - raw gRPC client (base layer)
   MID-LEVEL  (MT5Service) - typed facade
   HIGH-LEVEL (MT5Sugar)   - THIS FILE: one-liner helpers

 ℹ️  SAFE: read-only - no orders are sent.

 USAGE:
   cargo run --example quickstart
   (reads creds from examples/0_common/settings.json or the MT5_* env vars)
══════════════════════════════════════════════════════════════════════════════*/

use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // region: BUILD & CONNECT
    // ══════════════════════════════════════════════════════════════
    // 1. BUILD & CONNECT
    // ══════════════════════════════════════════════════════════════

    let cfg = metarpc_mt5::Settings::load()?;

    // ──────────────────────────────────────────────────────────────
    // Build the three layers: Account (low-level) -> Service (facade) -> Sugar.
    // ──────────────────────────────────────────────────────────────
    let account = MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone()); // None -> mt5.mrpc.pro:443
    let service = MT5Service::new(account);
    let mut sugar = MT5Sugar::new(service).with_default_symbol("EURUSD");

    // ──────────────────────────────────────────────────────────────
    // connect_by_server_name()
    //   Connect by broker server name; the gateway assigns the terminal id.
    // ──────────────────────────────────────────────────────────────
    sugar.connect_by_server_name(&cfg.server_name, 30).await?;
    // endregion: BUILD & CONNECT

    // region: READ
    // ══════════════════════════════════════════════════════════════
    // 2. READ (account + quote)
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // get_account_info()
    //   Login / company / currency / balance / equity.
    // ──────────────────────────────────────────────────────────────
    let info = sugar.get_account_info().await?;
    println!(
        "Login {} | {} {} | balance {:.2} | equity {:.2}",
        info.login, info.company, info.currency, info.balance, info.equity
    );

    // ──────────────────────────────────────────────────────────────
    // bid() / ask() / spread()
    //   Current EURUSD quote.
    // ──────────────────────────────────────────────────────────────
    println!(
        "EURUSD  bid/ask: {:.5} / {:.5}  (spread {:.5})",
        sugar.bid(None).await?,
        sugar.ask(None).await?,
        sugar.spread(None).await?
    );

    // ──────────────────────────────────────────────────────────────
    // count_open_positions()
    //   Number of currently open positions.
    // ──────────────────────────────────────────────────────────────
    println!("open positions: {}", sugar.count_open_positions(None).await?);
    // endregion: READ

    sugar.disconnect(false).await?;
    Ok(())
}
