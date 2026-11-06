# MT5Service — Mid-Level API Overview

> The friendly facade over [`MT5Account`](../MT5Account/MT5Account.Master.Overview.md): the same operations, but you pass **plain arguments** (typed enums, `&str` symbols) and get back **clean values** (`f64`/`i64`/`String`/`bool`) instead of protobuf `…Data` wrappers and hand-built `…Request` structs.

---

## ⚠️ What this layer is (and isn't)

`MT5Service` is **architectural middleware** — it sits between `MT5Account` (raw gRPC/protobuf) and `MT5Sugar` (business logic).

```text
┌─────────────────────────────────────────────────┐
│ MT5Sugar   — HIGH   order building, pip math,   │
│                      risk sizing, one-liners    │
└───────────────┬─────────────────────────────────┘
                │ uses Service methods
                ▼
┌─────────────────────────────────────────────────┐
│ MT5Service  — MID    ← you are here             │
│   friendly args (typed enums, &str, sides)      │
│   clean values (unwraps …Data → scalar)         │
└───────────────┬─────────────────────────────────┘
                │ calls Account methods
                ▼
┌─────────────────────────────────────────────────┐
│ MT5Account  — LOW    raw gRPC, …Data payloads   │
└───────────────┬─────────────────────────────────┘
                │ gRPC
                ▼              MT5 terminal gateway
```

Defined in [`src/mt5/service.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/service.rs). It wraps one `MT5Account` (field `acc`); borrow it any time with `service.account()` / `service.account_mut()`.

### What the value-add actually is (Rust)

Two concrete conveniences, nothing magic:

1. **Friendly arguments** — pass a typed enum (`SymbolInfoDoubleProperty::SymbolBid`) and a `&str` symbol; the Service builds the `…Request` and does the `as i32` cast for you.
2. **Clean return values** — the simple getters return the inner scalar (`f64`, `i64`, `String`, `bool`, `i32`) instead of the `…Data` struct you'd otherwise destructure.

What it does **not** do (be honest about it): it does **not** aggregate extra RPC calls into `account_summary` (that's a pass-through here), and it does **not** convert protobuf `Timestamp` into a date type — timestamps stay as `prost_types::Timestamp`. Richer behavior (order construction, pip/risk math) lives one layer up in [`MT5Sugar`](../MT5Sugar/MT5Sugar.Master.Overview.md).

---

## Method value, at a glance

| Tier | Meaning | Examples |
|------|---------|----------|
| 🟢 **Clean value** | Unwraps `…Data` → a scalar you can use directly | `account_double`, `symbols_total`, `symbol_double`, `positions_total`, `symbol_select` |
| 🔵 **Friendly args** | Still returns a `…Data`, but you skip building the request & casting enums | `symbol_tick`, `symbol_params_many`, `opened_orders`, `order_history`, `calculate_margin`, the streams |
| ⚪ **Pass-through** | Same shape as `MT5Account`, just a friendlier name/args | `account_summary`, `symbol_exist`, `opened_orders_tickets`, `place_order`/`modify_order`/`close_order`/`check_order` |

> Rule of thumb: using **Sugar**? Everything here just works beneath it. Using **Service directly**? Prefer the 🟢/🔵 methods; for a ⚪ pass-through you can equally call `MT5Account` via `service.account()`.

---

## Quick navigation — section files

* **[1. Account Information](./1.%20Account_Information.md)** — 4 methods. Summary snapshot + single typed properties as scalars.
* **[2. Symbol Information](./2.%20Symbol_Information.md)** — 13 methods. Discovery, quotes, properties, sessions, bulk params.
* **[3. Positions & Orders](./3.%20Positions_Orders.md)** — 5 methods. Open book, tickets, history.
* **[4. Market Depth](./4.%20Market_Depth.md)** — 3 methods. Subscribe / read / release the DOM.
* **[5. Trading Operations](./5.%20Trading_Operations.md)** — 6 methods. Send/modify/close/check + margin/profit calculators.
* **[6. Streaming Methods](./6.%20Streaming_Methods.md)** — 5 streams. Ticks, trades, profit, tickets, transactions.

Plus **4 connection delegates** (`connect_by_server_name`, `connect_by_host_port`, `reconnect`, `disconnect`) — see the usage pattern below.

---

## Complete method index (36)

### 1. Account Information (4)

| Method | Returns | Tier |
|--------|---------|------|
| `account_summary()` | `AccountSummaryData` | ⚪ pass-through |
| `account_double(property_id)` | `f64` | 🟢 clean |
| `account_integer(property_id)` | `i64` | 🟢 clean |
| `account_string(property_id)` | `String` | 🟢 clean |

### 2. Symbol Information (13)

| Method | Returns | Tier |
|--------|---------|------|
| `symbols_total(selected_only)` | `i32` | 🟢 clean |
| `symbol_exist(name)` | `SymbolExistData` | ⚪ pass-through |
| `symbol_name(index, selected_only)` | `String` | 🟢 clean |
| `symbol_select(symbol, select)` | `bool` | 🟢 clean |
| `symbol_is_synchronized(symbol)` | `bool` | 🟢 clean |
| `symbol_double(symbol, property)` | `f64` | 🟢 clean |
| `symbol_integer(symbol, property)` | `i64` | 🟢 clean |
| `symbol_string(symbol, property)` | `String` | 🟢 clean |
| `symbol_margin_rate(symbol, order_type)` | `SymbolInfoMarginRateData` | 🔵 friendly args |
| `symbol_tick(symbol)` | `MrpcMqlTick` | 🔵 friendly args |
| `symbol_session_quote(symbol, day, session_index)` | `SymbolInfoSessionQuoteData` | 🔵 friendly args |
| `symbol_session_trade(symbol, day, session_index)` | `SymbolInfoSessionTradeData` | 🔵 friendly args |
| `symbol_params_many(symbol_name, page_number, items_per_page)` | `SymbolParamsManyData` | 🔵 friendly args |

### 3. Positions & Orders (5)

| Method | Returns | Tier |
|--------|---------|------|
| `positions_total()` | `i32` | 🟢 clean |
| `opened_orders(sort_mode)` | `OpenedOrdersData` | 🔵 friendly args |
| `opened_orders_tickets()` | `OpenedOrdersTicketsData` | ⚪ pass-through |
| `order_history(sort_mode, from, to, page, per_page)` | `OrdersHistoryData` | 🔵 friendly args |
| `positions_history(sort_type, from, to, page, per_page)` | `PositionsHistoryData` | 🔵 friendly args |

### 4. Market Depth (3)

| Method | Returns | Tier |
|--------|---------|------|
| `subscribe_market_depth(symbol)` | `MarketBookAddData` | 🔵 friendly args |
| `market_depth(symbol)` | `MarketBookGetData` | 🔵 friendly args |
| `unsubscribe_market_depth(symbol)` | `MarketBookReleaseData` | 🔵 friendly args |

### 5. Trading Operations (6)

| Method | Returns | Tier |
|--------|---------|------|
| `place_order(request)` | `OrderSendData` | ⚪ pass-through (raw request) |
| `modify_order(request)` | `OrderModifyData` | ⚪ pass-through (raw request) |
| `close_order(request)` | `OrderCloseData` | ⚪ pass-through (raw request) |
| `check_order(request)` | `OrderCheckData` | ⚪ pass-through (raw request) |
| `calculate_margin(symbol, order_type, volume, open_price)` | `OrderCalcMarginData` | 🔵 friendly args |
| `calculate_profit(symbol, order_type, volume, open_price, close_price)` | `OrderCalcProfitData` | 🔵 friendly args |

### 6. Streaming Methods (5)

| Method | Item | Tier |
|--------|------|------|
| `stream_ticks(symbols)` | `OnSymbolTickData` | 🔵 friendly args |
| `stream_trade_updates()` | `OnTradeData` | ⚪ pass-through |
| `stream_position_profits(timer_period_ms, ignore_empty)` | `OnPositionProfitData` | 🔵 friendly args |
| `stream_opened_tickets(timer_period_ms)` | `OnPositionsAndPendingOrdersTicketsData` | 🔵 friendly args |
| `stream_transactions()` | `OnTradeTransactionData` | ⚪ pass-through |

---

## Usage pattern

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use metarpc_mt5::{MT5Account, MT5Service};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. low-level account
    let account = MT5Account::new(12_345_678, "password", None);

    // 2. wrap in the mid-level service
    let mut service = MT5Service::new(account);

    // 3. connect (delegates to the account)
    service.connect_by_server_name("MetaQuotes-Demo", 30).await?;

    // 4. clean values — no …Data to destructure
    let balance = service.account_double(pb::AccountInfoDoublePropertyType::AccountBalance).await?;
    let digits  = service.symbol_integer("EURUSD", pb::SymbolInfoIntegerProperty::SymbolDigits).await?;
    let tick    = service.symbol_tick("EURUSD").await?;
    println!("balance {balance:.2}, digits {digits}, bid {:.5}", tick.bid);

    // 5. need the raw layer? borrow it
    let _acc = service.account();

    // 6. disconnect (the bool mirrors the Python facade; it is ignored here)
    service.disconnect(false).await?;
    Ok(())
}
```

---

## When to use Service vs Account

**Use `MT5Service` when** you want ergonomic calls (typed-enum args, scalar returns) or you're building on top via `MT5Sugar`.

**Call `MT5Account` directly (via `service.account()`) when** the Service method is a ⚪ pass-through and you'd rather not add a layer, or you need a field/variant the Service doesn't surface.

---

**See also:** [MT5Account — Master Overview](../MT5Account/MT5Account.Master.Overview.md) · [MT5Sugar — Master Overview](../MT5Sugar/MT5Sugar.Master.Overview.md)
