use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub login: u64,
    pub currency: String,
    pub balance: f64,
    pub equity: f64,
    pub margin: f64,
    pub free_margin: f64,
    pub margin_level: f64,
    pub leverage: u32,
    pub name: String,
    pub server: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub time: i64,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResult {
    pub ticket: u64,
    pub retcode: u32,
    pub deal: u64,
    pub order: u64,
    pub volume: f64,
    pub price: f64,
    pub comment: String,
}
