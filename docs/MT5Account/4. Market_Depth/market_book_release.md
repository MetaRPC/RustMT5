# Market Book Release

> Unsubscribe from a symbol's Depth of Market — the cleanup step.

**API Information:**

* **Low-level API:** `MT5Account::market_book_release(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `MarketBookRelease(MarketBookReleaseRequest) -> MarketBookReleaseReply`
* **Payload:** `MarketBookReleaseData` → `closed_successfully: bool`

## 💬 Just the essentials

* **What it is.** MQL5 `MarketBookRelease(symbol)` — closes the DOM feed.
* **Why you need it.** Frees server/terminal resources once you're done reading the book.
* **When to use.** At the end of a DOM session — one release per [`market_book_add`](./market_book_add.md).

---

## Method Signature

```rust
pub async fn market_book_release(
    &self,
    request: pb::MarketBookReleaseRequest,
) -> Result<pb::MarketBookReleaseData, MT5Error>
```

**Request / Reply:**

```protobuf
message MarketBookReleaseRequest { string symbol = 1; }
message MarketBookReleaseData    { bool closed_successfully = 1; }
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol to unsubscribe. |

## ⬆️ Output — `MarketBookReleaseData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `closed_successfully` | `bool` | `true` if the subscription closed. |

---

## 🧩 Notes & Tips

* **Pair with add.** Every successful `market_book_add` should get a matching release.
* **Idempotent-ish.** Releasing a symbol that isn't subscribed typically returns `false` — safe to ignore.
* **Do it in cleanup paths.** Release on error paths too, so a failed read doesn't leak the feed.

---

## 🔗 Usage Examples

### 1) Release when done

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let ok = account.market_book_release(pb::MarketBookReleaseRequest { symbol: "EURUSD".into() })
    .await?.closed_successfully;
println!("released: {ok}");
```

### 2) RAII-style guard

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

account.market_book_add(pb::MarketBookAddRequest { symbol: "EURUSD".into() }).await?;
let result = (|| async {
    let book = account.market_book_get(pb::MarketBookGetRequest { symbol: "EURUSD".into() }).await?;
    Ok::<_, metarpc_mt5::MT5Error>(book.mql_book_infos.len())
})().await;
// always release, even if the read failed
let _ = account.market_book_release(pb::MarketBookReleaseRequest { symbol: "EURUSD".into() }).await;
println!("levels: {result:?}");
```

### 3) Release several symbols

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

for sym in ["EURUSD", "GBPUSD"] {
    let _ = account.market_book_release(pb::MarketBookReleaseRequest { symbol: sym.into() }).await;
}
```

---

## ⚠️ Pitfalls

* **Forgetting to release.** Leaves the DOM feed open on the terminal.
* **Releasing without adding.** Returns `false`; not harmful, just a no-op.

---

## 🧪 Testing Suggestions

* **After add.** `closed_successfully == true`.
* **Double release.** Second call returns `false`.

---

## 📚 See also

* [market_book_add](market_book_add.md) · [market_book_get](market_book_get.md)
* **Back to:** [Market Depth — Overview](Market_Depth.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
