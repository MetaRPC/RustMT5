//! Orchestrator · Market One-Shot.

use super::{entry_price, manage_position, pip_size, size_lot, RiskPreset, StrategyPreset};
use crate::{MT5Error, MT5Sugar};

/// The simplest orchestrator: **size → enter at market with a pip bracket → manage → close**.
///
/// Management honours the `RiskPreset`: auto-breakeven once `be_trigger_pips` is reached, and a
/// trailing stop (starting as soon as the breakeven trigger — or immediately if no trigger is set).
/// Returns the ticket.
pub async fn run_market_one_shot(sugar: &MT5Sugar, sp: &StrategyPreset, rp: &RiskPreset) -> Result<u64, MT5Error> {
    println!("── Market One-Shot · {} ──", sp.symbol);

    // 1) size the lot — fixed from the strategy, or by risk% over the stop
    let lots = size_lot(sugar, sp, rp).await?;
    let tp = rp.tp_pips.map(|t| format!("{t:.0}p")).unwrap_or_else(|| "-".into());
    println!("sizing : {lots:.2} lot (risk {:.1}%, SL {:.0}p, TP {tp})", rp.risk_percent, rp.sl_pips);

    // 2) enter at market with the pip-based SL/TP bracket
    let ticket = sugar
        .buy_market_with_pips(Some(&sp.symbol), lots, Some(rp.sl_pips), rp.tp_pips, &sp.comment, sp.magic)
        .await?;
    let pip = pip_size(sugar, &sp.symbol).await?;
    let entry = entry_price(sugar, ticket, &sp.symbol).await?;
    println!("entry  : BUY #{ticket} @ {entry:.5}");

    // 3) manage for a few seconds: auto-breakeven, then trail (both from the preset)
    let trail_from = rp.be_trigger_pips.unwrap_or(0.0);
    manage_position(sugar, ticket, &sp.symbol, entry, pip, rp, 5, trail_from).await?;

    // 4) flatten
    println!("close  : #{ticket}");
    sugar.close_position(ticket).await?;
    Ok(ticket)
}
