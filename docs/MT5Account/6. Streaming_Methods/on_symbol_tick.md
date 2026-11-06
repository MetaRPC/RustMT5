# On Symbol Tick (stream)

> Live tick updates for one or more symbols — the push alternative to polling `symbol_info_tick`.

**API Information:**

* **Low-level API:** `MT5Account::on_symbol_tick(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.SubscriptionService`
* **RPC:** `OnSymbolTick(OnSymbolTickRequest) -> stream OnSymbolTickReply`
* **Item payload:** `OnSymbolTickData`

## 💬 Just the essentials

* **What it is.** A server-push stream of ticks for the symbols you subscribe to.
* **Why you need it.** React to every price change without a polling loop.
* **When to use.** Live dashboards, tick-driven strategies. For a one-off quote use [`symbol_info_tick`](../2.%20Symbol_Information/symbol_info_tick.md).

---

## Method Signature

```rust
pub async fn on_symbol_tick(
    &self,
    request: pb::OnSymbolTickRequest,
) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnSymbolTickData, MT5Error>>, MT5Error>
```

**Request / item:**

```protobuf
message OnSymbolTickRequest { repeated string symbol_names = 1; }

message OnSymbolTickData {
  MrpcSubscriptionMqlTick symbol_tick = 1;   // optional
  string terminal_instance_guid_id = 2;
}

message MrpcSubscriptionMqlTick {
  google.protobuf.Timestamp time = 1;
  double bid = 2; double ask = 3; double last = 4;
  uint64 volume = 5; int64 time_msc = 6; uint32 flags = 7;
  double volume_real = 8; string symbol = 9;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol_names` | `Vec<String>` | One or more symbols to stream. |

## ⬆️ Item — `OnSymbolTickData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `symbol_tick` | `Option<MrpcSubscriptionMqlTick>` | The tick (bid/ask/last/volume/time + `symbol`). |
| `terminal_instance_guid_id` | `String` | Originating terminal session. |

`MrpcSubscriptionMqlTick` mirrors [`MrpcMqlTick`](../2.%20Symbol_Information/symbol_info_tick.md) but `time` is a `Timestamp` and it adds the `symbol` field (handy when streaming many).

---

## 🧩 Notes & Tips

* **Select the symbols first** so the feed is live.
* **`symbol_tick` is `Option`.** Guard before using.
* **`symbol` disambiguates.** With multiple subscriptions, route by `symbol_tick.symbol`.
* **Drop to unsubscribe.** Ending the loop / dropping the stream stops the feed.

---

## 🔗 Usage Examples

### 1) Stream EURUSD ticks

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_symbol_tick(pb::OnSymbolTickRequest { symbol_names: vec!["EURUSD".into()] }).await?;
while let Some(item) = s.next().await {
    match item {
        Ok(d) => if let Some(t) = d.symbol_tick {
            println!("{} {:.5}/{:.5}", t.symbol, t.bid, t.ask);
        },
        Err(e) => { eprintln!("stream error: {e}"); break; }
    }
}
```

### 2) Several symbols, routed by name

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_symbol_tick(pb::OnSymbolTickRequest {
    symbol_names: vec!["EURUSD".into(), "GBPUSD".into()],
}).await?;
while let Some(Ok(d)) = s.next().await {
    if let Some(t) = d.symbol_tick {
        println!("[{}] mid {:.5}", t.symbol, (t.bid + t.ask) / 2.0);
    }
}
```

### 3) Stop after N ticks

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_symbol_tick(pb::OnSymbolTickRequest { symbol_names: vec!["EURUSD".into()] }).await?;
let mut n = 0;
while let Some(Ok(d)) = s.next().await {
    if d.symbol_tick.is_some() { n += 1; }
    if n >= 10 { break; }   // dropping `s` unsubscribes
}
```

---

## ⚠️ Pitfalls

* **Forgetting `use tokio_stream::StreamExt;`.** `.next()` won't resolve otherwise.
* **Unwrapping `symbol_tick`.** It's an `Option`.
* **Blocking the runtime.** Do heavy work off the stream task (e.g. `tokio::spawn`).

---

## 🧪 Testing Suggestions

* **During market hours.** Frames arrive steadily; `bid`/`ask` populated.
* **Closed market.** Few/no frames — your loop just waits.

---

## 📚 See also

* [symbol_info_tick](../2.%20Symbol_Information/symbol_info_tick.md) · [on_position_profit](on_position_profit.md)
* **Back to:** [Streaming Methods — Overview](Streaming_Methods.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
