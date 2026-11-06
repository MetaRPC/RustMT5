# MT5Account — Master Overview

> One page to orient fast: the low-level async gRPC client for MetaTrader 5. Jump to any group overview or method spec from here.

`MT5Account` is **Layer 1** — direct gRPC/protobuf calls to the MT5 terminal gateway. [`MT5Service`](../MT5Service/MT5Service.Overview.md) (typed facade) and [`MT5Sugar`](../MT5Sugar/MT5Sugar.Master.Overview.md) (high-level helpers) build on top of it.

It exposes **39 feature methods** across 6 groups (plus connection helpers). Defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs).

MT5 is richer than MT4: positions and deals are separate from orders, there is a Market Book (DOM), and pre-trade `order_check` / `order_calc_*` calculators.

---

## 🚦 Start here — Section Overviews

* **[1. Account Information — Overview](./1.%20Account_Information/Account_Information.Overview.md)**
  Balance/equity/margin/leverage, a complete snapshot or single typed properties.
* **[2. Symbol Information — Overview](./2.%20Symbol_Information/Symbol_Information.Overview.md)**
  Quotes (ticks), symbol properties, trading rules, Market Watch select, bar history.
* **[3. Positions & Orders — Overview](./3.%20Positions_Orders/Positions_Orders.Overview.md)**
  Open positions, pending orders, their ticket lists, and closed-deal/order history.
* **[4. Market Depth (DOM) — Overview](./4.%20Market_Depth/Market_Depth.Overview.md)**
  Level-II order book: subscribe, read, release.
* **[5. Trading Operations — Overview](./5.%20Trading_Operations/Trading_Operations.Overview.md)**
  Send/modify/close orders, pre-trade check and margin/profit calculators.
* **[6. Streaming Methods — Overview](./6.%20Streaming_Methods/Streaming_Methods.Overview.md)**
  Real-time server streams: ticks, trade events, position profit, ticket lists, transactions.

---

## 🧭 How to pick a method

| If you need… | Group | Typical methods |
|--------------|-------|-----------------|
| Account snapshot | Account Information | `account_summary`, `account_info_double/integer/string` |
| Quotes & symbol properties | Symbol Information | `symbol_info_tick`, `symbol_info_double`, `symbols_total`, `symbol_params_many` |
| Bar history | Symbol Information | `price_history` |
| Current positions & orders | Positions & Orders | `positions_total`, `opened_orders`, `opened_orders_tickets` |
| Historical trades | Positions & Orders | `order_history`, `positions_history` |
| Tick value for P/L | Positions & Orders | `tick_value_with_size` |
| Level-II / order book | Market Depth | `market_book_add`, `market_book_get`, `market_book_release` |
| Trading operations | Trading Operations | `order_send`, `order_modify`, `order_close` |
| Pre-trade calculations | Trading Operations | `order_check`, `order_calc_margin`, `order_calc_profit` |
| Real-time updates | Streaming Methods | `on_symbol_tick`, `on_trade`, `on_position_profit` |

---

## 🔌 Usage pattern (Rust, async)

`MT5Account` is **async** (tokio): each call returns the reply's typed `…Data` payload, or an [`MT5Error`](../All_Guides/RETURN_CODES_REFERENCE.md). Every request carries the `apikey` + terminal `id` metadata automatically.

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use metarpc_mt5::MT5Account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // user, password, grpc_server (None -> mt5.mrpc.pro:443), api key from MRPC_API_KEY / "TRIAL"
    let mut account = MT5Account::new(12_345_678, "password", None);
    account.connect_by_server_name("MetaQuotes-Demo", 30).await?;

    let s = account.account_summary().await?;
    println!("{} {} eq {:.2}", s.account_balance, s.account_currency, s.account_equity);

    let tick = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: "EURUSD".into() }).await?;
    println!("bid/ask {:.5}/{:.5}", tick.bid, tick.ask);

    account.disconnect().await?;
    Ok(())
}
```

---

## 🧱 How the call contract works

Every feature method:

1. builds the protobuf `…Request`,
2. attaches the `apikey` + `id` metadata (`authed`),
3. calls the right tonic service client,
4. unwraps the reply — each `…Reply` carries a `oneof { data, error }`, returned as `Result<…Data, MT5Error>`.

So you always get the typed `…Data` directly, never a raw reply to inspect by hand. A few low-level methods take a raw `…Request` (MT5's requests often have many fields); friendly argument shapes live in [`MT5Service`](../MT5Service/MT5Service.Overview.md) and [`MT5Sugar`](../MT5Sugar/MT5Sugar.Master.Overview.md).

---

## 🔐 Constructor & connection

```rust
MT5Account::new(user: u64, password: impl Into<String>, grpc_server: Option<String>) -> MT5Account
```

Connect by broker **server name** or explicit **host/port**:

```rust
account.connect_by_server_name("MetaQuotes-Demo", 30).await?;   // by cluster name
account.connect_by_host_port("mt5.broker.com", 443, 30).await?; // by address
```

The gateway assigns a terminal session `id` on connect (`account.id()`), reused on every later call. `reconnect()` re-establishes it; `disconnect()` drops the channel.

---

## 🧩 Layer map

| Layer | Type | What it adds |
|-------|------|--------------|
| 1 — low-level | **`MT5Account`** | raw gRPC, auto-metadata, typed `…Data` returns |
| 2 — facade | [`MT5Service`](../MT5Service/MT5Service.Overview.md) | friendly args (property enums, symbols), clean values |
| 3 — high-level | [`MT5Sugar`](../MT5Sugar/MT5Sugar.Master.Overview.md) | order construction, pip math, risk sizing, positions |

---

**See also:** [Getting Started](../All_Guides/GETTING_STARTED.md) · [Return Codes](../All_Guides/RETURN_CODES_REFERENCE.md) · [ENUMs Reference](../All_Guides/ENUMS_USAGE_REFERENCE.md)
