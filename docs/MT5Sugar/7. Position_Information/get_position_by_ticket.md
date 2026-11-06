# Get Position By Ticket (`get_position_by_ticket`)

> **Sugar method:** One open position by ticket, or `None` if not open.

**API Information:**

* **Method:** `sugar.get_position_by_ticket(ticket)`
* **Returns:** `Result<Option<PositionInfo>, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_position_by_ticket(&self, ticket: u64) -> Result<Option<pb::PositionInfo>, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `ticket` | `u64` | Position ticket. |

| Output | Type | Description |
|--------|------|-------------|
| `Option<PositionInfo>` | — | `Some(pos)` if open, `None` otherwise. |

---

## 🏛️ Essentials

* **What it is:** Finds a position in the open list by ticket.
* **Why you need it:** Read current SL/TP/volume/profit before modifying or closing.
* **Not found ≠ error:** returns `Ok(None)`.

---

## ⚡ Under the Hood

```text
MT5Sugar.get_position_by_ticket(ticket)
  → get_open_positions().into_iter().find(|p| p.ticket == ticket)
```

---

## When to Use

* **Before a modify/close** that needs the position's current fields.

---

## 🔗 Usage Examples

### 1) Read a position

```rust
if let Some(p) = sugar.get_position_by_ticket(123456).await? {
    println!("{} {:.2} lot @ {:.5}  P/L {:+.2}", p.symbol, p.volume, p.price_open, p.profit);
}
```

### 2) Move SL to breakeven using its entry

```rust
if let Some(p) = sugar.get_position_by_ticket(123456).await? {
    sugar.modify_position_sl(p.ticket, p.price_open).await?;
}
```

### 3) Exists?

```rust
let still_open = sugar.get_position_by_ticket(123456).await?.is_some();
```

---

## Related Methods

* [get_open_positions](get_open_positions.md) · [get_positions_by_symbol](get_positions_by_symbol.md)

---

## ⚠️ Pitfalls

* **`None` means not open** (closed or never existed) — not an error.
* **Fetches the whole list** internally; if you need many lookups, call `get_open_positions` once and index yourself.

---

## 📚 See Also

* [get_open_positions](get_open_positions.md) · [modify_position_sltp](../6.%20Position_Management/modify_position_sltp.md)
