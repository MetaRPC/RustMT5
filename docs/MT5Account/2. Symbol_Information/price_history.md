# Price History (Bars)

> OHLC candle history for a symbol and timeframe — the input for charts, indicators and backtests.

**API Information:**

* **Low-level API:** `MT5Account::price_history(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `PriceHistory(PriceHistoryRequest) -> PriceHistoryReply`
* **Payload:** `PriceHistoryData` → `bars: Vec<PriceBar>`

## 💬 Just the essentials

* **What it is.** MQL5 `CopyRates`-style bar history.
* **Why you need it.** Candles for analysis: moving averages, ranges, swing highs/lows, backtests.
* **When to use.** Any time you need historical OHLC. Make sure the symbol is selected and synchronized first.

---

## Method Signature

```rust
pub async fn price_history(
    &self,
    request: pb::PriceHistoryRequest,
) -> Result<pb::PriceHistoryData, MT5Error>
```

**Request message:**

```protobuf
message PriceHistoryRequest {
  string symbol = 1;
  int32  timeframe = 2;   // MQL5 ENUM_TIMEFRAMES value (PERIOD_M1=1, PERIOD_H1=16385, PERIOD_D1=16408)
  int64  from_time = 3;   // Unix seconds; both 0 = newest max_bars bars
  int64  to_time = 4;     // Unix seconds
  int32  max_bars = 5;    // upper bound (defaults to 500); terminal has its own ceiling
}
```

**Reply payload:**

```protobuf
message PriceHistoryData { repeated PriceBar bars = 1; }

message PriceBar {
  int64  time = 1;         // Unix seconds, bar open
  double open = 2;
  double high = 3;
  double low = 4;
  double close = 5;
  int64  tick_volume = 6;
  int32  spread = 7;
  int64  real_volume = 8;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol name (selected & synchronized). |
| `timeframe` | `i32` | MQL5 `ENUM_TIMEFRAMES` value — see table below. |
| `from_time` | `i64` | Range start, Unix seconds. `0` + `to_time 0` → newest bars. |
| `to_time` | `i64` | Range end, Unix seconds. |
| `max_bars` | `i32` | Cap on returned bars (default 500); don't request a decade of M1. |

## ⬆️ Output — `PriceHistoryData.bars: Vec<PriceBar>`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `time` | `i64` | Bar open time, Unix seconds. |
| `open` / `high` / `low` / `close` | `f64` | OHLC prices. |
| `tick_volume` | `i64` | Number of ticks in the bar. |
| `spread` | `i32` | Spread at bar close, points. |
| `real_volume` | `i64` | Exchange volume (0 for pure FX). |

---

## ⏱️ Common `timeframe` values (MQL5 `ENUM_TIMEFRAMES`)

| Constant | Value | | Constant | Value |
|----------|-------|---|----------|-------|
| M1 | 1 | | H1 | 16385 |
| M5 | 5 | | H4 | 16388 |
| M15 | 15 | | D1 | 16408 |
| M30 | 30 | | W1 | 32769 |
| | | | MN1 | 49153 |

---

## 🧩 Notes & Tips

* **Newest bars shortcut.** Leave `from_time` and `to_time` at `0` to get the newest `max_bars` bars — what opening a chart does.
* **Respect the ceiling.** The terminal caps bar counts (`ERR_HISTORY_BARS_LIMIT`); page by narrowing the time range if you need more.
* **Select + sync first.** Call `symbol_select` and wait for `symbol_is_synchronized` to avoid empty/partial history.
* **Times are Unix seconds** — convert to `DateTime` for display.

---

## 🔗 Usage Examples

### 1) Last 100 H1 bars

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let data = account.price_history(pb::PriceHistoryRequest {
    symbol: "EURUSD".into(),
    timeframe: 16385, // H1
    from_time: 0, to_time: 0,
    max_bars: 100,
}).await?;
println!("{} bars", data.bars.len());
if let Some(b) = data.bars.last() {
    println!("latest close {:.5} @ {}", b.close, b.time);
}
```

### 2) A specific date range of D1 bars

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let from = chrono::Utc::now().timestamp() - 30 * 86_400; // 30 days ago
let data = account.price_history(pb::PriceHistoryRequest {
    symbol: "XAUUSD".into(),
    timeframe: 16408, // D1
    from_time: from, to_time: chrono::Utc::now().timestamp(),
    max_bars: 500,
}).await?;
for b in &data.bars {
    println!("{}  O{:.2} H{:.2} L{:.2} C{:.2}", b.time, b.open, b.high, b.low, b.close);
}
```

### 3) Simple SMA(close, 20)

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let bars = account.price_history(pb::PriceHistoryRequest {
    symbol: "EURUSD".into(), timeframe: 16385, from_time: 0, to_time: 0, max_bars: 50,
}).await?.bars;
let n = 20.min(bars.len());
if n > 0 {
    let sma: f64 = bars.iter().rev().take(n).map(|b| b.close).sum::<f64>() / n as f64;
    println!("SMA({n}) = {sma:.5}");
}
```

---

## ⚠️ Pitfalls

* **Empty bars.** Symbol not selected/synchronized, or the range falls outside available history.
* **Asking for too much.** A wide range on a small timeframe hits the bar-count ceiling — narrow it or raise granularity.
* **Timeframe is a raw `i32`.** Use the MQL5 constant values above, not an ordinal.
* **Bar order.** Iterate by `time`; don't assume a particular ordering — sort if you depend on it.

---

## 🧪 Testing Suggestions

* **Count cap.** With `max_bars: 100`, `bars.len() <= 100`.
* **OHLC sanity.** For each bar `low <= open,close <= high`.
* **Latest vs tick.** The last D1 `close` is near the current `symbol_info_tick` bid/ask.

---

## 📚 See also

* [symbol_select](symbol_select.md) · [symbol_is_synchronized](symbol_is_synchronized.md) · [symbol_info_tick](symbol_info_tick.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
