# Pending Bracket Orchestrator

> Place a **BUY LIMIT** with a bracket, **wait for the fill**, then manage (auto-breakeven + trailing); if it isn't filled within `timeout_seconds`, cancel the pending order.

!!! warning "Demonstration only — not production ready"
    Shows the strategy idea and the API calls behind it. **Demo accounts only.**

→ Source: [`src/helpers/strategies/pending_bracket.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/helpers/strategies/pending_bracket.rs)

---

## What it does

Enters on a **pullback** rather than at market: it places a BUY LIMIT below the current price and waits. If price trades down to the limit and fills, it manages the resulting position like the other orchestrators; if the fill never comes within the timeout, it cancels the resting order and exits cleanly.

## How it works

1. Compute **pip size**; resolve the **entry price** — `sp.entry_price` if set, else 5 pips below the current bid (demo default).
2. **Size** the lot and place the **BUY LIMIT** with `buy_limit(symbol, lots, entry, comment, magic)`.
3. **Wait for the fill:** poll `has_open_position(symbol)` every 500 ms until filled or until `timeout_seconds` elapses.
   - **Timeout:** cancel the pending via `close_position(ticket)` and return.
4. **On fill:** read the actual fill price, then **manage** via [`manage_position`](../Presets.md#manage_position) (`trail_activate_pips = be_trigger_pips`, ~5 s window).
5. **Close** everything on the symbol (`close_all_positions(symbol)`).

## Signature & parameters

```rust
pub async fn run_pending_bracket(
    sugar: &MT5Sugar,
    sp: &StrategyPreset,
    rp: &RiskPreset,
    timeout_seconds: u64,
) -> Result<u64, MT5Error>
```

| Parameter | Type | Meaning |
| --- | --- | --- |
| `sugar` | `&MT5Sugar` | High-level API. |
| `sp` | `&StrategyPreset` | What to trade; `entry_price` sets the limit (else 5 pips below bid). |
| `rp` | `&RiskPreset` | Risk + protection. |
| `timeout_seconds` | `u64` | How long to wait for the fill before cancelling. |
| **returns** | `u64` | The order ticket (filled or cancelled). |

## Example scenario

`limit_eurusd(price)` (or `market_eurusd()` for the auto 5-pip offset) + `balanced()`, `timeout_seconds = 30`:

- Place BUY LIMIT ~5 pips below market (or at `sp.entry_price`).
- **Filled within 30 s:** manage with breakeven 10/+2 and trailing 15, then flatten.
- **Not filled:** after 30 s the pending order is cancelled; nothing is left resting.

## Run it

```bash
cargo run --example strategies_demo
```

## Minimal usage

```rust
use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};
use metarpc_mt5::strategies::{run_pending_bracket, market_eurusd, balanced};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = MT5Account::new(12_345_678, "password", None);
    let mut sugar = MT5Sugar::new(MT5Service::new(account)).with_default_symbol("EURUSD");
    sugar.connect_by_server_name("MetaQuotes-Demo", 30).await?;

    // auto 5-pip-below-market limit; cancel if unfilled in 30 s
    let ticket = run_pending_bracket(&sugar, &market_eurusd(), &balanced(), 30).await?;
    println!("ticket #{ticket}");

    sugar.disconnect(false).await?;
    Ok(())
}
```

## API methods used

- **MT5Sugar:** `get_symbol_info()` (pip), `bid()` (limit offset), `calculate_position_size()`, `buy_limit()` (placement), `has_open_position()` (fill poll), `get_position_by_ticket()` / `ask()` (fill price), `bid()` + `modify_position_sl()` (management), `close_position()` / `close_all_positions()` (cancel/flatten).

## Notes

- **Difference from the market orchestrators:** there's a **waiting phase** first — nothing is managed until the limit actually fills.
- **Cancel path matters:** on timeout the resting order is deleted so nothing lingers on the account.
- Default limit offset is 5 pips below bid when `sp.entry_price` is `None` — set `entry_price` for a specific level via `StrategyPreset::limit_at(price)`.

## See also

- [Strategies — Master Overview](../Strategies.Master.Overview.md) · [Presets](../Presets.md)
- [Market One-Shot](MarketOneShot.md) · [Bracket + Trailing](BracketTrailing.md)
