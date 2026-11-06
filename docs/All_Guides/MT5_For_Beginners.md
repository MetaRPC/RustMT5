# MT5 for Beginners: Your First Steps in Trading

Welcome! This guide is for complete beginners who have never worked with MetaTrader 5 or trading. We'll get you from zero to a working demo account you can automate with RustMT5.

---

## What is MetaTrader 5 (MT5)?

**MetaTrader 5** is a trading platform used by millions of traders worldwide. It lets you:

- Trade forex, stocks, commodities, and cryptocurrencies
- Analyze markets with charts and indicators
- Execute automated trading strategies
- Monitor your positions in real-time

Think of MT5 as the "cockpit" for the markets. **RustMT5** lets you control that cockpit programmatically from Rust, over gRPC.

---

## Demo vs Live Account: Start Safely

### Why start with a demo account?

A **demo account** is a risk-free practice account with virtual money. We **strongly recommend** demo first:

- ✅ **Zero risk** — virtual funds (typically $10,000 – $100,000)
- ✅ **Real market conditions** — live prices and behavior
- ✅ **Learn the platform** — before risking real money
- ✅ **Test your code** — perfect for developing RustMT5 apps

### Live Account

A **live account** uses real money. Only go live once you understand MT5, have tested thoroughly on demo, and understand the risks.

---

## Quick Start: Creating a Demo Account

The fastest way is to create a demo account directly in the MT5 terminal.

### Step 1️⃣: Download and install the MT5 terminal

1. Visit [https://www.metatrader5.com/en/download](https://www.metatrader5.com/en/download)
2. Download the version for your OS (Windows/Mac/Linux)
3. Install the application

### Step 2️⃣: Launch MT5 and find the Navigator panel

On first launch a popup usually offers to open a new account. If not, find the **Navigator** panel (left side, below **Market Watch**).

![Navigator Panel Location](../Guide_Images/1.%20NAV.png)

Inside Navigator you'll see **Accounts**, Subscriptions, Indicators, Expert Advisors, and more.

### Step 3️⃣: Open a new demo account

1. In **Navigator**, find **Accounts**
2. **Right-click** "Accounts"
3. Select **"Open an account"**

### Step 4️⃣: Select the MetaQuotes demo server

In the broker search window, look for **"MetaQuotes-Demo"** (the standard demo server). Use the search field if needed, select it, and click **Next**.

![MetaQuotes Server Selection](../Guide_Images/2.%20META.png)

### Step 5️⃣: Account type

Choose **"Open a new demo account"** and click **Next**.

![Open New Demo Account](../Guide_Images/3.%20OPEN.png)

### Step 6️⃣: Fill in your details

Name, email, account type, virtual deposit (e.g. $10,000), leverage (1:100 is fine to start), currency. Agree to the terms and click **Next**.

![Personal Information Form](../Guide_Images/4.%20creator.png)

### Step 7️⃣: Save your credentials

MT5 creates the account and shows:

- **Login** — your account number (e.g. 591129415)
- **Password** — your master (trading) password
- **Investor Password** — read-only password
- **Server** — e.g. "MetaQuotes-Demo"

**⚠️ Save these now** — you need **Login**, **Password**, and **Server** to connect from RustMT5. Click **Finish**.

![Account Credentials](../Guide_Images/5.%20OKK.png)

---

### Step 8️⃣: Connect from RustMT5

There's no config file to edit — you pass the credentials straight to `MT5Account::new` and `connect_by_server_name`, and provide your MetaRPC key via the `MRPC_API_KEY` environment variable.

```bash
export MRPC_API_KEY=your_mrpc_token    # from https://mrpc.pro/my
```

```rust
use metarpc_mt5::MT5Account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Login, password, grpc_server (None → mt5.mrpc.pro:443)
    let mut account = MT5Account::new(591_129_415, "YourPasswordHere", None);
    account.connect_by_server_name("MetaQuotes-Demo", 30).await?;  // exact server name
    let s = account.account_summary().await?;
    println!("balance {:.2} {}", s.account_balance, s.account_currency);
    account.disconnect().await?;
    Ok(())
}
```

- **Login / password** — exactly what MT5 showed you.
- **Server name** — the exact string (e.g. `"MetaQuotes-Demo"`).
- **grpc_server / API key** — `None` uses the default endpoint; the key comes from `MRPC_API_KEY`.

### Step 9️⃣: Verify the MT5 connection

In the MT5 terminal, the **bottom-right corner** should show a green connection indicator and your balance (e.g. $10,000).

### Step 🔟: Run it

```bash
cargo run
```

Trades your program makes appear in the MT5 terminal in real time. It's virtual money — experiment freely. See [Your First Project](Your_First_Project.md) for a fuller walkthrough.

---

## 🧩 MT5 password types

### Master password
Full access — open/close trades, change settings. **This is the one RustMT5 uses to trade.**

### Investor password
View-only — see trades/balance/history but cannot trade. Good for sharing performance or monitoring. Set it via **Tools → Options → Server → Change** (Investor Password).

---

## Choosing a Broker (optional)

The MetaQuotes demo server is fine for practice. For live trading later you may pick a specific broker for better conditions or regional support.

**Not financial advice — do your own research.** Popular MT5 brokers include IC Markets, Pepperstone, OANDA, XM, RoboForex, FxPro.

**What to look for:** regulation (FCA/ASIC/CySEC/…), genuine MT5 support (not just MT4), a free demo, tight spreads, responsive support.

---

## What's Next?

1. **[Getting Started](GETTING_STARTED.md)** — install the crate and make your first calls.
2. **Understand the layers:**
   - [MT5Account](../MT5Account/MT5Account.Master.Overview.md) — low-level gRPC foundation.
   - [MT5Service](../MT5Service/MT5Service.Overview.md) — typed, friendlier wrappers.
   - [MT5Sugar](../MT5Sugar/MT5Sugar.Master.Overview.md) — high-level one-liners (most beginner-friendly).
3. **[Strategies](../Strategies/Strategies.Master.Overview.md)** — ready orchestrators and presets.

---

## Important reminders

### 🛡️ Security
- Never share your **master password**; use the investor password for monitoring tools.
- Keep credentials safe (a password manager helps). Don't commit them to a repo.

### 📊 Risk management
- Even on demo, build good habits: always use a stop-loss, risk ≤ 1–2% per trade, understand position sizing (`MT5Sugar::calculate_position_size`).

### 🧪 Testing
- Test on **demo first**; verify strategies over weeks/months, not days. Demo results don't guarantee live results.

### 📚 Education
- Trading has a steep learning curve — focus on learning, not earning.

---

## Useful commands (Rust)

```bash
# add the SDK to a project
cargo add metarpc-mt5
cargo add tokio --features full

# build / run / test
cargo build
cargo run
cargo test

# run a bundled example (see examples/)
cargo run --example strategies_demo

# check your Rust toolchain
rustc --version
```

---

**Ready to code?** Head to [Getting Started](GETTING_STARTED.md).  Good luck on your trading journey! 🚀
