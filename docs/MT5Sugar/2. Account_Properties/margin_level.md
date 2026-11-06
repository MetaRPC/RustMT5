# Margin Level (`margin_level`)

> **Sugar method:** Margin level as a percentage (equity / margin × 100).

**API Information:**

* **Method:** `sugar.margin_level()`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn margin_level(&self) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Margin level in **percent**. |

---

## 🏛️ Essentials

* **What it is:** Health gauge — the broker issues margin calls / stop-outs below thresholds.
* **Why you need it:** De-risk before you hit stop-out.
* **Flat account:** typically `0.0` (no margin used → no division).

---

## ⚡ Under the Hood

```text
MT5Sugar.margin_level()
  → MT5Service.account_double(AccountMarginLevel)
    → MT5Account.account_info_double(...).requested_value
```

---

## When to Use

* **Risk monitoring** / alerts.
* Deciding to reduce exposure.

---

## 🔗 Usage Examples

### 1) Basic

```rust
println!("margin level {:.1}%", sugar.margin_level().await?);
```

### 2) Alert

```rust
let lvl = sugar.margin_level().await?;
if lvl != 0.0 && lvl < 200.0 { println!("[ALERT] margin level {lvl:.0}% — de-risk"); }
```

### 3) Guard a new trade

```rust
if sugar.margin_level().await? < 300.0 { println!("skip: margin level too low"); }
```

---

## Related Methods

* [margin](margin.md) · [free_margin](free_margin.md) · [equity](equity.md)

---

## ⚠️ Pitfalls

* **It's a percent**, not currency.
* **Zero when flat** — guard the `0.0` case before alerting.

---

## 📚 See Also

* [margin](margin.md) · [free_margin](free_margin.md)
