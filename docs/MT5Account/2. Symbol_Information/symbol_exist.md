# Symbol Exist

> Does a symbol name exist on this server — and is it a custom (synthetic) symbol?

**API Information:**

* **Low-level API:** `MT5Account::symbol_exist(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolExist(SymbolExistRequest) -> SymbolExistReply`
* **Payload:** `SymbolExistData` → `exists: bool`, `is_custom: bool`

## 💬 Just the essentials

* **What it is.** A name check against the broker's standard and custom symbols.
* **Why you need it.** Validate user input before you try to select, quote, or trade a symbol.
* **When to use.** First step of "can I trade `X`?" — then [`symbol_select`](./symbol_select.md) → [`symbol_info_tick`](./symbol_info_tick.md).

---

## Method Signature

```rust
pub async fn symbol_exist(
    &self,
    request: pb::SymbolExistRequest,
) -> Result<pb::SymbolExistData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolExistRequest { string name = 1; }
message SymbolExistData {
  bool exists = 1;     // found among standard or custom symbols
  bool is_custom = 2;  // the found symbol is custom (synthetic)
}
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `name` | `String` | Exact symbol name to check (case-sensitive on most servers). |

## ⬆️ Output — `SymbolExistData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `exists` | `bool` | `true` if the symbol exists (standard **or** custom). |
| `is_custom` | `bool` | `true` if the existing symbol is a custom/synthetic one. |

---

## 🧩 Notes & Tips

* **Existence ≠ visibility.** A symbol can exist but not be in Market Watch — call [`symbol_select`](./symbol_select.md) before quoting it.
* **Names are broker-specific.** `EURUSD` on one server may be `EURUSD.m` or `EURUSD-5` on another; check first.

---

## 🔗 Usage Examples

### 1) Validate before trading

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let e = account.symbol_exist(pb::SymbolExistRequest { name: "EURUSD".into() }).await?;
if !e.exists {
    println!("no such symbol — check the broker's naming");
    return Ok(());
}
println!("exists{}", if e.is_custom { " (custom)" } else { "" });
```

### 2) Resolve a preferred name with fallbacks

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

async fn pick(account: &metarpc_mt5::MT5Account, names: &[&str]) -> Option<String> {
    for n in names {
        if account.symbol_exist(pb::SymbolExistRequest { name: (*n).into() }).await.ok()?.exists {
            return Some((*n).to_string());
        }
    }
    None
}
let sym = pick(&account, &["EURUSD", "EURUSD.m", "EURUSD-5"]).await;
println!("using {sym:?}");
```

### 3) Flag custom symbols

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let e = account.symbol_exist(pb::SymbolExistRequest { name: "MyIndex".into() }).await?;
if e.exists && e.is_custom { println!("MyIndex is synthetic — prices may be derived"); }
```

---

## ⚠️ Pitfalls

* **Case & suffix sensitivity.** `eurusd` may not match `EURUSD`; brokers add suffixes.
* **`exists == false` is not an error.** It's a valid answer — don't treat it as a failure to retry.

---

## 🧪 Testing Suggestions

* **Known-good.** A symbol you can see in the terminal returns `exists == true`.
* **Nonsense name.** `"NOPE123"` returns `exists == false`, `is_custom == false`.

---

## 📚 See also

* [symbol_select](symbol_select.md) · [symbol_info_tick](symbol_info_tick.md) · [symbol_list](symbol_list.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
