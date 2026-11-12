/*══════════════════════════════════════════════════════════════════════════════
 FILE: examples/2_service/service_demo.rs - MID-LEVEL MT5Service DEMO
 PURPOSE:
   The typed facade over MT5Account: friendly arguments (property enums, symbols,
   sort modes) and clean return values (plain f64 / i64 / String). Read-only -
   account, symbols, positions and pre-trade calculators. No orders are placed.

 🎯 WHO SHOULD USE THIS:
   • Developers who want typed values instead of raw protobuf replies
   • Building monitoring / reporting tools with less boilerplate
   • Learning the mid-level wrapper before the high-level Sugar API

 📚 WHAT THIS DEMO COVERS (4 sections):

   1. ACCOUNT
      • account_summary()                         - balance / equity / currency
      • account_double/integer/string()           - one clean property value

   2. SYMBOLS
      • symbols_total() / symbol_select()         - count, add to Market Watch
      • symbol_tick()                             - latest Bid/Ask
      • symbol_double() / symbol_integer()        - Bid, digits

   3. POSITIONS & ORDERS
      • positions_total() / opened_orders()       - open positions + pendings

   4. CALCULATORS (pre-trade)
      • calculate_margin()                        - required margin for a lot
      • calculate_profit()                        - profit for a price move

 🔄 API LEVELS:
   LOW-LEVEL  (MT5Account) - raw gRPC/protobuf requests & replies
   MID-LEVEL  (MT5Service) - THIS FILE: typed wrappers, clean values
   HIGH-LEVEL (MT5Sugar)   - one-liner helpers with smart defaults

 ℹ️  SAFE: read-only - no orders are placed.

 USAGE:
   cargo run --example service_demo
   (reads creds from examples/0_common/settings.json or the MT5_* env vars)
══════════════════════════════════════════════════════════════════════════════*/

use metarpc_mt5::proto::mt5_term_api as pb;
use metarpc_mt5::{MT5Account, MT5Service};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // region: CONNECT
    // ══════════════════════════════════════════════════════════════
    // CONNECT
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // Settings::load() + MT5Service::connect_by_server_name()
    //   Load credentials, wrap a raw MT5Account in MT5Service, connect.
    // ──────────────────────────────────────────────────────────────
    let cfg = metarpc_mt5::Settings::load()?;
    let mut service = MT5Service::new(MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone()));
    service.connect_by_server_name(&cfg.server_name, 30).await?;
    let sym = "EURUSD";
    // endregion: CONNECT

    // region: ACCOUNT
    // ══════════════════════════════════════════════════════════════
    // 1. ACCOUNT
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // account_summary() + account_double/integer/string()
    //   One-call summary, then individual properties as clean values.
    // ──────────────────────────────────────────────────────────────
    let s = service.account_summary().await?;
    println!("balance {:.2} {}  equity {:.2}", s.account_balance, s.account_currency, s.account_equity);
    println!("equity(double) {:.2}", service.account_double(pb::AccountInfoDoublePropertyType::AccountEquity).await?);
    println!("leverage 1:{}", service.account_integer(pb::AccountInfoIntegerPropertyType::AccountLeverage).await?);
    println!("company {}", service.account_string(pb::AccountInfoStringPropertyType::AccountCompany).await?);
    // endregion: ACCOUNT

    // region: SYMBOLS
    // ══════════════════════════════════════════════════════════════
    // 2. SYMBOLS
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // symbols_total() / symbol_select() / symbol_tick() / symbol_double() / symbol_integer()
    //   Count symbols, add to Market Watch, latest tick, and property getters.
    // ──────────────────────────────────────────────────────────────
    println!("symbols total {}", service.symbols_total(false).await?);
    service.symbol_select(sym, true).await?;
    let tick = service.symbol_tick(sym).await?;
    println!("{sym} bid/ask {:.5}/{:.5}", tick.bid, tick.ask);
    println!("{sym} bid(double) {:.5}  digits {}", service.symbol_double(sym, pb::SymbolInfoDoubleProperty::SymbolBid).await?, service.symbol_integer(sym, pb::SymbolInfoIntegerProperty::SymbolDigits).await?);
    // endregion: SYMBOLS

    // region: POSITIONS & ORDERS
    // ══════════════════════════════════════════════════════════════
    // 3. POSITIONS & ORDERS
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // positions_total() / opened_orders()
    //   Count open positions, list open positions + pending orders.
    // ──────────────────────────────────────────────────────────────
    println!("open positions {}", service.positions_total().await?);
    let opened = service.opened_orders(pb::Bmt5EnumOpenedOrderSortType::Bmt5OpenedOrderSortByOpenTimeAsc).await?;
    println!("opened: {} positions / {} pending", opened.position_infos.len(), opened.opened_orders.len());
    // endregion: POSITIONS & ORDERS

    // region: CALCULATORS
    // ══════════════════════════════════════════════════════════════
    // 4. CALCULATORS (pre-trade)
    // ══════════════════════════════════════════════════════════════

    // ──────────────────────────────────────────────────────────────
    // calculate_margin() / calculate_profit()
    //   Required margin for a lot, and profit for a given price move.
    // ──────────────────────────────────────────────────────────────
    println!("margin 0.1 lot {:.2}", service.calculate_margin(sym, pb::EnumOrderTypeTf::OrderTypeTfBuy, 0.1, tick.ask).await?.margin);
    println!("profit +50p {:.2}", service.calculate_profit(sym, pb::EnumOrderTypeTf::OrderTypeTfBuy, 0.1, tick.ask, tick.ask + 0.0050).await?.profit);
    // endregion: CALCULATORS

    service.disconnect(false).await?;
    Ok(())
}
