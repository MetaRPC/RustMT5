//! Orchestrator · Bracket + Trailing activation.

use super::{entry_price, manage_position, pip_size, size_lot, RiskPreset, StrategyPreset};
use crate::{MT5Error, MT5Sugar};

/// Enter with a bracket, then **only start trailing once the trade is `activate_at_pips` in
/// profit** — "let it prove itself before trailing". Auto-breakeven (from the preset's
/// `be_trigger_pips`) still applies earlier, at its own smaller threshold. Returns the ticket.
///
/// The difference from [`run_market_one_shot`](super::run_market_one_shot): there, trailing starts
/// as soon as breakeven does; here it waits for the larger `activate_at_pips` cushion.
pub async fn run_bracket_trailing(sugar: &MT5Sugar, sp: &StrategyPreset, rp: &RiskPreset, activate_at_pips: f64) -> Result<u64, MT5Error> {
    println!("── Bracket + Trailing · {} ──", sp.symbol);

    // 1) size & enter with the pip bracket
    let lots = size_lot(sugar, sp, rp).await?;
    let ticket = sugar
        .buy_market_with_pips(Some(&sp.symbol), lots, Some(rp.sl_pips), rp.tp_pips, &sp.comment, sp.magic)
        .await?;
    let pip = pip_size(sugar, &sp.symbol).await?;
    let entry = entry_price(sugar, ticket, &sp.symbol).await?;
    println!("entry  : BUY #{ticket} {lots:.2} lot @ {entry:.5}");

    let be = rp.be_trigger_pips.map(|t| format!("{t:.0}p")).unwrap_or_else(|| "-".into());
    println!("manage : breakeven at +{be}, trailing starts at +{activate_at_pips:.0}p");

    // 2) manage: breakeven at its trigger, trailing only after `activate_at_pips`
    manage_position(sugar, ticket, &sp.symbol, entry, pip, rp, 10, activate_at_pips).await?;

    // 3) flatten
    println!("close  : #{ticket}");
    sugar.close_position(ticket).await?;
    Ok(ticket)
}
