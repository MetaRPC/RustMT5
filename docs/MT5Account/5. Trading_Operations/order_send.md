# Order Send (RECOMMENDED)

> Place a market or pending order, optionally with SL/TP, slippage, comment and magic number.

**API Information:**

* **Low-level API:** `MT5Account::order_send(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.TradeFunctions`
* **RPC:** `OrderSend(OrderSendRequest) -> OrderSendReply`
* **Payload:** `OrderSendData` → `returned_code: u32`, `deal`, `order`, …

## 💬 Just the essentials

* **What it is.** MQL5 `OrderSend` — the one call that opens trades.
* **Why you need it.** Market entries and pending orders, with SL/TP in one shot.
* **When to use.** Entering a trade. For friendlier buy/sell-by-pips, use `MT5Sugar`.

---

## Method Signature

```rust
pub async fn order_send(
    &self,
    request: pb::OrderSendRequest,
) -> Result<pb::OrderSendData, MT5Error>
```

**Request message (key fields):**

```protobuf
message OrderSendRequest {
  string symbol = 1;                     // REQUIRED
  Tmt5EnumOrderType operation = 2;       // Rust field: operation (i32)
  double volume = 3;                     // lots
  optional double price = 4;             // required for pendings; market uses current
  optional uint64 slippage = 5;          // max deviation, points
  optional double stop_loss = 6;
  optional double take_profit = 7;
  optional string comment = 8;
  optional uint64 expert_id = 9;         // magic number
  optional double stop_limit_price = 10;
  optional Tmt5EnumOrderTypeTime expiration_time_type = 11;
  optional google.protobuf.Timestamp expiration_time = 12;
  optional int32 quotes_wait_ms = 13;    // wait for first quote (default 5000, max 60000)
}
```

**Reply payload `OrderSendData` (key fields):** `returned_code: u32`, `deal: u64`, `order: u64`, `volume: f64`, `price: f64`, `bid: f64`, `ask: f64`, `comment: String`, `returned_string_code`, `returned_code_description`.

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol (required). |
| `operation` | `i32` (`Tmt5EnumOrderType`) | Order type — see enum. |
| `volume` | `f64` | Lots. |
| `price` | `Option<f64>` | Required for pendings; omit (`None`) for market. |
| `slippage` | `Option<u64>` | Max price deviation, points. |
| `stop_loss` / `take_profit` | `Option<f64>` | Absolute price levels. |
| `comment` | `Option<String>` | Free text. |
| `expert_id` | `Option<u64>` | Magic number. |
| `stop_limit_price` | `Option<f64>` | For stop-limit orders. |
| `expiration_time_type` / `expiration_time` | `Option<..>` | Pending expiry policy. |
| `quotes_wait_ms` | `Option<i32>` | How long to wait for the first quote. |

### `Tmt5EnumOrderType`

| Variant | Value | Variant | Value |
|---------|-------|---------|-------|
| `Tmt5OrderTypeBuy` | 0 | `Tmt5OrderTypeSellLimit` | 3 |
| `Tmt5OrderTypeSell` | 1 | `Tmt5OrderTypeBuyStop` | 4 |
| `Tmt5OrderTypeBuyLimit` | 2 | `Tmt5OrderTypeSellStop` | 5 |
| `Tmt5OrderTypeBuyStopLimit` | 6 | `Tmt5OrderTypeSellStopLimit` | 7 |

## ⬆️ Output — `OrderSendData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `returned_code` | `u32` | **`10009` = success** (`TRADE_RETCODE_DONE`). |
| `deal` | `u64` | Deal ticket (if executed). |
| `order` | `u64` | Order ticket (if placed). |
| `volume` | `f64` | Filled volume. |
| `price` | `f64` | Fill price. |
| `bid` / `ask` | `f64` | Prices at execution. |
| `comment` | `String` | Broker comment. |
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

* **Check `returned_code == 10009`.** Don't assume success — read the code every time.
* **Market vs pending.** Market: `operation` Buy/Sell, `price: None`. Pending: a limit/stop type **and** a `price`.
* **SL/TP are absolute prices**, not distances — compute them from the entry and `point`.
* **Respect stops level.** SL/TP closer than `SYMBOL_TRADE_STOPS_LEVEL` are rejected (see [`symbol_info_integer`](../2.%20Symbol_Information/symbol_info_integer.md)).

---

## 🔗 Usage Examples

### 1) Market buy with SL/TP

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::Tmt5EnumOrderType as OT;

let tick = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: "EURUSD".into() }).await?;
let point = account.symbol_info_double(pb::SymbolInfoDoubleRequest {
    symbol: "EURUSD".into(), r#type: pb::SymbolInfoDoubleProperty::SymbolPoint as i32,
}).await?.value;

let res = account.order_send(pb::OrderSendRequest {
    symbol: "EURUSD".into(),
    operation: OT::Tmt5OrderTypeBuy as i32,
    volume: 0.10,
    price: None,                                    // market
    slippage: Some(20),
    stop_loss: Some(tick.ask - 200.0 * point),      // 20 pips
    take_profit: Some(tick.ask + 400.0 * point),    // 40 pips
    comment: Some("demo entry".into()),
    expert_id: Some(12345),
    stop_limit_price: None, expiration_time_type: None, expiration_time: None, quotes_wait_ms: None,
}).await?;

if res.returned_code == 10009 { println!("opened, order #{}", res.order); }
else { println!("rejected: {} ({})", res.returned_code, res.returned_code_description); }
```

### 2) Buy limit (pending)

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::Tmt5EnumOrderType as OT;

let bid = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: "EURUSD".into() }).await?.bid;
let res = account.order_send(pb::OrderSendRequest {
    symbol: "EURUSD".into(),
    operation: OT::Tmt5OrderTypeBuyLimit as i32,
    volume: 0.10,
    price: Some(bid - 0.0010),   // 10 pips below market
    slippage: None, stop_loss: None, take_profit: None,
    comment: Some("limit".into()), expert_id: Some(12345),
    stop_limit_price: None, expiration_time_type: None, expiration_time: None, quotes_wait_ms: None,
}).await?;
println!("pending #{} code {}", res.order, res.returned_code);
```

### 3) Minimal market sell

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::Tmt5EnumOrderType as OT;

let res = account.order_send(pb::OrderSendRequest {
    symbol: "EURUSD".into(), operation: OT::Tmt5OrderTypeSell as i32, volume: 0.01,
    price: None, slippage: Some(30), stop_loss: None, take_profit: None,
    comment: None, expert_id: None, stop_limit_price: None,
    expiration_time_type: None, expiration_time: None, quotes_wait_ms: None,
}).await?;
println!("code {}", res.returned_code);
```

---

## ⚠️ Pitfalls

* **Ignoring `returned_code`.** A non-10009 code means it didn't open as asked.
* **Pending without `price`.** Limits/stops require `price: Some(..)`.
* **SL/TP as distances.** They're absolute prices.
* **Invalid stops.** Too-tight SL/TP → rejection; check the stops level first.
* **Enum as `i32`.** `operation: OT::Tmt5OrderTypeBuy as i32`.

---

## 🧪 Testing Suggestions

* **Demo market buy.** `returned_code == 10009`, `order > 0`; then it shows up in `opened_orders`.
* **Bad volume.** Below `VolumeMin` → rejection with a descriptive code.
* **Too-tight SL.** Rejected per stops level.

---

## 📚 See also

* [order_modify](order_modify.md) · [order_close](order_close.md) · [order_check](order_check.md) · [Return Codes](../../All_Guides/RETURN_CODES_REFERENCE.md)
* **Back to:** [Trading Operations — Overview](Trading_Operations.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
