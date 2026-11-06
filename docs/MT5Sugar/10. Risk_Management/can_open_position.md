# Can Open Position (`can_open_position`)

> **Sugar method:** Validate whether a BUY of `volume` lots could be opened; returns `(can_open, reason)`.

**API Information:**

* **Method:** `sugar.can_open_position(symbol, volume)`
* **Returns:** `Result<(bool, String), MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn can_open_position(&self, symbol: &str, volume: f64) -> Result<(bool, String), MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `&str` | Symbol name. |
| `volume` | `f64` | Lots to validate. |

| Output | Type | Description |
|--------|------|-------------|
| `(bool, String)` | — | `(true, "OK")` if it would be accepted, else `(false, reason)`. |

---

## 🏛️ Essentials

* **What it is:** Builds a market-BUY trade request (IOC fill, GTC time) and runs `check_order`.
* **Why you need it:** Catch "no money" / "invalid volume" / "market closed" before actually sending.
* **Success condition:** the check result's `returned_code == 0`.

---

## ⚡ Under the Hood

```text
MT5Sugar.can_open_position(symbol, volume)
  → tick = symbol_tick(symbol)
  → build MrpcMqlTradeRequest { action=Deal, symbol, volume, price=ask,
       order_type=Buy(Tf), type_filling=IOC, type_time=GTC }
  → MT5Service.check_order(...).mql_trade_check_result
    → MT5Account.order_check(...)  (gRPC)
  → code==0 → (true, "OK") else (false, "check failed: code=.. ..")
```

> Note the check uses retcode **0** as the "would succeed" signal for the *check* (distinct from the send success code `10009`).

---

## When to Use

* **Pre-trade gate** in front of `buy_market`/`buy_market_with_pips`.

---

## 🔗 Usage Examples

### 1) Gate an entry

```rust
let (ok, why) = sugar.can_open_position("EURUSD", 0.50).await?;
if ok {
    sugar.buy_market(Some("EURUSD"), 0.50, "validated", 3002).await?;
} else {
    println!("cannot open: {why}");
}
```

### 2) Find the largest size that passes

```rust
let mut v = 1.0;
while v >= 0.01 {
    if sugar.can_open_position("EURUSD", v).await?.0 { break; }
    v -= 0.1;
}
println!("max passing size ≈ {v:.2}");
```

### 3) Log the reason

```rust
let (_ok, why) = sugar.can_open_position("EURUSD", 100.0).await?;
println!("{why}");
```

---

## Related Methods

* [calculate_required_margin](calculate_required_margin.md) · [calculate_position_size](calculate_position_size.md) · low-level [order_check](../../MT5Account/5.%20Trading_Operations/order_check.md)

---

## ⚠️ Pitfalls

* **BUY-only check.** It validates a buy; a sell's margin can differ slightly.
* **Check code is `0`**, not `10009` — that's the order-check convention.
* **Validation ≠ execution** — market can move between check and send.

---

## 📚 See Also

* [calculate_position_size](calculate_position_size.md) · [calculate_required_margin](calculate_required_margin.md)
