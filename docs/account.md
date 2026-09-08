# Account Information

```rust
let acc = client.get_account_info().await?;
println!("Login:        {}", acc.login);
println!("Balance:      {}", acc.balance);
println!("Equity:       {}", acc.equity);
println!("Free Margin:  {}", acc.free_margin);
println!("Margin Level: {}%", acc.margin_level);
println!("Leverage:     1:{}", acc.leverage);
```
