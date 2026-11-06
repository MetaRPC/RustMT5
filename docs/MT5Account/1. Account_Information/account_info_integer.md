# Account Info — Integer

> Read **one** integer account property (login, leverage, trade mode, permission flags, …) by its property id.

**API Information:**

* **Low-level API:** `MT5Account::account_info_integer(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.AccountInformation`
* **RPC:** `AccountInfoInteger(AccountInfoIntegerRequest) -> AccountInfoIntegerReply`
* **Payload:** `AccountInfoIntegerData` → single field `requested_value: i64`

## 💬 Just the essentials

* **What it is.** The MQL5 `AccountInfoInteger(property)` call — one integer property at a time.
* **Why you need it.** Identity and permissions: login, leverage, trade mode, and the boolean flags (`trade_allowed`, `trade_expert`, FIFO/hedging) that gate whether you may trade at all.
* **When to use.** A pre-flight check before trading, or to read leverage for position sizing.

---

## 🎯 Purpose

* Read account identity (login) and leverage
* Read trade mode (DEMO / CONTEST / REAL) as an enum value
* Read permission flags: `trade_allowed`, `trade_expert`, FIFO, hedging
* Read margin mode / stop-out mode / currency digits

---

## Method Signature

```rust
pub async fn account_info_integer(
    &self,
    request: pb::AccountInfoIntegerRequest,
) -> Result<pb::AccountInfoIntegerData, MT5Error>
```

**Request message:**

```protobuf
message AccountInfoIntegerRequest {
  AccountInfoIntegerPropertyType property_id = 1;
}
```

**Reply message:**

```protobuf
message AccountInfoIntegerReply {
  oneof response {
    AccountInfoIntegerData data = 1;
    Error error = 2;
  }
}

message AccountInfoIntegerData {
  int64 requested_value = 1;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `property_id` | `i32` (`AccountInfoIntegerPropertyType`) | Which integer property to read. Pass `PropertyType::X as i32`. |

---

## ⬆️ Output — `AccountInfoIntegerData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `requested_value` | `i64` | The property's value. Booleans come back as `1` / `0`; enum properties as the enum's integer value. |

---

## 🧱 `AccountInfoIntegerPropertyType` (from proto)

| Variant | Value | Meaning | Decode as |
|---------|-------|---------|-----------|
| `AccountLogin` | 0 | Account number | `i64` |
| `AccountTradeMode` | 1 | Trade mode | `MrpcEnumAccountTradeMode` (0 demo / 1 contest / 2 real) |
| `AccountLeverage` | 2 | Leverage | `i64` (e.g. 100) |
| `AccountLimitOrders` | 3 | Max open positions + pending (0 = unlimited) | `i64` |
| `AccountMarginSoMode` | 4 | Stop-out mode | `ENUM_ACCOUNT_STOP_OUT_MODE` |
| `AccountTradeAllowed` | 5 | Trading allowed for the account | bool (1/0) |
| `AccountTradeExpert` | 6 | Trading allowed for an EA | bool (1/0) |
| `AccountMarginMode` | 7 | Margin calculation mode | `ENUM_ACCOUNT_MARGIN_MODE` |
| `AccountCurrencyDigits` | 8 | Decimal places in account currency | `i64` |
| `AccountFifoClose` | 9 | Positions must close FIFO | bool (1/0) |
| `AccountHedgeAllowed` | 10 | Opposite positions on one symbol allowed | bool (1/0) |

---

## 🧩 Notes & Tips

* **Booleans are integers.** `AccountTradeAllowed`, `AccountTradeExpert`, `AccountFifoClose`, `AccountHedgeAllowed` return `1` (true) or `0` (false) — compare with `== 1` or `!= 0`.
* **Enum properties return the enum's number.** `AccountTradeMode` returns 0/1/2 — decode with `MrpcEnumAccountTradeMode::try_from(v as i32)`.
* **Pre-trade gate.** Before sending orders, confirm `AccountTradeAllowed == 1` **and** (for automated trading) `AccountTradeExpert == 1`.
* **Leverage for sizing.** `AccountLeverage` here matches `account_summary().account_leverage`.

---

## 🔗 Usage Examples

### 1) Pre-trade permission gate

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::AccountInfoIntegerPropertyType as P;

async fn can_trade(account: &metarpc_mt5::MT5Account) -> Result<bool, metarpc_mt5::MT5Error> {
    let allowed = account
        .account_info_integer(pb::AccountInfoIntegerRequest { property_id: P::AccountTradeAllowed as i32 })
        .await?.requested_value;
    let expert = account
        .account_info_integer(pb::AccountInfoIntegerRequest { property_id: P::AccountTradeExpert as i32 })
        .await?.requested_value;
    Ok(allowed == 1 && expert == 1)
}
```

### 2) Trade mode via the enum

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::{AccountInfoIntegerPropertyType as P, MrpcEnumAccountTradeMode as Mode};

let v = account
    .account_info_integer(pb::AccountInfoIntegerRequest { property_id: P::AccountTradeMode as i32 })
    .await?.requested_value;
match Mode::try_from(v as i32) {
    Ok(Mode::MrpcAccountTradeModeReal) => println!("REAL account — be careful"),
    Ok(Mode::MrpcAccountTradeModeContest) => println!("contest account"),
    _ => println!("demo account"),
}
```

### 3) Leverage for position sizing

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::AccountInfoIntegerPropertyType as P;

let lev = account
    .account_info_integer(pb::AccountInfoIntegerRequest { property_id: P::AccountLeverage as i32 })
    .await?.requested_value;
println!("leverage 1:{lev}");
```

---

## ⚠️ Pitfalls

* **Pass the enum as `i32`.** `property_id: P::AccountLeverage as i32`.
* **Don't treat bool properties as counts.** `AccountTradeAllowed` is `1`/`0`, not a quantity.
* **Enum decode needs `as i32`.** `requested_value` is `i64`; `MrpcEnumAccountTradeMode::try_from` wants `i32`.
* **`AccountLimitOrders == 0` means unlimited**, not "zero orders allowed".

---

## 🧪 Testing Suggestions

* **Login parity.** `account_info_integer(AccountLogin)` equals the `user` you constructed `MT5Account` with and `account_summary().account_login`.
* **Demo flags.** On a demo account `AccountTradeAllowed` is usually `1`; `AccountTradeMode` decodes to `MrpcAccountTradeModeDemo`.
* **Leverage parity.** Matches `account_summary().account_leverage`.

---

## 📚 See also

* [Account summary](account_summary.md) · [double property](account_info_double.md) · [string property](account_info_string.md)
* **Back to:** [Account Information — Overview](Account_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
