# Profit Period Shortcuts (`get_profit_today` / `_this_week` / `_this_month`)

> **Sugar methods:** Zero-argument shortcuts for realized profit over the common windows.

**API Information:**

* **Methods:** `sugar.get_profit_today()`, `get_profit_this_week()`, `get_profit_this_month()`
* **Returns:** `Result<f64, MT5Error>` each
* **Layer:** HIGH (MT5Sugar)

---

## Method Signatures

```rust
pub async fn get_profit_today(&self)      -> Result<f64, MT5Error>
pub async fn get_profit_this_week(&self)  -> Result<f64, MT5Error>
pub async fn get_profit_this_month(&self) -> Result<f64, MT5Error>
```

> Note: there is **no** `get_profit_yesterday` — use `get_profit(Period::Yesterday)` if you need it.

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Realized profit (sum of `profit`) for the window. |

---

## 🏛️ Essentials

* **What they are:** Wrappers over [`get_profit`](get_profit.md) with the matching `Period`.
* **Gross profit:** sum of `profit` fields (excludes swap/commission/fee).

---

## ⚡ Under the Hood

```text
get_profit_today()      → get_profit(Period::Today)
get_profit_this_week()  → get_profit(Period::ThisWeek)
get_profit_this_month() → get_profit(Period::ThisMonth)
```

---

## 🔗 Usage Examples

### 1) Today

```rust
println!("today {:+.2}", sugar.get_profit_today().await?);
```

### 2) Week & month

```rust
println!("week {:+.2}, month {:+.2}", sugar.get_profit_this_week().await?, sugar.get_profit_this_month().await?);
```

### 3) Yesterday (via the generic method)

```rust
use metarpc_mt5::sugar::Period;
println!("yesterday {:+.2}", sugar.get_profit(Period::Yesterday).await?);
```

---

## Related Methods

* [get_profit](get_profit.md) · [get_deals shortcuts](get_deals_today.md) · [get_daily_stats](get_daily_stats.md)

---

## ⚠️ Pitfalls

* **No `_yesterday` variant** — use the generic `get_profit(Period::Yesterday)`.
* **Gross, not net** — excludes costs.

---

## 📚 See Also

* [get_profit](get_profit.md) · [get_daily_stats](get_daily_stats.md)
