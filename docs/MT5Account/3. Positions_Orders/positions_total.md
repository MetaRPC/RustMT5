# Positions Total

> How many positions are open right now — a single integer.

**API Information:**

* **Low-level API:** `MT5Account::positions_total()` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.TradeFunctions`
* **RPC:** `PositionsTotal(PositionsTotalRequest) -> PositionsTotalReply`
* **Payload:** `PositionsTotalData` → `total_positions: i32`

## 💬 Just the essentials

* **What it is.** MQL5 `PositionsTotal()` — the count of open positions.
* **Why you need it.** A cheap "do I have exposure?" check before heavier calls.
* **When to use.** Guards and dashboards. For details use [`opened_orders`](./opened_orders.md).

---

## Method Signature

```rust
pub async fn positions_total(&self) -> Result<pb::PositionsTotalData, MT5Error>
```

> Takes **no argument** — the underlying request is the unit `()`, so there's no `…Request` struct to build.

**Reply payload:**

```protobuf
message PositionsTotalData { int32 total_positions = 1; }
```

---

## 🔽 Input

None.

## ⬆️ Output — `PositionsTotalData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `total_positions` | `i32` | Number of currently open positions. |

---

## 🧩 Notes & Tips

* **Positions, not orders.** Pending orders are not counted here — see `opened_orders().opened_orders` for those.
* **Cheapest exposure check.** Prefer this over fetching all positions just to test for zero.

---

## 🔗 Usage Examples

### 1) Quick guard

```rust
let n = account.positions_total().await?.total_positions;
if n == 0 { println!("flat — no open positions"); }
else { println!("{n} open position(s)"); }
```

### 2) Gate a "close all" routine

```rust
if account.positions_total().await?.total_positions > 0 {
    // ... fetch opened_orders and close them
}
```

### 3) Poll exposure

```rust
use std::time::Duration;
for _ in 0..5 {
    println!("positions: {}", account.positions_total().await?.total_positions);
    tokio::time::sleep(Duration::from_secs(2)).await;
}
```

---

## ⚠️ Pitfalls

* **Don't pass a request.** This method takes no args; there is no `PositionsTotalRequest` to construct.
* **Count ≠ volume.** Two positions could net to zero exposure; the count doesn't tell you direction or size.

---

## 🧪 Testing Suggestions

* **Flat account.** Returns `0`.
* **After opening one position.** Returns `1`; matches `opened_orders().position_infos.len()`.

---

## 📚 See also

* [opened_orders](opened_orders.md) · [opened_orders_tickets](opened_orders_tickets.md)
* **Back to:** [Positions & Orders — Overview](Positions_Orders.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
