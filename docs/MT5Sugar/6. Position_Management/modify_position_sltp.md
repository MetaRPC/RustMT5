# Modify Position SL/TP (`modify_position_sltp`)

> **Sugar method:** Set stop-loss and/or take-profit on a position; the unspecified side is **preserved**.

**API Information:**

* **Method:** `sugar.modify_position_sltp(ticket, sl, tp)`
* **Returns:** `Result<bool, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn modify_position_sltp(
    &self,
    ticket: u64,
    sl: Option<f64>,
    tp: Option<f64>,
) -> Result<bool, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `ticket` | `u64` | Position ticket. |
| `sl` | `Option<f64>` | New SL (absolute price), or `None` to keep the current SL. |
| `tp` | `Option<f64>` | New TP (absolute price), or `None` to keep the current TP. |

| Output | Type | Description |
|--------|------|-------------|
| `bool` | — | `true` if `returned_code == 10009`. |

---

## 🏛️ Essentials

* **What it is:** Reads the position, then re-sends SL/TP, defaulting each unspecified side to the position's current value.
* **Why you need it:** Safe SL/TP edit without accidentally clearing the other side.
* **Prices are absolute** — convert pips first with [calculate_sltp](../10.%20Risk_Management/calculate_sltp.md).

---

## ⚡ Under the Hood

```text
MT5Sugar.modify_position_sltp(ticket, sl, tp)
  → get_position_by_ticket(ticket)                       // to read existing sl/tp
  → MT5Service.modify_order(OrderModifyRequest {
        ticket,
        stop_loss: Some(sl.unwrap_or(pos.stop_loss)),
        take_profit: Some(tp.unwrap_or(pos.take_profit)),
    })
    → MT5Account.order_modify(...)  (gRPC)
```

If the ticket isn't an open position, it returns `InvalidArgument`.

---

## When to Use

* **Adjust protection** on a live trade (set a target, tighten a stop).

---

## 🔗 Usage Examples

### 1) Set both

```rust
let ok = sugar.modify_position_sltp(123456, Some(1.08300), Some(1.09000)).await?;
```

### 2) Set TP, keep SL

```rust
sugar.modify_position_sltp(123456, None, Some(1.09200)).await?;
```

### 3) From pips

```rust
let (sl, tp) = sugar.calculate_sltp("EURUSD", true, Some(20.0), Some(40.0)).await?;
sugar.modify_position_sltp(123456, sl, tp).await?;
```

---

## Related Methods

* [modify_position_sl](modify_position_sl.md) · [modify_position_tp](modify_position_tp.md) · [calculate_sltp](../10.%20Risk_Management/calculate_sltp.md)

---

## ⚠️ Pitfalls

* **Unknown ticket** → `InvalidArgument` (it must be an open position).
* **Absolute prices**, not distances.
* **Stops level** — new SL/TP too close to market → rejection.

---

## 📚 See Also

* [modify_position_sl](modify_position_sl.md) · [modify_position_tp](modify_position_tp.md)
