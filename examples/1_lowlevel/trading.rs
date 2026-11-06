//! LOW-LEVEL · trading.
//!
//! Margin/profit calculators, a pre-trade `order_check`, then a real market BUY → modify → close
//! round-trip on `MT5Account`. ⚠️ Opens and closes a 0.01-lot position — DEMO accounts only. Run:
//!   MT5_LOGIN=… MT5_PASSWORD=… MT5_SERVER=MetaQuotes-Demo cargo run --example lowlevel_trading

use metarpc_mt5::proto::mt5_term_api as pb;
use metarpc_mt5::MT5Account;

const TRADE_RETCODE_DONE: u32 = 10009;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = metarpc_mt5::Settings::load()?;
    let mut account = MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone());
    account.connect_by_server_name(&cfg.server_name, 30).await?;
    let sym = "EURUSD";
    let volume = 0.01;

    // ── current price (needed for the calculators and the order price) ──
    let tick = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: sym.into() }).await?;
    println!("{sym} ask={:.5}", tick.ask);

    // ── order_calc_margin: margin a BUY would require ──
    let margin = account
        .order_calc_margin(pb::OrderCalcMarginRequest {
            symbol: sym.into(),
            order_type: pb::EnumOrderTypeTf::OrderTypeTfBuy as i32,
            volume,
            open_price: tick.ask,
        })
        .await?;
    println!("required margin: {:.2}", margin.margin);

    // ── order_calc_profit: profit if price moved +50 points ──
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

    // ── order_check: validate a trade request before sending it ──
    let check = account
        .order_check(pb::OrderCheckRequest {
            mql_trade_request: Some(pb::MrpcMqlTradeRequest {
                action: pb::MrpcEnumTradeRequestActions::TradeActionDeal as i32,
                symbol: sym.into(),
                volume,
                price: tick.ask,
                order_type: pb::EnumOrderTypeTf::OrderTypeTfBuy as i32,
                type_filling: pb::MrpcEnumOrderTypeFilling::OrderFillingIoc as i32,
                type_time: pb::MrpcEnumOrderTypeTime::OrderTimeGtc as i32,
                ..Default::default()
            }),
        })
        .await?;
    if let Some(r) = &check.mql_trade_check_result {
        println!("order_check: code={} free_margin_after={:.2}", r.returned_code, r.free_margin);
    }

    // ── order_send: open a market BUY ──
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

    // ── order_modify: attach SL/TP (200 points around entry) ──
    let modified = account
        .order_modify(pb::OrderModifyRequest {
            ticket,
            stop_loss: Some(tick.ask - 0.0200),
            take_profit: Some(tick.ask + 0.0200),
            ..Default::default()
        })
        .await?;
    println!("modify SL/TP: code={}", modified.returned_code);

    // ── order_close: flatten the position ──
    let closed = account.order_close(pb::OrderCloseRequest { ticket, volume: 0.0, slippage: 10 }).await?;
    println!("closed #{ticket}: code={}", closed.returned_code);

    account.disconnect().await?;
    Ok(())
}
