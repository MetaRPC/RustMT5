# Get Account Info (`get_account_info`)

> **Sugar method:** A compact account snapshot (`AccountInfo`) in one call.

**API Information:**

* **Method:** `sugar.get_account_info()`
* **Returns:** `Result<AccountInfo, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_account_info(&self) -> Result<AccountInfo, MT5Error>
```

### `AccountInfo`

```rust
pub struct AccountInfo {
    pub login: i64,
    pub name: String,
    pub company: String,
    pub currency: String,
    pub leverage: i64,
    pub balance: f64,
    pub equity: f64,
}
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `AccountInfo` | struct | Login, name, company, currency, leverage, balance, equity. |

---

## 🏛️ Essentials

* **What it is:** The handful of fields you usually want, built from one `account_summary()`.
* **Why you need it:** One round-trip instead of several `balance()`/`equity()`/… calls.
* **Note:** It does **not** include margin fields — read those with [margin](margin.md)/[free_margin](free_margin.md)/[margin_level](margin_level.md).

---

## ⚡ Under the Hood

```text
MT5Sugar.get_account_info()
  → MT5Service.account_summary()  (maps AccountSummaryData → AccountInfo)
    → MT5Account.account_summary()  (gRPC)
```

---

## When to Use

* **Account banner / header.**
* Any time you need more than one of login/name/company/currency/leverage/balance/equity.

---

## 🔗 Usage Examples

### 1) Banner

```rust
let a = sugar.get_account_info().await?;
println!("#{} {} — {:.2} {} (eq {:.2})  1:{}", a.login, a.company, a.balance, a.currency, a.equity, a.leverage);
```

### 2) Floating P/L from the snapshot

```rust
let a = sugar.get_account_info().await?;
println!("floating {:+.2}", a.equity - a.balance);
```

### 3) Owner & currency

```rust
let a = sugar.get_account_info().await?;
println!("{} trades in {}", a.name, a.currency);
```

---

## Related Methods

* [balance](balance.md) · [equity](equity.md) · [margin](margin.md)

---

## ⚠️ Pitfalls

* **No margin fields** in `AccountInfo` — use the dedicated margin getters.

---

## 📚 See Also

* [balance](balance.md) · [equity](equity.md) · [ping](../1.%20Connection/ping.md)
