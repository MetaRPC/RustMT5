# Opened Orders Tickets

> Just the ticket numbers of open pending orders and open positions — a lightweight list.

**API Information:**

* **Low-level API:** `MT5Account::opened_orders_tickets(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.TradeFunctions`
* **RPC:** `OpenedOrdersTickets(OpenedOrdersTicketsRequest) -> OpenedOrdersTicketsReply`
* **Payload:** `OpenedOrdersTicketsData` → `opened_orders_tickets`, `opened_position_tickets`

## 💬 Just the essentials

* **What it is.** The ticket ids only — no prices, no P/L.
* **Why you need it.** Cheap iteration or diffing "what's open now vs before" without the full payload.
* **When to use.** When you only need ids (e.g. to modify/close by ticket). For details use [`opened_orders`](./opened_orders.md).

---

## Method Signature

```rust
pub async fn opened_orders_tickets(
    &self,
    request: pb::OpenedOrdersTicketsRequest,
) -> Result<pb::OpenedOrdersTicketsData, MT5Error>
```

**Request / Reply:**

```protobuf
message OpenedOrdersTicketsRequest {}   // no fields
message OpenedOrdersTicketsData {
  repeated int64 opened_orders_tickets = 1;    // pending order tickets
  repeated int64 opened_position_tickets = 2;  // open position tickets
}
```

---

## 🔽 Input

None — pass `pb::OpenedOrdersTicketsRequest {}`.

## ⬆️ Output — `OpenedOrdersTicketsData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `opened_orders_tickets` | `Vec<i64>` | Tickets of active pending orders. |
| `opened_position_tickets` | `Vec<i64>` | Tickets of open positions. |

---

## 🧩 Notes & Tips

* **Two lists, like `opened_orders`.** Pendings vs positions, but ids only.
* **Pair with detail calls.** Use a ticket with `order_modify` / `order_close`, or match against `opened_orders` entries.
* **Great for diffs.** Store the sets and compare across polls to detect fills/closes.

---

## 🔗 Usage Examples

### 1) Count open vs pending

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let t = account.opened_orders_tickets(pb::OpenedOrdersTicketsRequest {}).await?;
println!("{} positions, {} pendings", t.opened_position_tickets.len(), t.opened_orders_tickets.len());
```

### 2) Iterate position tickets

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let t = account.opened_orders_tickets(pb::OpenedOrdersTicketsRequest {}).await?;
for ticket in t.opened_position_tickets {
    println!("position ticket {ticket}");
}
```

### 3) Detect new fills across two polls

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use std::collections::HashSet;

let before: HashSet<i64> = account.opened_orders_tickets(pb::OpenedOrdersTicketsRequest {}).await?
    .opened_position_tickets.into_iter().collect();
// ... later ...
let after: HashSet<i64> = account.opened_orders_tickets(pb::OpenedOrdersTicketsRequest {}).await?
    .opened_position_tickets.into_iter().collect();
for t in after.difference(&before) { println!("newly opened position {t}"); }
```

---

## ⚠️ Pitfalls

* **Ids are `i64` here.** The detail structs use `u64` for `ticket`; cast when matching (`ticket as i64`).
* **No details.** If you need symbol/volume/P/L, call `opened_orders`.

---

## 🧪 Testing Suggestions

* **Length parity.** `opened_position_tickets.len() == positions_total().total_positions`.
* **Cross-check.** Every id appears as a `ticket` in `opened_orders`'s matching list.

---

## 📚 See also

* [opened_orders](opened_orders.md) · [positions_total](positions_total.md) · [order_modify](../5.%20Trading_Operations/order_modify.md)
* **Back to:** [Positions & Orders — Overview](Positions_Orders.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
