use metarpc_mt5::{MT5Client, OrderRequest, TradeAction, AccountInfo};

#[tokio::test]
async fn test_client_connection_lifecycle() {
    let mut client = MT5Client::new("mt5.mrpc.pro", 443);
    assert!(!client.is_connected());

    let conn_res = client.connect(1001, "demo_pass").await;
    assert!(conn_res.is_ok());
    assert!(client.is_connected());

    let acc_res = client.get_account_info().await;
    assert!(acc_res.is_ok());
    let acc = acc_res.unwrap();
    assert_eq!(acc.currency, "USD");
    assert!(acc.balance > 0.0);

    client.disconnect().await;
    assert!(!client.is_connected());
}

#[tokio::test]
async fn test_order_lifecycle() {
    let mut client = MT5Client::new("mt5.mrpc.pro", 443);
    client.connect(1001, "demo_pass").await.unwrap();

    let req = OrderRequest {
        symbol: "EURUSD".to_string(),
        action: TradeAction::Buy,
        volume: 0.1,
        price: Some(1.0850),
        stop_loss: Some(1.0800),
        take_profit: Some(1.0900),
        slippage: Some(5),
        comment: Some("Test order".to_string()),
    };

    let res = client.order_send(req).await;
    assert!(res.is_ok());
    let order = res.unwrap();
    assert_eq!(order.volume, 0.1);
    assert_eq!(order.retcode, 0);

    let mod_res = client.order_modify(order.ticket, 1.0790, 1.0910).await;
    assert!(mod_res.is_ok());

    let close_res = client.order_close(order.ticket, 0.1).await;
    assert!(close_res.is_ok());
}

#[test]
fn test_serialization() {
    let acc = AccountInfo {
        login: 12345,
        currency: "EUR".to_string(),
        balance: 10000.0,
        equity: 10000.0,
        margin: 0.0,
        free_margin: 10000.0,
        margin_level: 0.0,
        leverage: 100,
        name: "Test User".to_string(),
        server: "demo".to_string(),
    };

    let json = serde_json::to_string(&acc).expect("Failed to serialize");
    let deserialized: AccountInfo = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(deserialized.login, 12345);
    assert_eq!(deserialized.currency, "EUR");
}