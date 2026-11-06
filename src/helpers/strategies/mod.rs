//! Ready-made **orchestrators** and **presets** (Rust port of the JavaMT4/PyMT5 strategy set).
//!
//! Each orchestrator is its own module (one `async fn run_*(sugar, strategy, risk, …)`), driven by
//! [`MT5Sugar`]. A [`StrategyPreset`] says *what* to trade, a [`RiskPreset`] says *how much*.
//! **Demonstration templates — DEMO accounts only.**

use crate::{MT5Error, MT5Sugar};
use std::time::{Duration, Instant};

mod bracket_trailing;
mod market_one_shot;
mod pending_bracket;

pub use bracket_trailing::run_bracket_trailing;
pub use market_one_shot::run_market_one_shot;
pub use pending_bracket::run_pending_bracket;

/// WHAT to trade.
#[derive(Debug, Clone)]
pub struct StrategyPreset {
    pub symbol: String,
    pub use_market: bool,
    pub entry_price: Option<f64>,
    pub lots: Option<f64>,
    pub magic: u64,
    pub deviation_pips: f64,
    pub comment: String,
}

impl StrategyPreset {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            use_market: true,
            entry_price: None,
            lots: None,
            magic: 0,
            deviation_pips: 2.0,
            comment: String::new(),
        }
    }
    pub fn market(mut self) -> Self {
        self.use_market = true;
        self.entry_price = None;
        self
    }
    pub fn limit_at(mut self, price: f64) -> Self {
        self.use_market = false;
        self.entry_price = Some(price);
        self
    }
    pub fn lots(mut self, lots: f64) -> Self {
        self.lots = Some(lots);
        self
    }
    pub fn magic(mut self, magic: u64) -> Self {
        self.magic = magic;
        self
    }
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = comment.into();
        self
    }
}

/// HOW MUCH to risk and how to protect.
#[derive(Debug, Clone)]
pub struct RiskPreset {
    pub risk_percent: f64,
    pub sl_pips: f64,
    pub tp_pips: Option<f64>,
    pub trailing_pips: Option<f64>,
    pub be_trigger_pips: Option<f64>,
    pub be_plus_pips: f64,
}

impl RiskPreset {
    pub fn new(risk_percent: f64, sl_pips: f64) -> Self {
        Self {
            risk_percent,
            sl_pips,
            tp_pips: None,
            trailing_pips: None,
            be_trigger_pips: None,
            be_plus_pips: 0.0,
        }
    }
    pub fn tp(mut self, pips: f64) -> Self {
        self.tp_pips = Some(pips);
        self
    }
    pub fn trailing(mut self, pips: f64) -> Self {
        self.trailing_pips = Some(pips);
        self
    }
    pub fn breakeven(mut self, trigger_pips: f64, plus_pips: f64) -> Self {
        self.be_trigger_pips = Some(trigger_pips);
        self.be_plus_pips = plus_pips;
        self
    }
}

// ----- shared helpers used by the orchestrators -----

pub(crate) async fn size_lot(sugar: &MT5Sugar, sp: &StrategyPreset, rp: &RiskPreset) -> Result<f64, MT5Error> {
    match sp.lots {
        Some(l) => Ok(l),
        None => sugar.calculate_position_size(&sp.symbol, rp.risk_percent, rp.sl_pips).await,
    }
}

pub(crate) async fn pip_size(sugar: &MT5Sugar, symbol: &str) -> Result<f64, MT5Error> {
    Ok(sugar.get_symbol_info(symbol).await?.point * 10.0)
}

/// Manage an open **BUY** position for `manage_secs`, applying (from the `RiskPreset`):
///
/// 1. **Auto-breakeven** — once price has run `be_trigger_pips` in profit, move the stop to
///    `entry + be_plus_pips` (once).
/// 2. **Trailing** — once price is at least `trail_activate_pips` in profit, trail the stop
///    `trailing_pips` behind the mid, ratcheting **up only** (never loosening it).
///
/// Both are optional: a preset with neither `be_trigger_pips` nor `trailing_pips` just holds.
/// The stop is tracked locally (`cur_sl`) and only sent when it actually moves.
pub(crate) async fn manage_position(
    sugar: &MT5Sugar,
    ticket: u64,
    symbol: &str,
    entry: f64,
    pip: f64,
    rp: &RiskPreset,
    manage_secs: u64,
    trail_activate_pips: f64,
) -> Result<(), MT5Error> {
    // Where the bracket's stop started, for a BUY: sl_pips below the entry.
    let mut cur_sl = entry - rp.sl_pips * pip;
    let mut breakeven_done = false;

    let deadline = Instant::now() + Duration::from_secs(manage_secs);
    while Instant::now() < deadline {
        let mid = sugar.bid(Some(symbol)).await?;
        let pips_in_profit = (mid - entry) / pip;

        // 1) auto-breakeven
        if let Some(trigger) = rp.be_trigger_pips {
            if !breakeven_done && pips_in_profit >= trigger {
                let be = entry + rp.be_plus_pips * pip;
                if be > cur_sl {
                    cur_sl = be;
                    sugar.modify_position_sl(ticket, cur_sl).await?;
                    println!("  breakeven: +{:.0}p reached, SL -> {cur_sl:.5}", trigger);
                }
                breakeven_done = true;
            }
        }

        // 2) trailing stop (ratchet up only)
        if let Some(trail) = rp.trailing_pips {
            if pips_in_profit >= trail_activate_pips {
                let desired = mid - trail * pip;
                if desired > cur_sl {
                    cur_sl = desired;
                    sugar.modify_position_sl(ticket, cur_sl).await?;
                    println!("  trailing : SL -> {cur_sl:.5}");
                }
            }
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    Ok(())
}

/// The BUY entry price of a just-opened position (falls back to the current ask).
pub(crate) async fn entry_price(sugar: &MT5Sugar, ticket: u64, symbol: &str) -> Result<f64, MT5Error> {
    match sugar.get_position_by_ticket(ticket).await? {
        Some(p) => Ok(p.price_open),
        None => sugar.ask(Some(symbol)).await,
    }
}

// ===== Presets catalog =====

/// Market EURUSD (magic 777).
pub fn market_eurusd() -> StrategyPreset {
    StrategyPreset::new("EURUSD").magic(777).comment("market EURUSD")
}
/// BUY LIMIT EURUSD at `price` (magic 778).
pub fn limit_eurusd(price: f64) -> StrategyPreset {
    StrategyPreset::new("EURUSD").limit_at(price).magic(778).comment("limit EURUSD")
}
/// 0.5% risk, SL 25, TP 50.
pub fn conservative() -> RiskPreset {
    RiskPreset::new(0.5, 25.0).tp(50.0)
}
/// 1.0% risk, SL 20, TP 40, trail 15, breakeven 10/+2.
pub fn balanced() -> RiskPreset {
    RiskPreset::new(1.0, 20.0).tp(40.0).trailing(15.0).breakeven(10.0, 2.0)
}
/// 2.0% risk, SL 15, TP 45, trail 10, breakeven 8/+1.
pub fn aggressive() -> RiskPreset {
    RiskPreset::new(2.0, 15.0).tp(45.0).trailing(10.0).breakeven(8.0, 1.0)
}
