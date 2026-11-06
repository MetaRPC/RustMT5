# Has Open Position (`has_open_position`)

> **Sugar method:** Is any position open (optionally on a specific symbol)?

**API Information:**

* **Method:** `sugar.has_open_position(symbol)`
* **Returns:** `Result<bool, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn has_open_position(&self, symbol: Option<&str>) -> Result<bool, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `Option<&str>` | A specific symbol, or `None` for **any** symbol. |

| Output | Type | Description |
|--------|------|-------------|
| `bool` | — | `true` if at least one matching position is open. |

---

## 🏛️ Essentials

* **What it is:** A yes/no exposure check.
* **Why you need it:** Guard before opening (avoid stacking), or detect a pending's fill.
* **`None` = any symbol** (this is a filter, not the default-symbol fallback).

---

## ⚡ Under the Hood

```text
MT5Sugar.has_open_position(symbol)
  → get_open_positions()
  → match symbol { Some(s) => any(p.symbol == s), None => !is_empty() }
```

---

## When to Use

* **"Only one at a time"** entry guards.
* **Pending-fill polling** (see the Pending Bracket orchestrator).

---

## 🔗 Usage Examples

### 1) Any position?

```rust
if sugar.has_open_position(None).await? { println!("already in a trade"); }
```

### 2) On a symbol

```rust
if !sugar.has_open_position(Some("EURUSD")).await? {
    sugar.buy_market(Some("EURUSD"), 0.10, "entry", 1).await?;
}
```

### 3) Wait for a limit to fill

```rust
use std::time::Duration;
while !sugar.has_open_position(Some("EURUSD")).await? {
    tokio::time::sleep(Duration::from_millis(500)).await;
}
```

---

## Related Methods

* [count_open_positions](count_open_positions.md) · [get_open_positions](get_open_positions.md)

---

## ⚠️ Pitfalls

* **`None` = any symbol**, not the default symbol.
* **Positions only** — a resting pending order is not an open position yet.

---

## 📚 See Also

* [count_open_positions](count_open_positions.md) · [get_positions_by_symbol](get_positions_by_symbol.md)
