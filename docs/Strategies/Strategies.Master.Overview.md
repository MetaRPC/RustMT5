# Strategies — Master Overview

> One page to orient fast: which orchestrators ship with the RustMT5 SDK, how to choose one, how to run it, and links to every spec.

---

!!! warning "Educational & demonstration purpose only"
    Every orchestrator here is a **demonstration template** that showcases the RustMT5 API. They are **not** production trading systems: entries are simplified (demo BUY side), risk controls are minimal, and there is no slippage/adverse-condition handling.

    **Use demo accounts only.** Before anything live: understand every line, backtest, forward-test on demo, add real risk management and monitoring. Trading carries substantial risk of loss; the decisions are yours.

---

## The orchestrators

Three orchestrators, each its own module under [`src/helpers/strategies/`](https://github.com/MetaRPC/RustMT5/blob/main/src/helpers/strategies), each a single `async fn` driven by [`MT5Sugar`]. A [`StrategyPreset`](Presets.md) says **what** to trade; a [`RiskPreset`](Presets.md) says **how much** and how to protect.

| Strategy | What it does | Best for | Entry point |
| --- | --- | --- | --- |
| **[Market One-Shot](Orchestrators/MarketOneShot.md)** | Size → market entry with a pip bracket → manage (breakeven + trailing) → close. | A simple, self-contained managed trade | `run_market_one_shot(sugar, sp, rp)` |
| **[Bracket + Trailing](Orchestrators/BracketTrailing.md)** | Market bracket, but trailing only starts after the trade is `activate_at_pips` in profit. | "Let it prove itself before trailing" | `run_bracket_trailing(sugar, sp, rp, activate_at_pips)` |
| **[Pending Bracket](Orchestrators/PendingBracket.md)** | Place a BUY LIMIT, wait for the fill (or cancel on timeout), then manage. | Entering on a pullback | `run_pending_bracket(sugar, sp, rp, timeout_seconds)` |

## Pick one by goal

- **One managed market trade, start to finish** → [Market One-Shot](Orchestrators/MarketOneShot.md)
- **Protect only once a cushion exists** → [Bracket + Trailing](Orchestrators/BracketTrailing.md)
- **Enter on a limit pullback, manage after fill** → [Pending Bracket](Orchestrators/PendingBracket.md)

---

## How an orchestrator fits the SDK

Orchestrators sit on top of the three-layer API — plain `async fn`s that drive `MT5Sugar` (and through it `MT5Service` / `MT5Account`):

```text
ORCHESTRATORS  (src/helpers/strategies/)
  Market One-Shot · Bracket + Trailing · Pending Bracket
        │  size, enter, poll price, move the stop, close
        ▼
MT5Sugar      high-level helpers  (buy_market_with_pips, modify_position_sl, close_position, bid, …)
        ▼
MT5Service    typed facade        (place_order, modify_order, symbol_tick, …)
        ▼
MT5Account    low-level gRPC
```

All three manage a **BUY** demo position and poll the price with `sugar.bid(...)` on a short interval (the shared [`manage_position`](Presets.md#manage_position) worker), then flatten what they opened.

## Shared pieces

- **[Presets](Presets.md):** `StrategyPreset` (what), `RiskPreset` (how much), the builder methods, and the ready catalog (`market_eurusd`, `limit_eurusd`, `conservative`, `balanced`, `aggressive`).
- **`manage_position`:** the common worker that applies auto-breakeven and a ratchet-up trailing stop from the `RiskPreset`. See [Presets → manage_position](Presets.md#manage_position).
- **Pips vs points:** pip size is derived per symbol — `point × 10` (so 1 pip = 10 points on 3/5-digit symbols).

---

## Run pattern

```rust
use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};
use metarpc_mt5::strategies::{run_market_one_shot, market_eurusd, balanced};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) connect
    let account = MT5Account::new(12_345_678, "password", None);
    let mut sugar = MT5Sugar::new(MT5Service::new(account)).with_default_symbol("EURUSD");
    sugar.connect_by_server_name("MetaQuotes-Demo", 30).await?;

    // 2) choose presets and run an orchestrator
    let sp = market_eurusd();       // StrategyPreset
    let rp = balanced();            // RiskPreset (trail 15, breakeven 10/+2)
    let ticket = run_market_one_shot(&sugar, &sp, &rp).await?;
    println!("done, ticket #{ticket}");

    // 3) disconnect
    sugar.disconnect(false).await?;
    Ok(())
}
```

The runnable demo of all three is [`examples/4_strategies/strategies_demo.rs`](https://github.com/MetaRPC/RustMT5/blob/main/examples/4_strategies/strategies_demo.rs):

```bash
cargo run --example strategies_demo
```

---

## Related documentation

- [MT5Sugar — Master Overview](../MT5Sugar/MT5Sugar.Master.Overview.md) — the helpers orchestrators call.
- [MT5Service — Overview](../MT5Service/MT5Service.Overview.md) — typed facade.
- [MT5Account — Master Overview](../MT5Account/MT5Account.Master.Overview.md) — low-level gRPC API.

## Quick jump

- [Presets](Presets.md)
- [Market One-Shot](Orchestrators/MarketOneShot.md)
- [Bracket + Trailing](Orchestrators/BracketTrailing.md)
- [Pending Bracket](Orchestrators/PendingBracket.md)
