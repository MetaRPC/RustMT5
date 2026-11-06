//! `MT5Sugar` — the high-level convenience API over [`MT5Service`].
//!
//! Port of Python `pymt5.mt5_sugar.MT5Sugar`: one-liners and business logic — quick balances,
//! prices, order construction from `symbol`/`side`/`volume`, position management, risk sizing.
//! This is batch A (foundation, balances, prices, market orders, close); the remaining groups
//! (pending/SL-TP/pips entries, position queries, history, risk sizing, daily stats) follow.

use crate::error::MT5Error;
use crate::proto::mt5_term_api as pb;
use crate::service::MT5Service;

use pb::AccountInfoDoublePropertyType as AccDouble;
use pb::Tmt5EnumOrderType as OrderType;

/// MT5 `TRADE_RETCODE_DONE` — a trade request completed successfully.
pub const TRADE_RETCODE_DONE: u32 = 10009;

/// Default opened-orders sort (by open time, ascending).
const SORT_OPEN_ASC: pb::Bmt5EnumOpenedOrderSortType = pb::Bmt5EnumOpenedOrderSortType::Bmt5OpenedOrderSortByOpenTimeAsc;

/// A compact snapshot of a symbol's trading parameters (from `symbol_params_many`).
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub bid: f64,
    pub ask: f64,
    pub spread: i32,
    pub digits: i32,
    pub point: f64,
    pub volume_min: f64,
    pub volume_max: f64,
    pub volume_step: f64,
    pub contract_size: f64,
}

/// A predefined reporting period for deals/profit queries.
#[derive(Debug, Clone, Copy)]
pub enum Period {
    Today,
    Yesterday,
    ThisWeek,
    ThisMonth,
}

/// A compact account snapshot.
#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub login: i64,
    pub name: String,
    pub company: String,
    pub currency: String,
    pub leverage: i64,
    pub balance: f64,
    pub equity: f64,
}

/// Aggregated statistics for a day's closed positions.
#[derive(Debug, Clone)]
pub struct DailyStats {
    pub deals: usize,
    pub profit: f64,
    pub wins: usize,
    pub losses: usize,
}

fn to_ts(dt: chrono::DateTime<chrono::Utc>) -> prost_types::Timestamp {
    prost_types::Timestamp { seconds: dt.timestamp(), nanos: 0 }
}

/// Resolve a [`Period`] into a UTC `[from, to]` timestamp range.
fn period_range(p: Period) -> (prost_types::Timestamp, prost_types::Timestamp) {
    use chrono::{Datelike, Duration, Utc};
    let now = Utc::now();
    let today = now.date_naive();
    let (start_date, end_dt) = match p {
        Period::Today => (today, now),
        Period::Yesterday => {
            let y = today - Duration::days(1);
            (y, y.and_hms_opt(23, 59, 59).unwrap().and_utc())
        }
        Period::ThisWeek => (today - Duration::days(today.weekday().num_days_from_monday() as i64), now),
        Period::ThisMonth => (today.with_day(1).unwrap(), now),
    };
    let start_dt = start_date.and_hms_opt(0, 0, 0).unwrap().and_utc();
    (to_ts(start_dt), to_ts(end_dt))
}

pub struct MT5Sugar {
    svc: MT5Service,
    default_symbol: Option<String>,
}

impl MT5Sugar {
    /// Wrap a service.
    pub fn new(svc: MT5Service) -> Self {
        Self { svc, default_symbol: None }
    }

    /// Set a default symbol used when a method's `symbol` argument is `None`.
    pub fn with_default_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.default_symbol = Some(symbol.into());
        self
    }

    pub fn service(&self) -> &MT5Service {
        &self.svc
    }
    pub fn service_mut(&mut self) -> &mut MT5Service {
        &mut self.svc
    }

    // ===== Connection =====

    pub async fn connect_by_server_name(&mut self, server_name: &str, timeout_seconds: u32) -> Result<(), MT5Error> {
        self.svc.connect_by_server_name(server_name, timeout_seconds).await
    }
    pub async fn connect_by_host_port(&mut self, host: &str, port: u16, timeout_seconds: u32) -> Result<(), MT5Error> {
        self.svc.connect_by_host_port(host, port, timeout_seconds).await
    }
    pub async fn disconnect(&mut self, delete: bool) -> Result<(), MT5Error> {
        self.svc.disconnect(delete).await
    }
    /// Liveness check — true if an account snapshot succeeds.
    pub async fn ping(&self) -> bool {
        self.svc.account_summary().await.is_ok()
    }

    fn resolve_symbol<'a>(&'a self, symbol: Option<&'a str>) -> Result<&'a str, MT5Error> {
        symbol
            .or(self.default_symbol.as_deref())
            .ok_or_else(|| MT5Error::InvalidArgument("symbol required (no default symbol set)".into()))
    }

    // ===== Quick balances =====

    /// Balance (realized P/L only).
    pub async fn balance(&self) -> Result<f64, MT5Error> {
        self.svc.account_double(AccDouble::AccountBalance).await
    }
    /// Equity (balance + floating P/L).
    pub async fn equity(&self) -> Result<f64, MT5Error> {
        self.svc.account_double(AccDouble::AccountEquity).await
    }
    /// Margin currently used.
    pub async fn margin(&self) -> Result<f64, MT5Error> {
        self.svc.account_double(AccDouble::AccountMargin).await
    }
    /// Free margin available for new positions.
    pub async fn free_margin(&self) -> Result<f64, MT5Error> {
        self.svc.account_double(AccDouble::AccountMarginFree).await
    }
    /// Margin level %.
    pub async fn margin_level(&self) -> Result<f64, MT5Error> {
        self.svc.account_double(AccDouble::AccountMarginLevel).await
    }
    /// Current floating P/L of open positions.
    pub async fn floating_profit(&self) -> Result<f64, MT5Error> {
        self.svc.account_double(AccDouble::AccountProfit).await
    }

    // ===== Prices =====

    /// Current bid (defaults to the default symbol).
    pub async fn bid(&self, symbol: Option<&str>) -> Result<f64, MT5Error> {
        let s = self.resolve_symbol(symbol)?;
        Ok(self.svc.symbol_tick(s).await?.bid)
    }
    /// Current ask.
    pub async fn ask(&self, symbol: Option<&str>) -> Result<f64, MT5Error> {
        let s = self.resolve_symbol(symbol)?;
        Ok(self.svc.symbol_tick(s).await?.ask)
    }
    /// Current spread in price terms (`ask - bid`).
    pub async fn spread(&self, symbol: Option<&str>) -> Result<f64, MT5Error> {
        let s = self.resolve_symbol(symbol)?;
        let t = self.svc.symbol_tick(s).await?;
        Ok(t.ask - t.bid)
    }

    // ===== Order entry =====

    /// Open a BUY at market; returns the order (ticket).
    pub async fn buy_market(&self, symbol: Option<&str>, volume: f64, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        self.send_order(symbol, OrderType::Tmt5OrderTypeBuy, volume, None, None, None, comment, magic).await
    }
    /// Open a SELL at market; returns the order (ticket).
    pub async fn sell_market(&self, symbol: Option<&str>, volume: f64, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        self.send_order(symbol, OrderType::Tmt5OrderTypeSell, volume, None, None, None, comment, magic).await
    }

    /// Place a BUY LIMIT pending order at `price` (below market).
    pub async fn buy_limit(&self, symbol: Option<&str>, volume: f64, price: f64, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        self.send_order(symbol, OrderType::Tmt5OrderTypeBuyLimit, volume, Some(price), None, None, comment, magic).await
    }
    /// Place a SELL LIMIT pending order at `price` (above market).
    pub async fn sell_limit(&self, symbol: Option<&str>, volume: f64, price: f64, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        self.send_order(symbol, OrderType::Tmt5OrderTypeSellLimit, volume, Some(price), None, None, comment, magic).await
    }
    /// Place a BUY STOP pending order at `price` (above market).
    pub async fn buy_stop(&self, symbol: Option<&str>, volume: f64, price: f64, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        self.send_order(symbol, OrderType::Tmt5OrderTypeBuyStop, volume, Some(price), None, None, comment, magic).await
    }
    /// Place a SELL STOP pending order at `price` (below market).
    pub async fn sell_stop(&self, symbol: Option<&str>, volume: f64, price: f64, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        self.send_order(symbol, OrderType::Tmt5OrderTypeSellStop, volume, Some(price), None, None, comment, magic).await
    }

    /// Open a BUY at market with absolute and/or pip-based SL/TP; returns the order (ticket).
    #[allow(clippy::too_many_arguments)]
    pub async fn buy_market_with_sltp(&self, symbol: Option<&str>, volume: f64, sl: Option<f64>, tp: Option<f64>, sl_pips: Option<f64>, tp_pips: Option<f64>, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        self.market_with_sltp(symbol, OrderType::Tmt5OrderTypeBuy, volume, sl, tp, sl_pips, tp_pips, comment, magic).await
    }
    /// Open a SELL at market with absolute and/or pip-based SL/TP.
    #[allow(clippy::too_many_arguments)]
    pub async fn sell_market_with_sltp(&self, symbol: Option<&str>, volume: f64, sl: Option<f64>, tp: Option<f64>, sl_pips: Option<f64>, tp_pips: Option<f64>, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        self.market_with_sltp(symbol, OrderType::Tmt5OrderTypeSell, volume, sl, tp, sl_pips, tp_pips, comment, magic).await
    }
    /// Alias: BUY at market with SL/TP given purely in pips.
    pub async fn buy_market_with_pips(&self, symbol: Option<&str>, volume: f64, sl_pips: Option<f64>, tp_pips: Option<f64>, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        self.buy_market_with_sltp(symbol, volume, None, None, sl_pips, tp_pips, comment, magic).await
    }
    /// Alias: SELL at market with SL/TP given purely in pips.
    pub async fn sell_market_with_pips(&self, symbol: Option<&str>, volume: f64, sl_pips: Option<f64>, tp_pips: Option<f64>, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        self.sell_market_with_sltp(symbol, volume, None, None, sl_pips, tp_pips, comment, magic).await
    }

    // ----- order internals -----

    fn is_buy_side(op: OrderType) -> bool {
        matches!(
            op,
            OrderType::Tmt5OrderTypeBuy
                | OrderType::Tmt5OrderTypeBuyLimit
                | OrderType::Tmt5OrderTypeBuyStop
                | OrderType::Tmt5OrderTypeBuyStopLimit
        )
    }

    /// Point size for a symbol (`SYMBOL_POINT`).
    async fn point(&self, symbol: &str) -> Result<f64, MT5Error> {
        self.svc.symbol_double(symbol, pb::SymbolInfoDoubleProperty::SymbolPoint).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn send_order(&self, symbol: Option<&str>, op: OrderType, volume: f64, price: Option<f64>, sl: Option<f64>, tp: Option<f64>, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        let s = self.resolve_symbol(symbol)?.to_string();
        let price = match price {
            Some(p) => p,
            None => {
                let tick = self.svc.symbol_tick(&s).await?;
                if Self::is_buy_side(op) { tick.ask } else { tick.bid }
            }
        };
        let req = pb::OrderSendRequest {
            symbol: s,
            operation: op as i32,
            volume,
            price: Some(price),
            slippage: Some(10),
            stop_loss: sl,
            take_profit: tp,
            comment: if comment.is_empty() { None } else { Some(comment.to_string()) },
            expert_id: if magic == 0 { None } else { Some(magic) },
            ..Default::default()
        };
        let res = self.svc.place_order(req).await?;
        if res.returned_code != TRADE_RETCODE_DONE {
            return Err(MT5Error::Api {
                code: res.returned_string_code,
                message: res.returned_code_description,
                mql_code: res.returned_code as i32,
            });
        }
        Ok(res.order)
    }

    #[allow(clippy::too_many_arguments)]
    async fn market_with_sltp(&self, symbol: Option<&str>, op: OrderType, volume: f64, sl: Option<f64>, tp: Option<f64>, sl_pips: Option<f64>, tp_pips: Option<f64>, comment: &str, magic: u64) -> Result<u64, MT5Error> {
        let s = self.resolve_symbol(symbol)?.to_string();
        let tick = self.svc.symbol_tick(&s).await?;
        let buy = Self::is_buy_side(op);
        let entry = if buy { tick.ask } else { tick.bid };
        let (mut sl_price, mut tp_price) = (sl, tp);
        if sl_pips.is_some() || tp_pips.is_some() {
            let point = self.point(&s).await?;
            if let Some(p) = sl_pips {
                sl_price = Some(if buy { entry - p * point * 10.0 } else { entry + p * point * 10.0 });
            }
            if let Some(p) = tp_pips {
                tp_price = Some(if buy { entry + p * point * 10.0 } else { entry - p * point * 10.0 });
            }
        }
        self.send_order(Some(s.as_str()), op, volume, Some(entry), sl_price, tp_price, comment, magic).await
    }

    // ===== Close =====

    /// Close a position completely by ticket; `true` if the server reported `TRADE_RETCODE_DONE`.
    pub async fn close_position(&self, ticket: u64) -> Result<bool, MT5Error> {
        let res = self.svc.close_order(pb::OrderCloseRequest { ticket, volume: 0.0, slippage: 0 }).await?;
        Ok(res.returned_code == TRADE_RETCODE_DONE)
    }

    /// Partially close a position by volume.
    pub async fn close_position_partial(&self, ticket: u64, volume: f64) -> Result<bool, MT5Error> {
        let res = self.svc.close_order(pb::OrderCloseRequest { ticket, volume, slippage: 10 }).await?;
        Ok(res.returned_code == TRADE_RETCODE_DONE)
    }

    /// Close every open position (optionally only those on `symbol`); returns how many closed.
    pub async fn close_all_positions(&self, symbol: Option<&str>) -> Result<usize, MT5Error> {
        let mut closed = 0usize;
        for p in self.get_open_positions().await? {
            if let Some(s) = symbol {
                if p.symbol != s {
                    continue;
                }
            }
            if self.close_position(p.ticket).await.unwrap_or(false) {
                closed += 1;
            }
        }
        Ok(closed)
    }

    // ===== Position management / queries =====

    /// All open positions.
    pub async fn get_open_positions(&self) -> Result<Vec<pb::PositionInfo>, MT5Error> {
        Ok(self.svc.opened_orders(SORT_OPEN_ASC).await?.position_infos)
    }
    /// One position by ticket (if open).
    pub async fn get_position_by_ticket(&self, ticket: u64) -> Result<Option<pb::PositionInfo>, MT5Error> {
        Ok(self.get_open_positions().await?.into_iter().find(|p| p.ticket == ticket))
    }
    /// All open positions on a symbol.
    pub async fn get_positions_by_symbol(&self, symbol: &str) -> Result<Vec<pb::PositionInfo>, MT5Error> {
        Ok(self.get_open_positions().await?.into_iter().filter(|p| p.symbol == symbol).collect())
    }
    /// Whether any position is open (optionally on a specific symbol).
    pub async fn has_open_position(&self, symbol: Option<&str>) -> Result<bool, MT5Error> {
        let ps = self.get_open_positions().await?;
        Ok(match symbol {
            Some(s) => ps.iter().any(|p| p.symbol == s),
            None => !ps.is_empty(),
        })
    }
    /// Count of open positions (optionally on a specific symbol).
    pub async fn count_open_positions(&self, symbol: Option<&str>) -> Result<usize, MT5Error> {
        let ps = self.get_open_positions().await?;
        Ok(match symbol {
            Some(s) => ps.iter().filter(|p| p.symbol == s).count(),
            None => ps.len(),
        })
    }
    /// Total floating profit across all open positions.
    pub async fn get_total_profit(&self) -> Result<f64, MT5Error> {
        Ok(self.get_open_positions().await?.iter().map(|p| p.profit).sum())
    }
    /// Floating profit summed over positions on a symbol.
    pub async fn get_profit_by_symbol(&self, symbol: &str) -> Result<f64, MT5Error> {
        Ok(self.get_open_positions().await?.iter().filter(|p| p.symbol == symbol).map(|p| p.profit).sum())
    }

    /// Modify a position's SL/TP (unspecified side is preserved); `true` on success.
    pub async fn modify_position_sltp(&self, ticket: u64, sl: Option<f64>, tp: Option<f64>) -> Result<bool, MT5Error> {
        let pos = self
            .get_position_by_ticket(ticket)
            .await?
            .ok_or_else(|| MT5Error::InvalidArgument(format!("position {ticket} not found")))?;
        let req = pb::OrderModifyRequest {
            ticket,
            stop_loss: Some(sl.unwrap_or(pos.stop_loss)),
            take_profit: Some(tp.unwrap_or(pos.take_profit)),
            ..Default::default()
        };
        let res = self.svc.modify_order(req).await?;
        Ok(res.returned_code == TRADE_RETCODE_DONE)
    }
    /// Move only the stop-loss of a position.
    pub async fn modify_position_sl(&self, ticket: u64, sl: f64) -> Result<bool, MT5Error> {
        self.modify_position_sltp(ticket, Some(sl), None).await
    }
    /// Move only the take-profit of a position.
    pub async fn modify_position_tp(&self, ticket: u64, tp: f64) -> Result<bool, MT5Error> {
        self.modify_position_sltp(ticket, None, Some(tp)).await
    }

    // ===== Symbol info =====

    /// A compact snapshot of a symbol's trading parameters.
    pub async fn get_symbol_info(&self, symbol: &str) -> Result<SymbolInfo, MT5Error> {
        let data = self.svc.symbol_params_many(Some(symbol.to_string()), None, None).await?;
        let p = data
            .symbol_infos
            .into_iter()
            .next()
            .ok_or_else(|| MT5Error::InvalidArgument(format!("symbol {symbol} not found")))?;
        Ok(SymbolInfo {
            name: p.name,
            bid: p.bid,
            ask: p.ask,
            spread: p.spread,
            digits: p.digits,
            point: p.point,
            volume_min: p.volume_min,
            volume_max: p.volume_max,
            volume_step: p.volume_step,
            contract_size: p.trade_contract_size,
        })
    }

    /// Every available symbol name (paged through `symbol_params_many`).
    pub async fn get_all_symbols(&self) -> Result<Vec<String>, MT5Error> {
        let mut symbols = Vec::new();
        let mut page = 1i32;
        let page_size = 100i32;
        loop {
            let data = self.svc.symbol_params_many(None, Some(page), Some(page_size)).await?;
            let total = data.symbols_total;
            let n = data.symbol_infos.len() as i32;
            for s in data.symbol_infos {
                symbols.push(s.name);
            }
            if symbols.len() as i32 >= total || n < page_size {
                break;
            }
            page += 1;
        }
        Ok(symbols)
    }

    /// Whether a symbol exists on the account.
    pub async fn is_symbol_available(&self, symbol: &str) -> Result<bool, MT5Error> {
        Ok(self.svc.symbol_exist(symbol).await?.exists)
    }

    /// Price decimal places for a symbol (`SYMBOL_DIGITS`).
    pub async fn get_symbol_digits(&self, symbol: &str) -> Result<i64, MT5Error> {
        self.svc.symbol_integer(symbol, pb::SymbolInfoIntegerProperty::SymbolDigits).await
    }

    /// Minimum SL/TP distance in points (`SYMBOL_TRADE_STOPS_LEVEL`).
    pub async fn get_min_stop_level(&self, symbol: &str) -> Result<i64, MT5Error> {
        self.svc.symbol_integer(symbol, pb::SymbolInfoIntegerProperty::SymbolTradeStopsLevel).await
    }

    // ===== Risk & sizing =====

    /// Margin required to open `volume` lots of `symbol` (default order type: BUY).
    pub async fn calculate_required_margin(&self, symbol: &str, volume: f64, order_type: Option<pb::EnumOrderTypeTf>) -> Result<f64, MT5Error> {
        let ot = order_type.unwrap_or(pb::EnumOrderTypeTf::OrderTypeTfBuy);
        let tick = self.svc.symbol_tick(symbol).await?;
        let price = if ot == pb::EnumOrderTypeTf::OrderTypeTfBuy { tick.ask } else { tick.bid };
        Ok(self.svc.calculate_margin(symbol, ot, volume, price).await?.margin)
    }

    /// Maximum allowed lot size for a symbol.
    pub async fn get_max_lot_size(&self, symbol: &str) -> Result<f64, MT5Error> {
        Ok(self.get_symbol_info(symbol).await?.volume_max)
    }

    /// Lot size that risks `risk_percent`% of balance over an `sl_pips` stop, rounded to the
    /// volume step and clamped to the symbol's min/max.
    pub async fn calculate_position_size(&self, symbol: &str, risk_percent: f64, sl_pips: f64) -> Result<f64, MT5Error> {
        let balance = self.balance().await?;
        let si = self.get_symbol_info(symbol).await?;
        let risk_amount = balance * (risk_percent / 100.0);
        let pip_value = si.point * 10.0 * si.contract_size;
        if sl_pips <= 0.0 || pip_value <= 0.0 {
            return Ok(si.volume_min);
        }
        let step = if si.volume_step > 0.0 { si.volume_step } else { 0.01 };
        let mut volume = (risk_amount / (sl_pips * pip_value) / step).round() * step;
        if volume < si.volume_min {
            volume = si.volume_min;
        } else if volume > si.volume_max {
            volume = si.volume_max;
        }
        Ok(volume)
    }

    /// Convert pip SL/TP distances into absolute prices (`None` for an unspecified side).
    pub async fn calculate_sltp(&self, symbol: &str, is_buy: bool, sl_pips: Option<f64>, tp_pips: Option<f64>) -> Result<(Option<f64>, Option<f64>), MT5Error> {
        let tick = self.svc.symbol_tick(symbol).await?;
        let point = self.point(symbol).await?;
        let entry = if is_buy { tick.ask } else { tick.bid };
        let d = |pips: f64| pips * point * 10.0;
        let (sl, tp) = if is_buy {
            (sl_pips.map(|p| entry - d(p)), tp_pips.map(|p| entry + d(p)))
        } else {
            (sl_pips.map(|p| entry + d(p)), tp_pips.map(|p| entry - d(p)))
        };
        Ok((sl, tp))
    }

    /// Validate whether a BUY of `volume` lots could be opened; returns `(can_open, reason)`.
    pub async fn can_open_position(&self, symbol: &str, volume: f64) -> Result<(bool, String), MT5Error> {
        let tick = self.svc.symbol_tick(symbol).await?;
        let mql = pb::MrpcMqlTradeRequest {
            action: pb::MrpcEnumTradeRequestActions::TradeActionDeal as i32,
            symbol: symbol.to_string(),
            volume,
            price: tick.ask,
            order_type: pb::EnumOrderTypeTf::OrderTypeTfBuy as i32,
            type_filling: pb::MrpcEnumOrderTypeFilling::OrderFillingIoc as i32,
            type_time: pb::MrpcEnumOrderTypeTime::OrderTimeGtc as i32,
            ..Default::default()
        };
        let res = self.svc.check_order(pb::OrderCheckRequest { mql_trade_request: Some(mql) }).await?;
        match res.mql_trade_check_result {
            Some(r) if r.returned_code == 0 => Ok((true, "OK".to_string())),
            Some(r) => Ok((false, format!("check failed: code={} {}", r.returned_code, r.comment))),
            None => Ok((false, "no check result".to_string())),
        }
    }

    // ===== History / deals =====

    async fn deals_between(&self, from: prost_types::Timestamp, to: prost_types::Timestamp) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error> {
        Ok(self
            .svc
            .positions_history(
                pb::AhEnumPositionsHistorySortType::AhPositionOpenTimeAsc,
                Some(from),
                Some(to),
                Some(0),
                Some(10_000),
            )
            .await?
            .history_positions)
    }

    /// Closed positions (deals) for a predefined period.
    pub async fn get_deals(&self, period: Period) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error> {
        let (from, to) = period_range(period);
        self.deals_between(from, to).await
    }
    /// Closed positions (deals) over an explicit UTC timestamp range.
    pub async fn get_deals_date_range(&self, from: prost_types::Timestamp, to: prost_types::Timestamp) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error> {
        self.deals_between(from, to).await
    }
    /// Total realized profit for a period (sum of closed-position profit).
    pub async fn get_profit(&self, period: Period) -> Result<f64, MT5Error> {
        Ok(self.get_deals(period).await?.iter().map(|d| d.profit).sum())
    }

    pub async fn get_deals_today(&self) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error> {
        self.get_deals(Period::Today).await
    }
    pub async fn get_deals_yesterday(&self) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error> {
        self.get_deals(Period::Yesterday).await
    }
    pub async fn get_deals_this_week(&self) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error> {
        self.get_deals(Period::ThisWeek).await
    }
    pub async fn get_deals_this_month(&self) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error> {
        self.get_deals(Period::ThisMonth).await
    }
    pub async fn get_profit_today(&self) -> Result<f64, MT5Error> {
        self.get_profit(Period::Today).await
    }
    pub async fn get_profit_this_week(&self) -> Result<f64, MT5Error> {
        self.get_profit(Period::ThisWeek).await
    }
    pub async fn get_profit_this_month(&self) -> Result<f64, MT5Error> {
        self.get_profit(Period::ThisMonth).await
    }

    // ===== Account info / daily stats =====

    /// A compact account snapshot.
    pub async fn get_account_info(&self) -> Result<AccountInfo, MT5Error> {
        let s = self.svc.account_summary().await?;
        Ok(AccountInfo {
            login: s.account_login,
            name: s.account_user_name,
            company: s.account_company_name,
            currency: s.account_currency,
            leverage: s.account_leverage,
            balance: s.account_balance,
            equity: s.account_equity,
        })
    }

    /// Aggregated stats (count, profit, wins, losses) for today's closed positions.
    pub async fn get_daily_stats(&self) -> Result<DailyStats, MT5Error> {
        let deals = self.get_deals(Period::Today).await?;
        let profit: f64 = deals.iter().map(|d| d.profit).sum();
        let wins = deals.iter().filter(|d| d.profit > 0.0).count();
        let losses = deals.iter().filter(|d| d.profit < 0.0).count();
        Ok(DailyStats { deals: deals.len(), profit, wins, losses })
    }
}
