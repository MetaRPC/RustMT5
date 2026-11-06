//! Quickstart: connect to MT5 via the gateway and read the account.
//!
//! Run with your demo credentials in the environment:
//!   MT5_LOGIN=12345678 MT5_PASSWORD=yourpass MT5_SERVER=MetaQuotes-Demo \
//!     MRPC_API_KEY=TRIAL cargo run --example quickstart

use metarpc_mt5::{MT5Account, MT5Service, MT5Sugar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = metarpc_mt5::Settings::load()?;

    // Build the three layers: Account (low-level) -> Service (facade) -> Sugar (high-level).
    let account = MT5Account::new(cfg.user, cfg.password.clone(), cfg.grpc_server.clone()); // None -> mt5.mrpc.pro:443
    let service = MT5Service::new(account);
    let mut sugar = MT5Sugar::new(service).with_default_symbol("EURUSD");

    // Connect by broker server name; the gateway assigns the terminal id.
    sugar.connect_by_server_name(&cfg.server_name, 30).await?;

    let info = sugar.get_account_info().await?;
    println!(
        "Login {} | {} {} | balance {:.2} | equity {:.2}",
        info.login, info.company, info.currency, info.balance, info.equity
    );

    println!(
        "EURUSD  bid/ask: {:.5} / {:.5}  (spread {:.5})",
        sugar.bid(None).await?,
        sugar.ask(None).await?,
        sugar.spread(None).await?
    );

    println!("open positions: {}", sugar.count_open_positions(None).await?);

    sugar.disconnect(false).await?;
    Ok(())
}
