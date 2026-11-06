# Order Close

> Close a position (fully or partially) by ticket.

**API Information:**

* **Low-level API:** `MT5Account::order_close(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.TradeFunctions`
* **RPC:** `OrderClose(OrderCloseRequest) -> OrderCloseReply`
* **Payload:** `OrderCloseData` → `returned_code: u32`, `close_mode`, …

## 💬 Just the essentials

* **What it is.** Close by ticket — full or partial.
* **Why you need it.** Exit a trade or scale out part of it.
* **When to use.** Taking profit, cutting losses, flattening. For "close all", loop over `opened_orders`.

---

## Method Signature

```rust
pub async fn order_close(
    &self,
    request: pb::OrderCloseRequest,
) -> Result<pb::OrderCloseData, MT5Error>
```

**Request / Reply:**

```protobuf
message OrderCloseRequest {
  uint64 ticket = 1;
  double volume = 2;     // lots to close (full volume = full close)
  int32  slippage = 5;
}
message OrderCloseData {
  uint32 returned_code = 1;             // 10009 = success
  string returned_string_code = 2;
  string returned_code_description = 3;
  MrpcOrderCloseMode close_mode = 4;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `ticket` | `u64` | Position ticket to close. |
| `volume` | `f64` | Lots to close. Pass the position's full volume to close completely; less for a partial. |
| `slippage` | `i32` | Max price deviation, points. |

## ⬆️ Output — `OrderCloseData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `returned_code` | `u32` | **`10009` = success**. |
| `returned_code_description` | `String` | Human-readable retcode. |
| `close_mode` | `i32` (`MrpcOrderCloseMode`) | How it was closed. |

---

## 🧱 `MrpcOrderCloseMode` (reply field `close_mode`)

| Variant | Value | Meaning |
|---------|-------|---------|
| `MrpcMarketOrderClose` | 0 | Full market close of a position |
| `MrpcMarketOrderPartialClose` | 1 | Partial market close |
| `MrpcPendingOrderRemove` | 2 | A pending order was removed |

---

## 🧩 Notes & Tips

* **Full vs partial.** `volume` equal to the position's size → full close; smaller → partial (the remainder stays open).
* **Get the volume first.** Read it from `opened_orders().position_infos[..].volume`.
* **Pendings.** To cancel a resting pending order, close it by its ticket the same way (the gateway deletes it).
* **Check `returned_code`.**

---

## 🔗 Usage Examples

### 1) Full close by ticket

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

// find the position's volume
let pos = account.opened_orders(pb::OpenedOrdersRequest {
    input_sort_mode: pb::Bmt5EnumOpenedOrderSortType::Bmt5OpenedOrderSortByOpenTimeAsc as i32,
}).await?;
if let Some(p) = pos.position_infos.iter().find(|p| p.ticket == 123456) {
    let res = account.order_close(pb::OrderCloseRequest { ticket: p.ticket, volume: p.volume, slippage: 20 }).await?;
    println!("close code {}", res.returned_code);
}
```

### 2) Partial close (scale out half)

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let res = account.order_close(pb::OrderCloseRequest {
    ticket: 123456,
    volume: 0.05,   // half of a 0.10 lot position
    slippage: 20,
}).await?;
println!("partial close code {}", res.returned_code);
```

### 3) Close all positions

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let data = account.opened_orders(pb::OpenedOrdersRequest {
    input_sort_mode: pb::Bmt5EnumOpenedOrderSortType::Bmt5OpenedOrderSortByOpenTimeAsc as i32,
}).await?;
for p in &data.position_infos {
    let r = account.order_close(pb::OrderCloseRequest { ticket: p.ticket, volume: p.volume, slippage: 30 }).await?;
    println!("#{} -> {}", p.ticket, r.returned_code);
}
```

---

## ⚠️ Pitfalls

* **Wrong `volume`.** More than the position holds → rejection; use the actual volume.
* **Partial leaves a remainder.** Don't assume the ticket is gone after a partial close.
* **Ignoring `returned_code`.**
* **Slippage too small** in fast markets → requote/rejection.

---

## 🧪 Testing Suggestions

* **Full close.** After opening 0.10 lot, close 0.10 → `10009`; position disappears from `opened_orders`.
* **Partial.** Close 0.05 of 0.10 → remaining 0.05 still listed.

---

## 📚 See also

* [order_send](order_send.md) · [order_modify](order_modify.md) · [opened_orders](../3.%20Positions_Orders/opened_orders.md)
* **Back to:** [Trading Operations — Overview](Trading_Operations.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
