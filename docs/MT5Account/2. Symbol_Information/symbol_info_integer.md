# Symbol Info — Integer

> Read **one** integer symbol property (digits, spread, trade mode, stops level, flags, times).

**API Information:**

* **Low-level API:** `MT5Account::symbol_info_integer(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolInfoInteger(SymbolInfoIntegerRequest) -> SymbolInfoIntegerReply`
* **Payload:** `SymbolInfoIntegerData` → `value: i64`

## 💬 Just the essentials

* **What it is.** MQL5 `SymbolInfoInteger(symbol, property)` — one integer property.
* **Why you need it.** `SYMBOL_DIGITS` scales every price you print; `SYMBOL_TRADE_STOPS_LEVEL` is the minimum SL/TP distance the broker enforces.
* **When to use.** Price formatting, SL/TP validation, checking whether a symbol is tradable.

---

## Method Signature

```rust
pub async fn symbol_info_integer(
    &self,
    request: pb::SymbolInfoIntegerRequest,
) -> Result<pb::SymbolInfoIntegerData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolInfoIntegerRequest {
  string symbol = 1;
  SymbolInfoIntegerProperty type = 2;   // Rust field: r#type (i32)
}
message SymbolInfoIntegerData { int64 value = 1; }
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol name. |
| `r#type` | `i32` (`SymbolInfoIntegerProperty`) | Which property. Pass `Prop::X as i32`. |

## ⬆️ Output — `SymbolInfoIntegerData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `value` | `i64` | The property value (booleans as 1/0, enums as their number, times as Unix seconds). |

---

## 🧱 `SymbolInfoIntegerProperty` — common values (from proto)

| Variant | Value | Meaning |
|---------|-------|---------|
| `SymbolSelect` | 7 | Is selected in Market Watch (bool) |
| `SymbolVisible` | 8 | Is visible in Market Watch (bool) |
| `SymbolTime` | 15 | Time of last quote (Unix seconds) |
| `SymbolTimeMsc` | 16 | Time of last quote (ms) |
| `SymbolDigits` | 17 | Decimal places in the price |
| `SymbolSpreadFloat` | 18 | Floating spread? (bool) |
| `SymbolSpread` | 19 | Current spread, points |
| `SymbolTradeCalcMode` | 21 | Contract price calc mode (enum) |
| `SymbolTradeMode` | 22 | Trade permission mode (enum) |
| `SymbolStartTime` | 23 | Trading start date (futures) |
| `SymbolExpirationTime` | 24 | Trading end date (futures) |
| `SymbolTradeStopsLevel` | 25 | Min SL/TP distance from price, points |
| `SymbolTradeFreezeLevel` | 26 | Freeze distance, points |
| `SymbolTradeExemode` | 27 | Deal execution mode (enum) |
| `SymbolFillingMode` | 32 | Allowed fill modes (bit flags) |
| `SymbolOrderMode` | 33 | Allowed order types (bit flags) |

> ~37 variants total (sector/industry, session counts, swap mode, option fields, …). Full set in the proto `SymbolInfoIntegerProperty` / the [ENUMs reference](../../All_Guides/ENUMS_USAGE_REFERENCE.md).

---

## 🧩 Notes & Tips

* **`SymbolDigits` formats prices.** Use it as the precision when printing bid/ask (`{:.*}` with `digits as usize`).
* **`SymbolTradeStopsLevel` gates SL/TP.** Stops closer than this many points are rejected — validate before `order_send`.
* **Booleans are 1/0.** `SymbolSelect`, `SymbolVisible`, `SymbolSpreadFloat` → compare `== 1`.
* **Enum properties return the number.** Decode `SymbolTradeMode` against the matching proto enum.

---

## 🔗 Usage Examples

### 1) Format prices with the symbol's digits

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::SymbolInfoIntegerProperty as I;

let digits = account
    .symbol_info_integer(pb::SymbolInfoIntegerRequest { symbol: "EURUSD".into(), r#type: I::SymbolDigits as i32 })
    .await?.value as usize;
let bid = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: "EURUSD".into() }).await?.bid;
println!("bid {bid:.*}", digits);
```

### 2) Validate an SL distance against stops level

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::SymbolInfoIntegerProperty as I;

let stops = account
    .symbol_info_integer(pb::SymbolInfoIntegerRequest { symbol: "EURUSD".into(), r#type: I::SymbolTradeStopsLevel as i32 })
    .await?.value;
let my_sl_points = 8;
if (my_sl_points as i64) < stops { println!("SL too tight: min {stops} points"); }
```

### 3) Current spread in points

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::SymbolInfoIntegerProperty as I;

let spread = account
    .symbol_info_integer(pb::SymbolInfoIntegerRequest { symbol: "EURUSD".into(), r#type: I::SymbolSpread as i32 })
    .await?.value;
println!("spread: {spread} points");
```

---

## ⚠️ Pitfalls

* **The field is `r#type`** (raw identifier).
* **Pass the enum as `i32`.**
* **Times are Unix seconds** (`SymbolTime`, `SymbolStartTime`) — convert before display.
* **Ignoring stops/freeze level.** The commonest cause of "invalid stops" order rejections.

---

## 🧪 Testing Suggestions

* **Digits.** EURUSD returns 5 (or 4); JPY pairs return 3 (or 2).
* **Spread ≥ 0.** And roughly `(ask − bid) / point`.
* **Selected flag.** After `symbol_select(sym, true)`, `SymbolSelect == 1`.

---

## 📚 See also

* [symbol_info_double](symbol_info_double.md) · [symbol_info_string](symbol_info_string.md) · [symbol_params_many](symbol_params_many.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
