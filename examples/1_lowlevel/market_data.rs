/*══════════════════════════════════════════════════════════════════════════════
 FILE: examples/1_lowlevel/market_data.rs - LOW-LEVEL MT5 MARKET DATA DEMO
 PURPOSE:
   Read the account, symbols, quotes, positions and history straight off the raw
   MT5Account client - protobuf requests in, typed ...Data out. No trades here.

 🎯 WHO SHOULD USE THIS:
   • Developers learning the low-level MT5 gRPC API (read side)
   • Building monitoring / dashboard / reporting tools
   • Seeing exactly what each protobuf reply contains

 📚 WHAT THIS DEMO COVERS (5 sections):

   1. CONNECT
      • Settings::load() + MT5Account::connect_by_server_name()

   2. ACCOUNT INFO
      • account_summary()                     - balance / equity / currency / login
      • account_info_double/integer/string()  - one property at a time

   3. SYMBOLS & QUOTES
      • symbols_total() / symbol_exist() / symbol_name() / symbol_select()
      • symbol_info_tick() / symbol_info_integer() - latest tick, digits
      • symbol_params_many()                  - point, volume min/max/step

   4. POSITIONS & ORDERS
      • positions_total() / opened_orders() / opened_orders_tickets()

   5. HISTORY
      • positions_history()                   - closed positions over a window

 🔄 API LEVELS:
   LOW-LEVEL  (MT5Account) - THIS FILE: raw gRPC/protobuf requests & replies
   MID-LEVEL  (MT5Service) - typed wrappers, less boilerplate
   HIGH-LEVEL (MT5Sugar)   - one-liner helpers with smart defaults

 USAGE:
   cargo run --example lowlevel_market_data
   (reads creds from examples/0_common/settings.json or the MT5_* env vars)
══════════════════════════════════════════════════════════════════════════════*/

use metarpc_mt5::proto::mt5_term_api as pb;
use metarpc_mt5::MT5Account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // region: CONNECT
    // ══════════════════════════════════════════════════════════════
    // 1. CONNECT
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // Settings::load() + connect_by_server_name()
    //   Load credentials (settings.json / MT5_* env), build the raw
    //   MT5Account and connect via the broker server name.
    // ──────────────────────────────────────────────────────────────
    let cfg = metarpc_mt5::Settings::load()?;
    let mut account = MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone());
    account.connect_by_server_name(&cfg.server_name, 30).await?;
    let sym = "EURUSD";
    // endregion: CONNECT

    // region: ACCOUNT INFO
    // ══════════════════════════════════════════════════════════════
    // 2. ACCOUNT INFO
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // account_summary()
    //   Balance / equity / currency / login in one round-trip.
    // ──────────────────────────────────────────────────────────────
    let s = account.account_summary().await?;
    println!(
        "account #{}  balance {:.2} {}  equity {:.2}",
        s.account_login, s.account_balance, s.account_currency, s.account_equity
    );

    // ──────────────────────────────────────────────────────────────
    // account_info_double/integer/string()
    //   One property at a time, selected via the property enums.
    // ──────────────────────────────────────────────────────────────
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
    // endregion: ACCOUNT INFO

    // region: SYMBOLS & QUOTES
    // ══════════════════════════════════════════════════════════════
    // 3. SYMBOLS & QUOTES
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // symbols_total() / symbol_exist() / symbol_name() / symbol_select()
    //   Count symbols, check existence, name-by-index, add to Market Watch.
    // ──────────────────────────────────────────────────────────────
    println!("symbols total: {}", account.symbols_total(pb::SymbolsTotalRequest { mode: false }).await?.total);
    println!("{sym} exists: {}", account.symbol_exist(pb::SymbolExistRequest { name: sym.into() }).await?.exists);
    println!("symbol[0]: {}", account.symbol_name(pb::SymbolNameRequest { index: 0, selected: false }).await?.name);
    account.symbol_select(pb::SymbolSelectRequest { symbol: sym.into(), select: true }).await?;

    // ──────────────────────────────────────────────────────────────
    // symbol_info_tick() / symbol_info_integer()
    //   Latest Bid/Ask tick + individual symbol properties (digits).
    // ──────────────────────────────────────────────────────────────
    let tick = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: sym.into() }).await?;
    println!("{sym} bid/ask: {:.5}/{:.5}", tick.bid, tick.ask);
    let digits = account
        .symbol_info_integer(pb::SymbolInfoIntegerRequest { symbol: sym.into(), r#type: pb::SymbolInfoIntegerProperty::SymbolDigits as i32 })
        .await?;
    println!("digits: {}", digits.value);

    // ──────────────────────────────────────────────────────────────
    // symbol_params_many()
    //   Full trading parameters for one symbol (point, volume rules).
    // ──────────────────────────────────────────────────────────────
    let params = account
        .symbol_params_many(pb::SymbolParamsManyRequest { symbol_name: Some(sym.into()), sort_type: None, page_number: None, items_per_page: None })
        .await?;
    if let Some(p) = params.symbol_infos.first() {
        println!("{} point={} vol[min/max/step]={}/{}/{}", p.name, p.point, p.volume_min, p.volume_max, p.volume_step);
    }
    // endregion: SYMBOLS & QUOTES

    // region: POSITIONS & ORDERS
    // ══════════════════════════════════════════════════════════════
    // 4. POSITIONS & ORDERS (read)
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // positions_total() / opened_orders() / opened_orders_tickets()
    //   Count open positions, list open positions + pending orders,
    //   and fetch just the ticket numbers.
    // ──────────────────────────────────────────────────────────────
    println!("open positions: {}", account.positions_total().await?.total_positions);
    let opened = account
        .opened_orders(pb::OpenedOrdersRequest { input_sort_mode: pb::Bmt5EnumOpenedOrderSortType::Bmt5OpenedOrderSortByOpenTimeAsc as i32 })
        .await?;
    println!("opened: {} positions, {} pending orders", opened.position_infos.len(), opened.opened_orders.len());
    let tickets = account.opened_orders_tickets(pb::OpenedOrdersTicketsRequest::default()).await?;
    println!("opened tickets fetched (debug): {tickets:?}");
    // endregion: POSITIONS & ORDERS

    // region: HISTORY
    // ══════════════════════════════════════════════════════════════
    // 5. HISTORY
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // positions_history()
    //   Closed positions over the default window (paginated).
    // ──────────────────────────────────────────────────────────────
    let hist = account
        .positions_history(pb::PositionsHistoryRequest {
            sort_type: pb::AhEnumPositionsHistorySortType::AhPositionOpenTimeAsc as i32,
            position_open_time_from: None,
            position_open_time_to: None,
            page_number: Some(0),
            items_per_page: Some(100),
            ..Default::default()
        })
        .await?;
    println!("history positions: {}", hist.history_positions.len());
    // endregion: HISTORY

    account.disconnect().await?;
    Ok(())
}
