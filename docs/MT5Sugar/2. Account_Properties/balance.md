# Balance (`balance`)

> **Sugar method:** Current account balance (realized P/L only) in one line.

**API Information:**

* **Method:** `sugar.balance()`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn balance(&self) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Balance in the deposit currency. |

---

## 🏛️ Essentials

* **What it is:** Money excluding floating P/L of open positions.
* **Why you need it:** Risk sizing and "enough funds?" checks.
* **Sanity check:** `balance ≤ equity` when positions are in profit, `≥` when in loss.

---

## ⚡ Under the Hood

```text
MT5Sugar.balance()
  → MT5Service.account_double(AccountBalance)
    → MT5Account.account_info_double(...).requested_value
```

---

## When to Use

* **Before trading** — confirm funds.
* **Position sizing** — base risk on balance (see [calculate_position_size](../10.%20Risk_Management/calculate_position_size.md)).

---

## 🔗 Usage Examples

### 1) Basic

```rust
let bal = sugar.balance().await?;
println!("balance {bal:.2}");
```

### 2) Risk amount

```rust
let risk = sugar.balance().await? * 0.02;  // 2%
println!("risking {risk:.2}");
```

### 3) Gate trading

```rust
if sugar.balance().await? < 1000.0 { println!("insufficient"); return Ok(()); }
```

---

## Related Methods

* [equity](equity.md) · [free_margin](free_margin.md) · [get_account_info](get_account_info.md)

---

## ⚠️ Pitfalls

* **Balance ≠ equity.** Balance ignores open positions; use [equity](equity.md) for live account value.
* Fetching several properties? Prefer [get_account_info](get_account_info.md) (one call).

---

## 📚 See Also

* [equity](equity.md) · [get_account_info](get_account_info.md)
