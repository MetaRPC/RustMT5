# Synchronous vs Asynchronous Methods - When to Use What (Rust)

> Understanding execution models in RustMT5: non-blocking streaming vs synchronous execution.

---

## 🎯 Quick Comparison

Rust SDK runs on Tokio async runtime, utilizing `tokio_stream::Stream` for zero-allocation quote and tick dispatch.

| Aspect | Asynchronous Pattern | Synchronous Call |
|--------|----------------------|------------------|
| **Thread Blocking** | ❌ Non-blocking (high concurrency) | ✅ Blocks current thread |
| **Throughput** | ✅ Handles thousands of events/sec | ❌ Limited by thread pool |
| **Real-Time Data** | ✅ Perfect for tick & trade streams | ⚠️ Inefficient for streams |
| **Simplicity** | Requires async runtime awareness | Simple, linear execution |
| **Recommended for** | Automated bots, microservices, GUIs | CLI scripts, notebooks, one-offs |

---

## 🚀 When to Use Asynchronous Methods (Recommended)

### 1. Market Data Streaming
Market ticks arrive at microsecond intervals during peak sessions. Asynchronous handlers ensure zero tick drops without freezing your execution thread:

```
let mut account = MT5Account::new(user, password, grpc_server);
account.connect_by_server_name(server_name, "EURUSD", 30).await?;
let summary = account.account_summary().await?;
println!("Balance: {}, Equity: {}", summary.account_balance, summary.account_equity);
```

### 2. High-Frequency Order Execution
When operating across multiple currency pairs simultaneously, asynchronous dispatch allows your bot to send orders concurrently rather than sequentially.

---

## 💡 Summary

Always prefer asynchronous paradigms for production bots, multi-symbol trading, and background services. Use synchronous wrappers for quick setup scripts, testing, or exploratory analysis.
