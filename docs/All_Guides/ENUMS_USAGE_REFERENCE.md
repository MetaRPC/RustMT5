# ENUMs Usage Reference

> How protobuf enums work across the RustMT5 layers — passing them in, the `r#type` fields, decoding them back, and a catalog of the ones you'll actually use.

All enums live under `metarpc_mt5::proto::mt5_term_api` (aliased `pb` throughout the docs).

---

## The three rules

### 1. Low-level (`MT5Account`): cast the enum to `i32`

The generated request structs store enum fields as `i32`. Pass `Enum::Variant as i32`:

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::SymbolInfoDoubleProperty as D;

let point = account.symbol_info_double(pb::SymbolInfoDoubleRequest {
    symbol: "EURUSD".into(),
    r#type: D::SymbolPoint as i32,   // ← cast
}).await?.value;
```

### 2. Mid/high-level (`MT5Service` / `MT5Sugar`): pass the typed enum

The facade takes the enum directly and casts for you:

```rust
use metarpc_mt5::proto::mt5_term_api::SymbolInfoDoubleProperty as D;

let point = service.symbol_double("EURUSD", D::SymbolPoint).await?;  // ← typed, no `as i32`
```

### 3. Decoding an enum you received: `try_from`

Reply fields are also `i32`. Convert back with `Enum::try_from(value)`:

```rust
use metarpc_mt5::proto::mt5_term_api::MrpcEnumAccountTradeMode as Mode;

let s = account.account_summary().await?;
match Mode::try_from(s.account_trade_mode) {
    Ok(Mode::MrpcAccountTradeModeReal) => println!("REAL"),
    Ok(Mode::MrpcAccountTradeModeContest) => println!("CONTEST"),
    _ => println!("DEMO"),
}
```

---

## ⚠️ The `r#type` field

Several request/reply structs have a field literally named `type` — a Rust keyword — so prost emits it as the raw identifier **`r#type`**. Write it as-is in struct literals and field access:

```rust
pb::SymbolInfoDoubleRequest { symbol: "EURUSD".into(), r#type: D::SymbolPoint as i32 }
//                                                      ^^^^^^
let side = position.r#type;  // Bmt5EnumPositionType: 0 Buy, 1 Sell
```

Structs with `r#type`: `SymbolInfoDoubleRequest` / `…IntegerRequest` / `…StringRequest`, `PositionInfo`, `OpenedOrderInfo`, `OrderHistoryData`, `DealHistoryData`, `MrpcMqlBookInfo`, and the streaming `On*Data` event structs.

> `account_info_*` requests use `property_id` (not `r#type`); `symbol_info_margin_rate` uses `order_type`.

---

## Enum catalog by group

### Account Information

| Enum | Used by | Key variants (value) |
|------|---------|----------------------|
| `AccountInfoDoublePropertyType` | `account_info_double` / `account_double` | `AccountBalance`(0), `AccountProfit`(2), `AccountEquity`(3), `AccountMargin`(4), `AccountMarginFree`(5), `AccountMarginLevel`(6) |
| `AccountInfoIntegerPropertyType` | `account_info_integer` / `account_integer` | `AccountLogin`(0), `AccountTradeMode`(1), `AccountLeverage`(2), `AccountTradeAllowed`(5), `AccountTradeExpert`(6) |
| `AccountInfoStringPropertyType` | `account_info_string` / `account_string` | `AccountName`(0), `AccountServer`(1), `AccountCurrency`(2), `AccountCompany`(3) |
| `MrpcEnumAccountTradeMode` | decode `account_trade_mode` | `…Demo`(0), `…Contest`(1), `…Real`(2) |

### Symbol Information

| Enum | Used by | Notes |
|------|---------|-------|
| `SymbolInfoDoubleProperty` | `symbol_info_double` / `symbol_double` | ~40 variants: `SymbolBid`(0), `SymbolAsk`(3), `SymbolPoint`(13), `SymbolTradeTickValue`(14), `SymbolTradeTickSize`(17), `SymbolVolumeMin/Max/Step`(22/23/24), … |
| `SymbolInfoIntegerProperty` | `symbol_info_integer` / `symbol_integer` | `SymbolDigits`(17), `SymbolSpread`(19), `SymbolTradeStopsLevel`(25), `SymbolTradeMode`(22), … |
| `SymbolInfoStringProperty` | `symbol_info_string` / `symbol_string` | `SymbolCurrencyBase`(5), `SymbolCurrencyProfit`(6), `SymbolDescription`(9), `SymbolPath`(14) |
| `EnumOrderType` | `symbol_info_margin_rate` / `symbol_margin_rate` | `OrderTypeBuy`(0), `OrderTypeSell`(1), … |
| `DayOfWeek` | session methods | `Sunday`(0) … `Saturday`(6) |
| `AhSymbolParamsManySortType` | `symbol_params_many` (sort) | name/MQL-index × asc/desc (0–3) |

### Positions & Orders

| Enum | Used by | Key variants |
|------|---------|--------------|
| `Bmt5EnumOpenedOrderSortType` | `opened_orders` | open-time / ticket × asc/desc (0–3) |
| `Bmt5EnumOrderHistorySortType` | `order_history` | open/close/ticket × asc/desc (0–5) |
| `AhEnumPositionsHistorySortType` | `positions_history` | open-time / ticket × asc/desc (0–3) |
| `Bmt5EnumPositionType` | decode `PositionInfo.r#type` | `…Buy`(0), `…Sell`(1) |

### Market Depth

| Enum | Used by | Variants |
|------|---------|----------|
| `BookType` | decode `MrpcMqlBookInfo.r#type` | `Sell`(0), `Buy`(1), `SellMarket`(2), `BuyMarket`(3) |

### Trading Operations

| Enum | Used by | Variants |
|------|---------|----------|
| `Tmt5EnumOrderType` | `order_send.operation` | `…Buy`(0), `…Sell`(1), `…BuyLimit`(2), `…SellLimit`(3), `…BuyStop`(4), `…SellStop`(5), `…BuyStopLimit`(6), `…SellStopLimit`(7) |
| `EnumOrderTypeTf` | `order_calc_margin/profit`, `calculate_margin/profit` | `OrderTypeTfBuy`(0), `OrderTypeTfSell`(1), … |
| `Tmt5EnumOrderTypeTime` | `order_send`/`order_modify` expiration | GTC / DAY / SPECIFIED / SPECIFIED_DAY |
| `MrpcEnumTradeRequestActions` | `order_check` request | `TradeActionDeal`, … |

---

## Patterns

### A tiny decode helper

```rust
use metarpc_mt5::proto::mt5_term_api::Bmt5EnumPositionType as Side;

fn side_str(p: &metarpc_mt5::proto::mt5_term_api::PositionInfo) -> &'static str {
    match Side::try_from(p.r#type) {
        Ok(Side::Bmt5PositionTypeSell) => "SELL",
        _ => "BUY",
    }
}
```

### Don't hardcode magic integers

```rust
// ✗ brittle
r#type: 13,
// ✓ self-documenting
r#type: pb::SymbolInfoDoubleProperty::SymbolPoint as i32,
```

---

## 📚 See also

- [Getting Started](GETTING_STARTED.md) · [Return Codes](RETURN_CODES_REFERENCE.md)
- Low-level pages list each enum's full variant table — e.g. [symbol_info_double](../MT5Account/2.%20Symbol_Information/symbol_info_double.md), [order_send](../MT5Account/5.%20Trading_Operations/order_send.md).
