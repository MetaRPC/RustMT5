# Get Symbol Info (`get_symbol_info`)

> **Sugar method:** A compact `SymbolInfo` snapshot of a symbol's trading parameters.

**API Information:**

* **Method:** `sugar.get_symbol_info(symbol)`
* **Returns:** `Result<SymbolInfo, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_symbol_info(&self, symbol: &str) -> Result<SymbolInfo, MT5Error>
```

### `SymbolInfo`

```rust
pub struct SymbolInfo {
    pub name: String,
    pub bid: f64,
    pub ask: f64,
    pub spread: i32,
    pub digits: i32,
    pub point: f64,
    pub volume_min: f64,
    pub volume_max: f64,
    pub volume_step: f64,
    pub contract_size: f64,
}
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `&str` | Symbol name (required). |

| Output | Type | Description |
|--------|------|-------------|
| `SymbolInfo` | struct | Prices + the constants pip/lot math needs. |

---

## 🏛️ Essentials

* **What it is:** The handful of fields you need for sizing/rounding, from one `symbol_params_many`.
* **Why you need it:** `point`, `volume_min/step/max`, `contract_size` and `digits` in one call.
* **Not found:** returns `InvalidArgument`.

---

## ⚡ Under the Hood

```text
MT5Sugar.get_symbol_info(symbol)
  → MT5Service.symbol_params_many(Some(symbol), None, None)
    → MT5Account.symbol_params_many(...)  (gRPC)
  → maps first SymbolParameters → SymbolInfo
```

---

## When to Use

* **Lot rounding and pip math** inputs; a symbol spec panel.

---

## 🔗 Usage Examples

### 1) Inspect a symbol

```rust
let si = sugar.get_symbol_info("EURUSD").await?;
println!("{}: point {}, step {}, contract {}", si.name, si.point, si.volume_step, si.contract_size);
```

### 2) Round a lot to the step

```rust
let si = sugar.get_symbol_info("EURUSD").await?;
let raw = 0.137;
let lots = ((raw / si.volume_step).round() * si.volume_step).clamp(si.volume_min, si.volume_max);
println!("lots {lots:.2}");
```

### 3) Spread in points

```rust
let si = sugar.get_symbol_info("EURUSD").await?;
println!("spread {} points", si.spread);
```

---

## Related Methods

* [get_symbol_digits](get_symbol_digits.md) · [get_min_stop_level](get_min_stop_level.md) · [get_max_lot_size](../10.%20Risk_Management/get_max_lot_size.md)

---

## ⚠️ Pitfalls

* **Unknown symbol** → `InvalidArgument`.
* **`spread` is in points** (`i32`), prices are `f64`.

---

## 📚 See Also

* [get_all_symbols](get_all_symbols.md) · [get_symbol_digits](get_symbol_digits.md)
