# Presets & Shared Helpers

> `StrategyPreset` (what to trade) and `RiskPreset` (how much, how to protect) — plus the ready-made catalog and the `manage_position` worker all three orchestrators share.

→ Source: [`src/helpers/strategies/mod.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/helpers/strategies/mod.rs)

!!! warning "Demo only"
    These drive demonstration orchestrators. Run on a **demo account** and review every value before trusting it.

---

## `StrategyPreset` — WHAT to trade

```rust
pub struct StrategyPreset {
    pub symbol: String,
    pub use_market: bool,        // true = market entry, false = pending at entry_price
    pub entry_price: Option<f64>,// pending price (when use_market = false)
    pub lots: Option<f64>,       // fixed lots; None = size by risk
    pub magic: u64,
    pub deviation_pips: f64,     // intended slippage budget (pips)
    pub comment: String,
}
```

**Builder methods** (chainable):

| Method | Effect |
| --- | --- |
| `StrategyPreset::new(symbol)` | Defaults: market entry, auto-size, magic 0, deviation 2.0, no comment. |
| `.market()` | Market entry (clears `entry_price`). |
| `.limit_at(price)` | Pending entry at `price` (`use_market = false`). |
| `.lots(l)` | Fixed lot size (skips risk sizing). |
| `.magic(m)` | Set the magic number. |
| `.comment(c)` | Set the order comment. |

```rust
use metarpc_mt5::strategies::StrategyPreset;

let sp = StrategyPreset::new("EURUSD").market().magic(777).comment("demo");
let pending = StrategyPreset::new("EURUSD").limit_at(1.0800).lots(0.10);
```

---

## `RiskPreset` — HOW MUCH to risk & how to protect

```rust
pub struct RiskPreset {
    pub risk_percent: f64,          // % of balance to risk (used when lots are auto-sized)
    pub sl_pips: f64,               // stop distance in pips
    pub tp_pips: Option<f64>,       // optional take-profit in pips
    pub trailing_pips: Option<f64>, // trail distance; None = no trailing
    pub be_trigger_pips: Option<f64>, // move to breakeven once this profit is reached
    pub be_plus_pips: f64,          // breakeven offset (entry + be_plus_pips)
}
```

**Builder methods:**

| Method | Effect |
| --- | --- |
| `RiskPreset::new(risk_percent, sl_pips)` | Base preset; no TP/trail/breakeven. |
| `.tp(pips)` | Set take-profit distance. |
| `.trailing(pips)` | Enable a trailing stop at this distance. |
| `.breakeven(trigger_pips, plus_pips)` | Move SL to `entry + plus_pips` once `trigger_pips` profit is reached. |

```rust
use metarpc_mt5::strategies::RiskPreset;

let rp = RiskPreset::new(1.0, 20.0).tp(40.0).trailing(15.0).breakeven(10.0, 2.0);
```

---

## Ready-made catalog

| Function | Kind | Value |
| --- | --- | --- |
| `market_eurusd()` | `StrategyPreset` | Market EURUSD, magic 777. |
| `limit_eurusd(price)` | `StrategyPreset` | BUY LIMIT EURUSD at `price`, magic 778. |
| `conservative()` | `RiskPreset` | 0.5% risk, SL 25, TP 50. |
| `balanced()` | `RiskPreset` | 1.0% risk, SL 20, TP 40, trail 15, breakeven 10/+2. |
| `aggressive()` | `RiskPreset` | 2.0% risk, SL 15, TP 45, trail 10, breakeven 8/+1. |

```rust
use metarpc_mt5::strategies::{market_eurusd, balanced, run_market_one_shot};

let ticket = run_market_one_shot(&sugar, &market_eurusd(), &balanced()).await?;
```

---

## Shared helpers

The orchestrators share a few `pub(crate)` helpers (internal, but worth understanding):

| Helper | What it does |
| --- | --- |
| `size_lot(sugar, sp, rp)` | `sp.lots` if set, else `sugar.calculate_position_size(symbol, risk%, sl_pips)`. |
| `pip_size(sugar, symbol)` | Pip size = `get_symbol_info(symbol).point × 10`. |
| `entry_price(sugar, ticket, symbol)` | The opened position's `price_open` (falls back to current ask). |

### `manage_position`

The common management loop used by all three orchestrators:

```rust
async fn manage_position(
    sugar, ticket, symbol, entry, pip, rp: &RiskPreset,
    manage_secs: u64,         // how long to manage
    trail_activate_pips: f64, // profit cushion before trailing starts
) -> Result<(), MT5Error>
```

For a **BUY** position, every 500 ms until `manage_secs` elapses:

1. **Auto-breakeven** — once `(mid − entry)` reaches `be_trigger_pips`, move SL to `entry + be_plus_pips` (once), if that's above the current stop.
2. **Trailing (ratchet up only)** — once profit ≥ `trail_activate_pips`, set SL to `mid − trailing_pips × pip` whenever that's higher than the current stop; the stop never loosens.

The stop is tracked locally (`cur_sl`, starting at `entry − sl_pips × pip`) and sent via `modify_position_sl` **only when it actually moves** — avoiding redundant modify calls. A preset with neither `be_trigger_pips` nor `trailing_pips` simply holds for the duration.

> The difference between orchestrators is mostly **when** trailing activates (`trail_activate_pips`) and **what happens before** management (immediate market, or wait-for-fill).

---

## See also

- [Strategies — Master Overview](Strategies.Master.Overview.md)
- [Market One-Shot](Orchestrators/MarketOneShot.md) · [Bracket + Trailing](Orchestrators/BracketTrailing.md) · [Pending Bracket](Orchestrators/PendingBracket.md)
- [MT5Sugar — Risk Management](../MT5Sugar/10.%20Risk_Management/calculate_position_size.md)
