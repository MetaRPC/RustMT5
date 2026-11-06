# Positions By Magic (`positions_by_magic`)

> **Sugar method:** Open positions whose `magic_number` matches — the per-strategy filter.

**API Information:**

* **Method:** `sugar.positions_by_magic(magic)`
* **Returns:** `Result<Vec<PositionInfo>, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn positions_by_magic(&self, magic: i64) -> Result<Vec<pb::PositionInfo>, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `magic` | `i64` | The magic number to match (the EA/strategy tag set on entry). |

| Output | Type | Description |
|--------|------|-------------|
| `Vec<PositionInfo>` | — | Open positions with `magic_number == magic`. |

---

## 🏛️ Essentials

* **What it is:** `get_open_positions()` filtered by `magic_number`.
* **Why you need it:** Isolate the positions **your** strategy opened (you set `magic` on entry) from everything else on the account.
* **Pairs with:** the `magic` argument on `buy_market` / `buy_market_with_pips` / etc.

---

## ⚡ Under the Hood

```text
MT5Sugar.positions_by_magic(magic)
  → get_open_positions().filter(|p| p.magic_number == magic)
    → MT5Service.opened_orders(..).position_infos
```

---

## 🔗 Usage Examples

### 1) List this strategy's positions

```rust
const MAGIC: u64 = 1001;
for p in sugar.positions_by_magic(MAGIC as i64).await? {
    println!("#{} {} {:.2} lot  P/L {:+.2}", p.ticket, p.symbol, p.volume, p.profit);
}
```

### 2) Close only this strategy's positions

```rust
const MAGIC: u64 = 1001;
for p in sugar.positions_by_magic(MAGIC as i64).await? {
    sugar.close_position(p.ticket).await?;
}
```

### 3) This strategy's floating P/L

```rust
let pl: f64 = sugar.positions_by_magic(1001).await?.iter().map(|p| p.profit).sum();
println!("strategy P/L {pl:+.2}");
```

---

## Related Methods

* [get_open_positions](get_open_positions.md) · [get_positions_by_symbol](get_positions_by_symbol.md) · [count_open_positions](count_open_positions.md)

---

## ⚠️ Pitfalls

* **Type is `i64`.** Entry methods take `magic: u64`; cast with `magic as i64` when filtering.
* **`magic == 0`** matches every position opened without a magic number.

---

## 📚 See Also

* [get_open_positions](get_open_positions.md) · [get_profit_by_symbol](get_profit_by_symbol.md)
