# Connection & Authentication

Managing connections with `MT5Client`.

```rust
use metarpc_mt5::MT5Client;

let mut client = MT5Client::new("mt5.mrpc.pro", 443);
client.connect(2005432, "password").await?;

if client.is_connected() {
    println!("Connected and authenticated!");
}

client.disconnect().await;
```


> **Authentication Note**: Connecting to `mt5.mrpc.pro:443` requires a valid MetaRPC API key. Register for free at [https://mrpc.pro/signup](https://mrpc.pro/signup) and generate your token in [https://mrpc.pro/my](https://mrpc.pro/my).

