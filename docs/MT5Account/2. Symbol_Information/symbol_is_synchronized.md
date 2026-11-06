# Symbol Is Synchronized

> Is the symbol's data in sync with the trade server? Check before trusting prices or history.

**API Information:**

* **Low-level API:** `MT5Account::symbol_is_synchronized(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolIsSynchronized(SymbolIsSynchronizedRequest) -> SymbolIsSynchronizedReply`
* **Payload:** `SymbolIsSynchronizedData` → `synchronized: bool`

## 💬 Just the essentials

* **What it is.** MQL5 `SymbolIsSynchronized(symbol)` — a readiness flag.
* **Why you need it.** Just-selected symbols may not have their specs/history loaded yet; acting too early gives stale or empty data.
* **When to use.** After `symbol_select`, poll this until `true` before reading history or trading.

---

## Method Signature

```rust
pub async fn symbol_is_synchronized(
    &self,
    request: pb::SymbolIsSynchronizedRequest,
) -> Result<pb::SymbolIsSynchronizedData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolIsSynchronizedRequest { string symbol = 1; }
message SymbolIsSynchronizedData    { bool synchronized = 1; }
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol name. |

## ⬆️ Output — `SymbolIsSynchronizedData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `synchronized` | `bool` | `true` once the symbol's data matches the server. |

---

## 🧩 Notes & Tips

* **Select first.** This is meaningful only after the symbol is in Market Watch.
* **Poll with a timeout.** Sync usually completes in well under a second; loop with a short sleep and a deadline rather than forever.

---

## 🔗 Usage Examples

### 1) Wait for sync after selecting

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use std::time::{Duration, Instant};

account.symbol_select(pb::SymbolSelectRequest { symbol: "EURUSD".into(), select: true }).await?;

let deadline = Instant::now() + Duration::from_secs(5);
loop {
    let s = account
        .symbol_is_synchronized(pb::SymbolIsSynchronizedRequest { symbol: "EURUSD".into() })
        .await?.synchronized;
    if s { break; }
    if Instant::now() >= deadline { println!("[WARN] not synchronized in time"); break; }
    tokio::time::sleep(Duration::from_millis(200)).await;
}
```

### 2) Guard a history read

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

if account.symbol_is_synchronized(pb::SymbolIsSynchronizedRequest { symbol: "EURUSD".into() }).await?.synchronized {
    // safe to call price_history now
}
```

### 3) Quick boolean helper

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

async fn synced(account: &metarpc_mt5::MT5Account, sym: &str) -> bool {
    account.symbol_is_synchronized(pb::SymbolIsSynchronizedRequest { symbol: sym.into() })
        .await.map(|d| d.synchronized).unwrap_or(false)
}
```

---

## ⚠️ Pitfalls

* **Polling forever.** Always bound the wait with a deadline.
* **Reading history before sync.** You may get empty or partial bars — check this first.

---

## 🧪 Testing Suggestions

* **After select.** On a live demo, `synchronized` becomes `true` within a second or two.
* **Unselected symbol.** May report `false` (or error) — select it first.

---

## 📚 See also

* [symbol_select](symbol_select.md) · [price_history](price_history.md) · [symbol_info_tick](symbol_info_tick.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
