# Floating Profit (`floating_profit`)

> **Sugar method:** Current floating P/L of all open positions.

**API Information:**

* **Method:** `sugar.floating_profit()`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn floating_profit(&self) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Floating profit (negative = loss) in the deposit currency. |

---

## 🏛️ Essentials

* **What it is:** The account-level unrealized P/L (`ACCOUNT_PROFIT`).
* **Why you need it:** One number for "how are my open trades doing?".
* **Relation:** equals `equity − balance`.

---

## ⚡ Under the Hood

```text
MT5Sugar.floating_profit()
  → MT5Service.account_double(AccountProfit)
    → MT5Account.account_info_double(...).requested_value
```

> Account-level figure. For a **per-symbol** or **per-position** breakdown, use [get_total_profit](../7.%20Position_Information/get_total_profit.md) / [get_profit_by_symbol](../7.%20Position_Information/get_profit_by_symbol.md).

---

## When to Use

* **Live P/L display.**
* Trailing/exit logic keyed on account profit.

---

## 🔗 Usage Examples

### 1) Basic

```rust
println!("floating P/L {:+.2}", sugar.floating_profit().await?);
```

### 2) Take-profit-all trigger

```rust
if sugar.floating_profit().await? >= 100.0 {
    sugar.close_all_positions(None).await?;
}
```

### 3) Same as equity − balance

```rust
let a = sugar.floating_profit().await?;
let b = sugar.equity().await? - sugar.balance().await?;
println!("{a:+.2} ≈ {b:+.2}");
```

---

## Related Methods

* [get_total_profit](../7.%20Position_Information/get_total_profit.md) · [get_profit_by_symbol](../7.%20Position_Information/get_profit_by_symbol.md) · [equity](equity.md)

---

## ⚠️ Pitfalls

* **Account-wide.** This sums everything; for one symbol use the position-info helpers.

---

## 📚 See Also

* [equity](equity.md) · [get_total_profit](../7.%20Position_Information/get_total_profit.md)
