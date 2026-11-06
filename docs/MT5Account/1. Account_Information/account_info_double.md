# Account Info — Double

> Read **one** floating-point account property (balance, equity, margin, profit, …) by its property id.

**API Information:**

* **Low-level API:** `MT5Account::account_info_double(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.AccountInformation`
* **RPC:** `AccountInfoDouble(AccountInfoDoubleRequest) -> AccountInfoDoubleReply`
* **Payload:** `AccountInfoDoubleData` → single field `requested_value: f64`

## 💬 Just the essentials

* **What it is.** The MQL5 `AccountInfoDouble(property)` call — one `double` property at a time.
* **Why you need it.** The money properties **not** in [`account_summary`](./account_summary.md) live here: margin, free margin, margin level, live profit.
* **When to use.** A single live value — e.g. check free margin before sizing a trade. For the whole picture at once, prefer `account_summary()`.

---

## 🎯 Purpose

* Read margin / free margin / margin level (missing from the summary)
* Read live `ACCOUNT_PROFIT` (floating P/L)
* Read any single `double` account property on demand

---

## Method Signature

```rust
pub async fn account_info_double(
    &self,
    request: pb::AccountInfoDoubleRequest,
) -> Result<pb::AccountInfoDoubleData, MT5Error>
```

**Request message:**

```protobuf
message AccountInfoDoubleRequest {
  AccountInfoDoublePropertyType property_id = 1;
}
```

**Reply message:**

```protobuf
message AccountInfoDoubleReply {
  oneof response {
    AccountInfoDoubleData data = 1;
    Error error = 2;
  }
}

message AccountInfoDoubleData {
  double requested_value = 1;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `property_id` | `i32` (`AccountInfoDoublePropertyType`) | Which `double` property to read. Pass `PropertyType::X as i32`. |

---

## ⬆️ Output — `AccountInfoDoubleData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `requested_value` | `f64` | The property's value, in the account's deposit currency (or percent for margin level). |

---

## 🧱 `AccountInfoDoublePropertyType` (from proto)

| Variant | Value | Meaning |
|---------|-------|---------|
| `AccountBalance` | 0 | Balance in deposit currency |
| `AccountCredit` | 1 | Credit in deposit currency |
| `AccountProfit` | 2 | Current floating profit |
| `AccountEquity` | 3 | Equity in deposit currency |
| `AccountMargin` | 4 | Margin used |
| `AccountMarginFree` | 5 | Free margin |
| `AccountMarginLevel` | 6 | Margin level, **percent** |
| `AccountMarginSoCall` | 7 | Margin-call level (percent or currency, per SO mode) |
| `AccountMarginSoSo` | 8 | Stop-out level (percent or currency, per SO mode) |
| `AccountMarginInitial` | 9 | Initial margin reserved for pending orders |
| `AccountMarginMaintenance` | 10 | Maintenance margin for open positions |
| `AccountAssets` | 11 | Current assets |
| `AccountLiabilities` | 12 | Current liabilities |
| `AccountCommissionBlocked` | 13 | Blocked commission |

---

## 🧩 Notes & Tips

* **This is where margin lives.** `AccountMargin`, `AccountMarginFree`, `AccountMarginLevel` are not in `AccountSummaryData` — read them here.
* **Margin level is a percent**, not currency (e.g. `842.0` means 842 %). The rest are in the deposit currency.
* **Need several values?** One `account_summary()` beats four `account_info_double()` calls for the fields it already carries (balance, equity, credit).
* **Higher layers wrap this.** `MT5Service::account_double(prop)` returns the bare `f64`; `MT5Sugar::margin()` / `free_margin()` / `margin_level()` name the common ones.

---

## 🔗 Usage Examples

### 1) Free margin before sizing a trade

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::AccountInfoDoublePropertyType as P;

let req = pb::AccountInfoDoubleRequest { property_id: P::AccountMarginFree as i32 };
let free = account.account_info_double(req).await?.requested_value;
println!("free margin: {free:.2}");
if free < 100.0 { println!("[WARN] not enough free margin"); }
```

### 2) Margin level (percent) as a health gauge

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::AccountInfoDoublePropertyType as P;

let lvl = account
    .account_info_double(pb::AccountInfoDoubleRequest { property_id: P::AccountMarginLevel as i32 })
    .await?
    .requested_value;
println!("margin level: {lvl:.1}%");
if lvl != 0.0 && lvl < 200.0 { println!("[ALERT] margin level below 200% — reduce exposure"); }
```

### 3) Live floating profit

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::AccountInfoDoublePropertyType as P;

let profit = account
    .account_info_double(pb::AccountInfoDoubleRequest { property_id: P::AccountProfit as i32 })
    .await?
    .requested_value;
println!("floating P/L: {profit:+.2}");
```

---

## ⚠️ Pitfalls

* **Pass the enum as `i32`.** The field is `property_id: i32`; use `P::AccountMargin as i32`, not the enum value directly.
* **Margin level units.** It's a percent — don't compare it against a currency threshold.
* **Flat account.** With no open positions `AccountMargin` is `0.0` and `AccountMarginLevel` is typically `0.0` (no division) — guard against the zero case before alerting.
* **One property per call.** For a snapshot of many, use `account_summary()`.

---

## 🧪 Testing Suggestions

* **Balance parity.** `account_info_double(AccountBalance)` equals `account_summary().account_balance`.
* **With a position open.** `AccountMargin > 0` and `AccountMarginFree < AccountEquity`.
* **Profit sign.** Open a losing position → `AccountProfit` is negative.

---

## 📚 See also

* [Account summary](account_summary.md) · [integer property](account_info_integer.md) · [string property](account_info_string.md)
* **Back to:** [Account Information — Overview](Account_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
