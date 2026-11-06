# Calculate Position Size (`calculate_position_size`)

> **Sugar method:** Lot size that risks `risk_percent`% of balance over an `sl_pips` stop — rounded to the volume step and clamped to min/max.

**API Information:**

* **Method:** `sugar.calculate_position_size(symbol, risk_percent, sl_pips)`
* **Returns:** `Result<f64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn calculate_position_size(
    &self,
    symbol: &str,
    risk_percent: f64,
    sl_pips: f64,
) -> Result<f64, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `&str` | Symbol name. |
| `risk_percent` | `f64` | Percent of **balance** to risk (e.g. `1.0` = 1%). |
| `sl_pips` | `f64` | Stop distance in pips. |

| Output | Type | Description |
|--------|------|-------------|
| `f64` | — | Lot size (step-rounded, clamped to `[volume_min, volume_max]`). |

---

## 🏛️ Essentials

* **What it is:** The core risk-sizing helper.
* **Formula:** `risk_amount = balance × risk%/100`; `pip_value = point × 10 × contract_size`; `lots = round(risk_amount / (sl_pips × pip_value) / step) × step`, clamped to min/max.
* **Guards:** if `sl_pips ≤ 0` or `pip_value ≤ 0`, returns `volume_min`.

---

## ⚡ Under the Hood

```text
MT5Sugar.calculate_position_size(symbol, risk_percent, sl_pips)
  → balance()                     // MT5Service.account_double(AccountBalance)
  → get_symbol_info(symbol)       // point, contract_size, volume_min/step/max
  → compute & round lots
```

---

## When to Use

* **Every risk-based entry** — pair with the pip-bracket trading methods.

---

## 🔗 Usage Examples

### 1) 1% over a 20-pip stop

```rust
let lots = sugar.calculate_position_size("EURUSD", 1.0, 20.0).await?;
println!("lots {lots:.2}");
```

### 2) Size then enter

```rust
let lots = sugar.calculate_position_size("EURUSD", 1.0, 20.0).await?;
sugar.buy_market_with_pips(Some("EURUSD"), lots, Some(20.0), Some(40.0), "sized", 3001).await?;
```

### 3) Conservative vs aggressive

```rust
let cons = sugar.calculate_position_size("EURUSD", 0.5, 30.0).await?;
let aggr = sugar.calculate_position_size("EURUSD", 2.0, 15.0).await?;
println!("cons {cons:.2}, aggr {aggr:.2}");
```

---

## Related Methods

* [calculate_required_margin](calculate_required_margin.md) · [can_open_position](can_open_position.md) · [buy_market_with_pips](../5.%20Trading_With_SLTP/buy_market_with_pips.md)

---

## ⚠️ Pitfalls

* **Risk is on balance**, not equity.
* **Approximate pip value** (`point × 10 × contract_size`) — for cross-currency instruments the true per-pip value may differ; validate for exotics.
* **Degenerate inputs** (`sl_pips ≤ 0`) fall back to `volume_min`.
* **Still check margin** with [calculate_required_margin](calculate_required_margin.md) before sending.

---

## 📚 See Also

* [can_open_position](can_open_position.md) · [calculate_sltp](calculate_sltp.md)
