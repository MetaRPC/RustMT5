# Symbol Info — String

> Read **one** string symbol property (base/profit/margin currency, description, path, ISIN, …).

**API Information:**

* **Low-level API:** `MT5Account::symbol_info_string(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolInfoString(SymbolInfoStringRequest) -> SymbolInfoStringReply`
* **Payload:** `SymbolInfoStringData` → `value: String`

## 💬 Just the essentials

* **What it is.** MQL5 `SymbolInfoString(symbol, property)` — one text property.
* **Why you need it.** Currencies for P/L conversion, a human description for UI, the Market-Watch path for grouping.
* **When to use.** Labelling instruments and figuring out which currency a symbol's profit is in.

---

## Method Signature

```rust
pub async fn symbol_info_string(
    &self,
    request: pb::SymbolInfoStringRequest,
) -> Result<pb::SymbolInfoStringData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolInfoStringRequest {
  string symbol = 1;
  SymbolInfoStringProperty type = 2;   // Rust field: r#type (i32)
}
message SymbolInfoStringData { string value = 1; }
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol name. |
| `r#type` | `i32` (`SymbolInfoStringProperty`) | Which property. Pass `Prop::X as i32`. |

## ⬆️ Output — `SymbolInfoStringData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `value` | `String` | The property's text value. |

---

## 🧱 `SymbolInfoStringProperty` (from proto — full set)

| Variant | Value | Meaning |
|---------|-------|---------|
| `SymbolBasis` | 0 | Underlying asset of a derivative |
| `SymbolCategory` | 1 | Category/sector name |
| `SymbolCountry` | 2 | Country |
| `SymbolSectorName` | 3 | Economic sector |
| `SymbolIndustryName` | 4 | Industry branch |
| `SymbolCurrencyBase` | 5 | Base currency |
| `SymbolCurrencyProfit` | 6 | Profit currency |
| `SymbolCurrencyMargin` | 7 | Margin currency |
| `SymbolBank` | 8 | Quote feeder |
| `SymbolDescription` | 9 | Human-readable description |
| `SymbolExchange` | 10 | Exchange name |
| `SymbolFormula` | 11 | Custom-symbol pricing formula |
| `SymbolIsin` | 12 | ISIN code |
| `SymbolPage` | 13 | Info web page |
| `SymbolPath` | 14 | Path in the symbol tree |

---

## 🧩 Notes & Tips

* **Profit currency matters for P/L.** If `SymbolCurrencyProfit` differs from your account currency, P/L needs conversion.
* **`SymbolDescription` for UI.** Friendlier than the raw symbol code.
* **`SymbolPath` for grouping.** E.g. `Forex\Majors\EURUSD` — split on `\` to build a tree.
* **Fields can be empty.** Not every broker fills every property.

---

## 🔗 Usage Examples

### 1) Currencies of a symbol

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::SymbolInfoStringProperty as S;

async fn str_prop(account: &metarpc_mt5::MT5Account, sym: &str, p: S) -> String {
    account.symbol_info_string(pb::SymbolInfoStringRequest { symbol: sym.into(), r#type: p as i32 })
        .await.map(|d| d.value).unwrap_or_default()
}
let base   = str_prop(&account, "EURUSD", S::SymbolCurrencyBase).await;
let profit = str_prop(&account, "EURUSD", S::SymbolCurrencyProfit).await;
println!("base {base}, profit {profit}");
```

### 2) Description for a label

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::SymbolInfoStringProperty as S;

let desc = account
    .symbol_info_string(pb::SymbolInfoStringRequest { symbol: "XAUUSD".into(), r#type: S::SymbolDescription as i32 })
    .await?.value;
println!("XAUUSD — {desc}");
```

### 3) Does profit currency match the account?

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::SymbolInfoStringProperty as S;

let prof = account.symbol_info_string(pb::SymbolInfoStringRequest { symbol: "USDJPY".into(), r#type: S::SymbolCurrencyProfit as i32 }).await?.value;
let acct = account.account_summary().await?.account_currency;
if prof != acct { println!("P/L in {prof}, account in {acct} — conversion applies"); }
```

---

## ⚠️ Pitfalls

* **The field is `r#type`** (raw identifier).
* **Pass the enum as `i32`.**
* **Empty strings are valid.** Guard before using as a label.

---

## 🧪 Testing Suggestions

* **EURUSD.** `SymbolCurrencyBase == "EUR"`, `SymbolCurrencyProfit == "USD"`.
* **Description present.** Major symbols return a non-empty `SymbolDescription`.

---

## 📚 See also

* [symbol_info_double](symbol_info_double.md) · [symbol_info_integer](symbol_info_integer.md) · [symbol_params_many](symbol_params_many.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
