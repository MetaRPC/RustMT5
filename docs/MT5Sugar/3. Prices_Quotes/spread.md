# Spread (`spread`)

> **Sugar method:** Current spread (`ask − bid`) in price units.

**API Information:**

* **Method:** `sugar.spread(symbol)`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn spread(&self, symbol: Option<&str>) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `Option<&str>` | Symbol, or `None` for the default symbol. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | `ask − bid`, in **price units** (not points). |

---

## 🏛️ Essentials

* **What it is:** The cost of crossing the book right now, in price.
* **Why you need it:** Avoid entering when the spread is abnormally wide.
* **Points:** divide by the symbol's `point` for spread in points (see [get_symbol_info](../9.%20Symbol_Information/get_symbol_info.md)).

---

## ⚡ Under the Hood

```text
MT5Sugar.spread(symbol)
  → resolve_symbol(symbol)
  → MT5Service.symbol_tick(sym)  → (ask - bid)
    → MT5Account.symbol_info_tick(...)  (gRPC)
```

---

## When to Use

* **Pre-trade filter** — skip wide spreads.
* Execution-cost estimates.

---

## 🔗 Usage Examples

### 1) Basic

```rust
println!("spread {:.5}", sugar.spread(None).await?);
```

### 2) In points

```rust
let si = sugar.get_symbol_info("EURUSD").await?;
let pts = sugar.spread(Some("EURUSD")).await? / si.point;
println!("spread {pts:.1} points");
```

### 3) Spread gate

```rust
if sugar.spread(Some("EURUSD")).await? > 0.0003 { println!("spread too wide — skip"); }
```

---

## Related Methods

* [bid](bid.md) · [ask](ask.md) · [get_symbol_info](../9.%20Symbol_Information/get_symbol_info.md)

---

## ⚠️ Pitfalls

* **Price units, not points.** Convert with `point` if you think in points.
* **`None` without a default** → `InvalidArgument`.

---

## 📚 See Also

* [bid](bid.md) · [ask](ask.md)
