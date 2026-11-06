# MT5Sugar — Master Overview

> High-level, one-line convenience methods for MetaTrader 5 automation in Rust.

**`MT5Sugar`** wraps an `MT5Service` (composition, not inheritance — field `svc`) and adds ergonomic business logic:

- Quick balances & a compact account snapshot
- Prices by symbol (bid/ask/spread) with an optional **default symbol**
- Order construction from `symbol` / side / `volume` (market, pending, pip-based SL/TP)
- Position management (close, partial close, close-all, modify SL/TP)
- Position queries (by ticket, by symbol, counts, floating profit)
- History & statistics over predefined periods
- Risk sizing and pre-trade validation

It is **Layer 3** — the top of the stack. Defined in [`src/mt5/sugar.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/sugar.rs).

```text
MT5Sugar (HIGH)  ← you are here      sugar.buy_market_with_pips(...)
   │ uses
MT5Service (MID)                     svc.place_order(req) / svc.symbol_tick(sym)
   │ uses
MT5Account (LOW)                     gRPC → MT5 terminal gateway
```

---

## Conventions used throughout

* **Default symbol.** Construct with `MT5Sugar::new(svc).with_default_symbol("EURUSD")`. Methods taking `symbol: Option<&str>` fall back to it when you pass `None`; without a default, `None` is an `InvalidArgument` error.
* **Pips.** A "pip" is `10 × point` (so on a 5-digit FX symbol, 1 pip = 0.0001). Pip→price conversion uses the symbol's `SYMBOL_POINT`.
* **Success code.** Trade calls check `TRADE_RETCODE_DONE` (`10009`); a different code becomes an `MT5Error::Api { code, message, mql_code }`. The boolean-returning helpers map "done" → `true`.
* **Tickets.** Order-entry methods return the order ticket (`u64`).
* **Async.** Every method is `async` — `.await` it.

---

## Data structures

| Struct | Fields |
|--------|--------|
| `SymbolInfo` | `name`, `bid`, `ask`, `spread`, `digits`, `point`, `volume_min`, `volume_max`, `volume_step`, `contract_size` |
| `AccountInfo` | `login`, `name`, `company`, `currency`, `leverage`, `balance`, `equity` |
| `DailyStats` | `deals`, `profit`, `wins`, `losses` |
| `Period` (enum) | `Today`, `Yesterday`, `ThisWeek`, `ThisMonth` |

---

## Navigation by category (58 methods)

### 🔌 [1] Connection

| Method | Description | Docs |
|--------|-------------|------|
| `connect_by_server_name(name, timeout)` | Connect by broker cluster name (recommended) | [Docs](1.%20Connection/connect_by_server_name.md) |
| `connect_by_host_port(host, port, timeout)` | Connect by explicit address | [Docs](1.%20Connection/connect_by_host_port.md) |
| `disconnect(delete)` | Drop the connection | [Docs](1.%20Connection/disconnect.md) |
| `ping()` | Liveness check (`bool`) | [Docs](1.%20Connection/ping.md) |

### 💰 [2] Account Properties

| Method | Description | Docs |
|--------|-------------|------|
| `balance()` | Balance (realized only) | [Docs](2.%20Account_Properties/balance.md) |
| `equity()` | Equity (balance + floating P/L) | [Docs](2.%20Account_Properties/equity.md) |
| `margin()` | Used margin | [Docs](2.%20Account_Properties/margin.md) |
| `free_margin()` | Free margin | [Docs](2.%20Account_Properties/free_margin.md) |
| `margin_level()` | Margin level % | [Docs](2.%20Account_Properties/margin_level.md) |
| `floating_profit()` | Floating P/L of open positions | [Docs](2.%20Account_Properties/floating_profit.md) |
| `get_account_info()` | Compact `AccountInfo` snapshot | [Docs](2.%20Account_Properties/get_account_info.md) |

### 💹 [3] Prices & Quotes

| Method | Description | Docs |
|--------|-------------|------|
| `bid(symbol)` | Current bid | [Docs](3.%20Prices_Quotes/bid.md) |
| `ask(symbol)` | Current ask | [Docs](3.%20Prices_Quotes/ask.md) |
| `spread(symbol)` | Spread (`ask − bid`) | [Docs](3.%20Prices_Quotes/spread.md) |

### 🟢 [4] Simple Trading

| Method | Description | Docs |
|--------|-------------|------|
| `buy_market(symbol, volume, comment, magic)` | Market BUY | [Docs](4.%20Simple_Trading/buy_market.md) |
| `sell_market(symbol, volume, comment, magic)` | Market SELL | [Docs](4.%20Simple_Trading/sell_market.md) |
| `buy_limit(symbol, volume, price, comment, magic)` | BUY LIMIT | [Docs](4.%20Simple_Trading/buy_limit.md) |
| `sell_limit(symbol, volume, price, comment, magic)` | SELL LIMIT | [Docs](4.%20Simple_Trading/sell_limit.md) |
| `buy_stop(symbol, volume, price, comment, magic)` | BUY STOP | [Docs](4.%20Simple_Trading/buy_stop.md) |
| `sell_stop(symbol, volume, price, comment, magic)` | SELL STOP | [Docs](4.%20Simple_Trading/sell_stop.md) |

### 🎯 [5] Trading with SL/TP

| Method | Description | Docs |
|--------|-------------|------|
| `buy_market_with_sltp(...)` | Market BUY with absolute and/or pip SL/TP | [Docs](5.%20Trading_With_SLTP/buy_market_with_sltp.md) |
| `sell_market_with_sltp(...)` | Market SELL with absolute and/or pip SL/TP | [Docs](5.%20Trading_With_SLTP/sell_market_with_sltp.md) |
| `buy_market_with_pips(...)` | Market BUY, SL/TP purely in pips | [Docs](5.%20Trading_With_SLTP/buy_market_with_pips.md) |
| `sell_market_with_pips(...)` | Market SELL, SL/TP purely in pips | [Docs](5.%20Trading_With_SLTP/sell_market_with_pips.md) |

### 🛠️ [6] Position Management

| Method | Description | Docs |
|--------|-------------|------|
| `close_position(ticket)` | Full close by ticket | [Docs](6.%20Position_Management/close_position.md) |
| `close_position_partial(ticket, volume)` | Partial close | [Docs](6.%20Position_Management/close_position_partial.md) |
| `close_all_positions(symbol)` | Close all (optionally per symbol) | [Docs](6.%20Position_Management/close_all_positions.md) |
| `modify_position_sltp(ticket, sl, tp)` | Set SL and/or TP | [Docs](6.%20Position_Management/modify_position_sltp.md) |
| `modify_position_sl(ticket, sl)` | Move SL only | [Docs](6.%20Position_Management/modify_position_sl.md) |
| `modify_position_tp(ticket, tp)` | Move TP only | [Docs](6.%20Position_Management/modify_position_tp.md) |

### 📋 [7] Position Information

| Method | Description | Docs |
|--------|-------------|------|
| `get_open_positions()` | All open positions | [Docs](7.%20Position_Information/get_open_positions.md) |
| `get_position_by_ticket(ticket)` | One position (if open) | [Docs](7.%20Position_Information/get_position_by_ticket.md) |
| `get_positions_by_symbol(symbol)` | Positions on a symbol | [Docs](7.%20Position_Information/get_positions_by_symbol.md) |
| `has_open_position(symbol)` | Any open? | [Docs](7.%20Position_Information/has_open_position.md) |
| `count_open_positions(symbol)` | Count open | [Docs](7.%20Position_Information/count_open_positions.md) |
| `get_total_profit()` | Floating P/L total | [Docs](7.%20Position_Information/get_total_profit.md) |
| `get_profit_by_symbol(symbol)` | Floating P/L on a symbol | [Docs](7.%20Position_Information/get_profit_by_symbol.md) |

### 📈 [8] History & Statistics

| Method | Description | Docs |
|--------|-------------|------|
| `get_deals(period)` | Closed positions for a `Period` | [Docs](8.%20History_Statistics/get_deals.md) |
| `get_deals_date_range(from, to)` | Closed positions over a range | [Docs](8.%20History_Statistics/get_deals_date_range.md) |
| `get_profit(period)` | Realized profit for a `Period` | [Docs](8.%20History_Statistics/get_profit.md) |
| `get_deals_today()` / `_yesterday()` / `_this_week()` / `_this_month()` | Period shortcuts | [Docs](8.%20History_Statistics/get_deals_today.md) |
| `get_profit_today()` / `_this_week()` / `_this_month()` | Period profit shortcuts | [Docs](8.%20History_Statistics/get_profit_today.md) |
| `get_daily_stats()` | `DailyStats` for today | [Docs](8.%20History_Statistics/get_daily_stats.md) |

### 🔎 [9] Symbol Information

| Method | Description | Docs |
|--------|-------------|------|
| `get_symbol_info(symbol)` | Compact `SymbolInfo` | [Docs](9.%20Symbol_Information/get_symbol_info.md) |
| `get_all_symbols()` | All symbol names | [Docs](9.%20Symbol_Information/get_all_symbols.md) |
| `is_symbol_available(symbol)` | Exists? | [Docs](9.%20Symbol_Information/is_symbol_available.md) |
| `get_symbol_digits(symbol)` | Price digits | [Docs](9.%20Symbol_Information/get_symbol_digits.md) |
| `get_min_stop_level(symbol)` | Min SL/TP distance (points) | [Docs](9.%20Symbol_Information/get_min_stop_level.md) |

### 🛡️ [10] Risk Management

| Method | Description | Docs |
|--------|-------------|------|
| `calculate_required_margin(symbol, volume, order_type)` | Margin for a size | [Docs](10.%20Risk_Management/calculate_required_margin.md) |
| `get_max_lot_size(symbol)` | Max lot | [Docs](10.%20Risk_Management/get_max_lot_size.md) |
| `calculate_position_size(symbol, risk_percent, sl_pips)` | Risk-based lot size | [Docs](10.%20Risk_Management/calculate_position_size.md) |
| `calculate_sltp(symbol, is_buy, sl_pips, tp_pips)` | Pips → absolute SL/TP prices | [Docs](10.%20Risk_Management/calculate_sltp.md) |
| `can_open_position(symbol, volume)` | Pre-trade validation `(bool, reason)` | [Docs](10.%20Risk_Management/can_open_position.md) |

---

## Minimal usage

```rust
use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = MT5Account::new(12_345_678, "password", None);
    let mut sugar = MT5Sugar::new(MT5Service::new(account)).with_default_symbol("EURUSD");

    sugar.connect_by_server_name("MetaQuotes-Demo", 30).await?;

    println!("balance {:.2}, bid {:.5}", sugar.balance().await?, sugar.bid(None).await?);

    // risk 1% over a 20-pip stop, then enter with a pip bracket
    let lots = sugar.calculate_position_size("EURUSD", 1.0, 20.0).await?;
    let ticket = sugar.buy_market_with_pips(None, lots, Some(20.0), Some(40.0), "demo", 1001).await?;
    println!("opened #{ticket}");

    sugar.disconnect(false).await?;
    Ok(())
}
```

---

**See also:** [MT5Service — Overview](../MT5Service/MT5Service.Overview.md) · [MT5Account — Master Overview](../MT5Account/MT5Account.Master.Overview.md) · [Strategies](../Strategies/Strategies.Master.Overview.md)
