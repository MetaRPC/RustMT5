# 1. Account Information — Overview

> The account's money picture: a one-call snapshot, plus single typed properties (double/integer/string).

These four methods are **read-only**. They live on the `mt5_term_api.AccountHelper` (`account_summary`) and `mt5_term_api.AccountInformation` (`account_info_*`) gRPC services and return a typed `…Data` payload.

---

## Methods in this group

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`account_summary()`](./account_summary.md) | `AccountSummaryData` | Balance, equity, leverage, currency, trade mode, server time — one round-trip. |
| [`account_info_double(req)`](./account_info_double.md) | `AccountInfoDoubleData` | One `double` property (balance, equity, margin, profit, …). |
| [`account_info_integer(req)`](./account_info_integer.md) | `AccountInfoIntegerData` | One `integer` property (login, leverage, trade mode, …). |
| [`account_info_string(req)`](./account_info_string.md) | `AccountInfoStringData` | One `string` property (currency, company, name, server). |

---

## When to use which

* **Startup banner / general overview / pre-trade snapshot** → `account_summary()` (one round-trip for balance, equity, leverage, currency, …).
* **A single live value not in the summary** (e.g. `ACCOUNT_MARGIN`, `ACCOUNT_MARGIN_FREE`, `ACCOUNT_MARGIN_LEVEL`, `ACCOUNT_PROFIT`) → `account_info_double(property)`.
* **A single integer/string property** → `account_info_integer` / `account_info_string`.

> Note: `AccountSummaryData` carries balance/equity/credit but **not** the margin fields on this gateway — read those with `account_info_double(ACCOUNT_MARGIN…)`. This is exactly the kind of thing the SDK's `MT5Sugar::margin()` / `free_margin()` wrap for you.

---

**Back to:** [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
