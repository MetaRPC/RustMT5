# Buy Stop (`buy_stop`)

> **Sugar method:** Place a BUY STOP pending order at `price` (above market).

**API Information:**

* **Method:** `sugar.buy_stop(symbol, volume, price, comment, magic)`
* **Returns:** `Result<u64, MT5Error>` (order ticket)
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn buy_stop(
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

* **What it is:** "Buy if price breaks up to `price`."
* **Why you need it:** Breakout entries above the market.
* **Rule:** `price` must be above the current ask.

---

## ⚡ Under the Hood

```text
MT5Sugar.buy_stop()
  → send_order(BuyStop, price=price) → MT5Service.place_order(...) → MT5Account.order_send(...)
```

---

## When to Use

* **Upside breakout** entries; stop-entry strategies.

---

## 🔗 Usage Examples

### 1) 20 pips above market

```rust
let ask = sugar.ask(Some("EURUSD")).await?;
let t = sugar.buy_stop(Some("EURUSD"), 0.10, ask + 0.0020, "breakout", 1005).await?;
```

### 2) Default symbol

```rust
let price = sugar.ask(None).await? + 0.0020;
let t = sugar.buy_stop(None, 0.05, price, "", 0).await?;
```

### 3) Cancel

```rust
sugar.close_position(t).await?;
```

---

## Related Methods

* [sell_stop](sell_stop.md) · [buy_limit](buy_limit.md) · [buy_market](buy_market.md)

---

## ⚠️ Pitfalls

* **Price below market** for a buy stop → rejection.
* Mind the symbol's stops/freeze level near the market ([get_min_stop_level](../9.%20Symbol_Information/get_min_stop_level.md)).

---

## 📚 See Also

* [sell_stop](sell_stop.md) · [buy_limit](buy_limit.md)
