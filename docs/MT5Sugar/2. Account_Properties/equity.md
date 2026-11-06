# Equity (`equity`)

> **Sugar method:** Current equity = balance + floating P/L.

**API Information:**

* **Method:** `sugar.equity()`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn equity(&self) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Equity in the deposit currency. |

---

## 🏛️ Essentials

* **What it is:** The live account value — balance adjusted for open positions' floating P/L.
* **Why you need it:** The number that matters for drawdown and margin level.
* **Sanity check:** `equity − balance` equals current floating P/L.

---

## ⚡ Under the Hood

```text
MT5Sugar.equity()
  → MT5Service.account_double(AccountEquity)
    → MT5Account.account_info_double(...).requested_value
```

---

## When to Use

* **Drawdown / performance monitoring.**
* **Margin-level math** (`equity / margin × 100`).

---

## 🔗 Usage Examples

### 1) Basic

```rust
println!("equity {:.2}", sugar.equity().await?);
```

### 2) Floating P/L

```rust
let floating = sugar.equity().await? - sugar.balance().await?;
println!("floating P/L {floating:+.2}");
```

### 3) Drawdown alert

```rust
let (bal, eq) = (sugar.balance().await?, sugar.equity().await?);
if bal > 0.0 && (bal - eq) / bal > 0.1 { println!("[ALERT] >10% drawdown"); }
```

---

## Related Methods

* [balance](balance.md) · [floating_profit](floating_profit.md) · [margin_level](margin_level.md)

---

## ⚠️ Pitfalls

* **Snapshot.** Equity moves with price — re-read before decisions.
* For floating P/L specifically, [floating_profit](floating_profit.md) is more direct.

---

## 📚 See Also

* [balance](balance.md) · [floating_profit](floating_profit.md) · [get_account_info](get_account_info.md)
