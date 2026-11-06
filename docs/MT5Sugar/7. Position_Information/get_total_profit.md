# Get Total Profit (`get_total_profit`)

> **Sugar method:** Total floating profit across all open positions.

**API Information:**

* **Method:** `sugar.get_total_profit()`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_total_profit(&self) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Sum of `profit` over all open positions. |

---

## 🏛️ Essentials

* **What it is:** Sums each position's floating P/L.
* **Why you need it:** Position-level total (excludes swap/commission, unlike the account figure).
* **Vs `floating_profit()`:** [floating_profit](../2.%20Account_Properties/floating_profit.md) reads the account-level `ACCOUNT_PROFIT`; this sums the per-position `profit` field.

---

## ⚡ Under the Hood

```text
MT5Sugar.get_total_profit()
  → get_open_positions().iter().map(|p| p.profit).sum()
```

---

## When to Use

* **Take-profit-all triggers** keyed on open-position P/L.

---

## 🔗 Usage Examples

### 1) Basic

```rust
println!("open P/L {:+.2}", sugar.get_total_profit().await?);
```

### 2) Flatten on target

```rust
if sugar.get_total_profit().await? >= 100.0 { sugar.close_all_positions(None).await?; }
```

### 3) Cut losses

```rust
if sugar.get_total_profit().await? <= -50.0 { sugar.close_all_positions(None).await?; }
```

---

## Related Methods

* [get_profit_by_symbol](get_profit_by_symbol.md) · [floating_profit](../2.%20Account_Properties/floating_profit.md)

---

## ⚠️ Pitfalls

* **Excludes swap/commission** — it's the sum of `profit` fields. For the all-in account figure use [floating_profit](../2.%20Account_Properties/floating_profit.md).

---

## 📚 See Also

* [get_profit_by_symbol](get_profit_by_symbol.md) · [floating_profit](../2.%20Account_Properties/floating_profit.md)
