# Connection & Authentication

Managing connections with `MT5Client`.

```rust
use metarpc_mt5::MT5Client;

let mut client = MT5Client::new("demo.broker.com", 443);
client.connect(2005432, "password").await?;

if client.is_connected() {
    println!("Connected and authenticated!");
}

client.disconnect().await;
```
