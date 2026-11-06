# Get Deals (Date Range) (`get_deals_date_range`)

> **Sugar method:** Closed positions over an explicit UTC timestamp range.

**API Information:**

* **Method:** `sugar.get_deals_date_range(from, to)`
* **Returns:** `Result<Vec<PositionHistoryInfo>, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_deals_date_range(
    &self,
    from: prost_types::Timestamp,
    to: prost_types::Timestamp,
) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `from` | `prost_types::Timestamp` | Range start (UTC). |
| `to` | `prost_types::Timestamp` | Range end (UTC). |

| Output | Type | Description |
|--------|------|-------------|
| `Vec<PositionHistoryInfo>` | — | Closed positions opened within `[from, to]`. |

---

## 🏛️ Essentials

* **What it is:** Like [get_deals](get_deals.md) but with an arbitrary range instead of a `Period`.
* **Why you need it:** Custom report windows (a specific day, a backtest span).
* **Build timestamps** as `prost_types::Timestamp { seconds, nanos: 0 }`.

---

## ⚡ Under the Hood

```text
MT5Sugar.get_deals_date_range(from, to)
  → deals_between(from, to)
    → MT5Service.positions_history(AhPositionOpenTimeAsc, from, to, page 0, 10_000)
```

---

## When to Use

* **Arbitrary windows** not covered by `Period`.

---

## 🔗 Usage Examples

### 1) Last 72 hours

```rust
fn ts(s: i64) -> prost_types::Timestamp { prost_types::Timestamp { seconds: s, nanos: 0 } }
let now = chrono::Utc::now().timestamp();
let deals = sugar.get_deals_date_range(ts(now - 3*86_400), ts(now)).await?;
println!("{} deals", deals.len());
```

### 2) A specific calendar day

```rust
use chrono::{TimeZone, Utc};
fn ts(s: i64) -> prost_types::Timestamp { prost_types::Timestamp { seconds: s, nanos: 0 } }
let start = Utc.with_ymd_and_hms(2026, 11, 5, 0, 0, 0).unwrap().timestamp();
let end   = start + 86_400;
let deals = sugar.get_deals_date_range(ts(start), ts(end)).await?;
```

### 3) Net over the range

```rust
let net: f64 = deals.iter().map(|d| d.profit + d.swap + d.commission + d.fee).sum();
println!("net {net:+.2}");
```

---

## Related Methods

* [get_deals](get_deals.md) · [get_profit](get_profit.md)

---

## ⚠️ Pitfalls

* **Open-time filter** — same semantics as `get_deals`.
* **UTC** — convert local/broker times before building the timestamps.
* **10,000-row cap** (single page).

---

## 📚 See Also

* [get_deals](get_deals.md) · [get_daily_stats](get_daily_stats.md)
