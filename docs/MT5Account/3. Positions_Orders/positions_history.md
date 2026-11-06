# Positions History

> Closed positions within a time range — one tidy record per position, paged.

**API Information:**

* **Low-level API:** `MT5Account::positions_history(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.AccountHelper`
* **RPC:** `PositionsHistory(PositionsHistoryRequest) -> PositionsHistoryReply`
* **Payload:** `PositionsHistoryData` → `history_positions: Vec<PositionHistoryInfo>`

## 💬 Just the essentials

* **What it is.** Closed positions with entry/exit prices and realized P/L — already aggregated per position.
* **Why you need it.** Cleaner than reconstructing positions from `order_history`'s orders+deals.
* **When to use.** Trade journals and realized-P/L reports per position.

---

## Method Signature

```rust
pub async fn positions_history(
    &self,
    request: pb::PositionsHistoryRequest,
) -> Result<pb::PositionsHistoryData, MT5Error>
```

**Request message:**

```protobuf
message PositionsHistoryRequest {
  AhEnumPositionsHistorySortType sort_type = 1;        // Rust field: sort_type (i32)
  google.protobuf.Timestamp position_open_time_from = 2;
  google.protobuf.Timestamp position_open_time_to = 3;
  optional int32 page_number = 4;
  optional int32 items_per_page = 5;
}
```

**Reply payload:**

```protobuf
message PositionsHistoryData {
  repeated PositionHistoryInfo history_positions = 1;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `sort_type` | `i32` (`AhEnumPositionsHistorySortType`) | Sort order (see enum). |
| `position_open_time_from` | `Option<Timestamp>` | Range start (by position open time). |
| `position_open_time_to` | `Option<Timestamp>` | Range end. |
| `page_number` | `Option<i32>` | 0-based page. |
| `items_per_page` | `Option<i32>` | Page size. |

### `AhEnumPositionsHistorySortType`

| Variant | Value | Order |
|---------|-------|-------|
| `AhPositionOpenTimeAsc` | 0 | Open time ↑ |
| `AhPositionOpenTimeDesc` | 1 | Open time ↓ |
| `AhPositionTicketAsc` | 2 | Ticket ↑ |
| `AhPositionTicketDesc` | 3 | Ticket ↓ |

## ⬆️ Output — `PositionHistoryInfo` (per position)

| Field | Rust type | Description |
|-------|-----------|-------------|
| `position_ticket` | `u64` | Position ticket |
| `symbol` | `String` | Symbol |
| `order_type` | `i32` (`AhEnumPositionsHistoryOrderType`) | Direction/type (0 Buy, 1 Sell, …) |
| `volume` | `f64` | Lots |
| `open_price` / `close_price` | `f64` | Entry / exit |
| `open_time` / `close_time` | `Option<Timestamp>` | Opened / closed |
| `stop_loss` / `take_profit` | `f64` | SL / TP |
| `profit` | `f64` | Realized profit |
| `swap` | `f64` | Swap |
| `commission` / `fee` | `f64` | Costs |
| `market_value` | `f64` | Position value |
| `comment` | `String` | Comment |
| `magic` | `i64` | EA magic |

---

## 🧱 `AhEnumPositionsHistoryOrderType` (field `order_type`)

| Variant | Value | | Variant | Value |
|---------|-------|---|---------|-------|
| `AhOrderTypeBuy` | 0 | | `AhOrderTypeSellStop` | 5 |
| `AhOrderTypeSell` | 1 | | `AhOrderTypeBuyStopLimit` | 6 |
| `AhOrderTypeBuyLimit` | 2 | | `AhOrderTypeSellStopLimit` | 7 |
| `AhOrderTypeSellLimit` | 3 | | `AhOrderTypeCloseBy` | 8 |
| `AhOrderTypeBuyStop` | 4 | | | |

---

## 🧩 Notes & Tips

* **One row per closed position** — no order/deal split to reconcile.
* **Net result** = `profit + swap + commission + fee`.
* **Range is by open time.** Positions opened in the window; a long-held trade opened earlier won't appear even if closed in the window.
* **Page via `items_per_page`.** With `None` you may get only the first page.

---

## 🔗 Usage Examples

### 1) Closed positions, last 30 days

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::AhEnumPositionsHistorySortType as Sort;

fn ts(s: i64) -> prost_types::Timestamp { prost_types::Timestamp { seconds: s, nanos: 0 } }
let now = chrono::Utc::now().timestamp();

let data = account.positions_history(pb::PositionsHistoryRequest {
    sort_type: Sort::AhPositionOpenTimeDesc as i32,
    position_open_time_from: Some(ts(now - 30 * 86_400)),
    position_open_time_to: Some(ts(now)),
    page_number: Some(0),
    items_per_page: Some(100),
}).await?;
for p in &data.history_positions {
    let side = if p.order_type == 0 { "BUY" } else { "SELL" };
    println!("#{} {side} {} {:.2} lot  {:.5}->{:.5}  P/L {:+.2}",
        p.position_ticket, p.symbol, p.volume, p.open_price, p.close_price, p.profit);
}
```

### 2) Net realized over the window

```rust
let net: f64 = data.history_positions.iter()
    .map(|p| p.profit + p.swap + p.commission + p.fee)
    .sum();
println!("net realized: {net:+.2}");
```

### 3) Win rate

```rust
let (wins, total) = data.history_positions.iter()
    .fold((0usize, 0usize), |(w, t), p| (w + (p.profit > 0.0) as usize, t + 1));
if total > 0 { println!("win rate: {:.1}%", wins as f64 / total as f64 * 100.0); }
```

---

## ⚠️ Pitfalls

* **Options are `Option<..>`.** Wrap times and paging in `Some(..)`.
* **Range semantics.** Filtered by **open** time, not close time.
* **`order_type as i32`** when comparing to the enum.

---

## 🧪 Testing Suggestions

* **Recently closed trade.** Appears with matching `open_price`/`close_price` and expected `profit` sign.
* **Empty window.** A quiet range returns an empty `history_positions`.

---

## 📚 See also

* [order_history](order_history.md) · [opened_orders](opened_orders.md)
* **Back to:** [Positions & Orders — Overview](Positions_Orders.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
