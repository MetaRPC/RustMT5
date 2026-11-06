# Order Modify

> Change SL/TP of a position, or price/SL/TP/expiry of a pending order — by ticket.

**API Information:**

* **Low-level API:** `MT5Account::order_modify(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.TradeFunctions`
* **RPC:** `OrderModify(OrderModifyRequest) -> OrderModifyReply`
* **Payload:** `OrderModifyData` → `returned_code: u32`, …

## 💬 Just the essentials

* **What it is.** MQL5 `OrderModify`/`PositionModify` — adjust an existing order/position.
* **Why you need it.** Move SL to breakeven, trail stops, re-price a pending.
* **When to use.** Managing a live trade. The `MT5Sugar` orchestrators call this to trail.

---

## Method Signature

```rust
pub async fn order_modify(
    &self,
    request: pb::OrderModifyRequest,
) -> Result<pb::OrderModifyData, MT5Error>
```

**Request message:**

```protobuf
message OrderModifyRequest {
  uint64 ticket = 1;                       // position or pending order
  optional double stop_loss = 2;           // new SL
  optional double take_profit = 3;         // new TP
  optional double price = 4;               // new price — pendings only
  optional Tmt5EnumOrderTypeTime expiration_time_type = 5;  // pendings only
  optional google.protobuf.Timestamp expiration_time = 6;
  optional double stop_limit = 8;          // pendings only
}
```

**Reply payload `OrderModifyData` (key fields):** `returned_code: u32`, `deal`, `order`, `volume`, `price`, `bid`, `ask`, `comment`, `returned_code_description`.

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `ticket` | `u64` | Order/position ticket (required). |
| `stop_loss` | `Option<f64>` | New SL (absolute price). |
| `take_profit` | `Option<f64>` | New TP (absolute price). |
| `price` | `Option<f64>` | New trigger price — **pending orders only**. |
| `expiration_time_type` | `Option<i32>` | Expiry policy — pendings only. |
| `expiration_time` | `Option<Timestamp>` | Expiry time — pendings only. |
| `stop_limit` | `Option<f64>` | New stop-limit price — pendings only. |

## ⬆️ Output — `OrderModifyData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `returned_code` | `u32` | **`10009` = success**. |
| `price` | `f64` | Confirmed price. |
| `bid` / `ask` | `f64` | Prices at the operation. |
| `returned_code_description` | `String` | Human-readable retcode. |

---

## 🧱 `Tmt5EnumOrderTypeTime` (field `expiration_time_type`, pending orders)

| Variant | Value | Meaning |
|---------|-------|---------|
| `Tmt5OrderTimeGtc` | 0 | Good-till-cancelled (default) |
| `Tmt5OrderTimeDay` | 1 | Valid for the current trading day |
| `Tmt5OrderTimeSpecified` | 2 | Valid until `expiration_time` |
| `Tmt5OrderTimeSpecifiedDay` | 3 | Valid until the end of `expiration_time`'s day |

---

## 🧩 Notes & Tips

* **Only set what you change.** Leave untouched fields as `None` — but note some servers want the SL/TP you intend to keep re-sent; when in doubt, send both `stop_loss` and `take_profit`.
* **`price` is for pendings.** Modifying an open position's entry price is not possible — only SL/TP.
* **Absolute prices.** SL/TP/price are price levels, not distances.
* **Respect stops level.** New SL/TP must sit beyond `SYMBOL_TRADE_STOPS_LEVEL` from the market.

---

## 🔗 Usage Examples

### 1) Move SL to breakeven

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let entry = 1.08500;  // the position's open price
let res = account.order_modify(pb::OrderModifyRequest {
    ticket: 123456,
    stop_loss: Some(entry),       // breakeven
    take_profit: None,
    price: None, expiration_time_type: None, expiration_time: None, stop_limit: None,
}).await?;
println!("modify code {}", res.returned_code);
```

### 2) Set both SL and TP

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let res = account.order_modify(pb::OrderModifyRequest {
    ticket: 123456,
    stop_loss: Some(1.08300),
    take_profit: Some(1.09000),
    price: None, expiration_time_type: None, expiration_time: None, stop_limit: None,
}).await?;
if res.returned_code != 10009 { println!("rejected: {}", res.returned_code_description); }
```

### 3) Re-price a pending order

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let res = account.order_modify(pb::OrderModifyRequest {
    ticket: 123457,
    stop_loss: None, take_profit: None,
    price: Some(1.08000),   // new trigger
    expiration_time_type: None, expiration_time: None, stop_limit: None,
}).await?;
println!("re-price code {}", res.returned_code);
```

---

## ⚠️ Pitfalls

* **Modifying a position's `price`.** Not allowed — only SL/TP on open positions.
* **Clearing SL/TP unintentionally.** If a server treats `None` as "clear", pass the value you want to keep.
* **Too-tight new stops.** Rejected per stops level.
* **Ignoring `returned_code`.**

---

## 🧪 Testing Suggestions

* **Breakeven move.** After a profitable open, set SL to entry → `10009`; verify via `opened_orders`.
* **Invalid ticket.** A non-existent ticket → rejection code.

---

## 📚 See also

* [order_send](order_send.md) · [order_close](order_close.md) · [opened_orders](../3.%20Positions_Orders/opened_orders.md)
* **Back to:** [Trading Operations — Overview](Trading_Operations.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
