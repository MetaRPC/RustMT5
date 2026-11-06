# Sell Market (`sell_market`)

> **Sugar method:** Open a SELL at market in one line; returns the order ticket.

**API Information:**

* **Method:** `sugar.sell_market(symbol, volume, comment, magic)`
* **Returns:** `Result<u64, MT5Error>` (order ticket)
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn sell_market(
    &self,
    symbol: Option<&str>,
    volume: f64,
    comment: &str,
    magic: u64,
) -> Result<u64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `Option<&str>` | Symbol, or `None` for the default. |
| `volume` | `f64` | Lots. |
| `comment` | `&str` | Comment (`""` = none). |
| `magic` | `u64` | Magic number (`0` = none). |

| Output | Type | Description |
|--------|------|-------------|
| `u64` | — | The opened order ticket. |

---

## 🏛️ Essentials

* **What it is:** Market sell at the current bid; no SL/TP.
* **Why you need it:** The simplest way to go short.
* **Price:** the bid is fetched automatically; slippage defaults to 10 points.

---

## ⚡ Under the Hood

```text
MT5Sugar.sell_market()
  → send_order(Sell, price=bid, sl=None, tp=None)
    → MT5Service.place_order(...) → MT5Account.order_send(...)
  → checks 10009, returns res.order
```

---

## When to Use

* **Immediate short entry** at market.

---

## 🔗 Usage Examples

### 1) Default symbol

```rust
let t = sugar.sell_market(None, 0.10, "short", 1002).await?;
```

### 2) Explicit symbol

```rust
let t = sugar.sell_market(Some("USDJPY"), 0.02, "", 0).await?;
```

### 3) Rejection handling

```rust
if let Err(e) = sugar.sell_market(None, 0.10, "x", 1).await { eprintln!("rejected: {e}"); }
```

---

## Related Methods

* [buy_market](buy_market.md) · [sell_limit](sell_limit.md) · [sell_market_with_pips](../5.%20Trading_With_SLTP/sell_market_with_pips.md)

---

## ⚠️ Pitfalls

* **No SL/TP** — use the SL/TP variants to protect the trade.
* **`None` without a default** → `InvalidArgument`.
* **Volume rules** apply.

---

## 📚 See Also

* [buy_market](buy_market.md) · [sell_market_with_sltp](../5.%20Trading_With_SLTP/sell_market_with_sltp.md)
