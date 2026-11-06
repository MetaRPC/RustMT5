# Subscribe Quotes (`subscribe_quotes`)

> **Sugar method:** Live ticks for one or more symbols delivered as `Quote`s on an mpsc channel.

**API Information:**

* **Method:** `sugar.subscribe_quotes(symbols)`
* **Returns:** `Result<tokio::sync::mpsc::Receiver<Quote>, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn subscribe_quotes(
    &self,
    symbols: Vec<String>,
) -> Result<tokio::sync::mpsc::Receiver<Quote>, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbols` | `Vec<String>` | Symbols to stream. |

| Output | Type | Description |
|--------|------|-------------|
| `Receiver<Quote>` | channel | Receive `Quote`s with `.recv().await`. |

---

## 🏛️ Essentials

* **What it is:** A friendlier wrapper over the tick stream — you get a plain channel of `Quote`s instead of handling a `Stream` of protobuf frames.
* **Why you need it:** Consume live prices with a simple `while let Some(q) = rx.recv().await` loop; no `StreamExt` needed.
* **Lifecycle:** a background task feeds the channel and **ends on the first stream error or when you drop the receiver**.

---

## ⚡ Under the Hood

```text
MT5Sugar.subscribe_quotes(symbols)
  → MT5Service.stream_ticks(symbols)        // the raw tick stream
  → tokio::spawn: for each frame → tx.send(Quote::from_subscription_tick(..))
  → returns rx (mpsc::Receiver<Quote>)
```

---

## 🔗 Usage Examples

### 1) Print live quotes

```rust
let mut rx = sugar.subscribe_quotes(vec!["EURUSD".into(), "GBPUSD".into()]).await?;
while let Some(q) = rx.recv().await {
    println!("{} {:.5}/{:.5}", q.symbol, q.bid, q.ask);
}
```

### 2) Stop after N

```rust
let mut rx = sugar.subscribe_quotes(vec!["EURUSD".into()]).await?;
for _ in 0..20 {
    if let Some(q) = rx.recv().await { println!("mid {:.5}", q.mid()); }
}
// drop(rx) here ends the background task and unsubscribes
```

### 3) Feed a strategy loop

```rust
let mut rx = sugar.subscribe_quotes(vec!["EURUSD".into()]).await?;
while let Some(q) = rx.recv().await {
    if q.is_usable() && q.spread() < 0.0002 {
        // ... react to a tight-spread tick ...
    }
}
```

---

## Related Methods

* [quote](quote.md) — one-shot · [ensure_symbol](ensure_symbol.md)
* Lower level: [MT5Service::stream_ticks](../../MT5Service/6.%20Streaming_Methods.md) · [on_symbol_tick](../../MT5Account/6.%20Streaming_Methods/on_symbol_tick.md)

---

## ⚠️ Pitfalls

* **Drop the receiver to stop.** Keeping `rx` alive keeps the subscription open.
* **Channel is bounded (1024).** If you stop reading, sends block and (once the buffer fills) the task stalls — drain it or drop it.
* **Only `symbol_tick`-bearing frames** become `Quote`s; empty frames are skipped.

---

## 📚 See Also

* [quote](quote.md) · [ensure_symbol](ensure_symbol.md)
