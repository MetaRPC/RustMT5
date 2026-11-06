# Get Symbol Digits (`get_symbol_digits`)

> **Sugar method:** Price decimal places for a symbol (`SYMBOL_DIGITS`).

**API Information:**

* **Method:** `sugar.get_symbol_digits(symbol)`
* **Returns:** `Result<i64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_symbol_digits(&self, symbol: &str) -> Result<i64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `&str` | Symbol name. |

| Output | Type | Description |
|--------|------|-------------|
| `i64` | — | Number of decimal places in the price. |

---

## 🏛️ Essentials

* **What it is:** Reads `SYMBOL_DIGITS` via the service.
* **Why you need it:** Format prices correctly (5 for most FX, 3 for JPY pairs, 2 for many indices).
* **Pairs with point:** `point = 10^(−digits)`.

---

## ⚡ Under the Hood

```text
MT5Sugar.get_symbol_digits(symbol)
  → MT5Service.symbol_integer(symbol, SymbolDigits)
    → MT5Account.symbol_info_integer(...).value
```

---

## When to Use

* **Price formatting** in logs/UI.

---

## 🔗 Usage Examples

### 1) Format a price

```rust
let d = sugar.get_symbol_digits("EURUSD").await? as usize;
let bid = sugar.bid(Some("EURUSD")).await?;
println!("bid {bid:.*}", d);
```

### 2) JPY check

```rust
let d = sugar.get_symbol_digits("USDJPY").await?;
println!("USDJPY digits {d}");  // typically 3
```

### 3) Derive point

```rust
let d = sugar.get_symbol_digits("EURUSD").await?;
let point = 10f64.powi(-(d as i32));
println!("point {point}");
```

---

## Related Methods

* [get_symbol_info](get_symbol_info.md) · [get_min_stop_level](get_min_stop_level.md)

---

## ⚠️ Pitfalls

* **Digits vs pip:** a pip is `10 × point` on 5/3-digit symbols — don't confuse digits with pip scale.

---

## 📚 See Also

* [get_symbol_info](get_symbol_info.md) · [spread](../3.%20Prices_Quotes/spread.md)
