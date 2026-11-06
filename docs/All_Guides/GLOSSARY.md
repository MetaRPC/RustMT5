# 📖 RustMT5 Glossary

> The terms and concepts used across the RustMT5 SDK and MetaTrader 5 trading — the three layers in depth, then the Rust, protocol, and trading vocabulary.

---

## 🏗️ The three layers

### MT5Account

**🔹 Layer 1 — Low-level API**

The base layer: direct access to the MT5 terminal over gRPC.

**Key characteristics**

- Builds each protobuf `…Request` and attaches `apikey` + session `id` metadata.
- Unwraps every reply's `oneof { data, error }` (or flat `data`/`error`) into `Result<…Data, MT5Error>`.
- Returns the typed protobuf `…Data` payloads — full control, maximum detail.
- Owns connection, session and streaming (`on_symbol_tick`, `on_trade`, …).
- **39 feature methods** across 6 groups, plus connection helpers.

**When to use:** you need a field or variant the higher layers don't surface, or raw protobuf control.

**Location:** `src/mt5/account.rs`

**Docs:** [MT5Account overview](../MT5Account/MT5Account.Master.Overview.md)

---

### MT5Service

**🔸 Layer 2 — Typed facade**

The middle layer: friendlier signatures without hand-building requests.

**Key characteristics**

- Takes typed enums and `&str` symbols; does the `as i32` casts and request-building for you.
- Simple getters unwrap `…Data` to a clean scalar (`f64` / `i64` / `String` / `bool`).
- Richer payloads (quotes, histories, streams) are returned as-is.
- **36 methods** across the same 6 groups, plus connection delegates.

**When to use:** you want ergonomic, typed calls but drive your own order shapes — the layer most code sits on.

**Location:** `src/mt5/service.rs`

**Docs:** [MT5Service overview](../MT5Service/MT5Service.Overview.md)

---

### MT5Sugar

**🔺 Layer 3 — Convenience API**

The high-level layer: business logic and one-liners.

**Key characteristics**

- Order construction from side/volume; pip-based SL/TP; risk-based position sizing.
- A **default symbol** (`with_default_symbol`) so price/order calls can take `None`.
- Compact result structs (`SymbolInfo`, `AccountInfo`, `DailyStats`) and a `Period` enum.
- Maps a non-`10009` trade result to `MT5Error::Api`, so `?` handles rejections.
- **58 methods** across 10 groups — the best starting point for most apps.

**Key examples**

- `buy_market(symbol, volume, comment, magic)` — open a position in one line.
- `buy_market_with_pips(symbol, volume, sl_pips, tp_pips, …)` — open with a pip bracket.
- `calculate_position_size(symbol, risk_percent, sl_pips)` — size by risk.
- `balance()` / `equity()` · `close_all_positions(symbol)` — quick reads and an emergency exit.

**When to use:** rapid development, prototyping, simple strategies.

**Location:** `src/mt5/sugar.rs`

**Docs:** [MT5Sugar overview](../MT5Sugar/MT5Sugar.Master.Overview.md)

---

## ⚙️ Rust & async concepts

### Async / await

Every API method is `async fn`; run them inside a tokio runtime and `.await` them. There is **no blocking API**. See [Async Model](SYNC_VS_ASYNC.md).

### `Result` & `MT5Error`

Calls return `Result<_, MT5Error>`. `MT5Error` variants: `Api { code, message, mql_code }`, `EmptyResponse`, `NotConnected`, `Connection`, `Transport`, `Status`, `InvalidArgument`. Propagate with `?`.

### `Stream` (tokio_stream)

Subscriptions return `Result<impl Stream<Item = Result<…Data, MT5Error>>, MT5Error>` — the outer `Result` is "subscription opened", each item is a frame or error. Read with `tokio_stream::StreamExt::next`. See [gRPC Streaming](GRPC_STREAM_MANAGEMENT.md).

### Result structs

`MT5Sugar` returns small plain structs instead of raw protobuf: **`SymbolInfo`** (name, bid, ask, spread, digits, point, volume min/step/max, contract_size), **`AccountInfo`** (login, name, company, currency, leverage, balance, equity), **`DailyStats`** (deals, profit, wins, losses).

### Enums & `r#type`

Protobuf enums are `i32` under the hood. Low level: pass `Enum::Variant as i32`; mid/high level: pass the typed enum; decode a received value with `Enum::try_from(v)`. Fields named `type` appear as the raw identifier **`r#type`**. See [ENUMs Usage](ENUMS_USAGE_REFERENCE.md).

---

## 🔌 gRPC & protocol

- **gRPC** — the RPC protocol the SDK uses to reach the MT5 gateway.
- **tonic** — the Rust gRPC client the SDK is built on.
- **protobuf / prost** — the message format; `prost` generates the Rust structs/enums, vendored in `package/mt5_term_api.rs`.
- **`…Request` / `…Data` / `…Reply`** — a call sends a `…Request`; the server answers a `…Reply` carrying a `…Data` payload or an `Error`.
- **oneof `{ data, error }`** — how most replies wrap the result; the SDK unwraps it to `Result<…Data, MT5Error>`.
- **apikey** — your MetaRPC token (`MRPC_API_KEY` env, else `"TRIAL"`), sent as metadata on every call.
- **session id (`terminalInstanceGuid`)** — the terminal session the gateway assigns on `connect_*`, auto-attached to later calls; read via `account.id()`.
- **RetCode (`returned_code`)** — the trade-server result after a trading call; **`10009` = success**. See [Return Codes](RETURN_CODES_REFERENCE.md).

---

## 💹 Market & prices

| Term | Meaning |
| --- | --- |
| **Symbol** | A tradable instrument (`EURUSD`); names are broker-specific (suffixes like `.m`). |
| **Bid / Ask** | Best sell / best buy price. **Spread** = ask − bid. |
| **Point** | Smallest price step (`SYMBOL_POINT`). |
| **Pip** | `10 × point` on 3/5-digit symbols (EURUSD 1 pip = 0.0001). |
| **Digits** | Decimal places in the price (`SYMBOL_DIGITS`). |
| **Tick** | One price update; **tick value** = money per tick per lot. |
| **Contract size** | Units per lot (e.g. 100,000 for FX). |
| **Volume / lot** | Trade size; bounded by `volume_min` / `volume_step` / `volume_max`. |
| **Market Watch** | The terminal's selected-symbols list; some calls need a symbol selected (`symbol_select`). |
| **DOM (Depth of Market)** | Level-II order book; subscribe/read/release (`market_book_add`/`get`/`release`). |

---

## 📈 Orders, positions & deals

In MT5, **orders**, **positions** and **deals** are distinct:

- **Order** — an instruction to trade (market or pending).
- **Position** — the open exposure from a filled order.
- **Deal** — an executed transaction in history; realized P/L lives on deals.
- **Pending order** — rests until a price triggers it: **Limit** (better price) / **Stop** (breakout) / **Stop-Limit**.
- **SL / TP** — protective Stop-Loss / Take-Profit exit prices.
- **Stops level** (`SYMBOL_TRADE_STOPS_LEVEL`) — minimum distance SL/TP may sit from the market, in points.
- **Freeze level** — distance within which modify/close is blocked.
- **Slippage / deviation** — allowed price movement between request and fill.
- **Magic number** — a client id tagging orders from one strategy/EA.
- **Ticket** — the numeric id of an order / position / deal.

---

## 💰 Account & money

| Term | Meaning |
| --- | --- |
| **Balance** | Realized funds, excluding floating P/L. |
| **Equity** | Balance + floating P/L (live account value). |
| **Margin** | Funds reserved to hold open positions. |
| **Free margin** | `equity − margin`; available for new positions. |
| **Margin level** | `equity / margin × 100`, in percent (margin-call/stop-out below thresholds). |
| **Leverage** | Borrowing ratio (e.g. 1:100). |
| **Floating / unrealized P/L** | Profit of still-open positions (`equity − balance`). |
| **Realized P/L** | Profit of closed positions (from history). |

---

## 🛡️ Strategy mechanics

- **Breakeven** — moving SL to (around) the entry once in profit, to remove risk.
- **Trailing stop** — an SL that follows price in the favorable direction, ratcheting and never loosening.
- **Risk sizing** — choosing lot size so a stop-out risks a fixed % of balance (`calculate_position_size`).
- **Period** — a predefined reporting window enum: `Today`, `Yesterday`, `ThisWeek`, `ThisMonth`.
- **Orchestrator** — a ready `async fn` strategy driving `MT5Sugar` (Market One-Shot, Bracket + Trailing, Pending Bracket).
- **Preset** — `StrategyPreset` (what to trade) / `RiskPreset` (how much, how to protect). See [Strategies](../Strategies/Strategies.Master.Overview.md).

---

## 📚 See also

- [Getting Started](GETTING_STARTED.md) · [Project Map](PROJECT_MAP.md) · [MT5 for Beginners](MT5_For_Beginners.md)
- [Return Codes](RETURN_CODES_REFERENCE.md) · [ENUMs Usage](ENUMS_USAGE_REFERENCE.md) · [gRPC Streaming](GRPC_STREAM_MANAGEMENT.md)
