# Get Positions By Symbol (`get_positions_by_symbol`)

> **Sugar method:** All open positions on a given symbol.

**API Information:**

* **Method:** `sugar.get_positions_by_symbol(symbol)`
* **Returns:** `Result<Vec<PositionInfo>, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_positions_by_symbol(&self, symbol: &str) -> Result<Vec<pb::PositionInfo>, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `&str` | Symbol to filter by (required — no default fallback here). |

| Output | Type | Description |
|--------|------|-------------|
| `Vec<PositionInfo>` | — | Open positions whose `symbol` matches. |

---

## 🏛️ Essentials

* **What it is:** `get_open_positions()` filtered by symbol.
* **Why you need it:** Manage exposure on one instrument.
* **Required `&str`:** pass the symbol explicitly.

---

## ⚡ Under the Hood

```text
MT5Sugar.get_positions_by_symbol(symbol)
  → get_open_positions().into_iter().filter(|p| p.symbol == symbol)
```

---

## When to Use

* **Per-symbol management** (net a symbol, scale a symbol).

---

## 🔗 Usage Examples

### 1) List EURUSD positions

```rust
for p in sugar.get_positions_by_symbol("EURUSD").await? {
    println!("#{} {:.2} lot  P/L {:+.2}", p.ticket, p.volume, p.profit);
}
```

### 2) Net volume on a symbol

```rust
let net: f64 = sugar.get_positions_by_symbol("EURUSD").await?
    .iter().map(|p| if p.r#type == 0 { p.volume } else { -p.volume }).sum();
println!("net lots: {net:+.2}");
```

### 3) Count

```rust
let n = sugar.get_positions_by_symbol("EURUSD").await?.len();
```

---

## Related Methods

* [get_open_positions](get_open_positions.md) · [get_profit_by_symbol](get_profit_by_symbol.md) · [count_open_positions](count_open_positions.md)

---

## ⚠️ Pitfalls

* **Exact match** on the symbol string (broker suffixes matter: `EURUSD` vs `EURUSD.m`).
* **Fetches all then filters** — one round-trip regardless.

---

## 📚 See Also

* [get_profit_by_symbol](get_profit_by_symbol.md) · [get_open_positions](get_open_positions.md)
