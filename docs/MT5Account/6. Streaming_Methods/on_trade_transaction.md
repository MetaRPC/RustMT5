# On Trade Transaction (stream)

> The raw MT5 `OnTradeTransaction` feed — each transaction with its originating request and result.

**API Information:**

* **Low-level API:** `MT5Account::on_trade_transaction(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.SubscriptionService`
* **RPC:** `OnTradeTransaction(OnTradeTransactionRequest) -> stream`
* **Item payload:** `OnTradeTransactionData`

## 💬 Just the essentials

* **What it is.** The lowest-level trade feed: MT5's `MqlTradeTransaction` plus the `MqlTradeRequest` and `MqlTradeResult` that produced it.
* **Why you need it.** Precise, step-by-step audit of order lifecycle (add/update/delete/deal-add) that [`on_trade`](./on_trade.md) summarizes.
* **When to use.** Detailed execution tracking, reconciliation, debugging order flow.

---

## Method Signature

```rust
pub async fn on_trade_transaction(
    &self,
    request: pb::OnTradeTransactionRequest,
) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnTradeTransactionData, MT5Error>>, MT5Error>
```

**Request / item:**

```protobuf
message OnTradeTransactionRequest {}   // no fields

message OnTradeTransactionData {
  Mt5SubEnumEventGroupType type = 1;         // Rust field: r#type
  MqlTradeTransaction trade_transaction = 2; // optional — the transaction
  MqlTradeRequest     trade_request = 3;     // optional — the request behind it
  MqlTradeResult      trade_result = 4;      // optional — the result
  string …guid… = 5;
}
```

---

## 🔽 Input

None — pass `pb::OnTradeTransactionRequest {}`.

## ⬆️ Item — `OnTradeTransactionData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `r#type` | `i32` (`Mt5SubEnumEventGroupType`) | Event group. |
| `trade_transaction` | `Option<MqlTradeTransaction>` | The transaction (type: order add/update/delete, deal add, …). |
| `trade_request` | `Option<MqlTradeRequest>` | The request that caused it (when applicable). |
| `trade_result` | `Option<MqlTradeResult>` | The server result (retcode etc.). |

Transaction kinds come from `SubEnumTradeTransactionType` — e.g. `OrderAdd` (0), `OrderUpdate` (1), `OrderDelete` (2), `DealAdd` (3), `DealUpdate` (4), …

---

## 🧱 Enums

**`Mt5SubEnumEventGroupType`** (field `r#type`): `OrderProfit` = 0, `OrderUpdate` = 1.

**`SubEnumTradeTransactionType`** — the transaction kind inside `trade_transaction`:

| Variant | Value | | Variant | Value |
|---------|-------|---|---------|-------|
| `SubTradeTransactionOrderAdd` | 0 | | `SubTradeTransactionDealDelete` | 5 |
| `SubTradeTransactionOrderUpdate` | 1 | | `SubTradeTransactionHistoryAdd` | 6 |
| `SubTradeTransactionOrderDelete` | 2 | | `SubTradeTransactionHistoryUpdate` | 7 |
| `SubTradeTransactionDealAdd` | 3 | | `SubTradeTransactionHistoryDelete` | 8 |
| `SubTradeTransactionDealUpdate` | 4 | | `SubTradeTransactionPosition` | 9 |
| | | | `SubTradeTransactionRequest` | 10 |

---

## 🧩 Notes & Tips

* **More granular than `on_trade`.** You see each sub-step (order placed → partially filled → deal added), not just "something changed".
* **All three sub-messages are `Option`.** Many transactions carry only the transaction; `request`/`result` appear for operations you initiated.
* **`r#type` is a raw identifier.**
* **Prefer `on_trade`** if you only need "refresh my view" — this feed is noisier.

---

## 🔗 Usage Examples

### 1) Log every transaction

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_trade_transaction(pb::OnTradeTransactionRequest {}).await?;
while let Some(item) = s.next().await {
    match item {
        Ok(d) => {
            if let Some(tx) = &d.trade_transaction { let _ = tx; println!("transaction (group {})", d.r#type); }
            if let Some(r)  = &d.trade_result      { let _ = r;  println!("  result attached"); }
        }
        Err(e) => { eprintln!("stream error: {e}"); break; }
    }
}
```

### 2) React only to deal additions

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_trade_transaction(pb::OnTradeTransactionRequest {}).await?;
while let Some(Ok(d)) = s.next().await {
    if d.trade_transaction.is_some() {
        // inspect the transaction's own type field (SubEnumTradeTransactionType) to filter DealAdd
        println!("tx event, group {}", d.r#type);
    }
}
```

### 3) Capture results of your own requests

```rust
use tokio_stream::StreamExt;
use metarpc_mt5::proto::mt5_term_api as pb;

let mut s = account.on_trade_transaction(pb::OnTradeTransactionRequest {}).await?;
while let Some(Ok(d)) = s.next().await {
    if let (Some(_req), Some(_res)) = (&d.trade_request, &d.trade_result) {
        println!("request → result pair received");
    }
}
```

---

## ⚠️ Pitfalls

* **Noise.** This feed is detailed; filter by transaction type rather than reacting to everything.
* **Unwrapping `Option`s.** Each sub-message may be absent.
* **Not a snapshot.** It reports transitions; keep your own state if you need current totals.

---

## 🧪 Testing Suggestions

* **Send an order.** You see an order-add transaction, then a deal-add on fill.
* **Modify SL/TP.** An order-update transaction arrives.

---

## 📚 See also

* [on_trade](on_trade.md) · [order_send](../5.%20Trading_Operations/order_send.md)
* **Back to:** [Streaming Methods — Overview](Streaming_Methods.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
