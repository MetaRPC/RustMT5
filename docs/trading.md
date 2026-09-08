# Trading & Orders

Placing, modifying, and closing orders.

```rust
// Open Order
let order = client.order_send(OrderRequest {
    symbol: "EURUSD".into(),
    action: TradeAction::Buy,
    volume: 0.1,
    price: None,
    stop_loss: Some(1.0800),
    take_profit: Some(1.0950),
    slippage: Some(10),
    comment: Some("Algo Trade".into()),
}).await?;

// Modify SL/TP
client.order_modify(order.ticket, 1.0820, 1.0960).await?;

// Close Order
client.order_close(order.ticket, 0.1).await?;
```
