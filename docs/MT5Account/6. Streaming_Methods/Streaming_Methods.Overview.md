# 6. Streaming Methods — Overview

> Real-time server-push streams: ticks, trade events, live position profit, ticket lists, and raw trade transactions.

Unlike every other group, these return a **stream**, not a single value. Each method yields:

```rust
Result<impl tokio_stream::Stream<Item = Result<…Data, MT5Error>>, MT5Error>
```

The outer `Result` reports whether the subscription **opened**; then you iterate the stream, and each item is either a data frame or an `MT5Error`. Under the hood they use the gateway's `SubscriptionService` and carry the usual `apikey` + terminal `id` metadata.

---

## Methods in this group

| Method | Item type | One-liner |
|--------|-----------|-----------|
| [`on_symbol_tick(req)`](./on_symbol_tick.md) | `OnSymbolTickData` | Live ticks for one or more symbols. |
| [`on_trade(req)`](./on_trade.md) | `OnTradeData` | Trade events (orders/positions/deals change) + account snapshot. |
| [`on_position_profit(req)`](./on_position_profit.md) | `OnPositionProfitData` | Per-position floating P/L on a timer (new/updated/deleted). |
| [`on_positions_and_pending_orders_tickets(req)`](./on_positions_and_pending_orders_tickets.md) | `OnPositionsAndPendingOrdersTicketsData` | Open-position & pending-order ticket lists on a timer. |
| [`on_trade_transaction(req)`](./on_trade_transaction.md) | `OnTradeTransactionData` | The raw MT5 `OnTradeTransaction` feed (transaction + request + result). |

---

## Consuming a stream (pattern)

```rust
use tokio_stream::StreamExt;  // brings `.next()` into scope
use metarpc_mt5::proto::mt5_term_api as pb;

let mut stream = account
    .on_symbol_tick(pb::OnSymbolTickRequest { symbol_names: vec!["EURUSD".into()] })
    .await?;                       // subscription opened

while let Some(item) = stream.next().await {
    match item {
        Ok(data) => { /* handle frame */ }
        Err(e)   => { eprintln!("stream error: {e}"); break; }
    }
}
```

* **Timer-based streams** (`on_position_profit`, `on_positions_and_pending_orders_tickets`) push every `timer_period_milliseconds`.
* **Event-based streams** (`on_trade`, `on_trade_transaction`) push when something actually changes.
* **`on_symbol_tick`** pushes on every new tick for the subscribed symbols.
* Dropping the stream unsubscribes.

---

## When to use which

* **Live prices** → `on_symbol_tick` (vs polling `symbol_info_tick`).
* **React to fills/closes** → `on_trade` or the lower-level `on_trade_transaction`.
* **Live P/L dashboard** → `on_position_profit`.
* **Track what's open without re-fetching details** → `on_positions_and_pending_orders_tickets`.

---

**Back to:** [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
