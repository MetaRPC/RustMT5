# Get Min Stop Level (`get_min_stop_level`)

> **Sugar method:** Minimum SL/TP distance from price, in points (`SYMBOL_TRADE_STOPS_LEVEL`).

**API Information:**

* **Method:** `sugar.get_min_stop_level(symbol)`
* **Returns:** `Result<i64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_min_stop_level(&self, symbol: &str) -> Result<i64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `&str` | Symbol name. |

| Output | Type | Description |
|--------|------|-------------|
| `i64` | — | Minimum stop distance in **points**. |

---

## 🏛️ Essentials

* **What it is:** The broker-enforced floor for how close SL/TP (and some pendings) may sit to the market.
* **Why you need it:** Validate SL/TP distances before sending — too-tight stops are rejected.
* **Units:** points; convert pips → points as `pips × 10` on 5/3-digit symbols.

---

## ⚡ Under the Hood

```text
MT5Sugar.get_min_stop_level(symbol)
  → MT5Service.symbol_integer(symbol, SymbolTradeStopsLevel)
    → MT5Account.symbol_info_integer(...).value
```

---

## When to Use

* **Pre-trade SL/TP validation**; clamping stop distances.

---

## 🔗 Usage Examples

### 1) Read it

```rust
println!("min stop {} points", sugar.get_min_stop_level("EURUSD").await?);
```

### 2) Validate a pip stop

```rust
let min_pts = sugar.get_min_stop_level("EURUSD").await?;
let sl_points = 8 * 10;   // 8 pips
if (sl_points as i64) < min_pts { println!("SL too tight: min {min_pts} points"); }
```

### 3) Clamp

```rust
let min_pts = sugar.get_min_stop_level("EURUSD").await?;
let sl_points = (5i64 * 10).max(min_pts);   // at least the broker minimum
```

---

## Related Methods

* [get_symbol_info](get_symbol_info.md) · [get_symbol_digits](get_symbol_digits.md)

---

## ⚠️ Pitfalls

* **Points, not pips.** Multiply pips by 10 (on 5/3-digit symbols) before comparing.
* **Zero is possible** on some symbols (no enforced minimum).

---

## 📚 See Also

* [get_symbol_info](get_symbol_info.md) · low-level [symbol_info_integer](../../MT5Account/2.%20Symbol_Information/symbol_info_integer.md)
