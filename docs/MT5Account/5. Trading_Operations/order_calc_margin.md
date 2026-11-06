# Order Calc Margin

> How much margin a hypothetical order would require — before you place it.

**API Information:**

* **Low-level API:** `MT5Account::order_calc_margin(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.TradeFunctions`
* **RPC:** `OrderCalcMargin(OrderCalcMarginRequest) -> OrderCalcMarginReply`
* **Payload:** `OrderCalcMarginData` → `margin: f64`

## 💬 Just the essentials

* **What it is.** MQL5 `OrderCalcMargin` — margin in account currency for a would-be order.
* **Why you need it.** Size positions to your free margin; avoid "no money" rejections.
* **When to use.** During sizing, before `order_send`. More precise than multiplying margin rates yourself.

---

## Method Signature

```rust
pub async fn order_calc_margin(
    &self,
    request: pb::OrderCalcMarginRequest,
) -> Result<pb::OrderCalcMarginData, MT5Error>
```

**Request / Reply:**

```protobuf
message OrderCalcMarginRequest {
  string symbol = 1;
  EnumOrderTypeTf order_type = 2;   // Rust field: order_type (i32)
  double volume = 3;                // lots
  double open_price = 4;            // hypothetical entry
}
message OrderCalcMarginData { double margin = 1; }
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol. |
| `order_type` | `i32` (`EnumOrderTypeTf`) | Order type. Pass `EnumOrderTypeTf::X as i32`. |
| `volume` | `f64` | Lots. |
| `open_price` | `f64` | Entry price to assume. |

### `EnumOrderTypeTf` (common)

| Variant | Value |
|---------|-------|
| `OrderTypeTfBuy` | 0 |
| `OrderTypeTfSell` | 1 |
| `OrderTypeTfBuyLimit` | 2 |
| `OrderTypeTfSellLimit` | 3 |

## ⬆️ Output — `OrderCalcMarginData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `margin` | `f64` | Required margin in the account's deposit currency. |

---

## 🧩 Notes & Tips

* **Account currency.** The result is in your deposit currency, ready to compare with `free_margin`.
* **Assumes a clean account.** The calc ignores your existing positions/orders (hedging nets may make the real requirement lower).
* **Sizing loop.** Try a volume, compare `margin` to free margin, adjust; or divide a target margin budget by the per-lot margin.

---

## 🔗 Usage Examples

### 1) Margin for 1 lot EURUSD

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::EnumOrderTypeTf as TF;

let price = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: "EURUSD".into() }).await?.ask;
let m = account.order_calc_margin(pb::OrderCalcMarginRequest {
    symbol: "EURUSD".into(),
    order_type: TF::OrderTypeTfBuy as i32,
    volume: 1.0,
    open_price: price,
}).await?.margin;
println!("margin for 1.0 lot: {m:.2}");
```

### 2) Largest size that fits free margin

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::{EnumOrderTypeTf as TF, AccountInfoDoublePropertyType as P};

let price = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: "EURUSD".into() }).await?.ask;
let per_lot = account.order_calc_margin(pb::OrderCalcMarginRequest {
    symbol: "EURUSD".into(), order_type: TF::OrderTypeTfBuy as i32, volume: 1.0, open_price: price,
}).await?.margin;
let free = account.account_info_double(pb::AccountInfoDoubleRequest { property_id: P::AccountMarginFree as i32 }).await?.requested_value;
let max_lots = if per_lot > 0.0 { (free * 0.9) / per_lot } else { 0.0 };  // keep 10% buffer
println!("max ≈ {max_lots:.2} lot (round to volume step)");
```

### 3) Margin for a sell

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::EnumOrderTypeTf as TF;

let m = account.order_calc_margin(pb::OrderCalcMarginRequest {
    symbol: "XAUUSD".into(), order_type: TF::OrderTypeTfSell as i32, volume: 0.10, open_price: 2400.0,
}).await?.margin;
println!("margin: {m:.2}");
```

---

## ⚠️ Pitfalls

* **Enum as `i32`** (`TF::OrderTypeTfBuy as i32`).
* **Ignoring the buffer.** Sizing to 100% of free margin leaves no room — keep a cushion.
* **Hedging accounts.** The real requirement may be lower than this clean-account estimate.

---

## 🧪 Testing Suggestions

* **Scales with volume.** `margin(2.0) ≈ 2 × margin(1.0)`.
* **Leverage effect.** Higher account leverage → lower margin for the same volume.

---

## 📚 See also

* [order_calc_profit](order_calc_profit.md) · [order_check](order_check.md) · [symbol_info_margin_rate](../2.%20Symbol_Information/symbol_info_margin_rate.md)
* **Back to:** [Trading Operations — Overview](Trading_Operations.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
