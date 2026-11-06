# 2. Symbol Information — Overview

> Everything about an instrument: does it exist, is it in Market Watch, its live tick, its properties and trading rules, session times, and bar history.

These methods are **read-only** (except `symbol_select`, which toggles Market Watch visibility). Most live on the `mt5_term_api.MarketInfo` gRPC service and return a typed `…Data` payload.

---

## Methods in this group

### Discovery & Market Watch

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`symbols_total(req)`](./symbols_total.md) | `SymbolsTotalData` | How many symbols exist (all, or Market-Watch-only). |
| [`symbol_exist(req)`](./symbol_exist.md) | `SymbolExistData` | Does a symbol name exist (and is it custom)? |
| [`symbol_name(req)`](./symbol_name.md) | `SymbolNameData` | Symbol name by index. |
| [`symbol_select(req)`](./symbol_select.md) | `SymbolSelectData` | Add / remove a symbol in Market Watch. |
| [`symbol_is_synchronized(req)`](./symbol_is_synchronized.md) | `SymbolIsSynchronizedData` | Is the symbol's data synced with the server? |
| [`symbol_list(req)`](./symbol_list.md) | `SymbolListData` | The full list of symbol names. |

### Quote (tick)

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`symbol_info_tick(req)`](./symbol_info_tick.md) | `MrpcMqlTick` | Latest tick: bid, ask, last, volume, time. |

### Properties (one at a time)

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`symbol_info_double(req)`](./symbol_info_double.md) | `SymbolInfoDoubleData` | One `double` property (point, bid, ask, volume step, …). |
| [`symbol_info_integer(req)`](./symbol_info_integer.md) | `SymbolInfoIntegerData` | One `integer` property (digits, spread, trade mode, …). |
| [`symbol_info_string(req)`](./symbol_info_string.md) | `SymbolInfoStringData` | One `string` property (base/profit currency, description, …). |
| [`symbol_info_margin_rate(req)`](./symbol_info_margin_rate.md) | `SymbolInfoMarginRateData` | Initial/maintenance margin rates for an order type. |

### Sessions

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`symbol_info_session_quote(req)`](./symbol_info_session_quote.md) | `SymbolInfoSessionQuoteData` | Quote session window for a weekday. |
| [`symbol_info_session_trade(req)`](./symbol_info_session_trade.md) | `SymbolInfoSessionTradeData` | Trade session window for a weekday. |

### Bulk & history

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`symbol_params_many(req)`](./symbol_params_many.md) | `SymbolParamsManyData` | Many symbols' full parameter sets in one call. |
| [`price_history(req)`](./price_history.md) | `PriceHistoryData` | OHLC bar history for a symbol/timeframe range. |

---

## When to use which

* **"Is this symbol tradable right now?"** → `symbol_exist` → `symbol_select` (make sure it's in Market Watch) → `symbol_info_tick`.
* **One live number** (point size, digits, spread, volume step) → `symbol_info_double` / `symbol_info_integer`.
* **Everything about one symbol at once** → `symbol_params_many` (pass a single name) — far fewer round-trips than many `symbol_info_*` calls.
* **Candles / backtest input** → `price_history`.
* **Is the market open?** → `symbol_info_session_trade` for today's weekday.

> The single-property `symbol_info_*` calls mirror MQL5's `SymbolInfoDouble/Integer/String`. When you need several properties, `symbol_params_many` is the efficient choice — and `MT5Service` / `MT5Sugar` wrap the common ones (`point`, `digits`, `bid`, `ask`) for you.

---

**Back to:** [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
