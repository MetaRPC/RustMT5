/*══════════════════════════════════════════════════════════════════════════════
 FILE: examples/1_lowlevel/streaming.rs - LOW-LEVEL MT5 STREAMING DEMO
 PURPOSE:
   Exercise every MT5Account server stream. Each on_* returns an
   impl Stream<Item = Result<...Data>>; we pull a few items (with timeouts so the
   demo never blocks). Read-only - no orders are placed.

 🎯 WHO SHOULD USE THIS:
   • Developers learning the low-level MT5 streaming API
   • Building live tick / P&L / trade-event monitors
   • Seeing the exact shape of each stream item

 📚 WHAT THIS DEMO COVERS (3 sections):

   1. CONNECT
      • Settings::load() + MT5Account::connect_by_server_name()

   2. TICK STREAM
      • on_symbol_tick()   - live Bid/Ask ticks (take up to 3, then stop)

   3. EVENT & TIMER STREAMS
      • on_trade()                               - trade events (orders/positions)
      • on_position_profit()                     - per-position floating P/L (1s timer)
      • on_positions_and_pending_orders_tickets()- live ticket lists (1s timer)
      • on_trade_transaction()                   - the MT5 OnTradeTransaction feed

 🔄 API LEVELS:
   LOW-LEVEL  (MT5Account) - THIS FILE: raw async streams
   MID-LEVEL  (MT5Service) - typed streaming wrappers
   HIGH-LEVEL (MT5Sugar)   - callback-style watch helpers

 ℹ️  SAFE: read-only streaming, no orders are placed.

 USAGE:
   cargo run --example lowlevel_streaming
   (reads creds from examples/0_common/settings.json or the MT5_* env vars)
══════════════════════════════════════════════════════════════════════════════*/

use metarpc_mt5::proto::mt5_term_api as pb;
use metarpc_mt5::MT5Account;
use std::time::Duration;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // region: CONNECT
    // ══════════════════════════════════════════════════════════════
    // 1. CONNECT
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // Settings::load() + connect_by_server_name()
    //   Load credentials and connect the raw MT5Account.
    // ──────────────────────────────────────────────────────────────
    let cfg = metarpc_mt5::Settings::load()?;
    let mut account = MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone());
    account.connect_by_server_name(&cfg.server_name, 30).await?;
    let sym = "EURUSD";
    // endregion: CONNECT

    // region: TICK STREAM
    // ══════════════════════════════════════════════════════════════
    // 2. TICK STREAM
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // on_symbol_tick()
    //   Live Bid/Ask ticks; take up to 3 ticks (15s cap), then stop.
    // ──────────────────────────────────────────────────────────────
    println!("── on_symbol_tick (3 ticks / 15s) ──");
    let mut ticks = account.on_symbol_tick(pb::OnSymbolTickRequest { symbol_names: vec![sym.into()] }).await?;
    let mut n = 0;
    while let Ok(Some(item)) = tokio::time::timeout(Duration::from_secs(15), ticks.next()).await {
        match item {
            Ok(d) => println!("  tick: {d:?}"),
            Err(e) => {
                eprintln!("  stream error: {e}");
                break;
            }
        }
        n += 1;
        if n >= 3 {
            break;
        }
    }
    drop(ticks);
    // endregion: TICK STREAM

    // region: EVENT & TIMER STREAMS
    // ══════════════════════════════════════════════════════════════
    // 3. EVENT & TIMER STREAMS (one item each, via pull_one())
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // on_trade()
    //   Trade events (new/updated/removed orders & positions).
    // ──────────────────────────────────────────────────────────────
    pull_one("on_trade", account.on_trade(pb::OnTradeRequest::default()).await?).await;

    // ──────────────────────────────────────────────────────────────
    // on_position_profit()
    //   Per-position floating P/L on a 1s timer.
    // ──────────────────────────────────────────────────────────────
    pull_one(
        "on_position_profit",
        account
            .on_position_profit(pb::OnPositionProfitRequest { timer_period_milliseconds: 1000, ignore_empty_data: false })
            .await?,
    )
    .await;

    // ──────────────────────────────────────────────────────────────
    // on_positions_and_pending_orders_tickets()
    //   Live ticket lists on a 1s timer.
    // ──────────────────────────────────────────────────────────────
    pull_one(
        "on_positions_and_pending_orders_tickets",
        account
            .on_positions_and_pending_orders_tickets(pb::OnPositionsAndPendingOrdersTicketsRequest { timer_period_milliseconds: 1000 })
            .await?,
    )
    .await;

    // ──────────────────────────────────────────────────────────────
    // on_trade_transaction()
    //   The MT5 OnTradeTransaction feed.
    // ──────────────────────────────────────────────────────────────
    pull_one("on_trade_transaction", account.on_trade_transaction(pb::OnTradeTransactionRequest::default()).await?).await;
    // endregion: EVENT & TIMER STREAMS

    account.disconnect().await?;
    Ok(())
}

/// Pull a single item from a stream with a short timeout (so a quiet feed doesn't block the demo).
async fn pull_one<T: std::fmt::Debug, S>(name: &str, mut stream: S)
where
    S: tokio_stream::Stream<Item = Result<T, metarpc_mt5::MT5Error>> + Unpin,
{
    print!("── {name}: ");
    match tokio::time::timeout(Duration::from_secs(4), stream.next()).await {
        Ok(Some(Ok(d))) => println!("{d:?}"),
        Ok(Some(Err(e))) => println!("error: {e}"),
        Ok(None) => println!("(stream ended)"),
        Err(_) => println!("(no data in 4s)"),
    }
}
