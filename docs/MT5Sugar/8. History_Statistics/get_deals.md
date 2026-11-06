# Get Deals (`get_deals`)

> **Sugar method:** Closed positions (deals) for a predefined `Period`.

**API Information:**

* **Method:** `sugar.get_deals(period)`
* **Returns:** `Result<Vec<PositionHistoryInfo>, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_deals(&self, period: Period) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error>
```

### `Period`

```rust
pub enum Period { Today, Yesterday, ThisWeek, ThisMonth }
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `period` | `Period` | Predefined UTC window (Today / Yesterday / ThisWeek / ThisMonth). |

| Output | Type | Description |
|--------|------|-------------|
| `Vec<PositionHistoryInfo>` | — | Closed positions opened within the window. |

`PositionHistoryInfo` key fields: `position_ticket`, `symbol`, `order_type`, `volume`, `open_price`, `close_price`, `profit`, `swap`, `commission`, `fee`, `open_time`, `close_time`. Full table: low-level [positions_history](../../MT5Account/3.%20Positions_Orders/positions_history.md).

---

## 🏛️ Essentials

* **What it is:** Resolves the `Period` to a `[from, to]` UTC range and fetches closed positions.
* **Why you need it:** Trade journals, realized-P/L reports.
* **Range is by open time** (see pitfalls).

---

## ⚡ Under the Hood

```text
MT5Sugar.get_deals(period)
  → period_range(period)  → (from, to)   (UTC, chrono)
  → deals_between(from, to)
    → MT5Service.positions_history(AhPositionOpenTimeAsc, from, to, page 0, 10_000)
      → MT5Account.positions_history(...)  (gRPC)
```

---

## When to Use

* **Period reports** (today/yesterday/week/month) of closed trades.

---

## 🔗 Usage Examples

### 1) Today's deals

```rust
use metarpc_mt5::sugar::Period;

let deals = sugar.get_deals(Period::Today).await?;
println!("{} closed today", deals.len());
```

### 2) This week's net

```rust
use metarpc_mt5::sugar::Period;

let net: f64 = sugar.get_deals(Period::ThisWeek).await?
    .iter().map(|d| d.profit + d.swap + d.commission + d.fee).sum();
println!("week net {net:+.2}");
```

### 3) Win rate this month

```rust
use metarpc_mt5::sugar::Period;

let d = sugar.get_deals(Period::ThisMonth).await?;
let wins = d.iter().filter(|x| x.profit > 0.0).count();
if !d.is_empty() { println!("win rate {:.1}%", wins as f64 / d.len() as f64 * 100.0); }
```

---

## Related Methods

* [get_profit](get_profit.md) · [get_deals_date_range](get_deals_date_range.md) · [period shortcuts](get_deals_today.md) · [get_daily_stats](get_daily_stats.md)

---

## ⚠️ Pitfalls

* **Filtered by open time**, not close time — a trade opened before the window won't appear even if it closed inside it.
* **Cap:** fetches up to 10,000 rows (page 0); very long windows may exceed that.

---

## 📚 See Also

* [get_profit](get_profit.md) · [get_deals_date_range](get_deals_date_range.md)
