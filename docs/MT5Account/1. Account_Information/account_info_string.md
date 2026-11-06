# Account Info — String

> Read **one** string account property (owner name, server, currency, company) by its property id.

**API Information:**

* **Low-level API:** `MT5Account::account_info_string(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.AccountInformation`
* **RPC:** `AccountInfoString(AccountInfoStringRequest) -> AccountInfoStringReply`
* **Payload:** `AccountInfoStringData` → single field `requested_value: String`

## 💬 Just the essentials

* **What it is.** The MQL5 `AccountInfoString(property)` call — one text property at a time.
* **Why you need it.** Labels for UI and logs: who owns the account, which broker/server, and the deposit currency.
* **When to use.** Building a header/banner, or reading `ACCOUNT_CURRENCY` to format money. Most of these are also in `account_summary()`.

---

## 🎯 Purpose

* Read the account owner's name
* Read the broker/company and trade server names
* Read the deposit currency (for formatting money amounts)

---

## Method Signature

```rust
pub async fn account_info_string(
    &self,
    request: pb::AccountInfoStringRequest,
) -> Result<pb::AccountInfoStringData, MT5Error>
```

**Request message:**

```protobuf
message AccountInfoStringRequest {
  AccountInfoStringPropertyType property_id = 1;
}
```

**Reply message:**

```protobuf
message AccountInfoStringReply {
  oneof response {
    AccountInfoStringData data = 1;
    Error error = 2;
  }
}

message AccountInfoStringData {
  string requested_value = 1;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `property_id` | `i32` (`AccountInfoStringPropertyType`) | Which string property to read. Pass `PropertyType::X as i32`. |

---

## ⬆️ Output — `AccountInfoStringData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `requested_value` | `String` | The property's text value. |

---

## 🧱 `AccountInfoStringPropertyType` (from proto)

| Variant | Value | Meaning |
|---------|-------|---------|
| `AccountName` | 0 | Account owner's name |
| `AccountServer` | 1 | Trade server name |
| `AccountCurrency` | 2 | Deposit currency (e.g. `USD`) |
| `AccountCompany` | 3 | Broker / company name |

---

## 🧩 Notes & Tips

* **Currency drives formatting.** Read `AccountCurrency` once and use it to label every money amount you print.
* **Server vs company.** `AccountServer` is the cluster/server name (e.g. `MetaQuotes-Demo`); `AccountCompany` is the broker's display name.
* **Mostly redundant with the summary.** `account_user_name`, `account_currency`, `account_company_name` are already in `AccountSummaryData` — use this call when you want just one, or specifically `AccountServer` (not in the summary).

---

## 🔗 Usage Examples

### 1) Deposit currency for money formatting

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::AccountInfoStringPropertyType as P;

let ccy = account
    .account_info_string(pb::AccountInfoStringRequest { property_id: P::AccountCurrency as i32 })
    .await?.requested_value;
let balance = account.account_summary().await?.account_balance;
println!("balance: {balance:.2} {ccy}");
```

### 2) Broker / server banner

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::AccountInfoStringPropertyType as P;

let company = account
    .account_info_string(pb::AccountInfoStringRequest { property_id: P::AccountCompany as i32 })
    .await?.requested_value;
let server = account
    .account_info_string(pb::AccountInfoStringRequest { property_id: P::AccountServer as i32 })
    .await?.requested_value;
println!("{company} — server {server}");
```

### 3) Account owner

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::AccountInfoStringPropertyType as P;

let name = account
    .account_info_string(pb::AccountInfoStringRequest { property_id: P::AccountName as i32 })
    .await?.requested_value;
println!("owner: {name}");
```

---

## ⚠️ Pitfalls

* **Pass the enum as `i32`.** `property_id: P::AccountCurrency as i32`.
* **Possible empty strings.** Some brokers leave a field blank; guard before using it as a label.
* **`AccountServer` is here, not in the summary.** If you need the server name, this is the only place to read it.

---

## 🧪 Testing Suggestions

* **Currency parity.** `account_info_string(AccountCurrency)` equals `account_summary().account_currency`.
* **Name parity.** `AccountName` equals `account_summary().account_user_name`.
* **Non-empty broker.** On a live demo `AccountCompany` and `AccountServer` are non-empty.

---

## 📚 See also

* [Account summary](account_summary.md) · [double property](account_info_double.md) · [integer property](account_info_integer.md)
* **Back to:** [Account Information — Overview](Account_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
