# Symbol Info — Session Quote

> The quote-session window (open → close) for a symbol on a given weekday and session index.

**API Information:**

* **Low-level API:** `MT5Account::symbol_info_session_quote(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolInfoSessionQuote(SymbolInfoSessionQuoteRequest) -> SymbolInfoSessionQuoteReply`
* **Payload:** `SymbolInfoSessionQuoteData` → `from`, `to`

## 💬 Just the essentials

* **What it is.** MQL5 `SymbolInfoSessionQuote(symbol, day, session, from, to)` — when quotes are provided.
* **Why you need it.** Know when prices update for a symbol on each weekday (distinct from when you can trade).
* **When to use.** Building a session calendar / "is the market quoting now?" display. For trade hours use [`symbol_info_session_trade`](./symbol_info_session_trade.md).

---

## Method Signature

```rust
pub async fn symbol_info_session_quote(
    &self,
    request: pb::SymbolInfoSessionQuoteRequest,
) -> Result<pb::SymbolInfoSessionQuoteData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolInfoSessionQuoteRequest {
  string symbol = 1;
  DayOfWeek day_of_week = 2;   // Rust field: day_of_week (i32)
  uint32 session_index = 3;    // 0-based
}
message SymbolInfoSessionQuoteData {
  google.protobuf.Timestamp from = 1;   // seconds since 00:00 (ignore the date part)
  google.protobuf.Timestamp to = 2;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol name. |
| `day_of_week` | `i32` (`DayOfWeek`) | Weekday. Pass `DayOfWeek::X as i32`. |
| `session_index` | `u32` | Session number for that day, starting at 0. |

## ⬆️ Output — `SymbolInfoSessionQuoteData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `from` | `Option<Timestamp>` | Session start — **time of day** (ignore the date). |
| `to` | `Option<Timestamp>` | Session end — time of day. |

---

## 🧱 `DayOfWeek` (from proto)

| Variant | Value |
|---------|-------|
| `Sunday` | 0 |
| `Monday` | 1 |
| `Tuesday` | 2 |
| `Wednesday` | 3 |
| `Thursday` | 4 |
| `Friday` | 5 |
| `Saturday` | 6 |

---

## 🧩 Notes & Tips

* **Date is meaningless.** Only the time-of-day part of `from`/`to` matters — take seconds-since-midnight.
* **Multiple sessions per day.** Increment `session_index` from 0; an error/empty means no more sessions.
* **Quote ≠ trade.** A symbol may quote outside tradable hours — compare with `symbol_info_session_trade`.

---

## 🔗 Usage Examples

### 1) Monday's first quote session

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::DayOfWeek as Dow;

let s = account.symbol_info_session_quote(pb::SymbolInfoSessionQuoteRequest {
    symbol: "EURUSD".into(),
    day_of_week: Dow::Monday as i32,
    session_index: 0,
}).await?;
if let (Some(f), Some(t)) = (s.from, s.to) {
    println!("quote session: {}..{} (secs of day)", f.seconds % 86_400, t.seconds % 86_400);
}
```

### 2) Enumerate all sessions for a day

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::DayOfWeek as Dow;

for idx in 0..8u32 {
    match account.symbol_info_session_quote(pb::SymbolInfoSessionQuoteRequest {
        symbol: "EURUSD".into(), day_of_week: Dow::Tuesday as i32, session_index: idx,
    }).await {
        Ok(s) if s.from.is_some() => println!("session {idx}: ok"),
        _ => break,
    }
}
```

### 3) Format as HH:MM

```rust
let secs = 3_600 * 8 + 60 * 30; // example
println!("{:02}:{:02}", secs / 3600, (secs % 3600) / 60);
```

---

## ⚠️ Pitfalls

* **Treating the date as real.** Use `seconds % 86_400` for time-of-day.
* **Pass `day_of_week` as `i32`.**
* **Assuming one session.** FX often has a single long session; exchanges may have several.

---

## 🧪 Testing Suggestions

* **Weekday present.** `Monday, index 0` returns a `from`/`to` pair on a liquid FX symbol.
* **Weekend empty.** `Saturday`/`Sunday` sessions may be absent — your loop stops at index 0.

---

## 📚 See also

* [symbol_info_session_trade](symbol_info_session_trade.md) · [symbol_info_integer](symbol_info_integer.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
