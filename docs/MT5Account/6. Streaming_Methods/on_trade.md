# On Trade (stream)

> Trade events — whenever orders, positions or deals change — with an account snapshot attached.

**API Information:**

* **Low-level API:** `MT5Account::on_trade(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.SubscriptionService`
* **RPC:** `OnTrade(OnTradeRequest) -> stream OnTradeReply`
* **Item payload:** `OnTradeData`

## 💬 Just the essentials

* **What it is.** The MT5 `OnTrade` event feed — pushes when your trading state changes.
* **Why you need it.** React to fills, closes and modifications without polling `opened_orders`.
* **When to use.** Event-driven bots and live blotters. For the raw transaction detail, see [`on_trade_transaction`](./on_trade_transaction.md).

---

## Method Signature

```rust
pub async fn on_trade(
    &self,
    request: pb::OnTradeRequest,
) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnTradeData, MT5Error>>, MT5Error>
```

**Request / item:**

```protobuf
message OnTradeRequest {}   // no fields

message OnTradeData {
  Mt5SubEnumEventGroupType type = 1;          // Rust field: r#type (i32)
  OnTadeEventData event_data = 2;             // optional
  OnEventAccountInfo account_info = 3;        // optional
  string terminal_instance_guid_id = 4;
}

message OnEventAccountInfo {
  double balance = 1; double credit = 2; double equity = 3;
  double margin = 4; double free_margin = 5; double profit = 6;
  double margin_level = 7; int64 login = 8;
}
```

---

## 🔽 Input

None — pass `pb::OnTradeRequest {}`.

## ⬆️ Item — `OnTradeData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `r#type` | `i32` (`Mt5SubEnumEventGroupType`) | Which event group changed. |
| `event_data` | `Option<OnTadeEventData>` | Details of the change. |
| `account_info` | `Option<OnEventAccountInfo>` | Balance/equity/margin snapshot at the event. |
| `terminal_instance_guid_id` | `String` | Originating terminal session. |

`OnEventAccountInfo` gives live `balance`, `equity`, `margin`, `free_margin`, `profit`, `margin_level` — a cheap way to keep an account header fresh on every trade event.

---

## 🧱 `Mt5SubEnumEventGroupType` (field `r#type`)

| Variant | Value | Meaning |
|---------|-------|---------|
| `OrderProfit` | 0 | Position-profit group update |
| `OrderUpdate` | 1 | Order / position change group |

---

## 🧩 Notes & Tips

* **Event-driven.** Frames arrive only when something changes — quiet periods are silent.
* **Both sub-messages are `Option`.** Guard before use.
* **`r#type` is a raw identifier** (Rust keyword).
* **Snapshot included.** Use `account_info` to update equity/margin without a separate `account_summary` call.

---

## 🔗 Usage Examples

### 1) Log trade events with equity

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_trade(pb::OnTradeRequest {}).await?;
while let Some(item) = s.next().await {
    match item {
        Ok(d) => {
            if let Some(a) = d.account_info {
                println!("event type {} | equity {:.2} free {:.2}", d.r#type, a.equity, a.free_margin);
            }
        }
        Err(e) => { eprintln!("stream error: {e}"); break; }
    }
}
```

### 2) Trigger a re-sync on any event

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_trade(pb::OnTradeRequest {}).await?;
while let Some(Ok(_)) = s.next().await {
    // something changed — refresh the full picture
    let data = account.opened_orders(pb::OpenedOrdersRequest {
        input_sort_mode: pb::Bmt5EnumOpenedOrderSortType::Bmt5OpenedOrderSortByOpenTimeAsc as i32,
    }).await?;
    println!("now {} open position(s)", data.position_infos.len());
}
```

### 3) Watch margin level drop

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_trade(pb::OnTradeRequest {}).await?;
while let Some(Ok(d)) = s.next().await {
    if let Some(a) = d.account_info {
        if a.margin_level != 0.0 && a.margin_level < 150.0 {
            println!("[ALERT] margin level {:.0}%", a.margin_level);
        }
    }
}
```

---

## ⚠️ Pitfalls

* **Expecting periodic frames.** This is event-driven, not a timer — use `on_position_profit` for steady P/L ticks.
* **Unwrapping `Option` fields.**
* **Heavy work inline.** Offload to a spawned task to keep consuming promptly.

---

## 🧪 Testing Suggestions

* **Open a trade.** A frame arrives soon after `order_send`; `account_info.margin` rises.
* **Close it.** Another frame; `free_margin` recovers.

---

## 📚 See also

* [on_trade_transaction](on_trade_transaction.md) · [opened_orders](../3.%20Positions_Orders/opened_orders.md)
* **Back to:** [Streaming Methods — Overview](Streaming_Methods.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
