# Order Calc Profit

> The profit a hypothetical open→close move would produce — in account currency.

**API Information:**

* **Low-level API:** `MT5Account::order_calc_profit(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.TradeFunctions`
* **RPC:** `OrderCalcProfit(OrderCalcProfitRequest) -> OrderCalcProfitReply`
* **Payload:** `OrderCalcProfitData` → `profit: f64`

## 💬 Just the essentials

* **What it is.** MQL5 `OrderCalcProfit` — profit for `volume` moving from `open_price` to `close_price`.
* **Why you need it.** Turn a target/stop distance into money; evaluate risk:reward before trading.
* **When to use.** Planning TP/SL, backtest-style what-ifs. Handles cross-currency conversion for you.

---

## Method Signature

```rust
pub async fn order_calc_profit(
    &self,
    request: pb::OrderCalcProfitRequest,
) -> Result<pb::OrderCalcProfitData, MT5Error>
```

**Request / Reply:**

```protobuf
message OrderCalcProfitRequest {
  EnumOrderTypeTf order_type = 1;   // Rust field: order_type (i32)
  string symbol = 2;
  double volume = 3;                // lots
  double open_price = 4;
  double close_price = 5;
}
message OrderCalcProfitData { double profit = 1; }
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `order_type` | `i32` (`EnumOrderTypeTf`) | Buy or Sell (direction matters for the sign). |
| `symbol` | `String` | Symbol. |
| `volume` | `f64` | Lots. |
| `open_price` | `f64` | Assumed entry. |
| `close_price` | `f64` | Assumed exit. |

## ⬆️ Output — `OrderCalcProfitData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `profit` | `f64` | Profit (negative = loss) in the deposit currency. |

---

## 🧱 `EnumOrderTypeTf` (field `order_type`)

| Variant | Value | | Variant | Value |
|---------|-------|---|---------|-------|
| `OrderTypeTfBuy` | 0 | | `OrderTypeTfSellStop` | 5 |
| `OrderTypeTfSell` | 1 | | `OrderTypeTfBuyStopLimit` | 6 |
| `OrderTypeTfBuyLimit` | 2 | | `OrderTypeTfSellStopLimit` | 7 |
| `OrderTypeTfSellLimit` | 3 | | `OrderTypeTfCloseBy` | 8 |
| `OrderTypeTfBuyStop` | 4 | | | |

> For a profit estimate you'll almost always use `OrderTypeTfBuy` / `OrderTypeTfSell` — the direction sets the sign.

---

## 🧩 Notes & Tips

* **Direction sets the sign.** For `Buy`, `close > open` is profit; for `Sell`, it's the opposite — use the correct `order_type`.
* **Cross-currency handled.** The server converts to account currency, so you don't need the symbol's profit-currency rate.
* **Risk:reward.** Compute profit at your TP and the (negative) profit at your SL; the ratio is your R:R.

---

## 🔗 Usage Examples

### 1) Profit of a 50-pip buy move

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::EnumOrderTypeTf as TF;

let open = 1.08500;
let p = account.order_calc_profit(pb::OrderCalcProfitRequest {
    order_type: TF::OrderTypeTfBuy as i32,
    symbol: "EURUSD".into(),
    volume: 0.10,
    open_price: open,
    close_price: open + 0.0050,   // +50 pips
}).await?.profit;
println!("profit: {p:+.2}");
```

### 2) Risk:reward from SL/TP

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::EnumOrderTypeTf as TF;

let (entry, sl, tp) = (1.08500, 1.08300, 1.09000);
let calc = |close: f64| {
    let acc = &account;
    async move {
        acc.order_calc_profit(pb::OrderCalcProfitRequest {
            order_type: TF::OrderTypeTfBuy as i32, symbol: "EURUSD".into(),
            volume: 0.10, open_price: entry, close_price: close,
        }).await.map(|d| d.profit)
    }
};
let reward = calc(tp).await?;
let risk = calc(sl).await?;          // negative
println!("R:R ≈ {:.2}", reward / risk.abs());
```

### 3) Profit of a sell

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::EnumOrderTypeTf as TF;

let p = account.order_calc_profit(pb::OrderCalcProfitRequest {
    order_type: TF::OrderTypeTfSell as i32, symbol: "USDJPY".into(),
    volume: 0.10, open_price: 156.00, close_price: 155.50,   // price fell → sell profits
}).await?.profit;
println!("sell profit: {p:+.2}");
```

---

## ⚠️ Pitfalls

* **Wrong `order_type`.** Using Buy for a short flips the sign.
* **Enum as `i32`.**
* **Spread/commission not included.** This is gross price-move profit, not net of costs — subtract commission/swap separately (see `tick_value_with_size` / history).

---

## 🧪 Testing Suggestions

* **Sign check.** Buy with `close > open` → positive; `close < open` → negative.
* **Scales with volume.** Double the lots → double the profit.

---

## 📚 See also

* [order_calc_margin](order_calc_margin.md) · [tick_value_with_size](../3.%20Positions_Orders/tick_value_with_size.md) · [order_send](order_send.md)
* **Back to:** [Trading Operations — Overview](Trading_Operations.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
