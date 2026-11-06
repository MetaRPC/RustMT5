# Count Open Positions (`count_open_positions`)

> **Sugar method:** How many positions are open (optionally on a specific symbol).

**API Information:**

* **Method:** `sugar.count_open_positions(symbol)`
* **Returns:** `Result<usize, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn count_open_positions(&self, symbol: Option<&str>) -> Result<usize, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `Option<&str>` | A specific symbol, or `None` for **all**. |

| Output | Type | Description |
|--------|------|-------------|
| `usize` | — | Number of matching open positions. |

---

## 🏛️ Essentials

* **What it is:** Count from `get_open_positions`, optionally filtered by symbol.
* **Why you need it:** Enforce max-position limits; dashboards.
* **`None` = all symbols.**

---

## ⚡ Under the Hood

```text
MT5Sugar.count_open_positions(symbol)
  → get_open_positions()
  → match symbol { Some(s) => count(p.symbol == s), None => len() }
```

---

## When to Use

* **Position-limit checks** before adding exposure.

---

## 🔗 Usage Examples

### 1) Total count

```rust
println!("{} open", sugar.count_open_positions(None).await?);
```

### 2) Per-symbol limit

```rust
if sugar.count_open_positions(Some("EURUSD")).await? >= 3 {
    println!("max EURUSD positions reached");
}
```

### 3) Only act when flat

```rust
if sugar.count_open_positions(None).await? == 0 {
    sugar.buy_market(Some("EURUSD"), 0.1, "", 0).await?;
}
```

---

## Related Methods

* [has_open_position](has_open_position.md) · [get_open_positions](get_open_positions.md)

---

## ⚠️ Pitfalls

* **`None` = all symbols**, not the default symbol.
* For a plain yes/no, [has_open_position](has_open_position.md) is cheaper to read.

---

## 📚 See Also

* [has_open_position](has_open_position.md) · [get_open_positions](get_open_positions.md)
