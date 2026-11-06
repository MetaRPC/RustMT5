# Close Position (`close_position`)

> **Sugar method:** Fully close a position by ticket; `true` if the server reported success.

**API Information:**

* **Method:** `sugar.close_position(ticket)`
* **Returns:** `Result<bool, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn close_position(&self, ticket: u64) -> Result<bool, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `ticket` | `u64` | Position (or pending) ticket. |

| Output | Type | Description |
|--------|------|-------------|
| `bool` | — | `true` if `returned_code == 10009`. |

---

## 🏛️ Essentials

* **What it is:** Sends a close with `volume: 0.0` (full close) and `slippage: 0`.
* **Why you need it:** One-line exit by ticket.
* **Pendings:** passing a pending order's ticket deletes the resting order.

---

## ⚡ Under the Hood

```text
MT5Sugar.close_position(ticket)
  → MT5Service.close_order(OrderCloseRequest { ticket, volume: 0.0, slippage: 0 })
    → MT5Account.order_close(...)  (gRPC)
  → returns res.returned_code == 10009
```

---

## When to Use

* **Exit a known ticket** (from `get_open_positions`).

---

## 🔗 Usage Examples

### 1) Close by ticket

```rust
let ok = sugar.close_position(123456).await?;
println!("closed: {ok}");
```

### 2) Close the first open position

```rust
if let Some(p) = sugar.get_open_positions().await?.first() {
    sugar.close_position(p.ticket).await?;
}
```

### 3) Close on profit target

```rust
if sugar.get_total_profit().await? >= 50.0 {
    for p in sugar.get_open_positions().await? { sugar.close_position(p.ticket).await?; }
}
```

---

## Related Methods

* [close_position_partial](close_position_partial.md) · [close_all_positions](close_all_positions.md) · [get_open_positions](../7.%20Position_Information/get_open_positions.md)

---

## ⚠️ Pitfalls

* **`false` is not an error.** It means the server declined (bad ticket, already closed) — check it.
* **Slippage 0** can requote in fast markets; use [close_position_partial](close_position_partial.md) (slippage 10) or the raw layer if needed.

---

## 📚 See Also

* [close_all_positions](close_all_positions.md) · [close_position_partial](close_position_partial.md)
