# On Positions And Pending Orders Tickets (stream)

> Open-position and pending-order ticket lists, pushed on a timer — a lightweight "what's open now" feed.

**API Information:**

* **Low-level API:** `MT5Account::on_positions_and_pending_orders_tickets(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.SubscriptionService`
* **RPC:** `OnPositionsAndPendingOrdersTickets(...) -> stream`
* **Item payload:** `OnPositionsAndPendingOrdersTicketsData`

## 💬 Just the essentials

* **What it is.** A timer stream of just the ticket ids for open positions and pendings.
* **Why you need it.** Cheap way to notice opens/closes without pulling full details each time.
* **When to use.** Keeping a live set of active tickets; diffing to detect fills/cancellations.

---

## Method Signature

```rust
pub async fn on_positions_and_pending_orders_tickets(
    &self,
    request: pb::OnPositionsAndPendingOrdersTicketsRequest,
) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnPositionsAndPendingOrdersTicketsData, MT5Error>>, MT5Error>
```

**Request / item:**

```protobuf
message OnPositionsAndPendingOrdersTicketsRequest {
  int32 timer_period_milliseconds = 1;
}

message OnPositionsAndPendingOrdersTicketsData {
  repeated uint64 position_tickets = 1;
  repeated uint64 pending_order_tickets = 2;
  google.protobuf.Timestamp server_time = 3;
  string terminal_instance_guid_id = 4;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `timer_period_milliseconds` | `i32` | Push interval (e.g. 1000). |

## ⬆️ Item — `OnPositionsAndPendingOrdersTicketsData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `position_tickets` | `Vec<u64>` | Current open-position tickets. |
| `pending_order_tickets` | `Vec<u64>` | Current pending-order tickets. |
| `server_time` | `Option<Timestamp>` | Server time of the snapshot. |
| `terminal_instance_guid_id` | `String` | Originating terminal session. |

> Note: tickets are `u64` here (vs `i64` in the non-streaming [`opened_orders_tickets`](../3.%20Positions_Orders/opened_orders_tickets.md)).

---

## 🧩 Notes & Tips

* **Full snapshot each frame.** Every push is the complete current set — diff against your last set to find adds/removes.
* **Lighter than `on_trade` + fetch.** Ids only; fetch details only when the set changes.
* **Pick a modest period.** 1000 ms is fine for most monitors.

---

## 🔗 Usage Examples

### 1) Detect opens and closes by diffing

```rust
use tokio_stream::StreamExt;
use std::collections::HashSet;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_positions_and_pending_orders_tickets(
    pb::OnPositionsAndPendingOrdersTicketsRequest { timer_period_milliseconds: 1000 },
).await?;

let mut prev: HashSet<u64> = HashSet::new();
while let Some(Ok(d)) = s.next().await {
    let now: HashSet<u64> = d.position_tickets.iter().copied().collect();
    for t in now.difference(&prev) { println!("opened position {t}"); }
    for t in prev.difference(&now) { println!("closed position {t}"); }
    prev = now;
}
```

### 2) Keep live counts

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_positions_and_pending_orders_tickets(
    pb::OnPositionsAndPendingOrdersTicketsRequest { timer_period_milliseconds: 2000 },
).await?;
while let Some(Ok(d)) = s.next().await {
    println!("{} positions, {} pendings", d.position_tickets.len(), d.pending_order_tickets.len());
}
```

### 3) Fetch details only on change

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_positions_and_pending_orders_tickets(
    pb::OnPositionsAndPendingOrdersTicketsRequest { timer_period_milliseconds: 1000 },
).await?;
let mut last_len = usize::MAX;
while let Some(Ok(d)) = s.next().await {
    if d.position_tickets.len() != last_len {
        last_len = d.position_tickets.len();
        // set changed — pull full detail once
        let _ = account.opened_orders(pb::OpenedOrdersRequest {
            input_sort_mode: pb::Bmt5EnumOpenedOrderSortType::Bmt5OpenedOrderSortByOpenTimeAsc as i32,
        }).await?;
    }
}
```

---

## ⚠️ Pitfalls

* **Treating a frame as a delta.** It's a full snapshot — diff it yourself.
* **Ticket type.** `u64` here; cast if comparing with `i64` sources.
* **Count-only change detection misses swaps.** A close + open between frames can keep the count equal — diff the sets, not just lengths, when correctness matters.

---

## 🧪 Testing Suggestions

* **Open a trade.** Its ticket appears in `position_tickets` on the next frame.
* **Place a pending.** It shows in `pending_order_tickets`; on fill it moves to `position_tickets`.

---

## 📚 See also

* [opened_orders_tickets](../3.%20Positions_Orders/opened_orders_tickets.md) · [on_trade](on_trade.md)
* **Back to:** [Streaming Methods — Overview](Streaming_Methods.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
