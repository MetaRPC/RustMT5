# Tick Value With Size

> Tick value, tick size and contract size for one or more symbols — the inputs for money-per-point P/L math.

**API Information:**

* **Low-level API:** `MT5Account::tick_value_with_size(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.AccountHelper`
* **RPC:** `TickValueWithSize(TickValueWithSizeRequest) -> TickValueWithSizeReply`
* **Payload:** `TickValueWithSizeData` → `symbol_tick_size_infos: Vec<TickSizeSymbol>`

## 💬 Just the essentials

* **What it is.** Per-symbol tick economics: how much money one tick is worth per lot, and the price step.
* **Why you need it.** To convert a price move (points) into account-currency P/L and to size positions by risk.
* **When to use.** Risk sizing and P/L estimation across several symbols at once.

> ⚠️ **Reply shape B.** Unlike most methods, this reply uses flat `data` / `error` fields (not the usual `oneof`). The SDK unwraps it the same way — you still get `Result<TickValueWithSizeData, MT5Error>`.

---

## Method Signature

```rust
pub async fn tick_value_with_size(
    &self,
    request: pb::TickValueWithSizeRequest,
) -> Result<pb::TickValueWithSizeData, MT5Error>
```

**Request / Reply:**

```protobuf
message TickValueWithSizeRequest {
  repeated string symbol_names = 1;   // one or many
}
message TickValueWithSizeReply {
  TickValueWithSizeData data = 1;     // flat (shape B)
  Error error = 2;
}
message TickValueWithSizeData {
  repeated TickSizeSymbol symbol_tick_size_infos = 1;
}
message TickSizeSymbol {
  int32  index = 1;
  string name = 2;
  double trade_tick_value = 3;
  double trade_tick_value_profit = 4;
  double trade_tick_value_loss = 5;
  double trade_tick_size = 6;
  double trade_contract_size = 7;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol_names` | `Vec<String>` | One or more symbols to query in a single call. |

## ⬆️ Output — `TickSizeSymbol` (per symbol)

| Field | Rust type | Description |
|-------|-----------|-------------|
| `index` | `i32` | Row index |
| `name` | `String` | Symbol name |
| `trade_tick_value` | `f64` | Money per tick per lot (deposit currency) |
| `trade_tick_value_profit` | `f64` | Tick value for a winning position |
| `trade_tick_value_loss` | `f64` | Tick value for a losing position |
| `trade_tick_size` | `f64` | Minimal price change |
| `trade_contract_size` | `f64` | Contract size (e.g. 100000 for FX) |

---

## 🧩 Notes & Tips

* **Money per point.** `value_per_point = trade_tick_value × (point / trade_tick_size)`. When tick size equals point, it's just `trade_tick_value` per point per lot.
* **Risk sizing.** `lots ≈ risk_money / (stop_points × value_per_point)`, then round to the symbol's volume step.
* **Batch it.** Pass several names at once — one round-trip for your whole watchlist.
* **Profit vs loss tick value** can differ for some instruments; use the relevant one for conservative sizing.

---

## 🔗 Usage Examples

### 1) Tick value for a few symbols

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let data = account.tick_value_with_size(pb::TickValueWithSizeRequest {
    symbol_names: vec!["EURUSD".into(), "GBPUSD".into(), "XAUUSD".into()],
}).await?;
for s in &data.symbol_tick_size_infos {
    println!("{}: tick value {:.5}, tick size {:.5}, contract {}",
        s.name, s.trade_tick_value, s.trade_tick_size, s.trade_contract_size);
}
```

### 2) Risk-based lot sizing

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let s = account.tick_value_with_size(pb::TickValueWithSizeRequest { symbol_names: vec!["EURUSD".into()] })
    .await?.symbol_tick_size_infos.into_iter().next().unwrap();

let point = account.symbol_info_double(pb::SymbolInfoDoubleRequest {
    symbol: "EURUSD".into(),
    r#type: pb::SymbolInfoDoubleProperty::SymbolPoint as i32,
}).await?.value;

let value_per_point = s.trade_tick_value * (point / s.trade_tick_size);
let risk_money = 50.0;      // risk $50
let stop_points = 200.0;    // 20 pips on a 5-digit symbol
let raw_lots = risk_money / (stop_points * value_per_point);
println!("raw lots ≈ {raw_lots:.2} (round to volume step before trading)");
```

### 3) Estimate P/L of a move

```rust
// A 30-point move on 0.10 lot:
let value_per_point = /* as above */ 1.0_f64;
let pl = 30.0 * value_per_point * 0.10;
println!("≈ P/L for 30 points @ 0.10 lot: {pl:+.2}");
```

---

## ⚠️ Pitfalls

* **Tick size vs point.** They're not always equal — scale by `point / trade_tick_size`, don't assume 1.
* **Unknown symbol.** A name the server doesn't know may be omitted from the result — match by `name`, don't assume positional order.
* **Round lots.** Always round the computed size to the symbol's volume step and clamp to min/max.

---

## 🧪 Testing Suggestions

* **FX major.** EURUSD `trade_contract_size == 100000`; `trade_tick_value > 0`.
* **Batch parity.** Requesting three names returns three `symbol_tick_size_infos` (for known symbols).
* **Sizing sanity.** Doubling `stop_points` halves the computed lots.

---

## 📚 See also

* [symbol_info_double](../2.%20Symbol_Information/symbol_info_double.md) · [order_calc_profit](../5.%20Trading_Operations/order_calc_profit.md)
* **Back to:** [Positions & Orders — Overview](Positions_Orders.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
