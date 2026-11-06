# Symbols Total

> How many symbols the terminal knows — all of them, or only those in Market Watch.

**API Information:**

* **Low-level API:** `MT5Account::symbols_total(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolsTotal(SymbolsTotalRequest) -> SymbolsTotalReply`
* **Payload:** `SymbolsTotalData` → `total: i32`

## 💬 Just the essentials

* **What it is.** MQL5 `SymbolsTotal(selected)` — a count.
* **Why you need it.** The upper bound for walking symbols by index with [`symbol_name`](./symbol_name.md).
* **When to use.** Enumerating the symbol universe, or showing "N symbols in Market Watch".

---

## Method Signature

```rust
pub async fn symbols_total(
    &self,
    request: pb::SymbolsTotalRequest,
) -> Result<pb::SymbolsTotalData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolsTotalRequest { bool mode = 1; }   // true = only Market Watch
message SymbolsTotalData    { int32 total = 1; }
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `mode` | `bool` | `true` → count only symbols selected in Market Watch; `false` → count **all** symbols. |

## ⬆️ Output — `SymbolsTotalData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `total` | `i32` | Number of symbols. |

---

## 🧩 Notes & Tips

* **Pairs with `symbol_name`.** Loop `index` in `0..total` and call [`symbol_name`](./symbol_name.md) with the same `selected`/`mode` meaning.
* **`mode` selects the universe.** Market-Watch count is usually far smaller than the full broker list.

---

## 🔗 Usage Examples

### 1) Count both universes

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let all = account.symbols_total(pb::SymbolsTotalRequest { mode: false }).await?.total;
let watch = account.symbols_total(pb::SymbolsTotalRequest { mode: true }).await?.total;
println!("{all} symbols total, {watch} in Market Watch");
```

### 2) Walk every Market-Watch symbol

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let n = account.symbols_total(pb::SymbolsTotalRequest { mode: true }).await?.total;
for i in 0..n {
    let name = account
        .symbol_name(pb::SymbolNameRequest { index: i, selected: true })
        .await?.name;
    println!("[{i}] {name}");
}
```

### 3) Guard a later index lookup

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let n = account.symbols_total(pb::SymbolsTotalRequest { mode: false }).await?.total;
let idx = 5.min(n - 1).max(0);
let name = account.symbol_name(pb::SymbolNameRequest { index: idx, selected: false }).await?.name;
println!("symbol #{idx} = {name}");
```

---

## ⚠️ Pitfalls

* **`selected` must match.** Use the same universe here and in `symbol_name` — mixing them indexes a different list.
* **`total` can be large.** The full broker list may be thousands; prefer `mode: true` for UI.

---

## 🧪 Testing Suggestions

* **Market Watch ≤ all.** `symbols_total(true).total <= symbols_total(false).total`.
* **After `symbol_select`.** Selecting a new symbol increments the Market-Watch count by 1.

---

## 📚 See also

* [symbol_name](symbol_name.md) · [symbol_list](symbol_list.md) · [symbol_select](symbol_select.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
