# Symbol Select

> Add a symbol to Market Watch (or remove it) — the switch that makes quotes and history available.

**API Information:**

* **Low-level API:** `MT5Account::symbol_select(request)` (defined in [`src/mt5/account.rs`](https://github.com/MetaRPC/RustMT5/blob/main/src/mt5/account.rs))
* **gRPC service:** `mt5_term_api.MarketInfo`
* **RPC:** `SymbolSelect(SymbolSelectRequest) -> SymbolSelectReply`
* **Payload:** `SymbolSelectData` → `success: bool`

## 💬 Just the essentials

* **What it is.** MQL5 `SymbolSelect(symbol, select)` — the only **write** in this group.
* **Why you need it.** Many quote/history calls require the symbol to be in Market Watch first.
* **When to use.** Right after confirming the symbol exists, before `symbol_info_tick` / `price_history`.

---

## Method Signature

```rust
pub async fn symbol_select(
    &self,
    request: pb::SymbolSelectRequest,
) -> Result<pb::SymbolSelectData, MT5Error>
```

**Request / Reply:**

```protobuf
message SymbolSelectRequest {
  string symbol = 1;
  bool   select = 2;   // true = add to Market Watch, false = remove
}
message SymbolSelectData { bool success = 1; }
```

---

## 🔽 Input

| Param | Rust type | Description |
|-------|-----------|-------------|
| `symbol` | `String` | Symbol name. |
| `select` | `bool` | `true` → add/select; `false` → remove from Market Watch. |

## ⬆️ Output — `SymbolSelectData`

| Field | Rust type | Description |
|-------|-----------|-------------|
| `success` | `bool` | `false` on failure (e.g. removing a symbol with an open position/chart). |

---

## 🧩 Notes & Tips

* **Select before you quote.** On many servers `symbol_info_tick` / `price_history` only work for selected symbols.
* **Removal can refuse.** A symbol **can't** be removed while its chart is open or it has open positions — `success` comes back `false`.
* **Idempotent-ish.** Selecting an already-selected symbol just returns `success == true`.

---

## 🔗 Usage Examples

### 1) Ensure a symbol is ready to quote

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let ok = account
    .symbol_select(pb::SymbolSelectRequest { symbol: "EURUSD".into(), select: true })
    .await?.success;
if !ok { println!("could not add EURUSD to Market Watch"); }
let tick = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: "EURUSD".into() }).await?;
println!("bid {:.5}", tick.bid);
```

### 2) Exists → select → quote pipeline

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let sym = "GBPUSD";
if account.symbol_exist(pb::SymbolExistRequest { name: sym.into() }).await?.exists {
    account.symbol_select(pb::SymbolSelectRequest { symbol: sym.into(), select: true }).await?;
    let t = account.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: sym.into() }).await?;
    println!("{sym} {:.5}/{:.5}", t.bid, t.ask);
}
```

### 3) Tidy up — remove a symbol

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let ok = account
    .symbol_select(pb::SymbolSelectRequest { symbol: "GBPUSD".into(), select: false })
    .await?.success;
println!("removed: {ok}");  // false if a chart/position still holds it
```

---

## ⚠️ Pitfalls

* **Ignoring `success`.** A `false` means the change didn't happen — check it, don't assume.
* **Removing a busy symbol.** Open positions or charts block removal.
* **Forgetting to select.** The commonest cause of "empty tick" / "no history" errors.

---

## 🧪 Testing Suggestions

* **Add then count.** `symbol_select(sym, true)` → `symbols_total(true)` grows by 1 (if it wasn't selected).
* **Blocked removal.** With an open position on `sym`, `symbol_select(sym, false).success == false`.

---

## 📚 See also

* [symbol_exist](symbol_exist.md) · [symbol_info_tick](symbol_info_tick.md) · [price_history](price_history.md)
* **Back to:** [Symbol Information — Overview](Symbol_Information.Overview.md) · [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
