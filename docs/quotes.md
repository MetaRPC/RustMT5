# Quotes & Market Data

Receiving tick streams with Tokio channels:

```rust
let mut quote_rx = client.subscribe_quotes(vec!["EURUSD".to_string()]).await?;

while let Some(quote) = quote_rx.recv().await {
    println!("[{}] Bid: {} | Ask: {}", quote.symbol, quote.bid, quote.ask);
}
```
