# Quote (`quote`)

> **Sugar method:** A compact one-shot `Quote` (bid/ask/last/volume/time) for a symbol.

**API Information:**

* **Method:** `sugar.quote(symbol)`
* **Returns:** `Result<Quote, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn quote(&self, symbol: Option<&str>) -> Result<Quote, MT5Error>
```

### `Quote`

```rust
pub struct Quote {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume: f64,   // volume_real
    pub time: i64,     // unix seconds
    pub time_msc: i64, // unix milliseconds
    pub flags: u32,
}
// helpers: q.is_usable()  q.spread()  q.mid()
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `Option<&str>` | Symbol, or `None` for the default symbol. |

| Output | Type | Description |
|--------|------|-------------|
| `Quote` | struct | One tidy snapshot with `is_usable()` / `spread()` / `mid()` helpers. |

---

## 🏛️ Essentials

* **What it is:** `symbol_tick` mapped into a friendly `Quote`.
* **Why you need it:** bid/ask/spread/mid in one value, with a usability check for flaky demo feeds.
* **`is_usable()`:** `bid > 0 && ask > 0 && ask > bid` — guards against the `bid == ask`/zero ticks demo servers emit.

---

## ⚡ Under the Hood

```text
MT5Sugar.quote(symbol)
  → resolve_symbol(symbol)
  → MT5Service.symbol_tick(sym)  → Quote::from_tick(sym, &tick)
    → MT5Account.symbol_info_tick(...)
```

---

## 🔗 Usage Examples

### 1) Basic

```rust
let q = sugar.quote(Some("EURUSD")).await?;
println!("{} bid {:.5} ask {:.5} mid {:.5} spread {:.5}", q.symbol, q.bid, q.ask, q.mid(), q.spread());
```

### 2) Skip unusable ticks

```rust
let q = sugar.quote(None).await?;
if !q.is_usable() { println!("flaky quote — wait for a two-sided one"); }
```

### 3) Default symbol

```rust
let sugar = /* ... */.with_default_symbol("EURUSD");
println!("mid {:.5}", sugar.quote(None).await?.mid());
```

---

## Related Methods

* [ensure_symbol](ensure_symbol.md) — wait for a usable quote · [subscribe_quotes](subscribe_quotes.md) — a stream of `Quote`s
* [bid](bid.md) · [ask](ask.md) · [spread](spread.md)

---

## ⚠️ Pitfalls

* **`None` without a default** → `InvalidArgument`.
* **Select the symbol first** (or use [ensure_symbol](ensure_symbol.md)) — an unselected symbol may return an unusable quote.

---

## 📚 See Also

* [ensure_symbol](ensure_symbol.md) · [subscribe_quotes](subscribe_quotes.md)
