//! Smoke tests that need no network: construction and default state.

use metarpc_mt5::MT5Account;

#[test]
fn account_constructs_unconnected() {
    let acc = MT5Account::new(12_345_678, "password", None);
    assert!(acc.id().is_none(), "no terminal id before connect");
    assert!(acc.server_name().is_none(), "no server name before connect");
}

#[test]
fn account_accepts_custom_gateway() {
    let acc = MT5Account::new(1, "p", Some("localhost:50051".to_string()));
    assert!(acc.id().is_none());
}
