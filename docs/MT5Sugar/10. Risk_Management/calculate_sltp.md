# Calculate SL/TP (`calculate_sltp`)

> **Sugar method:** Convert pip SL/TP distances into absolute prices for a given side.

**API Information:**

* **Method:** `sugar.calculate_sltp(symbol, is_buy, sl_pips, tp_pips)`
* **Returns:** `Result<(Option<f64>, Option<f64>), MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn calculate_sltp(
    &self,
    symbol: &str,
    is_buy: bool,
    sl_pips: Option<f64>,
    tp_pips: Option<f64>,
) -> Result<(Option<f64>, Option<f64>), MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `&str` | Symbol name. |
| `is_buy` | `bool` | `true` for long (SL below / TP above), `false` for short. |
| `sl_pips` | `Option<f64>` | SL distance in pips, or `None`. |
| `tp_pips` | `Option<f64>` | TP distance in pips, or `None`. |

| Output | Type | Description |
|--------|------|-------------|
| `(Option<f64>, Option<f64>)` | — | `(sl_price, tp_price)` — each `None` if its pips input was `None`. |

---

## 🏛️ Essentials

* **What it is:** Pip → absolute-price helper that respects side (entry = ask for buy, bid for sell).
* **Why you need it:** Feed absolute SL/TP into [modify_position_sltp](../6.%20Position_Management/modify_position_sltp.md) or build order requests.
* **Pip size:** `point × 10`.

---

## ⚡ Under the Hood

```text
MT5Sugar.calculate_sltp(symbol, is_buy, sl_pips, tp_pips)
  → tick = symbol_tick(symbol); point = symbol_double(symbol, SymbolPoint)
  → entry = if is_buy { ask } else { bid }
  → buy:  sl = entry - p*point*10,  tp = entry + p*point*10
    sell: sl = entry + p*point*10,  tp = entry - p*point*10
```

---

## When to Use

* **Before modifying** a position's SL/TP from pip distances.
* Computing bracket levels for display or manual order building.

---

## 🔗 Usage Examples

### 1) Long bracket prices

```rust
let (sl, tp) = sugar.calculate_sltp("EURUSD", true, Some(20.0), Some(40.0)).await?;
println!("SL {sl:?}  TP {tp:?}");
```

### 2) Apply to a position

```rust
let (sl, tp) = sugar.calculate_sltp("EURUSD", true, Some(20.0), Some(40.0)).await?;
sugar.modify_position_sltp(123456, sl, tp).await?;
```

### 3) Short, SL only

```rust
let (sl, _tp) = sugar.calculate_sltp("EURUSD", false, Some(25.0), None).await?;
```

---

## Related Methods

* [modify_position_sltp](../6.%20Position_Management/modify_position_sltp.md) · [buy_market_with_pips](../5.%20Trading_With_SLTP/buy_market_with_pips.md)

---

## ⚠️ Pitfalls

* **Uses the current tick** as entry — for a position already open, the real entry differs; use the entry price when that matters.
* **`None` in → `None` out** for that side.

---

## 📚 See Also

* [calculate_position_size](calculate_position_size.md) · [modify_position_sltp](../6.%20Position_Management/modify_position_sltp.md)
