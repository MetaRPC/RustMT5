# Get Daily Stats (`get_daily_stats`)

> **Sugar method:** Aggregated statistics for **today's** closed positions.

**API Information:**

* **Method:** `sugar.get_daily_stats()`
* **Returns:** `Result<DailyStats, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_daily_stats(&self) -> Result<DailyStats, MT5Error>
```

### `DailyStats`

```rust
pub struct DailyStats {
    pub deals: usize,   // number of closed positions today
    pub profit: f64,    // sum of profit
    pub wins: usize,    // profit > 0
    pub losses: usize,  // profit < 0
}
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `DailyStats` | struct | `deals`, `profit`, `wins`, `losses` for today. |

---

## 🏛️ Essentials

* **What it is:** Today's deals rolled up into a small summary.
* **Why you need it:** A one-call end-of-day scorecard.
* **Break-even deals** (`profit == 0`) count in `deals` but in neither `wins` nor `losses`.

---

## ⚡ Under the Hood

```text
MT5Sugar.get_daily_stats()
  → get_deals(Period::Today)
  → aggregate: deals=len, profit=Σprofit, wins=count(>0), losses=count(<0)
```

---

## When to Use

* **Daily report / dashboard tile.**

---

## 🔗 Usage Examples

### 1) Print the scorecard

```rust
let s = sugar.get_daily_stats().await?;
println!("today: {} deals, {:+.2}, {}W/{}L", s.deals, s.profit, s.wins, s.losses);
```

### 2) Win rate

```rust
let s = sugar.get_daily_stats().await?;
let decided = s.wins + s.losses;
if decided > 0 { println!("win rate {:.1}%", s.wins as f64 / decided as f64 * 100.0); }
```

### 3) Stop trading after a bad day

```rust
let s = sugar.get_daily_stats().await?;
if s.profit <= -200.0 { println!("daily loss limit hit — stop"); }
```

---

## Related Methods

* [get_deals](get_deals.md) · [get_profit_today](get_profit_today.md)

---

## ⚠️ Pitfalls

* **Today only** — for other windows use [get_deals](get_deals.md)/[get_profit](get_profit.md).
* **Gross profit** (sum of `profit`); costs not deducted.
* **Break-even deals** sit outside `wins`/`losses`.

---

## 📚 See Also

* [get_deals](get_deals.md) · [get_profit_today](get_profit_today.md)
