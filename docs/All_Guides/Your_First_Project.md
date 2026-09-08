# Your First Project in 10 Minutes (Rust)

> **Hands-on Quick Start** - Create a working trading project with MetaTrader 5 and RustMT5 from scratch.

---

## Step 1: Create Your Project

Create a new directory for your trading bot:

```bash
mkdir my_rustmt5_bot
cd my_rustmt5_bot
```

Install the package:

```bash
cargo add metarpc-mt5
```

---

## Step 2: Write Your Trading Code

Create your main application file and paste the following snippet:

```
use metarpc_mt5::MT5Account;

let mut account = MT5Account::new(user, password, grpc_server);
account.connect_by_server_name(server_name, "EURUSD", 30).await?;
let summary = account.account_summary().await?;
println!("Balance: {}, Equity: {}", summary.account_balance, summary.account_equity);
```

---

## Step 3: Run the Program

Run your application:

```bash
# Verify connection output
# Balance: 10000.00, Equity: 10000.00
```

---

## 🚀 Next Steps

Congratulations! You have successfully established a direct gRPC connection to MetaTrader 5. Next:
- Explore **[gRPC Streaming](GRPC_STREAM_MANAGEMENT.md)** to listen to live ticks.
- Check the **[API Reference](../API_Reference/MT5Account.md)** for all 40+ available terminal methods.
- Learn about high-level risk management and auto-normalization in **[MT5Sugar](../API_Reference/MT5Sugar.md)**.
