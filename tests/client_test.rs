//! Offline unit tests (no network). Live tests against the gateway run only when
//! `MT5_LOGIN`, `MT5_PASSWORD`, `MT5_SERVER` and `MRPC_API_KEY` are set:
//!   MT5_LOGIN=... MT5_PASSWORD=... MT5_SERVER=FBS-Demo MRPC_API_KEY=... cargo test -- --ignored

use metarpc_mt5::{mt5_term_api as pb, AccountInfo, MT5Client, OrderRequest, Quote, TradeAction, TradeMode};

#[test]
fn test_deterministic_id() {
    let id = MT5Client::compute_deterministic_id(12345678, "demo_password");
    assert_eq!(id, "6ce74465-7aa9-3a79-2bd8-e40241b50c43");
}

#[test]
fn test_stable_terminal_id_is_stable_and_uuid_shaped() {
    let a = MT5Client::stable_terminal_id("fleet-audit-RustMT5");
    let b = MT5Client::stable_terminal_id("fleet-audit-RustMT5");
    assert_eq!(a, b);
    assert_eq!(a.len(), 36);
    assert_ne!(a, MT5Client::stable_terminal_id("S001"));
}

#[tokio::test]
async fn test_get_id_generation() {
    let mut client = MT5Client::new("mt5.mrpc.pro", 443);
    let id = client.get_id(2005432, "demo_pass").await.unwrap();
    assert!(!id.is_empty());
    assert_eq!(client.id.as_deref(), Some(id.as_str()));
    assert!(!client.is_connected());
}

#[test]
fn test_order_request_to_proto() {
    let req = OrderRequest::market("EURUSD", TradeAction::Buy, 0.01)
        .with_magic(599999)
        .with_comment("fleet-audit")
        .with_sl_tp(Some(1.0), Some(2.0));
    let p = req.to_proto();
    assert_eq!(p.operation, pb::Tmt5EnumOrderType::Tmt5OrderTypeBuy as i32);
    assert_eq!(p.expert_id, Some(599999));
    assert_eq!(p.comment.as_deref(), Some("fleet-audit"));
    assert_eq!(p.stop_loss, Some(1.0));
    assert_eq!(p.take_profit, Some(2.0));
    assert_eq!(p.price, None);
    assert_eq!(TradeAction::SellStopLimit.to_order_type() as i32, 7);
}

#[test]
fn test_trade_mode_and_quote_usable() {
    assert_eq!(TradeMode::from_i32(0), TradeMode::Demo);
    assert_eq!(TradeMode::from_i32(2), TradeMode::Real);
    assert!(TradeMode::from_i32(0).is_demo());
    let q = Quote { symbol: "X".into(), bid: 1.0, ask: 1.0, last: 0.0, volume: 0.0, time: 0, time_msc: 0, flags: 0 };
    assert!(!q.is_usable(), "bid == ask must be rejected");
    let q2 = Quote { bid: 1.0, ask: 1.0001, ..q };
    assert!(q2.is_usable());
}

#[test]
fn test_timeframe_parse() {
    use metarpc_mt5::timeframe;
    assert_eq!(timeframe::parse("m1"), Some(1));
    assert_eq!(timeframe::parse("H1"), Some(16385));
    assert_eq!(timeframe::parse("D1"), Some(16408));
    assert_eq!(timeframe::parse("X"), None);
}

#[test]
fn test_serialization() {
    let acc = AccountInfo {
        login: 12345,
        currency: "EUR".to_string(),
        balance: 10000.0,
        equity: 10000.0,
        credit: 0.0,
        leverage: 100,
        name: "Test User".to_string(),
        company: "Broker".to_string(),
        server: "demo".to_string(),
        trade_mode: TradeMode::Demo,
        server_time: 0,
        utc_shift_minutes: 180,
    };
    let json = serde_json::to_string(&acc).expect("Failed to serialize");
    let de: AccountInfo = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(de.login, 12345);
    assert_eq!(de.currency, "EUR");
    assert_eq!(de.trade_mode, TradeMode::Demo);
}

#[tokio::test]
#[ignore]
async fn live_connect_and_summary() {
    let login: u64 = std::env::var("MT5_LOGIN").unwrap().parse().unwrap();
    let password = std::env::var("MT5_PASSWORD").unwrap();
    let server = std::env::var("MT5_SERVER").unwrap();
    let client = MT5Client::new("mt5.mrpc.pro", 443).with_id(MT5Client::stable_terminal_id("rustmt5-live-test"));
    client.connect_ex(login, &password, &server, Some("rustmt5-live-test"), 120).await.unwrap();
    let acc = client.get_account_info().await.unwrap();
    assert_eq!(acc.login, login);
    assert!(acc.trade_mode.is_demo());
}
