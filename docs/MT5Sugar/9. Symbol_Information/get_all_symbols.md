# Get All Symbols (`get_all_symbols`)

> **Sugar method:** Every available symbol name, paged automatically.

**API Information:**

* **Method:** `sugar.get_all_symbols()`
* **Returns:** `Result<Vec<String>, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn get_all_symbols(&self) -> Result<Vec<String>, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `Vec<String>` | — | All symbol names known to the account. |

---

## 🏛️ Essentials

* **What it is:** Pages through `symbol_params_many` (100 per page) and collects the names.
* **Why you need it:** Build a picker, search names, snapshot the universe.
* **Can be large** — filter client-side.

---

## ⚡ Under the Hood

```text
MT5Sugar.get_all_symbols()
  → loop pages: MT5Service.symbol_params_many(None, Some(page), Some(100))
    → MT5Account.symbol_params_many(...)  (gRPC)
  → collect names until symbols_total reached (or a short page)
```

---

## When to Use

* **Symbol discovery / search.**

---

## 🔗 Usage Examples

### 1) Count

```rust
let all = sugar.get_all_symbols().await?;
println!("{} symbols", all.len());
```

### 2) Filter majors

```rust
let majors: Vec<_> = sugar.get_all_symbols().await?
    .into_iter().filter(|s| s.starts_with("EUR") || s.starts_with("GBP")).collect();
```

### 3) Membership

```rust
let has = sugar.get_all_symbols().await?.iter().any(|s| s == "XAUUSD");
```

---

## Related Methods

* [is_symbol_available](is_symbol_available.md) · [get_symbol_info](get_symbol_info.md)

---

## ⚠️ Pitfalls

* **Potentially large** — don't render unfiltered.
* **Paging cost** — several round-trips on big brokers; cache the result.

---

## 📚 See Also

* [is_symbol_available](is_symbol_available.md) · [get_symbol_info](get_symbol_info.md)
