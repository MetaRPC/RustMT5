# Sell Market with Pips (`sell_market_with_pips`)

> **Sugar method:** Market SELL with SL/TP given **purely in pips**.

**API Information:**

* **Method:** `sugar.sell_market_with_pips(symbol, volume, sl_pips, tp_pips, comment, magic)`
* **Returns:** `Result<u64, MT5Error>` (order ticket)
* **Layer:** HIGH (MT5Sugar) — alias over [sell_market_with_sltp](sell_market_with_sltp.md)

---

## Method Signature

```rust
pub async fn sell_market_with_pips(
    &self,
    symbol: Option<&str>,
    volume: f64,
    sl_pips: Option<f64>,
    tp_pips: Option<f64>,
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
| `sl_pips` | `Option<f64>` | SL distance in pips **above** entry (short). |
| `tp_pips` | `Option<f64>` | TP distance in pips **below** entry (short). |
| `comment` | `&str` | Comment. |
| `magic` | `u64` | Magic number. |

| Output | Type | Description |
|--------|------|-------------|
| `u64` | — | Order ticket. |

---

## 🏛️ Essentials

* **What it is:** Pip-only shortcut for a short: `sell_market_with_sltp(.., None, None, sl_pips, tp_pips, ..)`.
* **Direction handled:** for a short, SL sits above and TP below entry — the method applies the signs.
* **Pip size:** 1 pip = 10 × point.

---

## ⚡ Under the Hood

```text
MT5Sugar.sell_market_with_pips()
  → sell_market_with_sltp(sl=None, tp=None, sl_pips, tp_pips)
    → market_with_sltp(Sell) → send_order(...) → MT5Service.place_order(...)
```

---

## When to Use

* **Pip-based short bracket entries.**

---

## 🔗 Usage Examples

### 1) 20/40 bracket

```rust
let t = sugar.sell_market_with_pips(Some("EURUSD"), 0.10, Some(20.0), Some(40.0), "short", 2004).await?;
```

### 2) Risk-sized

```rust
let lots = sugar.calculate_position_size("EURUSD", 1.0, 20.0).await?;
let t = sugar.sell_market_with_pips(Some("EURUSD"), lots, Some(20.0), Some(40.0), "sized", 2004).await?;
```

### 3) SL only

```rust
let t = sugar.sell_market_with_pips(None, 0.05, Some(25.0), None, "", 0).await?;
```

---

## Related Methods

* [buy_market_with_pips](buy_market_with_pips.md) · [sell_market_with_sltp](sell_market_with_sltp.md) · [calculate_position_size](../10.%20Risk_Management/calculate_position_size.md)

---

## ⚠️ Pitfalls

* **Don't invert pips** for the short — the method does it.
* **Too-tight pips** → rejection.

---

## 📚 See Also

* [buy_market_with_pips](buy_market_with_pips.md) · [sell_market_with_sltp](sell_market_with_sltp.md)
