# Protobuf Inspector — Interactive Type Explorer

> An offline developer tool that explores the MT5 gRPC protobuf **messages, fields and enums** — browse, search, find which message declares a field, and list enum values, all **without connecting to MT5**.

## Why Rust does this differently

The C#, Go and Java inspectors use *runtime reflection* over the protobuf descriptors their libraries keep in memory. Rust's `prost` keeps **no** runtime descriptors, and this crate ships **no** `.proto` files — only the pre-generated [`package/mt5_term_api.rs`](https://github.com/MetaRPC/RustMT5/blob/main/package/mt5_term_api.rs).

So instead of reflecting, this tool **parses that generated file** (embedded at compile time with `include_str!`). Every field carries its `#[prost(..)]` attribute, so the inspector reads the exact proto wire type, the field **tag**, and `repeated` / `optional` / `map` / `oneof` shape — a little *more* than reflection alone would expose. Because it reads the very file the crate compiles, it can never drift from your types.

---

## 🏁 Running it

It's a normal example and needs no credentials or connection:

```bash
cargo run --example protobuf_inspector
```

…or pick **“Tools · Protobuf Inspector”** from the interactive menu (`cargo run --example menu`).

```
╔══════════════════════════════════════════════════════╗
║        🔍 PROTOBUF INSPECTOR — mt5_term_api          ║
║   Explore MT5 gRPC messages, fields & enums offline  ║
╚══════════════════════════════════════════════════════╝
Parsed 298 messages, 71 enums from the generated code.

>
```

---

## 🧰 Commands

| Command | Description | Example |
|---------|-------------|---------|
| `list` / `ls` | List all message types | `list` |
| `enums` | List all enum types | `enums` |
| `<TypeName>` | Inspect a message — its fields and types | `AccountSummaryData` |
| `search <text>` / `find <text>` | Search types by name | `search Position` |
| `field <name>` | Find messages that contain a field | `field ticket` |
| `enum <name>` | Show an enum's values | `enum Bmt5EnumOrderType` |
| `help` / `?` | Help | `help` |
| `exit` / `quit` / `q` | Exit | `exit` |

Type names and field names are **case-insensitive**.

---

## Practical examples

### 1) Inspect a message's fields

```
> AccountSummaryData
```
```
╔═══ MESSAGE: AccountSummaryData ═══╗

📋 Fields (21):

  • #1   account_login                            : int64
  • #2   account_balance                          : double
  • #3   account_equity                           : double
  • #6   account_trade_mode                       : MrpcEnumAccountTradeMode
  • #9   server_time                              : optional Timestamp
  • #17  account_margin_mode                      : MrpcEnumAccountMarginMode
  ...
```

`#n` is the protobuf field tag; the type is the proto type. `repeated …`, `optional …`, `map<…>` and `oneof …` are shown where relevant, and message/enum fields show the referenced type name. These are the real field names on the generated `struct` — `summary.account_balance`, `summary.account_trade_mode`, etc.

### 2) A `oneof` reply

```
> AccountSummaryReply
```
```
╔═══ MESSAGE: AccountSummaryReply ═══╗

📋 Fields (1):

  •      response                                 : oneof Response (tags 1, 2)
```

The `*Reply` messages wrap their payload in a `response` oneof of `Data` | `Error` — exactly what you `match` on in the low-level layer.

### 3) Which messages carry a `ticket` field?

```
> field ticket
```
```
📦 HistoryOrderInfo:
   └─ ticket : int64
📦 OnPositionsAndPendingOrdersTicketsData:
   └─ position_tickets : repeated uint64
   └─ pending_order_tickets : repeated uint64
📦 MqlTradeResult:
   └─ ...
```

### 4) See an enum's values

```
> enum Bmt5EnumOrderType
```
```
╔═══ ENUM: Bmt5EnumOrderType ═══╗

  Bmt5OrderTypeBuy                                 = 0
  Bmt5OrderTypeSell                                = 1
  Bmt5OrderTypeBuyLimit                            = 2
  Bmt5OrderTypeSellLimit                           = 3
  Bmt5OrderTypeBuyStop                             = 4
  Bmt5OrderTypeSellStop                            = 5
  Bmt5OrderTypeBuyStopLimit                        = 6
  Bmt5OrderTypeSellStopLimit                       = 7
  Bmt5OrderTypeCloseBy                             = 8
```

> Note: the inspector prints the **Rust** variant names (`Bmt5OrderTypeBuy`), which is what you write in code — `Bmt5EnumOrderType::Bmt5OrderTypeBuy`. The underlying proto value names are the SCREAMING_CASE ones.

---

## 🧩 How it works

- `const GEN: &str = include_str!("../../package/mt5_term_api.rs");` embeds the generated prost code at compile time.
- A small parser walks it: a `#[derive(.., ::prost::Message)]` marks a message `struct`; a `#[derive(.., ::prost::Enumeration)]` marks an enum. For each field it reads the `#[prost(..)]` attribute for the proto type + tag and the `pub name: Type` line for the field name and referenced type.
- Empty messages (`pub struct Foo {}`) and `oneof` fields are handled explicitly, so the index is the **complete 298 messages / 71 enums**.
- Everything is **static** — no terminal connection, no network, no extra dependencies.

Source: [`examples/tools/protobuf_inspector.rs`](https://github.com/MetaRPC/RustMT5/blob/main/examples/tools/protobuf_inspector.rs).

---

**See also:** [ENUMs Reference](ENUMS_USAGE_REFERENCE.md) · [Project Map](PROJECT_MAP.md) · [Glossary](GLOSSARY.md)
