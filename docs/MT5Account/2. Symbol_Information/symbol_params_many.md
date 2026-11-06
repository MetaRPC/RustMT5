# Symbol Params Many (RECOMMENDED for bulk)

> Full parameter sets for many symbols (or one) in a single, paged call — every property without a call per field.

**API Information:**

* **Low-level API:** `MT5Account::symbol_params_many(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.AccountHelper`
* **RPC:** `SymbolParamsMany(SymbolParamsManyRequest) -> SymbolParamsManyReply`
* **Payload:** `SymbolParamsManyData` → `symbol_infos: Vec<SymbolParameters>`, paging fields

## 💬 Just the essentials

* **What it is.** A bulk fetch of the whole `SymbolParameters` record for each symbol.
* **Why you need it.** One round-trip replaces dozens of `symbol_info_double/integer/string` calls.
* **When to use.** Caching symbol specs at startup, or reading one symbol's complete spec at once.

---

## Method Signature

```rust
pub async fn symbol_params_many(
    &self,
    request: pb::SymbolParamsManyRequest,
) -> Result<pb::SymbolParamsManyData, MT5Error>
```

**Request message:**

```protobuf
message SymbolParamsManyRequest {
  optional string symbol_name = 1;                 // one symbol; omit for all
  optional AhSymbolParamsManySortType sort_type = 2;
  optional int32 page_number = 3;                  // 0-based
  optional int32 items_per_page = 4;
}
message SymbolParamsManyData {
  repeated SymbolParameters symbol_infos = 1;
  int32 symbols_total = 2;
  optional int32 page_number = 3;
  optional int32 items_per_page = 4;
}
```

---

## 🔽 Input (all optional)

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol_name` | `Option<String>` | A single symbol; `None` → all symbols. |
| `sort_type` | `Option<i32>` (`AhSymbolParamsManySortType`) | Sort order (see enum). |
| `page_number` | `Option<i32>` | 0-based page index for paging. |
| `items_per_page` | `Option<i32>` | Page size. |

## ⬆️ Output — `SymbolParamsManyData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `symbol_infos` | `Vec<SymbolParameters>` | One full record per symbol on this page. |
| `symbols_total` | `i32` | Total matching symbols (across all pages). |
| `page_number` | `Option<i32>` | Echoed page index. |
| `items_per_page` | `Option<i32>` | Echoed page size. |

### `SymbolParameters` — a flat record of the symbol's spec

It carries the full property set in one struct (same values as the `symbol_info_*` calls). Commonly used fields:

| Field | Rust type | Description |
|-------|-----------|-------------|
| `name` | `String` | Symbol name |
| `bid` / `ask` / `last` | `f64` | Current prices |
| `point` | `f64` | Point size |
| `volume_real` | `f64` | Last volume |
| … | | plus digits, spread, contract size, volume min/max/step, swaps, margins, currencies, description, path and more |

> The struct mirrors MQL5's symbol properties; read the field you need directly instead of issuing a `symbol_info_*` call per property.

---

## 🧱 `AhSymbolParamsManySortType` (from proto)

| Variant | Value | Order |
|---------|-------|-------|
| `AhParamsManySortTypeSymbolNameAsc` | 0 | By name, ascending |
| `AhParamsManySortTypeSymbolNameDesc` | 1 | By name, descending |
| `AhParamsManySortTypeMqlIndexAsc` | 2 | By MQL index, ascending |
| `AhParamsManySortTypeMqlIndexDesc` | 3 | By MQL index, descending |

---

## 🧩 Notes & Tips

* **One symbol, full spec.** Pass `symbol_name: Some("EURUSD")` to get a single complete record — far cheaper than many single-property calls.
* **Page large universes.** With `None`, set `items_per_page` and loop `page_number` until you've read `symbols_total`.
* **Cache it.** Specs change rarely; read once at startup and keep the records.

---

## 🔗 Usage Examples

### 1) One symbol's complete spec

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let data = account.symbol_params_many(pb::SymbolParamsManyRequest {
    symbol_name: Some("EURUSD".into()),
    sort_type: None, page_number: None, items_per_page: None,
}).await?;
if let Some(p) = data.symbol_infos.first() {
    println!("{}: bid {:.5} ask {:.5} point {}", p.name, p.bid, p.ask, p.point);
}
```

### 2) First page, sorted by name

```rust
use metarpc_mt5::proto::mt5_term_api as pb;
use pb::AhSymbolParamsManySortType as Sort;

let data = account.symbol_params_many(pb::SymbolParamsManyRequest {
    symbol_name: None,
    sort_type: Some(Sort::AhParamsManySortTypeSymbolNameAsc as i32),
    page_number: Some(0),
    items_per_page: Some(50),
}).await?;
println!("{} of {} total", data.symbol_infos.len(), data.symbols_total);
for p in data.symbol_infos.iter().take(5) { println!("{}", p.name); }
```

### 3) Page through everything

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let page_size = 100;
let mut page = 0;
let mut seen = 0;
loop {
    let d = account.symbol_params_many(pb::SymbolParamsManyRequest {
        symbol_name: None, sort_type: None,
        page_number: Some(page), items_per_page: Some(page_size),
    }).await?;
    seen += d.symbol_infos.len() as i32;
    if d.symbol_infos.is_empty() || seen >= d.symbols_total { break; }
    page += 1;
}
println!("read {seen} symbol records");
```

---

## ⚠️ Pitfalls

* **Options are `Option<..>`.** Wrap values (`Some(..)`), use `None` to omit.
* **Forgetting to page.** With `None` and no page size you may get only the first page — check `symbols_total`.
* **Over-fetching.** Pulling every record each tick is wasteful; cache.

---

## 🧪 Testing Suggestions

* **Single-symbol parity.** `symbol_infos[0].point` equals `symbol_info_double(SymbolPoint)`.
* **Paging completeness.** Summed page lengths reach `symbols_total`.

---

## 📚 See also

* [symbol_info_double](symbol_info_double.md) · [symbol_info_integer](symbol_info_integer.md) · [symbol_list](symbol_list.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
