# Market One-Shot Orchestrator

> The simplest end-to-end managed trade: **size → market entry with a pip bracket → manage → close**.

!!! warning "Demonstration only — not production ready"
    Shows the strategy idea and the API calls behind it. A real deployment needs proper sizing, slippage control, logging, monitoring and testing. **Demo accounts only.**

→ Source: [`src/helpers/strategies/market_one_shot.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/helpers/strategies/market_one_shot.rs)

---

## What it does

Opens a single BUY at market with a pip-based SL/TP bracket, manages it for a few seconds (auto-breakeven + trailing from the `RiskPreset`), then flattens. It's the reference template — the other two orchestrators are variations on the "enter → manage → close" shape.

## How it works

1. **Size** the lot — `sp.lots` if fixed, else risk-based (`risk_percent` over `sl_pips`).
2. **Enter** at market with `buy_market_with_pips(symbol, lots, sl_pips, tp_pips, comment, magic)`.
3. Read the **entry price** (the position's `price_open`) and the **pip size** (`point × 10`).
4. **Manage** for ~5 s via the shared [`manage_position`](../Presets.md#manage_position): breakeven at `be_trigger_pips`, trailing starting at `trail_from = be_trigger_pips` (or immediately — `0.0` — if no breakeven trigger is set).
5. **Close** the position.

## Signature & parameters

```rust
pub async fn run_market_one_shot(
    sugar: &MT5Sugar,
    sp: &StrategyPreset,
    rp: &RiskPreset,
) -> Result<u64, MT5Error>
```

| Parameter | Type | Meaning |
| --- | --- | --- |
| `sugar` | `&MT5Sugar` | High-level API the orchestrator drives. |
| `sp` | `&StrategyPreset` | What to trade — symbol, lots, magic, comment. |
| `rp` | `&RiskPreset` | How much / how to protect — risk%, SL, TP, trailing, breakeven. |
| **returns** | `u64` | The opened order ticket. |

## Example scenario

`market_eurusd()` + `balanced()` (1% risk, SL 20, TP 40, trail 15, breakeven 10/+2):

- Size ≈ 1% of balance over a 20-pip stop.
- BUY EURUSD at market; SL 20 pips below, TP 40 pips above.
- At +10 pips, SL jumps to entry +2 (breakeven); trailing also eligible from +10.
- As price rises, SL ratchets to `mid − 15 pips`, never loosening.
- After the manage window, the position is closed and the ticket returned.

## Run it

```bash
cargo run --example strategies_demo
```

## Minimal usage

```rust
use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};
use metarpc_mt5::strategies::{run_market_one_shot, market_eurusd, balanced};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = MT5Account::new(12_345_678, "password", None);
    let mut sugar = MT5Sugar::new(MT5Service::new(account)).with_default_symbol("EURUSD");
    sugar.connect_by_server_name("MetaQuotes-Demo", 30).await?;

    let ticket = run_market_one_shot(&sugar, &market_eurusd(), &balanced()).await?;
    println!("ticket #{ticket}");

    sugar.disconnect(false).await?;
    Ok(())
}
```

## API methods used

- **MT5Sugar:** `calculate_position_size()` (sizing), `buy_market_with_pips()` (entry), `get_symbol_info()` (pip size), `get_position_by_ticket()` / `ask()` (entry price), `bid()` + `modify_position_sl()` (management), `close_position()` (exit).

## Notes

- **BUY side**, demo-oriented.
- Trailing here starts at the **breakeven trigger** (or immediately if none) — contrast with [Bracket + Trailing](BracketTrailing.md), which waits for a larger cushion.
- The manage window is short (~5 s) for demo visibility; lengthen it for real monitoring.

## See also

- [Strategies — Master Overview](../Strategies.Master.Overview.md) · [Presets](../Presets.md)
- [Bracket + Trailing](BracketTrailing.md) · [Pending Bracket](PendingBracket.md)
