# Order Check

> Validate a trade request against the live account **before** sending it — margin, balance, equity and free margin after the hypothetical deal.

**API Information:**

* **Low-level API:** `MT5Account::order_check(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.TradeFunctions`
* **RPC:** `OrderCheck(OrderCheckRequest) -> OrderCheckReply`
* **Payload:** `OrderCheckData` → `mql_trade_check_result: Option<MrpcMqlTradeCheckResult>`

## 💬 Just the essentials

* **What it is.** MQL5 `OrderCheck` — a dry run of `order_send`.
* **Why you need it.** Catch "not enough money" / "invalid stops" before committing.
* **When to use.** Right before `order_send`, especially for larger sizes.

---

## Method Signature

```rust
pub async fn order_check(
    &self,
    request: pb::OrderCheckRequest,
) -> Result<pb::OrderCheckData, MT5Error>
```

**Request / Reply:**

```protobuf
message OrderCheckRequest {
  MrpcMqlTradeRequest mql_trade_request = 1;   // the trade to validate
}
message OrderCheckData {
  MrpcMqlTradeCheckResult mql_trade_check_result = 1;
}
```

**`MrpcMqlTradeRequest` (key fields):**

```protobuf
message MrpcMqlTradeRequest {
  MrpcEnumTradeRequestActions action = 1;
  uint64 expert_advisor_magic_number = 2;
  uint64 order = 3;
  string symbol = 4;
  double volume = 5;
  double price = 6;
  double stop_limit = 7;
  double stop_loss = 8;
  // … take_profit, deviation, type, type_filling, type_time, expiration, comment …
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `mql_trade_request` | `Option<MrpcMqlTradeRequest>` | The hypothetical trade (`Some(..)`). Fill `symbol`, `volume`, `price`, `stop_loss`, `take_profit`, order `type`, etc. |

## ⬆️ Output — `MrpcMqlTradeCheckResult`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `returned_code` | `u32` | Validation retcode (`10009` = would succeed). |
| `balance_after_deal` | `f64` | Projected balance. |
| `equity_after_deal` | `f64` | Projected equity. |
| `profit` | `f64` | Projected floating profit. |
| `margin` | `f64` | Margin the deal needs. |
| `free_margin` | `f64` | Projected free margin. |
| `margin_level` | `f64` | Projected margin level (%). |
| `comment` | `String` | Description / error. |

---

## 🧱 `MrpcEnumTradeRequestActions` (field `action`)

| Variant | Value | Meaning |
|---------|-------|---------|
| `TradeActionDeal` | 0 | Market deal (open/close at market) |
| `TradeActionPending` | 1 | Place a pending order |
| `TradeActionSltp` | 2 | Modify a position's SL/TP |
| `TradeActionModify` | 3 | Modify a pending order |
| `TradeActionRemove` | 4 | Delete a pending order |
| `TradeActionCloseBy` | 5 | Close a position by an opposite one |

> The request also carries `order_type` (`EnumOrderTypeTf`), `type_filling` and `type_time` enums — set the ones your check needs; `..Default::default()` leaves the rest zeroed.

---

## 🧩 Notes & Tips

* **Preview, not execution.** Nothing is placed; it only validates and projects.
* **Check margin headroom.** If `free_margin` would go negative or `margin_level` too low, don't send.
* **`returned_code` here mirrors send codes.** `10009` means the real `order_send` would be accepted (market conditions permitting).
* **Result is `Option`.** Unwrap `mql_trade_check_result` before reading.

---

## 🔗 Usage Examples

### 1) Validate before sending

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let req = pb::MrpcMqlTradeRequest {
    symbol: "EURUSD".into(),
    volume: 0.50,
    price: 1.08500,
    stop_loss: 1.08300,
    take_profit: 1.09000,
    ..Default::default()
};
let out = account.order_check(pb::OrderCheckRequest { mql_trade_request: Some(req) }).await?;
if let Some(r) = out.mql_trade_check_result {
    if r.returned_code == 10009 {
        println!("OK — needs margin {:.2}, free after {:.2}", r.margin, r.free_margin);
    } else {
        println!("would fail: {} ({})", r.returned_code, r.comment);
    }
}
```

### 2) Guard a send with a margin-level floor

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let out = account.order_check(pb::OrderCheckRequest {
    mql_trade_request: Some(pb::MrpcMqlTradeRequest { symbol: "EURUSD".into(), volume: 1.0, price: 1.085, ..Default::default() }),
}).await?;
let ok = out.mql_trade_check_result
    .map(|r| r.returned_code == 10009 && r.margin_level > 200.0)
    .unwrap_or(false);
println!("safe to send: {ok}");
```

### 3) Projected equity impact

```rust
if let Some(r) = out.mql_trade_check_result {
    println!("equity now→after: {:.2}", r.equity_after_deal);
}
```

---

## ⚠️ Pitfalls

* **Forgetting `Some(..)`.** `mql_trade_request` is an `Option`.
* **Treating it as a send.** It never opens anything.
* **Not unwrapping the result option.**
* **`..Default::default()`** leaves other fields zeroed — set the ones that matter for your check.

---

## 🧪 Testing Suggestions

* **Oversized order.** A huge `volume` yields a non-10009 code and negative free margin.
* **Reasonable order.** `returned_code == 10009`, `margin > 0`.

---

## 📚 See also

* [order_send](order_send.md) · [order_calc_margin](order_calc_margin.md) · [Return Codes](../../All_Guides/RETURN_CODES_REFERENCE.md)
* **Back to:** [Trading Operations — Overview](Trading_Operations.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
