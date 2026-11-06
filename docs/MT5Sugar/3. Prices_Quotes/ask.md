# Ask (`ask`)

> **Sugar method:** Current ask price for a symbol (or the default symbol).

**API Information:**

* **Method:** `sugar.ask(symbol)`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn ask(&self, symbol: Option<&str>) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `Option<&str>` | Symbol, or `None` for the default symbol. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Best buy price. |

---

## 🏛️ Essentials

* **What it is:** The price you buy at.
* **Why you need it:** Buy entries, spread/exit math.
* **Default symbol:** `None` needs a default set, else `InvalidArgument`.

---

## ⚡ Under the Hood

```text
MT5Sugar.ask(symbol)
  → resolve_symbol(symbol)
  → MT5Service.symbol_tick(sym).ask
    → MT5Account.symbol_info_tick(...)  (gRPC)
```

---

## When to Use

* **Buy-side pricing**, SL/TP distance from entry.

---

## 🔗 Usage Examples

### 1) Default symbol

```rust
println!("ask {:.5}", sugar.ask(None).await?);
```

### 2) Explicit symbol

```rust
println!("XAUUSD ask {:.2}", sugar.ask(Some("XAUUSD")).await?);
```

### 3) Mid price

```rust
let mid = (sugar.bid(None).await? + sugar.ask(None).await?) / 2.0;
println!("mid {mid:.5}");
```

---

## Related Methods

* [bid](bid.md) · [spread](spread.md)

---

## ⚠️ Pitfalls

* **`None` without a default** → `InvalidArgument`.
* **Select the symbol first.**

---

## 📚 See Also

* [bid](bid.md) · [spread](spread.md)
