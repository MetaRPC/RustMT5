# Close Position Partial (`close_position_partial`)

> **Sugar method:** Close part of a position by volume; `true` on success.

**API Information:**

* **Method:** `sugar.close_position_partial(ticket, volume)`
* **Returns:** `Result<bool, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn close_position_partial(&self, ticket: u64, volume: f64) -> Result<bool, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `ticket` | `u64` | Position ticket. |
| `volume` | `f64` | Lots to close (less than the position's size). |

| Output | Type | Description |
|--------|------|-------------|
| `bool` | — | `true` if `returned_code == 10009`. |

---

## 🏛️ Essentials

* **What it is:** Scale out — close `volume` lots, leave the rest open.
* **Why you need it:** Lock in partial profit, reduce risk while staying in.
* **Slippage:** defaults to 10 points.

---

## ⚡ Under the Hood

```text
MT5Sugar.close_position_partial(ticket, volume)
  → MT5Service.close_order(OrderCloseRequest { ticket, volume, slippage: 10 })
    → MT5Account.order_close(...)  (gRPC)
```

---

## When to Use

* **Scaling out** at a first target; trimming exposure.

---

## 🔗 Usage Examples

### 1) Close half of 0.10

```rust
let ok = sugar.close_position_partial(123456, 0.05).await?;
```

### 2) Scale out, keep a runner

```rust
if let Some(p) = sugar.get_position_by_ticket(123456).await? {
    sugar.close_position_partial(p.ticket, p.volume / 2.0).await?;
}
```

### 3) Then move SL to breakeven on the rest

```rust
sugar.close_position_partial(123456, 0.05).await?;
sugar.modify_position_sl(123456, /* entry */ 1.08500).await?;
```

---

## Related Methods

* [close_position](close_position.md) · [modify_position_sl](modify_position_sl.md) · [get_position_by_ticket](../7.%20Position_Information/get_position_by_ticket.md)

---

## ⚠️ Pitfalls

* **Volume > position size** → rejection.
* **Remainder stays open** — the ticket persists after a partial close.
* **Respect volume step** for the closed amount.

---

## 📚 See Also

* [close_position](close_position.md) · [close_all_positions](close_all_positions.md)
