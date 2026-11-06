# Opened Orders (RECOMMENDED)

> Everything currently live: open **positions** (with P/L, SL/TP, prices) and active **pending orders**, in one call.

**API Information:**

* **Low-level API:** `MT5Account::opened_orders(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.TradeFunctions`
* **RPC:** `OpenedOrders(OpenedOrdersRequest) -> OpenedOrdersReply`
* **Payload:** `OpenedOrdersData` → `opened_orders: Vec<OpenedOrderInfo>`, `position_infos: Vec<PositionInfo>`

## 💬 Just the essentials

* **What it is.** A full snapshot of your live book: positions **and** pendings.
* **Why you need it.** `position_infos` carry live `profit`, `price_current`, SL/TP, swap — everything to manage trades.
* **When to use.** Any position/pending dashboard or management loop. This is the method `MT5Sugar`'s position helpers wrap.

---

## 🎯 Purpose

* List open positions with live P/L and current price
* List active pending orders (limits/stops) awaiting fill
* Read SL/TP, volumes, magic numbers, comments per entry

---

## Method Signature

```rust
pub async fn opened_orders(
    &self,
    request: pb::OpenedOrdersRequest,
) -> Result<pb::OpenedOrdersData, MT5Error>
```

**Request message:**

```protobuf
message OpenedOrdersRequest {
  Bmt5EnumOpenedOrderSortType input_sort_mode = 1;   // Rust field: input_sort_mode (i32)
}
```

**Reply payload:**

```protobuf
message OpenedOrdersData {
  repeated OpenedOrderInfo opened_orders = 1;  // pending orders
  repeated PositionInfo    position_infos = 2; // open positions
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `input_sort_mode` | `i32` (`Bmt5EnumOpenedOrderSortType`) | Sort order (see enum). Pass `Sort::X as i32`. |

### `Bmt5EnumOpenedOrderSortType`

| Variant | Value | Order |
|---------|-------|-------|
| `Bmt5OpenedOrderSortByOpenTimeAsc` | 0 | Open time ↑ |
| `Bmt5OpenedOrderSortByOpenTimeDesc` | 1 | Open time ↓ |
| `Bmt5OpenedOrderSortByOrderTicketIdAsc` | 2 | Ticket ↑ |
| `Bmt5OpenedOrderSortByOrderTicketIdDesc` | 3 | Ticket ↓ |

---

## ⬆️ Output — `OpenedOrdersData`

### `position_infos: Vec<PositionInfo>` (open positions)

| Field | Rust type | Description |
|-------|-----------|-------------|
| `ticket` | `u64` | Position ticket |
| `symbol` | `String` | Symbol |
| `volume` | `f64` | Lots |
| `price_open` | `f64` | Entry price |
| `price_current` | `f64` | Current price |
| `stop_loss` / `take_profit` | `f64` | SL / TP (0 if unset) |
| `swap` | `f64` | Accrued swap |
| `profit` | `f64` | Live floating P/L |
| `r#type` | `i32` (`Bmt5EnumPositionType`) | 0 = Buy, 1 = Sell |
| `magic_number` | `i64` | EA magic |
| `comment` | `String` | Comment |
| `open_time` / `last_update_time` | `Option<Timestamp>` | Times |
| `position_commission` | `f64` | Commission |
| `identifier` | `i64` | Position id |

### `opened_orders: Vec<OpenedOrderInfo>` (pending orders)

| Field | Rust type | Description |
|-------|-----------|-------------|
| `ticket` | `u64` | Order ticket |
| `symbol` | `String` | Symbol |
| `price_open` | `f64` | Trigger price |
| `stop_limit` | `f64` | Stop-limit price (for stop-limit orders) |
| `stop_loss` / `take_profit` | `f64` | SL / TP |
| `volume_initial` / `volume_current` | `f64` | Requested / remaining lots |
| `r#type` | `i32` (`Bmt5EnumOrderType`) | Order type (buy/sell limit/stop…) |
| `state` | `i32` (`Bmt5EnumOrderState`) | Order state |
| `type_filling` / `type_time` | `i32` | Filling / time policy |
| `time_setup` / `time_expiration` | `Option<Timestamp>` | Placed / expiry |
| `magic_number` | `i64` | EA magic |
| `comment` | `String` | Comment |

---

## 🧩 Notes & Tips

* **Two lists, different meanings.** `position_infos` = exposure you hold; `opened_orders` = instructions not yet filled.
* **`r#type` is a raw identifier** on both structs — Rust keyword.
* **Position side.** `PositionInfo.r#type`: `0` Buy, `1` Sell (`Bmt5EnumPositionType`).
* **Sum `profit`** across `position_infos` for total floating P/L.

---

## 🔗 Usage Examples

### 1) Print open positions with P/L

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::Bmt5EnumOpenedOrderSortType as Sort;

let data = account.opened_orders(pb::OpenedOrdersRequest {
    input_sort_mode: Sort::Bmt5OpenedOrderSortByOpenTimeAsc as i32,
}).await?;
for p in &data.position_infos {
    let side = if p.r#type == 0 { "BUY" } else { "SELL" };
    println!("#{} {side} {} {:.2} lot  P/L {:+.2}", p.ticket, p.symbol, p.volume, p.profit);
}
```

### 2) Total floating P/L

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::Bmt5EnumOpenedOrderSortType as Sort;

let data = account.opened_orders(pb::OpenedOrdersRequest { input_sort_mode: Sort::Bmt5OpenedOrderSortByOpenTimeAsc as i32 }).await?;
let total: f64 = data.position_infos.iter().map(|p| p.profit + p.swap).sum();
println!("floating P/L (incl. swap): {total:+.2}");
```

### 3) List pending orders

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::Bmt5EnumOpenedOrderSortType as Sort;

let data = account.opened_orders(pb::OpenedOrdersRequest { input_sort_mode: Sort::Bmt5OpenedOrderSortByOrderTicketIdAsc as i32 }).await?;
for o in &data.opened_orders {
    println!("pending #{} {} @ {:.5} ({} lot)", o.ticket, o.symbol, o.price_open, o.volume_current);
}
```

---

## ⚠️ Pitfalls

* **Mixing the two lists.** Closing a pending (`opened_orders`) deletes the order; closing a position (`position_infos`) flattens exposure — different operations.
* **`input_sort_mode` is required.** Pass a valid enum as `i32`.
* **Snapshot.** P/L is as-of the call; re-query for fresh numbers or stream with `on_position_profit`.

---

## 🧪 Testing Suggestions

* **Flat.** Both vectors empty; `position_infos.len() == positions_total().total_positions == 0`.
* **One market buy.** `position_infos.len() == 1`, `r#type == 0`, `profit` changes with price.
* **One pending.** Appears in `opened_orders`, not `position_infos`.

---

## 📚 See also

* [positions_total](positions_total.md) · [opened_orders_tickets](opened_orders_tickets.md) · [on_position_profit](../6.%20Streaming_Methods/on_position_profit.md)
* **Back to:** [Positions & Orders — Overview](Positions_Orders.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
