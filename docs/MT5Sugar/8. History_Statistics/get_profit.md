# Get Profit (`get_profit`)

> **Sugar method:** Total realized profit for a predefined `Period` (sum of closed-position profit).

**API Information:**

* **Method:** `sugar.get_profit(period)`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_profit(&self, period: Period) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `period` | `Period` | `Today` / `Yesterday` / `ThisWeek` / `ThisMonth`. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Sum of `profit` over the period's closed positions. |

---

## 🏛️ Essentials

* **What it is:** `get_deals(period)` then sums each deal's `profit`.
* **Why you need it:** Quick realized-P/L number per period.
* **Profit only:** this sums the `profit` field (not swap/commission/fee); for the net, sum those yourself from [get_deals](get_deals.md).

---

## ⚡ Under the Hood

```text
MT5Sugar.get_profit(period)
  → get_deals(period).iter().map(|d| d.profit).sum()
```

---

## When to Use

* **Daily/weekly/monthly realized P/L** at a glance.

---

## 🔗 Usage Examples

### 1) Today

```rust
use metarpc_mt5::sugar::Period;
println!("today realized {:+.2}", sugar.get_profit(Period::Today).await?);
```

### 2) Compare week vs month

```rust
use metarpc_mt5::sugar::Period;
let w = sugar.get_profit(Period::ThisWeek).await?;
let m = sugar.get_profit(Period::ThisMonth).await?;
println!("week {w:+.2}, month {m:+.2}");
```

### 3) Net (incl. costs) via get_deals

```rust
use metarpc_mt5::sugar::Period;
let net: f64 = sugar.get_deals(Period::Today).await?
    .iter().map(|d| d.profit + d.swap + d.commission + d.fee).sum();
println!("today net {net:+.2}");
```

---

## Related Methods

* [get_deals](get_deals.md) · [profit shortcuts](get_profit_today.md) · [get_daily_stats](get_daily_stats.md)

---

## ⚠️ Pitfalls

* **Gross profit** — excludes swap/commission/fee.
* **Open-time window** semantics (see [get_deals](get_deals.md)).

---

## 📚 See Also

* [get_deals](get_deals.md) · [get_profit_today](get_profit_today.md)
