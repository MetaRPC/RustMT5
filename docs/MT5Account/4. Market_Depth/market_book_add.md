# Market Book Add

> Subscribe to a symbol's Depth of Market — the first step before reading the order book.

**API Information:**

* **Low-level API:** `MT5Account::market_book_add(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `MarketBookAdd(MarketBookAddRequest) -> MarketBookAddReply`
* **Payload:** `MarketBookAddData` → `opened_successfully: bool`

## 💬 Just the essentials

* **What it is.** MQL5 `MarketBookAdd(symbol)` — opens the DOM feed for a symbol.
* **Why you need it.** [`market_book_get`](./market_book_get.md) only returns data after a successful add.
* **When to use.** Once, at the start of a DOM session; pair with [`market_book_release`](./market_book_release.md).

---

## Method Signature

```rust
pub async fn market_book_add(
    &self,
    request: pb::MarketBookAddRequest,
) -> Result<pb::MarketBookAddData, MT5Error>
```

**Request / Reply:**

```protobuf
message MarketBookAddRequest { string symbol = 1; }
message MarketBookAddData    { bool opened_successfully = 1; }
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol whose DOM to subscribe to. |

## ⬆️ Output — `MarketBookAddData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `opened_successfully` | `bool` | `true` if the subscription opened. |

---

## 🧩 Notes & Tips

* **Select first.** Ensure the symbol is in Market Watch ([`symbol_select`](../2.%20Symbol_Information/symbol_select.md)) before subscribing.
* **One add per symbol.** Re-adding an already-subscribed symbol is harmless but unnecessary.
* **Always release.** Balance every successful add with a `market_book_release` to free the feed.

---

## 🔗 Usage Examples

### 1) Subscribe and check

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let ok = account.market_book_add(pb::MarketBookAddRequest { symbol: "EURUSD".into() })
    .await?.opened_successfully;
if !ok { println!("DOM not available for this symbol"); }
```

### 2) Full lifecycle

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

account.market_book_add(pb::MarketBookAddRequest { symbol: "EURUSD".into() }).await?;
let book = account.market_book_get(pb::MarketBookGetRequest { symbol: "EURUSD".into() }).await?;
println!("{} levels", book.mql_book_infos.len());
account.market_book_release(pb::MarketBookReleaseRequest { symbol: "EURUSD".into() }).await?;
```

### 3) Guard before polling

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

if account.market_book_add(pb::MarketBookAddRequest { symbol: "XAUUSD".into() }).await?.opened_successfully {
    // safe to loop market_book_get now
}
```

---

## ⚠️ Pitfalls

* **Reading before adding.** `market_book_get` without a successful add yields nothing.
* **Leaking subscriptions.** Forgetting `market_book_release` keeps the feed open.
* **FX may have no DOM.** `opened_successfully` can be `false` on feeds without depth.

---

## 🧪 Testing Suggestions

* **Supported symbol.** `opened_successfully == true`, then `market_book_get` returns levels.
* **Idempotent-ish.** Adding twice still returns `true`.

---

## 📚 See also

* [market_book_get](market_book_get.md) · [market_book_release](market_book_release.md) · [symbol_select](../2.%20Symbol_Information/symbol_select.md)
* **Back to:** [Market Depth — Overview](Market_Depth.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
