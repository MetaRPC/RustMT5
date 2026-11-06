# Sell Market with SL/TP (`sell_market_with_sltp`)

> **Sugar method:** Market SELL with stop-loss / take-profit given as **absolute prices and/or pips**.

**API Information:**

* **Method:** `sugar.sell_market_with_sltp(symbol, volume, sl, tp, sl_pips, tp_pips, comment, magic)`
* **Returns:** `Result<u64, MT5Error>` (order ticket)
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn sell_market_with_sltp(
    &self,
    symbol: Option<&str>,
    volume: f64,
    sl: Option<f64>,
    tp: Option<f64>,
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
| `sl` / `tp` | `Option<f64>` | Absolute SL / TP price. |
| `sl_pips` / `tp_pips` | `Option<f64>` | SL / TP distance in pips from entry. |
| `comment` | `&str` | Comment. |
| `magic` | `u64` | Magic number. |

| Output | Type | Description |
|--------|------|-------------|
| `u64` | — | Order ticket. |

---

## 🏛️ Essentials

* **What it is:** Market sell at the bid, with protection.
* **Direction:** for a short, SL is **above** entry and TP is **below** — the method handles the sign for pips automatically.
* **Pips override** absolute on the same side.

---

## ⚡ Under the Hood

```text
MT5Sugar.sell_market_with_sltp()
  → market_with_sltp(Sell):
      entry = bid
      if sl_pips: sl = entry + sl_pips*point*10
      if tp_pips: tp = entry - tp_pips*point*10
  → send_order(Sell, entry, sl, tp) → ...
```

---

## When to Use

* **Protected short entries** in pips or exact prices.

---

## 🔗 Usage Examples

### 1) Pips SL/TP

```rust
let t = sugar.sell_market_with_sltp(Some("EURUSD"), 0.10, None, None, Some(20.0), Some(40.0), "short", 2002).await?;
```

### 2) Absolute

```rust
let t = sugar.sell_market_with_sltp(Some("EURUSD"), 0.10, Some(1.09000), Some(1.08300), None, None, "", 0).await?;
```

### 3) SL only

```rust
let t = sugar.sell_market_with_sltp(None, 0.05, None, None, Some(25.0), None, "", 0).await?;
```

---

## Related Methods

* [buy_market_with_sltp](buy_market_with_sltp.md) · [sell_market_with_pips](sell_market_with_pips.md) · [calculate_sltp](../10.%20Risk_Management/calculate_sltp.md)

---

## ⚠️ Pitfalls

* **Short-side signs** are handled for you — don't invert pips manually.
* **Pips override absolute** per side.
* **Too-tight stops** → rejection.

---

## 📚 See Also

* [sell_market_with_pips](sell_market_with_pips.md) · [buy_market_with_sltp](buy_market_with_sltp.md)
