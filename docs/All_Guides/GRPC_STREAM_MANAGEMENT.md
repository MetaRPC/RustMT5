# gRPC Streaming & Subscription Management

> How real-time subscriptions work in RustMT5 — how to open one, consume it, stop it cleanly, and the patterns you'll reuse.

---

## The streaming model

A streaming method returns:

```rust
Result<impl tokio_stream::Stream<Item = Result<…Data, MT5Error>>, MT5Error>
//     └─ outer: did the subscription OPEN? ──┘        └─ inner: each FRAME or an error ┘
```

- **Outer `Result`** — resolves once, when you `.await` the call: `Ok(stream)` means the subscription opened; `Err` means it never started.
- **Inner `Result` per item** — each `.next().await` yields `Some(Ok(data))` (a frame), `Some(Err(e))` (a stream error), or `None` (the stream ended).

Bring the `.next()` method into scope with `use tokio_stream::StreamExt;`.

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut stream = account
    .on_symbol_tick(pb::OnSymbolTickRequest { symbol_names: vec!["EURUSD".into()] })
    .await?;                              // outer: opened

while let Some(item) = stream.next().await {
    match item {
        Ok(d) => if let Some(t) = d.symbol_tick { println!("{} {:.5}", t.symbol, t.bid); },
        Err(e) => { eprintln!("stream error: {e}"); break; }
    }
}
// leaving scope drops `stream` → unsubscribes
```

Every frame carries the `apikey` + session `id` metadata automatically, the same as unary calls.

---

## The five streams

Available on `MT5Account` (and wrapped by `MT5Service` with friendlier args):

| MT5Account | MT5Service | Item | Trigger |
|------------|------------|------|---------|
| `on_symbol_tick(req)` | `stream_ticks(symbols)` | `OnSymbolTickData` | every tick |
| `on_trade(req)` | `stream_trade_updates()` | `OnTradeData` | on trade events |
| `on_position_profit(req)` | `stream_position_profits(ms, ignore_empty)` | `OnPositionProfitData` | timer |
| `on_positions_and_pending_orders_tickets(req)` | `stream_opened_tickets(ms)` | `OnPositionsAndPendingOrdersTicketsData` | timer |
| `on_trade_transaction(req)` | `stream_transactions()` | `OnTradeTransactionData` | on transactions |

- **Tick stream** pushes on every new tick for the subscribed symbols.
- **Timer streams** push every `timer_period_milliseconds`; `on_position_profit` also takes `ignore_empty_data` to suppress no-change frames.
- **Event streams** push only when something changes (quiet otherwise).

---

## Stopping a stream

**Drop it.** When the `Stream` value goes out of scope (loop ends, `break`, function returns), the subscription is torn down — there's no explicit `unsubscribe` for these feeds.

```rust
let mut s = service.stream_ticks(vec!["EURUSD".into()]).await?;
let mut n = 0;
while let Some(Ok(_)) = s.next().await {
    n += 1;
    if n >= 100 { break; }   // break → drop(s) → unsubscribed
}
```

> Market **Depth** (DOM) is different — it's a subscribe/read/release trio (`market_book_add` → `market_book_get` → `market_book_release`), not one of these push streams. See [Market Depth](../MT5Account/4.%20Market_Depth/Market_Depth.Overview.md).

---

## Patterns

### 1) Take a fixed number of frames

```rust
use tokio_stream::StreamExt;

let mut s = service.stream_ticks(vec!["EURUSD".into()]).await?;
let mut got = 0;
while let Some(item) = s.next().await {
    if let Ok(d) = item { if d.symbol_tick.is_some() { got += 1; } }
    if got >= 10 { break; }
}
```

### 2) Stop on a deadline

```rust
use tokio_stream::StreamExt;
use std::time::{Duration, Instant};

let mut s = service.stream_position_profits(1000, true).await?;
let deadline = Instant::now() + Duration::from_secs(30);
while Instant::now() < deadline {
    match tokio::time::timeout(Duration::from_secs(1), s.next()).await {
        Ok(Some(Ok(d))) => { /* handle frame */ let _ = d; }
        Ok(Some(Err(e))) => { eprintln!("err: {e}"); break; }
        Ok(None) => break,            // stream ended
        Err(_elapsed) => continue,    // no frame this second — re-check the deadline
    }
}
```

### 3) Cancel from elsewhere (channel / token)

```rust
use tokio_stream::StreamExt;

let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();
let mut s = service.stream_trade_updates().await?;

loop {
    tokio::select! {
        _ = &mut rx => break,                 // someone asked us to stop
        item = s.next() => match item {
            Some(Ok(d)) => { let _ = d; }
            _ => break,
        }
    }
}
// elsewhere: let _ = tx.send(());
```

### 4) Run a stream in a background task

```rust
use tokio_stream::StreamExt;

let mut s = service.stream_ticks(vec!["EURUSD".into()]).await?;
let handle = tokio::spawn(async move {
    while let Some(Ok(d)) = s.next().await {
        if let Some(t) = d.symbol_tick { /* push to a channel, update state, … */ let _ = t; }
    }
});
// ... later ...
handle.abort();   // dropping the task drops the stream → unsubscribes
```

### 5) Maintain a live position book

```rust
use tokio_stream::StreamExt;
use std::collections::HashMap;

let mut s = service.stream_position_profits(1000, true).await?;
let mut book: HashMap<i64, f64> = HashMap::new();
while let Some(Ok(d)) = s.next().await {
    for p in d.new_positions.iter().chain(d.updated_positions.iter()) { book.insert(p.ticket, p.profit); }
    for p in &d.deleted_positions { book.remove(&p.ticket); }
    println!("floating {:+.2} ({} pos)", book.values().sum::<f64>(), book.len());
}
```

---

## Best practices

- **Don't block the frame loop.** Do heavy work off the loop (send to a channel, `tokio::spawn`), so you keep draining frames promptly.
- **Pick a sane timer period.** 500–1000 ms is plenty for dashboards; smaller means more frames and more work.
- **Use `ignore_empty_data: true`** on `stream_position_profits` to stay quiet when nothing changed.
- **Handle both `Result` layers.** An opened stream can still yield an `Err` item (e.g. a transient gateway hiccup) — decide whether to `break` or continue.
- **Snapshot vs stream.** For a one-off value, use the unary call (`symbol_info_tick`, `opened_orders`); stream only when you need continuous updates.

---

## Troubleshooting

| Symptom | Likely cause |
|---------|--------------|
| `.next()` doesn't resolve / method missing | Forgot `use tokio_stream::StreamExt;`. |
| No frames arrive | Symbol not selected/synchronized (ticks), or market closed / nothing changed (event streams). |
| Stream "leaks" / keeps running | The `Stream` value is still alive — drop it or `abort()` its task. |
| Flood of frames | Timer period too small, or `ignore_empty_data` is `false`. |

---

## 📚 See also

- [Streaming Methods — Overview (MT5Account)](../MT5Account/6.%20Streaming_Methods/Streaming_Methods.Overview.md)
- [MT5Service — Streaming](../MT5Service/6.%20Streaming_Methods.md)
- [on_symbol_tick](../MT5Account/6.%20Streaming_Methods/on_symbol_tick.md) · [on_position_profit](../MT5Account/6.%20Streaming_Methods/on_position_profit.md)
