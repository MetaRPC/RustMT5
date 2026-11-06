# Free Margin (`free_margin`)

> **Sugar method:** Margin available for opening new positions.

**API Information:**

* **Method:** `sugar.free_margin()`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn free_margin(&self) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Free margin in the deposit currency (`equity − margin`). |

---

## 🏛️ Essentials

* **What it is:** What's left to open new trades.
* **Why you need it:** Pre-trade gate — compare with [calculate_required_margin](../10.%20Risk_Management/calculate_required_margin.md).
* **Flat account:** equals equity.

---

## ⚡ Under the Hood

```text
MT5Sugar.free_margin()
  → MT5Service.account_double(AccountMarginFree)
    → MT5Account.account_info_double(...).requested_value
```

---

## When to Use

* **Before opening** a position — does it fit?
* Sizing with a safety buffer.

---

## 🔗 Usage Examples

### 1) Basic

```rust
println!("free margin {:.2}", sugar.free_margin().await?);
```

### 2) Fit check

```rust
let need = sugar.calculate_required_margin("EURUSD", 1.0, None).await?;
if sugar.free_margin().await? < need { println!("not enough free margin"); }
```

### 3) Keep a buffer

```rust
let usable = sugar.free_margin().await? * 0.9;  // leave 10%
println!("usable {usable:.2}");
```

---

## Related Methods

* [margin](margin.md) · [margin_level](margin_level.md) · [calculate_required_margin](../10.%20Risk_Management/calculate_required_margin.md)

---

## ⚠️ Pitfalls

* **Don't size to 100%.** Leave headroom for spread/slippage and adverse moves.

---

## 📚 See Also

* [margin](margin.md) · [calculate_required_margin](../10.%20Risk_Management/calculate_required_margin.md)
