# Symbol Info — Margin Rate

> The initial and maintenance margin **rates** for a symbol and order type — multipliers on the base margin.

**API Information:**

* **Low-level API:** `MT5Account::symbol_info_margin_rate(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolInfoMarginRate(SymbolInfoMarginRateRequest) -> SymbolInfoMarginRateReply`
* **Payload:** `SymbolInfoMarginRateData` → `maintenance_margin_rate`, `initial_margin_rate`

## 💬 Just the essentials

* **What it is.** MQL5 `SymbolInfoMarginRate(symbol, order_type, initial, maintenance)` — the rate pair.
* **Why you need it.** Margin can differ by direction (buy vs sell) and order type; these rates scale the reserved amount.
* **When to use.** Pre-trade margin estimates. For the actual currency amount, prefer [`order_calc_margin`](../5.%20Trading_Operations/order_calc_margin.md).

---

## Method Signature

```rust
pub async fn symbol_info_margin_rate(
    &self,
    request: pb::SymbolInfoMarginRateRequest,
) -> Result<pb::SymbolInfoMarginRateData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolInfoMarginRateRequest {
  string symbol = 1;
  EnumOrderType order_type = 2;   // Rust field: order_type (i32)
}
message SymbolInfoMarginRateData {
  double maintenance_margin_rate = 1;
  double initial_margin_rate = 2;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol name. |
| `order_type` | `i32` (`EnumOrderType`) | Order type (e.g. buy/sell). Pass `EnumOrderType::X as i32`. |

## ⬆️ Output — `SymbolInfoMarginRateData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `initial_margin_rate` | `f64` | Multiplier on the initial margin (deposit to open 1 lot). |
| `maintenance_margin_rate` | `f64` | Multiplier on the maintenance margin (min to keep 1 lot open). |

---

## 🧱 `EnumOrderType` (field `order_type`)

| Variant | Value | | Variant | Value |
|---------|-------|---|---------|-------|
| `OrderTypeBuy` | 0 | | `OrderTypeSellStop` | 5 |
| `OrderTypeSell` | 1 | | `OrderTypeBuyStopLimit` | 6 |
| `OrderTypeBuyLimit` | 2 | | `OrderTypeSellStopLimit` | 7 |
| `OrderTypeSellLimit` | 3 | | `OrderTypeCloseBy` | 8 |
| `OrderTypeBuyStop` | 4 | | | |

---

## 🧩 Notes & Tips

* **Rates, not amounts.** Multiply by the symbol's base margin (`SYMBOL_MARGIN_INITIAL` / `SYMBOL_MARGIN_MAINTENANCE` from [`symbol_info_double`](./symbol_info_double.md)) to get currency.
* **Direction-dependent.** Buy and sell can carry different rates — query the type you'll actually trade.
* **Want the real number?** [`order_calc_margin`](../5.%20Trading_Operations/order_calc_margin.md) returns the margin in account currency for a concrete volume and price.

---

## 🔗 Usage Examples

### 1) Rates for a buy

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::EnumOrderType as OT;

let r = account
    .symbol_info_margin_rate(pb::SymbolInfoMarginRateRequest {
        symbol: "EURUSD".into(),
        order_type: OT::OrderTypeBuy as i32,
    })
    .await?;
println!("initial rate {}, maintenance rate {}", r.initial_margin_rate, r.maintenance_margin_rate);
```

### 2) Compare buy vs sell

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::EnumOrderType as OT;

async fn rate(account: &metarpc_mt5::MT5Account, sym: &str, t: OT) -> f64 {
    account.symbol_info_margin_rate(pb::SymbolInfoMarginRateRequest { symbol: sym.into(), order_type: t as i32 })
        .await.map(|d| d.initial_margin_rate).unwrap_or(0.0)
}
let buy = rate(&account, "EURUSD", OT::OrderTypeBuy).await;
let sell = rate(&account, "EURUSD", OT::OrderTypeSell).await;
println!("initial: buy {buy}, sell {sell}");
```

### 3) Estimate initial margin (rate × base)

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::{EnumOrderType as OT, SymbolInfoDoubleProperty as D};

let rate = account.symbol_info_margin_rate(pb::SymbolInfoMarginRateRequest { symbol: "EURUSD".into(), order_type: OT::OrderTypeBuy as i32 }).await?.initial_margin_rate;
let base = account.symbol_info_double(pb::SymbolInfoDoubleRequest { symbol: "EURUSD".into(), r#type: D::SymbolMarginInitial as i32 }).await?.value;
println!("≈ initial margin per lot: {:.2}", rate * base);  // prefer order_calc_margin for the exact figure
```

---

## ⚠️ Pitfalls

* **Rate ≠ money.** A rate of `1.0` doesn't mean "1 unit of currency" — multiply by the base margin, or use `order_calc_margin`.
* **Pass `order_type` as `i32`.**
* **Zero rates.** Some symbols/types return `0.0` where the broker uses the base margin as-is.

---

## 🧪 Testing Suggestions

* **FX majors.** Initial rate is typically `1.0` (full base margin).
* **Direction symmetry.** For plain FX, buy and sell rates usually match.

---

## 📚 See also

* [order_calc_margin](../5.%20Trading_Operations/order_calc_margin.md) · [symbol_info_double](symbol_info_double.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
