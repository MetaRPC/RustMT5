# Ensure Symbol (`ensure_symbol`)

> **Sugar method:** Select a symbol and wait until it's synchronized and quoting, or time out.

**API Information:**

* **Method:** `sugar.ensure_symbol(symbol, wait)`
* **Returns:** `Result<Quote, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn ensure_symbol(&self, symbol: Option<&str>, wait: std::time::Duration) -> Result<Quote, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `Option<&str>` | Symbol, or `None` for the default symbol. |
| `wait` | `Duration` | Maximum time to wait for a usable quote. |

| Output | Type | Description |
|--------|------|-------------|
| `Quote` | struct | The first **usable** two-sided quote. |

---

## 🏛️ Essentials

* **What it is:** `symbol_select` → poll `symbol_is_synchronized` + `quote` every 500 ms until `quote.is_usable()`.
* **Why you need it:** A just-selected symbol isn't instantly ready — this removes the "empty/one-sided tick" race.
* **On timeout:** returns [`MT5Error::Timeout`] with the last seen (unusable) quote in the message.

---

## ⚡ Under the Hood

```text
MT5Sugar.ensure_symbol(symbol, wait)
  → MT5Service.symbol_select(sym, true)
  → loop (≤ wait, every 500 ms):
       symbol_is_synchronized(sym) && quote(sym).is_usable() → return Quote
  → else MT5Error::Timeout
```

---

## 🔗 Usage Examples

### 1) Make a symbol ready before trading

```rust
use std::time::Duration;

let q = sugar.ensure_symbol(Some("EURUSD"), Duration::from_secs(5)).await?;
println!("ready: bid {:.5} ask {:.5}", q.bid, q.ask);
```

### 2) Guarded entry

```rust
use std::time::Duration;

if sugar.ensure_symbol(Some("GBPUSD"), Duration::from_secs(5)).await.is_ok() {
    sugar.buy_market(Some("GBPUSD"), 0.10, "entry", 1).await?;
}
```

### 3) Handle the timeout

```rust
use std::time::Duration;

match sugar.ensure_symbol(Some("EXOTIC"), Duration::from_secs(3)).await {
    Ok(q) => println!("ok {:.5}", q.mid()),
    Err(e) => println!("not ready: {e}"),
}
```

---

## Related Methods

* [quote](quote.md) · [subscribe_quotes](subscribe_quotes.md)
* [is_symbol_available](../9.%20Symbol_Information/is_symbol_available.md)

---

## ⚠️ Pitfalls

* **Pick a sane `wait`** — too short on a cold symbol returns `Timeout`.
* **`None` without a default** → `InvalidArgument`.

---

## 📚 See Also

* [quote](quote.md) · [subscribe_quotes](subscribe_quotes.md)
