//! MID-LEVEL · MT5Service.
//!
//! The typed facade: friendly arguments (property enums, symbols, sort modes) and clean return
//! values (plain `f64`/`i64`/`String`). Run:
//!   MT5_LOGIN=… MT5_PASSWORD=… MT5_SERVER=MetaQuotes-Demo cargo run --example service_demo

use metarpc_mt5::proto::mt5_term_api as pb;
use metarpc_mt5::{MT5Account, MT5Service};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = metarpc_mt5::Settings::load()?;
    let mut service = MT5Service::new(MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone()));
    service.connect_by_server_name(&cfg.server_name, 30).await?;
    let sym = "EURUSD";

    // ── account: one-call summary + individual properties (clean values) ──
    let s = service.account_summary().await?;
    println!("balance {:.2} {}  equity {:.2}", s.account_balance, s.account_currency, s.account_equity);
    println!("equity(double) {:.2}", service.account_double(pb::AccountInfoDoublePropertyType::AccountEquity).await?);
    println!("leverage 1:{}", service.account_integer(pb::AccountInfoIntegerPropertyType::AccountLeverage).await?);
    println!("company {}", service.account_string(pb::AccountInfoStringPropertyType::AccountCompany).await?);

    // ── symbols: count, select, tick, and property getters ──
    println!("symbols total {}", service.symbols_total(false).await?);
    service.symbol_select(sym, true).await?;
    let tick = service.symbol_tick(sym).await?;
    println!("{sym} bid/ask {:.5}/{:.5}", tick.bid, tick.ask);
    println!("{sym} bid(double) {:.5}  digits {}", service.symbol_double(sym, pb::SymbolInfoDoubleProperty::SymbolBid).await?, service.symbol_integer(sym, pb::SymbolInfoIntegerProperty::SymbolDigits).await?);

    // ── positions & orders ──
    println!("open positions {}", service.positions_total().await?);
    let opened = service.opened_orders(pb::Bmt5EnumOpenedOrderSortType::Bmt5OpenedOrderSortByOpenTimeAsc).await?;
    println!("opened: {} positions / {} pending", opened.position_infos.len(), opened.opened_orders.len());

    // ── calculators (pre-trade) ──
    println!("margin 0.1 lot {:.2}", service.calculate_margin(sym, pb::EnumOrderTypeTf::OrderTypeTfBuy, 0.1, tick.ask).await?.margin);
    println!("profit +50p {:.2}", service.calculate_profit(sym, pb::EnumOrderTypeTf::OrderTypeTfBuy, 0.1, tick.ask, tick.ask + 0.0050).await?.profit);

    service.disconnect(false).await?;
    Ok(())
}
