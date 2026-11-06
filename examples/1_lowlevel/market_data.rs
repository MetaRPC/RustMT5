//! LOW-LEVEL · market data.
//!
//! Reads the account, symbols, quotes, positions and history straight off `MT5Account` — raw
//! protobuf requests in, typed `…Data` out. Run:
//!   MT5_LOGIN=… MT5_PASSWORD=… MT5_SERVER=MetaQuotes-Demo cargo run --example lowlevel_market_data

use metarpc_mt5::proto::mt5_term_api as pb;
use metarpc_mt5::MT5Account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── connect (reads credentials from the environment) ──
    let cfg = metarpc_mt5::Settings::load()?;
    let mut account = MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone());
    account.connect_by_server_name(&cfg.server_name, 30).await?;
    let sym = "EURUSD";

    // ── account_summary(): balance/equity/currency/login in one round-trip ──
    let s = account.account_summary().await?;
    println!(
        "account #{}  balance {:.2} {}  equity {:.2}",
        s.account_login, s.account_balance, s.account_currency, s.account_equity
    );

    // ── account_info_*: one property at a time, selected via the property enums ──
    let bal = account
        .account_info_double(pb::AccountInfoDoubleRequest { property_id: pb::AccountInfoDoublePropertyType::AccountBalance as i32 })
        .await?;
    let lev = account
        .account_info_integer(pb::AccountInfoIntegerRequest { property_id: pb::AccountInfoIntegerPropertyType::AccountLeverage as i32 })
        .await?;
    let cur = account
        .account_info_string(pb::AccountInfoStringRequest { property_id: pb::AccountInfoStringPropertyType::AccountCurrency as i32 })
        .await?;
    println!("balance={:.2}  leverage=1:{}  currency={}", bal.requested_value, lev.requested_value, cur.requested_value);

    // ── symbols: total count, existence check, name-by-index, Market Watch select ──
    println!("symbols total: {}", account.symbols_total(pb::SymbolsTotalRequest { mode: false }).await?.total);
    println!("{sym} exists: {}", account.symbol_exist(pb::SymbolExistRequest { name: sym.into() }).await?.exists);
    println!("symbol[0]: {}", account.symbol_name(pb::SymbolNameRequest { index: 0, selected: false }).await?.name);
    account.symbol_select(pb::SymbolSelectRequest { symbol: sym.into(), select: true }).await?;

    // ── quote: latest tick + individual symbol properties ──
    let tick = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: sym.into() }).await?;
    println!("{sym} bid/ask: {:.5}/{:.5}", tick.bid, tick.ask);
    let digits = account
        .symbol_info_integer(pb::SymbolInfoIntegerRequest { symbol: sym.into(), r#type: pb::SymbolInfoIntegerProperty::SymbolDigits as i32 })
        .await?;
    println!("digits: {}", digits.value);

    // ── symbol_params_many: full trading parameters for one symbol ──
    let params = account
        .symbol_params_many(pb::SymbolParamsManyRequest { symbol_name: Some(sym.into()), sort_type: None, page_number: None, items_per_page: None })
        .await?;
    if let Some(p) = params.symbol_infos.first() {
        println!("{} point={} vol[min/max/step]={}/{}/{}", p.name, p.point, p.volume_min, p.volume_max, p.volume_step);
    }

    // ── positions & orders (read) ──
    println!("open positions: {}", account.positions_total().await?.total_positions);
    let opened = account
        .opened_orders(pb::OpenedOrdersRequest { input_sort_mode: pb::Bmt5EnumOpenedOrderSortType::Bmt5OpenedOrderSortByOpenTimeAsc as i32 })
        .await?;
    println!("opened: {} positions, {} pending orders", opened.position_infos.len(), opened.opened_orders.len());
    let tickets = account.opened_orders_tickets(pb::OpenedOrdersTicketsRequest::default()).await?;
    println!("opened tickets fetched (debug): {tickets:?}");

    // ── history: closed positions over the default window ──
    let hist = account
        .positions_history(pb::PositionsHistoryRequest {
            sort_type: pb::AhEnumPositionsHistorySortType::AhPositionOpenTimeAsc as i32,
            position_open_time_from: None,
            position_open_time_to: None,
            page_number: Some(0),
            items_per_page: Some(100),
        })
        .await?;
    println!("history positions: {}", hist.history_positions.len());

    account.disconnect().await?;
    Ok(())
}
