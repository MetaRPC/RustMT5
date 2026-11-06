# Account Summary (RECOMMENDED)

> Complete account snapshot — balance, equity, leverage, currency, trade mode, server time — in a single call.

**API Information:**

* **Low-level API:** `MT5Account::account_summary()` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.AccountHelper`
* **RPC:** `AccountSummary(AccountSummaryRequest) -> AccountSummaryReply`
* **Payload:** `AccountSummaryData`

## 💬 Just the essentials

* **What it is.** One call that returns every essential account property.
* **Why you need it.** The most efficient way to read balance/equity/leverage/currency at once — one round-trip instead of many single-property `account_info_*` calls.
* **When to use.** At session start (banner / sanity check), and before any risk decision or order. For a single property not included here (e.g. margin), use [`account_info_double`](./account_info_double.md).

---

## 🎯 Purpose

* RECOMMENDED way to read account state
* Balance, equity, credit in one round-trip
* Login, owner name, broker/company
* Leverage and trade mode (DEMO / CONTEST / REAL)
* Currency and server time (+ UTC offset)

---

## Method Signature

```rust
pub async fn account_summary(&self) -> Result<AccountSummaryData, MT5Error>
```

No arguments. Returns the typed payload, or an [`MT5Error`](../../All_Guides/RETURN_CODES_REFERENCE.md).

**Request message:**

```protobuf
message AccountSummaryRequest {}
```

**Reply message:**

```protobuf
message AccountSummaryReply {
  oneof response {
    AccountSummaryData data = 1;
    Error error = 2;
  }
}

message AccountSummaryData {
  int64  account_login = 1;
  double account_balance = 2;
  double account_equity = 3;
  string account_user_name = 4;
  int64  account_leverage = 5;
  MrpcEnumAccountTradeMode account_trade_mode = 6;
  string account_company_name = 7;
  string account_currency = 8;
  google.protobuf.Timestamp server_time = 9;
  int64  utc_timezone_server_time_shift_minutes = 10;
  double account_credit = 11;
}
```

---

## 🔽 Input

None. `account_summary()` takes no parameters.

---

## ⬆️ Output — `AccountSummaryData`

Fields are plain public fields on the prost struct (snake_case, same as the proto):

| Field | Rust type | Description |
|-------|-----------|-------------|
| `account_login` | `i64` | Account number |
| `account_balance` | `f64` | Balance in deposit currency (excludes floating P/L) |
| `account_equity` | `f64` | Equity = balance + floating P/L of open positions |
| `account_user_name` | `String` | Account owner |
| `account_leverage` | `i64` | Leverage (e.g. 100 = 1:100) |
| `account_trade_mode` | `i32` (`MrpcEnumAccountTradeMode`) | DEMO / CONTEST / REAL |
| `account_company_name` | `String` | Broker name |
| `account_currency` | `String` | Deposit currency (e.g. `USD`) |
| `server_time` | `Option<Timestamp>` | Current server time |
| `utc_timezone_server_time_shift_minutes` | `i64` | Server offset from UTC, minutes |
| `account_credit` | `f64` | Credit in deposit currency |

> This gateway's `AccountSummaryData` does **not** carry margin / free-margin / margin-level — read those with [`account_info_double`](./account_info_double.md) (`ACCOUNT_MARGIN`, `ACCOUNT_MARGIN_FREE`, `ACCOUNT_MARGIN_LEVEL`) or the high-level `MT5Sugar::margin()` / `free_margin()` / `margin_level()`.

---

## 🧱 Related enum (from proto)

### `MrpcEnumAccountTradeMode` — field `account_trade_mode`

| Variant | Value | Meaning |
|---------|-------|---------|
| `MrpcAccountTradeModeDemo` | 0 | Demo / practice account |
| `MrpcAccountTradeModeContest` | 1 | Contest account |
| `MrpcAccountTradeModeReal` | 2 | Real (live) account |

The stored field is an `i32`; compare via `MrpcEnumAccountTradeMode::try_from(data.account_trade_mode)`.

---

## 🧩 Notes & Tips

* **One call beats many.** `account_summary()` returns balance, equity, leverage and currency together — don't fan out into several `account_info_*` calls for these.
* **`balance` vs `equity`.** Balance ignores open positions; equity includes their floating P/L. On a flat account they're equal; `equity − balance` is your current floating P/L.
* **Money is in the deposit currency.** Every monetary field is in `account_currency`.
* **Server time.** `server_time` is a protobuf `Timestamp` (`Option`); `utc_timezone_server_time_shift_minutes` is the broker's offset from UTC.
* **Snapshot, not a stream.** Re-query before a risk check; for live equity/profit use [`on_position_profit`](../6.%20Streaming_Methods/on_position_profit.md).

---

## 🔗 Usage Examples

### 1) Session-start banner

```rust
use metarpc_mt5::proto::mt5_term_api::MrpcEnumAccountTradeMode as Mode;

let s = account.account_summary().await?;
let mode = match Mode::try_from(s.account_trade_mode) {
    Ok(Mode::MrpcAccountTradeModeReal) => "REAL",
    Ok(Mode::MrpcAccountTradeModeContest) => "CONTEST",
    _ => "DEMO",
};
println!("Account #{} ({mode})  {}", s.account_login, s.account_company_name);
println!("Balance {:.2} {}  Equity {:.2}  (floating {:+.2})",
    s.account_balance, s.account_currency, s.account_equity, s.account_equity - s.account_balance);
println!("Leverage 1:{}", s.account_leverage);
```

### 2) Floating P/L from the snapshot

```rust
let s = account.account_summary().await?;
let floating = s.account_equity - s.account_balance;
println!("floating P/L: {floating:+.2} {}", s.account_currency);
```

### 3) Equity / drawdown monitor

```rust
let mut peak: Option<f64> = None;
loop {
    let eq = account.account_summary().await?.account_equity;
    let p = peak.map_or(eq, |pk| pk.max(eq));
    peak = Some(p);
    let dd = if p > 0.0 { (p - eq) / p * 100.0 } else { 0.0 };
    println!("equity {eq:.2}  peak {p:.2}  drawdown {dd:.2}%");
    if dd >= 10.0 { println!("[ALERT] drawdown ≥ 10% — de-risk"); break; }
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}
```

---

## ⚠️ Pitfalls

* **No margin fields here.** `account_margin` / `account_margin_free` / `account_margin_level` are **not** in this payload on this gateway — use `account_info_double`.
* **Snapshot, not live.** Equity is a point-in-time read; don't cache it across time.
* **Currency mismatch.** All money fields are in `account_currency`; converting an instrument's P/L into account currency is on you.
* **Trade mode is an `i32`.** Convert with `MrpcEnumAccountTradeMode::try_from(..)` before matching.

---

## 🧪 Testing Suggestions

* **Happy path (demo).** Fields populated; `account_currency` non-empty; on a flat account `equity == balance`.
* **With open positions.** `equity != balance` and the difference equals the summed floating P/L.
* **Trade mode.** On a demo account `account_trade_mode` decodes to `MrpcAccountTradeModeDemo`.

---

## 📚 See also

* [Account double property](account_info_double.md) · [integer](account_info_integer.md) · [string](account_info_string.md)
* **Back to:** [Account Information — Overview](Account_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
