# Get Open Positions (`get_open_positions`)

> **Sugar method:** All open positions as a `Vec<PositionInfo>`.

**API Information:**

* **Method:** `sugar.get_open_positions()`
* **Returns:** `Result<Vec<PositionInfo>, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_open_positions(&self) -> Result<Vec<pb::PositionInfo>, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `Vec<PositionInfo>` | — | Open positions (sorted by open time ascending). |

`PositionInfo` key fields: `ticket: u64`, `symbol`, `volume`, `price_open`, `price_current`, `stop_loss`, `take_profit`, `profit`, `swap`, `r#type` (0 Buy / 1 Sell). Full table: low-level [opened_orders](../../MT5Account/3.%20Positions_Orders/opened_orders.md).

---

## 🏛️ Essentials

* **What it is:** The base query the other position helpers build on.
* **Why you need it:** Iterate live positions with full detail.
* **Pendings excluded:** this returns `position_infos` only (not resting orders).

---

## ⚡ Under the Hood

```text
MT5Sugar.get_open_positions()
  → MT5Service.opened_orders(SortByOpenTimeAsc).position_infos
    → MT5Account.opened_orders(...)  (gRPC)
```

---

## When to Use

* **Dashboards**, management loops, bulk operations.

---

## 🔗 Usage Examples

### 1) List

```rust
for p in sugar.get_open_positions().await? {
    let side = if p.r#type == 0 { "BUY" } else { "SELL" };
    println!("#{} {side} {} {:.2} lot  P/L {:+.2}", p.ticket, p.symbol, p.volume, p.profit);
}
```

### 2) Total floating P/L

```rust
let total: f64 = sugar.get_open_positions().await?.iter().map(|p| p.profit).sum();
println!("floating {total:+.2}");
```

### 3) Close all losers

```rust
for p in sugar.get_open_positions().await? {
    if p.profit < 0.0 { sugar.close_position(p.ticket).await?; }
}
```

---

## Related Methods

* [get_position_by_ticket](get_position_by_ticket.md) · [get_positions_by_symbol](get_positions_by_symbol.md) · [count_open_positions](count_open_positions.md)

---

## ⚠️ Pitfalls

* **Snapshot** — `profit`/`price_current` are as-of the call.
* **Positions only** — for pendings use the service/low-level `opened_orders`.

---

## 📚 See Also

* [get_total_profit](get_total_profit.md) · [has_open_position](has_open_position.md)
