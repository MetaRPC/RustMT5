# Buy Market with Pips (`buy_market_with_pips`)

> **Sugar method:** Market BUY with SL/TP given **purely in pips** — the convenient bracket entry.

**API Information:**

* **Method:** `sugar.buy_market_with_pips(symbol, volume, sl_pips, tp_pips, comment, magic)`
* **Returns:** `Result<u64, MT5Error>` (order ticket)
* **Layer:** HIGH (MT5Sugar) — alias over [buy_market_with_sltp](buy_market_with_sltp.md)

---

## Method Signature

```rust
pub async fn buy_market_with_pips(
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
| `sl_pips` | `Option<f64>` | SL distance in pips below entry. |
| `tp_pips` | `Option<f64>` | TP distance in pips above entry. |
| `comment` | `&str` | Comment. |
| `magic` | `u64` | Magic number. |

| Output | Type | Description |
|--------|------|-------------|
| `u64` | — | Order ticket. |

---

## 🏛️ Essentials

* **What it is:** The pip-only shortcut: `buy_market_with_sltp(.., None, None, sl_pips, tp_pips, ..)`.
* **Why you need it:** Cleanest call for "buy with a 20-pip stop and 40-pip target".
* **Pip size:** 1 pip = 10 × point (0.0001 on a 5-digit FX symbol).

---

## ⚡ Under the Hood

```text
MT5Sugar.buy_market_with_pips()
  → buy_market_with_sltp(sl=None, tp=None, sl_pips, tp_pips)
    → market_with_sltp(Buy) → send_order(...) → MT5Service.place_order(...)
```

---

## When to Use

* **Pip-based bracket entries** — the default building block for the strategy orchestrators.

---

## 🔗 Usage Examples

### 1) 20/40 bracket

```rust
let t = sugar.buy_market_with_pips(Some("EURUSD"), 0.10, Some(20.0), Some(40.0), "bracket", 2003).await?;
```

### 2) Risk-sized

```rust
let lots = sugar.calculate_position_size("EURUSD", 1.0, 20.0).await?;
let t = sugar.buy_market_with_pips(Some("EURUSD"), lots, Some(20.0), Some(40.0), "sized", 2003).await?;
```

### 3) SL only

```rust
let t = sugar.buy_market_with_pips(None, 0.05, Some(25.0), None, "", 0).await?;
```

---

## Related Methods

* [sell_market_with_pips](sell_market_with_pips.md) · [buy_market_with_sltp](buy_market_with_sltp.md) · [calculate_position_size](../10.%20Risk_Management/calculate_position_size.md)

---

## ⚠️ Pitfalls

* **Too-tight pips** (inside stops level) → rejection — check [get_min_stop_level](../9.%20Symbol_Information/get_min_stop_level.md).
* **`None` without a default** → `InvalidArgument`.

---

## 📚 See Also

* [sell_market_with_pips](sell_market_with_pips.md) · [buy_market_with_sltp](buy_market_with_sltp.md)
