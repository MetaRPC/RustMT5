# Sell Limit (`sell_limit`)

> **Sugar method:** Place a SELL LIMIT pending order at `price` (above market).

**API Information:**

* **Method:** `sugar.sell_limit(symbol, volume, price, comment, magic)`
* **Returns:** `Result<u64, MT5Error>` (order ticket)
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn sell_limit(
    &self,
    symbol: Option<&str>,
    volume: f64,
    price: f64,
    comment: &str,
    magic: u64,
) -> Result<u64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `Option<&str>` | Symbol, or `None` for default. |
| `volume` | `f64` | Lots. |
| `price` | `f64` | Trigger price — **above** current market. |
| `comment` | `&str` | Comment. |
| `magic` | `u64` | Magic number. |

| Output | Type | Description |
|--------|------|-------------|
| `u64` | — | Pending order ticket. |

---

## 🏛️ Essentials

* **What it is:** "Sell if price rises to `price`."
* **Why you need it:** Enter short on a rally without watching the screen.
* **Rule:** `price` must be above the current bid.

---

## ⚡ Under the Hood

```text
MT5Sugar.sell_limit()
  → send_order(SellLimit, price=price) → MT5Service.place_order(...) → MT5Account.order_send(...)
```

---

## When to Use

* **Rally entries**, grid levels above market.

---

## 🔗 Usage Examples

### 1) 10 pips above market

```rust
let ask = sugar.ask(Some("EURUSD")).await?;
let t = sugar.sell_limit(Some("EURUSD"), 0.10, ask + 0.0010, "rally", 1004).await?;
```

### 2) Default symbol

```rust
let price = sugar.ask(None).await? + 0.0015;
let t = sugar.sell_limit(None, 0.05, price, "", 0).await?;
```

### 3) Cancel

```rust
sugar.close_position(t).await?;  // deletes the pending
```

---

## Related Methods

* [buy_limit](buy_limit.md) · [sell_stop](sell_stop.md) · [sell_market](sell_market.md)

---

## ⚠️ Pitfalls

* **Price below market** for a sell limit → rejection.
* **Pending ≠ position** until filled.

---

## 📚 See Also

* [buy_limit](buy_limit.md) · [sell_stop](sell_stop.md)
