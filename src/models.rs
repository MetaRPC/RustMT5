use serde::{Deserialize, Serialize};

use crate::pb::mt5_term_api as pb;

/// `AccountSummaryData.account_trade_mode` decoded. Fleet runners refuse to trade unless `Demo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeMode {
    Demo,
    Contest,
    Real,
    Unknown(i32),
}

impl TradeMode {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => TradeMode::Demo,
            1 => TradeMode::Contest,
            2 => TradeMode::Real,
            other => TradeMode::Unknown(other),
        }
    }
    pub fn is_demo(&self) -> bool {
        matches!(self, TradeMode::Demo)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub login: u64,
    pub currency: String,
    pub balance: f64,
    pub equity: f64,
    pub credit: f64,
    pub leverage: u32,
    pub name: String,
    pub company: String,
    pub server: String,
    pub trade_mode: TradeMode,
    /// Broker server time (unix seconds) at the moment of the reply. NOTE: observed to be
    /// unreliable over gRPC on prod (2022-dated value for a 2026 reply); prefer bar/tick times.
    pub server_time: i64,
    /// Broker server time − UTC in minutes (e.g. 180 for FBS-Demo). History windows and all
    /// history timestamps are in server time.
    pub utc_shift_minutes: i64,
}

impl AccountInfo {
    pub fn from_summary(d: &pb::AccountSummaryData, server: &str) -> Self {
        AccountInfo {
            login: d.account_login as u64,
            currency: d.account_currency.clone(),
            balance: d.account_balance,
            equity: d.account_equity,
            credit: d.account_credit,
            leverage: d.account_leverage as u32,
            name: d.account_user_name.clone(),
            company: d.account_company_name.clone(),
            server: server.to_string(),
            trade_mode: TradeMode::from_i32(d.account_trade_mode),
            server_time: d.server_time.as_ref().map(|t| t.seconds).unwrap_or(0),
            utc_shift_minutes: d.utc_timezone_server_time_shift_minutes,
        }
    }
}

/// One tick from `SymbolInfoTick` or the `OnSymbolTick` stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume: f64,
    /// Unix seconds.
    pub time: i64,
    /// Unix milliseconds.
    pub time_msc: i64,
    pub flags: u32,
}

impl Quote {
    pub fn from_tick(symbol: &str, t: &pb::MrpcMqlTick) -> Self {
        Quote {
            symbol: symbol.to_string(),
            bid: t.bid,
            ask: t.ask,
            last: t.last,
            volume: t.volume_real,
            time: t.time,
            time_msc: t.time_msc,
            flags: t.flags,
        }
    }
    pub fn from_subscription_tick(t: &pb::MrpcSubscriptionMqlTick) -> Self {
        Quote {
            symbol: t.symbol.clone(),
            bid: t.bid,
            ask: t.ask,
            last: t.last,
            volume: t.volume_real,
            time: t.time.as_ref().map(|x| x.seconds).unwrap_or(t.time_msc / 1000),
            time_msc: t.time_msc,
            flags: t.flags,
        }
    }
    /// Demo feeds intermittently publish `bid == ask` or zero; such quotes are unusable.
    pub fn is_usable(&self) -> bool {
        self.bid > 0.0 && self.ask > 0.0 && self.ask > self.bid
    }
    pub fn spread(&self) -> f64 {
        self.ask - self.bid
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeAction {
    Buy,
    Sell,
    BuyLimit,
    SellLimit,
    BuyStop,
    SellStop,
    BuyStopLimit,
    SellStopLimit,
}

impl TradeAction {
    /// Maps to `TMT5_ENUM_ORDER_TYPE` (`pb::Tmt5EnumOrderType`).
    pub fn to_order_type(self) -> pb::Tmt5EnumOrderType {
        use pb::Tmt5EnumOrderType as T;
        match self {
            TradeAction::Buy => T::Tmt5OrderTypeBuy,
            TradeAction::Sell => T::Tmt5OrderTypeSell,
            TradeAction::BuyLimit => T::Tmt5OrderTypeBuyLimit,
            TradeAction::SellLimit => T::Tmt5OrderTypeSellLimit,
            TradeAction::BuyStop => T::Tmt5OrderTypeBuyStop,
            TradeAction::SellStop => T::Tmt5OrderTypeSellStop,
            TradeAction::BuyStopLimit => T::Tmt5OrderTypeBuyStopLimit,
            TradeAction::SellStopLimit => T::Tmt5OrderTypeSellStopLimit,
        }
    }
    pub fn is_market(self) -> bool {
        matches!(self, TradeAction::Buy | TradeAction::Sell)
    }
}

/// High-level order request. For market orders leave `price` as `None` — the gateway fills at
/// the current quote. `magic` becomes `expert_id` (MQL `magic`), which is how positions are
/// filtered per strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub symbol: String,
    pub action: TradeAction,
    pub volume: f64,
    pub price: Option<f64>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub slippage: Option<u32>,
    pub comment: Option<String>,
    #[serde(default)]
    pub magic: Option<u64>,
    /// Only for pending stop-limit orders.
    #[serde(default)]
    pub stop_limit_price: Option<f64>,
}

impl OrderRequest {
    pub fn market(symbol: &str, action: TradeAction, volume: f64) -> Self {
        OrderRequest {
            symbol: symbol.to_string(),
            action,
            volume,
            price: None,
            stop_loss: None,
            take_profit: None,
            slippage: Some(20),
            comment: None,
            magic: None,
            stop_limit_price: None,
        }
    }
    pub fn with_magic(mut self, magic: u64) -> Self {
        self.magic = Some(magic);
        self
    }
    pub fn with_comment(mut self, c: &str) -> Self {
        self.comment = Some(c.to_string());
        self
    }
    pub fn with_sl_tp(mut self, sl: Option<f64>, tp: Option<f64>) -> Self {
        self.stop_loss = sl;
        self.take_profit = tp;
        self
    }

    pub fn to_proto(&self) -> pb::OrderSendRequest {
        pb::OrderSendRequest {
            symbol: self.symbol.clone(),
            operation: self.action.to_order_type() as i32,
            volume: self.volume,
            price: self.price,
            slippage: self.slippage.map(|s| s as u64),
            stop_loss: self.stop_loss,
            take_profit: self.take_profit,
            comment: self.comment.clone(),
            expert_id: self.magic,
            stop_limit_price: self.stop_limit_price,
            expiration_time_type: None,
            expiration_time: None,
            expiration_time_utc: None,
            quotes_wait_ms: None,
        }
    }
}

/// `TRADE_RETCODE_PLACED`
pub const TRADE_RETCODE_PLACED: u32 = 10008;
/// `TRADE_RETCODE_DONE`
pub const TRADE_RETCODE_DONE: u32 = 10009;
/// `TRADE_RETCODE_DONE_PARTIAL`
pub const TRADE_RETCODE_DONE_PARTIAL: u32 = 10010;
/// `TRADE_RETCODE_MARKET_CLOSED`
pub const TRADE_RETCODE_MARKET_CLOSED: u32 = 10018;

pub fn retcode_ok(rc: u32) -> bool {
    matches!(rc, TRADE_RETCODE_PLACED | TRADE_RETCODE_DONE | TRADE_RETCODE_DONE_PARTIAL)
}

/// Result of `OrderSend`. For a market order `order` is the order ticket and `deal` the deal
/// ticket; on hedging accounts the resulting **position ticket equals `order`**. Confirm with
/// `positions_by_magic()` if you need to be sure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResult {
    /// Alias of `order` kept for backwards compatibility.
    pub ticket: u64,
    pub retcode: u32,
    pub retcode_description: String,
    pub deal: u64,
    pub order: u64,
    pub volume: f64,
    pub price: f64,
    pub bid: f64,
    pub ask: f64,
    pub comment: String,
}

impl OrderResult {
    pub fn from_proto(d: pb::OrderSendData) -> Self {
        OrderResult {
            ticket: d.order,
            retcode: d.returned_code,
            retcode_description: if d.returned_code_description.is_empty() {
                d.returned_string_code.clone()
            } else {
                d.returned_code_description.clone()
            },
            deal: d.deal,
            order: d.order,
            volume: d.volume,
            price: d.price,
            bid: d.bid,
            ask: d.ask,
            comment: d.comment,
        }
    }
    pub fn is_ok(&self) -> bool {
        retcode_ok(self.retcode)
    }
}

/// Minimal bar type (mirrors `pb::PriceBar`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    /// Unix seconds (bar open time, broker server time zone as the terminal reports it).
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub tick_volume: i64,
    pub spread: i32,
    pub real_volume: i64,
}

impl From<&pb::PriceBar> for Bar {
    fn from(b: &pb::PriceBar) -> Self {
        Bar {
            time: b.time,
            open: b.open,
            high: b.high,
            low: b.low,
            close: b.close,
            tick_volume: b.tick_volume,
            spread: b.spread,
            real_volume: b.real_volume,
        }
    }
}
