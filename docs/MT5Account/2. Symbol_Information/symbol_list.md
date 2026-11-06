# Symbol List

> The full list of symbol names in one call — no index walking.

**API Information:**

* **Low-level API:** `MT5Account::symbol_list(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolList(SymbolListRequest) -> SymbolListReply`
* **Payload:** `SymbolListData` → `symbols: Vec<String>`

## 💬 Just the essentials

* **What it is.** All symbol names at once.
* **Why you need it.** One round-trip instead of `symbols_total` + N × `symbol_name`.
* **When to use.** Populating a picker, searching names, or snapshotting the universe.

---

## Method Signature

```rust
pub async fn symbol_list(
    &self,
    request: pb::SymbolListRequest,
) -> Result<pb::SymbolListData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolListRequest {}                       // no fields
message SymbolListData { repeated string symbols = 1; }
```

---

## 🔽 Input

None — `SymbolListRequest` is empty. Pass `pb::SymbolListRequest {}`.

## ⬆️ Output — `SymbolListData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `symbols` | `Vec<String>` | Every symbol name known to the terminal. |

---

## 🧩 Notes & Tips

* **One call beats the loop.** Prefer this over `symbols_total` + `symbol_name` when you want the whole list.
* **Client-side filtering.** Filter/sort the returned `Vec` in Rust (`.iter().filter(..)`).
* **Can be large.** Thousands of entries on some brokers — don't print them all to a UI unfiltered.

---

## 🔗 Usage Examples

### 1) Get and count

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let list = account.symbol_list(pb::SymbolListRequest {}).await?.symbols;
println!("{} symbols", list.len());
```

### 2) Filter FX majors

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let list = account.symbol_list(pb::SymbolListRequest {}).await?.symbols;
let majors: Vec<_> = list.into_iter()
    .filter(|s| ["EURUSD", "GBPUSD", "USDJPY", "USDCHF"].iter().any(|m| s.starts_with(m)))
    .collect();
println!("{majors:?}");
```

### 3) Does a name exist (client-side)?

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let list = account.symbol_list(pb::SymbolListRequest {}).await?.symbols;
let has = list.iter().any(|s| s == "XAUUSD");
println!("XAUUSD present: {has}");
```

---

## ⚠️ Pitfalls

* **Existence vs visibility.** The list includes symbols not in Market Watch — still `symbol_select` before quoting.
* **Memory.** The `Vec` can be big; filter early.

---

## 🧪 Testing Suggestions

* **Length vs total.** `symbol_list().symbols.len()` ≈ `symbols_total(false).total`.
* **Membership.** A name from the list passes `symbol_exist`.

---

## 📚 See also

* [symbols_total](symbols_total.md) · [symbol_name](symbol_name.md) · [symbol_params_many](symbol_params_many.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
