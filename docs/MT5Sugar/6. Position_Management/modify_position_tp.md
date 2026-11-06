# Modify Position TP (`modify_position_tp`)

> **Sugar method:** Move only the take-profit of a position (stop-loss preserved).

**API Information:**

* **Method:** `sugar.modify_position_tp(ticket, tp)`
* **Returns:** `Result<bool, MT5Error>`
* **Layer:** HIGH (MT5Sugar) — convenience over [modify_position_sltp](modify_position_sltp.md)

---

## Method Signature

```rust
pub async fn modify_position_tp(&self, ticket: u64, tp: f64) -> Result<bool, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `ticket` | `u64` | Position ticket. |
| `tp` | `f64` | New take-profit (absolute price). |

| Output | Type | Description |
|--------|------|-------------|
| `bool` | — | `true` if `returned_code == 10009`. |

---

## 🏛️ Essentials

* **What it is:** `modify_position_sltp(ticket, None, Some(tp))` — TP changes, SL kept.
* **Why you need it:** Extend or pull in a target as the trade develops.
* **Absolute price.**

---

## ⚡ Under the Hood

```text
MT5Sugar.modify_position_tp(ticket, tp)
  → modify_position_sltp(ticket, None, Some(tp))
    → MT5Service.modify_order(...) → MT5Account.order_modify(...)
```

---

## When to Use

* **Extend a target** on momentum, or tighten it to bank sooner.

---

## 🔗 Usage Examples

### 1) Set a target

```rust
sugar.modify_position_tp(123456, 1.09200).await?;
```

### 2) Extend by 20 pips

```rust
if let Some(p) = sugar.get_position_by_ticket(123456).await? {
    sugar.modify_position_tp(p.ticket, p.take_profit + 0.0020).await?;
}
```

### 3) Check result

```rust
if !sugar.modify_position_tp(123456, 1.09000).await? { println!("TP move rejected"); }
```

---

## Related Methods

* [modify_position_sl](modify_position_sl.md) · [modify_position_sltp](modify_position_sltp.md)

---

## ⚠️ Pitfalls

* **Too-tight TP** (inside stops level) → rejection.
* **Unknown ticket** → `InvalidArgument`.

---

## 📚 See Also

* [modify_position_sl](modify_position_sl.md) · [modify_position_sltp](modify_position_sltp.md)
