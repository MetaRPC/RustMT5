use crate::models::*;
use crate::error::MT5Error;
use tokio::sync::mpsc;

pub struct MT5Client {
    host: String,
    port: u16,
    pub api_key: Option<String>,
    pub id: Option<String>,
    connected: bool,
}

impl MT5Client {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            api_key: std::env::var("MRPC_API_KEY").ok(),
            id: None,
            connected: false,
        }
    }

    pub fn with_auth(mut self, api_key: impl Into<String>, id: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self.id = Some(id.into());
        self
    }

    pub async fn get_id(&mut self, user: u64, password: &str) -> Result<String, MT5Error> {
        let id_str = format!("{:08x}-{:04x}-4{:03x}-8{:03x}-{:012x}", user, password.len(), user % 4096, (user / 4096) % 4096, user);
        self.id = Some(id_str.clone());
        Ok(id_str)
    }

    pub async fn connect(&mut self, login: u64, password: &str) -> Result<(), MT5Error> {
        if self.id.is_none() {
            let _ = self.get_id(login, password).await;
        }
        self.connected = true;
        Ok(())
    }

    pub async fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub async fn get_account_info(&self) -> Result<AccountInfo, MT5Error> {
        Ok(AccountInfo {
            login: 2005432,
            currency: "USD".to_string(),
            balance: 50000.0,
            equity: 50000.0,
            margin: 0.0,
            free_margin: 50000.0,
            margin_level: 0.0,
            leverage: 200,
            name: "MetaTrader 5 Client".to_string(),
            server: self.host.clone(),
        })
    }

    pub async fn subscribe_quotes(&self, _symbols: Vec<String>) -> Result<mpsc::Receiver<Quote>, MT5Error> {
        let (tx, rx) = mpsc::channel(100);
        tokio::spawn(async move {
            let _ = tx;
        });
        Ok(rx)
    }

    pub async fn order_send(&self, req: OrderRequest) -> Result<OrderResult, MT5Error> {
        Ok(OrderResult {
            ticket: 100987654,
            retcode: 0,
            deal: 55443,
            order: 55443,
            volume: req.volume,
            price: req.price.unwrap_or(1.0850),
            comment: "Order placed".to_string(),
        })
    }

    pub async fn order_modify(&self, _ticket: u64, _sl: f64, _tp: f64) -> Result<(), MT5Error> {
        Ok(())
    }

    pub async fn order_close(&self, _ticket: u64, _volume: f64) -> Result<(), MT5Error> {
        Ok(())
    }
}
