# RustMT5 — fleet SDK audit (MT5, gRPC, prod `mt5.mrpc.pro:443`)

Audited 2026-10-20 on the probe account **FBS-Demo / 106805114** (demo, USD, 1:100). Runner name:
`rust`. Crate: `metarpc-mt5` (path dependency on `lib/RustMT5`, **not** on crates.io).

**Verdict: usable.** Every audit step passes on Windows and on the VPS (connect → account → symbol/
tick/specs → 50 M1 bars → 10 s tick stream → market buy 0.01 → modify SL/TP → close → history).
Smoke program: `fleet/runners/rust/smoke/` (prints one `PASS/FAIL` line per step, exit 0 only when
all pass).

> The vendored SDK as received was a **stub**: `connect()` never opened a socket,
> `get_account_info()` returned a hard-coded 50 000 USD, `order_send()` returned a fake ticket. It
> was rewritten in this audit into a real tonic/prost gRPC client (see §6). Anything you read in
> older docs (`docs/`, mkdocs site) describing `client.connect(login, password)` without a server
> name is the stub API and does not work.

---

## 1. Build / run

Toolchain: Rust ≥ 1.85 (uses `AsyncFn`; tested with 1.98 on Windows, 1.99 on Debian 12). No
`protoc`, no OpenSSL (TLS = rustls + webpki roots, pure Rust). First build compiles tonic/hyper/
rustls: ~1 min on Windows, several minutes on the 2-vCPU VPS (`nice -n 10 cargo build --release -j 2`).

```toml
# Cargo.toml of a runner / strategy crate (fleet/runners/rust/<name>/Cargo.toml)
[package]
name = "fleet-rust"
version = "0.1.0"
edition = "2021"
[workspace]                      # keep it stand-alone — no root workspace in the repo

[dependencies]
metarpc-mt5 = { path = "../../../../lib/RustMT5" }   # adjust depth to your folder
tokio = { version = "1", features = ["full"] }
```

```bash
# Windows (git bash or PowerShell) — offline unit tests of the SDK + smoke build + live run
cd C:/git/trading-bots/lib/RustMT5 && cargo test
cd C:/git/trading-bots/fleet/runners/rust/smoke && cargo build --release
MRPC_API_KEY=... MT5_LOGIN=106805114 MT5_PASSWORD=... MT5_SERVER=FBS-Demo ./target/release/fleet-smoke-rust.exe
#  (PowerShell: $env:MRPC_API_KEY="..."; $env:MT5_LOGIN="..."; ...; .\target\release\fleet-smoke-rust.exe)

# VPS (Debian 12, /opt/fleet-audit/RustMT5 or /opt/fleet)
tar -C C:/git/trading-bots --exclude='*/target' -cf - lib/RustMT5 fleet/runners/rust \
  | ssh -i ~/.ssh/mrpc_diag_key -p 31683 root@back.mrpc.pro 'mkdir -p /opt/fleet-audit/RustMT5 && tar -C /opt/fleet-audit/RustMT5 -xf -'
ssh -p 31683 root@back.mrpc.pro 'cd /opt/fleet-audit/RustMT5/fleet/runners/rust/smoke && nice -n 10 cargo build --release -j 2 \
  && MRPC_API_KEY=... MT5_LOGIN=106805114 MT5_PASSWORD=... MT5_SERVER=FBS-Demo ./target/release/fleet-smoke-rust'
```

Optional smoke env: `SMOKE_LABEL` (terminal-id label, default `fleet-audit-RustMT5`),
`SMOKE_SYMBOLS=EURUSD,XAUUSD,BTCUSD`, `SMOKE_NO_TRADE=1`.

No Linux-only problems were found: the crate is pure Rust, paths are relative, line endings are
irrelevant to cargo.

---

## 2. Copy-pasteable call reference (all verified live)

```rust
use std::time::{Duration, SystemTime};
use metarpc_mt5::{mt5_term_api as pb, timeframe, MT5Client, MT5Error, OrderRequest, TradeAction, TradeMode, Quote};
```

### Connect (ConnectEx by server name, stable terminal id)

```rust
// Terminal id: uuid5(NAMESPACE_DNS, label). Same label → same terminal on every run.
// The fleet contract uses the strategy id as the label ("S042").
let client = MT5Client::with_api_key("mt5.mrpc.pro", 443, api_key)       // or MT5Client::new(host, port) → MRPC_API_KEY env
    .with_id(MT5Client::stable_terminal_id("S042"));
let data: pb::ConnectData = client
    .connect_ex(login_u64, &password, "FBS-Demo", Some("S042"), 120)       // name = label shown in /Accounts; 120 s login timeout
    .await?;                                                               // Err(MT5Error::InvalidLogin(..)) is final — never retry
println!("{}", data.terminal_instance_guid);   // prod returns "mt5_live_<id-without-dashes>"; the SDK uses it as the `id` header from here on
```

* First connect of a new terminal: **~6 s**. Re-connect to an existing terminal (same id): **~0.1 s**.
* `client.reconnect()` re-runs ConnectEx with the stored parameters; `client.check_connect()` is a
  cheap liveness probe (`pb::CheckConnectData`).
* Do **not** call `client.disconnect(..)` at the end of a strategy run; the gateway keeps the
  terminal and the next run reuses it. `disconnect(true)` deletes the terminal.

### Account summary (abort unless DEMO)

```rust
let acc = client.get_account_info().await?;    // models::AccountInfo
// acc.login, acc.balance, acc.equity, acc.credit, acc.currency ("USD"), acc.leverage (100),
// acc.trade_mode: TradeMode::{Demo, Contest, Real}, acc.company, acc.utc_shift_minutes (180 for FBS)
if !acc.trade_mode.is_demo() { return Err("refusing to trade: not a demo account".into()); }
let raw: pb::AccountSummaryData = client.account_summary().await?;   // raw proto if you need server_time etc.
```

### Symbols, specs, quote

```rust
// Select in Market Watch, wait until synchronized AND a two-sided (bid < ask, both > 0) quote exists.
let q: Quote = client.ensure_symbol("EURUSD", Duration::from_secs(20)).await?;   // q.bid q.ask q.spread() q.time_msc
let q: Quote = client.quote("EURUSD").await?;                                    // SymbolInfoTick → Quote (authoritative price)
let t: pb::MrpcMqlTick = client.symbol_info_tick("EURUSD").await?;              // raw: time, bid, ask, last, volume, time_msc, flags
let ok = client.symbol_select("EURUSD", true).await?;                            // bool
let synced = client.symbol_is_synchronized("EURUSD").await?;
let exists = client.symbol_exist("BTCUSD").await?;

let p: pb::SymbolParameters = client.symbol_params("EURUSD").await?;             // exact-name match from SymbolParamsMany
// p.digits (5), p.volume_min (0.01), p.volume_step (0.01), p.volume_max (100), p.trade_contract_size (100000),
// p.trade_tick_size (0.00001), p.trade_tick_value (1), p.currency_base/currency_profit/currency_margin,
// p.trade_mode (4 = SYMBOL_TRADE_MODE_FULL; 0 = disabled, 1 = long only, 2 = short only, 3 = close only),
// p.spread, p.spread_float, p.bid/p.ask (may be stale — use quote()).
let all: pb::SymbolParamsManyData = client.symbol_params_many(None).await?;      // every symbol, ONE call (paging does not advance)
```

### Bars

```rust
let bars: Vec<pb::PriceBar> = client.price_history("EURUSD", timeframe::M1, 50).await?;   // newest 50 bars, oldest first
// PriceBar { time: i64 (unix s, bar open), open, high, low, close, tick_volume: i64, spread: i32, real_volume: i64 }
let bars = client.bars("EURUSD", timeframe::H1, 200).await?;                             // same as models::Bar
let bars = client.price_history_range("EURUSD", timeframe::M5, from_unix, to_unix, 500).await?;
// timeframe::{M1=1, M2, M3, M5=5, M10, M15=15, M30=30, H1=16385, H2, H4=16388, H12, D1=16408, W1=32769, MN1=49153}
// timeframe::parse("H4") -> Option<i32>
```

Bar `time` values are the terminal's bar times (broker server time zone, i.e. UTC+shift). The
newest bar is the still-forming one. 50 M1 bars: **~50–120 ms**.

### Tick stream (subscribe / unsubscribe)

```rust
let mut rx = client.subscribe_quotes(vec!["EURUSD".into(), "XAUUSD".into()]).await?;   // tokio mpsc::Receiver<Quote>
while let Some(q) = rx.recv().await {           // q.symbol, q.bid, q.ask, q.time_msc; q.is_usable() rejects bid==ask / zero
    if !q.is_usable() { continue; }
    // ...
}
drop(rx);                                       // unsubscribe = drop the receiver (cancels the gRPC stream)
// Raw stream: let mut s: tonic::Streaming<pb::OnSymbolTickReply> = client.on_symbol_tick(&["EURUSD"]).await?; s.message().await
// Trade events: client.on_trade().await? -> Streaming<pb::OnTradeReply>
```

The receiver closes when the stream errors (gateway restart). Re-subscribe in a loop with
back-off; `client.reconnect()` first if `check_connect()` fails. EURUSD at 00:30 UTC delivered
1.5–2.3 ticks/s.

### Orders: open (market), modify SL/TP, close

```rust
let req = OrderRequest::market("EURUSD", TradeAction::Buy, 0.01)   // volume = lots (use p.volume_min / round to p.volume_step)
    .with_magic(600042)                                            // → expert_id; positions are filtered by this
    .with_comment("S042")                                          // ≤ 31 chars; broker may append text
    .with_sl_tp(None, None);                                       // SL/TP can be set on open too
let res = client.order_send(req).await?;                           // Err(MT5Error::TradeError(retcode, desc)) unless 10008/10009/10010
// res.retcode (10009 = TRADE_RETCODE_DONE), res.order (= position ticket on hedging accounts), res.deal, res.price (fill), res.volume
// res.bid/res.ask come back 0 from the gateway — ignore them.

// Position ticket: on FBS (hedging) position.ticket == res.order. Verify through the magic filter:
let pos = client.positions_by_magic(600042).await?.into_iter().find(|p| p.symbol == "EURUSD").unwrap();
// pb::PositionInfo { ticket, symbol, volume, price_open, price_current, stop_loss, take_profit, profit, swap,
//                    magic_number: i64, comment, r#type (0 = BUY, 1 = SELL), open_time, identifier, .. }

let m = client.order_modify(pos.ticket, Some(sl_price), Some(tp_price)).await?;   // absolute prices, rounded to p.digits
// m.returned_code == 10009; passing None leaves the gateway default (0 = remove)

let c = client.order_close(pos.ticket, pos.volume, 50).await?;      // volume in lots (full = pos.volume), slippage in points
// c.returned_code == 10009, c.close_mode

// Pending orders: TradeAction::{BuyLimit, SellLimit, BuyStop, SellStop, BuyStopLimit, SellStopLimit} with `price: Some(..)`
// (and `stop_limit_price` for stop-limits). Raw access: client.order_send_raw(pb::OrderSendRequest { .. }),
// client.order_modify_raw(pb::OrderModifyRequest { price, expiration_time_type, .. }).
```

Round-trip latencies on prod (Windows box → gateway): OrderSend **0.07–0.46 s**, OrderModify
**0.08–0.48 s**, OrderClose **0.09–0.49 s**. Retcodes: `models::{TRADE_RETCODE_PLACED=10008,
TRADE_RETCODE_DONE=10009, TRADE_RETCODE_DONE_PARTIAL=10010, TRADE_RETCODE_MARKET_CLOSED=10018}`,
`models::retcode_ok(rc)`. Trade calls are **not** auto-retried (a timed-out OrderSend may have
filled — check `positions_by_magic` before resending).

### Open state

```rust
let all: pb::OpenedOrdersData = client.opened_orders().await?;   // .opened_orders (pending) + .position_infos (open positions)
let mine: Vec<pb::PositionInfo> = client.positions_by_magic(600042).await?;
let n: i32 = client.positions_total().await?;
```

### History (closed deals / orders / positions)

```rust
let now = SystemTime::now();
let h: pb::OrdersHistoryData = client.order_history(now - Duration::from_secs(3600), now + Duration::from_secs(60)).await?;
for row in &h.history_data {                                       // one row per order; deals attached
    if let Some(o) = &row.history_order { /* pb::OrderHistoryData: ticket, magic_number, position_id, symbol, comment, state, r#type, volume_initial, price_open, setup_time, done_time */ }
    if let Some(d) = &row.history_deal  { /* pb::DealHistoryData: ticket, position_id, symbol, volume, price, profit, commission, swap, fee, entry_type (0 IN / 1 OUT), r#type, reason, comment, time */ }
}
let closed: Vec<pb::PositionHistoryInfo> = client.positions_history(now - Duration::from_secs(86400), now).await?;
// PositionHistoryInfo: position_ticket, symbol, order_type (0 BUY/1 SELL), volume, open_price, close_price, stop_loss, take_profit,
//                      profit, commission, swap, fee, magic: i64, comment ("S042; " — broker appends "; "), open_time, close_time
let my_closed: Vec<_> = closed.into_iter().filter(|p| p.magic == 600042).collect();
```

**Time zone trap (cost a failed step in this audit):** the gateway's history windows
(`inputFrom`/`inputTo`, `positionOpenTimeFrom/To`) and every timestamp it returns are in **broker
server time** (FBS = UTC+3). `order_history()` / `positions_history()` take **UTC** `SystemTime`s
and shift them using `utc_timezone_server_time_shift_minutes` learned from `account_summary()`
(called automatically if needed). `client.to_utc_secs(&ts)` converts a returned timestamp back to
unix UTC; `client.server_utc_shift_secs()` exposes the shift. The `*_server_time` variants take
windows already in server time. A closing deal has an empty comment — match the audit/strategy
trade by `position_id` or by the order's `magic_number`, not by the deal comment.

### Errors, retries, reconnect

```rust
match client.account_summary().await {
    Err(MT5Error::InvalidLogin(msg)) => { /* terminal: log, stop this strategy, never retry */ }
    Err(MT5Error::Api { code, message, mql_code, mql_trade_code, .. }) => { /* gateway error, e.g. "TERMINAL_INSTANCE_NOT_FOUND" */ }
    Err(MT5Error::TradeError(retcode, desc)) => { /* OrderSend/Modify/Close retcode != 10008/9/10 */ }
    Err(MT5Error::Grpc(status)) => { /* tonic::Status: Unavailable / DeadlineExceeded / .. */ }
    Err(e) => { /* Transport, Protocol, NotConnected, Io */ }
    Ok(d) => {}
}
```

* Every read call goes through `client.call(..)`: up to `max_retries` (3) attempts with back-off
  1 s / 2 s / 4 s on `Unavailable` / `DeadlineExceeded` / `Cancelled`, and on
  `TERMINAL_INSTANCE_NOT_FOUND` / `TERMINAL_REGISTRY_TERMINAL_NOT_FOUND` it first calls
  `reconnect()` (ConnectEx with the same id). `InvalidLogin` is never retried.
* Per-call deadline `client.call_timeout` (default 60 s; `with_call_timeout(..)`). Streams get no
  deadline. HTTP/2 keep-alive 30 s, TCP keep-alive 30 s, connect timeout 20 s.
* `MT5Client` is `Send + Sync`; share it with `Arc` across tasks. All methods take `&self`.

---

## 3. Enums and constants (`metarpc_mt5::mt5_term_api`)

| Purpose | Type / value |
|---|---|
| Order type (`OrderSendRequest.operation`) | `pb::Tmt5EnumOrderType`: Buy=0, Sell=1, BuyLimit=2, SellLimit=3, BuyStop=4, SellStop=5, BuyStopLimit=6, SellStopLimit=7, CloseBy=8 (`TradeAction::to_order_type()`) |
| Expiration (`expiration_time_type`) | `pb::Tmt5EnumOrderTypeTime`: Gtc=0, Day=1, Specified=2, SpecifiedDay=3 |
| Account trade mode | `pb::MrpcEnumAccountTradeMode`: Demo=0, Contest=1, Real=2 (`models::TradeMode`) |
| Timeframes (`PriceHistoryRequest.timeframe`) | MQL5 ints, `timeframe::{M1..MN1}` (H1 = 16385, D1 = 16408) |
| Position type (`PositionInfo.r#type`) | 0 = BUY, 1 = SELL |
| Deal entry (`DealHistoryData.entry_type`) | 0 = IN, 1 = OUT, 2 = INOUT, 3 = OUT_BY |
| History sort | `pb::Bmt5EnumOrderHistorySortType` (SDK uses CloseTimeDesc=3), `pb::AhEnumPositionsHistorySortType` (OpenTimeDesc=1) |
| Symbol trade mode (`SymbolParameters.trade_mode`) | 0 disabled, 1 long only, 2 short only, 3 close only, 4 full |
| gRPC metadata | `id` = terminal instance id, `apikey` = API key (lower-case; set by `client.request()`) |

Magic: `OrderRequest.magic` → `expert_id` → `PositionInfo.magic_number` / `OrderHistoryData.magic_number`
/ `PositionHistoryInfo.magic` (all `i64` on the read side, `u64` on the request).

---

## 4. Performance (prod, 2026-10-20 00:30 UTC, EURUSD)

| Metric | Windows box | VPS (Debian 12, 2 vCPU) |
|---|---|---|
| ConnectEx, new terminal | 6.1 s | (terminal already existed) |
| ConnectEx, existing terminal (stable id) | 0.10 s | 0.02 s |
| AccountSummary / SymbolInfoTick / SymbolParamsMany | < 0.1 s each | < 0.05 s each |
| PriceHistory 50×M1 | 0.05–0.12 s | < 0.01 s |
| Tick stream EURUSD (quiet hour) | 1.5–2.3 ticks/s | 0.9 ticks/s (same feed, quieter minute) |
| OrderSend / OrderModify / OrderClose | 0.07 / 0.08 / 0.09 s (warm) | 0.04 / 0.04 / 0.04 s |
| Process RSS at end of smoke | 8.4 MB (release build) | 6.3 MB |
| Whole smoke (incl. 10 s stream) | 11.3 s warm, 21.6 s cold | 10.3 s |
| `cargo build --release` of the smoke crate (cold, all deps) | ~1 min | 2 min 09 s (`nice -n 10 -j 2`) |
| Release binary size | — | 6.8 MB |
| `target/` footprint | — | 335 MB release; a debug `cargo test` target is another ~850 MB — delete it on the VPS after testing |

The VPS sits much closer to the gateway: every unary call is 2–3× faster than from the Windows box.
A Rust runner hosting the 7 planned strategies will stay well under 50 MB RSS.

### 4b. VPS run (2026-10-20 00:37 UTC, `/opt/fleet-audit/RustMT5/build.log`)

```
PASS a.connect: ConnectEx ok in 0.02s, terminalInstanceGuid=mt5_live_0072b5c0f74f5079ab8a1c03f16e12b1 type=1
PASS b.account: login=106805114 balance=99999.83 equity=99999.83 USD leverage=1:100 trade_mode=Demo company="FBS Markets Inc." server_time=1792467451 utc_shift_min=180
PASS c.symbol: EURUSD: bid=1.12191 ask=1.1219999999999999 spread=0.000090 digits=5 contract=100000 vol_min=0.01 vol_step=0.01 vol_max=100 tick_size=0.00001 tick_value=1 base=EUR profit=USD trade_mode=4
PASS d.bars: 50 M1 bars in 0.00s; first t=1792464480 last t=1792467420 o=1.12191 h=1.12192 l=1.12187 c=1.12191 tv=14
PASS e.stream: 9 ticks in 10.0s (0.90/s); last bid=1.1219 ask=1.12199 t_msc=1792467458683; unsubscribed
PASS f.order_open: EURUSD BUY 0.01 retcode=10009 (Request completed) order=2100556970 deal=1867586635 price=1.12199 bid=0 ask=0 in 0.04s
     position ticket=2100556970 (order ticket 2100556970) vol=0.01 open=1.12199 sl=0 tp=0 magic=599999 comment="fleet-audit"
PASS f.order_modify: ticket=2100556970 sl=1.11638 tp=1.1276 retcode=10009 (Request completed) in 0.04s; position now sl/tp=Some((1.11638, 1.1276))
PASS f.order_close: ticket=2100556970 retcode=10009 (Request completed) close_mode=0 in 0.04s; positions with magic left=0
PASS g.history: 6 orders + 6 deals in last hour (array_total=6); audit trade found: Some((1867586637, 2100556970, "EURUSD", 0.01, 1.1219, -0.09, ""))
     PositionsHistory: 3 closed positions in window, 3 with magic 599999: [(2100556970, "EURUSD", 0.01, 1.12199, 1.1219, -0.09), ...]
PASS h.resources: RSS=6.3 MB runtime=10.3s
RESULT: ALL PASS (10.3s)
```

Windows run (same day, 00:33 UTC) was identical except for the latencies in the table above. The
audit terminal `fleet-audit-RustMT5` (id `0072b5c0-f74f-5079-ab8a-1c03f16e12b1`) was left
connected on prod on purpose.

---

## 5. Bugs found and fixed (all in `lib/RustMT5`, mirrored to `C:/git/setup_RustMT5` and committed there)

| # | Bug | Fix |
|---|---|---|
| 1 | **The SDK was a stub.** `src/client.rs` had no gRPC at all: `connect()` set a flag, `get_account_info()` returned constant fake data, `order_send()` a fake ticket 100987654, `subscribe_quotes()` an immediately-closed channel. `Cargo.toml` had no tonic/prost although `package/mt5_term_api.rs` is prost/tonic output. | Rewrote `client.rs` as a real tonic 0.12 client over the generated code; added `src/pb.rs` (`include!` of `package/mt5_term_api.rs`), tonic/prost/prost-types/tokio-stream/uuid deps, real error enum, models extended (`TradeMode`, `magic`, `Bar`, retcodes). Old helpers kept (`compute_deterministic_id`, `get_id`, `with_auth`). |
| 2 | Generated `ConnectionClient` does not compile: tonic's `connect<D>(dst)` constructor collides with the `Connect` RPC method (`error[E0592]: duplicate definitions with name connect`). | Renamed the constructor to `connect_endpoint` in `package/mt5_term_api.rs` (use `ConnectionClient::new(channel)` anyway). |
| 3 | `PositionsTotal` RPC takes `google.protobuf.Empty` — there is no `PositionsTotalRequest` type. | SDK passes `()` (prost maps Empty to unit). |
| 4 | History windows silently empty: the gateway interprets `OrderHistory.inputFrom/inputTo` and `PositionsHistory` bounds in broker server time; a UTC window of the last hour returned 0 rows for a trade made 30 s earlier. | `order_history()` / `positions_history()` shift UTC → server time using `utc_timezone_server_time_shift_minutes` from `AccountSummary`; `*_server_time` raw variants and `to_utc_secs()` added. |
| 5 | `README.md` said `cargo add rustmt5` and showed the stub API; `examples/quickstart.rs` and `tests/client_test.rs` exercised the stub (tests "passed" without a network). | README rewritten; quickstart does connect/account/quote/bars/stream; tests are offline unit tests plus one `#[ignore]` live test. |
| 6 | `SymbolExistRequest` field is `name`, not `symbol` (documentation mismatch). | Wrapped by `client.symbol_exist(symbol)`. |

Not bugs but observations: `OrderSendData.bid/ask` are always 0 on prod; `AccountSummaryData.server_time`
was a 2022-dated value on the very first call after terminal start (correct on later calls); the
gateway returns `terminal_instance_guid = "mt5_live_<id without dashes>"`, i.e. not byte-equal to
the id you sent — the SDK stores and reuses it for the `id` header (both forms are accepted by
the gateway).

---

## 6. What a strategy implementer must know

1. **Stable terminal id per strategy**: `MT5Client::stable_terminal_id(strategy_id)` = UUID v5
   (DNS namespace) of the id string. A random id per run orphans terminals; the gateway caps them.
2. **DEMO guard**: `get_account_info().await?.trade_mode.is_demo()` before any order.
3. **Prices from `quote()` / the tick stream**, specs from `symbol_params()`. Reject
   `!q.is_usable()` (zero or `bid == ask`). A freshly selected symbol can report zeros for a few
   hundred ms — `ensure_symbol()` waits for sync + a two-sided quote.
4. **Volume**: use `p.volume_min`, round to `p.volume_step` (floor, then round away from zero to
   2 decimals), clamp to `p.volume_max`.
5. **Magic**: set `with_magic(600000 + id)` on every order; read back via `positions_by_magic`.
   The broker may append text to comments (`"fleet-audit; "`) — never match on comment equality.
6. **Position ticket == order ticket** on hedging accounts (FBS confirmed). On netting accounts
   look the position up by symbol + magic instead.
7. **History is in broker server time**; use the SDK's UTC-taking methods and `to_utc_secs()`.
8. **Stream lifecycle**: `subscribe_quotes` spawns a task; dropping the receiver cancels the
   stream. The receiver closing means the stream died — resubscribe (with `reconnect()` if
   `check_connect()` fails). Keep one stream per strategy with all its symbols, not one per symbol.
9. **Never retry `InvalidLogin`**; treat it as a terminal state for the strategy.
10. **Don't `disconnect()`** on normal shutdown (the terminal is reused next start). Only
    `disconnect(true)` when decommissioning a strategy.
11. Everything else (OrderCalcMargin, OrderCheck, MarketBook, SymbolInfoDouble/Integer/String,
    OnTrade/OnTradeTransaction/OnPositionProfit streams, TradeStats) is reachable through the
    generated clients: build the request from `pb::*`, wrap with `client.request(msg, Some(timeout))`
    for the headers, and call e.g. `pb::market_info_client::MarketInfoClient::new(channel)`
    (the channel is private — add a thin method to `client.rs` next to the existing ones).
