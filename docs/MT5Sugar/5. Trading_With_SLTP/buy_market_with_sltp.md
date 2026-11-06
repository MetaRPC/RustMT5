# Buy Market with SL/TP (`buy_market_with_sltp`)

> **Sugar method:** Market BUY with stop-loss / take-profit given as **absolute prices and/or pips**.

**API Information:**

* **Method:** `sugar.buy_market_with_sltp(symbol, volume, sl, tp, sl_pips, tp_pips, comment, magic)`
* **Returns:** `Result<u64, MT5Error>` (order ticket)
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn buy_market_with_sltp(
    &self,
    symbol: Option<&str>,
    volume: f64,
    sl: Option<f64>,        // absolute price
    tp: Option<f64>,        // absolute price
    sl_pips: Option<f64>,   // distance in pips
    tp_pips: Option<f64>,   // distance in pips
    comment: &str,
    magic: u64,
) -> Result<u64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `Option<&str>` | Symbol, or `None` for default. |
| `volume` | `f64` | Lots. |
| `sl` / `tp` | `Option<f64>` | Absolute SL / TP price. |
| `sl_pips` / `tp_pips` | `Option<f64>` | SL / TP distance in pips from entry (1 pip = 10 × point). |
| `comment` | `&str` | Comment. |
| `magic` | `u64` | Magic number. |

| Output | Type | Description |
|--------|------|-------------|
| `u64` | — | Order ticket. |

---

## 🏛️ Essentials

* **What it is:** Market buy at the ask, with protection in whichever form you prefer.
* **Why you need it:** Enter and protect in a single call.
* **Pips win:** if `sl_pips`/`tp_pips` are provided, they're converted from the entry and **override** the absolute `sl`/`tp` for that side.

---

## ⚡ Under the Hood

```text
MT5Sugar.buy_market_with_sltp()
  → market_with_sltp(Buy):
      entry = ask
      if sl_pips: sl = entry - sl_pips*point*10
      if tp_pips: tp = entry + tp_pips*point*10
  → send_order(Buy, entry, sl, tp) → MT5Service.place_order(...) → MT5Account.order_send(...)
```

---

## When to Use

* **Protected long entries** where you think in pips or in exact prices.

---

## 🔗 Usage Examples

### 1) Pips SL/TP

```rust
let t = sugar.buy_market_with_sltp(Some("EURUSD"), 0.10, None, None, Some(20.0), Some(40.0), "bracket", 2001).await?;
```

### 2) Absolute SL/TP

```rust
let t = sugar.buy_market_with_sltp(Some("EURUSD"), 0.10, Some(1.08300), Some(1.09000), None, None, "", 0).await?;
```

### 3) SL only (let profit run)

```rust
let t = sugar.buy_market_with_sltp(None, 0.05, None, None, Some(25.0), None, "", 0).await?;
```

---

## Related Methods

* [sell_market_with_sltp](sell_market_with_sltp.md) · [buy_market_with_pips](buy_market_with_pips.md) · [calculate_sltp](../10.%20Risk_Management/calculate_sltp.md)

---

## ⚠️ Pitfalls

* **Pips override absolute** on a given side when both are passed.
* **Too-tight stops** (inside the symbol's stops level) → rejection.
* **`None` without a default** → `InvalidArgument`.

---

## 📚 See Also

* [buy_market_with_pips](buy_market_with_pips.md) · [sell_market_with_sltp](sell_market_with_sltp.md)
