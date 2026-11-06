# Sell Stop (`sell_stop`)

> **Sugar method:** Place a SELL STOP pending order at `price` (below market).

**API Information:**

* **Method:** `sugar.sell_stop(symbol, volume, price, comment, magic)`
* **Returns:** `Result<u64, MT5Error>` (order ticket)
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn sell_stop(
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

* **What it is:** "Sell if price breaks down to `price`."
* **Why you need it:** Downside breakout entries.
* **Rule:** `price` must be below the current bid.

---

## ⚡ Under the Hood

```text
MT5Sugar.sell_stop()
  → send_order(SellStop, price=price) → MT5Service.place_order(...) → MT5Account.order_send(...)
```

---

## When to Use

* **Downside breakout** entries; stop-entry strategies.

---

## 🔗 Usage Examples

### 1) 20 pips below market

```rust
let bid = sugar.bid(Some("EURUSD")).await?;
let t = sugar.sell_stop(Some("EURUSD"), 0.10, bid - 0.0020, "breakdown", 1006).await?;
```

### 2) Default symbol

```rust
let price = sugar.bid(None).await? - 0.0020;
let t = sugar.sell_stop(None, 0.05, price, "", 0).await?;
```

### 3) Cancel

```rust
sugar.close_position(t).await?;
```

---

## Related Methods

* [buy_stop](buy_stop.md) · [sell_limit](sell_limit.md) · [sell_market](sell_market.md)

---

## ⚠️ Pitfalls

* **Price above market** for a sell stop → rejection.
* Respect the symbol's stops/freeze level ([get_min_stop_level](../9.%20Symbol_Information/get_min_stop_level.md)).

---

## 📚 See Also

* [buy_stop](buy_stop.md) · [sell_limit](sell_limit.md)
