# Bid (`bid`)

> **Sugar method:** Current bid price for a symbol (or the default symbol).

**API Information:**

* **Method:** `sugar.bid(symbol)`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn bid(&self, symbol: Option<&str>) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `Option<&str>` | Symbol, or `None` to use the default symbol. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Best sell price. |

---

## 🏛️ Essentials

* **What it is:** The price you sell at.
* **Why you need it:** Sell entries, spread/exit math, live price display.
* **Default symbol:** passing `None` requires a default set via `with_default_symbol(..)`, else `InvalidArgument`.

---

## ⚡ Under the Hood

```text
MT5Sugar.bid(symbol)
  → resolve_symbol(symbol)              // default-symbol fallback
  → MT5Service.symbol_tick(sym).bid
    → MT5Account.symbol_info_tick(...)  (gRPC)
```

---

## When to Use

* **Sell-side pricing** and spread checks.
* Quick price readout in logs/UI.

---

## 🔗 Usage Examples

### 1) Default symbol

```rust
let sugar = /* ... */ .with_default_symbol("EURUSD");
println!("bid {:.5}", sugar.bid(None).await?);
```

### 2) Explicit symbol

```rust
println!("GBPUSD bid {:.5}", sugar.bid(Some("GBPUSD")).await?);
```

### 3) Spread by hand

```rust
let (b, a) = (sugar.bid(None).await?, sugar.ask(None).await?);
println!("spread {:.5}", a - b);
```

---

## Related Methods

* [ask](ask.md) · [spread](spread.md)

---

## ⚠️ Pitfalls

* **`None` without a default** → `InvalidArgument`.
* **Select the symbol first** (Market Watch) or the tick may be empty.

---

## 📚 See Also

* [ask](ask.md) · [spread](spread.md)
