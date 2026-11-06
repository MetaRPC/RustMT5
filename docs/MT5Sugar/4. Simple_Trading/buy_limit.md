# Buy Limit (`buy_limit`)

> **Sugar method:** Place a BUY LIMIT pending order at `price` (below market).

**API Information:**

* **Method:** `sugar.buy_limit(symbol, volume, price, comment, magic)`
* **Returns:** `Result<u64, MT5Error>` (order ticket)
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn buy_limit(
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
| `price` | `f64` | Trigger price — **below** current market. |
| `comment` | `&str` | Comment. |
| `magic` | `u64` | Magic number. |

| Output | Type | Description |
|--------|------|-------------|
| `u64` | — | Pending order ticket. |

---

## 🏛️ Essentials

* **What it is:** "Buy if price drops to `price`."
* **Why you need it:** Enter long on a pullback without watching the screen.
* **Rule:** `price` must be below the current ask (else the server rejects it).

---

## ⚡ Under the Hood

```text
MT5Sugar.buy_limit()
  → send_order(BuyLimit, price=price, sl=None, tp=None)
    → MT5Service.place_order(...) → MT5Account.order_send(...)
```

---

## When to Use

* **Pullback entries**, grid levels, scaling in below market.

---

## 🔗 Usage Examples

### 1) 10 pips below market

```rust
let bid = sugar.bid(Some("EURUSD")).await?;
let t = sugar.buy_limit(Some("EURUSD"), 0.10, bid - 0.0010, "dip", 1003).await?;
println!("pending #{t}");
```

### 2) Default symbol

```rust
let price = sugar.bid(None).await? - 0.0015;
let t = sugar.buy_limit(None, 0.05, price, "", 0).await?;
```

### 3) Cancel later

```rust
let t = sugar.buy_limit(Some("EURUSD"), 0.1, 1.0800, "x", 1).await?;
sugar.close_position(t).await?;  // deletes the resting pending
```

---

## Related Methods

* [sell_limit](sell_limit.md) · [buy_stop](buy_stop.md) · [buy_market](buy_market.md)

---

## ⚠️ Pitfalls

* **Price above market** for a buy limit → rejection.
* **Pending ≠ position** until filled; check with [has_open_position](../7.%20Position_Information/has_open_position.md).

---

## 📚 See Also

* [sell_limit](sell_limit.md) · [buy_stop](buy_stop.md)
