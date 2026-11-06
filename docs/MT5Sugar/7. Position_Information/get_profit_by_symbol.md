# Get Profit By Symbol (`get_profit_by_symbol`)

> **Sugar method:** Floating profit summed over open positions on one symbol.

**API Information:**

* **Method:** `sugar.get_profit_by_symbol(symbol)`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_profit_by_symbol(&self, symbol: &str) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `&str` | Symbol to sum over (required). |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Sum of `profit` for that symbol's open positions. |

---

## 🏛️ Essentials

* **What it is:** Per-symbol floating P/L.
* **Why you need it:** Manage one instrument's exposure independently.
* **Zero** if no positions on the symbol.

---

## ⚡ Under the Hood

```text
MT5Sugar.get_profit_by_symbol(symbol)
  → get_open_positions().iter().filter(|p| p.symbol == symbol).map(|p| p.profit).sum()
```

---

## When to Use

* **Per-symbol P/L display** and symbol-level exit rules.

---

## 🔗 Usage Examples

### 1) Basic

```rust
println!("EURUSD P/L {:+.2}", sugar.get_profit_by_symbol("EURUSD").await?);
```

### 2) Close a symbol in profit

```rust
if sugar.get_profit_by_symbol("EURUSD").await? >= 30.0 {
    sugar.close_all_positions(Some("EURUSD")).await?;
}
```

### 3) Compare symbols

```rust
for s in ["EURUSD", "GBPUSD"] {
    println!("{s}: {:+.2}", sugar.get_profit_by_symbol(s).await?);
}
```

---

## Related Methods

* [get_total_profit](get_total_profit.md) · [get_positions_by_symbol](get_positions_by_symbol.md)

---

## ⚠️ Pitfalls

* **Exact symbol match** (broker suffixes).
* **Excludes swap/commission** (sum of `profit` fields).

---

## 📚 See Also

* [get_total_profit](get_total_profit.md) · [get_positions_by_symbol](get_positions_by_symbol.md)
