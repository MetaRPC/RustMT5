# Symbol Name

> The symbol name at a given index — the companion to [`symbols_total`](./symbols_total.md) for enumerating symbols.

**API Information:**

* **Low-level API:** `MT5Account::symbol_name(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolName(SymbolNameRequest) -> SymbolNameReply`
* **Payload:** `SymbolNameData` → `name: String`

## 💬 Just the essentials

* **What it is.** MQL5 `SymbolName(index, selected)` — name by position.
* **Why you need it.** To iterate the symbol list without fetching every property.
* **When to use.** Building a symbol picker; usually prefer [`symbol_list`](./symbol_list.md) for the whole list in one call.

---

## Method Signature

```rust
pub async fn symbol_name(
    &self,
    request: pb::SymbolNameRequest,
) -> Result<pb::SymbolNameData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolNameRequest {
  int32 index = 1;     // ordinal position
  bool  selected = 2;  // true = index into Market Watch list; false = full list
}
message SymbolNameData { string name = 1; }
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `index` | `i32` | Zero-based position in the chosen list. |
| `selected` | `bool` | `true` → Market Watch list; `false` → full symbol list. |

## ⬆️ Output — `SymbolNameData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `name` | `String` | The symbol name at that index. |

---

## 🧩 Notes & Tips

* **Bound it with `symbols_total`.** Valid `index` range is `0..symbols_total(selected).total`.
* **Keep `selected` consistent.** The index means different things for the Market-Watch list vs the full list.
* **One call per index.** For the whole set, [`symbol_list`](./symbol_list.md) is a single round-trip.

---

## 🔗 Usage Examples

### 1) Name at an index

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let name = account.symbol_name(pb::SymbolNameRequest { index: 0, selected: true }).await?.name;
println!("first Market-Watch symbol: {name}");
```

### 2) Full enumeration

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let total = account.symbols_total(pb::SymbolsTotalRequest { mode: false }).await?.total;
for i in 0..total {
    let name = account.symbol_name(pb::SymbolNameRequest { index: i, selected: false }).await?.name;
    if name.starts_with("EUR") { println!("{name}"); }
}
```

### 3) Index → name map for the Market Watch

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use std::collections::BTreeMap;

let n = account.symbols_total(pb::SymbolsTotalRequest { mode: true }).await?.total;
let mut map = BTreeMap::new();
for i in 0..n {
    let name = account.symbol_name(pb::SymbolNameRequest { index: i, selected: true }).await?.name;
    map.insert(i, name);
}
println!("{map:#?}");
```

---

## ⚠️ Pitfalls

* **Out-of-range index.** `index >= total` is an error — always bound with `symbols_total` first.
* **`selected` mismatch.** Indexing the full list while reading counts for Market Watch gives the wrong names.
* **N calls = N round-trips.** For the full list prefer `symbol_list`.

---

## 🧪 Testing Suggestions

* **Round-trip.** `symbol_name(0, true)` returns a name that `symbol_exist` confirms `exists`.
* **Range.** `symbol_name(total-1, false)` succeeds; `symbol_name(total, false)` errors.

---

## 📚 See also

* [symbols_total](symbols_total.md) · [symbol_list](symbol_list.md) · [symbol_exist](symbol_exist.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
