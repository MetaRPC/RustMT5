# Get Max Lot Size (`get_max_lot_size`)

> **Sugar method:** The maximum lot size allowed for a symbol.

**API Information:**

* **Method:** `sugar.get_max_lot_size(symbol)`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_max_lot_size(&self, symbol: &str) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `&str` | Symbol name. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Symbol's `volume_max`. |

---

## 🏛️ Essentials

* **What it is:** The `volume_max` field from [get_symbol_info](../9.%20Symbol_Information/get_symbol_info.md).
* **Why you need it:** Clamp computed sizes to the broker's ceiling.

---

## ⚡ Under the Hood

```text
MT5Sugar.get_max_lot_size(symbol)
  → get_symbol_info(symbol).volume_max
    → MT5Service.symbol_params_many(...)  (gRPC)
```

---

## When to Use

* **Clamping** a sizing result; validating user input.

---

## 🔗 Usage Examples

### 1) Read it

```rust
println!("max lot {}", sugar.get_max_lot_size("EURUSD").await?);
```

### 2) Clamp a size

```rust
let want = 25.0;
let lots = want.min(sugar.get_max_lot_size("EURUSD").await?);
```

### 3) Validate input

```rust
let req = 50.0;
if req > sugar.get_max_lot_size("EURUSD").await? { println!("exceeds max lot"); }
```

---

## Related Methods

* [get_symbol_info](../9.%20Symbol_Information/get_symbol_info.md) · [calculate_position_size](calculate_position_size.md)

---

## ⚠️ Pitfalls

* **Also mind `volume_min`/`volume_step`** — max alone isn't the whole volume rule.

---

## 📚 See Also

* [calculate_position_size](calculate_position_size.md) · [get_symbol_info](../9.%20Symbol_Information/get_symbol_info.md)
