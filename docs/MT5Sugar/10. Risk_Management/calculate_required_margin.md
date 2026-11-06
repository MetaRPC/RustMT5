# Calculate Required Margin (`calculate_required_margin`)

> **Sugar method:** Margin needed to open `volume` lots of a symbol (defaults to BUY), at the current price.

**API Information:**

* **Method:** `sugar.calculate_required_margin(symbol, volume, order_type)`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn calculate_required_margin(
    &self,
    symbol: &str,
    volume: f64,
    order_type: Option<pb::EnumOrderTypeTf>,
) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `&str` | Symbol name. |
| `volume` | `f64` | Lots. |
| `order_type` | `Option<EnumOrderTypeTf>` | Direction; `None` → `OrderTypeTfBuy`. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Required margin in the deposit currency. |

---

## 🏛️ Essentials

* **What it is:** Fetches the current price (ask for buy, bid for sell) and asks the server to compute margin.
* **Why you need it:** Confirm a size fits your [free_margin](../2.%20Account_Properties/free_margin.md) before trading.
* **Default side:** BUY when `order_type` is `None`.

---

## ⚡ Under the Hood

```text
MT5Sugar.calculate_required_margin(symbol, volume, order_type)
  → price = symbol_tick(symbol).ask|bid
  → MT5Service.calculate_margin(symbol, order_type, volume, price).margin
    → MT5Account.order_calc_margin(...)  (gRPC)
```

---

## When to Use

* **Pre-trade fit check**; sizing to margin.

---

## 🔗 Usage Examples

### 1) Margin for 1 lot

```rust
let m = sugar.calculate_required_margin("EURUSD", 1.0, None).await?;
println!("needs {m:.2}");
```

### 2) Fits free margin?

```rust
let need = sugar.calculate_required_margin("EURUSD", 0.50, None).await?;
if sugar.free_margin().await? < need { println!("too big"); }
```

### 3) Sell-side margin

```rust
use metarpc_mt5::proto::mt5_term_api::EnumOrderTypeTf as TF;
let m = sugar.calculate_required_margin("EURUSD", 0.10, Some(TF::OrderTypeTfSell)).await?;
```

---

## Related Methods

* [free_margin](../2.%20Account_Properties/free_margin.md) · [calculate_position_size](calculate_position_size.md) · [can_open_position](can_open_position.md)

---

## ⚠️ Pitfalls

* **Clean-account estimate** — ignores existing positions (hedging may make the real figure lower).
* **Keep a buffer** vs free margin.

---

## 📚 See Also

* [calculate_position_size](calculate_position_size.md) · [can_open_position](can_open_position.md)
