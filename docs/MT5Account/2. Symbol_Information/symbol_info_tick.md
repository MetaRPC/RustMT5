# Symbol Info Tick (RECOMMENDED)

> The latest tick for a symbol: bid, ask, last, volume and timestamps — the quote you trade against.

**API Information:**

* **Low-level API:** `MT5Account::symbol_info_tick(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolInfoTick(SymbolInfoTickRequest) -> SymbolInfoTickReply`
* **Payload:** `MrpcMqlTick`

## 💬 Just the essentials

* **What it is.** MQL5 `SymbolInfoTick(symbol, tick)` — one fresh quote.
* **Why you need it.** Current bid/ask drives every price decision: spread, entry, SL/TP distances, P/L.
* **When to use.** Right before sizing or sending an order; for a one-off price. For a live feed use [`on_symbol_tick`](../6.%20Streaming_Methods/on_symbol_tick.md).

---

## 🎯 Purpose

* Current bid / ask (and spread = ask − bid)
* Last deal price and its volume (exchange instruments)
* Precise update time (`time_msc`, milliseconds)

---

## Method Signature

```rust
pub async fn symbol_info_tick(
    &self,
    request: pb::SymbolInfoTickRequest,
) -> Result<pb::MrpcMqlTick, MT5Error>
```

**Request message:**

```protobuf
message SymbolInfoTickRequest { string symbol = 1; }
```

**Reply payload — `MrpcMqlTick`:**

```protobuf
message MrpcMqlTick {
  int64  time = 1;         // Unix seconds, last update
  double bid = 2;
  double ask = 3;
  double last = 4;         // last deal price
  uint64 volume = 5;       // volume for the last price
  int64  time_msc = 6;     // last update, milliseconds
  uint32 flags = 7;        // tick flags
  double volume_real = 8;  // higher-accuracy volume
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol name (must be selected in Market Watch). |

## ⬆️ Output — `MrpcMqlTick`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `time` | `i64` | Last update, Unix **seconds**. |
| `bid` | `f64` | Best sell price. |
| `ask` | `f64` | Best buy price. |
| `last` | `f64` | Last deal price (0 for pure FX with no exchange last). |
| `volume` | `u64` | Volume for the last price. |
| `time_msc` | `i64` | Last update, Unix **milliseconds**. |
| `flags` | `u32` | Bitmask of what changed (bid/ask/last/volume). |
| `volume_real` | `f64` | Volume with greater accuracy. |

---

## 🧩 Notes & Tips

* **Spread = `ask − bid`**, in price units; divide by `point` (from `symbol_info_double`) for spread in points.
* **FX `last` is often 0.** Use `bid`/`ask`; `last`/`volume` matter mainly for exchange instruments.
* **Two timestamps.** `time` is seconds, `time_msc` is milliseconds — use `time_msc` to order ticks precisely.
* **Select first.** On many servers you must `symbol_select` the symbol before the tick is populated.

---

## 🔗 Usage Examples

### 1) Bid / ask / spread

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let t = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: "EURUSD".into() }).await?;
println!("bid {:.5}  ask {:.5}  spread {:.5}", t.bid, t.ask, t.ask - t.bid);
```

### 2) Spread in points

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::SymbolInfoDoubleProperty as D;

let t = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: "EURUSD".into() }).await?;
let point = account
    .symbol_info_double(pb::SymbolInfoDoubleRequest { symbol: "EURUSD".into(), r#type: D::SymbolPoint as i32 })
    .await?.value;
println!("spread: {:.1} points", (t.ask - t.bid) / point);
```

### 3) Freshness check

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let t = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: "EURUSD".into() }).await?;
let age_ms = chrono::Utc::now().timestamp_millis() - t.time_msc;
if age_ms > 5_000 { println!("[WARN] quote is {age_ms} ms old — market may be closed"); }
```

---

## ⚠️ Pitfalls

* **Empty tick (all zeros).** Usually means the symbol isn't selected or isn't synchronized yet.
* **Dividing by `point` you didn't fetch.** Get `point`/`digits` from `symbol_info_double`/`integer` for the right scale.
* **Snapshot, not a stream.** Each call is one quote; for continuous updates subscribe with `on_symbol_tick`.

---

## 🧪 Testing Suggestions

* **During market hours.** `bid > 0`, `ask >= bid`, `time_msc` close to now.
* **Weekend / closed.** Values present but `time`/`time_msc` stale — your freshness check fires.

---

## 📚 See also

* [symbol_info_double](symbol_info_double.md) · [on_symbol_tick](../6.%20Streaming_Methods/on_symbol_tick.md) · [symbol_select](symbol_select.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
