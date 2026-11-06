# Order History

> Historical orders and their deals within a time range — paged.

**API Information:**

* **Low-level API:** `MT5Account::order_history(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.AccountHelper`
* **RPC:** `OrderHistory(OrderHistoryRequest) -> OrderHistoryReply`
* **Payload:** `OrdersHistoryData` → paging + `history_data: Vec<HistoryData>`

## 💬 Just the essentials

* **What it is.** MQL5 history of orders and deals in `[from, to]`.
* **Why you need it.** Audit what happened: fills, closes, commissions, realized profit.
* **When to use.** Statements, trade journals, realized-P/L reports. For closed **positions** specifically, [`positions_history`](./positions_history.md) is often simpler.

---

## Method Signature

```rust
pub async fn order_history(
    &self,
    request: pb::OrderHistoryRequest,
) -> Result<pb::OrdersHistoryData, MT5Error>
```

**Request message:**

```protobuf
message OrderHistoryRequest {
  google.protobuf.Timestamp input_from = 1;   // required (server time)
  google.protobuf.Timestamp input_to = 2;     // required (server time)
  Bmt5EnumOrderHistorySortType input_sort_mode = 3;
  int32 page_number = 4;       // default 0
  int32 items_per_page = 5;    // default 0 (server default)
}
```

**Reply payload:**

```protobuf
message OrdersHistoryData {
  int32 array_total = 1;       // total matching rows
  int32 page_number = 2;
  int32 items_per_page = 3;
  repeated HistoryData history_data = 4;
}

message HistoryData {
  uint32 index = 1;
  OrderHistoryData history_order = 2;  // optional
  DealHistoryData  history_deal = 3;   // optional
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `input_from` | `Option<Timestamp>` | Range start (server time). Required. |
| `input_to` | `Option<Timestamp>` | Range end (server time). Required. |
| `input_sort_mode` | `i32` (`Bmt5EnumOrderHistorySortType`) | Sort order (see enum). |
| `page_number` | `i32` | 0-based page. |
| `items_per_page` | `i32` | Page size (0 = server default). |

### `Bmt5EnumOrderHistorySortType`

| Variant | Value |
|---------|-------|
| `Bmt5SortByOpenTimeAsc` / `...Desc` | 0 / 1 |
| `Bmt5SortByCloseTimeAsc` / `...Desc` | 2 / 3 |
| `Bmt5SortByOrderTicketIdAsc` / `...Desc` | 4 / 5 |

## ⬆️ Output — `OrdersHistoryData`

`array_total` is the total matching rows (for paging); each `HistoryData` holds an optional **order** and/or **deal**.

### `OrderHistoryData` (key fields)

| Field | Rust type | Description |
|-------|-----------|-------------|
| `ticket` | `u64` | Order ticket |
| `symbol` | `String` | Symbol |
| `state` | `i32` (`Bmt5EnumOrderState`) | Final order state |
| `r#type` | `i32` (`Bmt5EnumOrderType`) | Order type |
| `price_open` / `price_current` | `f64` | Prices |
| `stop_loss` / `take_profit` | `f64` | SL / TP |
| `volume_initial` / `volume_current` | `f64` | Lots |
| `setup_time` / `done_time` | `Option<Timestamp>` | Placed / finished |
| `position_id` | `u64` | Owning position |
| `magic_number` | `i64` | EA magic |

### `DealHistoryData` (key fields)

| Field | Rust type | Description |
|-------|-----------|-------------|
| `ticket` | `u64` | Deal ticket |
| `symbol` | `String` | Symbol |
| `profit` | `f64` | Realized profit |
| `commission` / `fee` / `swap` | `f64` | Costs |
| `price` | `f64` | Fill price |
| `volume` | `f64` | Lots |
| `entry_type` | `i32` (`Bmt5EnumDealEntryType`) | In / out / inout |
| `r#type` | `i32` (`Bmt5EnumDealType`) | Deal type |
| `time` | `Option<Timestamp>` | Fill time |
| `position_id` | `u64` | Owning position |

---

## 🧩 Notes & Tips

* **Orders vs deals.** An order is the instruction; a deal is the execution that realized money. Realized P/L lives on **deals** (`DealHistoryData.profit`).
* **Both optional per row.** Check `history_order.is_some()` / `history_deal.is_some()` before unwrapping.
* **Server time.** `from`/`to` are server time — mind the broker's UTC offset (`account_summary().utc_timezone_server_time_shift_minutes`).
* **Page with `array_total`.** Loop pages until you've read `array_total` rows.

---

## 🔗 Usage Examples

### 1) Last 7 days, first page

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::Bmt5EnumOrderHistorySortType as Sort;

fn ts(secs: i64) -> prost_types::Timestamp { prost_types::Timestamp { seconds: secs, nanos: 0 } }

let now = chrono::Utc::now().timestamp();
let data = account.order_history(pb::OrderHistoryRequest {
    input_from: Some(ts(now - 7 * 86_400)),
    input_to: Some(ts(now)),
    input_sort_mode: Sort::Bmt5SortByCloseTimeDesc as i32,
    page_number: 0,
    items_per_page: 50,
}).await?;
println!("{} total rows", data.array_total);
```

### 2) Realized P/L from deals

```rust
let realized: f64 = data.history_data.iter()
    .filter_map(|h| h.history_deal.as_ref())
    .map(|d| d.profit + d.commission + d.swap + d.fee)
    .sum();
println!("realized (net): {realized:+.2}");
```

### 3) Page through everything

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::Bmt5EnumOrderHistorySortType as Sort;

let (from, to) = (ts(now - 30 * 86_400), ts(now));
let mut page = 0; let mut seen = 0;
loop {
    let d = account.order_history(pb::OrderHistoryRequest {
        input_from: Some(from.clone()), input_to: Some(to.clone()),
        input_sort_mode: Sort::Bmt5SortByOpenTimeAsc as i32,
        page_number: page, items_per_page: 100,
    }).await?;
    seen += d.history_data.len() as i32;
    if d.history_data.is_empty() || seen >= d.array_total { break; }
    page += 1;
}
```

---

## ⚠️ Pitfalls

* **Missing `from`/`to`.** Both are required; `None` yields nothing useful.
* **Unwrapping the wrong half.** A row may carry only an order or only a deal.
* **Server vs local time.** Off-by-offset ranges silently miss rows.
* **Profit on orders.** Orders don't carry realized P/L — read it from deals.

---

## 🧪 Testing Suggestions

* **Known trade.** After closing a position, its deal appears with the expected `profit`.
* **Paging completeness.** Summed page lengths reach `array_total`.

---

## 📚 See also

* [positions_history](positions_history.md) · [opened_orders](opened_orders.md)
* **Back to:** [Positions & Orders — Overview](Positions_Orders.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
