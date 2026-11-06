# Is Symbol Available (`is_symbol_available`)

> **Sugar method:** Does a symbol exist on this account?

**API Information:**

* **Method:** `sugar.is_symbol_available(symbol)`
* **Returns:** `Result<bool, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn is_symbol_available(&self, symbol: &str) -> Result<bool, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `&str` | Symbol name to check. |

| Output | Type | Description |
|--------|------|-------------|
| `bool` | — | `true` if the symbol exists. |

---

## 🏛️ Essentials

* **What it is:** Thin check over `symbol_exist(...).exists`.
* **Why you need it:** Validate user input before selecting/quoting/trading.
* **`false` ≠ error.**

---

## ⚡ Under the Hood

```text
MT5Sugar.is_symbol_available(symbol)
  → MT5Service.symbol_exist(symbol).exists
    → MT5Account.symbol_exist(...)  (gRPC)
```

---

## When to Use

* **Pre-flight validation** of a symbol name.

---

## 🔗 Usage Examples

### 1) Validate

```rust
if !sugar.is_symbol_available("EURUSD").await? { println!("no such symbol"); }
```

### 2) Pick first available

```rust
let mut chosen = None;
for s in ["EURUSD", "EURUSD.m", "EURUSD-5"] {
    if sugar.is_symbol_available(s).await? { chosen = Some(s); break; }
}
println!("using {chosen:?}");
```

### 3) Guard a trade

```rust
if sugar.is_symbol_available("XAUUSD").await? {
    sugar.buy_market(Some("XAUUSD"), 0.01, "", 0).await?;
}
```

---

## Related Methods

* [get_all_symbols](get_all_symbols.md) · [get_symbol_info](get_symbol_info.md)

---

## ⚠️ Pitfalls

* **Exact name** (broker suffixes matter).
* **Existence ≠ Market Watch visibility** — selecting may still be needed before quoting (the low-level `symbol_select`).

---

## 📚 See Also

* [get_all_symbols](get_all_symbols.md) · [get_symbol_info](get_symbol_info.md)
