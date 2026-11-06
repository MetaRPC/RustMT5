# On Position Profit (stream)

> Per-position floating P/L, pushed on a timer — new, updated and closed positions each call.

**API Information:**

* **Low-level API:** `MT5Account::on_position_profit(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.SubscriptionService`
* **RPC:** `OnPositionProfit(OnPositionProfitRequest) -> stream OnPositionProfitReply`
* **Item payload:** `OnPositionProfitData`

## 💬 Just the essentials

* **What it is.** A timer-driven stream of each open position's live profit.
* **Why you need it.** A live P/L dashboard without hammering `opened_orders`.
* **When to use.** Monitoring floating P/L; driving trailing/breakeven logic off real-time profit.

---

## Method Signature

```rust
pub async fn on_position_profit(
    &self,
    request: pb::OnPositionProfitRequest,
) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnPositionProfitData, MT5Error>>, MT5Error>
```

**Request / item:**

```protobuf
message OnPositionProfitRequest {
  int32 timer_period_milliseconds = 1;
  bool  ignore_empty_data = 2;
}

message OnPositionProfitData {
  Mt5SubEnumEventGroupType type = 1;                       // Rust field: r#type
  repeated OnPositionProfitPositionInfo new_positions = 2;
  repeated OnPositionProfitPositionInfo updated_positions = 3;
  repeated OnPositionProfitPositionInfo deleted_positions = 4;
  // + server_time
}

message OnPositionProfitPositionInfo {
  int32  index = 1;
  int64  ticket = 2;
  double profit = 3;
  string position_symbol = 4;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `timer_period_milliseconds` | `i32` | How often to push (e.g. 1000 = every second). |
| `ignore_empty_data` | `bool` | `true` → skip frames with no changes (less noise). |

## ⬆️ Item — `OnPositionProfitData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `r#type` | `i32` (`Mt5SubEnumEventGroupType`) | Event group. |
| `new_positions` | `Vec<OnPositionProfitPositionInfo>` | Positions that appeared since last frame. |
| `updated_positions` | `Vec<OnPositionProfitPositionInfo>` | Positions with changed profit. |
| `deleted_positions` | `Vec<OnPositionProfitPositionInfo>` | Positions closed since last frame. |

Each `OnPositionProfitPositionInfo`: `ticket: i64`, `profit: f64`, `position_symbol: String`.

---

## 🧱 `Mt5SubEnumEventGroupType` (field `r#type`)

| Variant | Value | Meaning |
|---------|-------|---------|
| `OrderProfit` | 0 | Position-profit group update |
| `OrderUpdate` | 1 | Order / position change group |

---

## 🧩 Notes & Tips

* **Three deltas per frame.** Merge `new` + `updated` into your map, remove `deleted`.
* **`ignore_empty_data: true`** keeps the stream quiet when nothing changed.
* **Pick a sane period.** 500–1000 ms is plenty for a dashboard; smaller means more frames.
* **`ticket` is `i64` here** (the detail structs use `u64`).

---

## 🔗 Usage Examples

### 1) Live total floating P/L

```rust
use tokio_stream::StreamExt;
use std::collections::HashMap;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_position_profit(pb::OnPositionProfitRequest {
    timer_period_milliseconds: 1000,
    ignore_empty_data: true,
}).await?;

let mut book: HashMap<i64, f64> = HashMap::new();
while let Some(Ok(d)) = s.next().await {
    for p in d.new_positions.iter().chain(d.updated_positions.iter()) {
        book.insert(p.ticket, p.profit);
    }
    for p in &d.deleted_positions { book.remove(&p.ticket); }
    let total: f64 = book.values().sum();
    println!("floating P/L: {total:+.2} ({} positions)", book.len());
}
```

### 2) Alert when a position crosses a profit target

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_position_profit(pb::OnPositionProfitRequest { timer_period_milliseconds: 500, ignore_empty_data: true }).await?;
while let Some(Ok(d)) = s.next().await {
    for p in d.updated_positions {
        if p.profit >= 100.0 { println!("#{} ({}) hit +{:.2}", p.ticket, p.position_symbol, p.profit); }
    }
}
```

### 3) Detect closes

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_position_profit(pb::OnPositionProfitRequest { timer_period_milliseconds: 1000, ignore_empty_data: false }).await?;
while let Some(Ok(d)) = s.next().await {
    for p in d.deleted_positions { println!("closed #{} ({})", p.ticket, p.position_symbol); }
}
```

---

## ⚠️ Pitfalls

* **Reading only one list.** You must merge `new`/`updated` and honor `deleted` to keep an accurate book.
* **Tiny timer period.** Floods you with frames; 500–1000 ms is usually enough.
* **Ticket type.** `i64` here vs `u64` elsewhere — cast when cross-referencing.

---

## 🧪 Testing Suggestions

* **With an open position.** `updated_positions` carries its ticket; `profit` moves with price.
* **Close it.** It appears once in `deleted_positions`, then no more.
* **Flat + `ignore_empty_data`.** Stream stays quiet.

---

## 📚 See also

* [opened_orders](../3.%20Positions_Orders/opened_orders.md) · [on_trade](on_trade.md) · [on_symbol_tick](on_symbol_tick.md)
* **Back to:** [Streaming Methods — Overview](Streaming_Methods.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
