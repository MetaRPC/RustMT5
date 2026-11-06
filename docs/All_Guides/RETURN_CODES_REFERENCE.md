# MT5 Return Codes (RetCodes) — Reference

> How the RustMT5 SDK surfaces MetaTrader 5 trade-server return codes, and what each code means.

---

## 🛡️ What is a RetCode?

A **RetCode** is the status the MT5 trade server returns after a **trading** operation. It tells you whether the trade succeeded or why it was rejected (no money, invalid stops, market closed, …).

RetCodes appear only on **trading** calls — `order_send` / `order_modify` / `order_close` (and their Service/Sugar wrappers) — because only those go through the broker and can be rejected. Read-only calls either return data or fail with an [`MT5Error`](#the-two-layers-of-error).

---

## The two layers of error

A trading call returns `Result<…Data, MT5Error>`. There are **two** things to check:

1. **Did the request reach the server?** `Err(MT5Error)` = transport/gateway failure (not connected, timeout, status). Handle it with `?` or a match.
2. **Did the trade succeed?** On `Ok(data)`, inspect `data.returned_code`. **`10009` (`TRADE_RETCODE_DONE`) = success.** Anything else is a rejection or partial.

```rust
use metarpc_mt5::proto::mt5_term_api as pb;

let res = account.order_send(req).await?;          // ← layer 1: MT5Error on transport failure
if res.returned_code == 10009 {                     // ← layer 2: trade result
    println!("done — order #{}, deal #{}", res.order, res.deal);
} else {
    eprintln!("rejected {}: {}", res.returned_code, res.returned_code_description);
}
```

### Result fields that carry the code

`OrderSendData` / `OrderModifyData` / `OrderCloseData` expose:

| Field | Type | Meaning |
|-------|------|---------|
| `returned_code` | `u32` | The numeric RetCode (`10009` = done). |
| `returned_string_code` | `String` | The symbolic name (e.g. `"TRADE_RETCODE_NO_MONEY"`). |
| `returned_code_description` | `String` | Human-readable description from the server. |

### The `TRADE_RETCODE_DONE` constant

`MT5Sugar` exposes it, and its boolean helpers use it internally:

```rust
use metarpc_mt5::sugar::TRADE_RETCODE_DONE; // = 10009

let ok = sugar.close_position(ticket).await?;  // true  ⇔ returned_code == TRADE_RETCODE_DONE
```

When a Sugar order helper sees a non-`10009` code it raises `MT5Error::Api { code, message, mql_code }` — so with Sugar you can rely on `?` and treat any `Ok` as a completed trade.

---

## ✅ Success & in-flight codes

| Code | Name | Meaning |
|------|------|---------|
| 10008 | `PLACED` | Order placed (pending accepted). |
| **10009** | **`DONE`** | **Request completed — success.** |
| 10010 | `DONE_PARTIAL` | Only part of the volume filled. |
| 10004 | `REQUOTE` | Price changed — requote offered. |
| 10006 | `REJECT` | Request rejected. |
| 10007 | `CANCEL` | Cancelled by trader. |

> Treat **10010 (partial)** deliberately: the position/volume may differ from what you asked for.

---

## ⚠️ Common rejection codes

| Code | Name | Typical cause / fix |
|------|------|---------------------|
| 10013 | `INVALID` | Malformed request — check required fields. |
| 10014 | `INVALID_VOLUME` | Volume off the symbol's min/step/max — round it. |
| 10015 | `INVALID_PRICE` | Bad price (e.g. pending on the wrong side of market). |
| 10016 | `INVALID_STOPS` | SL/TP inside the stops/freeze level — widen them. |
| 10017 | `TRADE_DISABLED` | Trading disabled for the account/symbol. |
| 10018 | `MARKET_CLOSED` | Out of trade session — check `symbol_info_session_trade`. |
| 10019 | `NO_MONEY` | Not enough free margin — size down / check margin. |
| 10020 | `PRICE_CHANGED` | Price moved past deviation — retry or widen slippage. |
| 10021 | `PRICE_OFF` | No quotes to process the request. |
| 10022 | `INVALID_EXPIRATION` | Bad pending-order expiration. |
| 10024 | `TOO_MANY_REQUESTS` | Throttled — back off. |
| 10025 | `NO_CHANGES` | Modify requested no actual change. |
| 10027 | `CLIENT_DISABLES_AT` | Autotrading disabled in the client. |
| 10030 | `INVALID_FILL` | Unsupported filling mode for the symbol. |
| 10031 | `CONNECTION` | No connection to the trade server. |
| 10034 | `LIMIT_VOLUME` | Would exceed the volume limit for the symbol. |
| 10036 | `POSITION_CLOSED` | The position is already closed. |
| 10040 | `LIMIT_POSITIONS` | Max number of open positions reached. |
| 10045 | `FIFO_CLOSE` | Account requires FIFO close order. |

<details>
<summary>Full numeric range (10004–10045)</summary>

`10004 REQUOTE · 10006 REJECT · 10007 CANCEL · 10008 PLACED · 10009 DONE · 10010 DONE_PARTIAL · 10011 ERROR · 10012 TIMEOUT · 10013 INVALID · 10014 INVALID_VOLUME · 10015 INVALID_PRICE · 10016 INVALID_STOPS · 10017 TRADE_DISABLED · 10018 MARKET_CLOSED · 10019 NO_MONEY · 10020 PRICE_CHANGED · 10021 PRICE_OFF · 10022 INVALID_EXPIRATION · 10023 ORDER_CHANGED · 10024 TOO_MANY_REQUESTS · 10025 NO_CHANGES · 10026 SERVER_DISABLES_AT · 10027 CLIENT_DISABLES_AT · 10028 LOCKED · 10029 FROZEN · 10030 INVALID_FILL · 10031 CONNECTION · 10032 ONLY_REAL · 10033 LIMIT_ORDERS · 10034 LIMIT_VOLUME · 10035 INVALID_ORDER · 10036 POSITION_CLOSED · 10038 INVALID_CLOSE_VOLUME · 10039 CLOSE_ORDER_EXIST · 10040 LIMIT_POSITIONS · 10041 REJECT_CANCEL · 10042 LONG_ONLY · 10043 SHORT_ONLY · 10044 CLOSE_ONLY · 10045 FIFO_CLOSE`

</details>

> Note: `order_check` (and `MT5Sugar::can_open_position`) use retcode **0** as the "would succeed" signal for the *check*, which is distinct from the send success code `10009`.

---

## 🔧 Handling patterns

### Low-level: branch on the code

```rust
let res = account.order_send(req).await?;
match res.returned_code {
    10009 => println!("done #{}", res.order),
    10019 => eprintln!("no money — size down"),
    10016 => eprintln!("invalid stops — widen SL/TP"),
    10018 => eprintln!("market closed"),
    other => eprintln!("rejected {other}: {}", res.returned_code_description),
}
```

### High-level: let Sugar convert to an error

```rust
// Sugar maps non-10009 to MT5Error::Api, so `?` handles rejections:
let ticket = sugar.buy_market_with_pips(None, 0.10, Some(20.0), Some(40.0), "x", 1).await?;
```

### Retry on transient codes

```rust
let transient = |c: u32| matches!(c, 10004 | 10020 | 10021 | 10024 | 10031);
// re-fetch price and resend on a transient code, with a small backoff and a retry cap.
```

---

## 📚 See also

- [Getting Started](GETTING_STARTED.md) · [ENUMs Usage](ENUMS_USAGE_REFERENCE.md)
- [order_send](../MT5Account/5.%20Trading_Operations/order_send.md) · [order_check](../MT5Account/5.%20Trading_Operations/order_check.md)
- [Trading Operations — Overview](../MT5Account/5.%20Trading_Operations/Trading_Operations.Overview.md)
