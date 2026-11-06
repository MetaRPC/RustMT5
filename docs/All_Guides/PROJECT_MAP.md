# 🗺️ RustMT5 Project Map

> The whole project as one annotated tree, with links. **Links point to section *overviews*, never to individual method pages** — open an overview to drill down into its methods.

---

## 🧱 The three layers

RustMT5 is built in three decoupled layers (composition, not inheritance). Your code can enter at any level.

| Layer | Source | What it does | Overview |
| ----- | ------ | ------------ | -------- |
| **3 · MT5Sugar** | `src/mt5/sugar.rs` | Convenience on top of Service: risk-based sizing, pip-based SL/TP, price/volume helpers, batch close, position queries, history. | [MT5Sugar overview](../MT5Sugar/MT5Sugar.Master.Overview.md) |
| **2 · MT5Service** | `src/mt5/service.rs` | Typed facade: `&str` symbols and typed-enum args; simple getters return clean scalars. What most code uses. | [MT5Service overview](../MT5Service/MT5Service.Overview.md) |
| **1 · MT5Account** | `src/mt5/account.rs` | Low-level gRPC: builds each proto `…Request`, attaches auth metadata, unwraps replies to `Result<…Data, MT5Error>`; connection & streaming. Everything builds on it. | [MT5Account overview](../MT5Account/MT5Account.Master.Overview.md) |

```text
Your code → MT5Sugar → MT5Service → MT5Account → gRPC → MetaRPC gateway → MT5 terminal
```

Mix freely: `sugar.service()` borrows the Service, `service.account()` borrows the Account.

---

## 🌳 Repository tree

- **`RustMT5/`** — crate root (`metarpc-mt5`)
  - **`src/`** — the library
    - `lib.rs` — crate root; re-exports `MT5Account` / `MT5Service` / `MT5Sugar` / `MT5Error` and the `strategies` module
    - `proto.rs` — `include!`s the vendored stubs into `mt5_term_api`
    - **`mt5/`** — the three SDK layers
      - `account.rs` — **Layer 1** source → [MT5Account overview](../MT5Account/MT5Account.Master.Overview.md)
      - `service.rs` — **Layer 2** source → [MT5Service overview](../MT5Service/MT5Service.Overview.md)
      - `sugar.rs` — **Layer 3** source → [MT5Sugar overview](../MT5Sugar/MT5Sugar.Master.Overview.md)
      - `mod.rs` — `pub mod account; service; sugar;`
    - **`helpers/`** — auxiliary (non-MT-layer) code
      - `error.rs` — the `MT5Error` enum + `From` impls → [Return Codes](RETURN_CODES_REFERENCE.md)
      - **`strategies/`** — orchestrators + presets → [Strategies overview](../Strategies/Strategies.Master.Overview.md)
        - `mod.rs` — `StrategyPreset`, `RiskPreset`, `manage_position`, presets catalog
        - `market_one_shot.rs` · `bracket_trailing.rs` · `pending_bracket.rs`
  - **`package/`** — the raw gRPC contract
    - `mt5_term_api.rs` — vendored prost/tonic generated stubs (no `build.rs`)
  - **`examples/`** — runnable demos (`cargo run --example …`), numbered by layer
    - **`1_lowlevel/`** → [MT5Account overview](../MT5Account/MT5Account.Master.Overview.md)
    - **`2_service/`** → [MT5Service overview](../MT5Service/MT5Service.Overview.md)
    - **`3_sugar/`** → [MT5Sugar overview](../MT5Sugar/MT5Sugar.Master.Overview.md)
    - **`4_strategies/`** → [Strategies overview](../Strategies/Strategies.Master.Overview.md)
  - **`tests/`** — smoke tests (construction, no network)
  - `Cargo.toml` — crate manifest · `mkdocs.yml` — docs site config

---

## 📚 Documentation tree

- **`docs/`**
  - [Home](../index.md) — landing page
  - **`All_Guides/`** — the guides ([Getting Started](GETTING_STARTED.md), [Your First Project](Your_First_Project.md), [Glossary](GLOSSARY.md), [Async Model](SYNC_VS_ASYNC.md), [gRPC Streaming](GRPC_STREAM_MANAGEMENT.md), [ENUMs](ENUMS_USAGE_REFERENCE.md), [Return Codes](RETURN_CODES_REFERENCE.md))
  - **`MT5Account/`** — low-level gRPC docs → [overview](../MT5Account/MT5Account.Master.Overview.md) *(per-method pages inside)*
  - **`MT5Service/`** — typed facade docs → [overview](../MT5Service/MT5Service.Overview.md) *(per-group pages inside)*
  - **`MT5Sugar/`** — convenience API docs → [overview](../MT5Sugar/MT5Sugar.Master.Overview.md) *(per-method pages inside)*
  - **`Strategies/`** — orchestrators + presets → [overview](../Strategies/Strategies.Master.Overview.md) *(per-strategy pages inside)*
  - `styles/` · `javascripts/` · `includes/` · `Guide_Images/` — assets & theme

> The `MT5Account/`, `MT5Service/`, `MT5Sugar/` and `Strategies/` folders hold per-method / per-strategy pages. By design this map links only their **overviews** — each overview is the index into its own detail pages.

---

## 🎯 Which layer should you use?

- **Most trading apps → [MT5Sugar](../MT5Sugar/MT5Sugar.Master.Overview.md).** Risk sizing, pip math and bulk ops out of the box.
- **Custom execution → [MT5Service](../MT5Service/MT5Service.Overview.md).** Typed calls when you manage your own request shapes.
- **Protocol-level control → [MT5Account](../MT5Account/MT5Account.Master.Overview.md).** Raw proto structures and streams.
