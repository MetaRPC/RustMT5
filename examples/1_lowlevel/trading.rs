/*══════════════════════════════════════════════════════════════════════════════
 FILE: examples/1_lowlevel/trading.rs - LOW-LEVEL MT5 TRADING DEMO
 PURPOSE:
   Margin / profit calculators, a pre-trade order_check, then a real market
   BUY -> modify -> close round-trip on the raw MT5Account client.

 🎯 WHO SHOULD USE THIS:
   • Developers learning the low-level MT5 trading API
   • Building a custom order layer on top of MT5Account
   • Seeing the full open / modify / close lifecycle with raw protobuf

 📚 WHAT THIS DEMO COVERS (3 sections):

   1. CONNECT
      • Settings::load() + MT5Account::connect_by_server_name()

   2. CALCULATORS & VALIDATION
      • symbol_info_tick()   - current price (input for the calculators)
      • order_calc_margin()  - margin a BUY would require
      • order_calc_profit()  - profit if price moved +50 points
      • order_check()        - validate a trade request before sending

   3. TRADE ROUND-TRIP
      • order_send()         - open a market BUY
      • order_modify()       - attach SL/TP (200 points around entry)
      • order_close()        - flatten the position

 🔄 API LEVELS:
   LOW-LEVEL  (MT5Account) - THIS FILE: raw order_send / modify / close
   MID-LEVEL  (MT5Service) - typed trade helpers
   HIGH-LEVEL (MT5Sugar)   - one-liner buy_market / close helpers

 ⚠️  WARNING: places, modifies and closes a REAL 0.01-lot position - DEMO
     accounts only (opened and closed within the run).

 USAGE:
   cargo run --example lowlevel_trading
   (reads creds from examples/0_common/settings.json or the MT5_* env vars)
══════════════════════════════════════════════════════════════════════════════*/

use metarpc_mt5::proto::mt5_term_api as pb;
use metarpc_mt5::MT5Account;

const TRADE_RETCODE_DONE: u32 = 10009;

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
    let volume = 0.01;
    // endregion: CONNECT

    // region: CALCULATORS & VALIDATION
    // ══════════════════════════════════════════════════════════════
    // 2. CALCULATORS & VALIDATION
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // symbol_info_tick()
    //   Current price - needed for the calculators and the order price.
    // ──────────────────────────────────────────────────────────────
    let tick = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: sym.into() }).await?;
    println!("{sym} ask={:.5}", tick.ask);

    // ──────────────────────────────────────────────────────────────
    // order_calc_margin()
    //   Margin a BUY of `volume` at the current ask would require.
    // ──────────────────────────────────────────────────────────────
    let margin = account
        .order_calc_margin(pb::OrderCalcMarginRequest {
            symbol: sym.into(),
            order_type: pb::EnumOrderTypeTf::OrderTypeTfBuy as i32,
            volume,
            open_price: tick.ask,
        })
        .await?;
    println!("required margin: {:.2}", margin.margin);

    // ──────────────────────────────────────────────────────────────
    // order_calc_profit()
    //   Profit if price moved +50 points from entry.
    // ──────────────────────────────────────────────────────────────
    let profit = account
        .order_calc_profit(pb::OrderCalcProfitRequest {
            order_type: pb::EnumOrderTypeTf::OrderTypeTfBuy as i32,
            symbol: sym.into(),
            volume,
            open_price: tick.ask,
            close_price: tick.ask + 0.0050,
        })
        .await?;
    println!("profit for +50 pips: {:.2}", profit.profit);

    // ──────────────────────────────────────────────────────────────
    // order_check()
    //   Validate a trade request before sending it (free margin, etc.).
    // ──────────────────────────────────────────────────────────────
    let check = account
        .order_check(pb::OrderCheckRequest {
            mql_trade_request: Some(pb::MrpcMqlTradeRequest {
                action: pb::MrpcEnumTradeRequestActions::TradeActionDeal as i32,
                symbol: sym.into(),
                volume,
                price: tick.ask,
                order_type: pb::EnumOrderTypeTf::OrderTypeTfBuy as i32,
                // Market deals on MetaQuotes-Demo reject IOC here (INVALID_FILL / 10030);
                // Fill-Or-Kill is accepted for this symbol's execution mode.
                type_filling: pb::MrpcEnumOrderTypeFilling::OrderFillingFok as i32,
                type_time: pb::MrpcEnumOrderTypeTime::OrderTimeGtc as i32,
                ..Default::default()
            }),
        })
        .await?;
    if let Some(r) = &check.mql_trade_check_result {
        println!("order_check: code={} free_margin_after={:.2}", r.returned_code, r.free_margin);
    }
    // endregion: CALCULATORS & VALIDATION

    // region: TRADE ROUND-TRIP
    // ══════════════════════════════════════════════════════════════
    // 3. TRADE ROUND-TRIP (⚠️ real orders)
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // order_send()
    //   Open a market BUY; bail out cleanly if the server rejects it.
    // ──────────────────────────────────────────────────────────────
    let sent = account
        .order_send(pb::OrderSendRequest {
            symbol: sym.into(),
            operation: pb::Tmt5EnumOrderType::Tmt5OrderTypeBuy as i32,
            volume,
            price: Some(tick.ask),
            slippage: Some(10),
            ..Default::default()
        })
        .await?;
    if sent.returned_code != TRADE_RETCODE_DONE {
        println!("order rejected: code={} {}", sent.returned_code, sent.returned_code_description);
        account.disconnect().await?;
        return Ok(());
    }
    let ticket = sent.order;
    println!("opened #{ticket} @ {:.5}", sent.price);

    // ──────────────────────────────────────────────────────────────
    // order_modify()
    //   Attach SL/TP (200 points around entry) to the open position.
    // ──────────────────────────────────────────────────────────────
    let modified = account
        .order_modify(pb::OrderModifyRequest {
            ticket,
            stop_loss: Some(tick.ask - 0.0200),
            take_profit: Some(tick.ask + 0.0200),
            ..Default::default()
        })
        .await?;
    println!("modify SL/TP: code={}", modified.returned_code);

    // ──────────────────────────────────────────────────────────────
    // order_close()
    //   Flatten the position (volume 0.0 = close in full).
    // ──────────────────────────────────────────────────────────────
    let closed = account.order_close(pb::OrderCloseRequest { ticket, volume: 0.0, slippage: 10 }).await?;
    println!("closed #{ticket}: code={}", closed.returned_code);
    // endregion: TRADE ROUND-TRIP

    account.disconnect().await?;
    Ok(())
}
