# Market Book Get

> A snapshot of the current order book: price levels with side and volume.

**API Information:**

* **Low-level API:** `MT5Account::market_book_get(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `MarketBookGet(MarketBookGetRequest) -> MarketBookGetReply`
* **Payload:** `MarketBookGetData` → `mql_book_infos: Vec<MrpcMqlBookInfo>`

## 💬 Just the essentials

* **What it is.** MQL5 `MarketBookGet(symbol, book[])` — the current depth levels.
* **Why you need it.** See resting liquidity above and below the market — bids vs offers and their sizes.
* **When to use.** After a successful [`market_book_add`](./market_book_add.md); call repeatedly to refresh.

---

## Method Signature

```rust
pub async fn market_book_get(
    &self,
    request: pb::MarketBookGetRequest,
) -> Result<pb::MarketBookGetData, MT5Error>
```

**Request / Reply:**

```protobuf
message MarketBookGetRequest { string symbol = 1; }
message MarketBookGetData { repeated MrpcMqlBookInfo mql_book_infos = 1; }

message MrpcMqlBookInfo {
  BookType type = 1;        // Rust field: r#type (i32)
  double price = 2;
  int64  volume = 3;
  double volume_real = 4;
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Subscribed symbol. |

## ⬆️ Output — `MrpcMqlBookInfo` (per level)

| Field | Rust type | Description |
|-------|-----------|-------------|
| `r#type` | `i32` (`BookType`) | Side/kind of the level (see enum). |
| `price` | `f64` | Price of the level. |
| `volume` | `i64` | Volume at the level. |
| `volume_real` | `f64` | Higher-accuracy volume. |

### `BookType`

| Variant | Value | Meaning |
|---------|-------|---------|
| `Sell` | 0 | Sell order (offer / ask side) |
| `Buy` | 1 | Buy order (bid side) |
| `SellMarket` | 2 | Sell at market |
| `BuyMarket` | 3 | Buy at market |

---

## 🧩 Notes & Tips

* **Two sides in one vector.** Filter by `r#type` to split bids (`Buy`) from offers (`Sell`).
* **Best bid / best ask.** Highest `Buy` price and lowest `Sell` price; their gap is the book spread.
* **Snapshot.** Each call is a point-in-time copy; poll to refresh.
* **`r#type` is a raw identifier** (Rust keyword).

---

## 🔗 Usage Examples

### 1) Print the book

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::BookType;

let book = account.market_book_get(pb::MarketBookGetRequest { symbol: "EURUSD".into() }).await?;
for lvl in &book.mql_book_infos {
    let side = match BookType::try_from(lvl.r#type) {
        Ok(BookType::Buy) | Ok(BookType::BuyMarket) => "BID",
        _ => "ASK",
    };
    println!("{side} {:.5} x {}", lvl.price, lvl.volume);
}
```

### 2) Best bid / best ask

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::BookType;

let book = account.market_book_get(pb::MarketBookGetRequest { symbol: "EURUSD".into() }).await?;
let best_bid = book.mql_book_infos.iter()
    .filter(|l| matches!(BookType::try_from(l.r#type), Ok(BookType::Buy)))
    .map(|l| l.price).fold(f64::MIN, f64::max);
let best_ask = book.mql_book_infos.iter()
    .filter(|l| matches!(BookType::try_from(l.r#type), Ok(BookType::Sell)))
    .map(|l| l.price).fold(f64::MAX, f64::min);
println!("book spread: {:.5}", best_ask - best_bid);
```

### 3) Total depth per side

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::BookType;

let book = account.market_book_get(pb::MarketBookGetRequest { symbol: "EURUSD".into() }).await?;
let (bid_vol, ask_vol) = book.mql_book_infos.iter().fold((0i64, 0i64), |(b, a), l| {
    match BookType::try_from(l.r#type) {
        Ok(BookType::Buy) => (b + l.volume, a),
        Ok(BookType::Sell) => (b, a + l.volume),
        _ => (b, a),
    }
});
println!("bid depth {bid_vol}, ask depth {ask_vol}");
```

---

## ⚠️ Pitfalls

* **Empty book.** Means no DOM for this symbol or you didn't `market_book_add` first.
* **`r#type as i32` / `try_from`.** Decode the side with `BookType::try_from(l.r#type)`.
* **Assuming order.** Don't assume levels are pre-sorted — sort by `price` if you need rank.

---

## 🧪 Testing Suggestions

* **Subscribed exchange symbol.** Returns multiple levels; best ask ≥ best bid.
* **Unsubscribed.** Empty vector — add first.

---

## 📚 See also

* [market_book_add](market_book_add.md) · [market_book_release](market_book_release.md)
* **Back to:** [Market Depth — Overview](Market_Depth.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
