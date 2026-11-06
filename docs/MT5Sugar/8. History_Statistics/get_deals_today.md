# Deals Period Shortcuts (`get_deals_today` / `_yesterday` / `_this_week` / `_this_month`)

> **Sugar methods:** Zero-argument shortcuts for the four `Period` deal queries.

**API Information:**

* **Methods:** `sugar.get_deals_today()`, `get_deals_yesterday()`, `get_deals_this_week()`, `get_deals_this_month()`
* **Returns:** `Result<Vec<PositionHistoryInfo>, MT5Error>` each
* **Layer:** HIGH (MT5Sugar)

---

## Method Signatures

```rust
pub async fn get_deals_today(&self)      -> Result<Vec<pb::PositionHistoryInfo>, MT5Error>
pub async fn get_deals_yesterday(&self)  -> Result<Vec<pb::PositionHistoryInfo>, MT5Error>
pub async fn get_deals_this_week(&self)  -> Result<Vec<pb::PositionHistoryInfo>, MT5Error>
pub async fn get_deals_this_month(&self) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `Vec<PositionHistoryInfo>` | — | Closed positions in the corresponding window. |

---

## 🏛️ Essentials

* **What they are:** Thin wrappers — each calls [`get_deals`](get_deals.md) with the matching `Period`.
* **Why you need them:** Readable, argument-free calls for the common windows.

---

## ⚡ Under the Hood

```text
get_deals_today()      → get_deals(Period::Today)
get_deals_yesterday()  → get_deals(Period::Yesterday)
get_deals_this_week()  → get_deals(Period::ThisWeek)
get_deals_this_month() → get_deals(Period::ThisMonth)
```

---

## 🔗 Usage Examples

### 1) Today

```rust
let n = sugar.get_deals_today().await?.len();
println!("{n} trades today");
```

### 2) Yesterday vs today count

```rust
let (y, t) = (sugar.get_deals_yesterday().await?.len(), sugar.get_deals_today().await?.len());
println!("yesterday {y}, today {t}");
```

### 3) Month's symbols traded

```rust
use std::collections::BTreeSet;
let syms: BTreeSet<_> = sugar.get_deals_this_month().await?.into_iter().map(|d| d.symbol).collect();
println!("{} symbols this month", syms.len());
```

---

## Related Methods

* [get_deals](get_deals.md) · [get_profit shortcuts](get_profit_today.md) · [get_daily_stats](get_daily_stats.md)

---

## ⚠️ Pitfalls

* **Open-time window** semantics (see [get_deals](get_deals.md)).
* **No `yesterday` for profit** — there is no `get_profit_yesterday` (see [profit shortcuts](get_profit_today.md)).

---

## 📚 See Also

* [get_deals](get_deals.md) · [get_profit_today](get_profit_today.md)
