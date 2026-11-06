# Modify Position SL (`modify_position_sl`)

> **Sugar method:** Move only the stop-loss of a position (take-profit preserved).

**API Information:**

* **Method:** `sugar.modify_position_sl(ticket, sl)`
* **Returns:** `Result<bool, MT5Error>`
* **Layer:** HIGH (MT5Sugar) — convenience over [modify_position_sltp](modify_position_sltp.md)

---

## Method Signature

```rust
pub async fn modify_position_sl(&self, ticket: u64, sl: f64) -> Result<bool, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `ticket` | `u64` | Position ticket. |
| `sl` | `f64` | New stop-loss (absolute price). |

| Output | Type | Description |
|--------|------|-------------|
| `bool` | — | `true` if `returned_code == 10009`. |

---

## 🏛️ Essentials

* **What it is:** `modify_position_sltp(ticket, Some(sl), None)` — SL changes, TP kept.
* **Why you need it:** Trailing stops and breakeven moves.
* **Absolute price.**

---

## ⚡ Under the Hood

```text
MT5Sugar.modify_position_sl(ticket, sl)
  → modify_position_sltp(ticket, Some(sl), None)
    → MT5Service.modify_order(...) → MT5Account.order_modify(...)
```

---

## When to Use

* **Move SL to breakeven**, trail a stop manually.

---

## 🔗 Usage Examples

### 1) Breakeven

```rust
let entry = 1.08500;
sugar.modify_position_sl(123456, entry).await?;
```

### 2) Trail behind price

```rust
let bid = sugar.bid(Some("EURUSD")).await?;
sugar.modify_position_sl(123456, bid - 0.0010).await?;  // 10 pips behind
```

### 3) Check the result

```rust
if !sugar.modify_position_sl(123456, 1.08400).await? { println!("SL move rejected"); }
```

---

## Related Methods

* [modify_position_tp](modify_position_tp.md) · [modify_position_sltp](modify_position_sltp.md)

---

## ⚠️ Pitfalls

* **Too-tight SL** (inside stops level) → rejection.
* **Unknown ticket** → `InvalidArgument`.

---

## 📚 See Also

* [modify_position_tp](modify_position_tp.md) · [modify_position_sltp](modify_position_sltp.md)
