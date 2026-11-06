# 5. Trading Operations — Overview

> The write path: send, modify and close orders — plus pre-trade calculators (check, margin, profit).

These are the **only methods in `MT5Account` that change account state** (`order_send`, `order_modify`, `order_close`). The three calculators (`order_check`, `order_calc_margin`, `order_calc_profit`) are read-only "what-if" helpers.

> The low-level calls take the raw protobuf requests (MT5's trade request has many fields). For friendlier argument shapes — buy/sell by symbol, pip-based SL/TP, risk sizing — use [`MT5Service`](../../MT5Service/MT5Service.Overview.md) and [`MT5Sugar`](../../MT5Sugar/MT5Sugar.Master.Overview.md), which build on these.

---

## Methods in this group

### Execution (state-changing)

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`order_send(req)`](./order_send.md) | `OrderSendData` | Place a market or pending order (with optional SL/TP). |
| [`order_modify(req)`](./order_modify.md) | `OrderModifyData` | Change SL/TP/price/expiry of an order or position. |
| [`order_close(req)`](./order_close.md) | `OrderCloseData` | Close (or partially close) by ticket. |

### Calculators (read-only)

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`order_check(req)`](./order_check.md) | `OrderCheckData` | Validate a trade request and preview margin/balance impact. |
| [`order_calc_margin(req)`](./order_calc_margin.md) | `OrderCalcMarginData` | Margin required for a hypothetical order. |
| [`order_calc_profit(req)`](./order_calc_profit.md) | `OrderCalcProfitData` | Profit for a hypothetical open→close move. |

---

## Return codes

Execution methods return a `returned_code` (`u32`) — the MT5 trade server retcode. **`10009` = `TRADE_RETCODE_DONE`** (success). Anything else is a rejection/partial; `returned_code_description` explains it. See the [Return Codes reference](../../All_Guides/RETURN_CODES_REFERENCE.md).

---

## A safe trading flow

1. **Calculate** — `order_calc_margin` / `order_calc_profit` to size and sanity-check.
2. **Check** — `order_check` to validate against the live account (margin, stops level, filling mode).
3. **Send** — `order_send`; confirm `returned_code == 10009`.
4. **Manage** — `order_modify` (move SL to breakeven, trail) as the trade runs.
5. **Close** — `order_close` by ticket.

> This is exactly what the `MT5Sugar` strategy orchestrators automate.

---

**Back to:** [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
