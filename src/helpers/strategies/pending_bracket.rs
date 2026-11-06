//! Orchestrator · Pending Bracket.

use super::{entry_price, manage_position, pip_size, size_lot, RiskPreset, StrategyPreset};
use crate::{MT5Error, MT5Sugar};
use std::time::{Duration, Instant};

/// Place a **BUY LIMIT** with a bracket, **wait for the fill**, then manage (auto-breakeven +
/// trailing from the preset); if it isn't filled within `timeout_seconds` the pending order is
/// cancelled. Returns the ticket.
///
/// The difference from the market orchestrators: there's a waiting phase first — nothing is managed
/// until the limit actually fills.
pub async fn run_pending_bracket(sugar: &MT5Sugar, sp: &StrategyPreset, rp: &RiskPreset, timeout_seconds: u64) -> Result<u64, MT5Error> {
    println!("── Pending Bracket · {} ──", sp.symbol);
    let pip = pip_size(sugar, &sp.symbol).await?;

    // 1) resolve the entry price (explicit, or 5 pips below market for the demo) and place the limit
    let entry = match sp.entry_price {
        Some(p) => p,
        None => sugar.bid(Some(&sp.symbol)).await? - 5.0 * pip,
    };
    let lots = size_lot(sugar, sp, rp).await?;
    let ticket = sugar.buy_limit(Some(&sp.symbol), lots, entry, &sp.comment, sp.magic).await?;
    println!("placed : BUY LIMIT #{ticket} {lots:.2} lot @ {entry:.5}");

    // 2) wait for the fill (poll), or cancel on timeout
    let deadline = Instant::now() + Duration::from_secs(timeout_seconds);
    let mut filled = false;
    while Instant::now() < deadline {
        if sugar.has_open_position(Some(&sp.symbol)).await? {
            filled = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    if !filled {
        println!("timeout: not filled in {timeout_seconds}s — cancelling");
        let _ = sugar.close_position(ticket).await; // delete the resting pending order
        return Ok(ticket);
    }

    // 3) on fill: manage then flatten
    let fill_price = entry_price(sugar, ticket, &sp.symbol).await?;
    println!("filled : #{ticket} @ {fill_price:.5}");
    let trail_from = rp.be_trigger_pips.unwrap_or(0.0);
    manage_position(sugar, ticket, &sp.symbol, fill_price, pip, rp, 5, trail_from).await?;

    sugar.close_all_positions(Some(&sp.symbol)).await?;
    println!("close  : flat");
    Ok(ticket)
}
