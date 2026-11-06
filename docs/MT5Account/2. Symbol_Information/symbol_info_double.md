# Symbol Info — Double

> Read **one** floating-point symbol property (point size, bid/ask, tick value/size, volume limits, swaps, margins).

**API Information:**

* **Low-level API:** `MT5Account::symbol_info_double(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolInfoDouble(SymbolInfoDoubleRequest) -> SymbolInfoDoubleReply`
* **Payload:** `SymbolInfoDoubleData` → `value: f64`

## 💬 Just the essentials

* **What it is.** MQL5 `SymbolInfoDouble(symbol, property)` — one `double` property.
* **Why you need it.** `point`, `tick_value`, `tick_size`, `volume_min/step` are the constants pip math and lot rounding depend on.
* **When to use.** Sizing and price rounding. For many properties at once, use [`symbol_params_many`](./symbol_params_many.md).

---

## Method Signature

```rust
pub async fn symbol_info_double(
    &self,
    request: pb::SymbolInfoDoubleRequest,
) -> Result<pb::SymbolInfoDoubleData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolInfoDoubleRequest {
  string symbol = 1;
  SymbolInfoDoubleProperty type = 2;   // Rust field: r#type (i32)
}
message SymbolInfoDoubleData { double value = 1; }
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol name. |
| `r#type` | `i32` (`SymbolInfoDoubleProperty`) | Which property. Pass `Prop::X as i32`. Note the raw-ident `r#type`. |

## ⬆️ Output — `SymbolInfoDoubleData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `value` | `f64` | The property value. |

---

## 🧱 `SymbolInfoDoubleProperty` — common values (from proto)

| Variant | Value | Meaning |
|---------|-------|---------|
| `SymbolBid` | 0 | Current bid |
| `SymbolAsk` | 3 | Current ask |
| `SymbolLast` | 6 | Last deal price |
| `SymbolPoint` | 13 | Point size (smallest price step) |
| `SymbolTradeTickValue` | 14 | Tick value (profit) in deposit currency |
| `SymbolTradeTickValueProfit` | 15 | Tick value for a winning position |
| `SymbolTradeTickValueLoss` | 16 | Tick value for a losing position |
| `SymbolTradeTickSize` | 17 | Minimal price change |
| `SymbolTradeContractSize` | 18 | Contract size (e.g. 100000 for FX) |
| `SymbolVolumeMin` | 22 | Minimal lot |
| `SymbolVolumeMax` | 23 | Maximal lot |
| `SymbolVolumeStep` | 24 | Lot step |
| `SymbolVolumeLimit` | 25 | Max aggregate volume per direction |
| `SymbolSwapLong` | 26 | Long swap |
| `SymbolSwapShort` | 27 | Short swap |
| `SymbolMarginInitial` | 35 | Initial margin per lot |
| `SymbolMarginMaintenance` | 36 | Maintenance margin per lot |

> This enum has ~40 variants (session open/close, day highs/lows, swaps per weekday, option strike, …). The full set lives in the proto `SymbolInfoDoubleProperty` and the [ENUMs reference](../../All_Guides/ENUMS_USAGE_REFERENCE.md).

---

## 🧩 Notes & Tips

* **Pip math needs `SymbolPoint` + digits.** A "pip" is usually `10 × point` on 5-digit FX; read `SYMBOL_DIGITS` via [`symbol_info_integer`](./symbol_info_integer.md).
* **Lot rounding uses `VolumeMin`/`VolumeStep`/`VolumeMax`.** Round computed lots to the step and clamp to [min, max].
* **`TickValue` for P/L.** Money-per-tick-per-lot; the P/L-focused [`tick_value_with_size`](../3.%20Positions_Orders/tick_value_with_size.md) bundles value + size.

---

## 🔗 Usage Examples

### 1) Point size

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::SymbolInfoDoubleProperty as D;

let point = account
    .symbol_info_double(pb::SymbolInfoDoubleRequest { symbol: "EURUSD".into(), r#type: D::SymbolPoint as i32 })
    .await?.value;
println!("point = {point}");  // e.g. 0.00001
```

### 2) Volume constraints for lot rounding

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::SymbolInfoDoubleProperty as D;

async fn vol(account: &metarpc_mt5::MT5Account, sym: &str, p: D) -> f64 {
    account.symbol_info_double(pb::SymbolInfoDoubleRequest { symbol: sym.into(), r#type: p as i32 })
        .await.map(|d| d.value).unwrap_or(0.0)
}
let (min, step, max) = (
    vol(&account, "EURUSD", D::SymbolVolumeMin).await,
    vol(&account, "EURUSD", D::SymbolVolumeStep).await,
    vol(&account, "EURUSD", D::SymbolVolumeMax).await,
);
let raw = 0.137;
let lots = ((raw / step).round() * step).clamp(min, max);
println!("rounded lots: {lots:.2}");
```

### 3) Contract size & tick value

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::SymbolInfoDoubleProperty as D;

let cs = account.symbol_info_double(pb::SymbolInfoDoubleRequest { symbol: "EURUSD".into(), r#type: D::SymbolTradeContractSize as i32 }).await?.value;
let tv = account.symbol_info_double(pb::SymbolInfoDoubleRequest { symbol: "EURUSD".into(), r#type: D::SymbolTradeTickValue as i32 }).await?.value;
println!("contract {cs}, tick value {tv}");
```

---

## ⚠️ Pitfalls

* **The field is `r#type`.** It's a Rust keyword; use the raw identifier `r#type:` in the struct literal.
* **Pass the enum as `i32`** (`D::SymbolPoint as i32`).
* **Point ≠ pip.** On 5-digit quotes a pip is 10 points — don't conflate them.
* **One property per call.** Many properties → `symbol_params_many`.

---

## 🧪 Testing Suggestions

* **Point sanity.** EURUSD `SymbolPoint` is `0.00001` (5-digit) or `0.0001` (4-digit).
* **Bid parity.** `SymbolBid` ≈ `symbol_info_tick().bid`.
* **Volume rule.** `VolumeMin <= VolumeStep × k <= VolumeMax`.

---

## 📚 See also

* [symbol_info_integer](symbol_info_integer.md) · [symbol_info_string](symbol_info_string.md) · [symbol_params_many](symbol_params_many.md) · [tick_value_with_size](../3.%20Positions_Orders/tick_value_with_size.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
