# Buy Market (`buy_market`)

> **Sugar method:** Open a BUY at market in one line; returns the order ticket.

**API Information:**

* **Method:** `sugar.buy_market(symbol, volume, comment, magic)`
* **Returns:** `Result<u64, MT5Error>` (order ticket)
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn buy_market(
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
| `symbol` | `Option<&str>` | Symbol, or `None` for the default symbol. |
| `volume` | `f64` | Lots. |
| `comment` | `&str` | Order comment (`""` for none). |
| `magic` | `u64` | EA magic number (`0` for none). |

| Output | Type | Description |
|--------|------|-------------|
| `u64` | — | The opened order ticket. |

---

## 🏛️ Essentials

* **What it is:** Market buy at the current ask; no SL/TP.
* **Why you need it:** The simplest way to go long.
* **Price:** fetched automatically (the ask); slippage defaults to 10 points.

---

## ⚡ Under the Hood

```text
MT5Sugar.buy_market()
  → send_order(Buy, price=ask, sl=None, tp=None)
    → MT5Service.place_order(OrderSendRequest)
      → MT5Account.order_send(...)  (gRPC)
  → checks returned_code == 10009, returns res.order
```

A non-`10009` retcode becomes `MT5Error::Api { code, message, mql_code }`.

---

## When to Use

* **Immediate long entry** at market.
* Building blocks for strategies (add SL/TP via [buy_market_with_pips](../5.%20Trading_With_SLTP/buy_market_with_pips.md)).

---

## 🔗 Usage Examples

### 1) Default symbol

```rust
let ticket = sugar.buy_market(None, 0.10, "entry", 1001).await?;
println!("opened #{ticket}");
```

### 2) Explicit symbol, no magic

```rust
let t = sugar.buy_market(Some("GBPUSD"), 0.05, "", 0).await?;
```

### 3) Handle rejection

```rust
match sugar.buy_market(None, 0.10, "x", 1).await {
    Ok(t) => println!("ok #{t}"),
    Err(e) => eprintln!("rejected: {e}"),
}
```

---

## Related Methods

* [sell_market](sell_market.md) · [buy_limit](buy_limit.md) · [buy_market_with_pips](../5.%20Trading_With_SLTP/buy_market_with_pips.md)

---

## ⚠️ Pitfalls

* **No SL/TP.** Use the SL/TP variants for protection.
* **`None` without a default** → `InvalidArgument`.
* **Volume rules.** Must satisfy the symbol's min/step/max, or the server rejects it.

---

## 📚 See Also

* [sell_market](sell_market.md) · [buy_market_with_sltp](../5.%20Trading_With_SLTP/buy_market_with_sltp.md)
