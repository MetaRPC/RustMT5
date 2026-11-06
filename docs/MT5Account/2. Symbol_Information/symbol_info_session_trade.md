# Symbol Info — Session Trade

> The trade-session window (open → close) for a symbol on a given weekday and session index.

**API Information:**

* **Low-level API:** `MT5Account::symbol_info_session_trade(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolInfoSessionTrade(SymbolInfoSessionTradeRequest) -> SymbolInfoSessionTradeReply`
* **Payload:** `SymbolInfoSessionTradeData` → `from`, `to`

## 💬 Just the essentials

* **What it is.** MQL5 `SymbolInfoSessionTrade(...)` — when you can actually **trade**.
* **Why you need it.** Avoid sending orders when the market is closed; schedule entries around session boundaries.
* **When to use.** "Is `X` tradable right now?" and session calendars. For quote hours use [`symbol_info_session_quote`](./symbol_info_session_quote.md).

---

## Method Signature

```rust
pub async fn symbol_info_session_trade(
    &self,
    request: pb::SymbolInfoSessionTradeRequest,
) -> Result<pb::SymbolInfoSessionTradeData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolInfoSessionTradeRequest {
  string symbol = 1;
  DayOfWeek day_of_week = 2;   // Rust field: day_of_week (i32)
  uint32 session_index = 3;    // 0-based
}
message SymbolInfoSessionTradeData {
  google.protobuf.Timestamp from = 1;   // time of day; ignore the date
  google.protobuf.Timestamp to = 2;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol name. |
| `day_of_week` | `i32` (`DayOfWeek`) | Weekday (`Sunday`=0 … `Saturday`=6). |
| `session_index` | `u32` | Session number for that day, starting at 0. |

## ⬆️ Output — `SymbolInfoSessionTradeData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `from` | `Option<Timestamp>` | Trade-session start — time of day. |
| `to` | `Option<Timestamp>` | Trade-session end — time of day. |

---

## 🧱 `DayOfWeek` (field `day_of_week`)

| Variant | Value | | Variant | Value |
|---------|-------|---|---------|-------|
| `Sunday` | 0 | | `Thursday` | 4 |
| `Monday` | 1 | | `Friday` | 5 |
| `Tuesday` | 2 | | `Saturday` | 6 |
| `Wednesday` | 3 | | | |

---

## 🧩 Notes & Tips

* **This gates orders.** Outside trade sessions, `order_send` will be rejected — check here first if a symbol "won't trade".
* **Same shape as the quote session**, different meaning: trade hours are usually a subset of quote hours.
* **Time-of-day only.** Use `seconds % 86_400`.

---

## 🔗 Usage Examples

### 1) Is EURUSD tradable now (rough check)?

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::DayOfWeek as Dow;
use chrono::{Datelike, Timelike, Utc};

let now = Utc::now();
let dow = match now.weekday().num_days_from_sunday() { // 0=Sun..6=Sat
    0 => Dow::Sunday, 1 => Dow::Monday, 2 => Dow::Tuesday, 3 => Dow::Wednesday,
    4 => Dow::Thursday, 5 => Dow::Friday, _ => Dow::Saturday,
};
let s = account.symbol_info_session_trade(pb::SymbolInfoSessionTradeRequest {
    symbol: "EURUSD".into(), day_of_week: dow as i32, session_index: 0,
}).await?;
if let (Some(f), Some(t)) = (s.from, s.to) {
    let now_secs = (now.hour() * 3600 + now.minute() * 60 + now.second()) as i64;
    let open = (f.seconds % 86_400..=t.seconds % 86_400).contains(&now_secs);
    println!("tradable now: {open}");
}
```

### 2) All trade sessions for Friday

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::DayOfWeek as Dow;

for idx in 0..8u32 {
    match account.symbol_info_session_trade(pb::SymbolInfoSessionTradeRequest {
        symbol: "XAUUSD".into(), day_of_week: Dow::Friday as i32, session_index: idx,
    }).await {
        Ok(s) if s.from.is_some() => println!("Fri session {idx}"),
        _ => break,
    }
}
```

### 3) Pre-trade guard

```rust
// Combine with symbol_info_session_trade before order_send to skip closed markets.
```

---

## ⚠️ Pitfalls

* **Confusing with quote sessions.** Quotes can tick while trading is closed.
* **Date part.** Ignore it; use time-of-day.
* **Pass `day_of_week` as `i32`.**

---

## 🧪 Testing Suggestions

* **Weekday open.** EURUSD `Monday, 0` returns a valid window.
* **Weekend closed.** `Saturday` typically has no trade session (index 0 empty/err).

---

## 📚 See also

* [symbol_info_session_quote](symbol_info_session_quote.md) · [order_send](../5.%20Trading_Operations/order_send.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
