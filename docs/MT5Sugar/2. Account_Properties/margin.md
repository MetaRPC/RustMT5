# Margin (`margin`)

> **Sugar method:** Margin currently used by open positions.

**API Information:**

* **Method:** `sugar.margin()`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn margin(&self) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Used margin in the deposit currency. |

---

## 🏛️ Essentials

* **What it is:** Funds reserved to hold current open positions.
* **Why you need it:** Combine with equity for margin level; see headroom for new trades.
* **Flat account:** `0.0`.

---

## ⚡ Under the Hood

```text
MT5Sugar.margin()
  → MT5Service.account_double(AccountMargin)
    → MT5Account.account_info_double(...).requested_value
```

---

## When to Use

* **Risk/exposure dashboards.**
* Deciding whether another position fits.

---

## 🔗 Usage Examples

### 1) Basic

```rust
println!("used margin {:.2}", sugar.margin().await?);
```

### 2) Margin level by hand

```rust
let (eq, m) = (sugar.equity().await?, sugar.margin().await?);
if m > 0.0 { println!("margin level {:.1}%", eq / m * 100.0); }
```

### 3) Headroom

```rust
println!("free vs used: {:.2} / {:.2}", sugar.free_margin().await?, sugar.margin().await?);
```

---

## Related Methods

* [free_margin](free_margin.md) · [margin_level](margin_level.md) · [calculate_required_margin](../10.%20Risk_Management/calculate_required_margin.md)

---

## ⚠️ Pitfalls

* **Used vs free.** This is *used* margin; available room is [free_margin](free_margin.md).

---

## 📚 See Also

* [free_margin](free_margin.md) · [margin_level](margin_level.md)
