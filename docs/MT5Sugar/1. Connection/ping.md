# Ping (`ping`)

> **Sugar method:** Liveness check — `true` if the account responds.

**API Information:**

* **Method:** `sugar.ping()`
* **Returns:** `bool`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn ping(&self) -> bool
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `bool` | — | `true` if an account snapshot succeeds, else `false`. |

---

## 🏛️ Essentials

* **What it is:** Tries `account_summary()`; success → `true`.
* **Why you need it:** Confirm the connection is alive before trading or in a health loop.
* **Note:** Returns a plain `bool` (no `Result`) — a failed call is reported as `false`, not an error.

---

## ⚡ Under the Hood

```text
MT5Sugar.ping()
  → MT5Service.account_summary().is_ok()
    → MT5Account.account_summary()  (gRPC)
```

---

## When to Use

* **Health checks** / watchdog loops.
* A quick gate before a batch of operations.

---

## 🔗 Usage Examples

### 1) Basic

```rust
if sugar.ping().await { println!("alive"); } else { println!("not connected"); }
```

### 2) Guard before work

```rust
if !sugar.ping().await {
    sugar.connect_by_server_name("MetaQuotes-Demo", 30).await?;
}
```

### 3) Periodic check

```rust
use std::time::Duration;
loop {
    println!("alive: {}", sugar.ping().await);
    tokio::time::sleep(Duration::from_secs(10)).await;
}
```

---

## Related Methods

* [connect_by_server_name](connect_by_server_name.md) · [disconnect](disconnect.md)
* [get_account_info](../2.%20Account_Properties/get_account_info.md)

---

## ⚠️ Pitfalls

* **Swallows the error.** `ping` returns `false` on any failure — use `account_summary()` directly if you need the error detail.
* **One round-trip.** It's cheap but not free; don't spin it in a tight loop.

---

## 📚 See Also

* [connect_by_server_name](connect_by_server_name.md) · [disconnect](disconnect.md)
