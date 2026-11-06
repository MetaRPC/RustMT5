# Bracket + Trailing Orchestrator

> Enter with a bracket, but **only start trailing once the trade is `activate_at_pips` in profit** — "let it prove itself before trailing".

!!! warning "Demonstration only — not production ready"
    Shows the strategy idea and the API calls behind it. **Demo accounts only.**

→ Source: [`src/helpers/strategies/bracket_trailing.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/helpers/strategies/bracket_trailing.rs)

---

## What it does

Same market-bracket entry as [Market One-Shot](MarketOneShot.md), but the **trailing stop stays off** until the position has earned a cushion of `activate_at_pips`. Auto-breakeven (from `be_trigger_pips`) still applies earlier, at its own smaller threshold. This avoids trailing a trade that hasn't moved in your favor yet.

## How it works

1. **Size** the lot (fixed or risk-based).
2. **Enter** at market with `buy_market_with_pips(symbol, lots, sl_pips, tp_pips, …)`.
3. Read **pip size** and **entry price**; print the plan (breakeven at `+be`, trailing from `+activate_at_pips`).
4. **Manage** for ~10 s via [`manage_position`](../Presets.md#manage_position) with `trail_activate_pips = activate_at_pips`:
   - breakeven fires at `be_trigger_pips` (smaller);
   - trailing only engages once profit ≥ `activate_at_pips` (larger).
5. **Close** the position.

## Signature & parameters

```rust
pub async fn run_bracket_trailing(
    sugar: &MT5Sugar,
    sp: &StrategyPreset,
    rp: &RiskPreset,
    activate_at_pips: f64,
) -> Result<u64, MT5Error>
```

| Parameter | Type | Meaning |
| --- | --- | --- |
| `sugar` | `&MT5Sugar` | High-level API. |
| `sp` | `&StrategyPreset` | What to trade. |
| `rp` | `&RiskPreset` | Risk + protection (SL/TP/trailing/breakeven). |
| `activate_at_pips` | `f64` | Profit cushion required before trailing switches on. |
| **returns** | `u64` | The opened order ticket. |

## Example scenario

`market_eurusd()` + `balanced()` (breakeven 10/+2, trail 15) with `activate_at_pips = 25`:

- BUY EURUSD at market, SL 20 / TP 40 pips.
- At **+10** pips → SL to breakeven (entry +2). Trailing still off.
- At **+25** pips → trailing engages; SL = `mid − 15 pips`, ratcheting up only.
- Reversal after the trail is set keeps most of the gain; window ends → close.

## Run it

```bash
cargo run --example strategies_demo
```

## Minimal usage

```rust
use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};
use metarpc_mt5::strategies::{run_bracket_trailing, market_eurusd, balanced};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = MT5Account::new(12_345_678, "password", None);
    let mut sugar = MT5Sugar::new(MT5Service::new(account)).with_default_symbol("EURUSD");
    sugar.connect_by_server_name("MetaQuotes-Demo", 30).await?;

    // trailing only after +25 pips
    let ticket = run_bracket_trailing(&sugar, &market_eurusd(), &balanced(), 25.0).await?;
    println!("ticket #{ticket}");

    sugar.disconnect(false).await?;
    Ok(())
}
```

## API methods used

- **MT5Sugar:** `calculate_position_size()`, `buy_market_with_pips()`, `get_symbol_info()`, `get_position_by_ticket()` / `ask()`, `bid()` + `modify_position_sl()`, `close_position()`.

## Notes

- **Difference from [Market One-Shot](MarketOneShot.md):** there, trailing starts at the breakeven trigger; here it waits for the larger `activate_at_pips` cushion.
- Breakeven and trailing both come from the `RiskPreset`; `activate_at_pips` is the extra knob this orchestrator adds.
- Manage window ~10 s for demo visibility.

## See also

- [Strategies — Master Overview](../Strategies.Master.Overview.md) · [Presets](../Presets.md)
- [Market One-Shot](MarketOneShot.md) · [Pending Bracket](PendingBracket.md)
