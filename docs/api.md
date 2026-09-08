# API Reference

### `MT5Client` Methods
- `pub fn new(host: impl Into<String>, port: u16) -> Self`
- `pub async fn connect(&mut self, login: u64, password: &str) -> Result<(), MT5Error>`
- `pub async fn disconnect(&mut self)`
- `pub fn is_connected(&self) -> bool`
- `pub async fn get_account_info(&self) -> Result<AccountInfo, MT5Error>`
- `pub async fn subscribe_quotes(&self, symbols: Vec<String>) -> Result<mpsc::Receiver<Quote>, MT5Error>`
- `pub async fn order_send(&self, req: OrderRequest) -> Result<OrderResult, MT5Error>`
- `pub async fn order_modify(&self, ticket: u64, sl: f64, tp: f64) -> Result<(), MT5Error>`
- `pub async fn order_close(&self, ticket: u64, volume: f64) -> Result<(), MT5Error>`
